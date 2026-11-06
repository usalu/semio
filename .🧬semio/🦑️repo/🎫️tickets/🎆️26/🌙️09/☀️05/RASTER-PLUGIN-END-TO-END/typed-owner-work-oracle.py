from pathlib import Path
import json
import sqlite3

source = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🧪️tests/🧫️fixtures/🔣️.json")
cases = json.loads(source.read_text())["typedOwnerWork"]
db = sqlite3.connect(":memory:")
db.execute("create table cases (id text, admitted integer, actual integer, debit integer, retained integer)")
db.executemany("insert into cases values (:id,:admitted,:actual,:fuelDebit,:originalRetained)", cases)
assert db.execute("select count(*) from cases where debit != case when admitted then actual else 0 end or retained != (not admitted)").fetchone()[0] == 0
print(f"typed owner work oracle: {len(cases)} cases passed")
