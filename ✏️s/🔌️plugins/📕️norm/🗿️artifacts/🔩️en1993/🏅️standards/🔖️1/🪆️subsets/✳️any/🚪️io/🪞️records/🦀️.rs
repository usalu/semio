//! 🪞️ En1993 snapshot record impls, owned by the crate that owns `En1993Snapshot` so the orphan rule holds.

use crate::En1993Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(En1993Snapshot, extension="en1993", envelope_id="norm.en1993", sqlite=crate::sqlite::snapshot::codec);
