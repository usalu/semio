from specs.lib import *
AV = M + '/🎭️actor/📖️vocabulary'
PRE = close_regions({549: ['//#endregion 🪪️JobBridge'], 1490: ['//#endregion 🗂️ActorRecord']}, ['//#endregion 🧩️ShardTable'])
CH = [
    ch('🆔', 'identities-and-budgets', 'identities_and_budgets', [(13, 365)],
       'Identities and budgets: the job replay request and digests, the package, actor and kind identities, the lanes and the lane budgets with their defaults.'),
    ch('🧵', 'job-operation-records', 'job_operation_records', [(370, 546)],
       'Job operation records: the job operation, checkpoint, commit candidate, step outcome, turn and publication records.'),
    ch('✉', 'envelope-and-payload', 'envelope_and_payload', [(552, 775)],
       'Envelope and payload: the window identity, the origin, the payload, the coalesce key and the envelope.'),
    ch('📬', 'turn-result-and-mailbox', 'turn_result_and_mailbox', [(779, 1110)],
       'Turn result and mailbox: the turn status, usage and result, the backpressure and the bounded mailbox.'),
    ch('🚑', 'capability-and-failure-policy', 'capability_and_failure_policy', [(1114, 1439)],
       'Capability and failure policy: the capability grant with its intersection and the failure signals, stages, state and escalation with the backoff and quarantine durations.'),
    ch('🧩', 'actor-status-and-shards', 'actor_status_and_shards', [(1444, 1563)],
       'Actor status and shards: the actor lifecycle status, the shard kind and identity and the shard count clamps.'),
]
SPEC = dict(orig='av.rs', file=AV + '/🦀️.rs', children=CH, wiring='block', external=mounted(AV + '/🦀️.rs', orig='av.rs'),
            pre=PRE, region_fix=3, crate_files=[])
