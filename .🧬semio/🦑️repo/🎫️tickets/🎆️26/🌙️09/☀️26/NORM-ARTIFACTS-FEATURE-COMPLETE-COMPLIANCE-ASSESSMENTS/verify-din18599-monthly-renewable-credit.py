"""⚡ Verify the live monthly GModG §23 credit against SQLite."""
from pathlib import Path
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
workspace = next(p.parent for p in ticket.parents if p.name == ".🧬semio")
artifact = next((workspace / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599").glob("🏅*/🔖*/🪆*/✳*"))
code = (artifact / "🧬️schema/💡️inferences/⚖️checks/🔥️derived-balance/🦀️.rs").read_text()
start = code.index("pub fn monthly_renewable_electricity_credit_kwh")
end = code.index("\nfn balance_for", start)
binary = ticket / "🗑️generated/din18599-monthly-renewable-credit"
source = binary.with_suffix(".rs")
source.write_text("use std::io::{self,BufRead};\n" + code[start:end] + '''
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let (generated, demand) = line.split_once('\\t').unwrap();
        match monthly_renewable_electricity_credit_kwh(generated.parse().unwrap(), demand.parse().unwrap()) {
            Some(credit) => println!("{credit}"),
            None => println!("scope"),
        }
    }
}
''')
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc", "--edition", "2024", str(source), "-o", str(binary)], check=True, cwd=workspace)
data = (artifact / "🧪️tests/🧫️fixtures/☀️monthly-renewable-credit/🔣️.tsv").read_text().splitlines()[1:]
rows = [line.split("\t") for line in data]
inputs = "".join(generated + "\t" + demand + "\n" for _, generated, demand, _ in rows)
observed = subprocess.check_output([binary], input=inputs, text=True).splitlines()
assert len(observed) == len(rows)
db = sqlite3.connect(":memory:")
for actual, (scenario, generated, demand, expected) in zip(observed, rows):
    result, = db.execute("""
       SELECT CASE WHEN :g BETWEEN 0 AND 1.7976931348623157e308 AND :q BETWEEN 0 AND 1.7976931348623157e308
              THEN min(:g,:q) END
       """, {"g":float(generated), "q":float(demand)}).fetchone()
    if result is None:
        assert actual == expected == "scope", scenario
    else:
        assert float(actual) == float(expected) == result, scenario
print(f"[DEBUG] {len(rows)} scenarios matched the production GModG §23 month predicate and SQLite.")
