# Store Fixture Source Contract Findings

The runtime report was read through its final Kernel30 diagnostic. The assigned cohort contains Demo3, Severity4, Timestamped1, Validated1, Lossy1, and the presence Value/Noop implementations.

## Timestamp Forwarding Gap

Mutation::timestamp exists in the lower protocol and TimestampedMutation currently returns HLT(logical=0, physical_ms=payload). MutationKind has no timestamp hook, and the Mutations derive does not forward it. A transparent derived Timestamped aggregate would silently lose the timestamp fixture semantics without a shared leaf timestamp hook/derive forwarder. This agent cannot edit SPR command or the derive under the bounded Store assignment; root owns that shared decision.

## Presence Noop

The Noop fixture is only selected as PresenceStore's generic mutation type for read/retirement tests, except one closed-store rejection probe that attempts Noop. Replace those generic uses with the genuine SetValue direct-leaf aggregate and make the closed-store probe attempt SetValue. An idle test has no dispatched operation; it needs no synthetic Noop descriptor or uninhabited aggregate.

## Existing Demo Deletion Sentinel

DemoSnapshot has n:i32. DemoMutation::DeleteN writes i32::MIN; SetN/BumpN treat that numeric value as target absence. This is preexisting deliberate conflict-fixture behavior, but genuine absence would require Option<i32> (or explicit presence) and a restore/create operation: the present inverse SetN is rejected against the deleted state. This would affect the Demo snapshot, two retained preparation fixtures, and many tests. Parent direction is needed on whether to preserve this existing fixture encoding in this metadata cohort or expand to true absence now; no production behavior change is required either way.

