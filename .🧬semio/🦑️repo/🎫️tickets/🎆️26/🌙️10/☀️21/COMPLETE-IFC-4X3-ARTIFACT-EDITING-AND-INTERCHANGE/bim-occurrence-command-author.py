"""🧾 Author shared canonical BIM property command inputs without introducing a second mutation implementation."""
import json
from pathlib import Path
ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
folder=repo/'✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests'
values=[{'Text':{'value':'Büro · 東京'}},{'Real':{'value':-0.0}},{'Integer':{'value':2147483647}},{'Boolean':{'value':False}}]
rows=[dict(mutation='setElementProperty',id=holder,pset='Authored',property=next(iter(value)),value=value) for holder in ['site','building','storey'] for value in values]
(folder/'occurrence.json').write_text(json.dumps(rows,indent=2)+'\n',encoding='utf-8')
(folder/'occurrence.schema.json').write_text(json.dumps({'$schema':'https://json-schema.org/draft/2020-12/schema','const':rows},indent=2)+'\n',encoding='utf-8')
print('[DEBUG] canonical commands '+str(len(rows)))
