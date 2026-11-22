"""🎛️ Cross-check VDI 3805 Part 23 correction fields in Rust and SQLite."""

import csv
import json
from pathlib import Path
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
workspace = next(parent.parent for parent in ticket.parents if parent.name == ".🧬semio")
family = workspace / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805"
artifact = next(family.glob("🏅*/🔖*/🪆*/✳*"))
module = artifact / "🧬️schema/💡️inferences/🧱️sheet-checks/🎛️ventilation-unit-fields/🦀️.rs"
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
runner = generated / "vdi3805-part23-corrections.rs"
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

with (ticket / "vdi3805-part23-correction-cases.tsv").open(newline="", encoding="utf-8") as f:
    cases = list(csv.DictReader(f, delimiter="\t"))
rust = subprocess.check_output(
    [binary], text=True, input="".join(
        "\t".join([case["date"], case["record"], *json.loads(case["fields"])]) + "\n"
        for case in cases
    ),
).splitlines()
assert len(rust) == len(cases)

rules = {
    "710.03": (202001, 99999, [(3, "drive"), (4, "stages")]),
    "710.17": (202001, 99999, [(5, "number")]),
    "710.11": (202501, 2147483647, [
        *[(position, "number") for position in range(3, 14)],
        (14, "blank"), (15, "curve"), (16, "number"), (17, "number"),
    ]),
}
with sqlite3.connect(":memory:") as db:
    db.execute("CREATE TABLE rule (position INTEGER PRIMARY KEY, kind TEXT)")
    for case, observed in zip(cases, rust):
        first_date, maximum, fields = rules[case["record"]]
        expected = case["bad_positions"]
        if int(case["date"]) < first_date:
            assert observed == expected == "scope", case
            continue
        db.execute("DELETE FROM rule")
        db.executemany("INSERT INTO rule VALUES (?,?)", [(1, "family"), (2, "index"), *fields])
        independent = db.execute("""
            WITH values_to_verify AS (
              SELECT position, kind, trim(document.value) AS entry
              FROM rule LEFT JOIN json_each(:fields) AS document ON document.key = position - 1
            ), typed AS (
              SELECT position, kind, entry,
                CASE WHEN json_valid(entry) THEN json_type(entry) END AS scalar
              FROM values_to_verify
            )
            SELECT position FROM typed WHERE NOT coalesce(CASE kind
              WHEN 'family' THEN entry = :family
              WHEN 'index' THEN scalar = 'integer' AND CAST(entry AS INTEGER) BETWEEN 1 AND :maximum
              WHEN 'drive' THEN entry IN ('AC','DC','EC')
              WHEN 'stages' THEN scalar = 'integer' AND CAST(entry AS INTEGER) BETWEEN 0 AND 2147483647
              WHEN 'curve' THEN scalar = 'integer' AND CAST(entry AS INTEGER) BETWEEN 1 AND 5
              WHEN 'blank' THEN entry = ''
              WHEN 'number' THEN entry = '' OR (
                scalar IN ('integer','real') AND abs(CAST(entry AS REAL)) <= 1.7976931348623157e308
              )
            END,0) ORDER BY position
        """, dict(fields=case["fields"], family=case["record"], maximum=maximum)).fetchall()
        comparison = ",".join(str(row[0]) for row in independent) or "-"
        assert observed == comparison == expected, (case, observed, comparison)

print(f"[DEBUG] Rust and SQLite matched all {len(cases)} Part 23 correction cases.")
