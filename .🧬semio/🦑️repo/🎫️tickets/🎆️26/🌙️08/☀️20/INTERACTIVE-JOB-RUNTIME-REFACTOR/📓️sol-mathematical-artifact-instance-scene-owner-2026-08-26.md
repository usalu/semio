# Mathematical Artifact-Instance Scene Owner

## Outcome

`MATH_SCRATCH` is deleted. The former thread-local `RefCell<HashMap<String, MathematicalWorkingScene>>` used a content-derived child ID as process authority for graph and geometry payloads. One app could therefore replace what another app or a stale snapshot resolved from the same ID.

The three composed children in one `MathematicalSnapshot` now retain one immutable `Arc<MathematicalWorkingScene>` through the private, serialization-skipped `ArtifactChild<S>::local_owner` seam. Each snapshot triple owns exactly three fixed references. A clone retains that owner; an independently materialized triple is isolated even under deliberately reused durable IDs. No registry, TLS, process global, compatibility path, or exemption remains.

## Framework Ownership Seam

`ArtifactChild<S>` now provides type-checked `with_local_owner`, `set_local_owner`, and `local_owner` behavior over a private `Arc<dyn Any + Send + Sync>`. JSON, DSL, pack, debug, and equality continue to expose only durable `child_id` and `target`. Writer's earlier local text is implemented through the same private owner seam while preserving its `Arc<str>` API.

This is the narrowest usable owner because Mathematical mutation, inverse, serializer, inference, and fixture functions receive only a snapshot, not an app or retained-operation context. The owner dies with the last exact snapshot/child handle; it cannot outlive itself in a process registry.

## Schema-First Hostile Law

Inputs:

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🧪️fixtures/mathematical-scene-owner.schema.json`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🧪️fixtures/mathematical-scene-owner-law.json`

The bounded five-case fixture covers triple pointer identity, cross-instance isolation under exact ID collision, stale-A/reused-B/stale-A ABA, third-party serde wire omission, and fixed three-slot close. Close retains a `Weak` witness, drops the snapshot's exactly three owners, verifies only the explicit witness remains, then verifies final reclamation.

## Fresh Source-Only Evidence

- Ajv 2020: exit `0`, `valid=true`, `cases=5`, `maximumCases=5`, `ownedSlots=3`. Log: `🧪️sol-math-scene-owner-ajv-2026-08-26.txt`.
- `rustfmt --edition 2021 --check` on the shared store and Mathematical component: exit `0`. Log: `🧪️sol-math-scene-owner-rustfmt-check-2026-08-26.txt`.
- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`: exit `0`, `self-tests=468 clean`. Log: `🧪️sol-math-scene-owner-tool-jobs-self-test-2026-08-26.txt`.
- Full JSON verifier: expected aggregate exit `1`; `globalPayloadStores=31` (Writer checkpoint `32`), `MATH_SCRATCH=0`, `WRITER_SCRATCH=0`, exemptions unchanged `3`, bounded `217`, remaining `719`, self-tests `468`. Evidence: `📊️sol-math-scene-owner-tool-jobs-2026-08-26.json` and `🧪️sol-math-scene-owner-tool-jobs-stderr-2026-08-26.txt`.

## Pending Runtime Gate

No Cargo or Nx command was started while the Framework executor owned the compiler lane. The focused Mathematical fixture test, Writer fixture test, and their native library checks remain queued. Runtime acceptance is not claimed yet.
