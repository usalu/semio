#!/usr/bin/env python3
"""📽️ Authors the `🧱️mutate-pptx-ecma-376` Examples rows for the revision-bound address vocabulary (ticket
26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): it reads the committed seven-slide deck in archive order, parses every XML
part into the same element/attribute/text tree the subject retains, and derives each address exactly as
`xml_address::address` does — part path, child-index node path, expanded name and the FNV-1a structural subtree
revision (`E name A name = value … /E`, `T text`). Rows replace both outlines' tables; idempotent; `--check` exits 1
while the feature differs.

@see ../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs
"""
from __future__ import annotations

import json
import pathlib
import re
import sys
import zipfile

ROOT = pathlib.Path(__file__).resolve().parents[7]
BASE = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base"
FEATURE = BASE / "🧪️tests/🧱️mutate-pptx-ecma-376/🥒️.feature"
PML = ("http://schemas.openxmlformats.org/presentationml/2006/main", "http://purl.oclc.org/ooxml/presentationml/main")
DML = ("http://schemas.openxmlformats.org/drawingml/2006/main", "http://purl.oclc.org/ooxml/drawingml/main")
ENTITIES = {"lt": "<", "gt": ">", "amp": "&", "quot": '"', "apos": "'"}
TOKEN = re.compile(r"<\?.*?\?>|<!--.*?-->|<!\[CDATA\[(.*?)\]\]>|</([^>\s]+)\s*>|<([^>\s/]+)((?:\s+[^\s=]+\s*=\s*(?:\"[^\"]*\"|'[^']*'))*)\s*(/?)>|([^<]+)", re.S)
ATTR = re.compile(r"([^\s=]+)\s*=\s*(?:\"([^\"]*)\"|'([^']*)')")


def unescape(text: str) -> str:
    return re.sub(r"&(#x[0-9a-fA-F]+|#[0-9]+|[a-z]+);", lambda m: chr(int(m.group(1)[2:], 16)) if m.group(1).startswith("#x") else chr(int(m.group(1)[1:])) if m.group(1).startswith("#") else ENTITIES[m.group(1)], text)


def parse(xml: str) -> dict:
    stack: list[dict] = [{"kind": "element", "name": "", "attrs": [], "children": []}]
    for match in TOKEN.finditer(xml):
        cdata, close, open_name, attrs, empty, text = match.groups()
        if close:
            node = stack.pop()
            assert node["name"] == close, (node["name"], close)
        elif open_name:
            node = {"kind": "element", "name": open_name, "attrs": [{"name": a, "value": unescape(v if v is not None else w)} for a, v, w in ATTR.findall(attrs)], "children": []}
            stack[-1]["children"].append(node)
            if not empty:
                stack.append(node)
        elif cdata is not None:
            stack[-1]["children"].append({"kind": "cData", "text": cdata})
        elif text is not None and len(stack) > 1:
            stack[-1]["children"].append({"kind": "text", "text": unescape(text)})
    roots = [child for child in stack[0]["children"] if child["kind"] == "element"]
    assert len(stack) == 1 and len(roots) == 1
    return roots[0]


def fnv(data: bytes, value: int) -> int:
    for byte in data:
        value = ((value ^ byte) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return value


def node_hash(node: dict, value: int) -> int:
    if node["kind"] == "element":
        value = fnv(b"E" + node["name"].encode(), value)
        for attr in node["attrs"]:
            value = fnv(b"A" + attr["name"].encode() + b"=" + attr["value"].encode(), value)
        for child in node["children"]:
            value = node_hash(child, value)
        return fnv(b"/E", value)
    marker = {"text": b"T", "cData": b"C", "comment": b"M"}[node["kind"]]
    return fnv(marker + node["text"].encode(), value)


def revision(node: dict) -> str:
    return f"{node_hash(node, 0xCBF29CE484222325):016x}"


def scope_of(parent: dict[str, str], node: dict) -> dict[str, str]:
    scope = dict(parent)
    for attr in node.get("attrs", []):
        if attr["name"] == "xmlns":
            scope[""] = attr["value"]
        elif attr["name"].startswith("xmlns:"):
            scope[attr["name"][6:]] = attr["value"]
    return scope


def expanded(node: dict, scope: dict[str, str]) -> tuple[str, str]:
    prefix, _, local = node["name"].rpartition(":")
    return scope.get(prefix, ""), local


def walk(root: dict, path: list[int]) -> tuple[dict, dict[str, str]]:
    node, scope = root, scope_of({}, root)
    for index in path:
        node = node["children"][index]
        scope = scope_of(scope, node)
    return node, scope


def address(parts: dict[str, dict], part: str, path: list[int]) -> dict:
    node, scope = walk(parts[part], path)
    namespace, local = expanded(node, scope)
    return {"partPath": part, "nodePath": path, "namespaceUri": namespace, "localName": local, "revision": revision(node)}


def child_index(node: dict, scope: dict[str, str], namespaces: tuple[str, ...], local: str) -> list[int]:
    return [i for i, child in enumerate(node["children"]) if child["kind"] == "element" and expanded(child, scope_of(scope, child)) in [(n, local) for n in namespaces]]


def shape_id(shape: dict, scope: dict[str, str]) -> str | None:
    for child in shape["children"]:
        if child["kind"] != "element" or not expanded(child, scope_of(scope, child))[1].startswith("nv"):
            continue
        for candidate in child["children"]:
            if candidate["kind"] == "element" and expanded(candidate, scope_of(scope_of(scope, child), candidate)) in [(n, "cNvPr") for n in PML]:
                return next((a["value"] for a in candidate["attrs"] if a["name"] == "id"), None)
    return None


def deck() -> tuple[list[str], dict[str, dict], list[dict]]:
    archive = zipfile.ZipFile(BASE / "🧫️fixtures/📽️.pptx")
    order = [name for name in archive.namelist() if not name.endswith("/") and name != "[Content_Types].xml" and not name.endswith(".rels") and name.lower().endswith((".xml", ".vml"))]
    parts = {name: parse(archive.read(name).decode("utf-8")) for name in order}
    root_rels = parse(archive.read("_rels/.rels").decode())
    presentation = next(a["value"] for rel in root_rels["children"] if rel["kind"] == "element" for a in rel["attrs"] if a["name"] == "Target" and any(b["name"] == "Type" and b["value"].endswith("/officeDocument") for b in rel["attrs"]))
    rels = parse(archive.read("ppt/_rels/presentation.xml.rels").decode())
    targets = {dict((a["name"], a["value"]) for a in rel["attrs"])["Id"]: dict((a["name"], a["value"]) for a in rel["attrs"])["Target"] for rel in rels["children"] if rel["kind"] == "element"}
    root = parts[presentation]
    root_scope = scope_of({}, root)
    list_index = child_index(root, root_scope, PML, "sldIdLst")[0]
    listing = root["children"][list_index]
    list_scope = scope_of(root_scope, listing)
    slides = []
    for entry_index in child_index(listing, list_scope, PML, "sldId"):
        entry = listing["children"][entry_index]
        attrs = {a["name"]: a["value"] for a in entry["attrs"]}
        part = "ppt/" + targets[attrs["r:id"]]
        sld = parts[part]
        sld_scope = scope_of({}, sld)
        c_sld = child_index(sld, sld_scope, PML, "cSld")[0]
        tree_node, tree_scope = walk(sld, [c_sld])
        tree = child_index(tree_node, tree_scope, PML, "spTree")[0]
        spTree, sp_scope = walk(sld, [c_sld, tree])
        shapes = []
        for index, shape in enumerate(spTree["children"]):
            if shape["kind"] != "element":
                continue
            namespace, local = expanded(shape, scope_of(sp_scope, shape))
            if namespace not in PML or local in ("nvGrpSpPr", "grpSpPr"):
                continue
            identity = shape_id(shape, scope_of(sp_scope, shape))
            if identity is not None:
                shapes.append({"address": {"node": address(parts, part, [c_sld, tree, index]), "shapeId": identity}, "path": [c_sld, tree, index]})
        slides.append({"address": {"entry": address(parts, presentation, [list_index, entry_index]), "slidePartPath": part, "relationshipId": attrs["r:id"], "slideId": attrs.get("id", "")}, "part": part, "tree": [c_sld, tree], "shapes": shapes, "entry": entry})
    return order, parts, slides


def element(name: str, attrs: list[tuple[str, str]], children: list[dict]) -> dict:
    return {"kind": "element", "name": name, "attrs": [{"name": n, "value": v} for n, v in attrs], "children": children}


def text_path(node: dict, path: list[int]) -> list[int] | None:
    if node["kind"] == "element" and node["name"] == "a:t":
        return path
    for index, child in enumerate(node.get("children", [])):
        found = text_path(child, path + [index])
        if found is not None:
            return found
    return None


def set_snapshot() -> dict:
    ns = [("xmlns:a", DML[0]), ("xmlns:r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships"), ("xmlns:p", PML[0])]
    presentation = element("p:presentation", ns, [element("p:sldIdLst", [], [element("p:sldId", [("id", "256"), ("r:id", "rId1")], [])])])
    title = element("p:sp", [], [element("p:nvSpPr", [], [element("p:cNvPr", [("id", "2"), ("name", "Title")], []), element("p:cNvSpPr", [], []), element("p:nvPr", [], [])]), element("p:spPr", [], [element("a:xfrm", [], [element("a:off", [("x", "0"), ("y", "0")], []), element("a:ext", [("cx", "100"), ("cy", "100")], [])])]), element("p:txBody", [], [element("a:bodyPr", [], []), element("a:p", [], [element("a:r", [], [element("a:t", [], [{"kind": "text", "text": "Replacement Deck"}])])])])])
    slide = element("p:sld", ns, [element("p:cSld", [], [element("p:spTree", [], [element("p:nvGrpSpPr", [], [element("p:cNvPr", [("id", "1"), ("name", "")], []), element("p:cNvGrpSpPr", [], []), element("p:nvPr", [], [])]), element("p:grpSpPr", [], []), title])])])
    main = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"
    slide_type = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"
    rel = lambda rid, kind, target: {"id": rid, "relType": f"http://schemas.openxmlformats.org/officeDocument/2006/relationships/{kind}", "target": target, "targetMode": "internal"}
    return {"snapshot": {"schema": "stdio.pptx", "opc": {"parts": [], "contentTypes": {"defaults": [["rels", "application/vnd.openxmlformats-package.relationships+xml"], ["xml", "application/xml"]], "overrides": [["/ppt/presentation.xml", main], ["/ppt/slides/slide1.xml", slide_type]]}, "relationships": {"": [rel("rId1", "officeDocument", "ppt/presentation.xml")], "ppt/presentation.xml": [rel("rId1", "slide", "slides/slide1.xml")]}, "comment": ""}, "xmlParts": [{"path": "ppt/presentation.xml", "contentType": main, "document": {"root": presentation}}, {"path": "ppt/slides/slide1.xml", "contentType": slide_type, "document": {"root": slide}}]}}


def rows() -> list[tuple[str, dict]]:
    order, parts, slides = deck()
    presentation = slides[0]["address"]["entry"]["partPath"]
    list_path = slides[0]["address"]["entry"]["nodePath"][:-1]
    duplicate = element("p:sldId", [("id", "9999"), ("r:id", slides[0]["address"]["relationshipId"])], [])
    shape = element("p:sp", [], [element("p:nvSpPr", [], [element("p:cNvPr", [("id", "999"), ("name", "Added Shape")], []), element("p:cNvSpPr", [("txBox", "1")], []), element("p:nvPr", [], [])]), element("p:spPr", [], [element("a:xfrm", [], [element("a:off", [("x", "100"), ("y", "100")], []), element("a:ext", [("cx", "500"), ("cy", "300")], [])])]), element("p:txBody", [], [element("a:bodyPr", [], []), element("a:p", [], [element("a:r", [], [element("a:t", [], [{"kind": "text", "text": "Added Shape"}])])])])])
    last = slides[6]
    target = last["shapes"][1]
    texts = [(shape["path"], text_path(walk(parts[last["part"]], shape["path"])[0], [])) for shape in last["shapes"]]
    written, inner = next((path, inner) for path, inner in texts if inner is not None)
    text_pointer = "/xmlParts/" + str(order.index(last["part"])) + "/document/root" + "".join(f"/children/{i}" for i in written + inner) + "/children/0/text"
    return [
        ("set-snapshot", set_snapshot()),
        ("insert-slide", {"vacancy": {"container": address(parts, presentation, list_path), "index": 3}, "entry": duplicate}),
        ("remove-slide", {"address": slides[2]["address"]}),
        ("move-slide", {"address": slides[0]["address"], "destinationIndex": 6}),
        ("insert-shape", {"vacancy": {"container": address(parts, slides[0]["part"], slides[0]["tree"]), "index": 2}, "shape": shape}),
        ("remove-shape", {"address": slides[1]["shapes"][2]["address"]}),
        ("set-shape-text", {"address": slides[0]["shapes"][0]["address"], "text": "Changed Title"}),
        ("set-shape-position", {"address": target["address"], "position": {"x": "1", "y": "2", "cx": "3", "cy": "4"}}),
        ("patch-snapshot", {"patch": {"operation": "set", "path": text_pointer, "value": "Patched Text"}}),
    ]


def main() -> int:
    check = "--check" in sys.argv
    table = "\n".join(f"      | {kind} | {json.dumps(params, separators=(',', ':'), ensure_ascii=False)} |" for kind, params in rows())
    text = FEATURE.read_text(encoding="utf-8")
    pattern = re.compile(r"(    Examples:\n      \| id +\| params \|\n)((?:      \|[^\n]*\n)+)")
    updated = pattern.sub(lambda m: m.group(1) + table + "\n", text)
    pending = int(updated != text)
    if pending and not check:
        FEATURE.write_text(updated, encoding="utf-8")
    print(f"{pending} feature {'pending' if check else 'written'}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
