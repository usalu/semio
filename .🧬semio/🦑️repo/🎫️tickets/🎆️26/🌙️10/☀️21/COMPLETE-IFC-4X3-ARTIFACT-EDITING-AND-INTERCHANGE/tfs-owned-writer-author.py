"""🔺 Author closed direct-writer policy over unchanged triangulated source cases."""
import hashlib
import json
from pathlib import Path

ticket = Path(__file__).resolve().parent
repo = next(p for p in ticket.parents if (p / 'AGENTS.md').exists())
tests = repo / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests'
path = tests / 'tfs-admission-cases.json'
source = json.loads(path.read_text(encoding='utf-8'))
cases = dict(version=1, source='tfs-admission-cases.json', sourceSha256=hashlib.sha256(path.read_bytes()).hexdigest(),
             requireClosed=True, normalTransport=False, nativeExecuted=False,
             cases=[dict(id=row['id'], expected='accept' if row['expected'] == 'accept' and row['source']['closed'] is True else 'refuse') for row in source['cases']])
schema = {'$schema':'https://json-schema.org/draft/2020-12/schema', 'type':'object', 'additionalProperties':False,
          'required':list(cases), 'properties':{key:{'const':value} for key,value in cases.items() if key != 'cases'}}
schema['properties']['cases'] = {'type':'array', 'minItems':32, 'maxItems':32, 'items':{'type':'object', 'additionalProperties':False,
    'required':['id','expected'], 'properties':{'id':{'enum':[row['id'] for row in cases['cases']]}, 'expected':{'enum':['accept','refuse']}}}}
for name,value in [('tfs-writer-cases.json',cases),('tfs-writer.schema.json',schema)]:
    (tests / name).write_text(json.dumps(value,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(cases=len(cases['cases']),accepted=sum(row['expected']=='accept' for row in cases['cases']))))
