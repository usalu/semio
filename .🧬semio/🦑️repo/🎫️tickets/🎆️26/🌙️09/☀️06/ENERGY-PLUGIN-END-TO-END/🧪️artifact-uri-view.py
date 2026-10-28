"""🧵️ SQLite independently constructs and slices the canonical original URI fixture."""
import json
import sqlite3
from pathlib import Path
import jsonschema
root=Path(__file__).resolve().parents[7]
folder=root/"🧰️framework/🔨️modules/🚪️io/📝️text/🗿️artifact-reference/👁️view"
law=json.loads((folder/"🔣️.json").read_text())
jsonschema.validate(law,json.loads((folder/"schema.json").read_text()))
connection=sqlite3.connect(":memory:")
rows=law["cases"]+[{"id":"λ"*law["wideRepeat"],"kind":"s.energy.model","standard":"1","subset":"*","uri":"λ"*law["wideRepeat"]+"!s.energy.model@1/*"}]
for row in rows:
    actual=connection.execute("select cast(? || '!' || ? || '@' || ? || '/' || ? as blob)",(row["id"],row["kind"],row["standard"],row["subset"])).fetchone()[0]
    assert actual==row["uri"].encode()
    for width in law["widths"]:
        fragments=[connection.execute("select substr(?,?,?)",(actual,index+1,width)).fetchone()[0] for index in range(0,len(actual),width)]
        assert b"".join(fragments)==actual
print("[DEBUG] Borrowed artifact URI corpus matches JSON Schema and SQLite byte fragments")
