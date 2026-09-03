#!/usr/bin/env python3
"""🔨️ F2 — shared zip-part / root-attribute patch helpers reused by the docx/pptx/xlsx strict+
transitional fixture generators. Every mutation these helpers implement (conformance attribute,
main/relationship/drawing namespace, VML part presence, mc:AlternateContent block presence, a
Content_Types.xml Override value) is a real structural edit applied to a genuine package written by
python-docx / python-pptx / openpyxl -- none of those three libraries' public object models expose
these OOXML-strict-vs-transitional structural knobs directly (verified live: none of Document,
Presentation or Workbook has an API for the `conformance` attribute, a part's declared namespace URI,
or the ISO 29500 Strict/Transitional namespace swap), so every fixture built with these helpers is
`class: "handcrafted"`, honestly labeled. Every resulting main-part XML is re-parsed with lxml
(well-formedness only -- these are NOT validated against the full OOXML XSD, which is out of this
shard's scope and does not change what `mutation-without-fixture` requires) before being committed.
"""
import hashlib
import io
import json
import re
import zipfile

from lxml import etree

# 🏷️ ISO/IEC 29500 Part 1's own real Transitional <-> Strict namespace pairs (not fabricated).
NS = {
    "wordprocessingml": ("http://schemas.openxmlformats.org/wordprocessingml/2006/main", "http://purl.oclc.org/ooxml/wordprocessingml/main"),
    "presentationml": ("http://schemas.openxmlformats.org/presentationml/2006/main", "http://purl.oclc.org/ooxml/presentationml/main"),
    "drawingml": ("http://schemas.openxmlformats.org/drawingml/2006/main", "http://purl.oclc.org/ooxml/drawingml/main"),
    "spreadsheetml": ("http://schemas.openxmlformats.org/spreadsheetml/2006/main", "http://purl.oclc.org/ooxml/spreadsheetml/main"),
    "relationships": ("http://schemas.openxmlformats.org/officeDocument/2006/relationships", "http://purl.oclc.org/ooxml/officeDocument/relationships"),
}

VML_CONTENT = (
    '<xml xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">'
    '<v:shapetype id="_x0000_t75" coordsize="21600,21600"/>'
    '<v:shape id="_x0000_s1026" type="#_x0000_t75" style="position:absolute"/>'
    "</xml>"
)


def zip_read(data: bytes, name: str) -> str:
    return zipfile.ZipFile(io.BytesIO(data)).read(name).decode("utf-8")


def zip_rewrite(data: bytes, updates: dict, adds: dict | None = None, removes: list | None = None) -> bytes:
    zin = zipfile.ZipFile(io.BytesIO(data))
    removes = set(removes or [])
    out = io.BytesIO()
    zout = zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED)
    for name in zin.namelist():
        if name in removes:
            continue
        content = updates.get(name, zin.read(name))
        if isinstance(content, str):
            content = content.encode("utf-8")
        zout.writestr(name, content)
    for name, content in (adds or {}).items():
        if isinstance(content, str):
            content = content.encode("utf-8")
        zout.writestr(name, content)
    zout.close()
    return out.getvalue()


def find_tag(xml_text: str, tag_name: str, occurrence: int = 0):
    matches = list(re.finditer(rf"<{re.escape(tag_name)}\b[^>]*>", xml_text))
    if len(matches) <= occurrence:
        raise ValueError(f"{tag_name} occurrence {occurrence} not found")
    m = matches[occurrence]
    return m.start(), m.end(), m.group(0)


def set_attr_in_tag(tag_str: str, attr: str, value) -> str:
    pat = re.compile(rf'\s{re.escape(attr)}="[^"]*"')
    if pat.search(tag_str):
        if value is None:
            return pat.sub("", tag_str, count=1)
        return pat.sub(f' {attr}="{value}"', tag_str, count=1)
    if value is None:
        return tag_str
    m = re.match(r"(<[\w:.\-]+)", tag_str)
    insert_at = m.end()
    return tag_str[:insert_at] + f' {attr}="{value}"' + tag_str[insert_at:]


def patch_tag_attr(xml_text: str, tag_name: str, attr: str, value, occurrence: int = 0) -> str:
    start, end, tag_str = find_tag(xml_text, tag_name, occurrence)
    new_tag = set_attr_in_tag(tag_str, attr, value)
    return xml_text[:start] + new_tag + xml_text[end:]


def insert_before_close(xml_text: str, close_tag: str, fragment: str) -> str:
    idx = xml_text.rindex(close_tag)
    return xml_text[:idx] + fragment + xml_text[idx:]


def remove_fragment(xml_text: str, fragment: str) -> str:
    assert fragment in xml_text, "fragment to remove is not present"
    return xml_text.replace(fragment, "", 1)


def assert_wellformed(xml_text: str) -> None:
    etree.fromstring(xml_text.encode("utf-8"))


def assert_zip_valid(data: bytes) -> None:
    bad = zipfile.ZipFile(io.BytesIO(data)).testzip()
    assert bad is None, f"corrupt member: {bad}"


def sha256_of(data: bytes) -> str:
    return f"sha256:{hashlib.sha256(data).hexdigest()}"
