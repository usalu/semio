from pathlib import Path
import json
import sqlite3

fixture = Path("✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/identity.json")
for case in json.loads(fixture.read_text()):
    database = sqlite3.connect(":memory:")
    database.execute("create table rows (position integer primary key, id integer unique not null)")
    survivors = [key for key in case["base"] if key not in case["removed"]]
    after = survivors.copy()
    for position, key in case["inserted"]:
        after.insert(position, key)
    accepted = True
    try:
        database.executemany("insert into rows values (?, ?)", enumerate(after))
    except sqlite3.IntegrityError:
        accepted = False
    assert accepted == case["accepted"], case["name"]
    if accepted:
        actual = [row[0] for row in database.execute("select id from rows order by position")]
        assert actual == case["expected"], (case["name"], actual)
    database.close()
    print(f"[DEBUG] {case['name']}: {'applies' if accepted else 'refuses'}")
print(f"[DEBUG] SQLite {sqlite3.sqlite_version}: all row identity oracle cases pass")

case = json.loads((fixture.parent / "slots.json").read_text())
database = sqlite3.connect(":memory:")
database.execute("create table slots (position integer primary key, value real not null)")
for slot in case["earlier"] + case["later"]:
    database.execute("insert into slots values (?, ?) on conflict(position) do update set value=excluded.value", (slot["index"], slot["value"]))
actual = [0.0] * len(case["expected"])
for position, value in database.execute("select position, value from slots order by position"):
    actual[position] = value
assert actual == case["expected"], actual
database.close()
print("[DEBUG] SQLite unordered slots and later assignments pass")
