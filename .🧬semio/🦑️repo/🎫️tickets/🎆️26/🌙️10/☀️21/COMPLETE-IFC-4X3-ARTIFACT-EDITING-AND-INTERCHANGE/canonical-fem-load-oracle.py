"""🗝️ Independent composite-key constraints corroborate canonical source identity scope."""
import json, sqlite3, struct, sys
from pathlib import Path
class ScopedIdentityOracle:
    def accepts(self, cases):
        raise NotImplementedError
class SqliteIdentityOracle(ScopedIdentityOracle):
    def accepts(self, cases):
        with sqlite3.connect(":memory:") as db:
            db.execute("CREATE TABLE cases(id TEXT PRIMARY KEY NOT NULL CHECK(length(id)>0))")
            db.execute("CREATE TABLE loads(owner TEXT NOT NULL, id TEXT NOT NULL CHECK(length(id)>0), PRIMARY KEY(owner,id), FOREIGN KEY(owner) REFERENCES cases(id))")
            try:
                for case in cases:
                    db.execute("INSERT INTO cases VALUES (?)",(case["id"],))
                    for word in case["loads"]:
                        db.execute("INSERT INTO loads VALUES (?,?)",(case["id"],word))
                return True
            except sqlite3.IntegrityError:
                return False
cases=json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
oracle=SqliteIdentityOracle()
rows=[{"id":row["id"],"expected":row["valid"],"actual":oracle.accepts(row["cases"])} for row in cases["identities"]]
assert all(row["expected"]==row["actual"] for row in rows)
bits=[int(word) for word in cases["bits"]]
assert all(struct.unpack(">Q",struct.pack(">d",struct.unpack(">d",word.to_bytes(8,"big"))[0]))[0]==word for word in bits)
print(json.dumps({"sqlite":sqlite3.sqlite_version,"python":sys.version,"identityCases":rows,"binary64Bits":cases["bits"],"nativeExecuted":False,"fundingCertified":False},indent=2))
