//! 🪞️ En1990 snapshot record impls, owned by the crate that owns `En1990Snapshot` so the orphan rule holds.

use crate::En1990Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(En1990Snapshot, extension="en1990", envelope_id="norm.en1990", sqlite=crate::sqlite::snapshot::sqlite_codec);
