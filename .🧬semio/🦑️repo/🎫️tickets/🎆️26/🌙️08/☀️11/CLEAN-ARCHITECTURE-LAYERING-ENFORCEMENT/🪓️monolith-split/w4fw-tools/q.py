import json, sys, collections
E=json.load(open('ents.json')); ents=E['ents']; edges=E['edges']
def closure(starts):
    seen=set(); st=list(starts)
    while st:
        x=st.pop()
        if x in seen: continue
        seen.add(x)
        st.extend(edges.get(x,[]))
    return seen
if __name__=='__main__':
    starts=sys.argv[1:]
    c=closure(starts)
    byf=collections.defaultdict(list)
    for x in c: byf[ents[x]['file']].append(x)
    tot=0
    for f,l in sorted(byf.items()):
        s=sum(ents[x]['lines'] for x in l); tot+=s
        print(f, len(l), s)
        for x in sorted(l, key=lambda x: ents[x]['start']): print('    ', ents[x]['name'], ents[x]['lines'])
    print('TOTAL', len(c), tot)
