use std::sync::{Arc, Mutex, RwLock};

use crate::diff::DesignChange;
use crate::error::{Result, SemioError};
use crate::kit::{Kit, KitRef};

/// In-memory transaction boundary around a [`Kit`].
///
/// Holds the canonical `Arc<RwLock<Kit>>` and exposes lock-managed read/write
/// access, plus a lightweight undo/redo stack of design changes.
pub struct KitGraphSession {
    inner: Mutex<Inner>,
}

struct Inner {
    kit: KitRef,
    undo: Vec<DesignChange>,
    redo: Vec<DesignChange>,
}

impl KitGraphSession {
    pub fn new(kit: Kit) -> Self {
        Self {
            inner: Mutex::new(Inner {
                kit: Arc::new(RwLock::new(kit)),
                undo: Vec::new(),
                redo: Vec::new(),
            }),
        }
    }

    pub fn from_ref(kit: KitRef) -> Self {
        Self {
            inner: Mutex::new(Inner { kit, undo: Vec::new(), redo: Vec::new() }),
        }
    }

    /// Shared handle to the kit; callers acquire read/write locks themselves.
    pub fn kit_handle(&self) -> Result<KitRef> {
        self.inner
            .lock()
            .map(|g| g.kit.clone())
            .map_err(|_| SemioError::LockPoisoned("session"))
    }

    /// Run a read-only mapping against the kit.
    pub fn map_kit<T, F: FnOnce(&Kit) -> T>(&self, f: F) -> Result<T> {
        let g = self.inner.lock().map_err(|_| SemioError::LockPoisoned("session"))?;
        let kit = g.kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
        Ok(f(&kit))
    }

    /// Run a mutating mapping against the kit.
    pub fn map_kit_mut<T, F: FnOnce(&mut Kit) -> T>(&self, f: F) -> Result<T> {
        let g = self.inner.lock().map_err(|_| SemioError::LockPoisoned("session"))?;
        let mut kit = g.kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
        Ok(f(&mut kit))
    }

    /// Commit a design change, recording it on the undo stack.
    pub fn commit(&self, change: DesignChange) -> Result<()> {
        let mut g = self.inner.lock().map_err(|_| SemioError::LockPoisoned("session"))?;
        g.undo.push(change);
        g.redo.clear();
        Ok(())
    }

    pub fn undo_depth(&self) -> Result<usize> {
        let g = self.inner.lock().map_err(|_| SemioError::LockPoisoned("session"))?;
        Ok(g.undo.len())
    }

    pub fn redo_depth(&self) -> Result<usize> {
        let g = self.inner.lock().map_err(|_| SemioError::LockPoisoned("session"))?;
        Ok(g.redo.len())
    }

    /// Peek at the last committed change without popping it.
    pub fn last_change(&self) -> Result<Option<DesignChange>> {
        let g = self.inner.lock().map_err(|_| SemioError::LockPoisoned("session"))?;
        Ok(g.undo.last().cloned())
    }
}
