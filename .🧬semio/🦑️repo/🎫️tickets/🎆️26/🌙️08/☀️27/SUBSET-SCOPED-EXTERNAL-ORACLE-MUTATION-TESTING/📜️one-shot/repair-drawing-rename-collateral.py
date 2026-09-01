import io,os,json,re
BASE="✏️s/🔌️plugins/🗄️stdio"
DRAW=f"{BASE}/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing"
MUT=f"{DRAW}/🧬️schema/🧬️mutations"
CAT=f"{DRAW}/🧪️oracle/🔣️.json"
# kind_old, kind_new, variant_old, variant_new, display_new, camel_new
M=[("rotate","rotate-node","Rotate","RotateNode","Rotate Node","rotateNode"),
   ("scale","scale-node","Scale","ScaleNode","Scale Node","scaleNode"),
   ("group","group-nodes","Group","GroupNodes","Group Nodes","groupNodes"),
   ("ungroup","ungroup-node","Ungroup","UngroupNode","Ungroup Node","ungroupNode"),
   ("flatten","flatten-node","Flatten","FlattenNode","Flatten Node","flattenNode"),
   ("unflatten","unflatten-node","Unflatten","UnflattenNode","Unflatten Node","unflattenNode")]
leafjson={os.path.join(MUT,d,"🔣️.json") for d in os.listdir(MUT) if os.path.isdir(os.path.join(MUT,d))}
reverted=0; fixtures=0
for dp,dn,fn in os.walk(DRAW):
    for n in fn:
        p=os.path.join(dp,n)
        if not n.endswith(".json"): continue
        if p==CAT or p in leafjson: continue
        s=io.open(p,encoding="utf8").read(); o=s
        for ko,kn,vo,vn,disp,camel in M:
            s=s.replace(f'"{kn}"',f'"{ko}"')          # revert data-position damage
            s=s.replace(f'"{vo}":',f'"{vn}":')        # externally-tagged mutation fixtures
        if s!=o: io.open(p,"w",encoding="utf8").write(s); reverted+=1
print("json files repaired:",reverted)
# leaf descriptors: aggregateVariant + displayName
for ko,kn,vo,vn,disp,camel in M:
    for d in os.listdir(MUT):
        p=os.path.join(MUT,d,"🔣️.json")
        if not os.path.isfile(p): continue
        j=json.load(open(p,encoding="utf8"))
        if j.get("semanticKind")==kn:
            j["aggregateVariant"]=vn; j["displayName"]=disp
            io.open(p,"w",encoding="utf8").write(json.dumps(j,ensure_ascii=False,indent=2)+"\n")
            print("  descriptor:",d,"->",vn)
# rust repairs
for dp,dn,fn in os.walk(DRAW):
    for n in fn:
        if not n.endswith(".rs"): continue
        p=os.path.join(dp,n); s=io.open(p,encoding="utf8").read(); o=s
        for ko,kn,vo,vn,disp,camel in M:
            s=s.replace(f'verb: "{kn}"',f'verb: "{ko}"')      # verbs must stay approved
            s=s.replace(f'"{kn}" =>',f'"{ko}" =>')            # text opcodes unchanged
            s=s.replace(f'"kind":"{kn}"',f'"kind":"{ko}"')    # doc comment: DrawNode tag
            s=s.replace(f'"{kn}", ',f'"{camel}", ').replace(f'"{kn}"]',f'"{camel}"]')  # binary camelCase list
        if s!=o: io.open(p,"w",encoding="utf8").write(s); fixtures+=1
print("rust files repaired:",fixtures)
