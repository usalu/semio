from specs.lib import *
HE = PL + '/🖥️host/⚡️effects'
CH = [
    ch('🆔', 'actor-registries-and-envelope-injector', 'actor_registries_and_envelope_injector', [(55, 263)],
       'Actor registries and envelope injector: the trace id allocator, the actor scope registry, the capability revocation registry, the lane deadlines, the envelope injector seam and the completion mailbox.'),
    ch('🧾', 'completion-sink-and-fault-encoding', 'completion_sink_and_fault_encoding', [(267, 378)],
       'Completion sink and fault encoding: the envelope completion sink with its completion sink implementation and the fault, event and completed envelope encoders.'),
    ch('🌉', 'router-effect-handler-and-jobs', 'router_effect_handler_and_jobs', [(382, 781)],
       'Router effect handler and jobs: the router effect, its handler, the dynamic, fault and complete effect jobs, the effect wallet, the router effect job runner and the unwired handler.'),
    ch('💾', 'storage-capabilities-backbone-and-metrics', 'storage_capabilities_backbone_and_metrics', [(785, 944)],
       'Storage, capabilities, backbone and metrics: the storage backend, the capability checker, the backbone transport and registry and the effect metrics recorders.'),
    ch('🎛', 'async-effect-executor', 'async_effect_executor', [(948, 1425)],
       'Async effect executor: the async services, the dispatch context and report, the executor with its effect classification and dispatch, the http response wire and the storage operations.'),
]
SPEC = dict(orig='he.rs', file=HE + '/🦀️.rs', children=CH, wiring='block', external=mounted(HE + '/🦀️.rs', orig='he.rs'))
