from specs.lib import *
F = U + '/🎬️action/🦀️.rs'
CH = [
    ch('🧭', 'intent-address-and-sequencer', 'intent_address_and_sequencer', [(15, 182)],
       'Intent address and sequencer: the action capacities, the intent fields, address, bindings, command, stale check and the intent sequencer.'),
    ch('🧱', 'bounded-action-model', 'bounded_action_model', [(185, 446)],
       'Bounded action model: the bounded faults, the flat action value model, the queue receipts, the retained string action and the `BoundedAction` with its builder state.'),
    ch('🏗', 'bounded-action-builder', 'bounded_action_builder', [(448, 684)],
       'Bounded action builder: the `BoundedActionBuilder` that records the flat action values within the byte and node capacities.'),
    ch('🎟', 'claims-and-reservations', 'claims_and_reservations', [(686, 934)],
       'Claims and reservations: the action and batch reservations, the claims and claimed drafts, the prepared batches and their reservation operations.'),
    ch('📬', 'action-queue', 'action_queue', [(936, 1278)],
       'Action queue: the `BoundedActionQueue` with the intent admission and the queue push, claim and drain operations.'),
]
SPEC = dict(orig='action.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='action.rs'))
