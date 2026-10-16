"""🔬 Region-level identifier dependency analysis of one Rust file: which top-level names each //#region defines and which names of other regions it mentions."""
import re,sys,collections
path=sys.argv[1]
text=open(path,encoding='utf-8').read()
lines=text.split('\n')
regions=[]
cur=None
stack=[]
for i,l in enumerate(lines):
    m=re.match(r'\s*//#region\s+(.*)$',l)
    if m and not stack:
        stack.append(m.group(1).strip());cur=[m.group(1).strip(),i,None];continue
    if m: stack.append(m.group(1).strip());continue
    m=re.match(r'\s*//#endregion\s+(.*)$',l)
    if m:
        stack.pop()
        if not stack: cur[2]=i;regions.append(cur);cur=None
defre=re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:unsafe\s+)?(?:fn|struct|enum|trait|type|const|static|mod)\s+(\w+)')
defs=collections.defaultdict(set)
for name,a,b in regions:
    for i in range(a,b+1):
        m=defre.match(lines[i])
        if m: defs[name].add(m.group(1))
        m=re.match(r'^impl(?:<[^>]*>)?\s+(?:\w+(?:<[^>]*>)?\s+for\s+)?(\w+)',lines[i])
for name,a,b in regions:
    body='\n'.join(lines[a:b+1])
    uses={}
    for other,names in defs.items():
        if other==name: continue
        hit=sorted(n for n in names if re.search(r'\b'+re.escape(n)+r'\b',body))
        if hit: uses[other]=hit
    print(f'## {name} lines {a+1}-{b+1} ({b-a+1}) defs={len(defs[name])}')
    for o,h in uses.items(): print(f'   -> {o}: {", ".join(h[:14])}{" ..." if len(h)>14 else ""}')
