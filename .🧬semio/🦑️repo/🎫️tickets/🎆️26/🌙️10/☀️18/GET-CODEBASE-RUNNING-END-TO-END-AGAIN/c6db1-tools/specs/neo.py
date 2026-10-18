from specs.lib import *
F = DB + '/🗄️storage/🌐️neo4j/🦀️.rs'
CH = [
    ch('🧬', 'codec-laws-and-error-mapping', 'codec_laws_and_error_mapping', [(72, 203)],
       'Codec, laws and error mapping: driver byte codecs, the WAL and lease laws and the Neo4j error mapping.'),
    ch('📜', 'cypher-statements', 'cypher_statements', [(207, 397)],
       'Cypher statements: the schema statements and every Cypher text of the WAL, snapshot, payload, catalog, index, writer and lease rows.'),
    ch('🔒', 'writer-fence-and-executor-core', 'writer_fence_and_executor_core', [(402, 753)],
       'Writer fence and executor core: the WAL writer lease and guard, the executor and its WAL storage.'),
    ch('🗃', 'executor-storage-impls', 'executor_storage_impls', [(757, 1011)],
       'Executor storage impls: the snapshot, payload, catalog, index and lease storage of the executor.'),
    ch('⚙', 'typed-executor', 'typed_executor', [(1014, 1233)],
       'Typed executor: the typed operation execution and the task executor impl.'),
    ch('🕸', 'neo4j-storage-backend', 'neo4j_storage_backend', [(1236, 1490)],
       'Neo4j storage backend: `Neo4jStorage`, its result helpers and the storage trait impls.'),
]
SPEC = dict(orig='neo.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='neo.rs'))
