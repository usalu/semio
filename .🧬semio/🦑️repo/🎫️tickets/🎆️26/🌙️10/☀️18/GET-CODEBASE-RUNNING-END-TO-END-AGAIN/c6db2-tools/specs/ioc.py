from specs.lib import *
F = DB + '/🗄️storage/🧱️io-core/🦀️.rs'
CH = [
    ch('💳', 'credit-limits-and-operation-ledger', 'credit_limits_and_operation_ledger', [(13, 98), (105, 370)],
       'Credit limits and operation ledger: the blocking queue signal, the page, item, byte and control limits, the credit, the test ledger owner and the operation ledger with its reserve, add, task, result lease and release operations.'),
    ch('📃', 'page-arena-and-lease', 'page_arena_and_lease', [(372, 571)],
       'Page arena and lease: the page phase, the page backing and slots, the page arena, the page lease and the page checkout.'),
    ch('✍', 'page-writer', 'page_writer', [(573, 867)],
       'Page writer: the page writer with its seal, the rejected writer and the writer drop and debug.'),
    ch('🔑', 'driver-reservation-artifact-id-and-external-bytes', 'driver_reservation_artifact_id_and_external_bytes', [(869, 1051)],
       'Driver reservation, artifact id and external bytes: the driver reservation, the artifact id owner and the external bytes owner.'),
    ch('📚', 'pages-text-and-u64-list', 'pages_text_and_u64_list', [(1053, 1552)],
       'Pages, text and u64 list: the retained pages with their comparisons, the page reader, the bounded text and the bounded u64 list.'),
    ch('📨', 'tasks-results-and-census', 'tasks_results_and_census', [(1554, 2065)],
       'Tasks, results and census: the backend control, the wal segment state, the task and result unions, the lease result, the task phase and kind census and the task and result operations.'),
    ch('🚦', 'executor-driver-and-faults', 'executor_driver_and_faults', [(2067, 2301)],
       'Executor, driver and faults: the execution step and mode, the async driver types and runtime, the writer release step, the task executor trait, the backend parts and the fault kinds, causes and terminal.'),
    ch('🧺', 'task-slots-lost-owners-and-admission', 'task_slots_lost_owners_and_admission', [(2303, 2587)],
       'Task slots, lost owners and admission: the task handle and slots, the task arena, the close queue, the result lease, the lost owner parking and the admission waiters.'),
    ch('🗃', 'storage-capabilities-and-traits', 'storage_capabilities_and_traits', [(2590, 2776)],
       'Storage capabilities and traits: the storage capabilities and the wal, snapshot, payload, catalog, index and lease storage traits.'),
    ch('🎫', 'wal-writer-permit-and-admission', 'wal_writer_permit_and_admission', [(2783, 2967)],
       'Wal writer permit and admission: the writer capacity, the permit and key and the admission census, lines and waiters.'),
    ch('📡', 'wal-writer-signals-release-and-controllers', 'wal_writer_signals_release_and_controllers', [(2970, 3356)],
       'Wal writer signals, release and controllers: the writer lease and signal cells, the signal reservation, the release and its failure and the writer controllers with deferred fault waiters.'),
]
SPEC = dict(orig='ioc.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='ioc.rs'))
