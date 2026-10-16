# orphan repairs of batch b2: impls of foreign traits for the space-schema types move into the crate that owns the types
import ns
S = ns.S
SS = 'os-store-space-schema'
SNAP = S + '/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs'
MOVES = [
    dict(src=S + '/🚪️io/💾️binary/📸️snapshot/🦀️.rs', match=r'impl ArtifactPack for SpaceHistorySnapshot', dest=SS, count=1),
    dict(src=S + '/🚪️io/📝️text/📸️snapshot/🦀️.rs', match=r'(impl ArtifactDsl for SpaceHistorySnapshot|fn canonical_ordinal|macro_rules!\s*record_tree|impl Tree for SpaceCheckpoint)', dest=SS, count=4),
    dict(src=S + '/🚪️io/📝️text/🧬️mutations/🦀️.rs', match=r'(impl OpText for SpaceHistoryMutation|fn canonical_ordinal|macro_rules!\s*record_tree|impl Tree for SpaceHistoryMutation)', dest=SS, count=4),
    dict(src=SNAP, match=r'(fn schema\(|fn rows\(|fn word\(|fn pair\(|fn write_rows\(|impl pack::snapshot::ArtifactSqliteSnapshot for SpaceHistorySnapshot)', dest=SS, count=6, reexport=True),
]
