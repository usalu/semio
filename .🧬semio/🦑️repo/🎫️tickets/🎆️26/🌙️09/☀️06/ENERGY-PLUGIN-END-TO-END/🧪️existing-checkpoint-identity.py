"""🔏️ Independent JSON/SQLite/BLAKE3 existing checkpoint commitment oracle."""
import json
import sqlite3
from pathlib import Path
import blake3
import jsonschema
folder=Path(__file__).resolve().parents[7]/"🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original"
f=json.loads((folder/"existing.json").read_text())
jsonschema.validate(f,json.loads((folder/"existing-schema.json").read_text()))
rows={row["id"]:row for row in f["changes"]}
data=f["parent"].encode()+b"\0"
for identity in f["changeIds"]:
    data+=blake3.blake3(json.dumps(rows[identity],ensure_ascii=False,separators=(",",":")).encode()).digest() if identity in rows else bytes(32)
data+=b"\0"+f["message"].encode()+b"\0"
for author in f["authors"]:
    data+=author.encode()+b"\0"
data+=b"\0"+f["timestamp"].encode()
connection=sqlite3.connect(":memory:")
connection.execute("create table pins(uri text, checkpoint text)")
for pin in f["pins"]:
    connection.execute("insert into pins values (? || '!' || ? || '@' || ? || '/' || ?,?)",(pin["id"],pin["kind"],pin["standard"],pin["subset"],pin["checkpoint"]))
if f["pins"]:
    data+=b"\0"
for uri,checkpoint in connection.execute("select uri,checkpoint from pins order by cast(uri as blob)"):
    data+=uri.encode()+b"\0"+checkpoint.encode()+b"\0"
assert blake3.blake3(data).hexdigest()==f["digest"]
print("[DEBUG] Existing checkpoint fixture matches JSON Schema, SQLite canonical pin order and independent BLAKE3")

order=json.loads((folder/"pin-order.json").read_text())
jsonschema.validate(order,json.loads((folder/"pin-order-schema.json").read_text()))
cases=[(case["order"],case.get("expected"),0) for case in order["cases"]]
cases+=[(list(reversed(range(order["manyCount"]))),list(range(order["manyCount"])),0),([2,0,1],[0,1,2],order["wideRepeat"])]
connection.execute("create table ordering(rank integer,uri blob)")
for ranks,expected,wide in cases:
    connection.execute("delete from ordering")
    for rank in ranks:
        uri=("energy-"+"λ"*wide+f"{rank:04}"+"!s.energy.model@1/*").encode()
        connection.execute("insert into ordering values (?,?)",(rank,uri))
    duplicate=connection.execute("select count(*)-count(distinct uri) from ordering").fetchone()[0]>0
    assert duplicate==(expected is None)
    if expected is not None:
        assert [row[0] for row in connection.execute("select rank from ordering order by uri")]==expected
print("[DEBUG] Original in-place pin ordering corpus matches JSON Schema and SQLite BLOB ordering/uniqueness")
