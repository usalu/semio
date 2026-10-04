"""🐍️ S4-GRAPHS: runs the INDEPENDENT Python graph oracle (`🌳️mutate-semio-graph/🐍️.py`) on its own, without cargo.

Every `mutate`/`inverse` feature row is applied to the real Nakagin Capsule Tower graph (forward application must succeed,
the oracle's own undo must restore the document exactly), and every `spec-vector` row applies the committed
`(before, mutation, after)` quintet. The Rust side of the differential is `cargo test` (owed under rule 43); this is the
Python half only. The harness module `semio_repo_test` is stubbed with inert names — only the pure functions run.
"""

import importlib.util
import json
import pathlib
import sys
import types

ROOT = pathlib.Path(__file__).resolve().parents[7]
CASE = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧪️tests/🌳️mutate-semio-graph"
FIXTURES = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧫️fixtures"

stub = types.ModuleType("semio_repo_test")
for name in ("Adapter", "Context", "Outcome", "digest", "patched_snapshot", "snapshot_patch_inverse"):
    setattr(stub, name, type(name, (), {}))
sys.modules["semio_repo_test"] = stub
spec = importlib.util.spec_from_file_location("graph_oracle", CASE / "🐍️.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)

tower = oracle.parse_dsl((FIXTURES / "🌳️mutate-semio-graph/🏢️nakagin-capsule-tower/🗣️.dsl.semio").read_text())
passed, failed = 0, []
for line in (CASE / "🥒️.feature").read_text().splitlines():
    cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
    if len(cells) == 2 and cells[1].startswith('{"prepare"'):
        identifier, plan = cells[0], json.loads(cells[1])
        try:
            document = oracle.apply_all(oracle.clone(tower), plan.get("prepare", []))
            mutated = oracle.apply_mutation(document, plan["mutation"])
            restored = oracle.apply_all(mutated, oracle.inverse_mutation(document, plan["mutation"]))
            assert restored == document, "the undo did not restore the tower"
            passed += 1
        except Exception as error:
            failed.append("row %s: %s" % (identifier, str(error)[:300]))
    elif len(cells) == 3 and cells[1] and (FIXTURES / "🧬️mutations" / cells[1] / cells[2]).is_dir():
        base = FIXTURES / "🧬️mutations" / cells[1] / cells[2]
        try:
            before = json.loads((base / "📸️snapshot/⬅️before/🔣️.json").read_text())
            after = json.loads((base / "📸️snapshot/➡️after/🔣️.json").read_text())
            applied = oracle.apply_mutation(before, json.loads((base / "🦠️mutation/🔣️.json").read_text()))
            assert applied == after, "applied != after"
            passed += 1
        except Exception as error:
            failed.append("vector %s: %s" % (cells[0], str(error)[:300]))
print("[python-oracle] passed=%d failed=%d" % (passed, len(failed)))
for entry in failed:
    print("  FAIL " + entry)
sys.exit(1 if failed else 0)
