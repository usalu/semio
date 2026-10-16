"""🏃️ Runs the energy mutation Python adapter through the repository's own Python host with a hand-built plan.

usage: pyrun.py <adapter 🐍️.py> <results.jsonl>
The plan mirrors what the coordinator declares: every Examples row of the feature expands to a `mutate-<id>` and an
`inverse-<id>` scenario, `identity-round-trip` is the one plain scenario, and every fixture file is a declared input.
"""
import json, os, re, shutil, subprocess, sys, tempfile

ROOT = "/Users/ueli/Documents/semio"
CASE = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1"
SUBSET = os.path.dirname(os.path.dirname(CASE))
HOST = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py"
PYTHON = os.path.join(ROOT, ".venv/bin/python3")

adapter, out = sys.argv[1], sys.argv[2]
feature = open(os.path.join(ROOT, CASE, "🥒️.feature"), encoding="utf-8").read().split("\n")
rows = []
for line in feature:
    cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
    if line.strip().startswith("|") and len(cells) == 3 and cells[0] != "id":
        rows.append(cells)
ids = []
for row in rows:
    if row[0] not in ids:
        ids.append(row[0])
fixtures = os.path.join(ROOT, SUBSET, "🧫️fixtures")
inputs = []
for dirpath, _, names in os.walk(os.path.join(ROOT, SUBSET, "🧪️tests", "🧫️fixtures")):
    for name in names:
        absolute = os.path.join(dirpath, name)
        relative = os.path.relpath(absolute, os.path.join(ROOT, SUBSET, "🧪️tests", "🧫️fixtures"))
        inputs.append({"uri": "shared://" + relative, "path": os.path.relpath(absolute, ROOT)})
asset = "asset://🎬️demo/🗣️.dsl.semio"
inputs.append({"uri": asset, "path": os.path.join(SUBSET, "🖼️assets", "🎬️demo", "🗣️.dsl.semio")})
scenarios = []
for outline in ("mutate", "inverse"):
    for scenario_id in ids:
        scenarios.append({"id": f"{outline}-{scenario_id}", "outlineOf": outline, "level": "exhaustive", "steps": []})
scenarios.append({"id": "identity-round-trip", "level": "long", "steps": [{"text": "Given the real committed document " + asset}]})
work = tempfile.mkdtemp(prefix="pfb-py-", dir=os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"))
plan = {"owner": "pfb", "case": "mutate-energy-model-1", "implementation": "python", "role": "oracle", "workDir": os.path.join(work, "work"), "outputDir": os.path.join(work, "out"), "inputs": inputs, "scenarios": scenarios}
plan_path = os.path.join(work, "plan.json")
json.dump(plan, open(plan_path, "w", encoding="utf-8"), ensure_ascii=False)
result = subprocess.run([PYTHON, "-I", "-B", os.path.join(ROOT, HOST), "--plan", plan_path, "--out", out, "--adapter", adapter], capture_output=True, text=True, cwd=work, env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"})
print("exit", result.returncode, "scenarios", len(scenarios))
print(result.stdout[-400:], result.stderr[-1500:])
rows_out = [json.loads(line) for line in open(out, encoding="utf-8") if line.strip()]
status = {}
for row in rows_out:
    status[row["status"]] = status.get(row["status"], 0) + 1
print("results", len(rows_out), status)

assert os.path.realpath(work).startswith(os.path.realpath(os.path.join(ROOT, '.🧬semio'))), work
shutil.rmtree(work)
