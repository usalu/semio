"""📏️ Quantity-kind oracle: every row of the fixture is declared in the TypeScript, GraphQL and proto twins and its SI unit is the factor-one unit of its family in the units table."""
import json
import re
import sys
from pathlib import Path

root = Path(__file__).resolve().parent.parent
rows = json.loads((root / "🧪️tests/🧫️fixtures/📏️quantity-kinds/🔣️.json").read_text("utf-8"))["rows"]
ts = (root / "🧬️schema/🟦️.ts").read_text("utf-8")
graphql = (root / "🧬️schema/🔗️.graphql").read_text("utf-8")
proto = (root / "🧬️schema/🛰️.proto").read_text("utf-8")
units = (root.parent / "⚖️compliance/📐️quantities-clauses-and-text/🔢️units/🦀️.rs").read_text("utf-8")
failures = []
for row in rows:
    if f'"{row["kind"]}"' not in ts:
        failures.append(f'{row["kind"]}: missing in TypeScript')
    if not re.search(rf"^  {row['graphql']}$", graphql, re.M):
        failures.append(f'{row["kind"]}: missing in GraphQL')
    if not re.search(rf"^  {row['proto']} = \d+;$", proto, re.M):
        failures.append(f'{row["kind"]}: missing in proto')
    unit = re.search(rf'"{re.escape(row["siUnit"])}", "[^"]*", {row["family"]}, 1\.0, 0\.0,', units)
    if not unit:
        failures.append(f'{row["kind"]}: no factor-one unit {row["siUnit"]} in family {row["family"]}')
numbers = [int(n) for n in re.findall(r"QUANTITY_KIND_\w+ = (\d+);", proto)]
if len(set(numbers)) != len(numbers):
    failures.append("duplicate proto numbers")
json.dump({"failures": failures, "rows": len(rows)}, sys.stdout)
sys.exit(1 if failures else 0)
