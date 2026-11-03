//! 🪞️ En1992 snapshot record impls, owned by the crate that owns `En1992Snapshot` so the orphan rule holds.

use crate::En1992Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(En1992Snapshot, extension="en1992", envelope_id="norm.en1992", sqlite=crate::sqlite::snapshot::codec);
