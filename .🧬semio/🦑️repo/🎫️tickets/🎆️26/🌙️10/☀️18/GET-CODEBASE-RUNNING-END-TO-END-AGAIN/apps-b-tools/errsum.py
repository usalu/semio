import sys,re,collections
s=open(sys.argv[1],encoding='utf-8',errors='replace').read().split('\n')
c=collections.Counter();ex={}
crates=[]
for l in s:
    m=re.match(r'^error: could not compile `([^`]+)`',l)
    if m: crates.append(l[7:140])
    m=re.match(r'^(?:/Users/ueli/Documents/semio/)?(.*?):(\d+):(\d+): error(\[E\d+\])?: (.*)$',l)
    if not m: continue
    p=re.sub(r'(/\.)+/','/',m.group(1)); p=p.split('/📦️packages/🦀️rust/')[-1] if '/📦️packages/🦀️rust/' in p else p
    key=(p[-70:],m.group(4),re.sub(r'`[^`]*`','`_`',m.group(5))[:60])
    c[key]+=1; ex.setdefault(key,l)
print('\n'.join(crates))
for k,v in c.most_common(int(sys.argv[2]) if len(sys.argv)>2 else 40): print(v,k)
