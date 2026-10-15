//! 🎮️ BIM editor commands: every command module of the surface, each mounted from its own folder.

#[path = "."]
pub mod create_entity {
    #[path = "🏗️create-entity/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod create_view {
    #[path = "🔭️create-view/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod delete_selection {
    #[path = "🗑️delete-selection/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod rename_entity {
    #[path = "🏷️rename-entity/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod set_field {
    #[path = "🩹️set-field/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod set_view {
    #[path = "🪟️set-view/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod edit_family {
    #[path = "🧬️edit-family/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod edit_schedule {
    #[path = "📋️edit-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod export_schedule_csv {
    #[path = "📊️export-schedule-csv/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod analyse_model {
    #[path = "🔎️analyse-model/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod export_model {
    #[path = "📤️export-model/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod export_sheets {
    #[path = "📄️export-sheets/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod set_camera {
    #[path = "🎥️set-camera/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod canvas_pointer_down {
    #[path = "🖱️canvas-pointer-down/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod canvas_pointer_move {
    #[path = "↔️canvas-pointer-move/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod canvas_pointer_up {
    #[path = "⬆️canvas-pointer-up/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod canvas_double_click {
    #[path = "👆️canvas-double-click/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod canvas_commit_draft {
    #[path = "✅️canvas-commit-draft/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod canvas_escape {
    #[path = "🚪️canvas-escape/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod world_pointer_down {
    #[path = "🌍️world-pointer-down/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod world_pointer_move {
    #[path = "🌐️world-pointer-move/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod arm_utility {
    #[path = "🛠️arm-utility/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod flip_walls {
    #[path = "🔃️flip-walls/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod attach_walls {
    #[path = "🔗️attach-walls/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod move_storey {
    #[path = "🪜️move-storey/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod split_wall {
    #[path = "✂️split-wall/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod set_property {
    #[path = "🧾️set-property/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod remove_property {
    #[path = "🗃️remove-property/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod select_findings {
    #[path = "🎯️select-findings/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod coordinate {
    #[path = "🤝️coordinate/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod set_classification {
    #[path = "🗂️set-classification/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod remove_classification {
    #[path = "🗄️remove-classification/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod apply_template {
    #[path = "🧰️apply-template/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod conditions {
    #[path = "🌡️conditions/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod edit_template {
    #[path = "🧮️edit-template/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod edit_classification {
    #[path = "📚️edit-classification/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod search_classification {
    #[path = "🔍️search-classification/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod cursor_keys {
    #[path = "🧭️cursor-keys/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod place_elements {
    #[path = "📍️place-elements/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod place_grid_columns {
    #[path = "🏛️place-grid-columns/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod engagement_input {
    #[path = "⌨️engagement-input/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod browse_families {
    #[path = "🪑️browse-families/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod set_override {
    #[path = "🎚️set-override/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod gesture_keys {
    #[path = "🔑️gesture-keys/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod engagement_submit {
    #[path = "📨️engagement-submit/🦀️.rs"]
    mod component;
    pub use component::*;
}
