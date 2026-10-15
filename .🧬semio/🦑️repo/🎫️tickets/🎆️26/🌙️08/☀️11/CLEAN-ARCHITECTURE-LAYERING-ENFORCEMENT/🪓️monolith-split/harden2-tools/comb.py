import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import ho
W=ho.WORK
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/"
ho.prep("gql")
ho.prep("gqlfull")
ho.neutral.run("gqlfs-split",ho.REPO,ho.SPEC["gqlfs"]["root"],W+"/nsbx-gqlfs-split")
core_split=W+"/nsbx-gql-split/"+B+"📦️packages/🦀️rust/🦀️.rs"
fs_split=W+"/nsbx-gqlfs-split/"+B+"🗄️fs-context/🦀️.rs"
core_base=W+"/nsbx-gqlfull-base/"+B+"📦️packages/🦀️rust/🦀️.rs"
t=open(fs_split,encoding="utf-8").read()
if "crate::semio_framework_repo_graphql" not in t:
    open(fs_split,"w",encoding="utf-8").write(t.replace("use semio_framework_repo_graphql::","use crate::semio_framework_repo_graphql::"))
os.makedirs(W+"/comb",exist_ok=True)
open(W+"/comb/split.rs","w",encoding="utf-8").write('#[path = "%s"]\npub mod semio_framework_repo_graphql;\n#[path = "%s"]\npub mod fs_context;\n'%(core_split,fs_split))
open(W+"/comb/base.rs","w",encoding="utf-8").write('#[path = "%s"]\npub mod semio_framework_repo_graphql;\n'%core_base)
fam=r"🔗️graphql/(📦️packages/🦀️rust/(🦀️\.rs|(🔤️grammar|🏗️repo-schema|🎥️context-port|🖼️sources|⚡️execution|🩻️recording-context)/🦀️\.rs)|🗄️fs-context/🦀️\.rs)$"
for cfg in ("native","test"):
    ex=ho.CFG[cfg]
    A=ho.rustc(W+"/comb/base.rs","comb","lib",ex,W+"/out-comb-"+cfg+"-base",fam)[0]
    Bk=ho.rustc(W+"/comb/split.rs","comb","lib",ex,W+"/out-comb-"+cfg+"-split",fam)[0]
    more=Bk-A; less=A-Bk
    print(f"[comb gql+fs {cfg}] base={sum(A.values())} split={sum(Bk.values())} nonext base={sum(ho.nonext(A).values())} split={sum(ho.nonext(Bk).values())} | split-only={sum(more.values())} base-only={sum(less.values())}")
    for k,n in sorted(more.items()): print("  +SPLIT",n,tuple(str(x)[:130] for x in k))
    for k,n in sorted(less.items()): print("  -BASE ",n,tuple(str(x)[:130] for x in k))
