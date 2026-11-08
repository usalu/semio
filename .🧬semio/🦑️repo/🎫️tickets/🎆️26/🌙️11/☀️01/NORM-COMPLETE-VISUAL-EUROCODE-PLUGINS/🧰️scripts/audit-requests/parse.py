import re,json,sys
src=sys.argv[1]
lines=open(src,encoding='utf-8').read().split('\n')
items=[];lane=None;idx=0
for i,l in enumerate(lines,1):
    m=re.match(r'^## (.+?) \((\d+)\)',l)
    if m: lane=m.group(1); continue
    if l.startswith('- ') and lane:
        idx+=1
        rq=re.search(r'\[requests\.md:([\d, requests.md:]+)\]\s*$',l)
        rline=rq.group(1) if rq else ''
        hint=re.search(r'\(→ ([^)]*)\)\s*\[requests',l)
        items.append(dict(n=idx,srcline=i,lane=lane,text=l[2:],reqline=rline,hint=hint.group(1) if hint else ''))
json.dump(items,open(sys.argv[2],'w'),ensure_ascii=False,indent=0)
print(len(items))
