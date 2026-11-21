"""Compare the EN 1999 fire temperature-domain rules and thermal elongation with SQLite."""

import csv
import math
import os
from pathlib import Path
import sqlite3
import subprocess

TICKET = Path(__file__).resolve().parent
ROOT = next(p.parent for p in TICKET.parents if p.name == ".🧬semio")
STANDARD = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCHEMA = STANDARD / "🧬️schema"
FIXTURE = SCHEMA / "🧪️tests/⚖️compliance/🔬️resistance-worked-examples/thermal-effects.tsv"
RUST = SCHEMA / "💡️inferences/⚖️checks/📏️parts/🔥️fire/🦀️.rs"
OUTPUT = TICKET / "🗑️generated"


def main():
    with FIXTURE.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream, delimiter="\t"))
    source = RUST.read_text(encoding="utf-8")
    implementation = source[source.index("/// 📐 Relative thermal elongation"):source.index("/// 🌡️ Specific heat of aluminium")]
    inputs = ", ".join(f'("{row["model"]}", {float(row["input_a"])!r}, {float(row["input_b"])!r})' for row in rows)
    driver = """
fn main() {
    for (model, a, b) in [""" + inputs + """] {
        if model == "elongation" {
            match thermal_elongation(a) {
                None => println!("unavailable"),
                Some(value) => println!("{value}"),
            }
        } else {
            println!("{}", if transient_creep_requires_model(a, |_| b) {1} else {0});
        }
    }
}
"""
    harness = OUTPUT / "en1999-thermal-effects.rs"
    executable = OUTPUT / ("en1999-thermal-effects.exe" if os.name == "nt" else "en1999-thermal-effects")
    harness.write_text(implementation + driver, encoding="utf-8")
    subprocess.run(["rustc", "--edition=2024", str(harness), "-o", str(executable)], check=True)
    actual_values = subprocess.check_output([str(executable)], text=True).splitlines()
    assert len(actual_values) == len(rows)
    with sqlite3.connect(":memory:") as db:
        for row, actual in zip(rows, actual_values):
            model, a, b = row["model"], float(row["input_a"]), float(row["input_b"])
            expected, = db.execute("""
                WITH inputs AS (SELECT ? model, ? a, ? b)
                SELECT CASE WHEN model = 'creep' THEN a > 1800 AND b > 170
                    WHEN a > 0 AND a < 500 THEN (a*a + 2250*a - 45000.0) / 100000000.0 END
                FROM inputs
            """, (model, a, b)).fetchone()
            if expected is None:
                assert actual == row["expected"] == "unavailable", (row, actual)
            else:
                assert math.isclose(float(actual), expected, abs_tol=1e-12)
                assert math.isclose(float(row["expected"]), expected, abs_tol=1e-12)
    print(f"[DEBUG] {len(rows)} fire elongation and creep exposure cases matched Rust and SQLite.")


if __name__ == "__main__":
    main()
