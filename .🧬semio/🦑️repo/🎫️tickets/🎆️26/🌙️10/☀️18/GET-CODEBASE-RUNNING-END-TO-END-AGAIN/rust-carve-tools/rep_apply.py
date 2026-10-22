import sys,os,re,json
sys.path.insert(0,os.path.dirname(__file__))
from lib import *
OS=F+"/🛍️products/💻️os"
S=OS+"/🔨️modules/🏪️store"
RP=S+"/🔁️replay"
OP=RP+"/🎮️operation"
LPKG=RP+"/📦️packages/🦀️rust"
OPKG=OP+"/📦️packages/🦀️rust"
NAME="semio-framework-os-store-replay-operation"
# 1. operation crate manifest from the replay manifest, one directory deeper
lm=get(LPKG+"/Cargo.toml")
m=lm.replace('name = "semio-framework-os-store-replay"','name = "'+NAME+'"').replace('id = "os-store-replay"','id = "os-store-replay-operation"')
m=re.sub(r'(?m)^(workspace\s*=\s*")((?:\.\./)+\.\.)(")',lambda x:x.group(1)+"../"+x.group(2)+x.group(3),m)
m=re.sub(r'(path\s*=\s*")(\.\./)',lambda x:x.group(1)+"../"+x.group(2),m)
assert 'name = "'+NAME+'"' in m
m=re.sub(r'(?m)^(\[package\]\n(?:.*\n)*?)(description\s*=.*\n)?',lambda x:x.group(0),m,count=1)
if "description =" in m:
    m=re.sub(r'(?m)^description\s*=.*$','description = "Domain-owned cooperative semantic preparation of one history operation: the preparation traits and task, native drivers, original history and derived operations"',m,count=1)
else:
    m=m.replace('[package.metadata.semio]','description = "Domain-owned cooperative semantic preparation of one history operation: the preparation traits and task, native drivers, original history and derived operations"\n\n[package.metadata.semio]',1)
m=re.sub(r'(?m)^\[lib\]\npath\s*=\s*"[^"]*"','[lib]\npath = "../../🦀️.rs"',m)
put(OPKG+"/Cargo.toml",m)
put(OPKG+"/📋️project.json",project_json("os-store-replay-operation"))
# 2. operation root: drop the ReplayOwnedState impl, widen what the impl uses
p=OP+"/🦀️.rs"
s=get(p)
a=s.index("impl<P, M> crate::ReplayOwnedState<P, M>")
b=s.index('#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]')
block=s[a:b].rstrip("\n")+"\n"
texts[p]=s[:a]+s[b:]
sub(p,"    fn new(outcome: semio_framework_os_spr::MutationReplayOutcome) -> Self {","    pub fn new(outcome: semio_framework_os_spr::MutationReplayOutcome) -> Self {")
sub(p,"    pub(super) fn close_cold(","    pub fn close_cold(")
sub(p,"//! 🎮️ Domain-owned cooperative semantic preparation for one history operation.\n","//! 🎮️ Domain-owned cooperative semantic preparation for one history operation.\n")
s=get(p)
texts[p]=s.rstrip("\n")+'''

#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
'''
# 3. replay: alias + moved impl
r=RP+"/🦀️.rs"
sub(r,'#[path = "🎮️operation/🦀️.rs"]\npub mod replay_preparation;\n','use semio_framework_os_store_replay_operation as replay_preparation;\n')
block=block.replace("pub(super) fn begin_replay_snapshot_retirement","pub(crate) fn begin_replay_snapshot_retirement").replace("pub(super) fn begin_replay_mutations_retirement","pub(crate) fn begin_replay_mutations_retirement").replace("pub(super) fn step_prepared_operation","pub(crate) fn step_prepared_operation")
block="\n".join(("    "+l if l.strip() else l) for l in block.split("\n"))
ops=RP+"/⏩️replay-owned-state-operations/🦀️.rs"
s=get(ops)
texts[ops]=s.rstrip("\n")+'''

//#region 🎮️OperationPreparation
mod operation_preparation {
    use super::*;
    use crate::replay_preparation::{ArtifactReplayPreparationRequest, OperationMessageSettlement, ReplayRetirement};
    use semio_framework_os_spr::{InputReplacement, MutationMessage};
    use semio_framework_os_store_one_item::ArtifactStoreOneItemGrant;

'''+block.rstrip()+'''
}
//#endregion 🎮️OperationPreparation
'''
sub(LPKG+"/Cargo.toml",'[dependencies]\n','[dependencies]\n'+NAME+' = { path = "../../🎮️operation/📦️packages/🦀️rust" }\n')
# 4. importers
import subprocess
out=subprocess.run(["rg","-l","-g","*.rs","-g","!.🧬semio","-g","!node_modules","semio_framework_os_store_replay::replay_preparation",R],capture_output=True,text=True).stdout.split("\n")
files=[f for f in out if f]
owners=json.load(open(R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-carve-x/rep-owners.json")) if os.path.exists(R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-carve-x/rep-owners.json") else {}
needs={}
for f in files:
    t=get(f)
    t=re.sub(r'(pub )?use semio_framework_os_store_replay::replay_preparation;',lambda x:(x.group(1) or "")+"use semio_framework_os_store_replay_operation as replay_preparation;",t)
    t=t.replace("semio_framework_os_store_replay::replay_preparation::","semio_framework_os_store_replay_operation::")
    assert "semio_framework_os_store_replay::replay_preparation" not in t,f
    texts[f]=t
    rel=os.path.relpath(f,R)
    if f.startswith(OP+"/") or f.startswith(RP+"/🦀️.rs") :continue
    o=owners.get(rel)
    if not o:o=[os.path.relpath(owner_manifest(f),R)]
    needs.setdefault(o[0],[]).append(rel)
for mf,fl in needs.items():
    add_dep_like(R+"/"+mf,"semio-framework-os-store-replay",NAME,OPKG)
print("owners:",{k[-80:]:len(v) for k,v in needs.items()})
# 5. registries
insert_sorted_member(R+"/Cargo.toml",os.path.relpath(OPKG,R))
add_build_owner("os-store-replay-operation",os.path.relpath(OPKG,R))
CB=OS+"/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json"
anchor='''    {
      "package": "semio-framework-os-store-replay",
      "manifest": "🔨️modules/🏪️store/🔁️replay/📦️packages/🦀️rust/Cargo.toml",'''
sub(CB,anchor,'''    {
      "package": "semio-framework-os-store-replay-operation",
      "manifest": "🔨️modules/🏪️store/🔁️replay/🎮️operation/📦️packages/🦀️rust/Cargo.toml",
      "entry": "🔨️modules/🏪️store/🔁️replay/🎮️operation/🦀️.rs",
      "maximumActiveLines": 3000,
      "forbiddenDependencies": [
        "semio-framework-os-kernel",
        "semio-framework-plugin",
        "semio-framework-os-store-replay",
        "semio-framework"
      ]
    },
'''+anchor)
flush()
