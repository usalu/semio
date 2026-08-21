# 🔬 Independent serde-shape validator: decodes+re-encodes every committed JSON against the
# parsed Rust type table and reports any field that serde would drop, reject, or add.
import json, re, os, copy
exec(open('rustparse.py').read().replace("print(","#print(").replace("json.dump(","#json.dump("))
BASE="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
s2,e2,p2 = parse([f"{BASE}/🔺️diff/🦀️component.rs", f"{BASE}/📸️snapshot/🦀️component.rs"])
structs.update(s2); enums.update(e2)
# synthesized framework types
structs['ArtifactDialect']={'rename_all':'camelCase','default':False,'fields':[
 {'name':'artifact_kind','type':'String','attrs':''},{'name':'standard','type':'String','attrs':''},{'name':'subset','type':'String','attrs':''}]}
structs['ArtifactRef']={'rename_all':'camelCase','default':False,'fields':[
 {'name':'artifact_id','type':'String','attrs':''},{'name':'target_dialect_marker','type':'ArtifactDialect','attrs':''}]}
structs['ArtifactRef']['fields'][1]['name']='dialect'
structs['ArtifactChild']={'rename_all':'camelCase','default':False,'fields':[
 {'name':'child_id','type':'String','attrs':''},{'name':'target','type':'ArtifactRef','attrs':''}]}
ALIAS={'crate::artifacts::program::ProgramKnowledgeChild':'ArtifactChild',
       'crate::artifacts::program::ProgramBenchmarksChild':'ArtifactChild',
       'Option<Box<crate::artifacts::program::schema::ProgramArtifact>>':'Option<ProgramArtifact>'}
def resolve(t):
    t=t.strip()
    return ALIAS.get(t,t)


def camel(x):
    p=x.split('_'); return p[0]+''.join(y[:1].upper()+y[1:] for y in p[1:])
def rename(n, rule):
    return camel(n) if rule=='camelCase' else n
def variant_name(en, v):
    return v[:1].lower()+v[1:] if enums[en]['rename_all']=='camelCase' else v

PRIM_INT={'u8','u16','u32','u64','usize','i8','i16','i32','i64','isize'}
PRIM_FLOAT={'f32','f64'}
errs=[]
def check(v, ty, where):
    ty=resolve(ty)
    if ty.startswith('Option<'):
        if v is None: return None
        return check(v, ty[7:-1], where)
    if ty.startswith('Vec<'):
        if not isinstance(v,list): errs.append(f"{where}: expected list for {ty}, got {type(v).__name__}"); return v
        return [check(x, ty[4:-1], f"{where}[{i}]") for i,x in enumerate(v)]
    if ty=='String' or ty=='EntityId':
        if not isinstance(v,str): errs.append(f"{where}: expected string for {ty}, got {v!r}")
        return v
    if ty=='bool':
        if not isinstance(v,bool): errs.append(f"{where}: expected bool, got {v!r}")
        return v
    if ty in PRIM_INT:
        if not isinstance(v,int) or isinstance(v,bool): errs.append(f"{where}: expected int, got {v!r}")
        return v
    if ty in PRIM_FLOAT:
        if not isinstance(v,(int,float)) or isinstance(v,bool): errs.append(f"{where}: expected float, got {v!r}")
        return float(v)
    if ty in enums:
        names={variant_name(ty,x['name']) for x in enums[ty]['variants']}
        if v not in names: errs.append(f"{where}: {v!r} is not a variant of {ty} ({sorted(names)[:4]}...)")
        return v
    if ty in structs:
        if not isinstance(v,dict): errs.append(f"{where}: expected object for {ty}, got {v!r}"); return v
        return check_struct(v, ty, where)
    errs.append(f"{where}: unknown type {ty}")
    return v

def emitted_keys(name):
    """(key, type, attrs) pairs serde will emit, flattening"""
    s=structs[name]; out=[]
    for f in s['fields']:
        if 'flatten' in f['attrs']:
            out.extend(emitted_keys(resolve(f['type']))); continue
        out.append((rename(f['name'], s['rename_all']), f['type'], f['attrs']))
    return out

def check_struct(v, name, where):
    keys=emitted_keys(name); out={}
    seen=set()
    for k,ty,attrs in keys:
        seen.add(k)
        rty=resolve(ty)
        if k not in v:
            if rty.startswith('Option<') or 'default' in attrs or structs.get(name,{}).get('default'):
                continue
            errs.append(f"{where}: missing field {k!r} ({ty}) — serde would reject or default it")
            continue
        val=check(v[k], ty, f"{where}.{k}")
        # would serde re-emit it?
        if 'skip_serializing_if = "Option::is_none"' in attrs and val is None: 
            errs.append(f"{where}.{k}: committed null but serde skips this field when None"); continue
        if 'skip_serializing_if = "Vec::is_empty"' in attrs and val==[]:
            errs.append(f"{where}.{k}: committed [] but serde skips this field when empty"); continue
        out[k]=val
    for k in v:
        if k not in seen: errs.append(f"{where}: extra key {k!r} not in {name} — serde would drop it")
    return out

MROOT=f"{BASE}/🧬️mutations"
plan=json.load(open('plan.json'))
mutstructs={}
for d,p in plan.items():
    src=open(os.path.join(MROOT,d,"🦠️mutation/🦀️component.rs")).read()
    m=re.search(r'pub struct (\w+) \{(.*?)\n\}', src, re.S)
    fields=[]
    for line in m.group(2).split('\n'):
        fm=re.match(r'\s*pub (\w+): (.+),$', line)
        if fm: fields.append({'name':fm.group(1),'type':fm.group(2),'attrs':''})
    structs['MUT_'+p['struct']]={'rename_all':'camelCase','default':False,'fields':fields}

cases=0
for d,p in sorted(plan.items()):
    tests=os.path.join(MROOT,d,"🧪️tests")
    for c in sorted(os.listdir(tests)):
        cd=os.path.join(tests,c); cases+=1
        for side in ["⬅️before","➡️after"]:
            j=json.load(open(os.path.join(cd,"📸️snapshot",side,"🔣️component.json")))
            r=check_struct(j,'ProgramSnapshot',f"{d}/{c}/{side}")
            if r!=j: errs.append(f"{d}/{c}/{side}: not a serde fixed point")
        mj=json.load(open(os.path.join(cd,"🦠️mutation/🔣️component.json")))
        tag=mj.pop('mutation',None)
        expect=p['struct'][:1].lower()+p['struct'][1:]
        if tag!=expect: errs.append(f"{d}/{c}: mutation tag {tag!r} != {expect!r}")
        r=check_struct(mj,'MUT_'+p['struct'],f"{d}/{c}/mutation")
        if r!=mj: errs.append(f"{d}/{c}/mutation: not a serde fixed point")
        df=os.path.join(cd,"🔺️diff/🔣️component.json")
        if os.path.isfile(df):
            dj=json.load(open(df))
            r=check_struct(dj,'ProgramDiff',f"{d}/{c}/diff")
            if r!=dj: errs.append(f"{d}/{c}/diff: not a serde fixed point")
print(f"validated {cases} cases; problems={len(errs)}")
for e in errs[:30]: print("  ⚠️", e)
