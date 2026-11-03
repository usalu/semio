//! 🪞️ En1999 snapshot record impls, owned by the crate that owns `En1999Snapshot` so the orphan rule holds.

use crate::En1999Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(En1999Snapshot, extension="en1999", envelope_id="norm.en1999", sqlite=crate::sqlite::snapshot::codec);
