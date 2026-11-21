"""Compare the production thermal-bridge flat rates against an SQLite model."""

import csv
import json
import os
from pathlib import Path
import sqlite3
import subprocess

TICKET = Path(__file__).resolve().parent
ROOT = next(p.parent for p in TICKET.parents if p.name == ".🧬semio")
SCHEMA = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
FIXTURE = SCHEMA / "🧪️tests/⚖️compliance/thermal-bridge-flat-rates.tsv"
RUST = SCHEMA / "💡️inferences/⚖️checks/🌉️bb-2-thermal-bridges/🦀️.rs"
OUTPUT = TICKET / "🗑️generated"


def main():
    with FIXTURE.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream, delimiter="\t"))
    production = RUST.read_text(encoding="utf-8")
    selector = production[production.index("pub const DELTA_U_WB_CONFORM"):production.index("/// 📐️ ΔU_WB =")]
    fixture = [
        (row["declared_categories"].split(",") if row["declared_categories"] else [], row["equivalence_confirmed"])
        for row in rows
    ]
    inputs = ",\n".join(f"(&{json.dumps(categories)}, {confirmed})" for categories, confirmed in fixture)
    driver = """
pub struct ThermalBridge {
    bb2_type: String,
}

fn main() {
    let inputs: &[(&[&str], bool)] = &[
""" + inputs + """
    ];
    for (categories, confirmed) in inputs {
        let bridges = categories.iter().map(|&value| ThermalBridge {
            bb2_type: value.into()
        }).collect::<Vec<_>>();
        println!("{}", delta_u_wb_limit(&bridges, *confirmed));
    }
}
"""
    OUTPUT.mkdir(exist_ok=True)
    harness = OUTPUT / "din4108-flat-rate.rs"
    executable = OUTPUT / ("din4108-flat-rate.exe" if os.name == "nt" else "din4108-flat-rate")
    harness.write_text(selector + driver, encoding="utf-8")
    subprocess.run(["rustc", "--edition=2024", str(harness), "-o", str(executable)], check=True)
    actual = subprocess.check_output([str(executable)], text=True).splitlines()
    assert len(actual) == len(rows)
    with sqlite3.connect(":memory:") as db:
        db.execute("CREATE TABLE selected_categories (category TEXT)")
        for (categories, confirmed), row, value in zip(fixture, rows, actual):
            db.execute("DELETE FROM selected_categories")
            db.executemany("INSERT INTO selected_categories VALUES (?)", ((category,) for category in categories))
            expected, = db.execute("""
                SELECT CASE
                    WHEN ? = 'false' OR COUNT(*) = 0 THEN 0.10
                    WHEN SUM(category IN ('categoryA', 'categoryB')) < COUNT(*) THEN 0.10
                    WHEN SUM(category = 'categoryB') = COUNT(*) THEN 0.03
                    ELSE 0.05 END
                FROM selected_categories
            """, (confirmed,)).fetchone()
            assert expected == float(value) == float(row["delta_u_w_m2_k"]), (row, value)
    print(f"[DEBUG] {len(rows)} DIN 4108 thermal-bridge cases matched Rust and SQLite.")


if __name__ == "__main__":
    main()
