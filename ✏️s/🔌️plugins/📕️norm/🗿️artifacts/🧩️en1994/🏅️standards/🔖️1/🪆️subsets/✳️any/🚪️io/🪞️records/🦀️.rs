//! 🪞️ En1994 snapshot record impls, owned by the crate that owns `En1994Snapshot` so the orphan rule holds.

use crate::En1994Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(En1994Snapshot, extension="en1994", envelope_id="norm.en1994", sqlite=crate::sqlite::snapshot::sqlite_codec);
