from specs.lib import *
F = DB + '/🗄️storage/🦀️.rs'
TK = '#[cfg(any(test, feature = "testkit"))]'
CH = [
    ch('🧭', 'db-backend', 'db_backend', [(78, 231)],
       'Db backend: the concrete backend enum naming every storage backend and its facet accessors.'),
    ch('📼', 'wal-and-snapshot-refs', 'wal_and_snapshot_refs', [(234, 484)],
       'Wal and snapshot refs: the wal storage reference and the snapshot storage reference with their storage trait implementations.'),
    ch('🗂', 'payload-catalog-index-and-lease-refs', 'payload_catalog_index_and_lease_refs', [(487, 746)],
       'Payload, catalog, index and lease refs: the payload, catalog, index and lease storage references with their storage trait implementations.'),
    ch('🧯', 'fault-storage', 'fault_storage', [(750, 1020)],
       'Fault storage: the fault script and the fault injecting storage with its storage trait implementations, available to tests and the testkit.', cfg_mod=TK),
]
SPEC = dict(orig='stor.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='stor.rs'))
