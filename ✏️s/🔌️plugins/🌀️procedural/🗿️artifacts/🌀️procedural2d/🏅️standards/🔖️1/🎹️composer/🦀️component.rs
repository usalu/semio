//! 🎹️ Procedural2dComposer (1 standard) — aggregates its subsets' composer entries value-level.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::procedural2d::standards::v1::subsets::any::composer::Procedural2dComposer as Procedural2dAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<Procedural2dAnyComposer>()]).as_slice()
}
