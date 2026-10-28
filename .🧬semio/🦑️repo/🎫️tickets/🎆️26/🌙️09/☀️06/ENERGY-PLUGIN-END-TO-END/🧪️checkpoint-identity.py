import json
from pathlib import Path
import blake3
import jsonschema
root=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original')
f=json.loads((root/'fixture.json').read_text())
jsonschema.validate(f,json.loads((root/'schema.json').read_text()))
pending=[f['pattern']*f['repeat'],'edit-2']
changes=[{'id':'change-old','editIds':['edit-old'],'savedAt':f['savedAt']},{'id':'change-described','editIds':['edit-described'],'description':'quoted "\nλ','savedAt':f['savedAt']}]
changes.append({'id':f['pendingId'],'editIds':pending,'savedAt':f['savedAt']})
wire=f['parent'].encode()+b'\0'
for change in changes:wire+=blake3.blake3(json.dumps(change,ensure_ascii=False,separators=(',',':')).encode()).digest()
wire+=bytes(3)+f['timestamp'].encode()
assert blake3.blake3(wire).hexdigest()==f['digest']
print('PASS jsonschema + independent blake3/JSON checkpoint identity')
import sqlite3
p=json.loads((root/'pending.json').read_text())
jsonschema.validate(p,json.loads((root/'pending.schema.json').read_text()))
c=sqlite3.connect(':memory:')
c.execute('create table applied(position integer,id text)')
c.execute('create table committed(id text)')
c.executemany('insert into applied values(?,?)',enumerate(p['applied']))
c.executemany('insert into committed values(?)',[(id,)for row in p['committed']for id in row])
assert [row[0]for row in c.execute('select id from applied where not exists(select 1 from committed where committed.id=applied.id) order by position')]==p['pending']
print('PASS jsonschema + independent SQLite pending history selection')
assert f['changeIdentity']=='change-'+blake3.blake3(b'change\0'+pending[0].encode()+b'\0edit-2\0').hexdigest()[:16]
assert f['alternativeIdentity']=='alternative-'+blake3.blake3(b'alternative\0'+'Alternative λ\n'.encode()+b'\0'+('ck-'+f['digest'][:16]).encode()).hexdigest()[:16]
print('PASS independent blake3 history entity identities')
facts=json.loads((root/'facts.json').read_text())
jsonschema.validate(facts,json.loads((root/'facts.schema.json').read_text()))
c.execute('create table checkpoint_change(position integer,id text)')
c.executemany('insert into checkpoint_change values(?,?)',enumerate(facts['parent']['changeIds']))
c.execute('insert into checkpoint_change values(?,?)',(len(facts['parent']['changeIds']),facts['changeId']))
assert [row[0]for row in c.execute('select id from checkpoint_change order by position')]==facts['parent']['changeIds']+[facts['changeId']]
print('PASS independent JSON Schema/SQLite branch canonical facts')
