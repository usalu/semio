# P8zf Independent Global Payload-Authority Acceptance Audit

## Verdict

**REJECT — P0.** The latest corrective pass makes the CAD generation domain
coherent across its declared surfaces, and the prior source-level repairs for
payload ownership, cold Puzzle3d restoration, bounded contribution ingestion,
and Note/Layout terminology remain present. It does not make the persisted CAD
preview generation an exact increment-only stamp for *every* engagement
checkpoint transition: two live command paths clear the checkpoint while
bypassing the only increment-and-identity helper.

No production source or ticket metadata was modified by this audit.

## Audit Basis

Read in full before inspecting the current worktree:

- `AGENTS.md`.
- `📓️p8t-independent-remaining-tools-global-audit.md`.
- `📓️p8w-global-payload-authority-repair.md`.
- `📓️p8z-independent-global-payload-authority-audit.md`.
- `📓️p8za-global-payload-authority-repair.md`.
- `📓️p8zb-independent-global-payload-authority-final-audit.md`.
- `📓️p8zc-global-payload-descriptor-coherence-repair.md`.

The review was read-only and source-first. It covered current CAD, Block,
Process, Sourcing, Note, Layout, and Puzzle source plus every changed JSON
descriptor and the changed Rust/Proto/GraphQL/TypeScript descriptor leaves.

## P0 Finding

| ID | Finding | Exact current evidence | Required repair |
| --- | --- | --- | --- |
| P0-01 | CAD has two production engagement-checkpoint transitions that do not advance the persisted generation or replace its operation identity. Consequently the previous and cleared checkpoint can carry an equal `CadPreviewStamp`, breaking the claimed exact monotonic freshness/ABA contract. | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️utility/🦀️component.rs:20-35` clears `runtime.engagement_session` at `:28`, obtains a config with `cad_config_from_runtime` at `:30`, and emits it directly at `:32`; the handler deliberately discards `CadDispatchCtx`. `…/🎮️commands/📥️io/🦀️component.rs:43-48` clears the same session at `:45` and emits `snapshot_of(&runtime, cfg.snapshot)` at `:47`, also discarding its context. In contrast, the only transition authority, `…/✏️editor/🦀️component.rs:356-367`, detects session change, requires the operation identity, and performs the checked increment. | Make a single mandatory session-transition snapshot helper which accepts `CadDispatchCtx`, detects every `engagement_session_json` change, advances the checked generation, and persists the exact operation identity. Route active-utility switch and import through it; remove or make inaccessible the bypassing construction path. Add fixtures that assert a strictly newer stamp after each clear and assert that both paths fail closed at `CAD_PREVIEW_GENERATION_MAX` or without public operation context. |

This is not a cosmetic concern: an in-flight preview tagged with the old stamp
is indistinguishable by `CadGesturePreview::is_fresher_than` from a consumer
that retained the direct-cleared config's equal stamp. The P8za/P8zc claim that
every changed engagement checkpoint has a persisted checked generation is
therefore not established by the current production routes.

## Re-Attack Results That Hold At Source Level

### CAD Domain And Operation Identity

- The actual preview-generation field is `i32` in both runtime and persisted
  config. `CAD_PREVIEW_GENERATION_MAX` is `i32::MAX`, and JSON-backed
  deserialization rejects negative input in
  `…/🎚️config/🦀️component.rs:14-28,113-117`.
- The successful helper path uses `checked_add(1)` and returns a typed conflict
  at exhaustion (`…/✏️editor/🦀️component.rs:356-367`).
- The descriptor leaves agree on `i32`/Proto `int32`/GraphQL `Int`/TypeScript
  `number` with documented `0..=2147483647`, and JSON Schema declares
  `integer`, `minimum: 0`, `maximum: 2147483647`:
  `…/🎚️config/🧬️schema/{🦀️component.rs,🛰️component.proto,🔗️component.graphql,🟦️component.ts,🔣️component.json}`.
- The source fixture at `…/✏️editor/🦀️component.rs:2280-2307` covers maximum
  JSON round trip, maximum-plus-one rejection, and direct helper exhaustion.
  It does **not** cover either bypass identified above.
- `CadPreviewOperationIdentity` still carries app instance, parent document,
  operation id, operation generation, and canonical base revision
  (`…/✏️editor/🦀️component.rs:764-783`), and freshness still requires exact
  identity plus a larger generation (`:786-801`). No preview-specific
  `u64`/`uint64`, `CAD_PREVIEW_SEQ`, or preview `DefaultHasher` path remains.
  The unrelated CAD child-handle hash at `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️component.rs:119-122` is not preview state.

### Puzzle, Bounded Inputs, And Owned Payload

- Puzzle3d serializes a full `FillWorkerState` including job identity, scene,
  raw mesh sources, fill checkpoint, cursors, revision/generation, observation,
  and emitted checkpoint (`…/🧩️puzzle/…/⏳️precompute/🦀️component.rs:63-83,
  822-848`). Restore rejects oversized bytes, enforces mesh/count/URL bounds,
  rebuilds collision meshes and fill state, and verifies operation/generation
  before driving (`:851-910,1001-1035`). The cold-reopen and ABA source fixtures
  remain at `:1484-1521`.
- Sourcing scans the byte/depth/string/cardinality envelope before
  `CurateSnapshot` deserialization (`…/🪵️sourcing/…/set-artifact-json/🦀️component.rs:18-25`).
  Process and Sourcing also scan outer and nested contribution JSON before
  typed decode (`…/🏭️process/…/✏️editor/🦀️component.rs:678-778`; `…/🪵️sourcing/…/🧬️schema/🦀️component.rs:576-675`).
- The bounded contribution implementations hold input-derived identifiers in
  owned `String` and the exact Process/Sourcing scan found no `Box::leak`,
  `into_boxed_str`, or `leak_str`.
- The exact target-tree scan found no mutable `thread_local!`,
  `OnceLock<Mutex<_>>`, `LazyLock<Mutex<_>>`, or `static mut`. The remaining
  Puzzle `OnceLock`/`LazyLock` matches are immutable descriptors/examples or
  the `Instant` clock origin; app-local `Mutex` fields are not process globals.
- Block3d `vortex_kind_extra`, Process `stock_payload`/`step_payloads`, and
  Sourcing `stock_extra` remain snapshot/diff-owned. The Process fields are
  present in runtime conversions, Rust snapshot/diff models, text/binary codecs,
  and Proto snapshot/diff descriptors.

### Note, Layout, And Descriptors

- Exact whole-tree scans across Note and Layout returned no
  `working-scene`, `working_scene`, `scratch-cache`, `scratch_cache`,
  `cache-miss`, `cache_miss`, `uncached`, `never-cached`, or `WorkingScene`
  identifier/test/comment match.
- All 9 changed JSON descriptors parsed successfully with `JSON.parse`.
  Their accompanying changed Rust/Proto/GraphQL/TypeScript descriptor leaves
  were inspected for the migrated durable fields. This is static syntax and
  source-coherence evidence only; it is not generated-descriptor discovery.

## Static Commands

All commands were read-only/static.

```text
bun JSON.parse each changed JSON descriptor
=> 9 parsed

bun ./📜️script.ts verify interactivity
=> exit 0; DENY mode clean in its declared four UI roots

bun ./📜️script.ts verify interactivity tool-jobs --format json
=> exit 1 (expected repository-wide residual): 34 global payload candidates,
   12 framework-reserved routes, and 875 live registrations pending disposition

git diff --check -- <seven repaired plugin trees>
=> exit 0

exact Note/Layout stale-vocabulary scan
=> clean

exact Process/Sourcing permanent-leak scan
=> clean

exact target mutable-global scan
=> clean
```

The repository-wide tool-job result is not the basis for this cohort rejection;
P0-01 is a direct production route analysis.

## Unrun Mandatory Gates

No Cargo command, compilation/type/borrow/Send gate, build, unit/integration
test execution, native/release runtime, worker launch, Wasm build/execution,
descriptor discovery/regeneration, cache mutation, git mutation, or ticket
metadata operation ran.

After P0-01 is repaired, run native and release coverage for CAD's utility
switch/import/session-clear paths at normal and maximum generation, including
stale-preview rejection, ABA, two-app isolation, and cold reopen. The existing
Puzzle3d first-tick/checkpoint/cold-worker/two-operation tests and the
Process/Sourcing exact envelope tests also require native and Wasm execution.
The independent repository-wide fail-closed tool-job ledger remains required
before Phase 8 can close.
