import subprocess,re,collections
macro=subprocess.run(['rg','-n',r'artifact_retire_leaf!\(([^)]*)\)','✏️s/🔌️plugins','-g','🦀️.rs','-o','-r','$1'],capture_output=True,text=True).stdout.split('\n')
names=collections.defaultdict(set)
for l in macro:
    m=re.match(r'(.*?):\d+:(.*)',l)
    if not m: continue
    art='/'.join(m.group(1).split('/')[:4])
    for t in m.group(2).split(','):
        t=t.strip().split('::')[-1]
        if t: names[art].add(t)
st=subprocess.run(['rg','-n',r'artifact_retire_struct!\(\s*([\w:]+)\s*\{','✏️s/🔌️plugins','-g','🦀️.rs','-o','-r','$1'],capture_output=True,text=True).stdout.split('\n')
for l in st:
    m=re.match(r'(.*?):\d+:(.*)',l)
    if m: names['/'.join(m.group(1).split('/')[:4])].add(m.group(2).split('::')[-1])
SKIP={'DrawCanvas'}
files=subprocess.run(['rg','-l','RetireOwned','✏️s/🔌️plugins','-g','🦀️.rs'],capture_output=True,text=True).stdout.split('\n')
n=0
for f in files:
    if not f.strip(): continue
    art='/'.join(f.split('/')[:4])
    if art not in names: continue
    s=open(f,encoding='utf-8').read();o=s
    for m in reversed(list(re.finditer(r'#\[derive\(([^\]]*)\)\]\s*(?:#\[[^\]]*\]\s*)*pub (?:struct|enum) (\w+)',s))):
        if m.group(2) in names[art] and m.group(2) not in SKIP and 'RetireOwned' in m.group(1):
            d=re.sub(r'\s*(?:value_derive|semio_framework_value)::RetireOwned,?','',m.group(1),count=1)
            d=re.sub(r',\s*,',',',d).strip().rstrip(',')
            s=s[:m.start(1)]+d+s[m.end(1):];n+=1
    if s!=o: open(f,'w',encoding='utf-8').write(s)
print(n)
