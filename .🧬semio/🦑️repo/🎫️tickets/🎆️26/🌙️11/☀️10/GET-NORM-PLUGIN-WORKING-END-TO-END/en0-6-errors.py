import re,sys,collections
s=open(sys.argv[1],errors='ignore').read()
print(re.findall(r'could not compile `([^`]+)` \(lib[^)]*\) due to (\d+)',s))
blocks=re.split(r'\n(?=error(?:\[E\d+\])?: )',s)
seen=set();n=0
for b in blocks:
    ls=b.split('\n')
    if len(ls)>1 and 'could not compile' not in ls[0] and re.search(r'artifacts/[^ ]*en199[0-6]',ls[1]):
        k=(ls[0],ls[1])
        if k in seen: continue
        seen.add(k);n+=1
        if n<=int(sys.argv[2]): print('\n'.join(ls[:int(sys.argv[3])])[:1500]);print('-----')
print(n)
