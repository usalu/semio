from specs.lib import *
F = DB + '/🗄️storage/🐘️postgres/🦀️.rs'
CH = [
    ch('🗃', 'schema-and-connection', 'schema_and_connection', [(38, 95), (125, 168)],
       'Postgres schema and connection: the schema statements and bootstrap lock, the schema bootstrap, the wal segment state query and the executor with its connection operations.'),
    ch('🧯', 'error-mapping-and-conversions', 'error_mapping_and_conversions', [(172, 256)],
       'Postgres error mapping and conversions: the sqlx and create error mapping, the integer and ascending id conversions and the read range and truncate validation.'),
    ch('🔐', 'wal-writer-fence', 'wal_writer_fence', [(260, 400)],
       'Postgres wal writer fence: the advisory lock namespace and keys, the writer session, the unlock and the writer guard.'),
    ch('💾', 'executor-storage-operations', 'executor_storage_operations', [(404, 705)],
       'Postgres executor storage operations: the wal, snapshot, payload, catalog and index storage operations of the executor.'),
    ch('🤝', 'executor-lease-storage', 'executor_lease_storage', [(709, 831)],
       'Postgres executor lease storage: the existing lease row, the acquire, renew and release decisions and the lease storage operations of the executor.'),
    ch('🧭', 'typed-executor', 'typed_executor', [(835, 1075)],
       'Postgres typed executor: the typed task operations and the task executor implementation of the executor.'),
    ch('🐘', 'postgres-storage-backend', 'postgres_storage_backend', [(1078, 1325)],
       'Postgres storage backend: the public postgres storage, its drop, the document, output, unit, page and list helpers, the storage trait implementations and the backend capabilities.'),
]
SPEC = dict(orig='pg.rs', file=F, children=CH, wiring='block', wire_at=124, external=mounted(F, orig='pg.rs'))
