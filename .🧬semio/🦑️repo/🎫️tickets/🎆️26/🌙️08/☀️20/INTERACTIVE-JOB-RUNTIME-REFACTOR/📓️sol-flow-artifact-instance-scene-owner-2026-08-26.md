# Flow Artifact-Instance Scene Owner

## Outcome

`FLOW_SCRATCH` is deleted. The former thread-local `RefCell<HashMap<String, FlowWorkingScene>>` made a durable, content-derived child ID the process authority for widgets, synapses, and layout. A separate app instance or stale snapshot could therefore resolve state through ambient process identity.

Each `FlowContentChild` now retains one typed immutable `Arc<FlowWorkingScene>` through the private, serialization-skipped `ArtifactChild::local_owner` seam. Clones retain the same exact owner. Independently minted handles remain isolated even under deliberate durable ID reuse. `cache_flow_content` now replaces only the passed mutable child handle's owner; it has no process side effect. The restart test no longer clears process state because none exists.

## Schema-First Hostile Law

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🧪️fixtures/flow-scene-owner.schema.json`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🧪️fixtures/flow-scene-owner-law.json`

The five bounded cases cover clone pointer identity, cross-instance isolation under exact durable ID collision, stale-A/reused-B/stale-A ABA, third-party serde wire omission/unresolved decode, and one-slot bounded close through a `Weak` reclamation witness. Ajv 2020 validates the fixture with exit `0`; evidence is `🧪️sol-flow-scene-owner-ajv-2026-08-26.txt`.

## Fresh Source-Only Evidence

- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`: exit `0`, 468 clean.
- Full JSON verifier: expected aggregate exit `1`; `globalPayloadStores=29`, exactly down from 30; no Flow row remains.
- `rg` finds no `FLOW_SCRATCH`, `thread_local!`, or matching `RefCell<HashMap<...>>` in the Flow artifact component.
- `rustfmt --edition 2021` and `git diff --check` on touched Flow source: exit `0`.
- Evidence: `🧪️sol-flow-scene-owner-source-gate-2026-08-26.txt`, `📊️sol-flow-artifact-instance-scene-owner-2026-08-26.json`.

## Pending Compiler Gate

No Cargo or Nx command was started while the compiler lane remained reserved. The focused Flow owner tests and native/Wasm checks remain queued; runtime acceptance is not claimed.
