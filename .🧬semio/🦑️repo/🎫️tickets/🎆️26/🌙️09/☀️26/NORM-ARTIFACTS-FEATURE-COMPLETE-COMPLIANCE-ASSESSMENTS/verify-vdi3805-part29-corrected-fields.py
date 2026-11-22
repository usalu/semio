"""🎛️ Cross-check VDI 3805 Part 29 correction fields in Rust and SQLite."""

import csv
import json
from pathlib import Path
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
workspace = next(parent.parent for parent in ticket.parents if parent.name == ".🧬semio")
family = workspace / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805"
artifact = next(family.glob("🏅*/🔖*/🪆*/✳*"))
module = artifact / "🧬️schema/💡️inferences/🧱️sheet-checks/🚰️pipe-record-fields/🦀️.rs"
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
runner = generated / "vdi3805-part29-corrections.rs"
binary = runner.with_suffix(".exe")
relative_module = Path("../" * len(generated.relative_to(workspace).parts)) / module.relative_to(workspace)
shared = relative_module.parents[1] / "📑️corrected-fields/🦀️.rs"
runner.write_text(
    f'#[path = "{shared.as_posix()}"]\nmod fields;\nuse fields::CorrectedFields;\n'
    f'#[path = "{relative_module.as_posix()}"]\nmod correction;\n'
    + r"""
use correction::corrected_fields;
use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut parts = line.split('\t');
        let date = parts.next().unwrap().parse().unwrap();
        let family = parts.next().unwrap();
        let fields: Vec<_> = parts.map(str::to_owned).collect();
        match corrected_fields(family, &fields, date) {
            None => println!("scope"),
            Some(result) if result.invalid.is_empty() => println!("-"),
            Some(result) => println!("{}", result.invalid.iter().map(usize::to_string).collect::<Vec<_>>().join(",")),
        }
    }
}
""",
    encoding="utf-8",
)
subprocess.run(
    ["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc",
     "--edition", "2024", str(runner), "-o", str(binary)],
    check=True, cwd=workspace,
)


fixture = artifact / "🧬️schema/💡️inferences/🧪️tests/🔬️unit/🧫️fixtures/part29-corrected-fields.tsv"
with fixture.open(newline="",encoding="utf-8") as stream:
    cases = list(csv.DictReader(stream,delimiter="\t"))
requests = []
for case in cases:
    fields = [case["family"],"1"] + [""] * (int(case["field_count"]) - 2)
    for override in filter(None,case["overrides"].split(";")):
        position,value = override.split("=",1)
        fields[int(position)-1] = value
    requests.append(fields)
observed = subprocess.check_output([binary], text=True, input="".join(
    "\t".join([case["date"],case["family"],*fields])+"\n" for case,fields in zip(cases,requests)
)).splitlines()
assert len(observed) == len(cases)
rules = {"700":([13],[14]), "710.01":([12,14,15,16],[]), "710.02":([7,9,20],[])}
with sqlite3.connect(":memory:") as database:
    database.execute("create table fields(position integer primary key, kind text)")
    for case,fields,rust in zip(cases,requests,observed):
        family = case["family"]
        expected = case["bad_positions"]
        if int(case["date"]) < 202604 or family not in rules:
            assert rust == expected == "scope", case
            continue
        numeric, text = rules[family]
        database.execute("delete from fields")
        database.executemany("insert into fields values(?,?)", [(1,"family"),(2,"index"),
            *[(position,"numeric") for position in numeric],*[(position,"text") for position in text]])
        reference = database.execute("""
            WITH source AS (
              SELECT position, kind, trim(value) AS cell FROM fields LEFT JOIN json_each(?) ON key = position-1
            )
            SELECT position FROM source
            WHERE NOT CASE
              WHEN cell IS NULL THEN 0
              WHEN kind = 'family' THEN cell = ?
              WHEN kind = 'index' THEN cell <> '' AND cell NOT GLOB '*[^0-9]*' AND CAST(cell AS INTEGER) BETWEEN 1 AND 99999
              WHEN kind = 'numeric' THEN CASE WHEN cell = '' THEN 1
                WHEN json_valid(cell) THEN json_type(cell) IN ('integer','real') AND abs(cast(cell AS REAL)) < 1e999 ELSE 0 END
              ELSE 1 END
            ORDER BY position
        """,(json.dumps(fields),family)).fetchall()
        assert rust == expected == (",".join(str(row[0]) for row in reference) or "-"), case
print(f"[DEBUG] Part 29 revised record formats matched in Rust and SQLite for {len(cases)} vectors.")
