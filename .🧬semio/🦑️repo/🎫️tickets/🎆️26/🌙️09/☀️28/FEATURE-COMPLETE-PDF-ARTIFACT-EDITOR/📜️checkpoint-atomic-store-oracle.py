"""🧪 Independently verify the mixed Check In checkpoint's neutral edit law with SQLite."""
import json
from pathlib import Path
import sqlite3

fixture = json.loads(Path(__file__).with_name("🔣️checkpoint-atomic-store-law.json").read_text())
connection = sqlite3.connect(":memory:")
connection.executescript("""
CREATE TABLE checkpoint_ledger (sequence INTEGER PRIMARY KEY, authored_edit TEXT NOT NULL, sample INTEGER NOT NULL);
CREATE TABLE checkpoint_cursor (baseline INTEGER NOT NULL);
""")
connection.execute("INSERT INTO checkpoint_cursor VALUES (?)", (fixture["baseline"],))
connection.executemany("INSERT INTO checkpoint_ledger VALUES (?, ?, ?)",
                       ((i, fixture["transaction"]["id"], value) for i, value in enumerate(fixture["values"])))
reply = dict(zip(("after", "undo", "rows", "edits"), connection.execute("""
SELECT (SELECT sample FROM checkpoint_ledger ORDER BY sequence DESC LIMIT 1),
       (SELECT baseline FROM checkpoint_cursor),
       COUNT(*), COUNT(DISTINCT authored_edit)
FROM checkpoint_ledger
""").fetchone()))
assert reply == fixture["expected"], reply
print("[DEBUG] SQLite verified", reply)

draft = fixture["draft"]
connection.execute("CREATE TABLE local_transaction (value INTEGER NOT NULL, revision INTEGER NOT NULL)")
connection.execute("INSERT INTO local_transaction VALUES (?, 0)", (draft["value"],))
gap_value, gap_revision = connection.execute("SELECT value, revision FROM local_transaction").fetchone()
merged_draft = connection.execute("""
SELECT coalesce((SELECT value FROM local_transaction),
                (SELECT sample FROM checkpoint_ledger ORDER BY sequence DESC LIMIT 1))
""").fetchone()[0]
connection.execute("DELETE FROM local_transaction")
aborted_draft, committed_edits = connection.execute("""
SELECT (SELECT sample FROM checkpoint_ledger ORDER BY sequence DESC LIMIT 1),
       count(DISTINCT authored_edit) FROM checkpoint_ledger
""").fetchone()
draft_reply = dict(zip(("gapValue", "gapRevision", "mergedDraft", "abortedDraft", "committedEdits"),
                       (gap_value, gap_revision, merged_draft, aborted_draft, committed_edits)))
assert draft_reply == draft["expected"], draft_reply
print("[DEBUG] SQLite verified the retained draft", draft_reply)
connection.close()
