//#region 🔖️Text
/// @emoji 📍️ 1-based line/column position inside DSL or op-log source text. Lives in `dsl_core`
/// (the token-native DSL engine's foundation crate, which sits below `vcs`); re-exported here so
/// every existing `store::TextSpan`/`store::TextError` import across the workspace keeps compiling.
pub use dsl_core::{TextError, TextSpan};

/// @emoji 📜️ Handcrafted textual representation of a document projection, implemented once per
/// technology next to its `Projection` type. LAW: `P::parse_dsl(&projection.print_dsl())` recovers
/// an equal projection — canonical `print_dsl` output is always a `parse_dsl` fixpoint; hand-written
/// text may normalize (whitespace, ordering) before reaching that fixpoint.
pub trait DocumentDsl: Sized {
    /// @emoji 🏷️ Canonical file extension WITHOUT the leading dot, e.g. `"note"`, `"puzzle3d"`.
    const EXTENSION: &'static str;
    fn parse_dsl(text: &str) -> Result<Self, TextError>;
    fn print_dsl(&self) -> String;
}

// 🎞️ CW3 kernel cut-over: `OpText` moved (method order flipped, behavior unchanged) to
// `protocol_command`, re-exported via the `🚧️TEMPORARY protocol shim` near the top of this file.

//#endregion 🔖️Text
