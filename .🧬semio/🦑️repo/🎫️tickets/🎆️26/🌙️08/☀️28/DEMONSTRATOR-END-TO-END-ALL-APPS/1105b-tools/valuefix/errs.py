import re,sys
t=open(sys.argv[1]).read()
blocks=re.split(r'\n(?=error)',t);out={}
for b in blocks:
    if not b.startswith('error'):continue
    m=re.search(r'--> .*?edit/📦️packages/🦀️rust/\.\./\.\./(\S+):(\d+):(\d+)',b)
    if not m:
        if 'could not compile' not in b and 'aborting' not in b: out.setdefault('OTHER',[]).append((0,b.split('\n')[0][:200]+' | '+(b.split('\n')[1][:150] if '\n' in b else '')))
        continue
    out.setdefault(m.group(1),[]).append((int(m.group(2)),int(m.group(3)),b.split('\n')[0][:120]))
for f,v in out.items():
    print(f,len(v))
    for x in sorted(v)[:int(sys.argv[2]) if len(sys.argv)>2 else 12]:print('  ',*x)
