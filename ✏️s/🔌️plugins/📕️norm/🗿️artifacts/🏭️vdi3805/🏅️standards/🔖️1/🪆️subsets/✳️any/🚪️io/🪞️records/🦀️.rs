//! 🪞️ Vdi3805 snapshot record impls, owned by the crate that owns `Vdi3805Snapshot` so the orphan rule holds.

use crate::Vdi3805Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(Vdi3805Snapshot, extension="vdi3805", envelope_id="norm.vdi3805", sqlite=crate::sqlite::snapshot::sqlite_codec);
