import json
import pathlib
import sqlite3

root = next(parent for parent in pathlib.Path(__file__).resolve().parents if (parent / "✏️s").is_dir())
fixture = next(root.glob("✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧫️fixtures/🌡️thermostat-zone/🔣️.json"))
law = json.loads(fixture.read_text())
with sqlite3.connect(":memory:") as db:
    db.execute("PRAGMA foreign_keys=ON")
    db.execute("CREATE TABLE zones(id INTEGER PRIMARY KEY)")
    db.execute("CREATE TABLE thermostats(id INTEGER PRIMARY KEY, zone INTEGER REFERENCES zones(id), heat INTEGER, cool INTEGER, heat_range REAL, cool_range REAL)")
    db.executemany("INSERT INTO zones VALUES(?)", [(1,), (law["replacement"],)])
    db.execute("INSERT INTO thermostats VALUES(62,1,10,20,2.0,3.0)")
    db.execute("UPDATE thermostats SET zone=? WHERE id=62", (law["replacement"],))
    assert db.execute("SELECT * FROM thermostats").fetchone() == (62, law["replacement"], 10, 20, 2.0, 3.0)
    try:
        db.execute("UPDATE thermostats SET zone=? WHERE id=62", (law["missing"],))
    except sqlite3.IntegrityError:
        pass
    else:
        raise AssertionError("foreign zone admitted")
    assert db.execute("SELECT zone FROM thermostats").fetchone() == (law["replacement"],)
print("thermostat zone independent SQLite reference and atomic refusal: PASS")
