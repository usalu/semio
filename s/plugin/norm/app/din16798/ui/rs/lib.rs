//! 🧩 DIN EN 16798 app — DocumentApp impl, render, manifest (constitutional: ui).
//!
//! 📌 Deviation from the constitutional-split recipe: unlike `note`/`gis`, this app's
//! `DocumentApp` impl, render/panels, and `create_app()` manifest are not defined per-app —
//! they're generated for all fifteen `norm` family apps at once by the
//! `define_norm_family_app!` macro in the shared plugin bundle (`s/plugin/norm/plugin/rs`),
//! which is out of scope for this split (owned by other concurrent agents). This crate exists
//! to complete the constitutional 7-crate shape and is ready to receive that content — depending
//! on {@link din16798}, {@link din16798_engine}, {@link din16798_op} — once the bundle is rewired.
