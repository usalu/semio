from pathlib import Path
import json,sqlite3
p=Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/📦️lease/🧪️tests/🧫️fixtures/🔣️.json")
fixture=json.loads(p.read_text())
cases=fixture["witnessSelection"]
db=sqlite3.connect(":memory:")
db.execute("create table cases(id text,fuel integer,quote integer,lookups integer,runs integer)")
db.executemany("insert into cases values (:id,:fuel,:quote,:lookups,:runs)",cases)
assert db.execute("select count(*) from cases where runs != (fuel>=quote) or lookups != runs").fetchone()[0]==0
assert db.execute("select :recover",{"recover":fixture["blockedReselect"]}).fetchone()[0]
print(f"cancellation witness oracle: {len(cases)} cases passed")
