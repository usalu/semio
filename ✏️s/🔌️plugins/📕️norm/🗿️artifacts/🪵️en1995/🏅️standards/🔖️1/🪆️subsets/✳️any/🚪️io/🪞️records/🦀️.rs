//! 🪞️ En1995 snapshot record impls, owned by the crate that owns `En1995Snapshot` so the orphan rule holds.

use crate::En1995Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(En1995Snapshot, extension="en1995", envelope_id="norm.en1995", sqlite=crate::sqlite::snapshot::sqlite_codec);
