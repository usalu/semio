from pathlib import Path
import json

root = Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc')
path = root / '🧩️composition/🔣️.json'
data = json.loads(path.read_text(encoding='utf-8-sig'))
if not any(app['variant'] == 'Ifc4x3AnyEditor' for app in data['apps']):
    data['apps'].append({'owner':'stdio-bim','order':10,'variant':'Ifc4x3AnyEditor','role':'editor','type':'semio_s_artifact_stdio_ifc_4x3::editor::Ifc4x3AnyEditor','factory':'semio_s_artifact_stdio_ifc_4x3::editor::create_ifc4x3_any_editor','mutationRoster':False,'laws':None})
if 'semio-s-artifact-stdio-ifc-4x3' not in data['symbolPackages']:
    data['symbolPackages'].append('semio-s-artifact-stdio-ifc-4x3')
path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

path = root / '📜️artifact-definition.json'
data = json.loads(path.read_text(encoding='utf-8-sig'))
standard = 's.stdio.ifc.standard.4x3'
source = json.loads((root / '🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🔣️.json').read_text(encoding='utf-8'))['source']
declaration = {'id':standard,'revision':'4x3','normative_source':source['url'],'publication_date':None,'source_checksum':source['sha256'],'redistribution_status':'unknown','clauses_or_features':['IFC4X3_ADD2 declarations','structural EXPRESS domains; WHERE, UNIQUE, inverse and MVD constraints not evaluated'],'status':'declared'}
data['standards'] = [value for value in data['standards'] if value['id'] != standard] + [declaration]
if not any(value['id'] == standard + '.profile.any' for value in data['profiles']):
    data['profiles'].append({'id':standard + '.profile.any','standard':standard,'profile':'any','status':'declared'})
if not any(value['id'] == standard + '.representation.document' for value in data['representations']):
    data['representations'].append({'id':standard + '.representation.document','standard':standard,'representation':'document','mimes':['application/x-ifc'],'extensions':['.ifc'],'is_binary':False,'aliases':[],'neutral':True,'status':'declared'})
path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
