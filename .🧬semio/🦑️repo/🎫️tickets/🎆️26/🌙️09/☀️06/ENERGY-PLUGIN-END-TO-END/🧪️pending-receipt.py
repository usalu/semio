import json,sqlite3
from pathlib import Path
import jsonschema
root=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🏁️publication/📨️report')
f=json.loads((root/'fixture.json').read_text())
jsonschema.validate(f,json.loads((root/'schema.json').read_text()))
for case in f['cases']:
 c=sqlite3.connect(':memory:')
 c.execute('create table outcomes(position integer,edit text,messages integer)')
 rows=case['outcomes']*case['repeat']
 c.executemany('insert into outcomes values(?,?,?)',[(i,r['edit'],len(r['messages']))for i,r in enumerate(rows)])
 groups=c.execute('with filtered as(select *,lag(edit) over(order by position) as previous from outcomes where messages>0) select coalesce(sum(case when previous is null or previous!=edit then 1 else 0 end),0),coalesce(sum(messages),0) from filtered').fetchone()
 assert groups==(case['expectedGroups'],case['expectedMessages'])
 c.close()
print('PASS JSON Schema and independent SQLite ordered replay receipt grouping')
