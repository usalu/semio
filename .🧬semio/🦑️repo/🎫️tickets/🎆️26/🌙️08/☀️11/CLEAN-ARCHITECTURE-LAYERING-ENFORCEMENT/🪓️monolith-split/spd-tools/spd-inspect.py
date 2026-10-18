"""🔍️ Shows, for one export, the schema definition, the hand-declared TypeScript and the Rust struct: `python3 spd-inspect.py <leaf .ts path> <Export>`."""
import json,sys,os,re,subprocess
ROOT='/Users/ueli/Documents/semio'
leaf=sys.argv[1]; name=sys.argv[2]
d=os.path.dirname(os.path.join(ROOT,leaf) if not leaf.startswith('/') else leaf)
doc=json.load(open(d+'/🔣️.json'))
defs={**doc.get('definitions',{}),**doc.get('$defs',{})}
node=defs.get(name) if name in defs else (doc if doc.get('title')==name else None)
print('SCHEMA',json.dumps(node,ensure_ascii=False)[:1400])
ts=open(d+'/🟦️.ts',encoding='utf8').read()
m=re.search(r'export (?:interface|type) '+name+r'\b[\s\S]*?\n}\n|export type '+name+r'\b[^\n]*\n',ts)
print('TS',(m.group(0) if m else 'none')[:1200])
art=d.split('/🧬️schema')[0]
out=subprocess.run(['git','-c','core.quotepath=false','grep','-n','-E','pub (struct|enum) '+name+r'( |<|\{|$)','--',os.path.relpath(art,ROOT)],cwd=ROOT,capture_output=True,text=True).stdout.splitlines()[:3]
print('RUST',out)
for line in out[:1]:
    f,ln=line.split(':')[0],int(line.split(':')[1])
    src=open(os.path.join(ROOT,f),encoding='utf8').read().split('\n')
    print('\n'.join(src[max(0,ln-4):ln+30]))
