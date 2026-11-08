import json,re,sys
T=sys.argv[1]; a,b=int(sys.argv[2]),int(sys.argv[3])
items=json.load(open(f"{T}/🗑️generated/audit-requests/items.json"))
tk=json.load(open(f"{T}/🗑️generated/audit-requests/tokens.json"))
res=tk['res']
for it in items:
    if not(a<=it['n']<=b): continue
    sig=[]
    for t in re.findall(r'`([^`]+)`',it['text']):
        t=t.strip()
        if t not in res: continue
        k,v=res[t]
        if k=='path': sig.append(f"{t[:60]}=>{'OK' if v!='MISSING' else 'MISSING'}")
        elif k=='ident': sig.append(f"{t[:50]}=#{v}")
    s=' | '.join(dict.fromkeys(sig))
    print(f"{it['n']}[{it['lane'][:6]}] {it['text'][:150]} || {s[:300]}")
