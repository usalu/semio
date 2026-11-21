"""Compare DIN EN 1991-1-2/NA:2015-09 Annex BB catalogue values with SQLite."""

import csv
import json
import math
import os
from pathlib import Path
import sqlite3
import subprocess


TICKET = Path(__file__).resolve().parent
ROOT = next(p.parent for p in TICKET.parents if p.name == ".🧬semio")
NORM = ROOT / "✏️s/🔌️plugins/📕️norm"
ARTIFACT = NORM / "🗿️artifacts/🏋️en1991"
STANDARD = next(p for p in ARTIFACT.rglob("✳️any") if (p / "🧬️schema").is_dir())
CHECKS = STANDARD / "🧬️schema/💡️inferences/⚖️checks"
FIXTURE = STANDARD / "🧬️schema/🧪️tests/📇️catalogue/fire-load-bb.tsv"
REGISTRY = NORM / "⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json"
OUTPUT = TICKET / "🗑️generated"


def main():
    with FIXTURE.open(newline="", encoding="utf-8") as source:
        rows = [(row["occupancy"], float(row["characteristic_mj_m2"]))
                for row in csv.DictReader(source, delimiter="\t")]
    ids = {row["id"]: row for row in json.loads(REGISTRY.read_text(encoding="utf-8"))["entries"]}
    for occupancy, value in rows:
        de = ids[f"ndp.en1991.1-2.fire-load-bb.{occupancy}"]
        en = ids[f"ndp.en1991.1-2.fire-load-fractile.{occupancy}"]
        assert de["naDe"] == value and de["recommended"] is None and en["naDe"] is None
        assert de["annexStatus"]["naDe"]["status"] == "verified" and de["source"]["edition"] == "2015-09"

    rust = (CHECKS / "📇️catalogue/🔥️fire/🦀️.rs").read_text(encoding="utf-8")
    evaluator = rust[rust.index("fn german_fire_load("):rust.index("fn e12_04(")]
    table = "\n".join(f'        "{key}" => Some({float(ids[f"ndp.en1991.1-2.fire-load-bb.{key}"]["naDe"])!r}e6),' for key, _ in rows)
    prelude = """
#[derive(Clone, Copy)]
enum AnnexChoice { De }
enum QuantityKind { Energy }
struct Outcome(f64);
impl Outcome {
    fn info(_: &str, _: &str, _: &str, _: QuantityKind, qty: f64) -> Self { Self(qty) }
    fn steps(self, _: String) -> Self { self }
}
type R = Result<Vec<Outcome>, String>;
struct Case<'a> { numbers: &'a [(&'a str, f64)], texts: &'a [(&'a str, &'a str)] }
impl Case<'_> {
    fn n(&self, key: &str) -> Result<f64, String> {
        self.numbers.iter().find_map(|&(id, value)| (id == key).then_some(value)).ok_or_else(|| format!("missing {key}"))
    }
    fn pos(&self, key: &str) -> Result<f64, String> {
        let value = self.n(key)?;
        if value > 0.0 { Ok(value) } else { Err(format!("invalid {key}")) }
    }
    fn s(&self, key: &str) -> Result<&str, String> {
        self.texts.iter().find_map(|&(id, value)| (id == key).then_some(value)).ok_or_else(|| format!("missing {key}"))
    }
}
mod part_1_2 {
    use super::AnnexChoice;
    pub fn characteristic_fire_load_j_m2(_: AnnexChoice, key: &str) -> Option<f64> {
        match key {
""" + table + """
            _ => None,
        }
    }
}
fn emit(case: Case<'_>) {
    match german_fire_load(&case) {
        Ok(rows) => println!("{}", rows[0].0 / 1.0e6),
        Err(_) => println!("unavailable"),
    }
}
"""
    scenarios = [(label, gamma, chi, construction)
                 for label, _ in rows for gamma in (0.80, 1.25)
                 for chi, construction in ((0.8, 0.0), (0.6, 20.0))]
    occupied = ", ".join(f'("{key}", {gamma}, {chi}, {construction})' for key, gamma, chi, construction in scenarios)
    surveyed = [(20.0, 1.25), (45.0, 1.15), (150.0, 0.8), (-5.0, 0.8), (1000.0, 0.0)]
    cases = ", ".join(f"({q}, {g})" for q, g in surveyed)
    driver = f"""
fn main() {{
    for (use_, gamma, chi, construction) in [{occupied}] {{
        emit(Case {{ numbers: &[("bb_gamma", gamma), ("bb_use_chi", chi), ("bb_construction_mj", construction)],
            texts: &[("bb_source", "typical_use"), ("occupancy", use_)] }});
    }}
    for (q, gamma) in [{cases}] {{
        emit(Case {{ numbers: &[("bb_gamma", gamma), ("bb_survey_mj", q)], texts: &[("bb_source", "survey")] }});
    }}
}}
"""
    path = OUTPUT / "en1991-fire-bb.rs"
    path.write_text(prelude + evaluator + driver, encoding="utf-8")
    executable = OUTPUT / ("en1991-fire-bb.exe" if os.name == "nt" else "en1991-fire-bb")
    subprocess.run(["rustc", "--edition=2024", str(path), "-o", str(executable)], check=True)
    actual = subprocess.check_output([str(executable)], text=True).splitlines()
    assert len(actual) == len(scenarios) + len(surveyed)
    with sqlite3.connect(":memory:") as db:
        db.execute("CREATE TABLE fire_load(occupancy TEXT PRIMARY KEY, q REAL)")
        db.executemany("INSERT INTO fire_load VALUES (?, ?)", rows)
        for (key, gamma, chi, construction), value in zip(scenarios, actual):
            expected, = db.execute("SELECT ? * (? * q + ?) FROM fire_load WHERE occupancy = ?",
                                   (gamma, chi, construction, key)).fetchone()
            assert math.isclose(float(value), expected, rel_tol=1e-12), (key, value, expected)
        for (q, gamma), value in zip(surveyed, actual[len(scenarios):]):
            expected, = db.execute("SELECT CASE WHEN ? >= 0 AND ? > 0 THEN MAX(50, ? * ?) END",
                                   (q, gamma, q, gamma)).fetchone()
            if expected is None:
                assert value == "unavailable", (q, gamma, value)
            else:
                assert math.isclose(float(value), expected, rel_tol=1e-12), (q, gamma, value, expected)
    print(f"[DEBUG] {len(rows)} Annex BB occupancy rows and {len(actual)} design-load cases matched Rust and SQLite.")


if __name__ == "__main__":
    main()
