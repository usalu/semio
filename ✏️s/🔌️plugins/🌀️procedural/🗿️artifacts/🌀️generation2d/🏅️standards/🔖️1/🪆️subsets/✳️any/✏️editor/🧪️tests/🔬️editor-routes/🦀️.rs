//! 🧪️ Editor route laws: the retained command, flow-eval, contributions and store-preparation routes, every command, mode window and panel driven through the registered editor app.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "../🔬️app-fixture/🦀️.rs"]
mod app_fixture;
#[path = "../🔬️unit/🦀️.rs"]
mod editor_unit_tests;
#[path = "../🪟️generation2d-window-camera-ownership/🦀️.rs"]
mod editor_window_camera_ownership;
#[path = "../../🎮️commands/➕️add-generation/🧪️tests/🔬️unit/🦀️.rs"]
mod add_generation_tests;
#[path = "../../🎮️commands/👁️set-show-mode/🧪️tests/🔬️unit/🦀️.rs"]
mod set_show_mode_tests;
#[path = "../../🎮️commands/📤️set-eval-outputs/🧪️tests/🔬️unit/🦀️.rs"]
mod set_eval_outputs_tests;
#[path = "../../🎮️commands/🗺️reorganize/🧪️tests/🔬️unit/🦀️.rs"]
mod reorganize_tests;
#[path = "../../🎮️commands/🧩️add-widget/🧪️tests/🔬️unit/🦀️.rs"]
mod add_widget_tests;
#[path = "../../🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs"]
mod node_graph_edit_tests;
#[path = "../../📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
mod inspection_panel_tests;
#[path = "../../📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
mod document_panel_tests;
#[path = "../../📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
mod catalogue_panel_tests;
#[path = "../../🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod edit_preview_window_tests;
#[path = "../../🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🔬️unit/🦀️.rs"]
mod flow_window_tests;
#[path = "../../🎭️modes/🧬️generate/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod generate_preview_window_tests;
#[path = "../../🎭️modes/🧬️generate/🪟️windows/📝️form/🧪️tests/🔬️unit/🦀️.rs"]
mod form_window_tests;
#[path = "../../🎭️modes/🧬️generate/🪟️windows/🗂️generations/🧪️tests/🔬️unit/🦀️.rs"]
mod generations_window_tests;
#[path = "../../../🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs"]
mod preview_eval_tests;
