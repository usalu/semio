//! 👁️ FEM 3D viewer — the (sole) `view` mode. Mirrors fem3d's own editor style: the manifest declares
//! this mode with the scalar `.mode(FEM3D_VIEW_MODE_VIEW, ..)` builder call directly (see
//! `crate::viewer::fem3d::create_fem3d_viewer`) — no `ModeDefinition` object is built anywhere, so this
//! node exists only to give the mode id a home, mirroring the taxonomy's requirement that every mode
//! referenced by id have a corresponding `🎭️modes/<mode>` node.

/// 🏷️ The manifest's sole viewer mode id.
pub const FEM3D_VIEW_MODE_VIEW: &str = "view";
