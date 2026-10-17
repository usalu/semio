#!/usr/bin/env python3
# 🧱️ One-shot: moves the shared editor vocabulary (ids, action values, interaction ids, store capacities) into 🧱️base and the FlowHost bridging into 🖥️host, then re-points every caller. Reads orig/editor-root.rs, writes the real tree.
import re,os,sys,glob
ROOT="/Users/ueli/Documents/semio"
A=ROOT+"/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow"
S=A+"/🏅️standards/🔖️1/🪆️subsets/✳️any"
ED=S+"/✏️editor"
TOOLS=os.path.dirname(os.path.abspath(__file__))
t=open(TOOLS+"/orig/editor-root.rs",encoding="utf8").read()

def cut(text,start,end,incl_end=True,expect=1):
    assert text.count(start)==expect,(start,text.count(start))
    i=text.index(start)
    j=text.index(end,i)
    if incl_end: j+=len(end)
    return text[:i]+text[j:],text[i:j]

# 1 app id
t,_=cut(t,'pub const FLOW_PLAY_APP_ID: &str = "flow-play";\n','\n')
# 2 action-value block
t,b1=cut(t,"/// 🎯️ An `ActionDescriptor` addressed at this app","    Ok(node)\n}\n\n")
# 3 interaction region
t,b2=cut(t,"//#region 🔖️Interaction\n","//#endregion 🔖️Interaction\n\n")
# 4 store capacity
t,b3=cut(t,"//#region 📏️StoreCapacity\n","//#endregion 📏️StoreCapacity\n\n")
# 5 host + selection
t,b4=cut(t,"//#region 🔖️Host\n","//#endregion 🔖️Selection\n\n")
# 6 value_fault
t,b5=cut(t,"/// 🧯️ Surfaces a retained-ownership refusal as an app fault carrying its message.\npub(crate) fn value_fault(","\n\n")
# 7 raw bytes
t,_=cut(t,"pub(crate) const FLOW_GRAPH_OPERATION_RAW_BYTES: usize = 16_384;\n","\n")
open(TOOLS+"/stage-editor-root.rs","w",encoding="utf8").write(t)

b1=b1.rstrip("\n")+"\n"
base=f'''//! 🧱️ Flow editor core: the app and window ids, the action-value factory, the "graph" interaction vocabulary and the store capacities every taxonomy node of the editor shares.

//#region 🔖️Constants
pub const FLOW_PLAY_APP_ID: &str = "flow-play";
pub const FLOW_PLAY_WINDOW_MAIN: &str = "flow-main";
pub const FLOW_PLAY_WINDOW_GENERATIONS: &str = "flow-generations";
pub const FLOW_PLAY_WINDOW_GENERATE_FORM: &str = "flow-generate-form";
pub const FLOW_PLAY_WINDOW_GENERATE_PREVIEW: &str = "flow-generate-preview";

{b1}//#endregion 🔖️Constants

{b2.rstrip(chr(10))}

//#region 📏️StoreCapacity
pub const FLOW_STORE_MAX_SCENE_ITEMS: usize = 256;
pub const FLOW_STORE_MAX_TEXT_BYTES: usize = 16_384;
pub const FLOW_STORE_MAX_MUTATION_ITEMS: usize = 256;
pub const FLOW_GRAPH_OPERATION_RAW_BYTES: usize = 16_384;
//#endregion 📏️StoreCapacity
'''
vf=b5.strip("\n").replace("pub(crate) fn value_fault","pub fn value_fault")
b4=b4.replace("pub(crate) fn cold_grant","pub fn cold_grant")
host=f'''//! 🖥️ Flow editor host bridge: rebuilds the stateful `FlowHost` from the document projection, view config and eval session, and maps host results back onto content-child leaves.

use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::{{FlowMutation, FlowSnapshot, FlowWorkingScene}};
use flow::{{flow_host_with_session, FlowEvalSession, FlowHost, FLOW_LOD_MODE_AUTOMATIC}};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_os_infinite_directed_presentation::DagDrawLod;
use semio_framework_plugin::app::ChildEmitPreparation;
use semio_framework_plugin::{{Emit, Fault, NoConfigMutation}};
use semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;

{b4.rstrip(chr(10))}

//#region 🧯️Faults
{vf}
//#endregion 🧯️Faults
'''
os.makedirs(ED+"/🧱️base",exist_ok=True); os.makedirs(ED+"/🖥️host",exist_ok=True)
open(ED+"/🧱️base/🦀️.rs","w",encoding="utf8").write(base)
open(ED+"/🖥️host/🦀️.rs","w",encoding="utf8").write(host)
print("base lines",base.count("\n"),"host lines",host.count("\n"),"root lines",t.count("\n"))
