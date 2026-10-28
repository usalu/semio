import json,re,subprocess,sys,os
R='/Users/ueli/Documents/semio'
lit=re.compile(r'''(?:Error|console\.(?:log|error|warn|info)|stderr\.write|stdout\.write)\(\s*("(?:[^"\\\n]|\\.)*"|'(?:[^'\\\n]|\\.)*'|`(?:[^`\\]|\\.)*`)''')
out=[]
for b in sys.argv[1:]:
    m=json.load(open(b+'/manifest.json'))
    for rel,e in m.items():
        if not e.get('base') or not e.get('new'): continue
        base=open(f"{b}/base/{e['file']}",encoding='utf8').read(); new=open(f"{b}/files/{e['file']}",encoding='utf8').read()
        gone=set(lit.findall(base))-set(lit.findall(new))
        for g in gone:
            body=g[1:-1]
            body=re.split(r'\$\{',body)[0]
            body=body.replace('\\n','\n').replace('\\"','"')
            if len(body.strip())<6 or ' ' not in body: continue
            out.append((b,rel,body[:60]))
print(len(out),'removed literals',file=sys.stderr)
seen=set()
for b,rel,body in out:
    key=body
    if key in seen: continue
    seen.add(key)
    r=subprocess.run(['git','grep','-lF','--',body,'--','🧰️framework','🚀️launcher','.github'],cwd=R,capture_output=True,text=True).stdout.split('\n')
    r=[x for x in r if x and x!=rel]
    if r: print(os.path.basename(b),'|',repr(body),'|',[x[-70:] for x in r][:4])
