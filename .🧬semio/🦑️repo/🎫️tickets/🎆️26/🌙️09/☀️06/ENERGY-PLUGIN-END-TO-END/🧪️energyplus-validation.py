"""⚡ Validate committed energy exchanges against the official EnergyPlus engine and schema."""

import argparse
import json
from pathlib import Path
import sqlite3
import subprocess

import jsonschema


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--energyplus-root", type=Path, required=True)
    args = parser.parse_args()
    ticket = Path(__file__).resolve().parent
    root = next(path for path in ticket.parents if (path / "AGENTS.md").exists())
    energy = root / "✏️s/🔌️plugins/🔋️energy"
    subset = energy / "🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
    output = ticket / "🗑️generated/energyplus-validation"
    output.mkdir(parents=True, exist_ok=True)
    tool = args.energyplus_root.resolve()
    binary = tool / "energyplus"
    schema = json.loads((tool / "Energy+.schema.epJSON").read_text())
    validator = jsonschema.Draft7Validator(schema)
    weather = subset / "🧪️tests/🧫️fixtures/🌦️denver-tmy/🌦️.epw"
    rows = []
    for source in sorted(subset.rglob("*.epJSON")):
        document = json.loads(source.read_text())
        validator.validate(document)
        destination = output / source.parent.name
        destination.mkdir(exist_ok=True)
        result = subprocess.run([str(binary), "-a", "-w", str(weather), "-d", str(destination), str(source)], capture_output=True, text=True, timeout=300)
        (destination / "process.log").write_text(result.stdout + result.stderr)
        if result.returncode:
            raise AssertionError(f"EnergyPlus refused {source}: {(destination / 'eplusout.err').read_text()}")
        with sqlite3.connect(destination / "eplusout.sql") as database:
            totals = database.execute("SELECT d.Name, SUM(r.Value)/3600000 FROM ReportData r JOIN ReportDataDictionary d ON r.ReportDataDictionaryIndex=d.ReportDataDictionaryIndex WHERE d.Name IN ('Zone Ideal Loads Supply Air Total Heating Energy','Zone Ideal Loads Supply Air Total Cooling Energy') GROUP BY d.Name").fetchall()
        rows.append({"fixture": str(source.relative_to(root)), "schema": "valid", "simulation": "successful", "annualKwh": dict(totals)})
    official = subset / "🚪️io/📤️export/⚡️idf/🧪️tests/1ZoneUncontrolled.idf"
    destination = output / "official-idf"
    destination.mkdir(exist_ok=True)
    result = subprocess.run([str(binary), "--convert-only", "-d", str(destination), str(official)], capture_output=True, text=True, timeout=60)
    (destination / "process.log").write_text(result.stdout + result.stderr)
    if result.returncode:
        raise AssertionError(result.stdout + result.stderr)
    converted = json.loads((destination / "1ZoneUncontrolled.epJSON").read_text())
    validator.validate(converted)
    rows.append({"fixture": str(official.relative_to(root)), "schema": "valid", "conversion": "successful"})
    (output / "results.json").write_text(json.dumps(rows, indent=2))
    lines = ["# EnergyPlus Exchange Verification", "", "Official EnergyPlus 25.2.0 and its shipped epJSON schema validated the following committed inputs.", ""]
    for row in rows:
        lines.append(f"- `{row['fixture']}`: {row.get('simulation', row.get('conversion'))}; schema valid.")
        for name, value in row.get("annualKwh", {}).items():
            lines.append(f"  - {name}: {value:.6f} kWh.")
    lines += ["", "These checks prove the committed exchanges are accepted by EnergyPlus. They do not prove numerical agreement of the native simulation or browser integration.", ""]
    (ticket / "📓️energyplus-verification.md").write_text("\n".join(lines))
    print(f"[DEBUG] EnergyPlus accepted {len(rows)} official/native exchange fixtures")


if __name__ == "__main__":
    main()
