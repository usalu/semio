"""Compare Part 22 air-to-air heat-pump records against the VDI corrections and SQLite."""
import csv
import json
from pathlib import Path
import re
import shlex
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
workspace = next(path.parent for path in ticket.parents if path.name == ".🧬semio")
artifact = workspace / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805"
standard = next(artifact.glob("🏅*/🔖*/🪆*/✳*"))
inference = standard / "🧬️schema/💡️inferences"
leaf = inference / "🧱️sheet-checks/🌡️air-to-air-heat-pump"

specification = json.loads((leaf / "📜️record.json").read_text())
schemas = {row["recordFamily"]: row["fields"] for row in specification["records"]}
with (inference / "🧪️tests/🔬️unit/🧫️fixtures/air-to-air-heat-pump-records.tsv").open(newline="") as fixture:
    rows = list(csv.reader(fixture, delimiter="\t"))[1:]

work = ticket / "🗑️generated"
work.mkdir(exist_ok=True)
source = work / "verify-part22-heat-pump-records.rs"
binary = work / "verify-part22-heat-pump-records"
source.write_text(
    f'#[path = "{(leaf / "📏️fields.rs").as_posix()}"]\nmod fields;\n'
    + r"""
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut values = line.split('\t');
        let family = values.next().unwrap();
        let fields: Vec<String> = values.map(str::to_owned).collect();
        match fields::validate_fields(family, &fields).expect("supported family") {
            Ok(()) => println!(),
            Err(fields::RecordError::FieldCount(_)) => println!("count"),
            Err(fields::RecordError::Fields(positions)) => println!("{}", positions.iter().map(usize::to_string).collect::<Vec<_>>().join(",")),
        }
    }
}
"""
)
subprocess.run(
    ["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc", "--edition", "2024", str(source), "-o", str(binary)],
    cwd=workspace, check=True
)
result = subprocess.check_output([binary], text=True, input="".join("\t".join([row[0], *row[3:]]) + "\n" for row in rows)).splitlines()
assert len(result) == len(rows)

db = sqlite3.connect(":memory:")
db.create_function("matches", 2, lambda regex, value: int(re.fullmatch(regex, value) is not None))
number = r"[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?"
integer = r"[+-]?[0-9]+"


def sqlite_errors(record_family, candidate):
    fields = schemas[record_family]
    if len(candidate) != len(fields):
        return "count"
    values = [value.strip() for value in candidate]
    violations = []
    for position, ((_, fmt, rule), value) in enumerate(zip(fields, values), 1):
        if "constant" in rule:
            ok, = db.execute("select ? = ?", (value, rule["constant"])).fetchone()
        elif "codes" in rule:
            codes = rule["codes"]
            slots = ",".join("?" for _ in codes)
            ok, = db.execute(f"select matches(?, ?) and cast(? as integer) in ({slots})", (integer, value, value, *codes)).fetchone()
        elif "codesWhen" in rule:
            condition = rule["codesWhen"]
            mode = values[condition["field"] - 1]
            slots = ",".join("?" for _ in condition["codes"])
            ok, = db.execute(
                f"select case cast(? as integer) when ? then matches(?, ?) and cast(? as integer) in ({slots}) when 2 then ? = '' else true end",
                (mode, condition["code"], integer, value, value, *condition["codes"], value),
            ).fetchone()
        elif "requiredWhen" in rule:
            condition = rule["requiredWhen"]
            trigger = values[condition["field"] - 1]
            ok, = db.execute("select not(matches(?, ?) and cast(? as integer) = ?) or ? <> ''", (integer, trigger, trigger, condition["code"], value)).fetchone()
        elif fmt == "I":
            minimum = rule.get("minimum", -2147483648)
            maximum = rule.get("maximum", 2147483647)
            ok, = db.execute("select matches(?, ?) and cast(? as integer) between ? and ?", (integer, value, value, minimum, maximum)).fetchone()
            ok = ok or (not rule and not value)
        else:
            ok, = db.execute("select ? = '' or (matches(?, ?) and abs(cast(? as real)) <= 1.7976931348623157e308)", (value, number, value, value)).fetchone()
        if not ok:
            violations.append(str(position))
    return ",".join(violations)


for (family, name, expected, *candidate), observed in zip(rows, result):
    assert (sqlite_errors(family, candidate), observed) == (expected, expected), (family, name, expected, observed)
asset = (standard / "🖼️assets/blatt-22/🗣️.dsl.semio").read_text()
records = re.findall(r'family="(710\.1[567])" fields=\[ ([^\]]+) \]', asset)
assert [row[0] for row in records] == list(schemas)
for family, example in records:
    values = shlex.split(example)
    assert any(row[0] == family and not row[2] and row[3:] == values for row in rows)
print(f"[DEBUG] {len(rows)} cases for {len(records)} Part 22 record types matched the production Rust validator and SQLite. The app example contains one accepted record of each type.")
