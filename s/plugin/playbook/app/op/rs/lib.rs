//! ⚡ Playbook-play app — operation enum + constructors (constitutional: op).
//!
//! `PlaybookOperation`/`PlaybookDiff` and their `protocol::Operation`/`OperationDiff` impls (plus the
//! private `apply_playbook_edit_operation` match) are owned by the kernel crate
//! `s/kernel/playbook/rs` — this crate re-exposes the operation type and its constructor helpers
//! under the app's own constitutional `op` slot so `protocol`/`ui` depend on `op` per the standard
//! layout instead of reaching into the kernel directly.

pub use playbook::{
    add_block_operation, add_step_operation, move_block_operation, move_step_operation, remove_block_operation, remove_step_operation,
    update_playbook_title_operation, PlaybookOperation,
};
