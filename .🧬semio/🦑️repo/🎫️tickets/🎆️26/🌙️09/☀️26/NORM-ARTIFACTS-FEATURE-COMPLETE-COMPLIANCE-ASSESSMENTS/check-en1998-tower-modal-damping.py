"""Compare the Annex B damping and elastic spectrum catalogue against SQLite."""

import csv
import json
from math import isclose
import os
from pathlib import Path
import sqlite3
import subprocess

TICKET = Path(__file__).resolve().parent
ROOT = next(p.parent for p in TICKET.parents if p.name == ".🧬semio")
SCHEMA = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
CATALOGUE = SCHEMA / "💡️inferences/📇️catalogue"
OUTPUT = TICKET / "🗑️generated"
ENERGIES = ("tw_welded_energy", "tw_bolted_energy", "tw_reinforced_energy", "tw_prestressed_energy",
    "tw_ceramic_energy", "tw_masonry_energy", "tw_liner_steel_energy", "tw_frp_energy",
    "tw_sway_energy", "tw_rocking_energy", "tw_vertical_energy")


def rust_source():
    formula = (SCHEMA / "📐️formulas/🦀️.rs").read_text(encoding="utf-8")
    spectrum = formula[formula.index("    pub fn elastic_response_spectrum_eta"):formula.index("    /// 📉️ Design spectrum S_d(T)")]
    factor = formula[formula.index("    pub fn damping_correction"):formula.index("    /// 📉 Design spectrum S_d(T)")]
    damping = formula[formula.index("    /// 📊 Damping ratios for structural"):formula.index("    /// 🪢 Horizontal guy-cable stiffness")]
    path = CATALOGUE / "🧮️evaluators/🦀️.rs"
    source = path.read_text(encoding="utf-8")
    rows = source[source.index("/// 📏️ One verification row"):source.index("/// 📉 EN 1998-1 §3.2.2.5")].replace("fn base_shear_row", "pub fn base_shear_row")
    elastic = source[source.index("pub fn elastic_force"):source.index("/// ↕️ EN 1998-1 §3.2.2.3")]
    evaluator = source[source.index("pub fn tower_modal_damping"):source.index("/// 🏗 EN 1998-6 §8.2")].replace(
        "use semio_s_artifact_norm_en1998_schema::part_6", "use crate::part_6")
    return f"mod part_1 {{\n{spectrum}{factor}}}\nmod part_6 {{\n{damping}}}\n" + rows + elastic + evaluator


def independent_result(db, keys, vals, modes):
    lookup = dict(zip(keys, vals))
    method, action, tower = modes
    if tower and method == "energy":
        db.execute("DELETE FROM weights")
        db.executemany("INSERT INTO weights VALUES (?,?,?)", (
            (index, lookup[key], lookup[xi] / 100 if index >= 8 else None)
            for index, (key, xi) in enumerate(zip(ENERGIES, [None] * 8 + ["tw_sway_xi", "tw_rocking_xi", "tw_vertical_xi"]))))
        damping, invalid = db.execute("""
            SELECT SUM(energy * COALESCE(experimental, CASE WHEN ? = 'design' THEN hi ELSE lo END))/SUM(energy),
                MAX(CASE WHEN energy < 0 OR energy > 0 AND experimental NOT BETWEEN lo AND hi THEN 1 ELSE 0 END)
            FROM weights JOIN published USING (id)
        """, (action,)).fetchone()
        if damping is None or invalid:
            return None
    else:
        damping = lookup["xi_percent"] / 100.0
    a, s, tb, tc, td, t, mass = [lookup[key] for key in ("a_g", "s_soil", "t_b", "t_c", "t_d", "t_1", "mass")]
    if not (0.0 < tb <= tc <= td and t >= 0.0):
        return None
    force, = db.execute("""
        WITH design(a, s, tb, tc, td, t, mass, eta) AS
            (SELECT ?, ?, ?, ?, ?, ?, ?, MAX(0.55, SQRT(10.0/(5.0 + ?))))
        SELECT mass*a*s*CASE
            WHEN t <= tb THEN 1+t*(2.5*eta-1)/tb
            WHEN t <= tc THEN 2.5*eta
            WHEN t <= td THEN 2.5*eta*tc/t
            ELSE 2.5*eta*tc*td/(t*t) END FROM design
    """, (a, s, tb, tc, td, t, mass, damping * 100)).fetchone()
    return [damping, force] if tower else [force]


def main():
    fixture = CATALOGUE / "🧪️tests/🔬️unit/tower-modal-damping.tsv"
    with fixture.open(newline="", encoding="utf-8") as stream:
        rows = [(key, float(lo), float(hi)) for key, lo, hi in list(csv.reader(stream, delimiter="\t"))[1:]]
    vocab = json.loads((CATALOGUE / "📚️inputs/🔣️.json").read_text(encoding="utf-8"))["inputs"]
    scope = "C98-09"
    keys = ["a_g", "s_soil", "t_b", "t_c", "t_d", "t_1", "mass", "v_rd", "xi_percent",
            *ENERGIES, "tw_sway_xi", "tw_rocking_xi", "tw_vertical_xi"]
    defaults = {row["key"]: row["example"] for row in vocab if row["key"] in keys and row["scope"] in ("", scope)}
    defaults.update({row["key"]: row["example"] for row in vocab if row["key"] in keys and row["scope"] == scope})
    trials = []
    for mode in ("damage", "design"):
        for component in ENERGIES:
            for t in (0.0, 0.05, 0.5, 1.4, 3.0):
                data = dict(defaults, **dict.fromkeys(ENERGIES, 0.0))
                data.update({component: 1.0, "t_1": t})
                trials.append((data, ("energy", mode, True)))
    trials.extend((dict(defaults, t_1=t, xi_percent=xi), ("experimental", "design", tower))
                  for t in (0.0, 0.05, 0.15, 0.5, 1.1, 5.0) for xi in (0.0, 2.0, 5.0, 20.0, 60.0) for tower in (True, False))
    trials.extend([
        (dict(defaults, tw_sway_energy=3.0, tw_sway_xi=20.0), ("energy", "design", True)),
        (dict(defaults, tw_sway_energy=3.0, tw_sway_xi=21.0), ("energy", "design", True)),
        (dict(defaults, tw_welded_energy=0.0, tw_reinforced_energy=0.0), ("energy", "design", True)),
        (dict(defaults, tw_welded_energy=-1.0), ("energy", "design", True)),
    ])
    setup = """
#![allow(dead_code)]
use std::collections::BTreeMap;
#[derive(Clone, Copy)] pub enum QuantityKind { Dimensionless, Force }
#[derive(Clone, Copy)] pub enum CheckStatus { Info }
pub struct Values { numbers: BTreeMap<String, f64>, texts: BTreeMap<String, String> }
pub struct DesignCase { values: Values }
fn run(numbers: BTreeMap<String, f64>, source: &str, action: &str, tower: bool) {
    let case = DesignCase { values: Values { numbers, texts: [
        ("tw_damping_source".into(), source.into()), ("tw_action".into(), action.into())
    ].into() } };
    let check = if tower { tower_modal_damping } else { elastic_force };
    match check(&Inputs(&case)) {
        Ok(rows) => println!("{}", rows.iter().map(|row| row.demand.to_string()).collect::<Vec<_>>().join("\t")),
        Err(_) => println!("unavailable"),
    }
}
"""
    calls = []
    for values, (method, mode, tower) in trials:
        numbers = ",".join(f"({json.dumps(key)}.into(), {float(values[key])!r})" for key in keys)
        calls.append(f"run([{numbers}].into(), {json.dumps(method)}, {json.dumps(mode)}, {str(tower).lower()});")
    driver = "fn main() {\nfor (key, lo, hi) in part_6::MODAL_DAMPING_RATIOS { println!(\x22{key}\t{lo}\t{hi}\x22); }\n" + "\n".join(calls) + "\n}\n"
    harness = OUTPUT / "en1998-tower-damping.rs"
    executable = OUTPUT / ("en1998-tower-damping.exe" if os.name == "nt" else "en1998-tower-damping")
    harness.write_text(setup + rust_source() + driver, encoding="utf-8")
    subprocess.run(["rustc", "--edition=2024", str(harness), "-o", str(executable)], check=True)
    actual = subprocess.check_output([str(executable)], text=True).splitlines()
    assert len(actual) == len(rows) + len(trials)
    for source, record in zip(actual, rows):
        key, lo, hi = source.split("\t")
        assert (key, float(lo), float(hi)) == record
    with sqlite3.connect(":memory:") as db:
        db.executescript("CREATE TABLE published (id INTEGER PRIMARY KEY, lo REAL, hi REAL); CREATE TABLE weights (id INTEGER, energy REAL, experimental REAL);")
        db.executemany("INSERT INTO published VALUES (?, ?, ?)", [(index, lo, hi) for index, (_, lo, hi) in enumerate(rows)])
        for (scenario, mode), result in zip(trials, actual[len(rows):]):
            expected = independent_result(db, keys, [scenario[k] for k in keys], mode)
            if expected is None:
                assert result == "unavailable", (scenario, mode, result)
            else:
                got = list(map(float, result.split("\t")))
                assert len(got) == len(expected)
                assert all(isclose(a, b, rel_tol=1e-11, abs_tol=1e-12) for a, b in zip(got, expected)), (mode, got, expected)
    print(f"[DEBUG] {len(rows)} EN 1998 damping records and {len(trials)} elastic spectrum cases matched Rust and SQLite.")


if __name__ == "__main__":
    main()
