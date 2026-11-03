//! 🪞️ En1998 snapshot record impls, owned by the crate that owns `En1998Snapshot` so the orphan rule holds.

use crate::En1998Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(En1998Snapshot, extension="en1998", envelope_id="norm.en1998", sqlite=crate::sqlite::snapshot::sqlite_codec);
