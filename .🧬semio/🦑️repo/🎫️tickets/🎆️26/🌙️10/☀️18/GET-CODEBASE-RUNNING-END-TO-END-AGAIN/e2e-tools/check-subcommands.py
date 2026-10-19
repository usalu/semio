import json,subprocess,re,os,sys,shlex
root='/Users/ueli/Documents/semio'
os.chdir(root)
here=os.path.dirname(os.path.abspath(sys.argv[0]))
closure=json.load(open(os.path.join(here,'../🗑️generated/e2erecheck-tmp/closure2.json'),encoding='utf8'))
files=subprocess.run(['git','ls-files','-co','--exclude-standard','*project.json'],capture_output=True,text=True).stdout.split('\n')
files=[f for f in files if f.endswith('project.json') and 'node_modules' not in f and '.🧬semio' not in f and os.path.exists(f)]
byname={}
for f in files:
    try: d=json.load(open(f,encoding='utf8'))
    except Exception: continue
    byname[d.get('name')]=(f,d)
imp=re.compile(r'''(?:\bfrom\s*|\bimport\s*\(?\s*)(["'])(\.{1,2}/[^"'\n]+)\1''')
cache={}
def closure_text(path,depth=0,seen=None):
    seen=seen if seen is not None else set()
    if path in seen or not os.path.exists(path) or depth>6: return ''
    seen.add(path)
    try: t=open(path,encoding='utf8').read()
    except: return ''
    out=t
    for m in imp.finditer(t):
        p=os.path.normpath(os.path.join(os.path.dirname(path),m.group(2)))
        if os.path.exists(p) and os.path.isfile(p): out+=closure_text(p,depth+1,seen)
    return out
bad=0; total=0
for script,keys in sorted(closure['scripts'].items()):
    for key in keys:
        proj,target=key.rsplit(':',1)
        if proj not in byname: continue
        t=byname[proj][1].get('targets',{}).get(target)
        if not t: continue
        cmd=t['options']['command']
        toks=shlex.split(cmd)
        i=[j for j,x in enumerate(toks) if x.endswith(('.ts','.mjs'))]
        if not i: continue
        sub=toks[i[0]+1] if len(toks)>i[0]+1 else None
        if sub is None: continue
        total+=1
        txt=cache.setdefault(script,closure_text(script))
        ok=re.search(r'''(register(?:Lazy)?\(\s*|case\s+|\[\s*|===\s*|==\s*|["']command["']\s*:\s*|\bcommands?\s*[:=]\s*\{[^}]*)["']'''+re.escape(sub)+r'''["']''',txt) or re.search(r'''["']'''+re.escape(sub)+r'''["']''',txt)
        strict=re.search(r'''register(?:Lazy)?\(\s*["']'''+re.escape(sub)+r'''["']''',txt)
        if not ok: bad+=1; print('NO-COMMAND',key,'->',script.split('/')[-4:],sub)
        elif not strict: print('weak (quoted only)',key,sub)
print(total,'commands checked;',bad,'missing')
