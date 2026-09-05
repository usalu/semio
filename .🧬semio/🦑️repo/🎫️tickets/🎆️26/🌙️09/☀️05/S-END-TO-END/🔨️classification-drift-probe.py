import json,os,re,sys
ROOT="/Users/ueli/Documents/semio"
PL=os.path.join(ROOT,"✏️s/🔌️plugins")
CAMEL={"Unclassified":"unclassified","Migrated":"migrated","BatchOnlyPendingRewrite":"batchOnlyPendingRewrite","ForbiddenFromUi":"forbiddenFromUi","Deleted":"deleted"}
RX=re.compile(r'\.action_interactive_job\(\s*"([^"]+)"\s*,\s*(?:\w+::)*InteractiveJobClassification::(\w+)\s*\)')
SKIP={"target","node_modules","🤖️generated","🧩️extensions",".git"}
def rust_map(root):
    m={}
    for dirpath,dirnames,files in os.walk(root):
        dirnames[:]=[d for d in dirnames if d not in SKIP and not d.startswith("target")]
        for f in files:
            if not f.endswith(".rs"): continue
            try: t=open(os.path.join(dirpath,f),encoding="utf8").read()
            except Exception: continue
            for aid,var in RX.findall(t):
                m.setdefault(aid,set()).add(CAMEL.get(var,var))
    return m
def desc_map(d):
    m={}
    for app in d.get("manifest",{}).get("apps",[]) or []:
        for wk in app.get("windowKinds",[]) or []:
            for act in wk.get("actions",[]) or []:
                v=(act.get("semantics") or {}).get("execution",{}).get("interactiveJob") or "unclassified"
                m.setdefault(act.get("id"),set()).add(v)
    return m
tot=0
for name in sorted(os.listdir(PL)):
    root=os.path.join(PL,name)
    j=os.path.join(root,"🔣️.json")
    if not os.path.isfile(j): continue
    try: d=json.load(open(j,encoding="utf8"))
    except Exception as e: print(name,"JSON ERR",e); continue
    rm=rust_map(root); dm=desc_map(d)
    bad=[]
    for aid,vals in sorted(dm.items()):
        rs=rm.get(aid)
        if rs is None:
            for v in sorted(vals):
                if v!="unclassified": bad.append(f"{aid}: descriptor {v} but no Rust declaration")
        else:
            for v in sorted(vals):
                if v not in rs: bad.append(f"{aid}: descriptor {v} not in Rust {{{','.join(sorted(rs))}}}")
    print(f"{name}: rust={len(rm)} descActions={len(dm)} drift={len(bad)}")
    for b in bad[:6]: print("   -",b)
    tot+=len(bad)
print("TOTAL DRIFT",tot)
