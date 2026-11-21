import csv
import re
import sqlite3
import subprocess
from pathlib import Path

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
artifact = next((workspace/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599").glob("🏅*/🔖*/🪆*/✳*"))
checks = artifact/"🧬️schema/💡️inferences/⚖️checks"
tables = (checks/"📜️norm-tables/🦀️.rs").read_text()
balance = (checks/"🧮️balance-model/🦀️.rs").read_text()
factor_module = tables[tables.index("pub mod geg_anlage4_primary_energy_factors"):].split("\n}\n", 1)[0] + "\n}\n"
functions = balance[balance.index("pub fn primary_energy_factor("):balance.index("/// 🎛️ DIN V 18599-11")]
binary = ticket/"🗑️generated/din18599-primary-energy-factors"
source = binary.with_suffix(".rs")
source.write_text(factor_module + functions.replace('pub(super) fn', 'fn') + """
fn main() {
    for (index, row) in geg_anlage4_primary_energy_factors::ROWS.iter().enumerate() {
        println!("{}\\t{}\\t{:?}\\t{}\\t{}", index + 1, row.carrier, row.factor, primary_energy_factor(row.carrier), has_direct_primary_energy_factor(row.carrier));
    }
    for carrier in ["heat_pump_electric", "district_heating", "unknown"] {
        println!("{}\\t{}\\t{}", carrier, has_direct_primary_energy_factor(carrier), uses_grid_electricity(carrier));
    }
}
""")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc",
                "--edition", "2024", str(source), "-o", str(binary)], cwd=workspace, check=True)
actual = subprocess.check_output([binary], text=True).splitlines()
with (artifact/"🧪️tests/🧫️fixtures/💡primary-energy-factors/🔣️.tsv").open() as fixture:
    data = list(csv.DictReader(fixture, delimiter="\t"))
db = sqlite3.connect(":memory:")
db.execute("CREATE TABLE annex (n INTEGER PRIMARY KEY, carrier TEXT, factor REAL)")
db.executemany("INSERT INTO annex VALUES (?,?,?)", (
    (int(row["annex_row"]), row["carrier"], None if row["factor"] == "Method B" else float(row["factor"]))
    for row in data
))
rows = db.execute("SELECT n, carrier, factor, n <= 9 FROM annex ORDER BY n").fetchall()
assert len(actual) == len(rows) + 3
for (number, carrier, factor, direct), line in zip(rows, actual):
    index, key, optional, used, basis = line.split("\t")
    assert (int(index), key, basis == "true") == (number, carrier, bool(direct)), line
    if factor is None:
        assert optional == "None", line
    else:
        assert re.fullmatch(r"Some\([0-9.]+\)", optional), line
        assert float(optional[5:-1]) == float(used) == factor, line
assert actual[-3:] == ["heat_pump_electric\ttrue\ttrue", "district_heating\tfalse\tfalse", "unknown\tfalse\tfalse"]
print(f"[DEBUG] {len(rows)} Annex 4 rows matched independent SQLite and three factor-basis cases passed.")
