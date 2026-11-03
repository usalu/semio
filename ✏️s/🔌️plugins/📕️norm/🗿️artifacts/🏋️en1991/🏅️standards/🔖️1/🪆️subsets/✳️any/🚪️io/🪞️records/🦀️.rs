//! 🪞️ En1991 snapshot record impls, owned by the crate that owns `En1991Snapshot` so the orphan rule holds.

use crate::En1991Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(En1991Snapshot, extension="en1991", envelope_id="norm.en1991", sqlite=crate::sqlite::snapshot::sqlite_codec);
