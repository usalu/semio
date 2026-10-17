import re,collections,sys
lines=open(sys.argv[1],encoding='utf8').read().split('\n')
st=[i for i,l in enumerate(lines) if l.startswith('blocker:')][int(sys.argv[2])]
en=[i for i,l in enumerate(lines) if l.startswith('blocker:') and i>st]
en=en[0] if en else len(lines)
short=lambda s:s.replace('{','')[:int(sys.argv[3]) if len(sys.argv)>3 else 50]
seen=collections.OrderedDict()
for l in lines[st:en]:
    m=re.match(r'^    \{(.*?)\} => \{(.*?)\} via (.*)$',l)
    if m:
        k=(short(m.group(1)),short(m.group(2)))
        seen.setdefault(k,m.group(3)[:130])
src=collections.defaultdict(list)
for (a,b),v in seen.items(): src[a].append((b,v))
for a,bs in src.items():
    # skip leaf-mutation groups to reduce noise unless asked
    print("##",a)
    for b,v in bs: print("    ->",b,"|",v)
