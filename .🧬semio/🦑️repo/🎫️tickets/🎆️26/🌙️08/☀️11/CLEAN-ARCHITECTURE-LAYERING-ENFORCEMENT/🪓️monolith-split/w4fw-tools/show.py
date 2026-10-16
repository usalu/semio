import json, sys
E=json.load(open('ents.json')); ents=E['ents']; edges=E['edges']
files=sys.argv[1:]
for i in E['order']:
    e=ents[i]
    if e['file'] in files and e['kind']!='mod':
        d=[f"{ents[x]['file'].replace('m_','')}.{ents[x]['name']}" if ents[x]['file']!=e['file'] else ents[x]['name'] for x in edges.get(i,[]) if ents[x]['kind']!='mod']
        print(f"{e['name']}({e['lines']}) -> {', '.join(d)}")
