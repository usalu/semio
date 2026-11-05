"""🧭 Independently check the neutral IO graph through SQLite recursive paths."""
from pathlib import Path
import json
import sqlite3

root = Path(__file__).resolve().parent
case = json.loads((root / "io-route-metadata.case.json").read_text())
ranks = {"Lossy": 0, "Semantic": 1, "Canonical": 2, "Exact": 3}
names = {value: key for key, value in ranks.items()}
db = sqlite3.connect(":memory:")
db.execute("CREATE TABLE edges(src TEXT, dst TEXT, rank INTEGER, direction TEXT, native TEXT, FOREIGN_KEY TEXT, PRIMARY KEY(src,dst))")
db.executemany("INSERT INTO edges VALUES(?,?,?,?,?,?)", [(row["from"], row["into"], ranks[row["fidelity"]], row["direction"], row["native"], row["foreign"]) for row in case["rows"]])
query = """
WITH RECURSIVE paths(node, depth, rank, visited) AS (
 SELECT ?, 0, 3, char(31)||?||char(31)
 UNION ALL
 SELECT e.dst, p.depth+1, min(p.rank,e.rank), p.visited||e.dst||char(31)
 FROM paths p JOIN edges e ON e.src=p.node
 WHERE p.depth<? AND instr(p.visited,char(31)||e.dst||char(31))=0
)
SELECT rank,depth FROM paths WHERE node=? AND rank>=?
ORDER BY rank DESC,depth ASC LIMIT 1
"""
observations = []
for law in case["routeCases"]:
    row = db.execute(query, (law["from"], law["from"], law["maximumHops"], law["into"], ranks[law["minimumFidelity"]])).fetchone()
    actual = {"reachable": False} if row is None else {"reachable": True, "fidelity": names[row[0]], "hops": row[1]}
    assert actual == law["expected"], (law["id"], actual, law["expected"])
    observations.append({"id": law["id"], "actual": actual})
for native in case["discovery"]["perNative"]:
    for direction, field in [("import", "imports"), ("export", "exports")]:
        actual = [row[0] for row in db.execute("SELECT FOREIGN_KEY FROM edges WHERE native=? AND direction=? ORDER BY FOREIGN_KEY", (native["native"], direction))]
        assert actual == native[field], (native["native"], field, actual)
destinations = [row[0] for row in db.execute("SELECT DISTINCT dst FROM edges WHERE direction='export' ORDER BY dst")]
assert destinations == case["discovery"]["pluginDestinations"]
counts = dict(db.execute("SELECT direction,count(*) FROM edges GROUP BY direction"))
assert counts == {"import": 7, "export": 13}
assert db.execute("PRAGMA integrity_check").fetchone() == ("ok",)
report = {"oracle": "SQLite recursive directed paths", "sqliteVersion": sqlite3.sqlite_version, "routeCases": observations, "perNativeDiscoveryCases": 6, "pluginDestinationCount": len(destinations), "registeredRowCount": sum(counts.values()), "productionExecuted": False}
out = root / "🗑️generated" / "io-route-sqlite-oracle.json"
out.parent.mkdir(exist_ok=True)
out.write_text(json.dumps(report, indent=2) + chr(10))
db.close()
print(json.dumps({"routeCasesPassed": len(observations), "discoveryCasesPassed": 7, "registeredRows": 20, "productionExecuted": False}))
