"""🧪 Verify Annex B unoccupied airflow vectors using production Rust and SQLite."""

import csv
import math
from pathlib import Path
import sqlite3
import subprocess


vectors = Path(__file__).with_name("din16798-unoccupied-ventilation-cases.tsv")
with vectors.open(newline="", encoding="utf-8") as source:
    rows = list(csv.DictReader(source, delimiter="\t"))

ticket = vectors.resolve().parent
workspace = next(parent.parent for parent in ticket.parents if parent.name == ".🧬semio")
family = workspace / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798"
root = next(family.glob("🏅*/🔖*/🪆*/✳*"))
kernel = root / "🧬️schema/💡️inferences/⚖️checks/🌙️unoccupied-ventilation/🦀️.rs"
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
runner = generated / "din16798-unoccupied-airflow.rs"
binary = runner.with_suffix(".exe")
relative_kernel = Path("../" * len(generated.relative_to(workspace).parts)) / kernel.relative_to(workspace)
runner.write_text(
    f'#[path = "{relative_kernel.as_posix()}"]\nmod assessment;\n'
    + r"""
use assessment::{AirflowAssessment, AirflowEvidence, assess_unoccupied_airflow};
use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split('\t').collect();
        let number = |index: usize| fields[index].parse::<f64>().unwrap();
        let evidence = match (fields[1], fields[2]) {
            ("commercial", "shutdown") => AirflowEvidence::NonResidentialPurge {
                zone_volume_m3: number(4),
                minutes_before_occupancy: number(6),
            },
            ("commercial", "setback") => AirflowEvidence::NonResidentialSetback {
                room_area_m2: number(3),
            },
            ("dwelling", "setback") => AirflowEvidence::ResidentialSetback {
                dwelling_area_m2: number(3),
                basis_l_s_m2: fields[7].parse().ok(),
            },
            ("dwelling", "shutdown") => AirflowEvidence::ResidentialShutdown,
            _ => panic!("unsupported fixture: {line}"),
        };
        match assess_unoccupied_airflow(fields[0] == "EN", evidence, number(5)) {
            AirflowAssessment::Checked { required_m3_h, meets } => {
                let status = if meets { "pass" } else { "fail" };
                println!("{status}\t{required_m3_h}");
            }
            AirflowAssessment::MissingPurge => println!("fail\t"),
            AirflowAssessment::Unverified => println!("unverified\t"),
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
inputs_by_row = ("annex", "building", "mode", "floor_m2", "volume_m3",
                 "flow_m3_h", "minutes_before", "residential_lpsm2")
rust_output = subprocess.check_output(
    [binary], text=True,
    input="".join("\t".join(row[name] for name in inputs_by_row) + "\n" for row in rows),
).splitlines()
assert len(rust_output) == len(rows)

query = """
WITH design_basis AS (
    SELECT CASE
        WHEN :annex <> 'EN' OR :flow_m3_h < 0 THEN NULL
        WHEN :building = 'commercial' AND :mode = 'shutdown'
             AND :minutes_before > 0 AND :volume_m3 > 0
            THEN 60000.0 * :volume_m3 / min(:minutes_before, 120)
        WHEN :building = 'commercial' AND :mode = 'setback' AND :floor_m2 > 0
            THEN 540.0 * :floor_m2
        WHEN :building = 'dwelling' AND :mode = 'setback' AND :floor_m2 > 0
             AND :residential_lpsm2 BETWEEN 0.10 AND 0.15
            THEN 3600.0 * :floor_m2 * :residential_lpsm2
    END AS litres_per_hour
)
SELECT CASE
    WHEN :annex = 'EN' AND :building = 'commercial' AND :mode = 'shutdown'
         AND :minutes_before = 0 AND :volume_m3 > 0 AND :flow_m3_h >= 0 THEN 'fail'
    WHEN litres_per_hour IS NULL THEN 'unverified'
    WHEN round(1000.0 * :flow_m3_h, 8) >= round(litres_per_hour, 8) THEN 'pass'
    ELSE 'fail'
END, litres_per_hour / 1000.0
FROM design_basis
"""

with sqlite3.connect(":memory:") as db:
    for row, rust in zip(rows, rust_output):
        inputs = {
            name: float(row[name]) if row[name] else None
            for name in ("floor_m2", "volume_m3", "flow_m3_h", "minutes_before", "residential_lpsm2")
        }
        inputs.update({name: row[name] for name in ("annex", "building", "mode")})
        sql_result = db.execute(query, inputs).fetchone()
        rust_status, rust_rate = rust.split("\t")
        rust_result = rust_status, float(rust_rate) if rust_rate else None
        for status, required in (sql_result, rust_result):
            assert status == row["status"], (row["case"], status, row["status"])
            if row["required_m3h"]:
                assert required is not None and math.isclose(
                    required, float(row["required_m3h"]), rel_tol=1e-12, abs_tol=1e-12
                ), (row["case"], required, row["required_m3h"])
            else:
                assert required is None, (row["case"], required)

print(f"[DEBUG] Production Rust and SQLite matched all {len(rows)} DIN EN 16798-1 unoccupied airflow vectors.")
