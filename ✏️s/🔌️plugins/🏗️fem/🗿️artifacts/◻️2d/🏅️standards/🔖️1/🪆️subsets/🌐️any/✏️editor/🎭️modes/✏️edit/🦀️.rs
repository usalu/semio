//! ✏️ Fem2d play app — the `edit` mode: a home for the mode id. Fem2d's manifest only calls a scalar
//! `.mode("edit", ...)` inline (no `ModeDefinition`/`mode_def` object is built anywhere in the
//! pre-migration code), so this file has nothing beyond the id constant.

pub const MODE_ID: &str = "edit";
