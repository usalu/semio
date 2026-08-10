//! 🎹️ Vdi3805Composer (1 standard) — aggregates its subsets' composer entries value-level.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::vdi3805::standards::v1::subsets::any::composer::Vdi3805Composer as Vdi3805AnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<Vdi3805AnyComposer>()]).as_slice()
}
