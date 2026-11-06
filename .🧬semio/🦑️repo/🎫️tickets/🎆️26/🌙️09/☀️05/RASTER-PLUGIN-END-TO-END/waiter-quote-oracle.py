from pathlib import Path
import json,sqlite3
p=Path("🧰️framework/🔨️modules/⏳️async/🛑️cancel/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")
case=json.loads(p.read_text())["ownedWaiterQuote"]
db=sqlite3.connect(":memory:")
assert db.execute("select :releaseElements=:capacity and :capacity>0 and not :requiresLock",case).fetchone()[0]
print("owned waiter quote oracle: 1 case passed")
