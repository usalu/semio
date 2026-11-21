#!/usr/bin/env python3
"""🌬️ Cross-check GModG §26 pressure-test fixtures with SQLite and production Rust."""
import csv
import json
import re
import shutil
import sqlite3
import subprocess
from pathlib import Path

from jsonschema import Draft202012Validator


artifact = Path("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108")
root = next(artifact.glob("*standards/*1/*subsets/*any"))
parent = root / "🧬️schema/💡️inferences/⚖️checks/🌬️part-7-airtightness"
checks = parent / "📏️pressure-test"
work = Path(__file__).resolve().parent / "🗑️generated"
work.mkdir(exist_ok=True)
volume_limit = re.search(r"pub fn n50_limit\([^\n]+\{.*?\n\}", (parent / "🦀️.rs").read_text(), re.S).group(0)
source = work / "din4108_pressure_test.rs"
module = (checks / "🦀️.rs").read_text().replace('#[path = "📇️catalogue/🦀️.rs"]\npub mod catalogue;\n','')
module = module.replace('"🧪️tests/🦀️.rs"', json.dumps(str((checks / "🧪️tests/🦀️.rs").absolute()), ensure_ascii=False))
source.write_text(volume_limit + "\nmod pressure_test {\n" + module + "\n}\n")
exe = source.with_suffix(".exe")
subprocess.run([shutil.which("rustc"), "--edition=2021", "--test", str(source), "-o", str(exe)], check=True)
subprocess.run([exe], check=True)

schema = json.loads((checks / "🔣️.json").read_text())
Draft202012Validator.check_schema(schema)
validator = Draft202012Validator(schema)
reference = sqlite3.connect(":memory:")
query = """WITH input AS (SELECT
       ? AS before_completion, ? AS annex_na, ? AS volume, ? AS mechanical,
       ? AS positive, ? AS negative, ? AS basis, ? AS area, ? AS scope,
       ? AS non_residential, ? AS only_selected, ? AS different_requirements,
       ? AS identical, ? AS exterior_only, ? AS annex_nb),
classification AS (SELECT *,
  CASE WHEN NOT (before_completion AND annex_na) THEN 'MeasurementBasis'
       WHEN NOT (scope = 'whole_building'
                 OR (scope = 'non_residential_zone' AND non_residential AND (only_selected OR different_requirements))
                 OR (scope = 'exterior_unit_sample' AND identical AND exterior_only AND annex_nb)) THEN 'InvalidTestScope'
       WHEN volume <= 0 THEN 'ConditionedVolume'
       WHEN min(positive, negative) < 0 THEN 'MeasuredAirflow'
       WHEN basis = 'envelope_area' AND volume <= 1500 THEN 'IneligibleAreaMethod'
       WHEN basis = 'envelope_area' AND area <= 0 THEN 'EnvelopeArea'
       ELSE 'ok' END AS outcome,
  CASE WHEN basis = 'volume' THEN volume * CASE WHEN mechanical THEN 1.5 ELSE 3 END
       ELSE area * CASE WHEN mechanical THEN 2.5 ELSE 4.5 END END AS limit_m3_h
  FROM input)
SELECT outcome, limit_m3_h, positive <= limit_m3_h, negative <= limit_m3_h FROM classification"""
with (checks / "🧪️tests/pressure-tests.tsv").open() as fixture:
    rows = list(csv.reader(fixture, delimiter="\t"))[1:]
for row in rows:
    args = [float(value) if index not in (6, 8) else value for index, value in enumerate(row[1:16])]
    outcome, limit, positive, negative = reference.execute(query, args).fetchone()
    assert outcome == row[16], (row[0], outcome, row[16])
    if outcome == "ok":
        assert (limit, positive, negative) == (float(row[17]), int(row[18]), int(row[19])), row[0]
    sample = {
        "before_completion": bool(args[0]), "annex_na_confirmed": bool(args[1]), "conditioned_volume_m3": args[2],
        "mechanical_ventilation": bool(args[3]), "pressurisation_m3_h": args[4], "depressurisation_m3_h": args[5],
        "basis": {"kind": args[6]}, "scope": {"kind": args[8]},
    }
    if args[6] == "envelope_area":
        sample["basis"]["envelope_area_m2"] = args[7]
    if args[8] == "non_residential_zone":
        sample["scope"].update(non_residential=bool(args[9]), only_selected_zones=bool(args[10]), different_requirements=bool(args[11]))
    elif args[8] == "exterior_unit_sample":
        sample["scope"].update(identical_units=bool(args[12]), exterior_access_only=bool(args[13]), annex_nb_confirmed=bool(args[14]))
    assert validator.is_valid(sample) == (outcome not in ("ConditionedVolume", "EnvelopeArea", "MeasuredAirflow")), row[0]
print(f"[DEBUG] GModG §26 JSON Schema, Rust production evaluator, and SQLite agreed on {len(rows)} cases")
