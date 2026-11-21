"""Compare the EN 1998 horizontal cable stiffness against SQLite."""

import csv
from itertools import product
from math import isclose
import os
from pathlib import Path
import sqlite3
import subprocess

TICKET = Path(__file__).resolve().parent
ROOT = next(p.parent for p in TICKET.parents if p.name == ".🧬semio")
SCHEMA = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
FIXTURE = SCHEMA / "💡️inferences/📇️catalogue/🧪️tests/🔬️unit/guy-cable-stiffness.tsv"
RUST = SCHEMA / "📐️formulas/🦀️.rs"
OUTPUT = TICKET / "🗑️generated"

QUERY = """
WITH inputs(l, a, slope, modulus, wrap, weight, tension) AS (SELECT ?,?,?,?,?,?,?),
equilibrium AS (SELECT l, a, COS(RADIANS(slope)) AS direction,
    modulus * POW(COS(RADIANS(wrap)), 3) AS axial, weight, tension
    FROM inputs WHERE l>0 AND a>0 AND slope>=0 AND slope<90 AND modulus>0 AND wrap>=0 AND wrap<90 AND weight>=0 AND tension>0)
SELECT a * direction * direction / l / (1/axial + POW(weight*l,2)/(12*POW(tension,3))) FROM equilibrium
"""


def main():
    with FIXTURE.open(newline="", encoding="utf-8") as stream:
        fixture = [tuple(map(float, row)) for row in list(csv.reader(stream, delimiter="\t"))[1:]]
    inputs = [row[:-1] for row in fixture]
    inputs.extend((length, 0.001, slope, 200e9, wrap, weight, stress)
        for length, slope, wrap, weight, stress in product([10.0, 40.0, 100.0], [0.0, 45.0, 75.0], [0.0, 20.0],
            [0.0, 78000.0, 120000.0], [50e6, 250e6]))
    inputs.extend([(-1.0, .001, 0.0, 200e9, 0.0, 0.0, 250e6), (40.0, .001, 90.0, 200e9, 0.0, 0.0, 250e6),
        (40.0, .001, 0.0, 200e9, 0.0, 78000.0, 0.0)])
    source = RUST.read_text(encoding="utf-8")
    implementation = source[source.index("    pub fn guy_cable_stiffness_n_m"):source.index("    /// 🗼 Fundamental period of a uniform cantilever")]
    samples = ",".join(str(case) for case in inputs)
    driver = """
fn main() {
    for (length, area, slope, modulus, wrap, weight, tension) in [""" + samples + """] {
        match guy_cable_stiffness_n_m(length, area, slope, modulus, wrap, weight, tension) {
            Some(value) => println!("{value}"),
            None => println!("unavailable"),
        }
    }
}
"""
    harness = OUTPUT / "en1998-guy-cable.rs"
    binary = OUTPUT / ("en1998-guy-cable.exe" if os.name == "nt" else "en1998-guy-cable")
    harness.write_text(implementation + driver, encoding="utf-8")
    subprocess.run(["rustc", "--edition=2024", str(harness), "-o", str(binary)], check=True)
    results = subprocess.check_output([str(binary)], text=True).splitlines()
    assert len(results) == len(inputs)
    with sqlite3.connect(":memory:") as db:
        for index, (case, value) in enumerate(zip(inputs, results)):
            result = db.execute(QUERY, case).fetchone()
            if result is None:
                assert value == "unavailable", (case, value)
                continue
            assert isclose(result[0], float(value), rel_tol=1e-12), (case, value, result)
            if index < len(fixture):
                assert isclose(result[0], fixture[index][-1], rel_tol=1e-12), (case, fixture[index], result)
    print(f"[DEBUG] {len(inputs)} guy-cable stiffness cases matched Rust and SQLite.")


if __name__ == "__main__":
    main()
