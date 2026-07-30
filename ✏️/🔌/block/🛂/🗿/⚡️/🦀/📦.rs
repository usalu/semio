//! 🧱 Block plugin — 2D, 3D, and 5D single-kind-definition editors in one hot-swappable WASM plugin.

fn register_block_exports() {
    block_2d_ui::register_block2d_exports();
    block_3d_ui::register_block3d_exports();
    block_5d_ui::register_block5d_exports();
}

semio_framework_plugin::semio_plugin! {
    id: "block",
    label: "Block",
    version: "0.1.0",
    setup: register_block_exports,
    apps: [
        block_2d_ui::create_block2d_app => block_2d_ui::Block2dPlayApp,
        block_3d_ui::create_block3d_app => block_3d_ui::Block3dPlayApp,
        block_5d_ui::create_block5d_app => block_5d_ui::Block5dPlayApp,
    ]
}
