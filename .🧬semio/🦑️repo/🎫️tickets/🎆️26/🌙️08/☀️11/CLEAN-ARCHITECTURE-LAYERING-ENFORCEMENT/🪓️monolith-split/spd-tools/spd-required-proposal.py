"""🧮️ For every disagreeing parser of an Ajv differential (verify.json) compares the schema `required` with the Rust struct of the same name: `python3 spd-required-proposal.py <verify.json>`."""
import json,sys,re,os,subprocess
ROOT='/Users/ueli/Documents/semio'
res=json.load(open(sys.argv[1]))
def camel(s):
    parts=s.split('_'); return parts[0]+''.join(p[:1].upper()+p[1:] for p in parts[1:])
def rust_struct(art_dir,name):
    out=subprocess.run(['git','-c','core.quotepath=false','grep','-n','-E','pub (struct|enum) '+name+r'( |<|\{|$)','--',art_dir],cwd=ROOT,capture_output=True,text=True).stdout.splitlines()
    if not out: return None
    f,ln=out[0].split(':')[0],int(out[0].split(':')[1])
    src=open(os.path.join(ROOT,f),encoding='utf8').read().split('\n')
    # attrs above
    at=ln-1; attrs=[]
    j=at-1
    while j>=0 and (src[j].strip().startswith('#[') or src[j].strip().startswith('///') or src[j].strip()==''):
        attrs.append(src[j]); j-=1
    rename=None
    for a in attrs:
        m=re.search(r'rename_all\s*=\s*"(\w+)"',a)
        if m: rename=m.group(1)
    fields=[]; depth=0; pend=[]
    k=at
    body=[]
    while k<len(src):
        body.append(src[k])
        if src[k].startswith('}'): break
        k+=1
    cur_attrs=[]
    for line in body[1:]:
        s=line.strip()
        if s.startswith('#['): cur_attrs.append(s); continue
        if s.startswith('///') or s=='' : continue
        m=re.match(r'pub (\w+):\s*(.+?),?$',s)
        if m:
            nm,ty=m.group(1),m.group(2)
            optional=ty.startswith('Option<') or any('default' in a or 'skip_serializing_if' in a for a in cur_attrs)
            rn=None
            for a in cur_attrs:
                mm=re.search(r'rename\s*=\s*"([^"]+)"',a)
                if mm: rn=mm.group(1)
            wire = rn or (camel(nm) if rename=='camelCase' else nm)
            fields.append((wire,ty,optional))
            cur_attrs=[]
    return f,fields,rename
seen=set()
for r in res:
    if not (r['stricter'] or r['lenientOther'] or r['lenientExtra']): continue
    key=(r['leaf'],r['name'])
    if key in seen: continue
    seen.add(key)
    leaf=r['leaf']; d=os.path.dirname(os.path.join(ROOT,leaf))
    doc=json.load(open(d+'/🔣️.json'))
    defs={**doc.get('definitions',{}),**doc.get('$defs',{})}
    node=defs.get(r['name']) if r['name'] in defs else (doc if doc.get('title')==r['name'] else None)
    props=set((node or {}).get('properties',{}).keys()); req=set((node or {}).get('required',[]))
    art=d.split('/🧬️schema')[0]
    art=os.path.relpath(art,ROOT)
    # artifact root = up to subsets dir
    art2=art.split('/🪆️subsets/')[0] if '/🪆️subsets/' in art else art
    rs=rust_struct(art2,r['name'])
    print('==',r['name'],'|',leaf.split('🗿️artifacts/')[-1].split('/🧬️schema')[0][:80])
    print('   schema props',sorted(props)[:30],'required',sorted(req))
    if rs:
        f,fields,ren=rs
        rreq={w for w,t,o in fields if not o}
        print('   rust',f.split('/')[-3:],'required',sorted(rreq),'optional',sorted(w for w,t,o in fields if o))
        print('   MISSING required in schema:',sorted((rreq&props)-req),'| rust fields not in schema:',sorted({w for w,t,o in fields}-props))
    else: print('   rust: none found')
