//! 🪞️ En1997 snapshot record impls, owned by the crate that owns `En1997Snapshot` so the orphan rule holds.

use crate::En1997Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(En1997Snapshot, extension="en1997", envelope_id="norm.en1997", sqlite=crate::sqlite::snapshot::codec);
