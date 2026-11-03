"""Wave 2 features agent: dry-run of the Python oracle adapters of the norm step layer WITHOUT the platform host.
Fakes `semio_repo_test` (Adapter/Outcome/Context shapes of the Python host), expands each feature with the platform's own gherkin parser (node) and runs every oracle handler.
Usage: .venv/bin/python -I wave2-features-dryrun.py <case dir> [max scenarios]"""
import importlib.util, json, subprocess, sys, types
from pathlib import Path

case = Path(sys.argv[1]).resolve()
limit = int(sys.argv[2]) if len(sys.argv) > 2 else 10**9
T = Path(__file__).resolve().parent.parent
dump = subprocess.run(["node", "--experimental-strip-types", str(T / "🗑️generated/features/dump.ts"), str(case / "🥒️.feature")], capture_output=True, check=True).stdout
plan = json.loads(dump)
assert not plan["errors"], plan["errors"]

class Outcome:
    def __init__(self, projection, raw=None, **_): self.projection, self.raw = projection, raw

class Adapter:
    def __init__(self, implementation="python"): self.h = {}
    def oracle(self, sid, fn): self.h[sid] = fn; return self
    def subject(self, sid, fn): return self

mod = types.ModuleType("semio_repo_test"); mod.Adapter, mod.Outcome = Adapter, Outcome; import hashlib; mod.digest = lambda data: hashlib.sha256(data).hexdigest(); sys.modules["semio_repo_test"] = mod
spec = importlib.util.spec_from_file_location("semio_test_adapter", case / "🐍️.py")
sys.path.insert(0, str(next(p for p in case.parents if (p / "🔮️oracles" / "🏃️execution").is_dir()) / "🔮️oracles" / "🏃️execution"))
adapter_module = importlib.util.module_from_spec(spec); spec.loader.exec_module(adapter_module)
adapter = adapter_module.adapter()
any_dir = case.parents[1]

class Ctx:
    def __init__(self, scenario): self.scenario = scenario
    def input_bytes(self, uri): return open(self.input(uri), "rb").read()
    def input(self, uri):
        if uri.startswith("asset://"):
            return str(any_dir / "🖼️assets" / uri[len("asset://"):])
        assert uri.startswith("shared://"), uri
        return str(any_dir / "🧪️tests" / "🧫️fixtures" / uri[len("shared://"):])

ok = bad = 0
for scenario in plan["scenarios"][:limit]:
    handler = adapter.h.get(scenario["id"]) or adapter.h.get(scenario["outlineOf"] or scenario["id"])
    if handler is None:
        print("NO HANDLER", scenario["id"]); bad += 1; continue
    try:
        out = handler(Ctx(scenario)); ok += 1
        if ok <= 2: print("  sample", scenario["id"], json.dumps(out.projection)[:160])
    except Exception as error:
        bad += 1; print("FAIL", scenario["id"], type(error).__name__, str(error)[:200])
print("%s: %d ok, %d failed" % (case.name, ok, bad))
