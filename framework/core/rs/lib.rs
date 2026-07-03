//! 🥅 Render-independent framework kernel: declarative {@link UiNode}, {@link Platform}, {@link CommandBus}.

pub mod command_bus;
pub mod layout;
pub mod platform;
pub mod ui;

pub use command_bus::{CommandBus, CommandHandler};
pub use layout::{NamedLayout, TabStackLayout, WindowLayout, WindowMeasure};
pub use platform::{PanelVisibility, Platform, PlatformSpec};
pub use ui::*;
