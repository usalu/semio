import re,sys,collections,json
def load(p, root):
    rows=[]
    for line in open(p,encoding='utf8'):
        m=re.match(r'^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$',line.rstrip('\n'))
        if m: rows.append((m.group(1).replace(root,'').lstrip('/'),int(m.group(2)),m.group(4),m.group(5)))
    return rows
SB='/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/'
a=load(sys.argv[1],SB+'spd-sandbox0/'); b=load(sys.argv[2],SB+'spd-sandbox/')
print('before',len(a),'after',len(b))
base=collections.Counter((f,c,m) for f,l,c,m in a)
new=[]
cnt=collections.Counter(base)
for f,l,c,m in b:
    k=(f,c,m)
    if cnt[k]>0: cnt[k]-=1
    else: new.append((f,l,c,m))
print('new',len(new))
codes=collections.Counter(c for f,l,c,m in new)
print(codes.most_common(15))
shape=collections.Counter((c,re.sub(r"'[^']*'","'…'",m)[:110]) for f,l,c,m in new)
for (c,m),v in shape.most_common(25): print(v,c,m)
json.dump(new,open(sys.argv[3],'w'))
