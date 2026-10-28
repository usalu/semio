"""🪃️ Independent SQLite transaction oracle for the original stream-abort fixture."""
import json
import sqlite3
from pathlib import Path

root = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
fixture = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/abort.json"
law = json.loads(fixture.read_text())
db = sqlite3.connect(":memory:")
db.executescript("CREATE TABLE model(value REAL); CREATE TABLE edits(id TEXT); CREATE TABLE messages(id TEXT); CREATE TABLE applied(id TEXT); CREATE TABLE redo(id TEXT); CREATE TABLE transitions(id TEXT); CREATE TABLE outbox(id TEXT);")
db.execute("INSERT INTO model VALUES (?)", (law["genesis"],))
db.commit()
before = {table: db.execute(f"SELECT * FROM {table}").fetchall() for table in ["model", "messages", "edits", "applied", "redo", "transitions", "outbox"]}
db.execute("BEGIN")
db.execute("INSERT INTO edits VALUES (?)", (law["transaction"],))
db.execute("INSERT INTO messages VALUES (?)", (law["transaction"],))
db.execute("INSERT INTO applied VALUES (?)", (law["transaction"],))
db.execute("UPDATE model SET value = ?", (law["streamed"],))
assert db.execute("SELECT value FROM model").fetchone()[0] == law["streamed"]
db.rollback()
after = {table: db.execute(f"SELECT * FROM {table}").fetchall() for table in before}
assert after == before
assert after["model"][0][0] == law["restored"]
for table in ["messages", "edits", "applied", "redo", "transitions", "outbox"]:
    assert len(after[table]) == law["afterAbort"][table]
print("SQLite original stream abort: restored projection, zero history and transport trace")
