"""Check railway, crane and silo cases with SQLite and the catalogue's fixtures."""

import csv
import json
from math import isclose
import os
from pathlib import Path
import re
import sqlite3
import subprocess


TICKET = Path(__file__).resolve().parent
ROOT = next(parent for parent in TICKET.parents if parent.name == ".🧬semio").parent
ARTIFACT = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991"
CHECKS = ARTIFACT / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks"
CATALOGUE = CHECKS / "📇️catalogue"
FIXTURE = CATALOGUE / "🗂️cases/🧪️tests/🔬️unit/🚆️.tsv"
HSLM_FIXTURE = FIXTURE.with_name("hslm-a.tsv")
HSLM_B_FIXTURE = FIXTURE.with_name("hslm-b.tsv")
HSLM_SELECTION_FIXTURE = FIXTURE.with_name("hslm-selection.tsv")
CRANE_FIXTURE = FIXTURE.with_name("crane-classes.tsv")
SILO_FIXTURE = FIXTURE.with_name("silo-thermal.tsv")
TANK_FIXTURE = FIXTURE.with_name("tank-actions.tsv")
AERO_FIXTURE = FIXTURE.with_name("rail-aerodynamics.tsv")
OUTPUT = TICKET / "🗑️generated"


def compile_catalogue_evaluators():
    catalogue = (CATALOGUE / "🦀️.rs").read_text(encoding="utf-8")
    case_and_outcomes = catalogue[catalogue.index("pub type R"):catalogue.index("pub struct Entry")]
    case_and_outcomes = case_and_outcomes.rsplit("\n///", 1)[0]
    parts = (CHECKS / "🧱️parts/🦀️.rs").read_text(encoding="utf-8")
    dynamic_factor, = re.findall(r"^\s*(pub fn rail_dynamic_factors[^\n]+)$", parts, re.MULTILINE)
    source = OUTPUT / "rail-fatigue-harness.rs"
    module = json.dumps(str(CATALOGUE / "🌉️structures/🚆️.rs"), ensure_ascii=False)
    hslm_module = json.dumps(str(CATALOGUE / "🌉️structures/🚄️.rs"), ensure_ascii=False)
    crane_module = json.dumps(str(CATALOGUE / "🌉️structures/🏗️.rs"), ensure_ascii=False)
    silo_module = json.dumps(str(CATALOGUE / "🌉️structures/🌡️.rs"), ensure_ascii=False)
    tank_module = json.dumps(str(CATALOGUE / "🌉️structures/🛢️.rs"), ensure_ascii=False)
    aero_module = json.dumps(str(CATALOGUE / "🌉️structures/💨️.rs"), ensure_ascii=False)
    fixture = json.dumps(str(FIXTURE), ensure_ascii=False)
    hslm_fixture = json.dumps(str(HSLM_FIXTURE), ensure_ascii=False)
    b_fixture = json.dumps(str(HSLM_B_FIXTURE), ensure_ascii=False)
    selection_fixture = json.dumps(str(HSLM_SELECTION_FIXTURE), ensure_ascii=False)
    crane_fixture = json.dumps(str(CRANE_FIXTURE), ensure_ascii=False)
    silo_fixture = json.dumps(str(SILO_FIXTURE), ensure_ascii=False)
    tank_fixture = json.dumps(str(TANK_FIXTURE), ensure_ascii=False)
    aero_fixture = json.dumps(str(AERO_FIXTURE), ensure_ascii=False)
    source.write_text(
        '#![allow(dead_code)]\n'
        'use std::collections::BTreeMap;\n'
        '#[derive(Clone, Debug, PartialEq)] pub enum AnnexChoice { En, De }\n'
        'pub type LocalizedCopy = String;\n'
        '#[derive(Clone, Debug)] pub enum QuantityKind { Stress, Mass, Dimensionless, Length, Force, Pressure }\n'
        f'{case_and_outcomes}\nmod part_2 {{ {dynamic_factor} }}\n'
        f'#[path = {module}] mod fatigue;\n'
        f'#[path = {hslm_module}] mod hslm;\n'
        f'#[path = {crane_module}] mod crane;\n'
        f'#[path = {silo_module}] mod silo;\n'
        f'#[path = {tank_module}] mod tank;\n'
        f'#[path = {aero_module}] mod aero;\n'
        f'const FIXTURE: &str = include_str!({fixture});\n'
        f'const HSLM_FIXTURE: &str = include_str!({hslm_fixture});\n'
        f'const HSLM_B_FIXTURE: &str = include_str!({b_fixture});\n'
        f'const HSLM_SELECTION_FIXTURE: &str = include_str!({selection_fixture});\n'
        f'const CRANE_FIXTURE: &str = include_str!({crane_fixture});\n'
        f'const SILO_FIXTURE: &str = include_str!({silo_fixture});\n'
        f'const TANK_FIXTURE: &str = include_str!({tank_fixture});\n'
        f'const AERO_FIXTURE: &str = include_str!({aero_fixture});\n'
        r'''
fn main() {
    for row in FIXTURE.lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();
        let mut case = Case::new("oracle", AnnexChoice::En);
        for input in fields[1].split(',') {
            let (key, value) = input.split_once('=').unwrap();
            case = if let Ok(number) = value.parse() {
                case.with_n(key, number)
            } else {
                case.with_s(key, value)
            };
        }
        let result = match fields[0] {
            "E2.21" => fatigue::steel_bridge(&case),
            "E2.21.1" => fatigue::real_train(&case),
            "E2.21.3" => fatigue::traffic_mix(&case),
            _ => panic!("{row}"),
        };
        match result {
            Ok(outcomes) => {
                let output = outcomes.iter().find(|outcome| outcome.key == fields[2]).unwrap();
                println!("{} {}", output.value, output.capacity);
            }
            Err(_) => println!("invalid"),
        }
    }
    for row in HSLM_FIXTURE.lines().skip(1) {
        let id = row.split('\t').next().unwrap();
        let case = Case::new("oracle", AnnexChoice::En).with_s("rail_hslm_a_train", id);
        let results = hslm::universal_train(&case).unwrap();
        let quantities = results.iter().map(|result| result.value.to_string()).collect::<Vec<_>>();
        println!("{}", quantities.join(" "));
    }
    for row in HSLM_B_FIXTURE.lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();
        let mut case = Case::new("oracle", AnnexChoice::En).with_n("rail_hslm_b_span", fields[0].parse().unwrap());
        for (key, value) in [("rail_hslm_b_count", fields[1]), ("rail_hslm_b_spacing", fields[2])] {
            if let Ok(number) = value.parse() { case = case.with_n(key, number); }
        }
        match hslm::uniform_train(&case) {
            Ok(results) => {
                let quantities = results.iter().map(|result| result.value.to_string()).collect::<Vec<_>>();
                println!("{}", quantities.join(" "));
            }
            Err(_) => println!("invalid"),
        }
    }
    for row in HSLM_SELECTION_FIXTURE.lines().skip(1) {
        let mut fields = row.split('\t');
        let case = Case::new("oracle", AnnexChoice::En)
            .with_n("span", fields.next().unwrap().parse().unwrap())
            .with_s("rail_hslm_structure", fields.next().unwrap());
        let results = hslm::load_model_selection(&case).unwrap();
        let quantities = results.iter().map(|result| result.value.to_string()).collect::<Vec<_>>();
        println!("{}", quantities.join(" "));
    }
    for row in CRANE_FIXTURE.lines().skip(1) {
        let item = row.split('\t').next().unwrap();
        let case = Case::new("oracle", AnnexChoice::En).with_s("crane_duty", item);
        let results = crane::classify(&case).unwrap();
        let quantities = results.iter().map(|result| result.value.to_string()).collect::<Vec<_>>();
        println!("{}", quantities.join(" "));
    }
    for row in SILO_FIXTURE.lines().skip(1) {
        let mut fields = row.split('\t');
        let mut case = Case::new("oracle", AnnexChoice::En)
            .with_s("silo_modulus_source", fields.next().unwrap())
            .with_n("dc", 8.0).with_n("silo_wall_thickness", 0.008)
            .with_n("silo_wall_expansion", 12.0).with_n("silo_wall_modulus", 200e9);
        for (key, value) in ["silo_delta_cooling", "silo_solid_poisson", "silo_unloading_modulus", "silo_solid_weight",
            "silo_base_filling_stress"].into_iter().zip(fields) {
            if let Ok(number) = value.parse() { case = case.with_n(key, number); }
        }
        match silo::wall_cooling(&case) {
            Ok(results) => {
                let quantities = results.iter().map(|result| result.value.to_string()).collect::<Vec<_>>();
                println!("{}", quantities.join(" "));
            }
            Err(_) => println!("invalid"),
        }
    }
    for row in TANK_FIXTURE.lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();
        let case = Case::new("oracle", AnnexChoice::En);
        let number = |index: usize| fields[index].parse::<f64>().unwrap();
        let (result, key) = if fields[0] == "E4.09" {
            (tank::wind(&case.with_s("tank_wind_enclosure", fields[1]).with_n("tank_wind_qe", number(2))
                .with_n("tank_wind_qi", number(3)).with_n("tank_wind_cpe", number(4))), "net_wind")
        } else {
            (tank::specified_pressure(&case.with_s("tank_pressure_situation", fields[1]).with_n("tank_pressure_min", number(2))
                .with_n("tank_pressure_max", number(3))), "pressure_range")
        };
        match result {
            Ok(results) => println!("{}", results.iter().find(|outcome| outcome.key == key).unwrap().value),
            Err(_) => println!("invalid"),
        }
    }
    for row in AERO_FIXTURE.lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();
        let mut case = Case::new("oracle", AnnexChoice::En);
        for input in fields[1].split(',') {
            let (key, value) = input.split_once('=').unwrap();
            case = if let Ok(number) = value.parse() { case.with_n(key, number) } else { case.with_s(key, value) };
        }
        let result = match fields[0] {
            "E2.18" => aero::vertical(&case),
            "E2.18.1" => aero::crossing_surface(&case),
            "E2.18.2" => aero::adjacent_surface(&case),
            "E2.18.3" => aero::multiple_surfaces(&case),
            "E2.18.4" => aero::enclosure(&case),
            _ => panic!("{row}"),
        };
        match result {
            Ok(results) => println!("{}", results.iter().find(|outcome| outcome.key == fields[2]).unwrap().value),
            Err(_) => println!("invalid"),
        }
    }
}
''',
        encoding="utf-8",
    )
    executable = OUTPUT / ("rail-fatigue-harness.exe" if os.name == "nt" else "rail-fatigue-harness")
    subprocess.run(["rustc", "--edition=2024", str(source), "-o", str(executable)], check=True)
    return subprocess.check_output([str(executable)], text=True).splitlines()


def sqlite_result(db, entry, inputs, result):
    if entry == "E2.21.1":
        return db.execute("""
            WITH input(length, speed) AS (VALUES (:l_phi, :rail_fatigue_speed)),
            parameter AS (
                SELECT length,
                       speed / 3.6 / (CASE WHEN length <= 20 THEN 160
                                          ELSE 47.16 * POW(length, 0.408) END) AS k
                FROM input WHERE length > 0 AND speed > 0 AND speed <= 200
            )
            SELECT 1 + 0.5 * (k / (1 - k + POW(k, 4)) + 0.28 * EXP(-length * length / 100)), 0
            FROM parameter
        """, inputs).fetchone()
    if entry == "E2.21":
        return db.execute("""
            WITH factors AS (
                SELECT MAX(1, MIN(1.67, 1.44 / MAX(0.01, SQRT(:l_phi) - 0.2) + 0.82)) AS phi
                WHERE :l_phi > 0 AND :rail_fatigue_gamma_m > 0 AND :rail_fatigue_gamma_f > 0
                    AND :rail_fatigue_lambda > 0 AND :rail_fatigue_stress >= 0 AND :rail_fatigue_strength > 0
            )
            SELECT :rail_fatigue_gamma_f * :rail_fatigue_lambda * phi * :rail_fatigue_stress,
                   :rail_fatigue_strength / :rail_fatigue_gamma_m
            FROM factors
        """, inputs).fetchone()
    column = {"trains_per_day": "SUM(passages)", "daily_mass": "1000 * SUM(passages * tonnes)"}
    return db.execute(f"""
        SELECT {column.get(result, "SUM(passages)")}, 0 FROM traffic
        WHERE mix = :rail_fatigue_mix GROUP BY mix
    """, inputs).fetchone()


def aero_result(db, row):
    params = dict.fromkeys(("q_other", "other_train", "edge", "other_height", "height", "train",
                            "small_panel", "clearance_min", "clearance_max", "enclosed_length",
                            "enclosed_overhead", "enclosed_tracks"))
    for assignment in row["inputs"].split(","):
        key, value = assignment.split("=")
        try:
            value = float(value)
        except ValueError:
            pass
        params[key[len("rail_aero_"):]] = value
    params.update(entry=row["entry"], result=row["result"])
    return db.execute("""
        WITH shape(name, factor) AS (VALUES ('unfavourable', 1), ('smooth', .85), ('streamlined', .6)),
        k(factor) AS (SELECT CASE WHEN :end <= 5 THEN 2 ELSE 1 END)
        SELECT CASE :entry
            WHEN 'E2.18' THEN factor * :q * (SELECT factor FROM shape WHERE name = :train)
                * CASE WHEN :small_panel = 'small' THEN 1.3 ELSE 1 END
            WHEN 'E2.18.1' THEN factor * CASE WHEN :edge <= 1.5 THEN .75 ELSE 1 END *
                (:q * (SELECT factor FROM shape WHERE name = :train) +
                 CASE WHEN :q_other IS NULL THEN 0 ELSE :q_other *
                     (SELECT factor FROM shape WHERE name = :other_train) END)
            WHEN 'E2.18.2' THEN factor * (:q * MIN(1, MAX(0, (7.5 - :height) / 3.7)) +
                 CASE WHEN :q_other IS NULL THEN 0 ELSE :q_other * MIN(1, MAX(0, (7.5 - :other_height) / 3.7)) END)
            WHEN 'E2.18.3' THEN CASE WHEN :clearance_min <= :clearance_max THEN
                MIN(6, .6 * :clearance_min + .4 * MIN(6, :clearance_max)) END
            WHEN 'E2.18.4' THEN CASE WHEN :enclosed_length <= 20 THEN
                CASE :result WHEN 'pressure' THEN 2 * :q * factor ELSE factor * :enclosed_overhead *
                    CASE WHEN :enclosed_tracks = 'one' THEN 2.5 ELSE 3.5 END END END
        END FROM k WHERE :q >= 0 AND :end >= 0
    """, params).fetchone()


def main():
    OUTPUT.mkdir(exist_ok=True)
    with FIXTURE.open(newline="", encoding="utf-8") as file:
        rows = list(csv.DictReader(file, delimiter="\t"))
    with HSLM_FIXTURE.open(newline="", encoding="utf-8") as file:
        trains = list(csv.DictReader(file, delimiter="\t"))
    with HSLM_B_FIXTURE.open(newline="", encoding="utf-8") as file:
        uniform_trains = list(csv.DictReader(file, delimiter="\t"))
    with HSLM_SELECTION_FIXTURE.open(newline="", encoding="utf-8") as file:
        selections = list(csv.DictReader(file, delimiter="\t"))
    with CRANE_FIXTURE.open(newline="", encoding="utf-8") as file:
        cranes = list(csv.DictReader(file, delimiter="\t"))
    with SILO_FIXTURE.open(newline="", encoding="utf-8") as file:
        silos = list(csv.DictReader(file, delimiter="\t"))
    with TANK_FIXTURE.open(newline="", encoding="utf-8") as file:
        tanks = list(csv.DictReader(file, delimiter="\t"))
    with AERO_FIXTURE.open(newline="", encoding="utf-8") as file:
        aerodynamics = list(csv.DictReader(file, delimiter="\t"))
    rust_results = compile_catalogue_evaluators()
    a_end = len(rows) + len(trains)
    b_end = a_end + len(uniform_trains)
    selection_end = b_end + len(selections)
    crane_end = selection_end + len(cranes)
    silo_end = crane_end + len(silos)
    tank_end = silo_end + len(tanks)
    assert len(rust_results) == tank_end + len(aerodynamics)
    with sqlite3.connect(":memory:") as db:
        db.executescript("""
            CREATE TABLE traffic(mix TEXT, passages INTEGER, tonnes INTEGER);
            INSERT INTO traffic VALUES
                ('standard',12,663), ('standard',12,530), ('standard',5,940), ('standard',5,510),
                ('standard',7,2160), ('standard',12,1431), ('standard',8,1035), ('standard',6,1035),
                ('heavy',6,2160), ('heavy',13,1431), ('heavy',16,1135), ('heavy',16,1135),
                ('light',10,663), ('light',5,530), ('light',2,2160), ('light',190,296);
        """)
        for row, rust in zip(rows, rust_results):
            inputs = {}
            for assignment in row["inputs"].split(","):
                key, value = assignment.split("=")
                inputs[key] = value if key == "rail_fatigue_mix" else float(value)
            sql = sqlite_result(db, row["entry"], inputs, row["result"])
            if row["result"] == "invalid":
                assert sql is None and rust == "invalid", (row, sql, rust)
                continue
            expected = [float(row["expected"]), 0.0 if row["capacity"] == "-" else float(row["capacity"])]
            assert all(isclose(got, want, rel_tol=1e-12, abs_tol=1e-12)
                       for got, want in zip(sql, expected)), (row, sql)
            rust_values = rust.split()
            assert len(rust_values) == 2, (row, rust)
            assert all(isclose(float(got), want, rel_tol=1e-12, abs_tol=1e-12)
                       for got, want in zip(rust_values, expected)), (row, rust)
        axle_count = 0
        for train, rust in zip(trains, rust_results[len(rows):a_end]):
            coaches, coach, bogie, force, count, rear = (
                float(train[key]) for key in ("coaches", "coach_m", "bogie_m", "axle_kn", "total_axles", "rear_m")
            )
            positions = db.execute("""
                WITH RECURSIVE point(i, position) AS (
                    SELECT 1, 0.0
                    UNION ALL
                    SELECT i + 1, position + CASE MIN(i, :count - i)
                        WHEN 1 THEN 3
                        WHEN 2 THEN 11
                        WHEN 3 THEN 3
                        WHEN 4 THEN 3.525
                        WHEN 5 THEN :bogie
                        WHEN 6 THEN :coach - (3 * :bogie + 3.525) / 2
                        ELSE CASE i % 2 WHEN 0 THEN :coach - :bogie ELSE :bogie END
                    END
                    FROM point WHERE i < :count
                )
                SELECT position FROM point ORDER BY i
            """, dict(count=count, bogie=bogie, coach=coach)).fetchall()
            assert isclose(positions[-1][0], rear, abs_tol=1e-10), (train, positions[-1])
            expected = [coaches, coach, bogie, force * 1000] + [point[0] for point in positions]
            rust_values = rust.split()
            assert len(rust_values) == len(expected), (train, rust)
            assert all(isclose(float(got), want, rel_tol=1e-12, abs_tol=1e-12)
                       for got, want in zip(rust_values, expected)), (train, rust, expected)
            axle_count += len(positions)
        for train, rust in zip(uniform_trains, rust_results[a_end:b_end]):
            if train["count"] == "-":
                assert rust == "invalid", (train, rust)
                continue
            count, spacing, rear = (float(train[key]) for key in ("count", "spacing_m", "rear_m"))
            positions = db.execute("""
                WITH RECURSIVE point(i, position) AS (
                    SELECT 1, 0.0
                    UNION ALL SELECT i + 1, position + :spacing FROM point WHERE i < :count
                )
                SELECT position FROM point ORDER BY i
            """, dict(count=count, spacing=spacing)).fetchall()
            assert positions[-1][0] == rear, (train, positions[-1])
            assert [float(value) for value in rust.split()] == [
                count, spacing, 170000, *(point[0] for point in positions)
            ], (train, rust, positions)
        for case, rust in zip(selections, rust_results[b_end:selection_end]):
            a, critical, b = db.execute("""
                WITH inputs(simple, short, floor) AS (
                    SELECT :shape = 'simple', :span < 7, :shape = 'complex_floor'
                )
                SELECT NOT (simple AND short), simple AND NOT short, (simple AND short) OR floor FROM inputs
            """, dict(shape=case["structure"], span=float(case["span_m"]))).fetchone()
            expected = [float(case[key]) for key in ("a_all", "a_one_eligible", "b")]
            assert list((a, critical, b)) == expected and list(map(float, rust.split())) == expected, (case, rust)
        db.execute("CREATE TABLE crane(item INTEGER PRIMARY KEY, hc_min INTEGER, hc_max INTEGER, s_min INTEGER, s_max INTEGER)")
        db.executemany("INSERT INTO crane VALUES (:item, :hc_min, :hc_max, :s_min, :s_max)", cranes)
        for row, rust in zip(cranes, rust_results[selection_end:crane_end]):
            fields = db.execute("SELECT hc_min, hc_max, s_min, s_max FROM crane WHERE item = :item", row).fetchone()
            assert tuple(map(int, rust.split())) == fields, (row, rust, fields)
        for row, rust in zip(silos, rust_results[crane_end:silo_end]):
            formula_inputs = {key: value if key == "source" else None if value == "-" else float(value)
                              for key, value in row.items()}
            sql = db.execute("""
                WITH material(esu, ct, dt, nu) AS (
                    SELECT :measured_pa, 1.2, :delta_k, COALESCE(:nu, 0.3)
                    WHERE :source = 'tested' AND :measured_pa > 0
                    UNION ALL
                    SELECT 7 * POW(:weight_n_m3 / 1000, 1.5) * :base_stress_pa,
                           3, :delta_k, COALESCE(:nu, 0.3)
                    WHERE :source = 'density' AND :weight_n_m3 > 0 AND :base_stress_pa > 0
                )
                SELECT esu, ct, ct * 12e-6 * dt * 200e9 / (4.0 / .008 + (1 - nu) * 200e9 / esu)
                FROM material WHERE dt >= 0 AND nu BETWEEN 0 AND .5
            """, formula_inputs).fetchone()
            if row["pressure_pa"] == "-":
                assert sql is None and rust == "invalid", (row, sql, rust)
                continue
            expected = [float(row[key]) for key in ("esu_pa", "ct", "pressure_pa")]
            assert all(isclose(got, want, rel_tol=1e-12) for got, want in zip(sql, expected)), (row, sql)
            rust_values = list(map(float, rust.split()))
            assert len(rust_values) == len(expected), (row, rust)
            assert all(isclose(got, want, rel_tol=1e-12) for got, want in zip(rust_values, expected)), (row, rust)
        for row, rust in zip(tanks, rust_results[silo_end:tank_end]):
            inputs = {key: None if row[key] == "-" else float(row[key]) for key in ("first", "second", "third")}
            inputs["situation"] = row["situation"]
            inputs["entry"] = row["entry"]
            sql = db.execute("""
                WITH cpi(situation, coefficient) AS (VALUES ('open_tank', -.6), ('open_basin', -.6), ('small_vents', -.4))
                SELECT :first * :third - :second * coefficient FROM cpi
                WHERE :entry = 'E4.09' AND situation = :situation AND :first >= 0 AND :second >= 0
                UNION ALL
                SELECT :second - :first
                WHERE :entry = 'E4.09.1' AND :situation IN ('operation', 'test') AND :second >= :first
            """, inputs).fetchone()
            if row["result"] == "-":
                assert sql is None and rust == "invalid", (row, sql, rust)
                continue
            expected = float(row["result"])
            assert isclose(sql[0], expected, rel_tol=1e-12) and isclose(float(rust), expected, rel_tol=1e-12), (row, sql, rust)
        for row, rust in zip(aerodynamics, rust_results[tank_end:]):
            sql = aero_result(db, row)
            if row["expected"] == "-":
                assert sql in (None, (None,)) and rust == "invalid", (row, sql, rust)
                continue
            expected = float(row["expected"])
            assert isclose(sql[0], expected, abs_tol=1e-10) and isclose(float(rust), expected, abs_tol=1e-10), (row, sql, rust)
    print(f"[DEBUG] {len(rows)} Annex D fixture rows matched the Rust catalogue evaluators and SQLite.")
    print(f"[DEBUG] {axle_count} axle positions across {len(trains)} HSLM-A trains matched Rust and SQLite.")
    print(f"[DEBUG] {len(uniform_trains)} HSLM-B cases and {len(selections)} load-model selections passed.")
    print(f"[DEBUG] All {len(cranes)} crane-use classifications matched the Table B.1 fixture.")
    print(f"[DEBUG] All {len(silos)} silo cooling pressure cases matched Rust and SQLite.")
    print(f"[DEBUG] All {len(tanks)} tank wind and pressure cases matched Rust and SQLite.")
    print(f"[DEBUG] All {len(aerodynamics)} railway aerodynamic load cases matched Rust and SQLite.")


if __name__ == "__main__":
    main()
