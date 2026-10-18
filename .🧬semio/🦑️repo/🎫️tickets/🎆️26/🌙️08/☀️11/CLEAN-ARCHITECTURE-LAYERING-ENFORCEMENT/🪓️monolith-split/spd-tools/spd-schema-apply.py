"""🛠️ Applies the proposals of spd-schema-from-ts.ts to the schema documents, deciding each field from its Rust struct field: `python3 spd-schema-apply.py <proposals.json> [--write]`. Without --write only the plan is printed. Snapshots every changed document to 🗑️generated/spd-snap/<n>.json and lists them in spd-snap/manifest.json."""
import json,sys,re,os,subprocess,shutil
ROOT='/Users/ueli/Documents/semio/'
F=ROOT+'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/'
SNAP=F+'🗑️generated/spd-snap/'
write='--write' in sys.argv
props=json.load(open(sys.argv[1]))
def camel(s):
    parts=s.split('_'); return parts[0]+''.join(p[:1].upper()+p[1:] for p in parts[1:])
LITERAL={'bool':False,'String':'','f64':0,'f32':0}
for t in ('u8','u16','u32','u64','usize','i8','i16','i32','i64','isize'): LITERAL[t]=0
def literal_default(ty):
    ty=ty.strip()
    if ty in LITERAL: return True,LITERAL[ty]
    if ty.startswith('Vec<'): return True,[]
    if re.match(r'(BTreeMap|HashMap|IndexMap)<',ty): return True,{}
    return False,None
def rust_struct(leaf_dir,name):
    art=leaf_dir.split('/🪆️subsets/')[0] if '/🪆️subsets/' in leaf_dir else leaf_dir
    out=subprocess.run(['git','-c','core.quotepath=false','grep','-n','-E','pub struct '+name+r'( |<|\{|$)','--',art],cwd=ROOT,capture_output=True,text=True).stdout.splitlines()
    out=sorted(out,key=lambda l: (not l.split(':')[0].startswith(leaf_dir),len(l)))
    if not out: return None
    f,ln=out[0].split(':')[0],int(out[0].split(':')[1])
    src=open(ROOT+f,encoding='utf8').read().split('\n')
    at=ln-1
    attrs=[]; j=at-1
    while j>=0 and (src[j].strip().startswith('#[') or src[j].strip().startswith('///') or src[j].strip()==''):
        attrs.append(src[j]); j-=1
    rename=None
    for a in attrs:
        m=re.search(r'rename_all\s*=\s*"(\w+)"',a)
        if m: rename=m.group(1)
    fields={}
    cur=[]
    k=at+1
    while k<len(src) and not src[k].startswith('}'):
        s=src[k].strip()
        if s.startswith('#['): cur.append(s)
        elif s.startswith('///') or s=='': pass
        else:
            m=re.match(r'pub (\w+):\s*(.+?),?$',s)
            if m:
                nm,ty=m.group(1),m.group(2)
                rn=None
                for a in cur:
                    mm=re.search(r'rename\s*=\s*"([^"]+)"',a)
                    if mm and 'rename_all' not in a: rn=mm.group(1)
                wire=rn or (camel(nm) if rename=='camelCase' else nm)
                plain=any(re.search(r'#\[value\(default\)\]|#\[serde\(default\)\]',a) for a in cur)
                custom=any(re.search(r'default\s*=',a) for a in cur)
                fields[wire]=(ty,plain,custom)
            cur=[]
        k+=1
    return f,fields
def is_nullable(shape):
    t=shape.get('type')
    if isinstance(t,list) and 'null' in t: return True
    for k in ('anyOf','oneOf'):
        a=shape.get(k)
        if isinstance(a,list) and any(isinstance(x,dict) and x.get('type')=='null' for x in a): return True
    return False
ANN=('description','title','x-semio-state','x-semio-ui','x-semio-formats')
def wrap_nullable(shape,default_null):
    ann={k:v for k,v in shape.items() if k in ANN or k.startswith('x-')}
    core={k:v for k,v in shape.items() if k not in ann and k!='default'}
    out={'anyOf':[core,{'type':'null'}]}
    out.update(ann)
    if default_null: out['default']=None
    return out
def load(rel):
    raw=open(ROOT+rel,encoding='utf8').read()
    d=json.loads(raw)
    for indent in (2,4,1):
        if json.dumps(d,indent=indent,ensure_ascii=False)+"\n"==raw or json.dumps(d,indent=indent,ensure_ascii=False)==raw.rstrip("\n"): return raw,d,indent
    return raw,d,None
changed={}
report=[]
for p in props:
    if 'skip' in p: report.append((p['name'],'SKIP '+p['skip'])); continue
    if not p['addRequired'] and not p['makeNullable']: continue
    doc=p['doc'].replace('🟦️.ts','🔣️.json')
    raw,d,indent=changed.get(doc) or load(doc)
    if indent is None: report.append((p['name'],'FORMAT does not round-trip '+doc)); continue
    name=p['name']
    cont=None
    for key in ('definitions','$defs'):
        if isinstance(d.get(key),dict) and name in d[key]: cont=d[key]; break
    node=cont[name] if cont is not None else (d if d.get('title')==name else None)
    if node is None: report.append((name,'NODE missing')); continue
    rs=rust_struct(os.path.dirname(p['doc']),name)
    if rs is None: report.append((name,'NO RUST STRUCT')); continue
    f,fields=rs
    plan=[]
    for field in p['addRequired']+[x for x in p['makeNullable'] if x not in p['addRequired']]:
        ty,plain,custom=fields.get(field,(None,False,False))
        shape=node['properties'][field]
        if ty is None: plan.append((field,'owner','no Rust field')); continue
        opt=ty.startswith('Option<')
        if field in p['makeNullable'] or opt:
            plan.append((field,'nullable-default' if (opt or plain) else 'nullable',ty)); continue
        if custom: plan.append((field,'owner','custom default fn: '+ty)); continue
        if plain:
            ok,lit=literal_default(ty)
            plan.append((field,'default' if ok else 'owner',ty)) if ok else plan.append((field,'owner','struct default: '+ty))
            continue
        plan.append((field,'required',ty))
    report.append((name,f.split('/')[-4:-1],plan))
    for field,action,info in plan:
        shape=node['properties'][field]
        if action=='required':
            req=node.setdefault('required',[])
            if field not in req: req.append(field)
        elif action=='default':
            ok,lit=literal_default(info)
            if 'default' not in shape: shape['default']=lit
        elif action in('nullable-default','nullable'):
            if not is_nullable(shape): node['properties'][field]=wrap_nullable(shape,action=='nullable-default')
            elif action=='nullable-default' and 'default' not in node['properties'][field]: node['properties'][field]['default']=None
    changed[doc]=(raw,d,indent)
for r in report: print(r)
if write:
    manifest=json.load(open(SNAP+'manifest.json')) if os.path.exists(SNAP+'manifest.json') else {}
    for doc,(raw,d,indent) in changed.items():
        n=len(manifest)
        shutil.copyfile(ROOT+doc,SNAP+f'doc{n}.json')
        manifest[f'doc{n}.json']=doc
        out=json.dumps(d,indent=indent,ensure_ascii=False)+("\n" if raw.endswith("\n") else "")
        open(ROOT+doc,'w',encoding='utf8').write(out)
    json.dump(manifest,open(SNAP+'manifest.json','w'),ensure_ascii=False,indent=1)
    print('written',len(changed))
