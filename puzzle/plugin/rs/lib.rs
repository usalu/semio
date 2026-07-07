//! 🧩 Puzzle plugin — 2D, 3D, and 5D play apps in one hot-swappable WASM component.

pub mod d2;
pub mod d3;
pub mod d5;

use std::sync::LazyLock;

use semio_framework_plugin::{install_plugin_bundle, PluginBundle};

//#region 🔖Bundle
fn register_puzzle_exports() {
    d2::register_puzzle2d_exports();
    d3::register_puzzle3d_exports();
    d5::register_puzzle5d_exports();
}

fn bundle() -> PluginBundle {
    register_puzzle_exports();
    PluginBundle::new("puzzle", "Puzzle", "0.1.0")
        .register_app(d2::create_puzzle2d_app(), || Box::new(d2::Puzzle2dPlayApp::default()))
        .register_app(d3::create_puzzle3d_app(), || Box::new(d3::Puzzle3dPlayApp::default()))
        .register_app(d5::create_puzzle5d_app(), || Box::new(d5::Puzzle5dPlayApp::default()))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

semio_framework_plugin::plugin_exports!();
//#endregion 🔖Bundle
