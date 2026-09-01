import io,os,re,subprocess
BASE="✏️s/🔌️plugins/🗄️stdio"
DRAW=f"{BASE}/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing"
MUT=f"{DRAW}/🧬️schema/🧬️mutations"
EXTRA=[f"{BASE}/📦️packages/🦀️rust/📦️glue.rs",
       f"{BASE}/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-drawing/🦀️.rs",
       f"{BASE}/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-drawing/🐍️.py"]
# kind, oldVariant, newVariant, oldMod, newMod, oldType, newType, oldDir, newDir
M=[("rotate","rotate-node","Rotate","RotateNode","rotate","rotate_node","Rotate","RotateNode","🔄rotate","🔄rotate-node"),
   ("scale","scale-node","Scale","ScaleNode","scale","scale_node","Scale","ScaleNode","📏scale","📏scale-node"),
   ("group","group-nodes","Group","GroupNodes","group","group_nodes","GroupNodes","GroupNodes","🧷group","🧷group-nodes"),
   ("ungroup","ungroup-node","Ungroup","UngroupNode","ungroup","ungroup_node","UngroupNode","UngroupNode","💫ungroup","💫ungroup-node"),
   ("flatten","flatten-node","Flatten","FlattenNode","flatten","flatten_node","FlattenNode","FlattenNode","🫓flatten","🫓flatten-node"),
   ("unflatten","unflatten-node","Unflatten","UnflattenNode","unflatten","unflatten_node","UnflattenNode","UnflattenNode","🎈unflatten","🎈unflatten-node")]

files=set(EXTRA)
for dp,dn,fn in os.walk(DRAW):
    for n in fn: files.add(os.path.join(dp,n))
changed=0
for p in sorted(files):
    if not os.path.isfile(p): continue
    try: s=io.open(p,encoding="utf8").read()
    except Exception: continue
    o=s
    for kind,nkind,ov,nv,om,nm,ot,nt,od,nd in M:
        s=s.replace(f"SemioDrawingMutation::{ov}(",f"SemioDrawingMutation::{nv}(")
        s=s.replace(f"{om}::{ot}",f"{nm}::{nt}")
        s=re.sub(rf"^(\s*)use super::{om};",rf"\1use super::{nm};",s,flags=re.M)
        s=re.sub(rf"^(\s{{4}}){ov}\({nm}::",rf"\1{nv}({nm}::",s,flags=re.M)
        s=re.sub(rf'kind: "{kind}"',f'kind: "{nkind}"',s)
        s=s.replace(f'"{kind}"',f'"{nkind}"')
        s=s.replace(f'"mutationId": "{kind}"',f'"mutationId": "{nkind}"')
        s=s.replace(f"/{od}",f"/{nd}")
    if s!=o:
        io.open(p,"w",encoding="utf8").write(s); changed+=1
print("files rewritten:",changed)
# leaf struct renames (scoped to the single leaf file)
for kind,nkind,ov,nv,om,nm,ot,nt,od,nd in M:
    if ot==nt: continue
    p=os.path.join(MUT,od,"🦀️.rs")
    if os.path.isfile(p):
        s=io.open(p,encoding="utf8").read()
        s=re.sub(rf"\b{ot}\b",nt,s)
        io.open(p,"w",encoding="utf8").write(s); print("struct renamed in",od)
# directory renames
for kind,nkind,ov,nv,om,nm,ot,nt,od,nd in M:
    a,b=os.path.join(MUT,od),os.path.join(MUT,nd)
    if os.path.isdir(a) and not os.path.isdir(b):
        subprocess.run(["mv",a,b],check=True); print("dir:",od,"->",nd)
