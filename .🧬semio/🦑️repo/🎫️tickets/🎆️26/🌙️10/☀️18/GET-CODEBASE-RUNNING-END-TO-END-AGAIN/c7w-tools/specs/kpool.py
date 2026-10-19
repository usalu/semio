from specs.lib import *
F = RW + '/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs'
c = Cut(F, 'kpool.rs').impl('impl KernelPoolState {', [275, 597, 984, 1397], at=6)
c.child('🏗', 'pool-construction-and-product-replay', 'pool_construction_and_product_replay', [(6, 274)],
        'Pool construction and product replay: the `KernelPoolState` constructor, the product replay mount and the product replay advance.')
c.child('📼', 'job-replay-and-command-maintenance', 'job_replay_and_command_maintenance', [(275, 596)],
        'Job replay and command maintenance: the job replay request and capture, the replay and command maintenance steps, the fault close and the job progress acknowledgement.')
c.child('🧩', 'app-lifecycle-and-realm-close', 'app_lifecycle_and_realm_close', [(597, 983)],
        'App lifecycle and realm close: the captured job progress, the app creation, the extension activation and the app destroy and realm close steps.')
c.child('🔄', 'exchange-and-turn-settling', 'exchange_and_turn_settling', [(984, 1396)],
        'Exchange and turn settling: the exchange, the command exchange, the turn run and settle, the cancelled turn retirement, the serving compile and the reserved job admission.')
c.child('🚀', 'turn-dispatch-and-pool-runner', 'turn_dispatch_and_pool_runner', [(1397, 1960)],
        'Turn dispatch and pool runner: the single turn run, the turn dispatch and result application, the ui patch, the retained surface steps, the maintenance yield, the pool runner and the pool futures.')
SPEC = c.spec()
