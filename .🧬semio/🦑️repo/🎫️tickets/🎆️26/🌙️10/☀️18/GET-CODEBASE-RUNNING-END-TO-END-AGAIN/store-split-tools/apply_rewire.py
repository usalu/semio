import os,sys,json,re
R="/Users/ueli/Documents/semio"
S=R+"/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
OS=R+"/🧰️framework/🛍️products/💻️os"
E=S+"/⚙️engine"
LIFT=E+"/📥️lifted/🏪️store"
apply=len(sys.argv)>1 and sys.argv[1]=="apply"
texts={}
def get(path):
    if path not in texts: texts[path]=open(path,encoding='utf8').read()
    return texts[path]
def sub(path,old,new,count=1):
    s=get(path)
    assert s.count(old)==count,(path,old,s.count(old))
    texts[path]=s.replace(old,new)
def relpath_dep(pkgdir,target): return os.path.relpath(target,pkgdir)
MG=S+"/🧩️composition/📬️publication/🤝️group/📦️packages/🦀️rust"
CL=E+"/🧯️close/📦️packages/🦀️rust"
def addline(path,after_prefix,line):
    lines=get(path).split("\n")
    idx=[i for i,l in enumerate(lines) if l.startswith(after_prefix)]
    assert len(idx)==1,(path,after_prefix,idx)
    assert line not in lines
    lines.insert(idx[0]+1,line)
    texts[path]="\n".join(lines)
# 4 engine root: remove member_group mount
sub(E+"/🦀️.rs",'#[path = "../🧩️composition/📬️publication/🤝️group/🦀️.rs"]\npub mod member_group;\n','')
# 5 engine manifest: close dep
EM=E+"/📦️packages/🦀️rust/Cargo.toml"
addline(EM,"semio-framework-os-store-engine-bundle =",f'semio-framework-os-store-engine-close = {{ path = "{relpath_dep(E+"/📦️packages/🦀️rust",CL)}" }}')
# 6 engine close-host + runtime-seed
CH=LIFT+"/🧯️close-host-drop-and-pair-merge/🦀️.rs"
sub(CH,"{ ArtifactStore::close_cursor_admit(self, phase, grant) }","{ self.parts().close_cursor_admit(phase, grant) }")
sub(CH,"{ ArtifactStore::owned_roots_terminal_is_empty(self) }","{ self.parts_ref().owned_roots_terminal_is_empty() }")
imp="use semio_framework_os_store_engine_close::{StoreCloseAdmission as _, StoreCloseWitness as _};\n"
sub(CH,"use super::*;\nuse semio_framework_os_spr::ColdMutation;\n","use super::*;\nuse semio_framework_os_spr::ColdMutation;\n"+imp)
RS=LIFT+"/🌱️runtime-seed-construction-and-owned-close/🦀️.rs"
sub(RS,"if !self.close_structural_owners_terminal_is_empty()","if !self.parts_ref().close_structural_owners_terminal_is_empty()")
sub(RS,"use super::*;\nuse semio_framework_os_spr::ColdMutation;\n","use super::*;\nuse semio_framework_os_spr::ColdMutation;\n"+imp)
# 7 space-member
SM=S+"/🧩️composition/👥️space-member"
SMF=SM+"/📥️lifted/🏪️store/🏪️space-member-for-artifact-store/🦀️.rs"
sub(SMF,"self.stage_apply_batch_group(","self.parts().stage_apply_batch_group(")
sub(SMF,"self.adopt_apply_batch_group(","self.parts().adopt_apply_batch_group(")
sub(SMF,"self.abort_apply_batch_group(","self.parts().abort_apply_batch_group(")
sub(SMF,"use super::*;\n","use super::*;\nuse semio_framework_os_store_engine_member_group::ApplyBatchGroup as _;\n")
mgdep=lambda pkg:f'semio-framework-os-store-engine-member-group = {{ path = "{relpath_dep(pkg,MG)}" }}'
addline(SM+"/📦️packages/🦀️rust/Cargo.toml","semio-framework-os-store-engine-core =",mgdep(SM+"/📦️packages/🦀️rust"))
# 8 kernel root
sub(S+"/🦀️.rs","use semio_framework_os_store_engine::member_group;\n","use semio_framework_os_store_engine_member_group as member_group;\n")
OSM=OS+"/📦️packages/🦀️rust/Cargo.toml"
addline(OSM,"semio-framework-os-store-engine-live =",mgdep(OS+"/📦️packages/🦀️rust"))
# 9 prelude
PR=S+"/🧪️testing/🧭️prelude"
sub(PR+"/🦀️.rs","pub use semio_framework_os_store_engine::member_group;\n","pub use semio_framework_os_store_engine_member_group::{self as member_group, ApplyBatchGroup as _};\n")
addline(PR+"/📦️packages/🦀️rust/Cargo.toml","semio-framework-os-store-engine-core =",mgdep(PR+"/📦️packages/🦀️rust"))
# 10 bin-01 test
T=S+"/🧪️tests/🏙️bin-01/🔬️unit/🏷️catalog-staging-command-refusal-and-direct-fixture-laws/🦀️.rs"
sub(T,"store.stage_apply_batch_group(","store.parts().stage_apply_batch_group(",2)
sub(T,"store.adopt_apply_batch_group(","store.parts().adopt_apply_batch_group(")
# 12 owners law
OW=S+"/🏗️initialization/🏠️owners/🏭️member/📥️source"
old="⚙️engine/📥️lifted/🏪️store/🚦️cursor-admission-dispatch-and-apply-batch/🦀️.rs"
new="⚙️engine/🧯️close/📥️lifted/🏪️store/🦀️.rs"
sub(OW+"/🔣️.json",old,new)
sub(OW+"/🧬️schema/🔣️.json",old,new,2)
# 11 registries
RC=R+"/Cargo.toml"
s=open(RC,encoding='utf8').read()
pre='    "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/'
def member(rel): return pre+rel+'",'
lines=s.split("\n")
start=lines.index("members = [");end=lines.index("]",start)
mem=lines[start+1:end]
def insert_member(mem,rel):
    line=member(rel)
    assert line not in mem
    keys=[l.encode() for l in mem]
    i=0
    while i<len(mem) and mem[i].encode()<line.encode(): i+=1
    mem.insert(i,line)
insert_member(mem,"⚙️engine/🧯️close/📦️packages/🦀️rust")
insert_member(mem,"🧩️composition/📬️publication/🤝️group/📦️packages/🦀️rust")
lines[start+1:end]=mem
texts[RC]="\n".join(lines)
BO=R+"/📋️build-owners.json"
sub(BO,'''    {
      "name": "@semio-tech/os-store-engine-core-rs",''','''    {
      "name": "@semio-tech/os-store-engine-close-rs",
      "root": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧯️close/📦️packages/🦀️rust",
      "manifest": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧯️close/📦️packages/🦀️rust/📋️project.json",
      "packages": []
    },
    {
      "name": "@semio-tech/os-store-engine-core-rs",''')
sub(BO,'''    {
      "name": "@semio-tech/os-store-engine-owners-rs",''','''    {
      "name": "@semio-tech/os-store-engine-member-group-rs",
      "root": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/📦️packages/🦀️rust",
      "manifest": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/📦️packages/🦀️rust/📋️project.json",
      "packages": []
    },
    {
      "name": "@semio-tech/os-store-engine-owners-rs",''')
CB=OS+"/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json"
s=get(CB)
anchor='''    {
      "package": "semio-framework-os-store-engine-bundle",'''
assert s.count(anchor)==1
def row(pkg,rel,maxlines): return f'''    {{
      "package": "{pkg}",
      "manifest": "🔨️modules/🏪️store/{rel}/📦️packages/🦀️rust/Cargo.toml",
      "entry": "🔨️modules/🏪️store/{rel}/🦀️.rs",
      "maximumActiveLines": {maxlines},
      "forbiddenDependencies": [
        "semio-framework-os-kernel",
        "semio-framework-plugin",
        "semio-framework"
      ]
    }},
'''
s=s.replace(anchor,row("semio-framework-os-store-engine-close","⚙️engine/🧯️close",1000)+row("semio-framework-os-store-engine-member-group","🧩️composition/📬️publication/🤝️group",1000)+anchor)
texts[CB]=s
if not apply:
    for p in texts: print("edit",p.replace(R,""))
else:
    for p,new in texts.items(): open(p,"w",encoding='utf8').write(new)
    print("applied",len(texts))
