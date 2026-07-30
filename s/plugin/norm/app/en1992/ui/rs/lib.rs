//! 🖥️ EN 1992 design of concrete structures — `DocumentApp` impl, render, manifest (constitutional: ui).
//!
//! 🏗️ Unlike most constitutional apps, this slot is intentionally empty: the original
//! `s/plugin/norm/en/1992/rs/lib.rs` monolith never defined an `En1992PlayApp`/`DocumentApp`/`create_app`
//! of its own — every one of the norm plugin's 15 families shares ONE generic `impl DocumentApp`
//! (`define_norm_family_app!` in `s/plugin/norm/plugin/rs`, out of scope for this split: the bundle
//! is owned by other concurrent work). See `semio-s-app-norm-en1990-ui`'s doc comment for the full
//! rationale — it applies identically here.
