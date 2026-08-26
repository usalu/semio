# Generic Reducer False-Green Hardening

## Result

The permanent all-tool gate now rejects a `TypedCommandFullOperationJob` that calls the generic `A::handle` or `A::ephemeral` reducer inside one worker grant without an exact bounded-first-step authority.

## Reason

Moving a reducer call onto the worker does not by itself make it resumable. The generic reducer can still traverse a document-sized snapshot, allocate a complete mutation/event collection, and exceed the 8 ms ceiling before returning. Input byte and decoded-item caps alone do not bound that work. The architecture plan nevertheless permits a genuinely small tool to finish in its first step, so the guard distinguishes that exact case from an unproved one-shot reducer.

## Gate Change

`toolJobFullOperationBounded` now treats direct generic reducer calls as monolithic unless the worker enforces `ToolExecutionShape::BoundedFirstStep`, work-unit and step-time limits, and the production constructor is selected through the compiler-bound `QualifiedToolProof::Bounded` branch. Hostile and positive self-tests prove both rejection and the explicit small-tool exception.

The focused gate remains green:

```text
[verify interactivity tool-jobs] self-tests=365 clean.
```

## Consequence

The shared route remains correctly fail-closed. Production acceptance requires an app-owned resumable reducer job or an exact owner-local bounded-first-step proof for every reachable command. Cache refresh, root acquisition, publication, and outcome retirement remain separate unresolved production gates.
