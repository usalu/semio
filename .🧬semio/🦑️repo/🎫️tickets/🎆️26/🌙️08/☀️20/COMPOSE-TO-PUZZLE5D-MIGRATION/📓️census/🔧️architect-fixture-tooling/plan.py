import json, re, sys
exec(open('values.py').read())
leaves=json.load(open('leaves.json'))
shape=json.load(open('shape.json'))
snap_fields=dict(shape['snapshot']); diff_fields=shape['diff']

SCALAR={'✏️🏷️rename-meta','🔁🏷️replace-meta','✏️📁rename-project','🔁📁replace-project','✏️🏛️rename-governance','🔁🏛️replace-governance'}
CHILD={'🌱📚create-knowledge-record','🗑️📚delete-knowledge-record','✏️📚rename-knowledge-record','🔁📚replace-knowledge-record',
       '🌱🏁create-benchmark-record','🗑️🏁delete-benchmark-record','✏️🏁rename-benchmark-record','🔁🏁replace-benchmark-record'}
EDGE={'🔗🧲connect-adjacency','✂️🧲disconnect-adjacency','🔗🧵connect-trace','✂️🧵disconnect-trace'}

def elem_type(coll):
    t=snap_fields[coll]
    m=re.match(r'Vec<(\w+)>$', t); assert m, (coll,t)
    return m.group(1)

plan={}
problems=[]
for d,e in leaves.items():
    cat=None
    if d in SCALAR: cat='scalar'
    elif d in CHILD: cat='child'
    elif d in EDGE: cat='edge'
    else: cat='regular'
    rec={'cat':cat,'verb':e['verb'],'entity':e['entity'],'kind':e['kind'],'struct':e['struct'],
         'fields':e['fields'],'coll':e.get('coll'),'diff_field':e.get('diff_field'),
         'delta':e.get('delta'),'patch_entry':e.get('patch_entry'),'patch_type':e.get('patch_type')}
    if cat=='regular':
        if not rec['diff_field']: problems.append((d,'no diff_field'))
        coll=rec['coll']
        if coll not in snap_fields: problems.append((d,'coll not a snapshot field: %r'%coll))
        else: rec['etype']=elem_type(coll)
        if e['verb'] in ('rename','replace'):
            if not rec['patch_entry']: problems.append((d,'no patch entry'))
        if e['verb'] in ('create','replace'):
            # payload field carrying the row
            pf=[f for f in e['fields'] if f[1] not in ('EntityId',)]
            if len(pf)!=1: problems.append((d,'ambiguous payload %r'%e['fields']))
            else:
                rec['payload_field']=pf[0][0]; rec['payload_type']=pf[0][1]
                if rec.get('etype') and pf[0][1]!=rec['etype']: problems.append((d,'type mismatch %s vs %s'%(pf[0][1],rec['etype'])))
    plan[d]=rec
print('problems:', len(problems))
for p in problems[:20]: print('  ', p)
json.dump(plan, open('plan.json','w'), ensure_ascii=False, indent=1)
