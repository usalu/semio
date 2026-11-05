//! 👁️ Ephemeral local menu visibility published by panels and read by shell hooks.

use std::sync::{Mutex, PoisonError};

static OPEN: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// 📣 Records whether the Edit menu of the artifact `domain` is open, as the viewport panel just rendered it; the shell hook reads it to close the menu on an outside tap or Escape without owning the lane.
pub fn note_touch_menu(domain: &str, open: bool) {
    let mut store = OPEN.lock().unwrap_or_else(PoisonError::into_inner);
    store.retain(|known| known != domain);
    if open {
        store.push(domain.to_owned());
    }
}

/// 🖐️ Whether the Edit menu of the artifact `domain` (`norm.en1993`) is open.
pub fn touch_menu_open(domain: &str) -> bool {
    OPEN.lock().unwrap_or_else(PoisonError::into_inner).iter().any(|known| known == domain)
}

