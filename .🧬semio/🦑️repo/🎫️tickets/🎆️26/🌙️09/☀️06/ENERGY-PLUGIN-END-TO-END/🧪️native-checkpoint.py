"""🚩️ Independent SQLite and BLAKE3 reference for authored native checkpoint metadata."""
import json
import sqlite3
from pathlib import Path
from blake3 import blake3

root = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
law = json.loads((root / "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original/commit-facts.json").read_text())
db = sqlite3.connect(":memory:")
db.execute("CREATE TABLE changes(rank INTEGER PRIMARY KEY, id TEXT)")
db.executemany("INSERT INTO changes VALUES (?,?)", enumerate(law["parent"]["changeIds"] + [law["changeId"]]))
checkpoint = {"id": "", "changeIds": [row[0] for row in db.execute("SELECT id FROM changes ORDER BY rank")], "parentId": law["parent"]["id"], "message": law["message"], "authors": law["authors"], "timestamp": law["timestamp"]}
assert checkpoint == law["expected"]["checkpoint"]
message = law["message"] + law["identity"]["messageSuffixPattern"] * law["identity"]["repeat"]
source = law["parent"]["id"].encode() + b"\0"
for change in law["committed"] + [law["expected"]["change"]]:
    source += blake3(json.dumps(change, ensure_ascii=False, separators=(",", ":")).encode()).digest()
source += b"\0" + message.encode() + b"\0"
for author in law["authors"]:
    source += author["id"].encode() + b"\0"
source += b"\0" + law["timestamp"].encode()
assert blake3(source).hexdigest() == law["identity"]["digestHex"]
print("SQLite checkpoint facts and BLAKE3 authored Unicode commitment agree")

checkout_path = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/checkout.json"
checkout = json.loads(checkout_path.read_text())
reference = sqlite3.connect(":memory:")
reference.executescript("CREATE TABLE edits (id TEXT PRIMARY KEY, zoom REAL); CREATE TABLE members (checkpoint TEXT, position INTEGER, edit TEXT); CREATE TABLE head (zoom REAL); CREATE TABLE transitions (id TEXT); CREATE TABLE outbox (id TEXT);")
reference.execute("INSERT INTO edits VALUES (?, ?)", ("original-publication-edit", checkout["visibleAfter"]))
reference.execute("INSERT INTO members VALUES (?, ?, ?)", ("checkpoint", 0, "original-publication-edit"))
reference.execute("INSERT INTO head VALUES (?)", (checkout["visibleBefore"],))
reference.execute("UPDATE head SET zoom = (SELECT edits.zoom FROM members JOIN edits ON edits.id = members.edit WHERE members.checkpoint = ? ORDER BY members.position DESC LIMIT 1)", ("checkpoint",))
assert reference.execute("SELECT zoom FROM head").fetchone()[0] == checkout["visibleAfter"]
assert reference.execute("SELECT count(*) FROM transitions").fetchone()[0] == checkout["newTransitions"]
assert reference.execute("SELECT count(*) FROM outbox").fetchone()[0] == checkout["newOutbox"]
print("SQLite checkpoint checkout agrees without a new transition or outbox row")
