import json,glob,re,sys
pat=sys.argv[1]
hits={}
for f in glob.glob("✏️s/🔌️plugins/*/🔣️.json")+glob.glob("✏️s/🔌️plugins/*/*/*/🔣️.json"):
    try: d=json.load(open(f))
    except Exception: continue
    m=d.get("manifest")
    if not isinstance(m,dict): continue
    pid=m["pluginId"]
    def acts(app):
        for a in app.get("actions",[]): yield a
        for w in app.get("windowKinds",[]):
            for a in w.get("actions",[]): yield a
        for mo in app.get("modes",[]):
            for c in mo.get("commands",[]): yield c
        for c in app.get("commands",[]): yield c
    for app in m["apps"]:
        for a in acts(app):
            i=a["id"]
            if re.search(pat,i):
                s=a.get("semantics",{})
                hits.setdefault(i,[]).append((pid,a.get("kind"),s.get("effects",{}).get("destructive"),s.get("audience")))
for i,v in sorted(hits.items()):
    ps=sorted(set(p for p,_,_,_ in v))
    ds=set(str(x[2]) for x in v); ks=set(str(x[1]) for x in v); au=set(str(x[3]) for x in v)
    print(f"{i:30} kind={','.join(ks):18} destr={','.join(ds):10} aud={','.join(au):12} n={len(ps)} {','.join(ps)[:80]}")
