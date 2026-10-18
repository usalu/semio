from specs.lib import *
F = DB + '/🗄️storage/🚦️io-driver/🦀️.rs'
CH = [
    ch('🔓', 'open-rejection-and-backend-lifecycle', 'open_rejection_and_backend_lifecycle', [(24, 224)],
       'Open rejection and backend lifecycle: the storage open rejection and the backend registration, retirement and close.'),
    ch('🏃', 'task-allocation-and-async-driver', 'task_allocation_and_async_driver', [(226, 655)],
       'Task allocation and async driver: task allocation, job submission, retries, one driver step and the async driver poll.'),
    ch('🎟', 'task-operation-lease-and-admission', 'task_operation_lease_and_admission', [(657, 974)],
       'Task operation lease and admission: the task operation, the async task lease and the admitted submission.'),
    ch('🔁', 'task-operation-futures', 'task_operation_futures', [(976, 1303)],
       'Task operation futures: the task operation impls, the retirement wait and the test pool helpers.'),
    ch('📝', 'wal-writer-table', 'wal_writer_table', [(1306, 1683)],
       'WAL writer table: the writer guard, the writer table and the file writer guard.'),
    ch('🛂', 'wal-writer-admission', 'wal_writer_admission', [(1685, 1853)],
       'WAL writer admission: the writer admission place, wait and the controller helpers.'),
]
SPEC = dict(orig='iodrv.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='iodrv.rs'))
