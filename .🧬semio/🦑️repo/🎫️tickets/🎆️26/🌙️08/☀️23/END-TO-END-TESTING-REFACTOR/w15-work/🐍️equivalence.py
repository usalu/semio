"""🔬 Behaviour equivalence of the fifteen norm oracle adapters BEFORE and AFTER the shared-module
refactor: every planned scenario, both adapters, projections and failure messages compared exactly."""
import importlib.util, json, os, sys, traceback
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
WORK = ROOT / ".🧬semio/🦑️repo/⚡️cache/tests/work"
OLD = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w15-work/old-adapters"
HOSTPY = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️host.py"

spec = importlib.util.spec_from_file_location("semio_repo_test", HOSTPY)
host = importlib.util.module_from_spec(spec)
sys.modules["semio_repo_test"] = host
spec.loader.exec_module(host)

sys.path.insert(0, str(ROOT / "✏️s/🔌️plugins/📕️norm/🧪️oracle/📦️packages/🐍️python"))


def load(path, name):
    s = importlib.util.spec_from_file_location(name, path)
    m = importlib.util.module_from_spec(s)
    s.loader.exec_module(m)
    return m.adapter()


def run(adapter, plan, scenario):
    ctx = host.Context(plan, scenario, "oracle", str(ROOT))
    handler = adapter.handler(scenario["id"], "oracle")
    if handler is None:
        return ("NO-HANDLER", None)
    try:
        outcome = handler(ctx)
        return ("ok", json.dumps(outcome.projection, sort_keys=True, ensure_ascii=False), host.digest(outcome.raw))
    except Exception as error:  # noqa: BLE001
        return ("raised", type(error).__name__, str(error))


total = same = 0
mismatches = []
for case_dir in sorted(WORK.glob("*norm*oracle-python")):
    plan = json.loads((case_dir / "📋️plan.json").read_text(encoding="utf-8"))
    case = plan["case"] if isinstance(plan.get("case"), str) else case_dir.name
    slug = next(s for s in case_dir.name.split("-oracle-python")[:1])
    stem = [p for p in OLD.glob("*.py") if p.stem in case_dir.name]
    assert len(stem) == 1, (case_dir.name, stem)
    old_path = stem[0]
    new_path = next((ROOT / "✏️s/🔌️plugins/📕️norm").rglob("*/🧪️tests/%s/🐍️component.py" % old_path.stem))
    for key in list(sys.modules):
        if key.startswith("adapter_"):
            del sys.modules[key]
    old = load(old_path, "adapter_old")
    new = load(new_path, "adapter_new")
    for scenario in plan["scenarios"]:
        total += 1
        a, b = run(old, plan, scenario), run(new, plan, scenario)
        if a == b:
            same += 1
        else:
            mismatches.append((old_path.stem, scenario["id"], a, b))

print("[equivalence] scenarios=%d identical=%d mismatched=%d" % (total, same, len(mismatches)))
for row in mismatches[:20]:
    print("  MISMATCH", row[0], row[1])
    print("    old:", str(row[2])[:300])
    print("    new:", str(row[3])[:300])
sys.exit(0 if not mismatches else 1)
