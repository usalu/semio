"""✂️ Batch C: re-point and delete zero-or-few-consumer module aliases of the actor, scene, flow and host facades."""
import re,os,sys
os.chdir("/Users/ueli/Documents/semio")
APPLY="--apply" in sys.argv
M="🧰️framework/🔨️modules/"
P="🧰️framework/🛍️products/💻️os/"
def rd(p): return open(p,encoding="utf8").read()
def wr(p,s):
    if APPLY: open(p,"w",encoding="utf8").write(s)
def sub_path(t,name,target):
    return re.sub(r"(?<![A-Za-z0-9_:.])"+name+r"::",target+"::",t)
def drop(t,line,count=1):
    assert t.count(line)==count,(line,t.count(line))
    return t.replace(line,"")
edits=[]
# C1 vocabulary
v=M+"🎭️actor/📖️vocabulary/🦀️.rs"
t=rd(v);o=t
t=drop(t,"pub use semio_framework_actor_pack as pack;\n");t=drop(t,"pub use semio_framework_job as job;\n");t=drop(t,"pub use semio_framework_actor_retained_turn as retained_turn;\n")
for n,c in (("pack","semio_framework_actor_pack"),("job","semio_framework_job"),("retained_turn","semio_framework_actor_retained_turn")): t=sub_path(t,n,c)
edits.append((v,o,t))
cp=M+"🎭️actor/📥️cold-pair/🦀️.rs"
t=rd(cp);o=t
assert t.count("use crate::{instance_lifetime::ActorInstanceLifetime, pack};")==1
t=t.replace("use crate::{instance_lifetime::ActorInstanceLifetime, pack};","use crate::instance_lifetime::ActorInstanceLifetime;")
t=sub_path(t,"pack","semio_framework_actor_pack")
edits.append((cp,o,t))
# C2 job publication
jp=M+"🎭️actor/💼️job/📨️publication/🦀️.rs"
t=rd(jp);o=t
t=drop(t,"pub use semio_framework_actor_pack as pack;\n");t=drop(t,"pub use semio_framework_job as job;\n")
t=sub_path(t,"job","semio_framework_job")
edits.append((jp,o,t))
jt=M+"🎭️actor/💼️job/📨️publication/🧪️tests/🦀️.rs"
t=rd(jt);o=t
t=sub_path(t,"job","semio_framework_job")
edits.append((jt,o,t))
# C3 retained-turn
rp=M+"🎭️actor/🎟️retained-turn/📦️packages/🦀️rust/🦀️.rs"
t=rd(rp);o=t
t=drop(t,"pub use semio_framework_actor_pack as pack;\n")
edits.append((rp,o,t))
ri=M+"🎭️actor/🎟️retained-turn/🦀️.rs"
t=rd(ri);o=t
t=re.sub(r"(?<![A-Za-z0-9_])crate::pack(?![A-Za-z0-9_])","semio_framework_actor_pack",t)
edits.append((ri,o,t))
# C4 ui scene
us=M+"🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️.rs"
t=rd(us);o=t
t=drop(t,"pub use semio_framework_ui_scene_pack as pack;\n")
edits.append((us,o,t))
# C5 flow root
fr=P+"🔨️modules/🌊️flow/📦️packages/🦀️rust/🦀️.rs"
t=rd(fr);o=t
for l in ("pub use semio_framework_os_flow_host as host;\n","pub use semio_framework_os_flow_drawing as drawing;\n","pub use semio_framework_os_flow_vcs as vcs;\n","    pub use semio_framework_os_flow_extensions as wasm;\n"): t=drop(t,l)
i=t.index("// 🌉️ `wasm_session`'s");j=t.index("pub use semio_framework_os_flow_wasm as wasm_session;\n")+len("pub use semio_framework_os_flow_wasm as wasm_session;\n")
t=t[:i]+t[j:]
t=t.replace("//! 🌊️ OS flow family facade — re-exports the pre-split public API of the document, catalogue, registry, bridge, host, drawing, vcs, geometry, mesh and wasm session producers.","//! 🌊️ OS flow family facade — re-exports the document, catalogue, registry, bridge, geometry and mesh producers.")
edits.append((fr,o,t))
# C6 artifact flow copy
af=P+"🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs"
t=rd(af);o=t
t=drop(t,"    pub use semio_framework_artifact_flow_flow_copy as copy;\n")
edits.append((af,o,t))
# C7 os host codec_abi
oh=P+"🖥️host/🦀️.rs"
t=rd(oh);o=t
t=drop(t,"pub use semio_framework_os_host_codec_abi as codec_abi;\n")
edits.append((oh,o,t))
# C8 framework facade source_projection
fw="🧰️framework/📦️packages/🦀️rust/🦀️.rs"
t=rd(fw);o=t
t=drop(t,"pub use semio_framework_source_projection as source_projection;\n")
edits.append((fw,o,t))
for p,o,t in edits:
    print("EDIT",p[-60:],"lines",o.count("\n"),"->",t.count("\n"))
    if APPLY:
        assert rd(p)==o
        wr(p,t)
