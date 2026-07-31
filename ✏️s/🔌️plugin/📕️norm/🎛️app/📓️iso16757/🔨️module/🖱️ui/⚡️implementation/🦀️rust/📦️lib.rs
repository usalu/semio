//! 🖼️ ISO 16757 app — DocumentApp impl, render, manifest (constitutional: ui).
//!
//! 📭️ Deliberately empty: unlike `note`/`mathematical`, every norm family's `Iso16757PlayApp`
//! `DocumentApp` impl, render/panels, and `create_app()` manifest builder are macro-generated in the
//! shared `s/plugin/norm/plugin/rs` bundle (`define_norm_family_app!`, one invocation per family) —
//! there was never a per-app `PlayApp`/`impl DocumentApp`/`#[cfg(test)]` app-behavior test in this
//! app's original `lib.rs` to redistribute here. This crate exists to keep the constitutional 7-slot
//! layout uniform across all `s/plugin/norm/app/*` crates and to give the bundle a stable dependency
//! target once it is migrated to depend on split per-app crates instead of the old monolith.
