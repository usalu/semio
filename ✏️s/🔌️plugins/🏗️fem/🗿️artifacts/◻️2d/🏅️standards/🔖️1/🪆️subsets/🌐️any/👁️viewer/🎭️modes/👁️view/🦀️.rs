//! 👁️ Fem2d viewer — the `view` mode: a home for the mode id. Fem2d's manifest only calls a scalar
//! `.mode(...)` inline (no `ModeDefinition` object is built anywhere in this plugin's manifest style),
//! so this file has nothing beyond the id constant — mirrors the sibling `✏️editor` mode file's shape.

pub const FEM2D_VIEW_MODE_VIEW: &str = "view";
