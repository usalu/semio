"""🧱️ Compares the retained EN 1998 jacket kernel with SQLite and clause fixtures."""
from pathlib import Path
import csv
import math
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
work = ticket / "🗑️generated"
work.mkdir(exist_ok=True)
module = Path("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📐️formulas/🧱️retrofit/🦀️.rs")
program = work / "en1998_retrofit.rs"
raw_cases = (ticket / "en1998-retrofit-cases.tsv").read_text().splitlines()

def fixture(row):
    """🔣️ Parse one language-neutral clause vector."""
    kind, applies, *fields, outcome, capacity = row
    return kind, int(applies), [float(x) for x in fields], int(outcome), float(capacity) if capacity else None

rust = '''
use std::io::{self, BufRead};
fn main() {
    for row in io::stdin().lock().lines() {
        let row = row.unwrap();
        let f: Vec<_> = row.split('\\t').collect();
        let a: Vec<f64> = f[2..8].iter().map(|v| v.parse().unwrap()).collect();
        let result = if f[1] == "0" { None } else { match f[0] {
            "steel-shear" => SteelJacket { depth_m: a[0], thickness_m: a[1], width_spacing_ratio: a[2], strut_deg: a[3], strap_deg: a[4], design_yield_pa: a[5] }.shear_contribution_n().map(|value| (value, true)),
            "splice" => SpliceClamp { splice_m: a[0], jacket_m: a[1], near_rows: a[2], far_rows: a[3], at_column_base: a[4]>0.0, base_pattern_confirmed: a[5]>0.0 }.evaluate(),
            case => concrete_jacket(
                if case == "jacket-shear" {a[0]} else {120_000.0},
                if case == "jacket-moment" {a[0]} else {150_000.0},
                if case == "jacket-yield" {a[0]} else {0.016},
                if case == "jacket-ultimate" {a[0]} else {0.035},
            ).map(|r| (r[["jacket-shear","jacket-moment","jacket-yield","jacket-ultimate"].iter().position(|v| *v==case).unwrap()], true)),
        }};
        match result {
            Some((value, pass)) => println!("{}\\t{value:.16}", if pass { 1 } else {-1}),
            None => println!("0"),
        }
    }
}
'''
program.write_text(module.read_text() + "\n" + rust)
binary = work / "en1998_retrofit"
subprocess.run(["rustc", "--edition=2024", str(program), "-o", str(binary)], check=True)
actual = subprocess.run([str(binary)], input="\n".join(raw_cases[1:]), text=True, check=True, capture_output=True).stdout.splitlines()
connection = sqlite3.connect(":memory:")
connection.create_function("sin", 1, math.sin)
connection.create_function("tan", 1, math.tan)
query = '''
WITH inputs(kind, applies, a,b,c,d,e,f) AS (VALUES(?,?,?,?,?,?,?,?)),
    projections AS (
        SELECT *,
        CASE
        WHEN applies = 0 THEN NULL
        WHEN kind = 'steel-shear' AND a>0 AND b>0 AND c>0 AND c<=1 AND d>0 AND d<90 AND e>0 AND e<=90 AND f>0
            THEN (a/2) * (2*b*c) * f * (1/tan(d*0.017453292519943295) + CASE WHEN e=90 THEN 0 ELSE 1/tan(e*0.017453292519943295) END) * sin(e*0.017453292519943295)
        WHEN kind = 'splice' AND a>0 AND b>0 AND c >= 0 AND d >= 0 AND c = CAST(c AS INT) AND d = CAST(d AS INT) THEN a*3/2
        WHEN kind <> 'steel-shear' AND kind <> 'splice' AND a > 0 THEN a * CASE kind WHEN 'jacket-shear' THEN .9 WHEN 'jacket-yield' THEN 1.05 ELSE 1 END
        END capacity
        FROM inputs
    ) SELECT CASE
        WHEN capacity IS NULL THEN 0
        WHEN kind = 'splice' AND (b+1e-14 < capacity OR c<2 OR d<2 OR (e=1 AND f=0)) THEN -1
        ELSE 1 END, capacity FROM projections
'''
cases = list(map(fixture, csv.reader(raw_cases[1:], delimiter="\t")))
assert len(actual) == len(cases)
for i, ((kind, applies, args, outcome, expected), raw) in enumerate(zip(cases, actual), 1):
    independent_outcome, independent = connection.execute(query, (kind, applies, *args)).fetchone()
    values = raw.split("\t")
    assert int(values[0]) == independent_outcome == outcome, (i, kind, values, independent_outcome, outcome)
    if expected is not None:
        assert math.isclose(float(values[1]), expected, rel_tol=1e-12), (i, kind, values, expected)
        assert math.isclose(independent, expected, rel_tol=1e-12), (i, kind, independent, expected)
    else:
        assert len(values) == 1 and independent is None, (i, kind, values, independent)
print(f"[DEBUG] EN 1998 Annex A retrofit expectations, Rust and SQLite agreed on {len(cases)} cases")
