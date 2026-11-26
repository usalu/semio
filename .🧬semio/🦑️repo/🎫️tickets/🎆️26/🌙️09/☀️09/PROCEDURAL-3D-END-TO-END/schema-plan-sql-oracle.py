import json
import sqlite3
from pathlib import Path

root = Path(__file__).resolve().parents[7]
fixture = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry/🧪️tests/🧫️fixtures/schema-plan.json"
db = sqlite3.connect(":memory:")
query = """
with source as (select json(?) as body),
values3 as (
 select body, json_extract(body,'$.vector') as vector,
 coalesce(json_extract(body,'$.x.value'),json_extract(body,'$.vector.x'),0) as x,
 coalesce(json_extract(body,'$.y.value'),json_extract(body,'$.vector.y'),0) as y,
 coalesce(json_extract(body,'$.z.value'),json_extract(body,'$.vector.z'),0) as z,
 json_type(body,'$.vector') is not null or json_type(body,'$.x.value') is not null or json_type(body,'$.y.value') is not null or json_type(body,'$.z.value') is not null as present from source
)
select case when present then json_object('vectorOut',json_object('$schema','vector','x',x,'y',y,'z',z),'xOut',json_object('$schema','number','value',x),'yOut',json_object('$schema','number','value',y),'zOut',json_object('$schema','number','value',z),'errors',json_object('$schema','list'))
else json_object('vectorOut',null,'xOut',null,'yOut',null,'zOut',null,'errors',json_object('$schema','list','0',json_object('$schema','text','value','no instance or field inputs provided'))) end from values3
"""
for case in json.loads(fixture.read_text()):
    actual = json.loads(db.execute(query, (json.dumps(case["input"]),)).fetchone()[0])
    assert actual == case["expected"], (case["name"], actual, case["expected"])
print("4 schema projection cases passed SQLite JSON oracle")
errors = fixture.with_name("schema-errors.json")
error_query = """
select case
 when json_type(?,'$.vector') is not null and json_type(?,'$.vector') != 'object' then 'missing vector'
 when json_type(?,'$.vector') = 'object' and json_extract(?,'$.vector."$schema"') != 'vector' then 'invalid vector'
 when json_type(?,'$.x') != 'object' then 'decimal channel is not a dictionary'
 else 'decimal channel is missing value' end
"""
for case in json.loads(errors.read_text()):
    body = json.dumps(case["input"])
    assert db.execute(error_query, (body,) * 5).fetchone()[0] == case["message"], case["name"]
composite = json.loads(fixture.with_name("schema-composite.json").read_text())
body = json.dumps(composite["input"])
row = db.execute("select json_extract(?,'$.label.value'),json_extract(?,'$.profile'),json_extract(?,'$.tags')", (body,) * 3).fetchone()
assert row[0] == composite["expected"]["memberOut"]["label"]
assert json.loads(row[1]) == composite["expected"]["profileOut"]
assert json.loads(row[2]) == composite["expected"]["tagsOut"]
print("4 error cases and composite projection passed SQLite JSON oracle")
