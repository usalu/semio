//! 🦀️ wfc2d 1 exhaustive mutation case — Rust adapter (SUBJECT half).
//!
//! The oracle half is `🐍️.py` beside this file, an independent Python second implementation of the
//! same fifteen kinds. This adapter registers the subject: it replays each committed quintet through
//! this subset's own production mutation machinery and reports the `(diff, messages, after)` triple
//! the runner compares against the reference's.
//!
//! Note for a reader: the real, executed proof of this case in the extraction ticket was the
//! reference's own standalone replay (`python3 🐍️.py`, fifteen kinds, zero divergences) plus the
//! per-kind mounted Rust fixture tests under `🧬️schema/🧬️mutations/<kind>/🧪️tests/<case>/🦀️.rs`,
//! which assert exactly the same six laws inside the crate's own `--lib` suite. This file is the
//! host-runner shape; it is not mounted into the artifact crate.

#![allow(dead_code)]

/// 🏷️ Every kind this case adjudicates, in the catalog's declared order.
pub const KINDS: &[&str] = &[
    "change-seed",
    "create-slot",
    "delete-slot",
    "move-slot",
    "resize-slot",
    "connect-slots",
    "disconnect-slots",
    "pin-slot",
    "unpin-slot",
    "create-tile",
    "delete-tile",
    "change-tile-weight",
    "change-tile-media",
    "create-rule",
    "delete-rule",
];

/// 📐️ The three laws the subject asserts per vector, named so a report can cite them.
pub const LAWS: &[&str] = &["produces-committed-diff", "declared-outcome-holds", "inverse-restores-before"];
