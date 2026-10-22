import os,re,subprocess,sys,json
# usage: adddep.py MAP.json  | build map: adddep.py --map MAP.json ; add: adddep.py MAP.json crate dep [dev] [features=a,b]
if sys.argv[1]=='--map':
    out=subprocess.run(['rg','-n','--no-heading',r'^name = "',  '--glob','Cargo.toml','🧰️framework','✏️s','🌎️hub','🎓️teaching'],capture_output=True,text=True).stdout
    m={}
    for l in out.split('\n'):
        mm=re.match(r'(.*Cargo\.toml):\d+:name = "([^"]+)"',l)
        if mm: m[mm.group(2)]=mm.group(1)
    json.dump(m,open(sys.argv[2],'w')); print(len(m)); sys.exit()
m=json.load(open(sys.argv[1]))
crate,dep=sys.argv[2],sys.argv[3]
dev='dev' in sys.argv[4:]
feats=[a[9:] for a in sys.argv[4:] if a.startswith('features=')]
f=m[crate]; d=os.path.dirname(m[dep])
rel=os.path.relpath(d,os.path.dirname(f))
line=f'{dep} = {{ path = "{rel}"'+(f', features = [{", ".join(chr(34)+x+chr(34) for x in feats[0].split(","))}]' if feats else '')+' }\n'
s=open(f).read()
sec='[dev-dependencies]\n' if dev else '[dependencies]\n'
body_has=re.search(r'(?m)^'+re.escape(dep)+r'\s*=',s.split(sec)[1].split('\n[')[0]) if sec in s else None
if body_has: print('present',crate,dep); sys.exit()
if sec in s: s=s.replace(sec,sec+line,1)
else: s=s.rstrip('\n')+'\n\n'+sec+line
open(f,'w').write(s); print('added',crate,dep,'->',f[-60:])
