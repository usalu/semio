//! 🖥️ EN 1990 basis of structural design — `DocumentApp` impl, render, manifest (constitutional: ui).
//!
//! 🏗️ Unlike most constitutional apps, this slot is intentionally empty: the original
//! `s/plugin/norm/en/1990/rs/lib.rs` monolith never defined an `En1990PlayApp`/`DocumentApp`/`create_app`
//! of its own — every one of the norm plugin's 15 families shares ONE generic `impl DocumentApp`
//! (`define_norm_family_app!` in `s/plugin/norm/plugin/rs`, out of scope for this split: the bundle
//! is owned by other concurrent work). That macro is generic over any `{Document, Operation,
//! NormFamily}` triple, so once the bundle is rewired to the constitutional crates it can bind
//! directly to `en1990::Document` / `en1990_op::Operation` / `en1990_engine::En1990Family` without
//! needing anything from this crate. This crate exists to keep the slot's shape uniform across all
//! seven constitutional crates and to give that future rewiring a stable landing spot.
