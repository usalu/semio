"""Verify Annex BB.5 parts and the catalogue workflow against SQLite and SciPy."""

import csv
from itertools import product
import json
import math
import os
from pathlib import Path
import sqlite3
import subprocess

from scipy.stats import norm

TICKET = Path(__file__).resolve().parent
ROOT = next(p.parent for p in TICKET.parents if p.name == ".🧬semio")
NORM = ROOT / "✏️s/🔌️plugins/📕️norm"
SUBSET = NORM / "🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any"
CHECKS = SUBSET / "🧬️schema/💡️inferences/⚖️checks"
CATALOGUE = CHECKS / "📇️catalogue"
FIXTURE = CATALOGUE / "🗂️cases/🧪️tests/🔬️unit/annex-bb-probability.tsv"
SHOWCASE = SUBSET / "🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json"
OUTPUT = TICKET / "🗑️generated"


def independent_outcome(db, values):
    model = db.execute("SELECT a, b, mean FROM ignition WHERE occupancy = ?", (values["bb_ignition_use"],)).fetchone()
    coefficient, exponent, average = model
    annual = -math.expm1(-coefficient * values["area"] ** exponent)
    p1 = {"area": annual, "average": average, "conservative": max(annual, average)}[values["bb_probability_method"]]
    time = values["bb_response_min"]
    brigade = values["bb_brigade"]
    if brigade == "public":
        response = float(db.execute("SELECT 0.2 + 0.3 * MIN(1, MAX(0, (? - 15)/5.0))", (time,)).fetchone()[0])
    elif brigade == "project":
        response = values["bb_brigade_project"]
    else:
        if not 0 < time < 10:
            return None
        response = 0.02 if brigade == "onsite_four" else 0.05
    p3 = db.execute("SELECT probability FROM suppression WHERE method = ?", (values["bb_suppression"],)).fetchone()[0]
    group, impact = values["bb_failure_group"], values["bb_consequence"]
    target = values["bb_project_failure"] if group == "project" else db.execute(
        "SELECT probability FROM targets WHERE occupancy = ? AND impact = ?", (group, impact)).fetchone()[0]
    occupants = values["bb_occupant_failure"]
    if not (target and 0.0 < occupants <= 1.0 and 0.0 < response <= 1.0):
        return None
    p2 = occupants * response
    annual_damage = p1 * p2 * p3
    conditional = target / annual_damage
    if not 0 < conditional < 1:
        return None
    beta = float(norm.isf(conditional))
    def factor(cov):
        u = -float(norm.logcdf(0.6 * beta))
        return (1.0 - 0.78 * cov * (0.5772 + math.log(u))) / (1.0 - 0.78 * cov * (0.5772 + math.log(-math.log(0.90))))
    return (p1, p2, annual_damage, conditional, beta, factor(0.3), factor(0.2))


def main():
    with FIXTURE.open(newline="", encoding="utf-8") as stream:
        ignition = [(use, float(a), float(b), float(mean)) for use, a, b, mean in list(csv.reader(stream, delimiter="\t"))[1:]]
    doc = json.loads(SHOWCASE.read_text(encoding="utf-8"))
    case = next(row for row in doc["cases"] if row["entryId"] == "E12.03.1")
    seed = {value["id"]: value["value"] for key in ("numbers", "texts") for value in case[key]}
    scenarios = [{}]
    for occupancy, area, method in product([row[0] for row in ignition], (20.0, 150.0, 4000.0), ("area", "average", "conservative")):
        scenarios.append({"bb_ignition_use": occupancy, "area": area, "bb_probability_method": method})
    for brigade, minutes in product(("public", "onsite_four", "onsite_two", "project"), (9.0, 10.0, 15.0, 17.5, 20.0, 25.0)):
        scenarios.append({"bb_brigade": brigade, "bb_response_min": minutes})
    for group, impact, suppression in product(("office", "other", "agricultural", "project"), ("high", "medium", "low"),
            ("absent", "standard", "other_sprinkler", "water", "gas")):
        scenarios.append({"bb_failure_group": group, "bb_consequence": impact, "bb_suppression": suppression})
    inputs = [{**seed, **overrides} for overrides in scenarios]
    cdf_path = NORM / "⚖️compliance/📊️normal-distribution/🦀️.rs"
    cdf = cdf_path.read_text(encoding="utf-8")
    core = (CHECKS / "🧱️parts/🧯️annex-bb-reliability/🦀️.rs").read_text(encoding="utf-8")
    core = core.replace("use semio_s_plugin_norm_compliance::normal_distribution", "use crate::normal_distribution")
    source = (CATALOGUE / "🔥️fire/🦀️.rs").read_text(encoding="utf-8")
    evaluator = source[source.index("fn e12_03_1"):source.index("fn e12_04")]
    harness = OUTPUT / "en1991-bb-reliability.rs"
    binary = OUTPUT / ("en1991-bb-reliability.exe" if os.name == "nt" else "en1991-bb-reliability")
    setup = """
use std::collections::BTreeMap;
struct Case { numbers: BTreeMap<&'static str, f64>, texts: BTreeMap<&'static str, &'static str> }
type R = Result<Vec<Outcome>, String>;
enum QuantityKind { Dimensionless }
struct Outcome(f64);
impl Outcome {
    fn info(_: &str, _: &str, _: &str, _: QuantityKind, value: f64) -> Self { Self(value) }
    fn steps(self, _: impl AsRef<str>) -> Self { self }
}
impl Case {
    fn de(&self) -> bool { true }
    fn pos(&self, key: &str) -> Result<f64, String> {
        let value = self.n(key)?;
        if value.is_finite() && value > 0.0 { Ok(value) } else { Err(key.into()) }
    }
    fn n(&self, key: &str) -> Result<f64, String> { self.numbers.get(key).copied().ok_or_else(|| key.into()) }
    fn s(&self, key: &str) -> Result<&str, String> { self.texts.get(key).copied().ok_or_else(|| key.into()) }
}
fn run(case: Case) {
    match e12_03_1(&case) {
        Ok(result) => println!("{}", result.iter().map(|row| row.0.to_string()).collect::<Vec<_>>().join("\t")),
        Err(_) => println!("unavailable"),
    }
}
"""
    invocations = []
    for values in inputs:
        numbers = ", ".join(f"({json.dumps(k)}, {float(v)!r})" for k, v in values.items() if isinstance(v, (int, float)))
        texts = ", ".join(f"({json.dumps(k)}, {json.dumps(v)})" for k, v in values.items() if isinstance(v, str))
        invocations.append(f"    run(Case {{ numbers: [{numbers}].into(), texts: [{texts}].into() }});")
    harness.write_text(f"mod normal_distribution {{\n{cdf}}}\nmod annex_bb_reliability {{\n{core}}}\n" +
        setup + evaluator + "fn main() {\n" + "\n".join(invocations) + "\n}\n", encoding="utf-8")
    subprocess.run(["rustc", "--edition=2024", str(harness), "-o", str(binary)], check=True)
    actual = subprocess.check_output([str(binary)], text=True).splitlines()
    assert len(actual) == len(inputs)
    with sqlite3.connect(":memory:") as db:
        db.executescript("""
            CREATE TABLE ignition(occupancy TEXT, a REAL, b REAL, mean REAL);
            CREATE TABLE suppression(method TEXT, probability REAL);
            INSERT INTO suppression VALUES ('absent',1),('standard',0.02),('other_sprinkler',0.05),('water',0.1),('gas',0.1);
            CREATE TABLE targets(occupancy TEXT, impact TEXT, probability REAL);
            INSERT INTO targets VALUES ('office','high',1.3e-6),('office','medium',1.3e-5),('office','low',1.1e-4),
                ('other','high',1e-7),('other','medium',1.3e-6),('other','low',1.3e-5),
                ('agricultural','high',NULL),('agricultural','medium',1.3e-5),('agricultural','low',1.1e-4);
        """)
        db.executemany("INSERT INTO ignition VALUES (?,?,?,?)", ignition)
        for scenario, result in zip(inputs, actual):
            expected = independent_outcome(db, scenario)
            if expected is None:
                assert result == "unavailable", (scenario, result)
            else:
                values = [float(value) for value in result.split("\t")]
                assert len(values) == len(expected)
                assert all(math.isclose(a, b, rel_tol=2e-11, abs_tol=2e-14) for a, b in zip(values, expected)), (scenario, values, expected)
    print(f"[DEBUG] {len(inputs)} Annex BB.5 scenarios matched production Rust, SQLite, and SciPy.")


if __name__ == "__main__":
    main()
