from specs.lib import *
HP = PL + '/🖥️host/🔀️post-turn-relay'
H = 'impl GuestRelayMountedRegistry {'
B = [1639]
def S(n):
    return n + 3 if n >= 1639 else n
CH = [
    ch('🏊', 'pool-future-requests-and-completion-slot', 'pool_future_requests_and_completion_slot', [(4, 368)],
       'Pool future, requests and completion slot: the relay job budget, the retained-waker pool future, the relay requests and completions, the drive signal and the completion slot.'),
    ch('🎟', 'instance-slots-leases-and-cancel-helpers', 'instance_slots_leases_and_cancel_helpers', [(370, 574)],
       'Instance slots, leases and cancel helpers: the guest instance slot, its lease with the drop, the quarantine, cleanup pending and cancel helpers.'),
    ch('📡', 'relay-attempts-and-publication', 'relay_attempts_and_publication', [(576, 861)],
       'Relay attempts and publication: the relay attempt, the request run, the failure recovery and the relay publication with its drop and close receive.'),
    ch('🥶', 'cold-relay-job', 'cold_relay_job', [(866, 1093)],
       'Cold relay job: the guest cold relay job, its interactive job implementation and its drop.'),
    ch('🔬', 'lifecycle-probe-and-refused-source', 'lifecycle_probe_and_refused_source', [(1098, 1260)],
       'Lifecycle probe and refused source: the relay lifecycle probe control and job and the refused source with its interactive job implementation.'),
    ch('🧾', 'mounted-session-types-and-registry-struct', 'mounted_session_types_and_registry_struct', [(1095, 1096), (1262, 1288), (1304, 1411)],
       'Mounted session types and registry struct: the mounted slots and reaper fallback, the session admission, the mounted owner, the wake authority and custody, the mounted session, its stage, kind and lifecycle enums, the output, terminal, close and slot and the registry struct.'),
    ch('🗂', 'mounted-registry-lifecycle', 'mounted_registry_lifecycle', [(1413, 1639)],
       'Mounted registry lifecycle: the registry constructor, the pool, the original wake checkout, collect, admit and close, the reservation, mount, detach, reaper and the detached takes.'),
    ch('⚙', 'mounted-registry-pump-and-reaper', 'mounted_registry_pump_and_reaper', [(1640, S(1844)), (S(1846), S(1949))],
       'Mounted registry pump and reaper: the pump close, the pump with its outcome and the outcome finish, the relay step, the outcome page read, the mounted reaper and the mounted future.'),
    ch('🧪', 'lifecycle-trace-exercises', 'lifecycle_trace_exercises', [(S(1951), S(2193))],
       'Lifecycle trace exercises: the lifecycle trace and projection, the barrier, wait and probe session helpers and the abandoned, live, stale and capacity trace exercises.'),
    ch('🔌', 'plugin-instance-handle', 'plugin_instance_handle', [(S(2195), S(2370)), (S(2378), S(2391))],
       'Plugin instance handle: the plugin instance handle with its debug implementation, the relay wake authority test constructor and the scripted relay release wait.'),
]
SPEC = dict(orig='fpr.rs', file=HP + '/🦀️.rs', children=CH, wiring='block', external=mounted(HP + '/🦀️.rs', orig='fpr.rs'),
            pre=split_impl(H, B), expect_extra=split_extra(H, B))
