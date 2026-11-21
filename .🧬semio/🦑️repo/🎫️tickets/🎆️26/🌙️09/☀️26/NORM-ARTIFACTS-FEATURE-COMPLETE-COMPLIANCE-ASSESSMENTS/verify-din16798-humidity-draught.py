"""💨 Check humidity and draught reference data, predictions and remedy bounds."""
import math
from pathlib import Path
import sqlite3
import subprocess
from scipy.optimize import brentq


ticket = Path(__file__).resolve().parent
workspace = next(p.parent for p in ticket.parents if p.name == ".🧬semio")
artifact = next((workspace / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798").glob("🏅*/🔖*/🪆*/✳*"))
code = (artifact / "🧬️schema/💡️inferences/⚖️checks/📜️annex-parameters-and-parts/🦀️.rs").read_text()
humidity = code[code.index("    pub fn humidity_band"):code.index("    /// 🫧 Standard design")]
draught = code[code.index("    pub fn draught_rate_percent"):code.index("\n}\n\npub mod part_3")]
binary = ticket / "🗑️generated/din16798-humidity-draught"
source = binary.with_suffix(".rs")
source.write_text("""
#![allow(private_interfaces)]
use std::io::{self, BufRead};
#[derive(Clone,Copy)]
enum AnnexChoice { En, De }
#[derive(Clone,Copy)]
enum ComfortCategory { I, II, III, IV }
""" + humidity + draught + r"""
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split('\t').collect();
        let annex = if fields[0] == "de" { AnnexChoice::De } else { AnnexChoice::En };
        let category = match fields[1] {
            "I" => ComfortCategory::I, "II" => ComfortCategory::II, "III" => ComfortCategory::III, _ => ComfortCategory::IV
        };
        let parse = |index: usize| fields[index].parse::<f64>().unwrap();
        let (t, v, tu) = (parse(2), parse(3), parse(4));
        let (low, high) = humidity_band(annex, category).map_or(("-".into(), "-".into()), |(a,b)| (a.to_string(),b.to_string()));
        let limit = draught_limit_percent(annex, category);
        let rate = draught_rate_percent(t, v, tu);
        let remedy = match (rate, limit) {
            (Some(rate),Some(limit)) if rate > limit => Some(maximum_air_speed_for_draught(t, v, tu, limit)),
            _ => None,
        };
        let as_text = |value: Option<f64>| value.map_or("-".into(), |num| num.to_string());
        println!("{low}\t{high}\t{}\t{}\t{}", as_text(limit), as_text(rate), as_text(remedy));
    }
}
""")
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024",str(source),"-o",str(binary)],check=True,cwd=workspace)
data = (artifact / "🧪️tests/🧫️fixtures/🌡️humidity-and-draught/🔣️.tsv").read_text().splitlines()[1:]
observed = subprocess.check_output([binary], input="\n".join(data) + "\n", text=True).splitlines()
assert len(observed) == len(data)
db = sqlite3.connect(":memory:")
db.executescript("""
CREATE TABLE reference (category TEXT PRIMARY KEY, humid_en REAL, humid_de REAL, dehumid REAL, draught_en REAL, draught_de REAL);
INSERT INTO reference VALUES ('I',30,40,50,10,10),('II',25,30,60,20,15),('III',20,20,70,30,30),('IV',NULL,NULL,NULL,NULL,NULL);
""")
remedies = 0
for row, actual in zip(data, observed):
    annex, category, temperature, speed, turbulence, lower, upper, limit, scope = row.split("\t")
    raw_lower, raw_upper, raw_limit, raw_rate, raw_remedy = actual.split("\t")
    values = db.execute("""
        SELECT CASE WHEN :annex='de' THEN humid_de ELSE humid_en END, dehumid,
               CASE WHEN :annex='de' THEN draught_de ELSE draught_en END
        FROM reference WHERE category=:category
        """, {"annex": annex, "category": category}).fetchone()
    numbers = tuple(None if val == "-" else float(val) for val in (lower, upper, limit))
    assert tuple(None if val == "-" else float(val) for val in (raw_lower, raw_upper, raw_limit)) == values == numbers, row
    t, v, tu = map(float, (temperature, speed, turbulence))
    valid = 20 <= t <= 26 and 0 <= v < .5 and 10 <= tu <= 60
    assert valid == (scope == "yes") == (raw_rate != "-"), row
    if valid:
        def percent(speed):
            air = max(speed, .05)
            return min(100, (34-t) * (air-.05)**.62 * (3.14+.37*tu*air))
        expected = percent(v)
        assert math.isclose(float(raw_rate), expected, abs_tol=1e-11), row
        if values[2] is not None and expected > values[2]:
            root = brentq(lambda velocity: percent(velocity)-values[2], .05, v)
            assert math.isclose(float(raw_remedy), math.floor(root*1000)/1000, abs_tol=1e-11), row
            remedies += 1
        else:
            assert raw_remedy == "-", row
print(f"[DEBUG] {len(data)} humidity and draught scenarios matched Rust, SQLite reference data and SciPy. {remedies} speed remedies matched the independent root solve.")
