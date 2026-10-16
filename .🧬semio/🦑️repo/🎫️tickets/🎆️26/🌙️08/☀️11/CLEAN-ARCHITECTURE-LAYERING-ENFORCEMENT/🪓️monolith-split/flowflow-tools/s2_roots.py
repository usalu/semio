"""🌱 Step 2: crate roots, owned-retirement file, text/io aggregators. Work tree only."""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from ffed import *
owned=open(SCR+"s1.owned.txt",encoding="utf-8").read()
write("🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs","//! 🧹️ Actual Flow mutation ownership uses the canonical typed retirement cursor.\n\nuse semio_framework_value::retirement::{RetireOwned,RetirementCursor,deferred,deferred_birth_bytes_for,sequence,sequence_birth_bytes};\n\n"+owned)
t=read("🚪️io/📝️text/📸️snapshot/🦀️.rs")
t=replace(t,"//! 🚪️ Native artifact representation codecs.\nuse std::collections::BTreeMap;","//! 🚪️ Native text representation of the Flow document components: value, tree, synapse and widget DSL twins with their field codecs.\n\nuse std::collections::BTreeMap;")
write("🚪️io/📝️text/📸️snapshot/🦀️.rs",t)
write("🚪️io/📝️text/🦀️.rs",'''//! 🚪️ Native representation assembly.
#[path = "📸️snapshot/🧾️host/🦀️.rs"]
pub mod snapshot;
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
''')
write("🧱️model/🦀️.rs",'''//! 🧱️ Flow document model: widgets, synapses, GUI projections, typed retirement and the component text codecs every Flow document representation shares.
extern crate semio_framework_os_kernel as dsl;
pub use neural_engine as neural;
pub use protocol::value::ordered::{OrderedMap, OrderedSet};
pub use semio_framework_os_kernel::os_dsl;
pub use semio_framework_os_kernel::os_pack;
pub use semio_framework_os_kernel::os_spr;
pub use semio_framework_os_kernel::os_store;
#[path = "../🧬️schema/📸️snapshot/🦀️.rs"]
pub mod artifact;
pub use artifact::*;
#[path = "../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
pub mod component_dsl;
#[path = "../🎚️parameter/📨️intent/🦀️.rs"]
pub mod graph_parameter;
#[path = "../🧵️retained/🦀️.rs"]
pub mod retained;

pub fn widget_id_for(widget: &Widget) -> &str {
    match widget {
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
''')
write("🧾️document/🦀️.rs",'''//! 🧾️ Flow document: the persisted host snapshot, its mutation catalogue and every native representation codec (text, binary pack, SQLite).
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
pub use semio_framework_artifact_flow_flow_model::*;
use semio_framework_ui_locale::LocalizedLabel;
#[path = "../🧬️schema/📸️snapshot/🧾️host/🦀️.rs"]
pub mod snapshot;
pub use snapshot::*;
#[path = "../🌿️vcs/🦀️.rs"]
pub mod vcs;
pub use vcs::*;
''')
print("ok")
