//! ✏️ FEM 3D app — the (sole) `edit` mode. fem3d's manifest declares this mode with the scalar
//! `.mode(MODE_ID, ..)` builder call directly (see `crate::editor::fem3d::create_fem3d_app`) — no
//! `ModeDefinition`/`mode_def` object is built anywhere in the pre-migration `create_fem3d_app`, so this
//! node exists only to give the mode id a home, mirroring the taxonomy's requirement that every mode
//! referenced by id have a corresponding `🎭️modes/<mode>` node.

/// 🏷️ The manifest's sole mode id.
pub const MODE_ID: &str = "edit";
