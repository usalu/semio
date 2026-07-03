//! 🖥️ Plugin-based OS kernel: hot-swappable WASM components, media graph, document VCS.

pub mod host;
pub mod instance;
pub mod media_graph;
pub mod registry;

pub use host::{LoadedPlugin, PluginHost, PluginHotSwapEvent};
pub use instance::{OsAppInstance, OsInstanceState};
pub use media_graph::{MediaGraph, MediaNode, ProgramRegistry};
pub use registry::PluginRegistry;
pub use semio_framework_core::*;
