# P8 Whole-Buffer Ingress Wave Order — 2026-08-24

## Verdict

The current production census is **12 occurrences = one shared fail-closed definition plus eleven
live raw callers**. Raster is not one of these callers. The eleven callers require retained paged
ingress packets after P2a1 is independently accepted; no compatibility whole-string constructor may
remain.

This report fixes the collision-safe execution order. It is not a source or runtime acceptance.

## Exact Current Census

The non-test production command is:

```text
rg -n "reject_whole_buffer_artifact_envelope_ingress" --glob '*.rs' --glob '!target/**' --glob '!**/🧪️tests/**'
```

It returns:

1. the shared definition in the OS store;
2. Process3d;
3. Dag;
4. Flow;
5. Puzzle5d;
6. Procedural3d;
7. Puzzle3d;
8. Procedural2d;
9. FEM2d;
10. FEM3d;
11. Shooting; and
12. CAD.

Every caller currently accepts an `Option<String>` whole envelope, calls the rejecting placeholder,
constructs an `ArtifactStore` in the bridge constructor, and exposes direct store dispatch and
whole snapshot/envelope serialization behind decorative `async` or immediate Wasm methods.

The removal target is not merely zero calls to the rejecting symbol. Each live constructor must be
replaced by retained fixed-page input, fixed operation/session registration, incremental typed
decode/history replay, freshness-validated atomic publication, paged output, and one-owner terminal
close. Deleting the guard while restoring `serde_json::from_str` is a verifier failure.

## Shared Prerequisites

All waves require:

- accepted P1q actual DB/I/O page ownership where the mounted store path performs I/O;
- accepted P2a1 universal retained job/session ownership;
- accepted fixed page, item, byte, output, control, and process ledgers;
- nonblocking one-step WorkerPool driving in native and Wasm hosts;
- checked nonzero generations and permanently exhausted slots;
- public take/resume/close plus registry rediscovery after handle loss; and
- exact schema-owned construction and retirement cursors for every domain field.

An executor may reuse accepted generic page/session mechanics. Domain-owned snapshot, mutation,
history, child, conflict, preview, and output taxonomies remain inside each plugin boundary. No
generic erased graph estimate or recursive destructor can replace that census.

## Wave 1 — Process3d

Execute the already prepared contract:

`📓️p8yx-process3d-retained-envelope-ingress-census-2026-08-23.md`.

Process3d is first because it provides the richest independent proof for nested machine,
capability, parameter, rule, step, measure, working-solid, child-reference, mutation, and history
ownership without colliding with active Puzzle or FEM work.

Acceptance changes the global census **12 → 11** and requires a fresh Terra audit before the shared
mechanics may be reused by later waves.

## Wave 2 — Procedural2d and Procedural3d

The two procedural bridges have the same small Wasm surface and sibling domain taxonomy. Implement
them in one executor packet but keep separate schema catalogs, fixed registries, generation tokens,
terminal witnesses, and fixtures. Shared implementation may be placed only in their existing
common domain layer when both variants genuinely use it.

The packet owns the two Wasm bridges, the minimum procedural snapshot/mutation retained cursors,
focused fixtures, verifier region, and ticket report. It does not touch Puzzle, FEM, Flow, Dag,
Shooting, CAD, or the generic store.

Target global census **11 → 9** after Process3d is accepted.

Required discriminator: a 2D or 3D catalog entry removed by mutation must fail only its respective
fixture, proving that one variant did not inherit a forged generic schema proof.

## Wave 3 — Puzzle3d and Puzzle5d

Begin only after P4d is independently GREEN and its fill-builder terminal authority is stable.
Puzzle3d ingress must compose with the accepted fill generation/close graph: replacing the document
freezes or cancels the exact stale fill and keeps the last-valid document/fill preview until the new
envelope publishes.

Puzzle5d uses the same outer bridge shape but requires its own snapshot/mutation schema and
parse-Dsl output cursor. The existing `parse_dsl_json` whole parse/serialization function must not
remain reachable from the interactive bridge.

Implement both in one sibling packet only if the exact pre-edit file census confirms no active P7
or P4 collision. Otherwise split Puzzle3d first, audit it, then Puzzle5d.

Target global census **9 → 7**.

Required discriminator: document replacement during every FillBuilder phase must reject stale
preview/commit and drain the exact fill/session/document owners without an Arc alias.

## Wave 4 — FEM2d and FEM3d

Begin only after P6g is independently GREEN. FEM2d ingress must bind document generation to the
mounted mesh/assembly/PCG/visual operation session. Envelope replacement, document close, or model
validation fault must freeze new solver admission and move the exact live solver owners into the P6
retained disposer.

FEM3d receives a separate schema and solver-operation registry; it may share only the accepted
dimension-neutral page/session interface. No 2D owner cap or close stack may be assumed sufficient
for 3D nodes/elements/DOFs/materials/loads/results.

Target global census **7 → 5**.

Required discriminator: maximum/+1 combined document-plus-active-solver working sets for both
dimensions, with exact process/page handback and last-valid validated result preservation.

## Wave 5 — Flow and Dag

Flow and Dag are framework/product-host callers rather than isolated plugin Wasm files. They must be
separate executor packets because both sources are large and own different live renderer/host state.

### Flow

The Flow bridge has whole `ArtifactDsl`/`ArtifactPack`, direct store construction/dispatch,
`snapshot_json`, complete `serde_json::Value` edit helpers, collection diffs, and live `FlowHost`
undo/redo. Retained ingress must publish through the exact Flow document generation and invalidate
the matching surface. Its close graph includes widget/synapse collections, preview dictionaries,
expanded sets, DSL/pack parser state, store history, and FlowHost live state.

Target global census **5 → 4** after independent acceptance.

### Dag

The Dag caller lives inside the very large directed-board module and coexists with layout,
preview/media, graph host, connection, cluster, input/output, and renderer authorities. The packet
must own a narrow retained VCS subregion and domain-owned Dag field catalog; it must not reformat or
rewrite the unrelated board module.

Dag replacement also composes with the P3n populated surface disposer. A document generation
replacement must retire GraphHost, sync caches, scene owners, and pending layout/preview work
through their accepted close APIs rather than ordinary module Drop.

Target global census **4 → 3**.

Required discriminator for both packets: quiet-wake, surface invalidation, stale generation, and
document close exercise the mounted product route, not only a Wasm unit fixture.

## Wave 6 — Shooting and CAD

Shooting and CAD are last because their bridge shapes are small but their domain snapshots reach
renderer/geometry owners whose Phase 3/5 and dependency-replacement boundaries must be stable.

### Shooting

Cover the complete shooting snapshot/mutation, camera/shot/sequence/media references, history,
projection output, and close taxonomy. Whole `projection_json` must become paged retained output or
an explicitly batch-only noninteractive route rejected from UI.

Target global census **3 → 2**.

### CAD

Cover the complete CAD model, geometry/topology/property/reference/mutation/history taxonomy and
all owned geometry control/backing. CAD ingress must compose with prepared geometry/GPU retirement
and may not invoke whole boolean, tessellation, serialization, or render work during decode or
publication.

Target global census **2 → 1**.

At that point the one remaining occurrence is the shared rejecting definition.

## Final Shared Guard Retirement

Delete `reject_whole_buffer_artifact_envelope_ingress` only after exact repository census proves it
has zero callers and a hostile mutation reintroducing any raw whole-buffer constructor makes the
permanent verifier fail. The final target is **1 → 0**.

Do not replace the definition with a permissive decoder, deprecated compatibility layer, alias, or
renamed helper. There are no legacy users and no compatibility requirement.

## Per-Caller Mandatory Contract

Every wave must provide:

1. fixed page ingress admitted before producer copy;
2. seal/operation generation validation;
3. one token/field/collection/control/page construction unit per worker grant;
4. retained mutation/history replay without whole clone/diff/apply;
5. source document immutability until atomic candidate publication;
6. progress/checkpoint output through bounded latest-wins preview and lossless terminal queues;
7. paged projection/envelope output or explicit UI-forbidden batch classification;
8. exact terminal take/resume/close and registry rediscovery after handle Drop;
9. populated ordinary Drop fail-closed;
10. exhaustive terminal-empty and process/page/item/byte/control counters zero;
11. exact rejected producer identity at maximum +1 and all full-output paths; and
12. native/Wasm deterministic parity on the final serialized matrix.

## Hostile Fixture Matrix

Each caller needs zero/max/+1 pages, items, bytes, strings, nesting, mutation/history entries,
outputs, operations, and control owners; zero/insufficient fuel and expired deadline; cancel/fault/
panic/drop before and after every transfer; wrong/stale/duplicate/ABA/exhausted handles; interrupted
close; full terminal/output registries; last-valid publication; and worker-count deterministic
digests.

One combined-depth fixture must include every simultaneously legal nested domain plus history and
conflict frames. Separate per-subtree maxima cannot justify an undersized shared close stack.

Every fixture has a source mutation that removes or weakens its exact production property and must
fail the permanent self-test.

## Acceptance and Serialization

Each wave requires scoped rustfmt, exact caller census, verifier self-test/live predicates, scoped
and whole diff checks, and an independent Terra source audit before the next count is accepted.

No Cargo, Nx, Wasm, browser, runtime, stress, allocation, replay, or timing command may overlap
active Rust source packets. After the final source wave and shared guard removal, one serialized gate
owner runs debug/release/strict warnings, native and both Wasm targets, real browser worker,
cancellation/fault/memory/close stress, deterministic replay, and the 8 ms watchdog matrix on the
same final tree.

Phase 8 remains RED until the caller count is zero and the complete command inventory is classified
and mounted.
