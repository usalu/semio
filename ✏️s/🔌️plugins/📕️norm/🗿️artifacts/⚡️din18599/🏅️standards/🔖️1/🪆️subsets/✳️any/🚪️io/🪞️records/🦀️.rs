//! 🪞️ Din18599 snapshot record impls, owned by the crate that owns `Din18599Snapshot` so the orphan rule holds.

use crate::Din18599Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(Din18599Snapshot, extension="din18599", envelope_id="norm.din18599", sqlite=crate::sqlite::snapshot::sqlite_codec);
