"""🔠 Verify case-insensitive physical reference identity against EnergyPlus itself."""
from pathlib import Path
import json
import sqlite3
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = next(path for path in ticket.parents if (path / "AGENTS.md").exists())
subset = root / "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
law = json.loads((subset / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🧪️tests/🔬️unit/reference-case.json").read_text())
source = subset / f"🧪️tests/🧫️fixtures/🏛️bestest-{law['case']}/⚡️model.epJSON"
weather = subset / "🧪️tests/🧫️fixtures/🌦️denver-tmy/🌦️.epw"
output = ticket / "🗑️generated/reference-case"
output.mkdir(exist_ok=True)
document = json.loads(source.read_text())
for key, kind in [("zones", "Zone"), ("surfaces", "BuildingSurface:Detailed"), ("fenestrations", "FenestrationSurface:Detailed")]:
    assert len(document[kind]) == law["expected"][key]
for group in document.values():
    for fields in group.values():
        for key, value in fields.items():
            if isinstance(value, str) and (key in law["referenceKeys"] or any(key.startswith(prefix) for prefix in law["referencePrefixes"])):
                fields[key] = value.upper()
mutant = output / "uppercase.epJSON"
mutant.write_text(json.dumps(document))
results = []
for label, path in [("baseline", source), ("uppercase", mutant)]:
    destination = output / label
    destination.mkdir(exist_ok=True)
    result = subprocess.run([str(Path(sys.argv[1]).resolve()), "-a", "-w", str(weather), "-d", str(destination), str(path)], capture_output=True, text=True, timeout=300)
    (destination / "process.log").write_text(result.stdout + result.stderr)
    assert result.returncode == 0, (destination / "eplusout.err").read_text()
    with sqlite3.connect(destination / "eplusout.sql") as database:
        results.append(database.execute("SELECT d.Name, SUM(r.Value) FROM ReportData r JOIN ReportDataDictionary d USING (ReportDataDictionaryIndex) WHERE d.Name IN ('Zone Ideal Loads Supply Air Total Heating Energy','Zone Ideal Loads Supply Air Total Cooling Energy') GROUP BY d.Name ORDER BY d.Name").fetchall())
assert results[0] == results[1], results
(ticket / "📓️reference-case-verification.md").write_text("# Case-Insensitive Energy References\n\nThe shared reference-case.json law uppercases zone, construction, host-surface and layer reference names without changing object names. Official EnergyPlus 25.2 successfully runs both documents and reports exactly equal annual heating and cooling joules. This independently validates the native importer regression expectation; full importer integration tests still require the shared store build.\n")
print("[DEBUG] EnergyPlus reference-case law: equal annual heating and cooling")
