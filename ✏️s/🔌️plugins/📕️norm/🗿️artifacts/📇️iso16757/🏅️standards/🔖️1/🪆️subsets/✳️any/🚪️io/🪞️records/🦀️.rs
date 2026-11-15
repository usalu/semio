//! 🪞️ Iso16757 snapshot record impls, owned by the crate that owns `Iso16757Snapshot` so the orphan rule holds.

use crate::Iso16757Snapshot;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(Iso16757Snapshot, extension="iso16757", envelope_id="norm.iso16757", sqlite=crate::sqlite::snapshot::codec);
