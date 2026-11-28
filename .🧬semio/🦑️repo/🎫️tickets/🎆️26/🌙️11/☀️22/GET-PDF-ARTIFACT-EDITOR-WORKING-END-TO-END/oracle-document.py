"""🔮️ Third-party oracle for the document-level sweep (`sweep-document.ts`): pymupdf and pypdf open the saved PDF, render every page, verify exact xref offsets and assert the requested document structure.

usage: uv run --offline --with pymupdf==1.26.7 --with pypdf python -I oracle-document.py check <pdf> <checks.json> [password]
`checks.json` maps a check name to its expected value; `null` means the entry must be absent. See ./🧭️sweep-document.md
"""
import json
import pathlib
import re
import sys

import pymupdf
import pypdf


def xref_offsets(path):
    data = pathlib.Path(path).read_bytes()
    assert data.startswith(b"%PDF-") and data.rstrip().endswith(b"%%EOF"), path
    sections = 0
    for table in re.finditer(rb"xref\r?\n((?:\d+ \d+\r?\n(?:\d{10} \d{5} [nf][ \r\n]{2})+)+)trailer", data):
        sections += 1
        for sub in re.finditer(rb"(\d+) (\d+)\r?\n((?:\d{10} \d{5} [nf][ \r\n]{2})+)", table.group(1)):
            for index, entry in enumerate(re.findall(rb"(\d{10}) \d{5} ([nf])", sub.group(3))):
                if entry[1] == b"n":
                    number = int(sub.group(1)) + index
                    assert re.match(rb"%d \d+ obj" % number, data[int(entry[0]):]), (path, number, int(entry[0]))
    return sections


def catalog_key(doc, key):
    kind, value = doc.xref_get_key(doc.pdf_catalog(), key)
    return None if kind == "null" else value


def trailer_key(doc, key):
    kind, value = doc.xref_get_key(-1, key)
    return None if kind == "null" else value


def info_xref(doc):
    kind, value = doc.xref_get_key(-1, "Info")
    return int(value.split()[0]) if kind == "xref" else None


def info_key(doc, key):
    xref = info_xref(doc)
    if xref is None:
        return None
    kind, value = doc.xref_get_key(xref, key)
    return None if kind == "null" else value


def text_of(value):
    if value is None:
        return None
    value = value.strip()
    if value.startswith("("):
        return value[1:-1]
    if value.startswith("<"):
        raw = bytes.fromhex(value[1:-1])
        return raw[2:].decode("utf-16-be") if raw.startswith(b"\xfe\xff") else raw.decode("latin-1")
    return value


def resolved(doc, kind, value):
    return doc.xref_object(int(value.split()[0]), compressed=False) if kind == "xref" else value


def check_info(doc, py, want):
    info = doc.metadata
    for key, value in want.items():
        if key == "trapped":
            assert text_of(info_key(doc, "Trapped")) == value, (key, info_key(doc, "Trapped"), value)
        elif value is None:
            assert not info.get(key), (key, info.get(key))
        elif key in ("creationDate", "modDate"):
            assert (info.get(key) or "").startswith(f"D:{value}"), (key, info.get(key), value)
        else:
            assert (info.get(key) or None) == value, (key, info.get(key), value)


def check_lang(doc, py, want):
    assert text_of(catalog_key(doc, "Lang")) == want, (catalog_key(doc, "Lang"), want)


def check_layout(doc, py, want):
    assert catalog_key(doc, "PageLayout") == (None if want is None else f"/{want}"), (catalog_key(doc, "PageLayout"), want)
    assert py.page_layout == (None if want is None else f"/{want}"), (py.page_layout, want)


def check_mode(doc, py, want):
    assert catalog_key(doc, "PageMode") == (None if want is None else f"/{want}"), (catalog_key(doc, "PageMode"), want)
    assert py.page_mode == (None if want is None else f"/{want}"), (py.page_mode, want)


def check_toc(doc, py, want):
    assert doc.get_toc() == want, (doc.get_toc(), want)


def check_markinfo(doc, py, want):
    kind, value = doc.xref_get_key(doc.pdf_catalog(), "MarkInfo")
    if want is None:
        assert kind == "null", (kind, value)
        return
    text = resolved(doc, kind, value)
    for key, flag in want.items():
        assert bool(re.search(rf"/{key}\s+true", text)) == flag, (key, text)


def check_xmp(doc, py, want):
    xml = doc.get_xml_metadata()
    if want is None:
        assert not xml, xml
        return
    assert want in xml, (want, xml[:300])
    assert py.xmp_metadata is not None


def viewer(doc):
    kind, value = doc.xref_get_key(doc.pdf_catalog(), "ViewerPreferences")
    return None if kind == "null" else resolved(doc, kind, value)


def check_viewer(doc, py, want):
    text = viewer(doc)
    for key, pattern in want.items():
        found = text is not None and re.search(rf"/{key}\s*{pattern}", text) is not None
        assert found == (pattern is not None), (key, pattern, text)


def check_openaction(doc, py, want):
    kind, value = doc.xref_get_key(doc.pdf_catalog(), "OpenAction")
    if want is None:
        assert kind == "null", (kind, value)
        return
    text = resolved(doc, kind, value)
    if want["kind"] == "page":
        target = doc[want["page"] - 1].xref
        assert re.search(rf"\[\s*{target} 0 R\s*/Fit", text), (text, target)
    else:
        assert "/URI" in text and want["uri"] in text, text


def check_docid(doc, py, want):
    if want is None:
        assert py.trailer.get("/ID") is None
        return
    got = [getattr(item, "original_bytes", None) or str(item).encode("latin-1") for item in py.trailer["/ID"]]
    assert got == [value.encode() for value in want], (got, want)


def check_dests(doc, py, want):
    got = {name: value["page"] + 1 for name, value in doc.resolve_names().items()}
    assert got == want, (got, want)
    assert set(py.named_destinations) == set(want), (set(py.named_destinations), want)


def check_embedded(doc, py, want):
    assert sorted(doc.embfile_names()) == sorted(want), (doc.embfile_names(), want)
    for name, body in want.items():
        assert doc.embfile_get(name) == body.encode(), (name, doc.embfile_get(name))
    assert sorted(py.attachments) == sorted(want), py.attachments


def check_outputintents(doc, py, want):
    kind, value = doc.xref_get_key(doc.pdf_catalog(), "OutputIntents")
    if want is None:
        assert kind == "null", (kind, value)
        return
    intents = py.trailer["/Root"]["/OutputIntents"]
    assert len(intents) == len(want), (intents, want)
    for item, expected in zip(intents, want):
        item = item.get_object()
        assert item["/S"] == f"/{expected['subtype']}", item
        assert item["/OutputConditionIdentifier"] == expected["identifier"], item
        if expected.get("info"):
            assert item["/Info"] == expected["info"], item


def check_catalog(doc, py, want):
    for key, value in want.items():
        assert catalog_key(doc, key) == value, (key, catalog_key(doc, key), value)


def check_trailer(doc, py, want):
    for key, value in want.items():
        assert trailer_key(doc, key) == value, (key, trailer_key(doc, key), value)


def check_infoextra(doc, py, want):
    for key, value in want.items():
        assert info_key(doc, key) == value, (key, info_key(doc, key), value)


def check_indirect(doc, py, want):
    for number, value in want.items():
        number = int(number)
        if value is None:
            assert number >= doc.xref_length() or doc.xref_object(number, compressed=False).strip() in ("", "null"), (number, doc.xref_object(number, compressed=False))
        else:
            assert number < doc.xref_length(), (number, doc.xref_length())
            assert doc.xref_object(number, compressed=False).strip() == value, (number, doc.xref_object(number, compressed=False))


def check_ocg(doc, py, want):
    groups = doc.get_ocgs()
    names = sorted(group["name"] for group in groups.values())
    assert names == sorted(want["names"]), (names, want)
    for group in groups.values():
        if group["name"] in want.get("off", []):
            assert not group["on"], group
        if group["name"] in want.get("on", []):
            assert group["on"], group


def check_labels(doc, py, want):
    labels = doc.get_page_labels()
    for item in want:
        assert any(label["startpage"] == item["startpage"] and label.get("prefix", "") == item["prefix"] for label in labels), (labels, item)


def check_pages(doc, py, want):
    assert doc.page_count == want == len(py.pages), (doc.page_count, len(py.pages), want)


def check_text(doc, py, want):
    for index, marker in want.items():
        assert marker in doc[int(index)].get_text(), (index, marker, doc[int(index)].get_text())


def check_encrypt(doc, py, want):
    if not want:
        assert not doc.is_encrypted and "/Encrypt" not in py.trailer, (doc.is_encrypted, py.trailer)
        return
    fields = py.trailer["/Encrypt"]
    assert fields["/V"] == want["v"] and fields["/R"] == want["r"], (dict(fields), want)


CHECKS = {key[6:]: value for key, value in globals().items() if key.startswith("check_")}


def run(path, checks, password):
    encrypted = b"/Encrypt" in pathlib.Path(path).read_bytes()
    doc = pymupdf.open(path)
    py = pypdf.PdfReader(path)
    if encrypted:
        assert doc.needs_pass, "an encrypted file must need a password"
        assert doc.authenticate(password) > 0, f"user password {password!r} must authenticate"
    assert doc.page_count > 0
    for index in range(doc.page_count):
        pixmap = doc[index].get_pixmap(dpi=50)
        assert pixmap.width > 0 and pixmap.height > 0
    sections = xref_offsets(path)
    for name, want in checks.items():
        assert name in CHECKS, f"unknown check {name}"
        CHECKS[name](doc, py, want)
    print(f"ok {path}: {doc.page_count} pages rendered, {sections} xref sections with exact offsets, checks {sorted(checks)}")


def compare(original, saved, title):
    before, after = pymupdf.open(original), pymupdf.open(saved)
    assert before.page_count == after.page_count, (before.page_count, after.page_count)
    xref_offsets(saved)
    for index in range(after.page_count):
        assert before[index].get_text() == after[index].get_text(), f"page {index + 1} text differs"
        assert before[index].rect == after[index].rect, f"page {index + 1} size differs"
        pixmap = after[index].get_pixmap(dpi=36)
        assert pixmap.width > 0 and pixmap.height > 0
    if title is not None:
        assert after.metadata["title"] == title, (after.metadata["title"], title)
    assert len(pypdf.PdfReader(saved).pages) == after.page_count
    print(f"ok {saved}: {after.page_count} pages with the same text and size as {original}, all rendered, title {after.metadata['title']!r}")


if sys.argv[1] == "compare":
    compare(sys.argv[2], sys.argv[3], sys.argv[4] if len(sys.argv) > 4 else None)
elif sys.argv[1] == "check":
    run(sys.argv[2], json.loads(pathlib.Path(sys.argv[3]).read_text()), sys.argv[4] if len(sys.argv) > 4 else "")
