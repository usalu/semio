//! 🥅 Render-independent framework kernel: declarative {@link UiNode}, {@link Platform}, {@link CommandBus}.

pub mod command_bus;
pub mod layout;
pub mod platform;
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
pub use platform::{PanelVisibility, Platform, PlatformSpec};
pub use ui::*;
