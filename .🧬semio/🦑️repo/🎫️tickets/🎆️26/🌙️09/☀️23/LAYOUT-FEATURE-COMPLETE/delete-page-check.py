import json
from pathlib import Path
import sqlite3


def decision(case):
    with sqlite3.connect(":memory:") as connection:
        connection.executescript("""
            CREATE TABLE page (ordinal INTEGER PRIMARY KEY, id TEXT, spread TEXT);
            CREATE TABLE frame (page INTEGER, id TEXT, successor TEXT);
            CREATE TABLE spread (ordinal INTEGER PRIMARY KEY, id TEXT);
            CREATE TABLE member (spread INTEGER, ordinal INTEGER, id TEXT);
        """)
        prefix_spec = case.get("framePrefix", {})
        prefix = prefix_spec.get("character", "") * prefix_spec.get("count", 0)
        for ordinal, page in enumerate(case["pages"]):
            connection.execute("INSERT INTO page VALUES (?, ?, ?)", (ordinal, page["id"], page["spread"]))
            connection.executemany("INSERT INTO frame VALUES (?, ?, ?)", [
                (ordinal, prefix + frame_id, prefix + successor if successor is not None else None)
                for frame_id, successor in page["frames"]
            ])
        for ordinal, (spread, members) in enumerate(case["spreads"]):
            connection.execute("INSERT INTO spread VALUES (?, ?)", (ordinal, spread))
            connection.executemany("INSERT INTO member VALUES (?, ?, ?)", [
                (ordinal, member_ordinal, member) for member_ordinal, member in enumerate(members)
            ])
        page = connection.execute(
            "SELECT ordinal, spread FROM page WHERE id = ? ORDER BY ordinal LIMIT 1", (case["delete"],)
        ).fetchone()
        if page is None:
            return {"error": "missing-page", "code": "mutation.target-missing"}
        inbound = connection.execute("""
            SELECT 1 FROM frame AS source
            JOIN frame AS destination ON source.successor = destination.id
            WHERE destination.page = ?
            AND source.id NOT IN (SELECT id FROM frame WHERE page = ?)
            LIMIT 1
        """, (page[0], page[0])).fetchone()
        if inbound is not None:
            return {"error": "surviving-thread", "code": "mutation.invariant"}
        spread = connection.execute(
            "SELECT ordinal FROM spread WHERE id = ? ORDER BY ordinal LIMIT 1", (page[1],)
        ).fetchone()
        if spread is None:
            return {"error": "missing-spread", "code": "mutation.target-missing", "page": page[0]}
        member = connection.execute(
            "SELECT ordinal FROM member WHERE spread = ? AND id = ? ORDER BY ordinal LIMIT 1",
            (spread[0], case["delete"]),
        ).fetchone()
        return {"page": page[0], "spread": spread[0], "member": member[0] if member is not None else None}


fixture_path = next(Path("✏️s/🔌️plugins/📏️layout").glob("**/🗑️page-deletion/🧪️tests/🔣️.json"))
with fixture_path.open(encoding="utf-8") as fixture_file:
    cases = json.load(fixture_file)["cases"]
for case in cases:
    actual = decision(case)
    assert actual == case["expected"], (case["name"], actual, case["expected"])
print(f"[DEBUG] {len(cases)} retained DeletePage decisions matched SQLite")

registered = next(Path("✏️s/🔌️plugins/📏️layout").glob("**/🧫️fixtures/🧬️mutations/🗑️delete-page"))


def load(directory, name):
    return json.loads((directory / name).read_text(encoding="utf-8"))


for directory in sorted(registered.iterdir()):
    before = load(directory, "📸️snapshot/⬅️before/🔣️.json")
    after = load(directory, "📸️snapshot/➡️after/🔣️.json")
    wire = load(directory, "🔺️diff/🔣️.json")
    command = load(directory, "🦠️mutation/🔣️.json")["DeletePage"]
    with sqlite3.connect(":memory:") as connection:
        page_at, spread_id = connection.execute("""
            SELECT key, json_extract(value, '$.spreadId')
            FROM json_each(?, '$.pages')
            WHERE json_extract(value, '$.id') = ?
        """, (json.dumps(before), command["id"])).fetchone()
        spread_at, raw_members = connection.execute("""
            SELECT key, json_extract(value, '$.pageIds')
            FROM json_each(?, '$.spreads')
            WHERE json_extract(value, '$.id') = ?
        """, (json.dumps(before), spread_id)).fetchone()
        kept = [member for member, in connection.execute(
            "SELECT value FROM json_each(?) WHERE value != ? ORDER BY key", (raw_members, command["id"])
        )]
        actual_after, = connection.execute(
            "SELECT json_set(json_remove(?, ?), ?, json(?))",
            (json.dumps(before), f"$.pages[{page_at}]", f"$.spreads[{spread_at}].pageIds", json.dumps(kept)),
        ).fetchone()
    assert json.loads(actual_after) == after
    assert wire["pages"]["removed"] == [{"index": page_at, "id": command["id"]}]
    assert wire["spreads"]["modified"] == [{"id": spread_id, "patch": {"name": None, "pageIds": kept}}]
print("[DEBUG] both registered DeletePage snapshots and sparse spread/page patches matched SQLite")
