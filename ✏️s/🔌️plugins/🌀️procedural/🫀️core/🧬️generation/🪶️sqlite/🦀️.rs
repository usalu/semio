//! 🌀️ Typed relational support for the identical owned procedural host and generation fields.
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_artifact_flow_flow::{FlowHostSnapshot,FlowUi,Widget,NodeChrome,CameraJson,neural::{Dictionary,Value as NeuralValue,Atom,Tree}};
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
use semio_framework_value::DslValue;
use semio_framework_value::Number;
use semio_framework_os_kernel::sqlite_snapshot::SqliteDatabase;
use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl;
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell;
use semio_framework_os_kernel::sqlite_snapshot::artifact::Projection;
use semio_framework_os_kernel::sqlite_snapshot::artifact::FloatColumn;
use semio_framework_os_kernel::sqlite_snapshot::artifact::insert_ieee754;
use semio_framework_os_kernel::sqlite_snapshot::artifact::insert_key_ieee754;
#[path="📤️projection/🦀️.rs"]mod projection;
pub use projection::project;
fn ordinal(n:usize)->Result<i64,ValueError>{i64::try_from(n).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"procedural ordinal overflow"))}
fn columns(table:&str)->&'static[FloatColumn]{match table{"generation_host"=>&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"generation_slider_widget"|"generation_gui_slider"=>&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"generation_host_layout"=>&[FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"generation_neural_value"=>&[FloatColumn::Binary64(4)],"generation_gui"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3)],"generation_gui_node"=>&[FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"generation_gui_preview"=>&[FloatColumn::Binary64(8),FloatColumn::Binary64(9)],"generation_value"=>&[FloatColumn::Binary64(5)],_=>&[]}}

#[path="📥️reconstruction/🦀️.rs"]mod reconstruction;
pub use reconstruction::reconstruct;

#[path="📏️native/🦀️.rs"]mod native;
pub use native::preflight;
