"""Verify Tables 4.1 and 4.2 and the production EN 1994 beam design case."""

import csv
from itertools import product
import json
import math
import os
from pathlib import Path
import sqlite3
import subprocess

TICKET = Path(__file__).resolve().parent
ROOT = next(p.parent for p in TICKET.parents if p.name == ".🧬semio")
SUBSET = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any"
CHECKS = SUBSET / "🧬️schema/💡️inferences/⚖️checks"
TABLE = CHECKS / "🧮️catalogue/🔥️beam-fire-tables"
OUTPUT = TICKET / "🗑️generated"


def read_rows(name):
    with (TABLE / "🧪️tests" / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream, delimiter="\t"))


def compare(db, width, rating):
    if width < 170 or rating == 30:
        return "unavailable"
    width = min(width, 300)
    record = db.execute("""
        WITH lower AS (SELECT * FROM table_axes WHERE class=? AND width<=? ORDER BY width DESC LIMIT 1),
        upper AS (SELECT * FROM table_axes WHERE class=? AND width>=? ORDER BY width LIMIT 1)
        SELECT (CASE WHEN lower.width=upper.width THEN lower.u1
                ELSE lower.u1 + (?-lower.width) * (upper.u1-lower.u1)/(upper.width-lower.width) END) / 1000.0,
               (CASE WHEN lower.width=upper.width THEN lower.u2
                ELSE lower.u2 + (?-lower.width) * (upper.u2-lower.u2)/(upper.width-lower.width) END) / 1000.0
        FROM lower CROSS JOIN upper
    """, (rating, width, rating, width, width, width)).fetchone()
    return "unavailable" if record is None else record


def main():
    table_rows = read_rows("table-4-1.tsv")
    axis_rows = read_rows("table-4-2.tsv")
    rating = (30, 60, 90, 120, 180)
    beam = [(float(row["load_ratio"]), float(row["min_h_over_b"]), value, row[f"R{value}"])
            for row, value in product(table_rows, rating)]
    axes = [(int(row["width_mm"]), value, row[f"R{value}"]) for row, value in product(axis_rows, rating[1:])]
    selections = [(value, width) for value, width in product(rating, (160, 170, 185, 199, 200, 225, 249, 250, 275, 300, 345))]
    exposed = (TABLE / "🦀️.rs").read_text(encoding="utf-8").split("#[cfg(test)]")[0]
    source = (CHECKS / "📇️catalogue/🧮️evaluators/🦀️.rs").read_text(encoding="utf-8")
    rows = source[source.index("/// 📏️ One verification row"):source.index("/// 🔥 Table 4.6 for a partially encased column")]
    parser = source[source.index("fn column_fire_rating"):source.index("/// 🏙️ Table 4.7")]
    evaluation = source[source.index("pub fn partially_encased_beam_fire_table"):source.index("/// 🔩 EN 1994-1-1 §6.6.4.2")]
    demo_path = SUBSET / "🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json"
    seed = next(row for row in json.loads(demo_path.read_text())["cases"] if row["entryId"] == "C94-18")
    numbers = {row["id"]: row["value"] for row in seed["numbers"]}
    choices = {row["id"]: row["value"] for row in seed["texts"]}
    cases = [
        ({}, {}, "pass"), ({"fire_beam_ambient_u1":0.07}, {}, "fail"), ({"fire_beam_fy":460e6}, {}, "fail"),
        ({"fire_beam_as":0.0}, {}, "fail"), ({"fire_beam_deck_coverage":0.849}, {"fire_beam_deck":"profiled"}, "unavailable"),
        ({"fire_beam_deck_coverage":0.85}, {"fire_beam_deck":"profiled"}, "pass"),
        ({"fire_beam_deck_coverage":0.849}, {"fire_beam_deck":"closed_voids"}, "pass"),
        ({"fire_beam_slab":0.119}, {}, "unavailable"), ({"fire_beam_slab":0.120}, {}, "pass"),
        ({"fire_beam_width":0.16,"fire_beam_web":0.01}, {}, "unavailable"),
        ({"fire_beam_as":0.0,"fire_beam_load_level":0.30}, {"fire_col_ext_rating":"R30"}, "pass"),
        ({"fire_beam_as":0.0,"fire_beam_load_level":0.71}, {}, "unavailable"),
        ({"fire_beam_width":0.225,"fire_beam_u1":0.0874,"fire_beam_u2":0.06}, {}, "fail"),
        ({"fire_beam_width":0.225,"fire_beam_u1":0.0875,"fire_beam_u2":0.06}, {}, "pass"),
    ]
    setup = """
#![allow(dead_code)]
use std::collections::BTreeMap;
#[derive(Clone, Copy)] pub enum QuantityKind { Dimensionless }
pub struct Values { numbers: BTreeMap<String, f64>, texts: BTreeMap<String, String> }
pub struct DesignCase { values: Values }
fn assess(case: DesignCase) {
    match partially_encased_beam_fire_table(&Inputs(&case)) {
        Err(_) => println!("unavailable"),
        Ok(rows) if rows.iter().all(|row| row.demand <= row.capacity + 1e-10) => println!("pass"),
        Ok(_) => println!("fail"),
    }
}
"""
    table_checks = """
for row in formulas::beam_fire_tables::BEAM_FIRE_ALTERNATIVES {
    for value in row.rating_cells {
        match value { Some((width, ratio)) => println!("{width}/{ratio}"), None => println!("-") }
    }
}
for (_, cells) in formulas::beam_fire_tables::BEAM_FIRE_AXES_MM {
    for value in cells {
        match value { Some((horizontal, vertical)) => println!("{horizontal}/{vertical}"), None => println!("-") }
    }
}
"""
    commands = []
    for num, text, _ in cases:
        ordered_num = ",".join(f"({json.dumps(k)}.into(),{float(v)!r})" for k, v in {**numbers, **num}.items())
        ordered_text = ",".join(f"({json.dumps(k)}.into(),{json.dumps(v)}.into())" for k, v in {**choices, **text}.items())
        commands.append(f"    assess(DesignCase {{ values: Values {{ numbers: [{ordered_num}].into(), texts: [{ordered_text}].into() }} }});")
    queries = ", ".join(f"({r}, {w / 1000!r})" for r, w in selections)
    driver = "fn main() {\n" + table_checks + """
    for (rating, width) in [""" + queries + """] {
        match formulas::beam_fire_tables::beam_reinforcement_axes(rating, width) {
            Some((u1, u2)) => println!("{u1} {u2}"),
            None => println!("unavailable"),
        }
    }
""" + "\n".join(commands) + "\n}\n"
    harness = OUTPUT / "en1994-beam-fire.rs"
    binary = OUTPUT / ("en1994-beam-fire.exe" if os.name == "nt" else "en1994-beam-fire")
    harness.write_text(setup + "mod formulas { pub mod beam_fire_tables {\n" + exposed + "} }\n" + rows + parser + evaluation + driver, encoding="utf-8")
    subprocess.run(["rustc", "--edition=2024", str(harness), "-o", str(binary)], check=True)
    actual = subprocess.check_output([str(binary)], text=True).splitlines()
    assert len(actual) == len(beam) + len(axes) + len(selections) + len(cases)
    for row, item in zip(beam + axes, actual):
        expected = row[-1]
        if expected == "-":
            assert item == expected
        else:
            assert list(map(float, item.split("/"))) == list(map(float, expected.split("/"))), row
    axis_selection = actual[len(beam) + len(axes):]
    with sqlite3.connect(":memory:") as db:
        db.execute("CREATE TABLE table_axes(width INTEGER, class INTEGER, u1 REAL, u2 REAL)")
        db.executemany("INSERT INTO table_axes VALUES (?,?,?,?)", [(w, r, *(float(x) for x in pair.split("/")))
            for w, r, pair in axes if pair != "-"])
        for (value, width), item in zip(selections, axis_selection):
            expected = compare(db, width, value)
            if expected == "unavailable":
                assert item == expected, (value, width, item)
            else:
                assert all(math.isclose(a,b,abs_tol=1e-12) for a,b in zip(expected,map(float,item.split()))), (value, width, item)
    assert actual[-len(cases):] == [case[-1] for case in cases], list(zip(actual[-len(cases):], cases))
    print(f"[DEBUG] {len(beam)} Table 4.1 cells, {len(axes)} Table 4.2 cells, {len(selections)} selections and {len(cases)} production beam cases matched.")

if __name__ == "__main__":
    main()
