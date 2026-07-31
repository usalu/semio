//! 📝️ Playbook-play app — document entities (constitutional: general).
//!
//! Unlike most constitutional splits, the Playbook domain model (steps, blocks, generation forms,
//! and their `dsl`/`pack`/`protocol` derive impls) is owned by the kernel crate
//! `s/kernel/playbook/rs` (`semio-s-kernel-playbook`) — a generic, potentially multi-app domain, not
//! something this one `playbook-play` app defines. This crate re-exports exactly the
//! projection-level surface the app's other constitutional slots need, so `engine`/`dsl`/`op`/
//! `pack`/`protocol`/`ui` can depend on `rs` per the standard layout instead of each reaching into
//! the kernel individually.

pub use playbook::{
    empty_playbook_projection, PlaybookBlock, PlaybookBlockOption, PlaybookExpr, PlaybookSpec, PlaybookStep, PlaybookVectorField, PLAYBOOK_BUILTIN_KINDS, PLAYBOOK_DOCUMENT_SCHEMA,
};
