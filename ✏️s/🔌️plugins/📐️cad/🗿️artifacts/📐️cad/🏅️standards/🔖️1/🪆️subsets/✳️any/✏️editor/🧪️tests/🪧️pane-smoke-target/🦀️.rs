//! 🪧️ Koordinator pane smoke target: only the fixture support and the pane smoke law, built against the public editor API.
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

use semio_plugin::ViewModel;
use semio_s_artifact_cad_cad_commands::node::add_node;
use semio_s_artifact_cad_cad_editor::*;
use semio_s_artifact_cad_cad_kit::editor::cad::terminology::cad_labels;

#[path = "../🧰️support/🦀️.rs"]
mod context;
use context::*;

#[path = "../🔬️unit/🪧️pane-smoke/🦀️.rs"]
mod pane_smoke;

#[global_allocator]
static COORDINATOR_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
