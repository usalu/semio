import importlib.util, json, sys, time
from pathlib import Path
S = Path(sys.argv[1])
spec = importlib.util.spec_from_file_location("energy_oracle", S / "🧪️tests" / "🔋️export-bim-1-energy" / "🐍️.py")
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
for case in ("🏠️room", "🏘️pair", "🧱️stack"):
    t = time.time()
    snap = json.loads((S / "🧫️fixtures" / "🚪️energy" / case / "📸️snapshot" / "🔣️.json").read_text(encoding="utf-8"))
    table = m.expected_table(snap)
    print(case, round(time.time() - t, 1), "s", {k: sorted(v["groups"]) for k, v in table["spaces"].items()})
    print("  project", {k: round(v, 4) for k, v in table["totals"]["project"].items() if isinstance(v, float)})
