#!/usr/bin/env python3
"""📜️ D4 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING for `📜️mutate-docx-ecma-376`: the `patch-snapshot` row and its
committed after-document. The row's pointer addresses the subject's own `DocxSnapshot` reading — logical XML parts in
archive order, each a retained arena whose `attributes` list every element's attributes in pre-order — and is resolved here
to one attribute of one element. python-docx (MIT, the repo `.venv`) then writes that one attribute through its own
package/part/oxml model and saves; `docProps/core.xml` stays a plain part and the archive is re-zipped with a fixed date, the
same normalizations as the case's other afters, so a rerun is byte-identical. Writes the rows of both outlines, the
`🧾️readme-afters/🩹️patch-snapshot/➡️after.docx` document and its fixture manifest; then re-reads the saved package and
requires its reading to equal the patched reading in everything but the addressed value's own effect. Idempotent;
`--check` exits 1 while anything is pending.

usage: .venv/bin/python 🧪️s4-stdio-docx-patch-after.py [--check]
"""
from __future__ import annotations

import hashlib
import importlib.util
import io
import json
import pathlib
import re
import sys
import zipfile

import docx
from docx.opc.constants import CONTENT_TYPE as CT
from docx.opc.part import PartFactory
from docx.oxml.ns import qn

HERE = pathlib.Path(__file__).resolve().parent
ROOT = pathlib.Path(__file__).resolve().parents[7]
SUBSET = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base"
FEATURE = SUBSET / "🧪️tests/📜️mutate-docx-ecma-376/🥒️.feature"
CATALOG = SUBSET / "🔮️oracles/🔣️.json"
INPUT = SUBSET / "🧫️fixtures/📜️example-readme.docx"
DIRECTORY = "🩹️patch-snapshot"
AFTER = SUBSET / "🧫️fixtures/🧾️readme-afters" / DIRECTORY / "➡️after.docx"
FIXTURE_ID = "patch-snapshot-readme-applied"
FIXED_DATE = (2026, 1, 1, 0, 0, 0)
PATCH = {"operation": "set", "path": "/xmlParts/0/document/attributes/1/value", "value": "Heading2"}


def module(name: str, path: pathlib.Path):
    spec = importlib.util.spec_from_file_location(name, path)
    loaded = importlib.util.module_from_spec(spec)
    sys.modules[name] = loaded
    spec.loader.exec_module(loaded)
    return loaded


XML = module("s4_stdio_pptx_rows", HERE / "🧪️s4-stdio-pptx-rows.py")
HOST = module("semio_repo_test", ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")


def arena(node: dict, nodes: list, attributes: list) -> int:
    if node["kind"] == "element":
        first = len(attributes)
        attributes.extend({"name": attr["name"], "value": attr["value"]} for attr in node["attrs"])
        children = [arena(child, nodes, attributes) for child in node["children"]]
        for left, right in zip(children, children[1:]):
            nodes[left]["nextSibling"] = right
        value = {"kind": "element", "name": node["name"], "first_attribute": first, "attribute_count": len(node["attrs"]), "first_child": children[0] if children else None}
    else:
        value = {"kind": "text", "text": node["text"]}
    nodes.append({"nextSibling": None, "value": value})
    return len(nodes) - 1


def logical(archive: zipfile.ZipFile) -> list[str]:
    return [name for name in archive.namelist() if not name.endswith("/") and name != "[Content_Types].xml" and not name.endswith(".rels") and name.lower().endswith((".xml", ".vml"))]


def reading(archive: zipfile.ZipFile) -> dict:
    parts = []
    for path in logical(archive):
        nodes, attributes = [], []
        root = arena(XML.parse(archive.read(path).decode("utf-8")), nodes, attributes)
        parts.append({"path": path, "document": {"nodes": nodes, "attributes": attributes, "prolog": [], "epilog": [], "root": root, "doctype": None, "declaration": None}})
    return {"xmlParts": parts}


def target(archive: zipfile.ZipFile) -> tuple[str, int, str]:
    """🎯️ The pointer's part, the addressed element's pre-order index among elements, and the attribute's name."""
    match = re.fullmatch(r"/xmlParts/(\d+)/document/attributes/(\d+)/value", PATCH["path"])
    assert match, "the patch must address one attribute value"
    path = logical(archive)[int(match.group(1))]
    elements = []

    def pre(node: dict) -> None:
        if node["kind"] == "element":
            elements.append(node)
            for child in node["children"]:
                pre(child)

    pre(XML.parse(archive.read(path).decode("utf-8")))
    ordinal = int(match.group(2))
    for index, element in enumerate(elements):
        if ordinal < len(element["attrs"]):
            name = element["attrs"][ordinal]["name"]
            assert not name.startswith("xmlns"), "a namespace declaration is not an attribute python-docx writes"
            return path, index, name
        ordinal -= len(element["attrs"])
    raise AssertionError("the attribute ordinal is outside the part")


def deterministic(saved: bytes) -> bytes:
    source = zipfile.ZipFile(io.BytesIO(saved))
    out = io.BytesIO()
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as written:
        for info in source.infolist():
            entry = zipfile.ZipInfo(info.filename, FIXED_DATE)
            entry.compress_type = zipfile.ZIP_DEFLATED
            entry.external_attr = 0o644 << 16
            written.writestr(entry, source.read(info.filename))
    return out.getvalue()


def after_document() -> bytes:
    path, index, name = target(zipfile.ZipFile(INPUT))
    PartFactory.part_type_for.pop(CT.OPC_CORE_PROPERTIES, None)
    document = docx.Document(str(INPUT))
    part = next(part for part in document.part.package.iter_parts() if part.partname == "/" + path)
    element = list(part.element.iter())[index]
    element.set(qn(name), PATCH["value"])
    saved = io.BytesIO()
    document.save(saved)
    return deterministic(saved.getvalue())


def shape(nodes: list) -> list:
    """🌳️ An arena's nodes without their attribute coordinates, which a writer's namespace declarations shift."""
    return [(node["nextSibling"], {key: value for key, value in node["value"].items() if key not in ("first_attribute", "attribute_count")}) for node in nodes]


def verified(after: bytes) -> None:
    """🔬️ The saved package, read back independently, carries exactly the patched reading's addressed value, and every other
    logical part's element tree is the input's."""
    before = reading(zipfile.ZipFile(INPUT))
    expected = HOST.patched_snapshot(before, PATCH)
    actual = reading(zipfile.ZipFile(io.BytesIO(after)))
    assert [part["path"] for part in actual["xmlParts"]] == [part["path"] for part in expected["xmlParts"]], "python-docx changed the logical part list"
    for wanted, got in zip(expected["xmlParts"], actual["xmlParts"]):
        assert [(a["name"], a["value"]) for a in got["document"]["attributes"] if not a["name"].startswith("xmlns")] == [(a["name"], a["value"]) for a in wanted["document"]["attributes"] if not a["name"].startswith("xmlns")], f"{wanted['path']}: attributes differ"
        assert shape(got["document"]["nodes"]) == shape(wanted["document"]["nodes"]), f"{wanted['path']}: element tree differs"


def with_rows(text: str) -> str:
    row = json.dumps({"patch": PATCH}, ensure_ascii=False, separators=(", ", ": "))
    lines = text.split("\n")
    inserts = []
    for index, line in enumerate(lines):
        title = line.strip()
        if title not in ("Scenario Outline: Apply <id> to the real document", "Scenario Outline: Undoing <id> restores the document"):
            continue
        header = next(at for at in range(index + 1, len(lines)) if lines[at].strip().startswith("Examples:")) + 1
        end = next((at for at in range(header + 1, len(lines)) if not lines[at].strip().startswith("|")), len(lines))
        if any(lines[at].split("|")[1].strip() == "patch-snapshot" for at in range(header + 1, end)):
            continue
        fixture = lines[header].rstrip().rstrip("|").split("|")[-1].strip() == "fixture"
        inserts.append((end, f"      | patch-snapshot | {row} |" + (f" {DIRECTORY} |" if fixture else "")))
    for at, line in reversed(inserts):
        lines.insert(at, line)
    return "\n".join(lines)


def with_manifest(text: str, after: bytes) -> str:
    catalog = json.loads(text)
    manifests = catalog["fixtureManifests"]
    template = next(entry for entry in manifests if entry["id"] == "set-run-text-readme-applied")
    entry = json.loads(json.dumps(template))
    entry["id"] = FIXTURE_ID
    entry["mutation"] = "patch-snapshot"
    entry["files"][1].update({"path": f"../🧫️fixtures/🧾️readme-afters/{DIRECTORY}/➡️after.docx", "sha256": "sha256:" + hashlib.sha256(after).hexdigest(), "bytes": len(after)})
    entry["generator"]["command"] = ".venv/bin/python .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s4-stdio-docx-patch-after.py"
    entry["provenance"]["attribution"] = "Generated with python-docx (MIT): the case's patch-snapshot row resolved against the subject's retained reading to one attribute, written through python-docx's package/part/oxml model and saved; untouched parts stay verbatim, entries re-zipped with a fixed date"
    entry["notes"] = "The first body paragraph's pStyle w:val (attribute 1 of word/document.xml's retained reading) set from Heading1 to Heading2."
    manifests[:] = [existing for existing in manifests if existing["id"] != FIXTURE_ID] + [entry]
    indent = 2 if text.startswith('{\n  "') else 1
    return json.dumps(catalog, indent=indent, ensure_ascii=False) + "\n"


def main() -> int:
    check = "--check" in sys.argv
    after = after_document()
    verified(after)
    pending = 0
    for path, current, wanted in [
        (AFTER, AFTER.read_bytes() if AFTER.exists() else None, after),
        (FEATURE, FEATURE.read_text(encoding="utf-8"), with_rows(FEATURE.read_text(encoding="utf-8"))),
        (CATALOG, CATALOG.read_text(encoding="utf-8"), with_manifest(CATALOG.read_text(encoding="utf-8"), after)),
    ]:
        if current != wanted:
            pending += 1
            print(f"{'pending' if check else 'written'}: {path.relative_to(SUBSET)}")
            if not check:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(wanted) if isinstance(wanted, bytes) else path.write_text(wanted, encoding="utf-8")
    print(f"python-docx after verified against the patched reading; {pending} file(s) {'pending' if check else 'written'}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
