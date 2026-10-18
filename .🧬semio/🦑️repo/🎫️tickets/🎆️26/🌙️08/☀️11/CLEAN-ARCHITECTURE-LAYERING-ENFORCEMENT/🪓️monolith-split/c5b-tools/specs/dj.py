from specs.lib import *
DJ = PL + '/📜️declaration/👷️job'
CH = [
    ch('🧾', 'bounded-proof-and-factory-registry', 'bounded_proof_and_factory_registry', [(55, 227)],
       'Bounded proof and factory registry of the tool job contract: the bounded first step proof, the publication lane and contract, the owned tool job factory, the registration and the factory registry.'),
    ch('🩺', 'typed-operation-faults-and-raw-input', 'typed_operation_faults_and_raw_input', [(229, 429)],
       'Typed operation faults and raw input of the tool job contract: the bounded fault page, the completion value and result lane, the authoring effect, the completion and raw input records and the completion rejections.'),
    ch('🎯', 'completion-and-owned-context', 'completion_and_owned_context', [(431, 710)],
       'Completion and owned context of the tool job contract: the tool completion operations, the owned job snapshots and context, the context identity digest and the context operations.'),
    ch('🧵', 'reserved-tool-job', 'reserved_tool_job', [(712, 865)],
       'Reserved tool job of the tool job contract: the owned job request, the reserved input and request, the reserved job trait and state and the reserved tool job with its interactive job and drop implementations.'),
    ch('🔎', 'reserved-outcome-witnesses', 'reserved_outcome_witnesses', [(871, 1061)],
       'Reserved outcome witnesses of the tool job contract: the owner witness, the reserved job outcomes and kinds, the receive and close helpers, the bounded fault text and the reducer fault detail.'),
]
SPEC = dict(orig='dj.rs', file=DJ + '/🦀️.rs', children=CH, wiring='block',
            external=[DJ + '/🎟️birth/🦀️.rs', DJ + '/🎟️birth/🧪️tests/🦀️.rs', DJ + '/♻️alias/🦀️.rs', DJ + '/🧪️inference-admission/🦀️.rs', DJ + '/🧪️inference-close/🦀️.rs'])
