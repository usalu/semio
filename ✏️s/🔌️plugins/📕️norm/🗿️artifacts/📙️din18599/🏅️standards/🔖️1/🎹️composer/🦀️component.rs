//! 🎹️ Din18599Composer (1 standard) — aggregates its subsets' composer entries value-level. W5a
//! (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) deleted the four stdio format-export entries (zip/csv/xlsx/json) this file used
//! to wrap: each one dumped or wrapped this artifact's raw DSL text as if it were that target
//! format's real binary shape (see the deleted `🚪️io/📤️export/🧵️serializers` leaves' own git
//! history) — a fabricated shape, not a real codec. Only the native `s.din18599` entry remains.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::din18599::standards::v1::subsets::any::composer::Din18599Composer as Din18599AnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![
        composer_entry_of::<Din18599AnyComposer>(),
    ]).as_slice()
}
