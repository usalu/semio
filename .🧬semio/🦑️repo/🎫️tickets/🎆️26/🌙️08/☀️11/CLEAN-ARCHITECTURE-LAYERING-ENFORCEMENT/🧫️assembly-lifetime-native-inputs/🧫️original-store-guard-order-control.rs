use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard, RwLock};
#[derive(Clone)] struct ArtifactCodec;
#[derive(Clone)] struct DialectMigration;
mod os_io { #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)] pub struct ArtifactDialect; }
#[derive(Clone, Debug, PartialEq, Eq)] pub enum DocumentCodecRegistryError { Unavailable }
#[derive(Clone, Debug, PartialEq, Eq)] pub enum DialectMigrationRegistryError { Unavailable }
impl std::fmt::Display for DocumentCodecRegistryError { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("document unavailable") } }
impl std::fmt::Display for DialectMigrationRegistryError { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("migration unavailable") } }
fn document_codec_registry() -> &'static RwLock<BTreeMap<String, ArtifactCodec>> { static VALUE: std::sync::OnceLock<RwLock<BTreeMap<String, ArtifactCodec>>> = std::sync::OnceLock::new(); VALUE.get_or_init(|| RwLock::new(BTreeMap::new())) }
fn dialect_migration_registry() -> &'static RwLock<BTreeMap<(os_io::ArtifactDialect, os_io::ArtifactDialect), DialectMigration>> { static VALUE: std::sync::OnceLock<RwLock<BTreeMap<(os_io::ArtifactDialect, os_io::ArtifactDialect), DialectMigration>>> = std::sync::OnceLock::new(); VALUE.get_or_init(|| RwLock::new(BTreeMap::new())) }

/// 🧷️ One process-wide guard for a plugin's all-registry publication phase.
pub struct ArtifactAssemblyTransaction {
    _guard: MutexGuard<'static, ()>,
}

/// 🚫️ The all-registry publication barrier is unavailable after a writer panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactAssemblyTransactionError {
    Unavailable,
}

impl std::fmt::Display for ArtifactAssemblyTransactionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("artifact assembly transaction unavailable")
    }
}

impl std::error::Error for ArtifactAssemblyTransactionError {}

fn artifact_assembly_lock() -> &'static Mutex<()> {
    static LOCK: std::sync::OnceLock<Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

/// 🧷️ Begins the only cross-registry transaction accepted by artifact registration APIs.
#[must_use]
pub fn begin_artifact_assembly() -> Result<ArtifactAssemblyTransaction, ArtifactAssemblyTransactionError> {
    Ok(ArtifactAssemblyTransaction { _guard: artifact_assembly_lock().lock().map_err(|_| ArtifactAssemblyTransactionError::Unavailable)? })
}

/// 🧷️ All writable store registries held before an artifact assembly can publish anything.
pub struct ArtifactAssemblyStoreRegistryGuards {
    document_codecs: std::sync::RwLockWriteGuard<'static, BTreeMap<String, ArtifactCodec>>,
    dialect_migrations: std::sync::RwLockWriteGuard<'static, BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>>,
}

/// 🚫️ A staged store registry assembly cannot be preflighted or committed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactAssemblyStoreRegistryError {
    DocumentCodec(DocumentCodecRegistryError),
    DialectMigration(DialectMigrationRegistryError),
}

impl std::fmt::Display for ArtifactAssemblyStoreRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DocumentCodec(error) => error.fmt(formatter),
            Self::DialectMigration(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ArtifactAssemblyStoreRegistryError {}

/// 🧷️ Acquires every store registry write lock before an assembly validates or mutates state.
#[must_use]
pub fn acquire_artifact_assembly_store_registry_guards(_assembly: &ArtifactAssemblyTransaction) -> Result<ArtifactAssemblyStoreRegistryGuards, ArtifactAssemblyStoreRegistryError> {
    let document_codecs = document_codec_registry().write().map_err(|_| ArtifactAssemblyStoreRegistryError::DocumentCodec(DocumentCodecRegistryError::Unavailable))?;
    let dialect_migrations = dialect_migration_registry().write().map_err(|_| ArtifactAssemblyStoreRegistryError::DialectMigration(DialectMigrationRegistryError::Unavailable))?;
    Ok(ArtifactAssemblyStoreRegistryGuards { document_codecs, dialect_migrations })
}


fn main() {
    let transaction = begin_artifact_assembly().unwrap();
    let guards = acquire_artifact_assembly_store_registry_guards(&transaction).unwrap();
    drop(guards);
    drop(transaction);
    println!("[DEBUG] Original Store API released registry guards before the barrier.");
}
