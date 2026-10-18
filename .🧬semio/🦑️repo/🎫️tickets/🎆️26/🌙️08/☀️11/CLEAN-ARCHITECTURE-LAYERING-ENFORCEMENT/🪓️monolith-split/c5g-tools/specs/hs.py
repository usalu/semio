from specs.lib import *
HS = PL + '/🖥️host/🧵️shard'
H = 'impl ShardLoop {'
B = [1590, 1934]
CH = [
    ch('📨', 'shard-frame-and-budget-bridge', 'shard_frame_and_budget_bridge', [(57, 280)],
       'Shard frame and budget bridge: the shard frame wire enum, the issued shard turn and retained recipient, the grant to budget conversions and the actor turn result mapping.'),
    ch('📊', 'shard-outcome-and-loop-state', 'shard_outcome_and_loop_state', [(282, 515)],
       'Shard outcome and loop state: the shard outcome with its wire pack, the shard loop record and the job authority.'),
    ch('📄', 'fixed-replay-seed', 'fixed_replay_seed', [(518, 830)],
       'Fixed replay seed: the replay page and slot limits, the replay test authority, the fixed replay seed page and seed, the replay phases, the mounted replay seed, the spawn refusals and the replay abi buffer.'),
    ch('🚦', 'deferred-owner-ring', 'deferred_owner_ring', [(833, 1030)],
       'Deferred owner ring: the shard drive, the deferred item and byte limits, the owner keys, the admission limit, the fixed owner ring, the cancel cursor, the deferred authority and the frame admission and overflow.'),
    ch('🏗', 'shard-loop-construction-and-replay-drive', 'shard_loop_construction_and_replay_drive', [(1033, 1590)],
       'Shard loop construction and replay drive: the shard loop constructor, the granted budgets, the authority queues and the replay seed opportunities, refusal, begin, close and drive.'),
    ch('🌀', 'shard-loop-registry-and-pump', 'shard_loop_registry_and_pump', [(1592, 1937)],
       'Shard loop registry and pump: the registered actors, the pump entry points, the terminal frame and completion takers, the frame epoch and the primed pump.'),
    ch('🎬', 'shard-loop-frame-turns-and-job-control', 'shard_loop_frame_turns_and_job_control', [(1939, 2395)],
       'Shard loop frame turns and job control: the original mesh and event frames, the turn execution, frame consumption and validation, envelope dispatch and the suspend, resume, cancel, job turn and outcome sends.'),
    ch('🔬', 'replay-lifecycle-and-fault-helpers', 'replay_lifecycle_and_fault_helpers', [(2397, 2487)],
       'Replay lifecycle and fault helpers: the replay lifecycle projection and trace exerciser and the turn fault, stage and start job fault helpers.'),
    ch('🛰', 'shard-transports-and-test-doubles', 'shard_transports_and_test_doubles', [(2490, 2680)],
       'Shard transports and test doubles: the shared thread transport, the shard transports enum with its conversions, the loopback transport and the fixture identity and probe helpers.'),
]
SPEC = dict(orig='hs.rs', file=HS + '/🦀️.rs', children=CH, wiring='block', pre=split_impl(H, B), expect_extra=split_extra(H, B),
            external=mounted(HS + '/🦀️.rs', orig='hs.rs'))
