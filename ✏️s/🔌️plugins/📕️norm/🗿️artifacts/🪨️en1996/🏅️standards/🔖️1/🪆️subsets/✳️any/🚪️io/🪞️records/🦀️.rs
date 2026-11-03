//! 🪞️ En1996 snapshot record impls, owned by the crate that owns `En1996Snapshot` so the orphan rule holds.

use crate::En1996Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(En1996Snapshot, extension="en1996", envelope_id="norm.en1996", sqlite=crate::sqlite::snapshot::sqlite_codec);
