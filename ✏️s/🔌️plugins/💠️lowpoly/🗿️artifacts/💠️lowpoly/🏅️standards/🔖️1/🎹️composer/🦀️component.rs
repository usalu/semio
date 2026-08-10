//! 🎹️ LowpolyComposer (1 standard) — aggregates its subsets' composer entries value-level.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::lowpoly::standards::v1::subsets::any::composer::LowpolyComposer as LowpolyAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<LowpolyAnyComposer>()]).as_slice()
}
