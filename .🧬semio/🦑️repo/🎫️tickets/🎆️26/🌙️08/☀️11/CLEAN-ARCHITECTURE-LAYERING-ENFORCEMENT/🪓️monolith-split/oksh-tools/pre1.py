# 🪓️ Pre-carve source edits of OS-KERNEL-SQLITE-HOST: split ArtifactSqliteSnapshotHost by layer (core hooks stay, derived `sqlite_codec` becomes ArtifactSqliteSnapshotOperations), lower TypedFileSettlement, relocate the space-history registration glue, rewrite the sqlite_codec callers.
# usage: pre1.py dry|write
import os, re, sys
R = '/Users/ueli/Documents/semio'
M = R + '/🧰️framework/🛍️products/💻️os/🔨️modules'
S = M + '/🏪️store'
SNAP = S + '/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot'
WRITE = sys.argv[1] == 'write'
log = []

def rd(p): return open(p, encoding='utf8').read()

def wr(p, t):
    log.append(p)
    if WRITE:
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, 'w', encoding='utf8').write(t)

def edit(p, fn):
    t = rd(p); n = fn(t)
    assert n != t, ('unchanged', p)
    wr(p, n)

def once(t, a, b):
    assert t.count(a) == 1, (t.count(a), a[:90])
    return t.replace(a, b)

CODEC_START = "    /// 🪶️ Builds the relational codec only when the snapshot actually implements it.\n    fn sqlite_codec()"
CODEC_END = "import_file:Some(crate::io::io_mechanism::import_snapshot_file_into::<Self>) }\n    }\n}\n"

def e_trait(t):
    assert t.count(CODEC_START) == 1 and t.count(CODEC_END) == 1
    s = t.index(CODEC_START); e = t.index(CODEC_END) + len(CODEC_END)
    block = t[s:e]
    t = t[:s].rstrip('\n') + '\n}\n' + t[e:]
    t = once(t, "/// 🪶️ A snapshot's handwritten relational model, independently interpretable through SQLite.\n", "/// 🪶️ A snapshot's handwritten relational model, independently interpretable through SQLite. The core hooks live here; the derived codec builder is `ArtifactSqliteSnapshotOperations` in the codec operations layer.\n")
    return t

OPS = '''//! 🧰️ Derived relational snapshot operations of the codec layer: they compose the registry-layer export and import with the typed file mechanism, so they sit above the core host trait and below nothing that the host trait needs.
//!
//! @see 🪶️ `ArtifactSqliteSnapshotHost` the core trait whose authored hooks the operations consume.
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use super::{ArtifactSqliteSnapshotCodec,ArtifactSqliteSnapshotHost,NativeSnapshotDecodeOwner,NativeSnapshotEncodeOwner};

/// 🏭️ Builds the erased relational codec of every core host; the blanket implementation is the only one, so no owner can fork the derived behaviour.
pub trait ArtifactSqliteSnapshotOperations: ArtifactSqliteSnapshotHost {
    /// 🪶️ Builds the relational codec only when the snapshot actually implements it.
    fn sqlite_codec() -> ArtifactSqliteSnapshotCodec
    where
        Self: Sized + ArtifactDsl + ArtifactPack + semio_framework_value::retirement::RetireOwned + Sync;
}

fn export_snapshot_impl<P: ArtifactDsl + ArtifactPack + ArtifactSqliteSnapshotHost + semio_framework_value::retirement::RetireOwned>(schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,payload:&crate::io_schema::IoPayload,control:&mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>,native:&mut NativeSnapshotDecodeOwner<'_,'_>)->crate::io_schema::IoResult<crate::sqlite_snapshot::SqliteDatabase>{super::sqlite_snapshot_retirement::export::<P>(schema,dialect,payload,control,native)}
fn import_snapshot_impl<P: ArtifactDsl + ArtifactPack + ArtifactSqliteSnapshotHost + semio_framework_value::retirement::RetireOwned>(schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,input:&mut Option<crate::sqlite_snapshot::SqliteDatabase>,encoding:crate::sqlite_snapshot::SnapshotEncoding,control:&mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>,native:&mut NativeSnapshotEncodeOwner<'_,'_>)->crate::io_schema::IoResult<crate::io_schema::IoPayload>{super::sqlite_snapshot_retirement::import::<P>(schema,dialect,input,encoding,control,native)}

impl<T: ArtifactSqliteSnapshotHost + ?Sized> ArtifactSqliteSnapshotOperations for T {
    fn sqlite_codec() -> ArtifactSqliteSnapshotCodec
    where
        Self: Sized + ArtifactDsl + ArtifactPack + semio_framework_value::retirement::RetireOwned + Sync,
    {
        ArtifactSqliteSnapshotCodec { schema: std::borrow::Cow::Borrowed(Self::SQLITE_SCHEMA), snapshot_type: Some(std::any::TypeId::of::<Self>()), export: export_snapshot_impl::<Self>, import: import_snapshot_impl::<Self>,export_file:Some(crate::io::io_mechanism::export_snapshot_file_read::<Self>),import_file:Some(crate::io::io_mechanism::import_snapshot_file_into::<Self>) }
    }
}
'''

REG = '''//! 📣️ Explicit owner registration of the space-history SQLite snapshot: it publishes the history factory and the semantic capability atomically, so it sits above the codec registry, the IO mechanism and the codec factory.
use super::space_history::io::sqlite::snapshot::SQLITE_SNAPSHOT_DIALECT;
use super::space_history_document::{S_SPACE_HISTORY_SCHEMA,SpaceHistorySnapshot};
use super::{artifact_codec,SpaceHistoryMutation};

/// 📣️ Explicit owner registration publishes the history factory and semantic capability atomically.
/// Call before registry I/O; generic Store construction and retained hydration do not publish codecs.
pub fn register_sqlite_snapshot() -> Result<(), crate::io::ArtifactAssemblyRegistryError> {
    crate::io::register_native_snapshot_codec(SQLITE_SNAPSHOT_DIALECT, artifact_codec::<SpaceHistorySnapshot, SpaceHistoryMutation>(S_SPACE_HISTORY_SCHEMA))
}
'''

SETTLE_HEAD = '''//! ⚖️ Settles the original receiving allocation of a typed snapshot file against the native owned bytes while its body is written or read, and keeps the first refusal cause.
use crate::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
use semio_framework_value::ValueError;

'''

def e_settlement_out(t):
    s = t.index("    pub(crate) struct TypedFileSettlement"); e = t.index("    fn typed_file_validation(")
    block = t[s:e]
    t = t[:s] + t[e:]
    t = once(t, "    use semio_framework_diagnostic::Diagnostic;\n", "    use semio_framework_diagnostic::Diagnostic;\n    use crate::os_store::space_history::io::sqlite::snapshot::settlement::TypedFileSettlement;\n")
    return t, block

def settle_file(block):
    b = block.replace('pub(crate) struct', 'pub struct').replace('pub(crate) control', 'pub control').replace('pub(crate) fn', 'pub fn')
    b = b.replace('crate::sqlite_snapshot::SqliteSnapshotReceivingControl', 'crate::sqlite_snapshot::SqliteSnapshotReceivingControl')
    lines = [l[4:] if l.startswith('    ') else l for l in b.split('\n')]
    return SETTLE_HEAD + '/// ⚖️ Typed snapshot file settlement over the receiving scope of one control.\n' + '\n'.join(lines).rstrip('\n') + '\n'

def main():
    # 1. core trait
    edit(S + '/🚪️io/🦀️.rs', e_trait)
    # 2. operations file + mounts
    wr(S + '/📦️codec/🪶️snapshot-capability/🧰️codec-operations/🦀️.rs', OPS)
    def e_root(t):
        t = once(t, '#[path = "📦️codec/🪶️snapshot-capability/🪶️native-retirement/🦀️.rs"]\nmod sqlite_snapshot_retirement;\n', '#[path = "📦️codec/🪶️snapshot-capability/🪶️native-retirement/🦀️.rs"]\nmod sqlite_snapshot_retirement;\n\n#[path = "📦️codec/🪶️snapshot-capability/🧰️codec-operations/🦀️.rs"]\nmod sqlite_snapshot_operations;\npub use sqlite_snapshot_operations::ArtifactSqliteSnapshotOperations;\n')
        t = once(t, '#[path = "📜️space-history/🦀️.rs"]\npub mod space_history;\n', '#[path = "📜️space-history/🦀️.rs"]\npub mod space_history;\n#[path = "📜️space-history/🏠️space/📣️registration/🦀️.rs"]\npub mod space_history_registration;\n')
        return t
    edit(S + '/🦀️.rs', e_root)
    # 3. space history glue and ArtifactStoreSnapshot impl
    wr(S + '/📜️space-history/🏠️space/📣️registration/🦀️.rs', REG)
    def e_snapfile(t):
        a = t.index('/// 📣️ Explicit owner registration publishes the history factory')
        b = t.index('pub use semio_framework_os_store_space_schema::lifted_snapshot::')
        t = t[:a] + t[b:]
        t = once(t, '#[path = "🚦️native/🦀️.rs"]\nmod native;\n', '#[path = "🚦️native/🦀️.rs"]\nmod native;\n#[path = "⚖️settlement/🦀️.rs"]\npub mod settlement;\n')
        return t
    edit(SNAP + '/🦀️.rs', e_snapfile)
    def e_bin(t):
        t = once(t, "impl ArtifactStoreSnapshot for SpaceHistorySnapshot {\nfn sqlite_snapshot_codec() -> Option<ArtifactSqliteSnapshotCodec> {\n        Some(<Self as ArtifactSqliteSnapshotHost>::sqlite_codec())\n    }\n}\n", "impl ArtifactStoreSnapshot for SpaceHistorySnapshot {}\n")
        t = once(t, "use crate::os_store::serialization::{ArtifactStoreSnapshot,ArtifactSqliteSnapshotHost,ArtifactSqliteSnapshotCodec};", "use crate::os_store::serialization::ArtifactStoreSnapshot;")
        return t
    edit(S + '/🚪️io/💾️binary/📸️snapshot/🦀️.rs', e_bin)
    # 4. TypedFileSettlement
    mt = rd(M + '/🚪️io/🦀️.rs')
    mt2, block = e_settlement_out(mt)
    wr(M + '/🚪️io/🦀️.rs', mt2)
    wr(SNAP + '/⚖️settlement/🦀️.rs', settle_file(block))
    edit(SNAP + '/🛂️validation/🦀️.rs', lambda t: once(t, 'use crate::io::io_mechanism::TypedFileSettlement;', 'use super::settlement::TypedFileSettlement;'))
    edit(SNAP + '/🫴️receiving/🦀️.rs', lambda t: once(once(t, ',io::io_mechanism::TypedFileSettlement,', ','), 'use semio_framework_value::ValueError;', 'use super::settlement::TypedFileSettlement;\nuse semio_framework_value::ValueError;'))
    # 5. tests, pins
    T = SNAP + '/🧪️tests'
    def e_tests(t):
        assert t.count('store::space_history::io::sqlite::snapshot::register_sqlite_snapshot()') == 2
        t = t.replace('store::space_history::io::sqlite::snapshot::register_sqlite_snapshot()', 'store::space_history_registration::register_sqlite_snapshot()')
        t = once(t, 'store::bare_artifact_codec::<SpaceHistorySnapshot, SpaceHistoryMutation>(S_SPACE_HISTORY_SCHEMA)', 'store::artifact_codec::<SpaceHistorySnapshot, SpaceHistoryMutation>(S_SPACE_HISTORY_SCHEMA)')
        return t
    edit(T + '/🦀️.rs', e_tests)
    def e_ts(t):
        t = once(t, '"store::space_history::io::sqlite::snapshot::register_sqlite_snapshot"', '"store::space_history_registration::register_sqlite_snapshot"')
        t = once(t, 'parse(join(snapshot,"🦀️.rs")), ownerRegistration', 'parse(join(store,"📜️space-history/🏠️space/📣️registration/🦀️.rs")), ownerRegistration')
        return t
    edit(T + '/🟦️.ts', e_ts)
    edit(T + '/🧫️fixtures/📣️registration/🔣️.json', lambda t: once(t, '"space_history::io::sqlite::snapshot::register_sqlite_snapshot"', '"space_history_registration::register_sqlite_snapshot"'))
    def e_typed(t):
        a = 'crate::os_store::space_history::io::sqlite::snapshot::register_sqlite_snapshot'
        assert t.count(a) == 4
        return t.replace(a, 'crate::os_store::space_history_registration::register_sqlite_snapshot')
    edit(M + '/🚪️io/📸️typed-snapshot/🧪️tests/🦀️.rs', e_typed)
    print('\n'.join(os.path.relpath(p, R) for p in log))

main()
