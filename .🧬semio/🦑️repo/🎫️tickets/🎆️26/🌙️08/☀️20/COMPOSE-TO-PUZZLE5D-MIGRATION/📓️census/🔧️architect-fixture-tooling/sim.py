# 🧮 Independent apply-simulation: replays ProgramDiff::apply_to_artifact + apply_collection_delta
# + impl_patchable's apply_row on the committed JSON and asserts before + diff == after.
import json, os, copy, re
T=json.load(open('types.json')); structs=T['structs']; patchable=T['patchable']
shape=json.load(open('shape.json'))
plan=json.load(open('plan.json'))
BASE="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
MROOT=f"{BASE}/🧬️mutations"
def camel(x):
    p=x.split('_'); return p[0]+''.join(y[:1].upper()+y[1:] for y in p[1:])
# diff field -> (snapshot json key, element type, entity type name)
DIFF_TO_SNAP={camel(n):camel(n) for n,_ in shape['snapshot']}
DIFF_TO_SNAP['documents']='artifacts'
SNAP_TYPE={camel(n): (re.match(r'Vec<(\w+)>$',t).group(1) if t.startswith('Vec<') else None) for n,t in shape['snapshot']}
ID_KEY={}
for name,s in structs.items():
    fn=[f['name'] for f in s['fields']]
    if 'header' in fn: ID_KEY[name]='id'   # flattened header.id
    elif 'id' in fn: ID_KEY[name]='id'

def entity_json_key(etype, rust_path):
    """json key for an impl_patchable rust path such as header.name or role"""
    return camel(rust_path.split('.')[-1])

SKIP={}
def skip_keys(name):
    if name in SKIP: return SKIP[name]
    out={}
    for f in structs[name]['fields']:
        if 'flatten' in f['attrs']:
            out.update(skip_keys(f['type'])); continue
        if 'skip_serializing_if = "Option::is_none"' in f['attrs']: out[camel(f['name'])]='none'
        elif 'skip_serializing_if = "Vec::is_empty"' in f['attrs']: out[camel(f['name'])]='empty'
    SKIP[name]=out; return out

def normalize_row(etype, row):
    sk=skip_keys(etype)
    return {k:v for k,v in row.items() if not ((sk.get(k)=='none' and v is None) or (sk.get(k)=='empty' and v==[]))}

def apply_patch(rowjson, etype, patch):
    pt, pairs = patchable[etype]
    out=copy.deepcopy(rowjson)
    for pfield, epath in pairs:
        v=patch.get(camel(pfield))
        if v is None: continue
        out[entity_json_key(etype, epath)]=v
    return normalize_row(etype, out)

TRACE_PAIRS=[('from_id','from_id'),('to_id','to_id'),('kind','kind'),('label','label')]
def apply_patch_trace(rowjson, patch):
    out=copy.deepcopy(rowjson)
    for f,_ in TRACE_PAIRS:
        v=patch.get(camel(f))
        if v is None: continue
        out[camel(f)]=v
    return normalize_row('TraceLink', out)

fails=[]
checked=0
for d,p in sorted(plan.items()):
    tests=os.path.join(MROOT,d,"🧪️tests")
    for c in sorted(os.listdir(tests)):
        cd=os.path.join(tests,c)
        df=os.path.join(cd,"🔺️diff/🔣️component.json")
        b=json.load(open(os.path.join(cd,"📸️snapshot/⬅️before/🔣️component.json")))
        a=json.load(open(os.path.join(cd,"📸️snapshot/➡️after/🔣️component.json")))
        if not os.path.isfile(df):
            if b!=a: fails.append(f"{d}/{c}: rejected case but before != after")
            checked+=1; continue
        dj=json.load(open(df))
        nxt=copy.deepcopy(b)
        for k,v in dj.items():
            if v is None: continue
            sk=DIFF_TO_SNAP.get(k)
            if sk is None: fails.append(f"{d}/{c}: diff field {k} has no snapshot target"); continue
            if not isinstance(v,dict) or set(v.keys())!={'added','removed','patched','reordered'}:
                nxt[sk]=copy.deepcopy(v); continue     # scalar facet / child handle
            etype=SNAP_TYPE[sk]
            rows=copy.deepcopy(nxt[sk])
            idk=ID_KEY[etype]
            for rid in v['removed']:
                if not any(r[idk]==rid for r in rows): fails.append(f"{d}/{c}: removed {rid} absent")
                rows=[r for r in rows if r[idk]!=rid]
            for entry in v['patched']:
                tgt=[r for r in rows if r[idk]==entry['id']]
                if not tgt: fails.append(f"{d}/{c}: patched {entry['id']} absent"); continue
                i=rows.index(tgt[0])
                rows[i]= apply_patch_trace(rows[i], entry['patch']) if etype=='TraceLink' else apply_patch(rows[i], etype, entry['patch'])
            for add in v['added']:
                if any(r[idk]==add[idk] for r in rows): fails.append(f"{d}/{c}: added duplicate {add[idk]}")
                rows.append(copy.deepcopy(add))
            nxt[sk]=rows
        if nxt!=a:
            diffkeys=[k for k in a if a[k]!=nxt.get(k)]
            fails.append(f"{d}/{c}: simulated apply != committed after (keys {diffkeys})")
        checked+=1
print(f"simulated {checked} cases; failures={len(fails)}")
for f in fails[:20]: print("  ❌️", f)
