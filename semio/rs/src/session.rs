use std::sync::{Arc, Mutex, RwLock};

use crate::diff::DesignChange;
use crate::error::{Result, SemioError};
use crate::kit::{KitStore, KitStoreRef};

/// In-memory transaction boundary around a [`KitStore`].
pub struct KitGraphSession {
    inner: Mutex<Inner>,
}

struct Inner {
    kit: KitStoreRef,
    undo: Vec<DesignChange>,
    redo: Vec<DesignChange>,
}

impl KitGraphSession {
    pub fn new(kit: KitStore) -> Self {
        Self {
            inner: Mutex::new(Inner {
                kit: Arc::new(RwLock::new(kit)),
                undo: Vec::new(),
                redo: Vec::new(),
            }),
        }
    }

    pub fn from_ref(kit: KitStoreRef) -> Self {
        Self {
            inner: Mutex::new(Inner { kit, undo: Vec::new(), redo: Vec::new() }),
        }
    }

    pub fn kit_handle(&self) -> Result<KitStoreRef> {
        self.inner
            .lock()
            .map(|g| g.kit.clone())
            .map_err(|_| SemioError::LockPoisoned("session"))
    }

    pub fn map_kit<T, F: FnOnce(&KitStore) -> T>(&self, f: F) -> Result<T> {
        let g = self.inner.lock().map_err(|_| SemioError::LockPoisoned("session"))?;
        let kit = g.kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
        Ok(f(&kit))
    }

    pub fn map_kit_mut<T, F: FnOnce(&mut KitStore) -> T>(&self, f: F) -> Result<T> {
        let g = self.inner.lock().map_err(|_| SemioError::LockPoisoned("session"))?;
        let mut kit = g.kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
        Ok(f(&mut kit))
    }

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

    pub fn last_change(&self) -> Result<Option<DesignChange>> {
        let g = self.inner.lock().map_err(|_| SemioError::LockPoisoned("session"))?;
        Ok(g.undo.last().cloned())
    }
}
