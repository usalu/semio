"""⏱️ Compare DIN EN 16798-1 NA.4.1 hourly comfort summaries against SQLite."""

import csv
import math
from pathlib import Path
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
workspace = next(parent.parent for parent in ticket.parents if parent.name == ".🧬semio")
family = workspace / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798"
artifact = next(family.glob("🏅*/🔖*/🪆*/✳*"))
kernel = artifact / "🧬️schema/💡️inferences/⚖️checks/⏱️hourly-comfort/🦀️.rs"
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
runner = generated / "din16798-hourly-comfort.rs"
binary = runner.with_suffix(".exe")
relative_kernel = Path("../" * len(generated.relative_to(workspace).parts)) / kernel.relative_to(workspace)
runner.write_text(
    f'#[path = "{relative_kernel.as_posix()}"]\nmod comfort;\n'
    + r"""
use comfort::{OccupiedHour, hourly_comfort};
use std::io::{self, BufRead};

fn main() {
    let source = io::stdin().lock();
    let hours = source.lines().flat_map(|line| {
        let line = line.unwrap();
        let fields: Vec<_> = line.split('\t').collect();
        let repeat = fields[0].parse::<usize>().unwrap();
        let hour = OccupiedHour {
            outdoor_mean_c: fields[1].parse().unwrap(),
            operative_c: fields[2].parse().unwrap(),
            occupied_fraction: fields[3].parse().unwrap(),
        };
        std::iter::repeat_n(hour, repeat)
    });
    if let Some(outcome) = hourly_comfort(hours) {
        let v = [
            outcome.occupied_hours,
            outcome.colder.hours, outcome.colder.degree_hours, outcome.colder.greatest_k,
            outcome.warmer.hours, outcome.warmer.degree_hours, outcome.warmer.greatest_k,
            outcome.category_ii_degree_hour_allowance(),
        ];
        println!("{}", v.map(|x| x.to_string()).join("\t"));
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

scenarios = {}
with (ticket / "din16798-hourly-comfort-cases.tsv").open(newline="", encoding="utf-8") as f:
    for name, *data in list(csv.reader(f, delimiter="\t"))[1:]:
        scenarios.setdefault(name, []).append(data)

query = """
WITH band AS (
    SELECT n * part AS occupation, n, part,
        CASE WHEN outdoor < 16 THEN 22 WHEN outdoor > 32 THEN 26 ELSE (outdoor + 72) / 4.0 END AS center,
        operative, outdoor
    FROM samples WHERE part <> 0
), differences AS (
    SELECT occupation, max(center - 2.0 - operative, 0.0) AS cold,
        max(operative - center - 2.0, 0.0) AS warm,
        CASE WHEN part < 0 OR part > 1 OR operative IS NULL OR outdoor IS NULL THEN 1 ELSE 0 END AS invalid
    FROM band
)
SELECT sum(occupation),
    sum(CASE WHEN cold > 0 THEN occupation ELSE 0 END), sum(cold * occupation), max(cold),
    sum(CASE WHEN warm > 0 THEN occupation ELSE 0 END), sum(warm * occupation), max(warm),
    sum(occupation) / 50.0
FROM differences HAVING sum(occupation) > 0 AND sum(invalid) = 0
"""

with sqlite3.connect(":memory:") as db:
    db.execute("CREATE TABLE samples(n INTEGER, outdoor REAL, operative REAL, part REAL)")
    with (ticket / "din16798-hourly-comfort-expected.tsv").open(newline="", encoding="utf-8") as f:
        for name, *expected in list(csv.reader(f, delimiter="\t"))[1:]:
            data = scenarios.pop(name)
            rust = subprocess.check_output(
                [binary], text=True, input="".join("\t".join(row) + "\n" for row in data)
            ).strip()
            db.execute("DELETE FROM samples")
            db.executemany("INSERT INTO samples VALUES (?,?,?,?)", [
                (int(n), float(outdoor), float(operative), float(part))
                for n, outdoor, operative, part in data
            ])
            result = db.execute(query).fetchone()
            if not expected[0]:
                assert not rust and result is None, (name, rust, result)
                continue
            for observed in (list(map(float, rust.split("\t"))), result):
                assert observed is not None and len(observed) == len(expected), (name, observed)
                assert all(
                    math.isclose(actual, float(target), rel_tol=1e-12, abs_tol=1e-12)
                    for actual, target in zip(observed, expected)
                ), (name, observed, expected)
    assert not scenarios, scenarios

print("[DEBUG] Production hourly comfort kernel and SQLite matched all seven DIN EN 16798-1 vectors.")
