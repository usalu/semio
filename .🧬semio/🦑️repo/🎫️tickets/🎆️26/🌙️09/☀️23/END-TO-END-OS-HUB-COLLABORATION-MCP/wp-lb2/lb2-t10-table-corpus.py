#!/usr/bin/env python3
"""📊️ LB2 item 10 (rule 22, test-only): the ui conformance corpus case `🧩️component/📊️table`.

Truth = the TableRow contract agreed with WG11 (session 14b): a table row's cells and row actions are PROPS (`cells`,
`rowActions` — the one row-action representation every renderer paints); a plain row has no children; an editable row
has exactly one child per materialised cell (`cell-<logicalColumn>`: Commit-bound, revision-guarded input). The peer's
00:21 snapshot added editable cell inputs (kept) AND duplicated each row action as a `row-action-<i>` child button
(removed here: the prop is the representation).

Step 1 rewrites `📸️snapshot.json` to the contract (drops every `row-action-*` node and its parent reference).
Step 2 DERIVES `🎯️expect.json` from the snapshot alone — never from the implementation under test: root, nodeCount,
shape (id/key/type/children by ascending id), accessibility (explicit record accessibility, contract defaults), and
the reachable action ids exactly as the harness enumerates them (bindings by ascending node id, `scope.name@version`).
Usage: lb2-t10-table-corpus.py [--dry-run | --write] [--root <repo-or-overlay root>]"""
import json
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
CASE = ROOT / "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/📊️table"
DESCRIPTION = ("A windowed Table (rows 12–13 of 40) with two TableRows — header, cells and row actions are props (the one "
               "row-action representation); the editable row's cells are its two Commit-bound, revision-guarded input "
               "children, the plain row has none, so the table is five node records.")

snapshot = json.loads((CASE / "📸️snapshot.json").read_text(encoding="utf-8"))
dropped = {node["id"] for node in snapshot["nodes"] if node["key"].startswith("row-action-")}
snapshot["nodes"] = [node for node in snapshot["nodes"] if node["id"] not in dropped]
for node in snapshot["nodes"]:
    if "children" in node:
        node["children"] = [child for child in node["children"] if child not in dropped]
        if not node["children"]:
            del node["children"]
nodes = sorted(snapshot["nodes"], key=lambda node: node["id"])
for node in nodes:
    if node["component"]["type"] == "tableRow":
        assert all(nodes_by_id["key"].startswith("cell-") for nodes_by_id in nodes if nodes_by_id["id"] in node.get("children", [])), f"{node['key']}: a row's children are its cells only"


def accessibility(node):
    explicit = node.get("accessibility", {})
    return {"id": node["id"], "label": explicit.get("label"), "description": explicit.get("description"), "live": explicit.get("live", "off"), "shortcut": explicit.get("shortcut"), "hidden": explicit.get("hidden", False)}


expect = {
    "case": "table",
    "kind": "component",
    "description": DESCRIPTION,
    "outcome": "accept",
    "limits": None,
    "tree": {"root": snapshot["root"], "nodeCount": len(nodes), "shape": [{"id": node["id"], "key": node["key"], "type": node["component"]["type"], "children": node.get("children", [])} for node in nodes]},
    "accessibility": [accessibility(node) for node in nodes],
    "actionIds": [f"{binding['action']['scope']}.{binding['action']['name']}@{binding['action']['version']}" for node in nodes for binding in node.get("bindings", [])],
}
print(f"dropped row-action nodes {sorted(dropped)}; nodeCount {len(nodes)}; actionIds {expect['actionIds']}")
if WRITE:
    (CASE / "📸️snapshot.json").write_text(json.dumps(snapshot, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    (CASE / "🎯️expect.json").write_text(json.dumps(expect, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print("written")
