//! @emoji 🎨 Board WASM build script; codegen: `elements/core/styling/rs/board_vello_build.inc.rs` (board canvas palette from `tokens.json` `colors`).
include!(concat!(
	env!("CARGO_MANIFEST_DIR"),
	"/../../../../core/styling/rs/board_vello_build.inc.rs"
));
