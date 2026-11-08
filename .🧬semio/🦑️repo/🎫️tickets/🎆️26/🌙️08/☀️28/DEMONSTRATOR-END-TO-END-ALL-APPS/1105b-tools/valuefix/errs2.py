import re,sys
t=open(sys.argv[1]).read();pat=sys.argv[2];n=int(sys.argv[3])
out={}
for b in re.split(r'\n(?=error)',t):
    if not b.startswith('error'):continue
    m=re.search(r'--> (\S+):(\d+):(\d+)',b)
    if not m or not re.search(pat,m.group(1)):continue
    out.setdefault(re.sub(r'.*?\.\./\.\./','',m.group(1)),[]).append((int(m.group(2)),int(m.group(3)),b.split('\n')[0][:130]))
for f,v in out.items():
    print(f,len(v))
    for x in sorted(v)[:n]:print('  ',*x)
