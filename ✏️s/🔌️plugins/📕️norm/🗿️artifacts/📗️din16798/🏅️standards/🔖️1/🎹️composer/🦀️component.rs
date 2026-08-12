//! 🎹️ Din16798Composer (1 standard) — aggregates its subsets' composer entries value-level. W5a
//! (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) deleted the four stdio format-export entries (zip/csv/xlsx/json) this file used
//! to wrap: each one dumped or wrapped this artifact's raw DSL text as if it were that target
//! format's real binary shape (see the deleted `🚪️io/📤️export/🧵️serializers` leaves' own git
//! history) — a fabricated shape, not a real codec. Only the native `s.din16798` entry remains.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::din16798::standards::v1::subsets::any::composer::Din16798Composer as Din16798AnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![
        composer_entry_of::<Din16798AnyComposer>(),
    ]).as_slice()
}
