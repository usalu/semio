from specs.lib import *
F = DB + '/🗄️storage/🗃️memory/🦀️.rs'
CH = [
    ch('🧩', 'memory-executor-types', 'memory_executor_types', [(28, 133)],
       'Memory executor types: the in-memory wal segment with its credit, the owners, the executor, its cursor and default and the fixed none box.'),
    ch('🧰', 'memory-executor', 'memory_executor', [(135, 331)],
       'Memory executor: the direct storage operations of the in-memory executor, the page owner close step, the lock helper and the cursor index.'),
    ch('📨', 'memory-task-executor', 'memory_task_executor', [(333, 863)],
       'Memory task executor: the typed task executor implementation of the in-memory executor.'),
    ch('🪣', 'memory-storage-backend', 'memory_storage_backend', [(865, 1155)],
       'Memory storage backend: the public in-memory storage, its task helpers, its drop and the storage trait implementations.'),
]
SPEC = dict(orig='mem.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='mem.rs'))
