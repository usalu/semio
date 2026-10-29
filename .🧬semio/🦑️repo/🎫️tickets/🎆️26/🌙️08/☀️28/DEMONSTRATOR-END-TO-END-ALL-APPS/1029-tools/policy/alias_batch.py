import sys,os,re,json,subprocess
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import edit
from edit import *
import owners as OW
FW="🧰️framework/🛍️products/💻️os/🔨️modules/"
ST=FW+"🏪️store/"
PL=FW+"🔌️plugin/"
def rd(p): return edit._read(os.path.join(R,p))
def wr(p,t): edit._write(os.path.join(R,p),t)
def rg_files(pat,roots=("🧰️framework","✏️s","🌎️hub")):
    r=subprocess.run(["rg","-l","--glob","*.rs","--glob","!.🧬semio/**",pat,*roots],cwd=R,capture_output=True,text=True)
    return [x for x in r.stdout.split("\n") if x]
deps_plan=[]
# cycle check
def deps_of(n):
    t=open(os.path.join(IX[n],"Cargo.toml"),encoding="utf-8").read()
    names=set(re.findall(r'^([A-Za-z0-9_\-]+)\s*=\s*\{',t,re.M))|set(re.findall(r'package\s*=\s*"([^"]+)"',t))
    return {d for d in names if d in IX}
_reach={}
def reach(n):
    if n in _reach: return _reach[n]
    seen=set(); st=[n]
    while st:
        x=st.pop()
        for d in deps_of(x):
            if d not in seen: seen.add(d); st.append(d)
    _reach[n]=seen; return seen
def need(consumer,target):
    if consumer==target: return
    assert consumer not in reach(target),("CYCLE",consumer,target)
    r=add_dep(consumer,target); print("dep",consumer[-45:],"<-",target[-40:],r)
# ---- A1
ER="semio-framework-os-store-engine-replay"
files=rg_files(r'\boriginal_replay_publication\b')
for f in files:
    if f==ST+"🦀️.rs": continue
    subre(f,r'(?:store::(?:os_store::)?|crate::)original_replay_publication::','semio_framework_os_store_engine_replay::')
    assert "original_replay_publication" not in rd(f),f
sub(ST+"🦀️.rs","pub use semio_framework_os_store_engine_replay as original_replay_publication;\n","")
for c in ["semio-framework-plugin-instance-stores","semio-framework-plugin-instance-history","semio-framework-plugin-job","semio-framework-plugin-retained-command","semio-framework-plugin-time-travel"]: need(c,ER)
# ---- A2 (item 14)
RO="semio_framework_os_store_replay_operation"
sub(ST+"🔁️replay/🦀️.rs","pub use semio_framework_os_store_replay_operation as replay_preparation;","use semio_framework_os_store_replay_operation as replay_preparation;")
p=ST+"⚙️engine/📥️lifted/🏪️store/🦀️.rs"
sub(p,"use semio_framework_os_store_replay::replay_preparation;\nuse replay_preparation::ArtifactReplayPreparationProgress;","use semio_framework_os_store_replay_operation::ArtifactReplayPreparationProgress;")
sub(p,"semio_framework_os_store_replay::replay_preparation::{","semio_framework_os_store_replay_operation::{")
sub(p,"replay_preparation::native::history::OriginalHistorySource::admit","semio_framework_os_store_replay_operation::native::history::OriginalHistorySource::admit")
assert not re.search(r'(?<![A-Za-z_])replay_preparation::',rd(p).replace("owners.replay_preparation","")),"engine leftovers"
p=ST+"🕰️history/🎮️command/🦀️.rs"
sub(p,"use semio_framework_os_store_replay::replay_preparation;\n","")
sub(p,"type Source<M>=replay_preparation::native::history::OriginalHistorySource<Edit<M>>;","type Source<M>=semio_framework_os_store_replay_operation::native::history::OriginalHistorySource<Edit<M>>;")
for f in [ST+"🕰️history/🏁️replay/📚️metadata/🦀️.rs",ST+"🏗️initialization/🔁️replay/🧬️native/🦀️.rs",ST+"🔗️read/📖️derived-snapshots-replays-and-history-read-plan/🦀️.rs",ST+"🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🦀️.rs",PL+"🫧️transient/🧵️publication/🧬️native/🦀️.rs",PL+"🫧️transient/🧵️publication/🧬️native/🧪️tests/🦀️.rs"]:
    subre(f,r'semio_framework_os_store_replay::replay_preparation::','semio_framework_os_store_replay_operation::')
for c in ["semio-framework-os-store-engine-command","semio-framework-os-store-engine-replay","semio-framework-os-store-initialization-replay","semio-framework-plugin-ownership"]: need(c,"semio-framework-os-store-replay-operation")
# ---- A3 staged_form
SF="semio_framework_plugin_decl_staged_form"
a3=set()
for f in rg_files(r'app::staged_form'):
    s=rd(f)
    s=s.replace("semio_framework_plugin::{action_staging::*,app::staged_form::*};","semio_framework_plugin::action_staging::*;use "+SF+"::*;")
    s=s.replace("semio_framework_plugin::app::staged_form::",SF+"::")
    assert "staged_form::" not in s.replace(SF+"::",""),f
    wr(f,s)
a3o=open(os.path.join(R,C,"🗑️generated/1029/policy-a3owners.txt"),encoding="utf-8").read()
a3=set(re.findall(r"'(semio-[^']+)'",a3o))
print("A3 crates",sorted(a3))
for c in sorted(a3): need(c,"semio-framework-plugin-decl-staged-form")
sub(PL+"🪝️mounts/📜️wp2-declaration/🦀️.rs","    pub use semio_framework_plugin_decl_staged_form as staged_form;\n","")
# ---- A4-13
SP="semio_framework_value_snapshot_projection::snapshot_projection::post"
sub(ST+"🎮️one-item/🧬️semantic/🦀️.rs","pub use semio_framework_value_snapshot_projection::snapshot_projection::post::{","use semio_framework_value_snapshot_projection::snapshot_projection::post::{")
sub(ST+"🎮️one-item/🦀️.rs","PagedOneItemEdit,PagedOneItemEditStep,OriginalPostRead,PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost}","PagedOneItemEdit,PagedOneItemEditStep}")
sub(ST+"📬️paged-one-item/🦀️.rs","use semio_framework_os_store_one_item::{OriginalPostRead,PagedOneItemPostMode};","use "+SP+"::{OriginalPostRead,PagedOneItemPostMode};")
for f in [ST+"📬️paged-one-item/🧪️tests/🔗️source-custody/🦀️.rs",ST+"📬️paged-one-item/🧬️mutation-apply/🦀️.rs"]:
    sub(f,"use semio_framework_os_store_one_item::{PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};","use "+SP+"::{PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};")
subre(ST+"📬️paged-one-item/🧬️mutation-apply/🧪️tests/🦀️.rs",r'semio_framework_os_store_one_item::(PagedOneItemPost(?:Mode|Read)?)\b',SP+r'::\1')
sub(ST+"🔁️replay/🎮️operation/🧬️native/🔌️drivers/🦀️.rs","use semio_framework_os_store_one_item::{PagedOneItemEdit,PagedOneItemEditStep,PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};","use semio_framework_os_store_one_item::{PagedOneItemEdit,PagedOneItemEditStep};use "+SP+"::{PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};")
sub(ST+"🔁️replay/🎮️operation/🧬️native/🦀️.rs","use semio_framework_os_store_one_item::{OriginalPostRead,PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};","use "+SP+"::{OriginalPostRead,PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};")
# ---- A4-18
sub(ST+"🦀️.rs","pub use semio_framework_os_store_engine_replay::{OriginalCheckpointEditOrder,ArtifactOriginalVcsFailure,original_replay_metadata};\n","")
sub(ST+"🕰️history/🎮️command/🧪️tests/🦀️.rs","use store::os_store::ArtifactOriginalVcsFailure;","use semio_framework_os_store_engine_replay::ArtifactOriginalVcsFailure;")
subre(ST+"🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🧪️tests/🦀️.rs",r'store::os_store::original_replay_metadata::','semio_framework_os_store_engine_replay::original_replay_metadata::')
# ---- A4-23/24/25
sub(PL+"🛠️tool-machine/🦀️.rs","pub use semio_framework_plugin_tool_run_operations::OriginalToolOverlayFold;\n","")
sub(PL+"🦀️.rs","pub use semio_framework_plugin_instance_construction::OriginalVcsApp;\n","")
sub(PL+"🦀️.rs","pub use semio_framework_plugin_instance_initialization::registration::OriginalSingleWindowRegistration;\n","")
sub("🌎️hub/🧩️compositions/🔋️energy/🦀️.rs","use semio_framework_plugin::{EditorApp, ExecutionMode, NeedsLabel, Plugin, PluginApp, PluginAssemblyError, PluginBuilder, OriginalVcsApp, ViewerApp};","use semio_framework_plugin::{EditorApp, ExecutionMode, NeedsLabel, Plugin, PluginApp, PluginAssemblyError, PluginBuilder, ViewerApp};\nuse semio_framework_plugin_instance_construction::OriginalVcsApp;")
need("semio-hub-energy","semio-framework-plugin-instance-construction")
E="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/"
for part,crate in [("✏️editor","semio-s-artifact-energy-model-editor"),("👁️viewer","semio-s-artifact-energy-model-viewer")]:
    subre(E+part+"/🌱️initialization/🦀️.rs",r'semio_framework_plugin::OriginalSingleWindowRegistration<','semio_framework_plugin_instance_initialization::registration::OriginalSingleWindowRegistration<')
    need(crate,"semio-framework-plugin-instance-initialization")
(None if os.environ.get("DRY") else flush())
print("DONE")
