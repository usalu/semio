//! 🥅 Render-independent framework kernel: declarative {@link UiNode}, {@link Platform}, {@link CommandBus}.

pub mod command_bus;
pub mod layout;
pub mod mesh;
pub mod platform;
pub mod tools;
pub mod ui;

pub use command_bus::{CommandBus, CommandHandler};
pub use layout::{
    create_default_layout, create_named_layout, create_stack_layout, create_tab_stack_layout,
    create_window_layout, merge_named_layouts, CommandDescriptor, NamedLayout, StyleSpec,
    WindowEngagement, WindowEngagementControl, WindowEngagementInput, WindowEngagementOption,
    WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot,
    WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
    FRAMEWORK_PANEL_TAB_PARAMETERS_ID, FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
};
pub use mesh::{
    mesh_box, mesh_cone, mesh_cylinder, mesh_from_glb, mesh_from_indexed, mesh_from_kind, mesh_ico_sphere,
    mesh_plane, mesh_to_glb, mesh_to_obj, mesh_torus, mesh_uv_sphere, MeshData,
};
pub use platform::{PanelVisibility, Platform, PlatformSpec};
pub use tools::{tool_button, tool_collection, tool_separator, tool_toggle, ToolNode};
pub use ui::*;
