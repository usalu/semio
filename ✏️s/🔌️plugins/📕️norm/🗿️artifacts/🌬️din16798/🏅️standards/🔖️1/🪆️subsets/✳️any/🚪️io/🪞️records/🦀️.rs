//! 🪞️ Din16798 snapshot record impls, owned by the crate that owns `Din16798Snapshot` so the orphan rule holds.

use crate::Din16798Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(Din16798Snapshot, extension="din16798", envelope_id="norm.din16798", sqlite=crate::sqlite::snapshot::sqlite_codec);
