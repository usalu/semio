from pathlib import Path
from json import loads
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
artifact = next((root/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108").glob("🏅*/🔖*/🪆*/✳*"))
cases = loads((artifact/"🧪️tests/🧫️fixtures/📋️airtightness-concept/🔣️.json").read_text())
catalogue = (artifact/"🧬️schema/💡️inferences/⚖️checks/🌬️part-7-airtightness/📋️concept/🦀️.rs").read_text()
presence = catalogue.split("pub(crate) fn documentation_missing",1)[1].split("\nfn plan_records",1)[0].strip()
binary = ticket/"🗑️generated/din4108-airtightness-concept"
source = binary.with_suffix(".rs")
def rust_literal(value):
    return '"' + value.replace("\\","\\\\").replace('"','\\"').replace("\t","\\t").replace("\n","\\n").replace("\r","\\r") + '"'
checks = []
db = sqlite3.connect(":memory:")
for case in cases:
    result = [db.execute("SELECT trim(?, char(9,10,13,32,160))=''",(record,)).fetchone()[0] for record in case["records"]]
    assert result == case["missing"], case
    args = ", ".join(map(rust_literal,case["records"]))
    expected = ", ".join(str(item).lower() for item in case["missing"])
    checks.append(f"    assert_eq!(documentation_missing([{args}]), [{expected}]);")
source.write_text("fn documentation_missing" + presence + "\nfn main() {\n" + "\n".join(checks) + "\n}\n")
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024",str(source),"-o",str(binary)],check=True,cwd=root)
subprocess.run([binary],check=True,cwd=root)
print(f"[DEBUG] DIN 4108-7 section 4 plan evidence {len(cases)} cases agreed between Rust and SQLite.")
