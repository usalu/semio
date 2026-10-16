//! 📣️ Explicit owner registration of the space-history SQLite snapshot: it publishes the history factory and the semantic capability atomically, so it sits above the codec registry, the IO mechanism and the codec factory.
use super::space_history::io::sqlite::snapshot::SQLITE_SNAPSHOT_DIALECT;
use super::space_history_document::{S_SPACE_HISTORY_SCHEMA,SpaceHistorySnapshot};
use super::{artifact_codec,SpaceHistoryMutation};

/// 📣️ Explicit owner registration publishes the history factory and semantic capability atomically.
/// Call before registry I/O; generic Store construction and retained hydration do not publish codecs.
pub fn register_sqlite_snapshot() -> Result<(), crate::io::ArtifactAssemblyRegistryError> {
    crate::io::register_native_snapshot_codec(SQLITE_SNAPSHOT_DIALECT, artifact_codec::<SpaceHistorySnapshot, SpaceHistoryMutation>(S_SPACE_HISTORY_SCHEMA))
}
