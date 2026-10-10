#!/usr/bin/env python3
"""🪑️ Wave W2 `w2-f3-graph`: writes the case `🪑️components-mep` of the three.js solids oracle under `🧫️fixtures/💡️inferences/🧊️element-solids/`: the committed room snapshot of the component corpus, the `expected` table
(closed-form volumes, written by the components oracle: every component with geometry and every duct and tray; pipes are tessellated and left to the three.js measure) and an empty `meshes` object that the Rust test
`the_committed_expectations_and_meshes_match_the_inference` blesses (`BIM_BLESS=1`). Usage: `python r12-w2-f3-graph-solids-case.py` from the repository root with the `.venv` interpreter."""
import importlib.util
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]


def child(parent, suffix):
    return next(path for path in sorted(parent.iterdir()) if path.name.endswith(suffix))


subset = child(child(child(child(child(child(next(p for p in ROOT.iterdir() if p.name.startswith("✏") and p.name.endswith("s")), "plugins"), "bim"), "artifacts"), "model"), "standards"), "1")
subset = child(child(subset, "subsets"), "any")
tests, fixtures = child(subset, "tests"), child(subset, "fixtures")
spec = importlib.util.spec_from_file_location("components_oracle", child(child(tests, "infer-bim-1-components"), ".py"))
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)

snapshot = json.loads(child(child(child(child(child(fixtures, "inferences"), "components"), "room"), "snapshot"), ".json").read_text(encoding="utf-8"))
table = oracle.table(snapshot)
expected = {}
for component_id, row in table["components"].items():
    if row["footprint"]:
        expected[component_id] = {"volume": row["volume"]}
for element_id, row in table["mep"].items():
    if row["kind"] in ("duct", "tray") and not row["issues"]:
        expected[element_id] = {"volume": row["volume"]}
case = child(child(fixtures, "inferences"), "element-solids")
target = case / ("\U0001fa91️components-mep") / "\U0001f523️.json"
target.parent.mkdir(parents=True, exist_ok=True)
body = json.dumps({"snapshot": snapshot, "expected": expected}, indent=2, ensure_ascii=False)
target.write_text(body[: body.rstrip().rfind("}")].rstrip() + ',\n  "meshes": {}\n}\n', encoding="utf-8")
print("%s: %d expected solids" % (target.parent.name, len(expected)))
