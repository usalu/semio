"""Verify Tables 4.3 to 4.7 and their selectors in the Rust catalogue against SQLite."""

import csv
import json
from math import isclose
import os
from pathlib import Path
import sqlite3
import subprocess


TICKET = Path(__file__).resolve().parent
ROOT = next(parent for parent in TICKET.parents if parent.name == ".🧬semio").parent
TABLE = ROOT / (
    "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/"
    "🧬️schema/💡️inferences/⚖️checks/🧮️catalogue/🔥️column-fire"
)
FIXTURES = TABLE / "🧪️tests"
OUTPUT = TICKET / "🗑️generated"


def rust_table_results():
    source = (TABLE / "🦀️.rs").read_text(encoding="utf-8")
    implementation = source[source.index("/// 📚️ One cell of a tabulated-data table"):source.index("//#region 🧪️Tests")]
    samples = [(rating, load) for rating in (30, 60, 90, 120, 180) for load in (0.20, 0.28, 0.40, 0.47, 0.50, 0.66, 0.80)]
    executable = OUTPUT / ("en1994-fire-table.exe" if os.name == "nt" else "en1994-fire-table")
    harness = OUTPUT / "en1994-fire-table.rs"
    rust_samples = ", ".join(f"({rating}, {load})" for rating, load in samples)
    harness.write_text(implementation + f"\nconst SAMPLES: &[(u32,f64)] = &[{rust_samples}];\n" + """
fn main() {
    for row in ENCASED_COLUMN_TABLE {
        println!("{} {} {} {} {}", row.rating_min, row.alternative, row.min_dimension_m,
            row.min_cover_m, row.min_axis_m);
    }
    for rating in [30, 60, 90, 120, 180, 240, 300] {
        for (size, cover, axis) in [(0.350, 0.050, 0.045), (0.300, 0.050, 0.045), (0.500, 0.080, 0.075)] {
            match check_fully_encased(rating, size, cover, axis) {
                None => println!("unavailable"),
                Some((row, utilization)) => println!("{} {utilization}", row.alternative),
            }
        }
    }
    for table in [COLUMN_TABLE, FILLED_COLUMN_TABLE] {
    for row in table {
        println!("{} {} {:?} {:?} {:?}", row.rating_min, row.load_level,
            row.min_width_m.value, row.min_axis_distance_m.value, row.min_reinforcement_ratio.value);
    }
    for &(rating, load) in SAMPLES {
        match check_tabulated(table, rating, load, 0.35, 0.06, 0.042) {
            Tabulated::NoData => println!("unavailable"),
            Tabulated::Evaluated { utilization, verified } => {
                assert!(verified);
                println!("{utilization}");
            }
        }
    }
    }
}
""", encoding="utf-8")
    subprocess.run(["rustc", "--edition=2024", str(harness), "-o", str(executable)], check=True)
    return samples, subprocess.check_output([str(executable)], text=True).splitlines()


def read_fixture(name):
    with (FIXTURES / name).open(newline="", encoding="utf-8") as file:
        return list(csv.DictReader(file, delimiter="\t"))


def compare_encased(rust):
    rows = [(int(row["rating_min"]), int(row["alternative"]),
             *(float(row[key]) / 1000 for key in ("min_dimension_mm", "min_cover_mm", "min_axis_mm")))
            for row in read_fixture("table-4-4.tsv")]
    samples = [(rating, size, cover, axis)
               for rating in (30, 60, 90, 120, 180, 240, 300)
               for size, cover, axis in ((.350, .050, .045), (.300, .050, .045), (.500, .080, .075))]
    count = len(rows) + len(samples)
    assert len(rust) >= count
    for row, actual in zip(rows, rust):
        assert tuple(map(float, actual.split())) == row, (row, actual)
    with sqlite3.connect(":memory:") as db:
        db.execute("CREATE TABLE geometry(rating INTEGER, alternative INTEGER, size REAL, cover REAL, axis REAL)")
        db.executemany("INSERT INTO geometry VALUES (?, ?, ?, ?, ?)", rows)
        for (rating, size, cover, axis), actual in zip(samples, rust[len(rows):count]):
            result = db.execute("""
                SELECT alternative, MAX(size / ?, cover / ?, axis / ?) AS utilization
                FROM geometry WHERE rating = ? ORDER BY utilization, alternative LIMIT 1
            """, (size, cover, axis, rating)).fetchone()
            if result is None:
                assert actual == "unavailable", (rating, actual)
            else:
                option, utilization = actual.split()
                assert int(option) == result[0] and isclose(float(utilization), result[1], rel_tol=1e-12), (result, actual)
    print(f"[DEBUG] {len(rows)} published rows and {len(samples)} Table 4.4 alternative selections matched Rust and SQLite.")
    return count


def compare_load_table(name, samples, rust):
    rows = read_fixture(name)
    count = len(rows) + len(samples)
    assert len(rust) >= count
    with sqlite3.connect(":memory:") as db:
        db.execute("CREATE TABLE geometry(rating INTEGER, eta REAL, size REAL, axis REAL, reinforcement REAL)")
        for row in rows:
            rating, load = int(row["rating_min"]), float(row["load_level"])
            size, axis, ratio = (
                None if row[key] == "-" else float(row[key]) / scale
                for key, scale in (("min_dimension_mm", 1000), ("min_axis_mm", 1000), ("min_reinforcement_pct", 100))
            )
            db.execute("INSERT INTO geometry VALUES (?, ?, ?, ?, ?)", (rating, load, size, axis, ratio))
        for actual, row in zip(rust, db.execute("SELECT * FROM geometry ORDER BY rowid")):
            rating, load, *cells = row
            expected = f"{rating} {load} " + " ".join("None" if cell is None else f"Some({cell})" for cell in cells)
            assert expected == actual, (row, actual)
        for (rating, load), actual in zip(samples, rust[len(rows):count]):
            result = db.execute("""
                SELECT MAX(size / .35, axis / .06, reinforcement / .042) FROM geometry
                WHERE rating = ? AND eta >= ? ORDER BY eta LIMIT 1
            """, (rating, load)).fetchone()
            if result is None or result[0] is None:
                assert actual == "unavailable", (rating, load, actual)
            else:
                assert isclose(result[0], float(actual), rel_tol=1e-12), (rating, load, result, actual)
    print(f"[DEBUG] {len(rows)} published rows and {len(samples)} {name} selection cases matched Rust and SQLite.")
    return count



def check_catalogue_examples():
    subset = next(p for p in TABLE.parents if p.name == "✳️any")
    showcase = subset / "🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json"
    cases = {case["entryId"]: case for case in json.loads(showcase.read_text())["cases"]}
    evaluator_path = TABLE.parent.parent / "📇️catalogue/🧮️evaluators/🦀️.rs"
    source = evaluator_path.read_text(encoding="utf-8")
    evaluator = source[source.index("/// 📏️ One verification row"):source.index("/// 🌉 Tables 4.1 and 4.2")]
    table_source = (TABLE / "🦀️.rs").read_text(encoding="utf-8")
    core = table_source[table_source.index("/// 📚️ One cell of a tabulated-data table"):table_source.index("//#region 🧪️Tests")]
    samples = [
        ("C94-13", "partially_encased_fire_table", {}, {}, "pass"),
        ("C94-13", "partially_encased_fire_table", {"fire_col_width": .25}, {}, "fail"),
        ("C94-13", "partially_encased_fire_table", {"fire_col_ambient_axis": .07}, {}, "fail"),
        ("C94-13", "partially_encased_fire_table", {}, {"fire_col_rating": "R120"}, "unavailable"),
        ("C94-14", "filled_column_fire_table", {}, {}, "pass"),
        ("C94-14", "filled_column_fire_table", {"fire_col_depth": .35, "fire_fill_ratio": 0, "fire_fill_axis": 0,
            "fire_col_ambient_axis": 0, "fire_fill_wall": .014},
            {"fire_col_ext_rating": "R30", "fire_fill_shape": "circular", "fire_fill_rebar": "none"}, "pass"),
        ("C94-14", "filled_column_fire_table", {"fire_col_load_level": .60}, {"fire_col_ext_rating": "R180"}, "unavailable"),
        ("C94-14", "filled_column_fire_table", {"fire_fill_wall": .025}, {}, "unavailable"),
        ("C94-15", "fully_encased_fire_table", {}, {}, "pass"),
        ("C94-15", "fully_encased_fire_table", {"fire_enc_steel_cover": .05}, {"fire_col_ext_rating": "R120"}, "pass"),
        ("C94-15", "fully_encased_fire_table", {"fire_enc_steel_cover": .05, "fire_col_width": .30}, {"fire_col_ext_rating": "R120"}, "fail"),
        ("C94-15", "fully_encased_fire_table", {"fire_enc_bar_diameter": .010}, {}, "fail"),
    ]
    samples += [
        ("C94-16", "insulating_beam_fire_table", {}, {}, "pass"),
        ("C94-17", "insulating_column_fire_table", {}, {}, "pass"),
        ("C94-17", "insulating_column_fire_table", {"l_col": 11.0}, {}, "unavailable"),
    ]
    fixture = evaluator_path.parent.parent / "🧪️tests/🔬️unit/insulating-encasement.tsv"
    with fixture.open(newline="", encoding="utf-8") as file, sqlite3.connect(":memory:") as db:
        for row in csv.DictReader(file, delimiter="\t"):
            column = row["entry"] == "C94-17"
            rating = row["rating"]
            key = "fire_ins_column_basis" if column else "fire_ins_beam_basis"
            function = "insulating_column_fire_table" if column else "insulating_beam_fire_table"
            values = [float(row[x]) for x in ("cover_m", "max_spacing_m", "min_diameter_m", "min_mesh_cover_m", "max_mesh_cover_m")]
            reference, = db.execute("""
                WITH available(rating, required) AS (
                    VALUES ('R30', 0), ('R60', 0.025), ('R90', 0.030), ('R120', 0.040), ('R180', 0.050)
                ), sample AS (SELECT ? class, ? basis, ? mesh, ? covering, ? pitch, ? diameter, ? low, ? high)
                SELECT CASE
                    WHEN basis <> 'verified' OR required IS NULL THEN 'unavailable'
                    WHEN required = 0 THEN 'pass'
                    WHEN mesh = 'yes' AND low > high THEN 'unavailable'
                    WHEN covering >= required AND mesh = 'yes'
                        AND pitch <= 0.250 AND diameter >= 0.004 AND low >= 0.020 AND high <= 0.050 THEN 'pass'
                    ELSE 'fail'
                END FROM sample LEFT JOIN available ON class = rating
            """, (rating, row["basis"], row["mesh"], *values)).fetchone()
            assert reference == row["expected"], row
            numbers = dict(zip(("fire_ins_cover", "fire_ins_spacing", "fire_ins_diameter", "fire_ins_mesh_cover_min", "fire_ins_mesh_cover_max"), values))
            texts = {"fire_col_ext_rating": rating, key: row["basis"], "fire_ins_mesh": row["mesh"]}
            samples.append((row["entry"], function, numbers, texts, reference))

    def quote(value):
        return json.dumps(value, ensure_ascii=False)
    def mapping(values, wrap):
        return "[" + ", ".join(f"({quote(key)}.into(), {wrap(value)})" for key, value in values.items()) + "].into_iter().collect()"
    commands = []
    for code, function, numbers, texts, result in samples:
        values = cases[code]
        numbers = mapping({**{row["id"]: row["value"] for row in values["numbers"]}, **numbers}, lambda value: repr(float(value)))
        texts = mapping({**{row["id"]: row["value"] for row in values["texts"]}, **texts}, lambda value: quote(value) + ".into()")
        commands.append(f"assert_eq!(status({function}(&Inputs(&DesignCase {{ values: Values {{ numbers: {numbers}, texts: {texts} }} }}))), {quote(result)});")
    prelude = """
use std::collections::BTreeMap;
#[derive(Clone, Copy)] pub enum QuantityKind { Dimensionless, Length }
pub struct Values { numbers: BTreeMap<String, f64>, texts: BTreeMap<String, String> }
pub struct DesignCase { values: Values }
fn status(outcome: Outcome) -> &'static str {
    match outcome {
        Err(_) => "unavailable",
        Ok(rows) if !rows.is_empty() && rows.iter().all(|row| row.demand <= row.capacity) => "pass",
        Ok(_) => "fail",
    }
}
"""
    harness = OUTPUT / "en1994-fire-catalogue.rs"
    executable = OUTPUT / ("en1994-fire-catalogue.exe" if os.name == "nt" else "en1994-fire-catalogue")
    harness.write_text(prelude + "mod formulas { pub mod column_fire {\n" + core + "} }\n" +
                       evaluator + "fn main() {\n" + "\n".join(commands) + "\n}\n", encoding="utf-8")
    subprocess.run(["rustc", "--edition=2024", str(harness), "-o", str(executable)], check=True)
    subprocess.run([str(executable)], check=True)
    print(f"[DEBUG] {len(samples)} production catalogue cases passed using the checked-in showcase inputs.")

def main():
    OUTPUT.mkdir(exist_ok=True)
    samples, rust = rust_table_results()
    cursor = compare_encased(rust)
    cursor += compare_load_table("table-4-6.tsv", samples, rust[cursor:])
    cursor += compare_load_table("table-4-7.tsv", samples, rust[cursor:])
    assert cursor == len(rust)
    check_catalogue_examples()


if __name__ == "__main__":
    main()
