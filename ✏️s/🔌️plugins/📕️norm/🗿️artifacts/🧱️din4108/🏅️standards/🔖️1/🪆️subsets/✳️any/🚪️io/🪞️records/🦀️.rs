//! 🪞️ Din4108 snapshot record impls, owned by the crate that owns `Din4108Snapshot` so the orphan rule holds.

use crate::Din4108Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(Din4108Snapshot, extension="din4108", envelope_id="norm.din4108", sqlite=crate::sqlite::snapshot::sqlite_codec);
