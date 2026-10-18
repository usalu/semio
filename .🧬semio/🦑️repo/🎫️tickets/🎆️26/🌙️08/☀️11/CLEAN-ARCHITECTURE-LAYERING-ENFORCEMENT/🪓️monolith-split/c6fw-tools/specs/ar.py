from specs.lib import *
AR = M + '/🎭️actor'
H = 'impl Kernel {'
B = [1594]
CH = [
    ch('🗂', 'actor-record-and-shard-table', 'actor_record_and_shard_table', [(203, 461)],
       'Actor record and shard table: the actor record with its shard map conversions and the shard table that pins actors to shards.'),
    ch('⏱', 'scheduler-and-scene', 'scheduler_and_scene', [(465, 828)],
       'Scheduler and scene: the scheduled actor, the decision and turn grant records, the deficit round robin scheduler and the scene snapshot and store.'),
    ch('📈', 'runtime-metrics', 'runtime_metrics', [(832, 1132)],
       'Runtime metrics: the actor, shard and kernel metrics with their samples, the runtime metrics snapshot and the publish interval check.'),
    ch('🚚', 'shard-transport-and-kernel-types', 'shard_transport_and_kernel_types', [(1137, 1308)],
       'Shard transport and kernel types: the shard transport trait with its thread transport, the activation event, the kernel error, the actor meta and the kernel struct.'),
    ch('🏛', 'kernel-core', 'kernel_core', [(1310, 1594)],
       'Kernel core: the kernel constructor, activation, submission, turn completion, suspension, exclusivity, scene, metrics and snapshot operations.'),
    ch('🌿', 'kernel-extension-activation', 'kernel_extension_activation', [(1596, 1786)],
       'Kernel extension activation: the pinned activation, capabilities, extension links and the subtree deactivation, kill, suspend and resume cascades.'),
]
SPEC = dict(orig='ar.rs', file=AR + '/🦀️.rs', children=CH, wiring='block', external=mounted(AR + '/🦀️.rs', orig='ar.rs'),
            pre=split_impl(H, B), expect_extra=split_extra(H, B), crate_files=[])
