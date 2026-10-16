import re,sys,collections
path=sys.argv[1]
pat=re.compile(r'^(?P<f>[^ ].*?)\((?P<l>\d+),(?P<c>\d+)\): error (?P<code>TS\d+): ')
rows=[]
for line in open(path,encoding='utf8'):
    m=pat.match(line)
    if m: rows.append((m['f'],m['code']))
print(len(rows))
key=sys.argv[2] if len(sys.argv)>2 else 'file'
sub=sys.argv[3] if len(sys.argv)>3 else ''
c=collections.Counter()
for f,code in rows:
    if sub and sub not in f: continue
    c[(f if key=='file' else code)]+=1
for k,v in c.most_common(60): print(v,k)
