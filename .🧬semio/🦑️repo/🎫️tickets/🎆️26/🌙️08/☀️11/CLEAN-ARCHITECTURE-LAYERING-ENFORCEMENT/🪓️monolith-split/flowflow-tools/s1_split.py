"""🪚 Step 1: cut the monolith files along the model / document line. Operates on the work tree only (ffed.WORK)."""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from ffed import *

ART="🧬️schema/📸️snapshot/🦀️.rs"
HOST="🧬️schema/📸️snapshot/🧾️host/🦀️.rs"
TEXT="🚪️io/📝️text/📸️snapshot/🦀️.rs"
THOST="🚪️io/📝️text/📸️snapshot/🧾️host/🦀️.rs"
RET="🧵️retained/🦀️.rs"
FLD="🧵️retained/🧬️fields/🦀️.rs"
VCS="🌿️vcs/🦀️.rs"
OWNED="🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs"

def artifact():
    t=read(ART)
    t,blockA=cut(t,"/// 🧩️ Flow-host retained fixture encoded through the first-party value contract.","fn widget_chrome(widget: &Widget) -> NodeChrome {",include_end=False)
    t,blockB=cut(t,"pub fn tree_from_host_snapshot(host_snapshot: &FlowHostSnapshot,","impl Default for FlowArtifact {",include_end=False)
    t,blockC=cut(t,"//#region 🔖️KeyboardTraversal","//#endregion 🔖️KeyboardTraversal\n\n")
    t=replace(t,"fn widget_chrome(widget: &Widget) -> NodeChrome {","pub fn widget_chrome(widget: &Widget) -> NodeChrome {")
    layout_entry='''/// 📍️ One layout assignment; absent or null layout removes the existing entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowLayoutEntry {
    pub id: String,
    #[dsl(block)]
    pub layout: Option<WidgetLayout>,
}

'''
    components='''/// 📸️ The owned component record of a host snapshot: schema, camera, widgets, synapses and layout exactly as the snapshot owner holds them.
/// The typed retirement owner carries this record, so the model never names the snapshot owner or its mutation catalogue.
pub struct FlowHostComponents {
    pub schema: String,
    pub camera: CameraJson,
    pub widgets: Vec<Widget>,
    pub synapses: Vec<SynapseSpec>,
    pub layout: OrderedMap<WidgetLayout>,
}

'''
    anchor="/// 🧾️ Flow document encoded through the first-party value contract.\n"
    one(t,anchor)
    t=t.replace(anchor,layout_entry+anchor)
    anchor2="fn widget_chrome" # reinserted below via pub fn
    collection='''//#region 🔖️CollectionIdentity
impl Identified<String> for Widget {
    fn id(&self) -> &String {
        match self {
            Widget::Neuron { id, .. }
            | Widget::InputSlider { id, .. }
            | Widget::InputNote { id, .. }
            | Widget::InputImage { id, .. }
            | Widget::Variable { id, .. }
            | Widget::OutputPreview { id, .. }
            | Widget::OutputAction { id, .. }
            | Widget::OutputExport { id, .. }
            | Widget::Cluster { id, .. } => id,
        }
    }
}

/// 🩹️ Whole-value replacement patch — flow widgets are heterogeneous enum variants, so a granular
/// per-field patch buys nothing; `Patch { patch: Widget }` LWW-replaces.
impl Patchable<Widget> for Widget {
    fn apply_patch(&mut self, patch: &Widget) {
        *self = patch.clone();
    }
}

impl Identified<String> for SynapseSpec {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Patchable<SynapseSpec> for SynapseSpec {
    fn apply_patch(&mut self, patch: &SynapseSpec) {
        *self = patch.clone();
    }
}
//#endregion 🔖️CollectionIdentity

'''
    # components + collection identity go right before widget_chrome
    one(t,"pub fn widget_chrome(widget: &Widget) -> NodeChrome {")
    t=t.replace("pub fn widget_chrome(widget: &Widget) -> NodeChrome {",components+collection+"pub fn widget_chrome(widget: &Widget) -> NodeChrome {")
    t=replace(t,"use crate::widget_id_for;\n","use crate::os_spr::{Identified, Patchable};\nuse crate::widget_id_for;\n")
    write(ART,t)
    return blockA,blockB,blockC

def host(blockA,blockB,blockC):
    head='''//! 🧾️ Flow host snapshot: the persisted document owner that composes the model's widgets, synapses, layout and camera.

use crate::neural::{Atom, Dictionary, Neuron, OperatorInfo, Synapse, Tree, Value as NeuralValue, CLUSTER_KIND};
use crate::retained::{FlowOwner, FlowRetirement};
use crate::{widget_chrome, widget_id_for, CameraJson, FlowArtifact, FlowChannelRef, FlowHostComponents, FlowNodeGui, FlowPreviewGui, FlowUi, OrderedMap, OrderedSet, SynapseSpec, Widget, WidgetLayout, FLOW_SLIDER_MAX, FLOW_SLIDER_MIN, FLOW_SLIDER_STEP};
use protocol::causal::transition::HistoryFoldIndex;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️HostSnapshot
'''
    retire='''//#endregion 🔖️HostSnapshot

//#region 🧊️HostSnapshotRetirement
semio_framework_value::artifact_retire_struct!(FlowHostSnapshot {schema,camera,widgets,synapses,layout});

impl From<FlowHostSnapshot> for FlowHostComponents {
    fn from(snapshot: FlowHostSnapshot) -> Self {
        let FlowHostSnapshot { schema, camera, widgets, synapses, layout } = snapshot;
        Self { schema, camera, widgets, synapses, layout }
    }
}

impl From<FlowHostComponents> for FlowHostSnapshot {
    fn from(components: FlowHostComponents) -> Self {
        let FlowHostComponents { schema, camera, widgets, synapses, layout } = components;
        Self { schema, camera, widgets, synapses, layout }
    }
}

impl FlowHostSnapshot {
    /// 🧊️ Explicit cold-only disposal of a detached fixture.
    pub fn retire_cold(self) {
        FlowRetirement::from_owner(FlowOwner::HostSnapshot(self.into())).retire_cold();
    }

    /// 🧊️ Installs a whole widget roster AND retires the one it displaces. A bare
    /// `fixture.widgets = vec![…]` drops every displaced `Widget` — each of which owns a
    /// `Dictionary`/`OrderedSet`/`Tree` that refuses a bare drop — so the assignment panics with
    /// `final Dictionary ownership must be explicitly retired or owned by a cold boundary` at the
    /// END of the caller, naming nothing that points back at the assignment. This is the paid form,
    /// the twin of the `std::mem::replace(&mut self.fixture, fixture).retire_cold()` the flow host's
    /// own `apply_fixture` already pays (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn replace_widgets(&mut self, widgets: Vec<Widget>) {
        FlowRetirement::from_owner(FlowOwner::Widgets(std::mem::replace(&mut self.widgets, widgets))).retire_cold();
    }
}
//#endregion 🧊️HostSnapshotRetirement
'''
    write(HOST,head+blockA.rstrip("\n")+"\n\n"+blockB.rstrip("\n")+"\n\n"+blockC.rstrip("\n")+"\n"+retire)

def text():
    t=read(TEXT)
    t=replace(t,"use super::super::super::*;\n","use std::collections::BTreeMap;\n\nuse crate::neural::{Atom, Dictionary, Neuron, Synapse, Tree, Value as NeuralValue};\nuse crate::{CameraJson, OrderedSet, SynapseSpec, Widget, WidgetLayout};\n")
    t,dsl_impls=cut(t,"/// 📜️ Handcrafted ArtifactDsl (P6): derive no longer emits ArtifactDsl/ArtifactPack.","\n/// 🌱️ `Value`/`Atom`/`Dictionary`/`Tree`/`Neuron`/`Synapse` are all defined in `neural_engine`",include_end=False)
    t,mirror=cut(t,"/// 📄️ Local mirror of `FlowHostSnapshot` — see this region's opening doc comment","//#endregion 🔖️Dsl",include_end=False)
    t,fieldimpls=cut(t,"impl semio_framework_dsl_record::BorrowedDslField for FlowHostSnapshot {","            other => Err(format!(\"expected Flow fixture record, found {other:?}\")),\n        }\n    }\n}\n")
    t=t.replace("pub(crate) ","pub ")
    write(TEXT,t.rstrip("\n")+"\n")
    thost='''//! 🚪️ Native text representation of the Flow host snapshot over the model's component codecs.

use std::collections::BTreeMap;

use crate::component_dsl::{synapse_from_dsl, synapse_to_dsl, widget_dsl_to_widget, widget_to_widget_dsl, SynapseDsl, WidgetDsl};
use crate::{CameraJson, FlowHostSnapshot, WidgetLayout};

'''
    return thost+dsl_impls.rstrip("\n")+"\n\n"+mirror.rstrip("\n")+"\n\n"+fieldimpls.rstrip("\n")+"\n"

def retained():
    t=read(RET)
    t=replace(t,"use crate::{neural, FlowArtifact, FlowHostSnapshot, FlowGui, FlowLayoutEntry,","use crate::{neural, FlowArtifact, FlowHostComponents, FlowGui, FlowLayoutEntry,")
    t,copyblock=cut(t,"#[path = \"📑️copy/🦀️.rs\"]\npub mod copy;","FlowCopyAllocations, FlowHostSnapshotCopy, FlowSynapseCopy, FlowWidgetCopy};\n\n")
    t=replace(t,"HostSnapshot(FlowHostSnapshot),","HostSnapshot(FlowHostComponents),")
    t=replace(t,"Chrome(NodeChrome),Mutation(crate::FlowMutation),","Chrome(NodeChrome),")
    t=replace(t,"Previews,Layout,Chrome,Mutation);","Previews,Layout,Chrome);")
    t,snapimpl=cut(t,"impl FlowHostSnapshot {\n    /// 🧊️ Explicit cold-only disposal of a detached fixture.","        FlowRetirement::from_owner(FlowOwner::Widgets(std::mem::replace(&mut self.widgets, widgets))).retire_cold();\n    }\n}\n\n")
    t,tests=cut(t,"//#region 🧪️RetirementLaws","//#endregion 🧪️RetirementLaws\n")
    write(RET,t.rstrip("\n")+"\n")
    return copyblock,tests

def fields():
    t=read(FLD)
    t=replace(t,"use crate::{FlowArtifact,FlowChannelRef,FlowGui,FlowHostSnapshot,FlowLayoutEntry,","use crate::{FlowArtifact,FlowChannelRef,FlowGui,FlowHostComponents,FlowLayoutEntry,")
    t=replace(t,"semio_framework_value::artifact_retire_struct!(FlowHostSnapshot {schema,camera,widgets,synapses,layout});","semio_framework_value::artifact_retire_struct!(FlowHostComponents {schema,camera,widgets,synapses,layout});")
    i=one(t,"macro_rules! owned_record {")
    owned_text=t[i:]
    t=t[:i]
    write(FLD,t.rstrip("\n")+"\n")
    return owned_text

def vcs():
    t=read(VCS)
    t,ident=cut(t,"impl Identified<String> for Widget {","impl Patchable<SynapseSpec> for SynapseSpec {\n    fn apply_patch(&mut self, patch: &SynapseSpec) {\n        *self = patch.clone();\n    }\n}\n\n")
    t,entry=cut(t,"/// 📍️ One layout assignment; absent or null layout removes the existing entry.","    pub layout: Option<WidgetLayout>,\n}\n\n")
    t=replace(t,"use crate::artifact::*;\n","use crate::artifact::*;\nuse crate::snapshot::*;\n")
    t=replace(t,"MutationApplyResult, MutationDiff, Patchable};","MutationApplyResult, MutationDiff};")
    t=replace(t,"#[cfg(test)]\n#[path = \"🧬️schema/🧹️retirement/🦀️.rs\"]\nmod flow_mutation_retirement;\n","#[path = \"🧬️schema/🧹️retirement/🦀️.rs\"]\nmod flow_mutation_retirement;\n")
    write(VCS,t)

if __name__=="__main__":
    A,B,C=artifact()
    host(A,B,C)
    thost=text()
    write(THOST,thost)
    copyblock,tests=retained()
    owned=fields()
    vcs()
    open(SCR+"s1.copyblock.txt","w").write(copyblock)
    open(SCR+"s1.retained-tests.txt","w").write(tests)
    open(SCR+"s1.owned.txt","w").write(owned)
    print("ok")
