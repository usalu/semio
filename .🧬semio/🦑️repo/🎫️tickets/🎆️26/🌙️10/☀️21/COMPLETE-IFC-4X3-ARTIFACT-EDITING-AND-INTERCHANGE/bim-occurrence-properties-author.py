"""🏷️ Author closed spatial occurrence property cases before implementing their actual exchange endpoints."""
import json
from pathlib import Path
ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
folder=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/bim/properties'
folder.mkdir(exist_ok=True)
values=[dict(id='text-empty',kind='Text',value=''),dict(id='text-unicode',kind='Text',value='Büro · 東京\nline'),dict(id='real-negative-zero',kind='Real',bits='8000000000000000'),dict(id='real-positive',kind='Real',bits='4004000000000000'),dict(id='integer-negative',kind='Integer',value='-17'),dict(id='integer-max',kind='Integer',value='2147483647'),dict(id='integer-min',kind='Integer',value='-2147483648'),dict(id='boolean-false',kind='Boolean',value=False),dict(id='boolean-true',kind='Boolean',value=True)]
data=dict(schema='semio.ifc4x3.bim-occurrence-properties.v1',scope='dimensionless-own-occurrence-properties-on-supported-spatial-holders',integerDomain=dict(minimum='-2147483648',maximum='2147483647',source='Canonical BIM PropertyValue JSON schema and Rust i32; not an IFC INTEGER restriction'),holders=['site','building','storey'],values=values,exportRefusals=['length','area','volume','angle','unsafe-integer','out-of-domain-integer','nonfinite-real','wrong-boolean','missing-holder','ambiguous-holder','reserved-storey','reserved-project','empty-set','empty-property','empty-set-name'],importRefusals=['physical-unit','logical-unknown','unset-nominal','unsafe-integer','out-of-domain-integer','measure','extra-items','untyped','duplicate-property','duplicate-set','missing-set-name','empty-property-name','non-single','reserved-wrong-holder','ambiguous-property-owner','late-unknown-property','ambiguous-set-owner'],importAliases=['IFCLABEL','IFCIDENTIFIER'])
(folder/'cases.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
(folder/'schema.json').write_text(json.dumps({'$schema':'https://json-schema.org/draft/2020-12/schema','const':data},indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(folder=str(folder),positiveCases=len(values)*len(data['holders']),exportRefusals=len(data['exportRefusals']),importRefusals=len(data['importRefusals']))))
