from specs.lib import *
F = DB + '/🗄️storage/📁️fs/🦀️.rs'
INC = DB + '/🗄️storage/🧪️tests/🔬️fs-storage-directory-durability/🦀️.rs'
CH = [
    ch('🚦', 'lifecycle-state', 'lifecycle_state', [(13, 113)],
       'Fs lifecycle state: the io error mapping, the lifecycle phases, the full sync group and epochs, the durable directories and the lifecycle with its test state.'),
    ch('💾', 'durable-file-operations', 'durable_file_operations', [(115, 275)],
       'Fs durable file operations: the directory sync, the durable file sync and directory creation and the durable wal create, seal and delete.'),
    ch('🛤', 'paths-and-lease-files', 'paths_and_lease_files', [(280, 380)],
       'Fs paths and lease files: the open error, the safe component, the wal, snapshot, index, payload, catalog, lease, segment, marker, generation and run paths and the lease file reader and writer.'),
    ch('🧰', 'fs-executor', 'fs_executor', [(382, 555)],
       'Fs executor: the filesystem executor, its read state and its direct storage operations.'),
    ch('📨', 'fs-task-executor', 'fs_task_executor', [(557, 927)],
       'Fs task executor: the typed task executor implementation of the filesystem executor.'),
    ch('📁', 'fs-storage-backend', 'fs_storage_backend', [(929, 1248)],
       'Fs storage backend: the task helpers, the public filesystem storage, its drop and the storage trait implementations.'),
]
SPEC = dict(
    orig='fs.rs', file=F, nest=False, children=[], expect_missing=['mod fs_storage {', '}'],
    external=mounted(F, orig='fs.rs') + [INC],
    patch={'💽️fs-storage': [('include!("../🧪️tests/🔬️fs-storage-directory-durability/🦀️.rs");', 'include!("../../🧪️tests/🔬️fs-storage-directory-durability/🦀️.rs");'), ('use fs_storage_backend::{list, pages, read_root_fence};', 'use fs_storage_backend::read_root_fence;')]},
    inner=dict(mod='fs_storage', indent=4, folder='💽️fs-storage', nest=False, rebase_super=True,
               doc='//! 💽️ Filesystem storage module: the lifecycle, durable file operations, paths, executor and storage backend of the filesystem storage, with the shared imports of the former inline module. See [`super`].',
               children=CH),
)
