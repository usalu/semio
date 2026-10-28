"""📊️ Independently exercise reporting event identity, order and inverse with SQLite."""
import json
import sqlite3
from pathlib import Path
import jsonschema

root = next(p for p in Path(__file__).resolve().parents if (p / ".🧬semio").is_dir())
mutations = root / "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
law = json.loads((mutations / "🧳️native/📊️outputs/🔣️.json").read_text())
recipe_schema = json.loads((mutations / "✨️derive/🧬️schema/🔣️.json").read_text())
for leaf in ["📊️add-output-variable", "📉️remove-output-variable"]:
    schema = json.loads((mutations / "🔋️model" / leaf / "🧬️schema/🔣️.json").read_text())
    jsonschema.Draft202012Validator(recipe_schema).validate(schema["x-semio-energy-operation"])

with sqlite3.connect(":memory:") as db:
    db.execute("CREATE TABLE registrations (position INTEGER PRIMARY KEY, name TEXT, key TEXT, frequency TEXT, UNIQUE(name,key))")
    for case in law["cases"]:
        db.execute("DELETE FROM registrations")
        db.executemany("INSERT INTO registrations VALUES (?,?,?,?)", [(i, v["name"], v["key"], v["reporting_frequency"]) for i, v in enumerate(case["before"])])
        p = case["payload"]
        old = db.execute("SELECT position,frequency FROM registrations WHERE name=? AND key=?", [p["name"], p["key"]]).fetchone()
        if case["insert"]:
            rank = len(case["before"]) if p["index"] is None else p["index"]
            outcome = "invalid" if not p["name"].strip() else "duplicate" if old else "missing" if rank > len(case["before"]) else "changed"
            if outcome == "changed":
                db.execute("UPDATE registrations SET position=-position-1 WHERE position>=?", [rank])
                db.execute("UPDATE registrations SET position=-position WHERE position<0")
                db.execute("INSERT INTO registrations VALUES (?,?,?,?)", [rank, p["name"], p["key"], p["reportingFrequency"]])
        else:
            outcome = "changed" if old else "missing"
            if old:
                assert case["inverse"] == {"name": p["name"], "key": p["key"], "reportingFrequency": old[1], "index": old[0]}
                db.execute("DELETE FROM registrations WHERE name=? AND key=?", [p["name"], p["key"]])
        assert outcome == case["outcome"]
        if outcome == "changed":
            actual = [{"name": n, "key": k, "reporting_frequency": f} for n, k, f in db.execute("SELECT name,key,frequency FROM registrations ORDER BY position")]
            assert actual == case["after"]
print(f"Native output registration oracle: {len(law['cases'])} SQLite cases and both recipe schemas passed")
