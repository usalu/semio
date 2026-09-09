//! 🔐️ Serialises every test that admits a `generation3d_admit_publication_authority` lease.
//!
//! The lease table is a PROCESS-GLOBAL fixed registry of `GENERATION3D_PUBLICATION_SLOTS` (4)
//! entries (`🧬️schema/🧬️mutations/💾️binary/🦀️.rs`), so two tests holding leases at the same time
//! saturate it and the loser fails with `generation3d-publication.saturated` — an order- and
//! thread-count-dependent failure that has nothing to do with what either test asserts
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use std::sync::{Mutex, MutexGuard};

static SERIAL: Mutex<()> = Mutex::new(());

pub fn lock() -> MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
