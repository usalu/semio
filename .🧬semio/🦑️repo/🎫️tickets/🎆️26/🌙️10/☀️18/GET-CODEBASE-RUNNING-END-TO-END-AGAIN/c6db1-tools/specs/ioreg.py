from specs.lib import *
F = DB + '/🗄️storage/📇️io-registry/🦀️.rs'
CH = [
    ch('📋', 'page-copy-hash-and-observed-writes', 'page_copy_hash_and_observed_writes', [(13, 233)],
       'Page copy, hash and observed writes: the page copy, owner copy, page hash, observed byte writes and list transfer futures.'),
    ch('🧱', 'platform-buffers', 'platform_buffers', [(235, 529)],
       'Platform buffers: the platform buffer arena, its prepare, copy and close futures and the backend kind.'),
    ch('📊', 'census', 'census', [(531, 653)],
       'Census: the executor census counters and `DbIoCensusV1`.'),
    ch('🗒', 'backend-registry-and-rejections', 'backend_registry_and_rejections', [(656, 1064)],
       'Backend registry and rejections: the backend registry, the rejected backend registry, rollback reservation and rejected backend parking.'),
    ch('🎛', 'backend-registration-and-executor', 'backend_registration_and_executor', [(1066, 1415)],
       'Backend registration and executor: backend registration, credit, control, execution, close steps and retirement turns.'),
    ch('🧹', 'task-faults-and-close', 'task_faults_and_close', [(1417, 1816)],
       'Task faults and close: task faults, cleanup, lost owner close, the maintenance turn and the task close step.'),
    ch('🎮', 'writer-controller', 'writer_controller', [(1819, 1966)],
       'Writer controller: the retry limits and the WAL writer controller.'),
]
SPEC = dict(orig='ioreg.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='ioreg.rs'))
