"""🪴 Carry the reviewed species-specific parts into their existing authoring sources."""
import json
from pathlib import Path

ticket=Path(__file__).parent
catalog=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
for name in ('bamboo-clump','goat','pigeon','pig','sheep'):
    live=json.loads(next(catalog.glob(f'*/{name}/*.json')).read_text())['asset']
    source=ticket/'volumes'/('vegetation' if name=='bamboo-clump' else 'animals')/(name+'.json')
    raw=source.read_text()
    source_json=json.loads(raw)
    changed=False
    for scale,current in live['volumes'].items():
        model=source_json['volumes'][scale]
        if name=='bamboo-clump':
            if scale=='50':
                body={s['id']:s for s in current['body']}
                for i,s in enumerate(model['body']):
                    if (s['material']=='bamboo-leaf' and s['role']=='texture' and len(s.get('sections',[]))==8) or (s['kind']=='capsule' and s['r0']==.0009 and s['r1']==.0007):
                        replacement=body[s['id']]
                        changed|=s!=replacement
                        model['body'][i]=replacement
            continue
        def owns(s):
            return s['id'].startswith('th-breath') or ((scale=='500' or (name=='goat' and scale=='200')) and s.get('region')=='hooves') or (name=='pigeon' and scale in ('200','500') and s.get('region') in ('eye','beak','cere'))
        entries=model['thermal']
        replacements=[s for s in current['thermal'] if owns(s)]
        if [s for s in entries if owns(s)]!=replacements:
            first=next(i for i,s in enumerate(entries) if owns(s))
            rest=[s for s in entries if not owns(s)]
            model['thermal']=rest[:first]+replacements+rest[first:]
            changed=True
        if name=='goat' and scale=='500':
            structure=model['structure']
            for i,s in enumerate(structure):
                if s['id']=='rib-girth':
                    structure[i:i+1]=[q for q in current['structure'] if q['id'] in ('rib-7-n','rib-7-f','rib-12-n','rib-12-f')]
                    changed=True
                    break
    if changed:
        if source.read_text()!=raw:raise SystemExit(f'{name} source changed concurrently')
        text=json.dumps(source_json,ensure_ascii=False,indent=2) if raw.startswith('{\n') else json.dumps(source_json,ensure_ascii=False,separators=(',',':'))
        source.write_text(text+('\n' if raw.endswith('\n') else ''))
        print('[DEBUG]',name)

