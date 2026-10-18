from specs.lib import *
F = DB + '/🗄️storage/🪶️sqlite/🦀️.rs'
INC = DB + '/🗄️storage/🪶️sqlite/🧪️tests/🔬️sqlite-storage-unit/🦀️.rs'
CH = [
    ch('🧬', 'schema-and-connection', 'schema_and_connection', [(13, 123)],
       'Sqlite schema and connection: the schema statements, the limits and statements, the error and integer conversions, the connection initialization, the wal writer guard and the executor type.'),
    ch('🧰', 'sqlite-executor', 'sqlite_executor', [(125, 395)],
       'Sqlite executor: the direct storage operations of the sqlite executor.'),
    ch('📨', 'sqlite-task-executor', 'sqlite_task_executor', [(399, 834)],
       'Sqlite task executor: the typed task executor implementation of the sqlite executor.'),
    ch('🪶', 'sqlite-storage-backend', 'sqlite_storage_backend', [(838, 1117)],
       'Sqlite storage backend: the public sqlite storage, the task helpers, its drop and the storage trait implementations.'),
]
SPEC = dict(
    orig='sqlite.rs', file=F, nest=False, children=[], expect_missing=['mod sqlite_storage {', '}'],
    external=mounted(F, orig='sqlite.rs') + [INC],
    patch={'💽️sqlite-storage': [('include!("🧪️tests/🔬️sqlite-storage-unit/🦀️.rs");', 'include!("../🧪️tests/🔬️sqlite-storage-unit/🦀️.rs");'), ('use sqlite_storage_backend::{length, pages};\n', '')]},
    inner=dict(mod='sqlite_storage', indent=4, folder='💽️sqlite-storage', nest=False, rebase_super=True,
               doc='//! 💽️ Sqlite storage module: the schema and connection, the executor, the task executor and the storage backend of the sqlite storage, with the shared imports of the former inline module. See [`super`].',
               children=CH),
)
