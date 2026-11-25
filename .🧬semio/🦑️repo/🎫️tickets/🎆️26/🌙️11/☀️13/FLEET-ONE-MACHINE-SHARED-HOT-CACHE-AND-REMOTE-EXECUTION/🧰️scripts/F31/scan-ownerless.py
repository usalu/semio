import json,subprocess,re,os,sys
root=os.getcwd()
reg=json.load(open('📋️build-owners.json'))
cp=json.load(open('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json'))
wp=json.load(open('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🔣️policy.json'))
gen=set(cp['generatedDirectories']);col=set(wp['collections']['names']);mm=wp['collections']['moduleMember']
def excluded(p):
    parts=p.split('/')
    return p.startswith('.🧬semio/') or any(x in gen or (x in col and parts[i-1]!=mm) for i,x in enumerate(parts))
covered=set()
rows={}
for o in reg['owners']:
    covered.add(o['root']);covered.update(o.get('nativeRoots') or []);rows[o['name']]=o
out=subprocess.run(['git','ls-files','-z','-c','-o','--exclude-standard'],capture_output=True,check=True).stdout.decode().split('\0')
files=[p for p in out if p and os.path.basename(p) in('Cargo.toml','📋️project.json') and not excluded(p) and os.path.exists(p)]
pd={os.path.dirname(p) for p in files if p.endswith('project.json')}
def pkg(p):
    t=open(p,encoding='utf8').read()
    m=re.search(r'^\[package\]\s*$([\s\S]*?)(?=^\[|\Z)',t,re.M)
    if not m:return None
    n=re.search(r'^name\s*=\s*"([^"]+)"',m.group(1),re.M);return n and n.group(1)
miss=[]
for p in files:
    d=os.path.dirname(p)
    if p.endswith('project.json'):
        if not any(o['root']==d and o['manifest']==p for o in reg['owners']):miss.append(('project-without-row',p))
    elif d not in pd and d not in covered and pkg(p):miss.append(('cargo-without-row',p,pkg(p)))
for m in miss:print('\t'.join(m))
print('total',len(miss),file=sys.stderr)
