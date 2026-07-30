//! 🖥️ EN 1995 app — `DocumentApp` facade (constitutional: ui).
//!
//! ⚠️ Deviation from the constitutional-split recipe (validated on the UI-heavy `note` app): the EN
//! family apps' `XPlayApp` struct, `impl DocumentApp`, `render`/`handle_action`, and `create_app()`
//! manifest builder are NOT defined per-app — they are generated once, for all fifteen `norm` family
//! apps together, by the `define_norm_family_app!` macro inside the still-monolithic plugin bundle
//! (`s/plugin/norm/plugin/rs/lib.rs`), which is out of scope for this split (other agents own it
//! concurrently). There is therefore no UI-layer source in `s/plugin/norm/en/1995/rs/lib.rs` to
//! redistribute here. This crate exists only to keep the constitutional 7-slot shape uniform across
//! the whole `norm` plugin and to give a future de-macro-ization of the bundle a ready landing spot —
//! it re-exports the pieces such a `DocumentApp` impl would need.

pub use en1995::Document;
pub use en1995_engine::evaluate;
pub use en1995_op::{En1995Family, Host, Operation};
