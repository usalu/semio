"""🧾️ Verify the release PDF dependency boundary against a Cargo tree or unit graph."""
import json
from pathlib import Path
import sys

fixture = json.loads(Path(__file__).with_name("procedural-drawing-dependencies.json").read_text())
evidence = Path(sys.argv[1])
if evidence.suffix == ".json":
    graph = json.loads(evidence.read_text())
    names = {unit["target"]["name"].replace("_", "-") for unit in graph["units"]}
else:
    names = {fields[0] for line in evidence.read_text().splitlines() if len(fields := line.split()) > 1 and fields[1].startswith("v")}
missing = set([fixture["root"], *fixture["required"]]) - names
unexpected = set(fixture["excluded"]) & names
if missing or unexpected:
    raise SystemExit(f"Dependency boundary failed. Missing {sorted(missing)}. Unexpected {sorted(unexpected)}.")
print("Drawing export, PDF lowering and byte writer present. PDF umbrella and mutation family 8 absent.")
