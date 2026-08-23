# Phase 8yw — Raster Retained Envelope Ingress

Date: 2026-08-23  
Owner: `/root/p3_browser_worker_repair`  
Verdict: source-audit-ready; acceptance remains pending an independent audit.

## Pre-edit exact caller census

The exact production scan

```text
rg -n "reject_whole_buffer_artifact_envelope_ingress" --glob '*.rs' .
```

returned 13 occurrences before this packet: one shared fail-closed definition and 12 live raw whole-buffer callers.

| Role | Exact source |
|---|---|
| Shared definition (out of scope) | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7926` |
| Dag caller (out of scope) | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:8925` |
| Flow caller (out of scope) | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:844` |
| Procedural3d caller (out of scope) | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` |
| Fem2d caller (out of scope) | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:22` |
| Procedural2d caller (out of scope) | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` |
| Fem3d caller (out of scope) | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:22` |
| Process3d caller (out of scope) | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:36` |
| Cad caller (out of scope) | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:24` |
| Puzzle5d caller (out of scope) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:27` |
| Puzzle3d caller (out of scope) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:27` |
| Shooting caller (out of scope) | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:31` |
| Raster caller (this packet) | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` |

The required post-edit structural target is therefore exactly 12 occurrences: the unchanged shared definition plus the unchanged 11 non-Raster callers.

## Pre-edit Raster owner census

The only Raster raw ingress is the WASM `RasterArtifactVcs::new(envelope_json: &str)` path. It materializes a borrowed whole JSON string into `RasterEnvelope`, then constructs `RasterStore` in the same call. The owners reachable from this boundary are:

- `RasterEnvelope = store::ArtifactEnvelope<RasterSnapshot, RasterMutation>` at `.../🧬️schema/🧬️mutations/🦀️component.rs:69`;
- `RasterStore = store::ArtifactStore<RasterSnapshot, RasterMutation>` at the same file's line 70;
- recursive `RasterSnapshot` at `.../🧬️schema/📸️snapshot/🦀️component.rs:30`, including `String`, `Vec<RasterLayerNode>`, and `BTreeMap<String, RasterAssetChild>` owners;
- twelve-variant `RasterMutation` at `.../🧬️schema/🧬️mutations/🦀️component.rs:41`, including nested layer/asset owners;
- the JS-facing `RasterArtifactVcs.store: RefCell<RasterStore>` owner in the scoped WASM component.

No second Raster raw caller or compatibility constructor exists in the pre-edit census.

## Packet invariants

- Raster alone changes; the shared fail-closed definition and other raw callers remain byte-for-byte out of scope.
- Ingress is fixed-page and pre-admitted before copying bytes.
- Operation/page/byte credits, generation, checkout, ACK, cancellation, handback, and close are explicit fixed authorities.
- Decode/build advances only through bounded poll grants; ordinary constructor/request paths do no whole-buffer decode.
- Rejection returns the exact generation/page owner; stale/duplicate tokens cannot mutate a reused slot.
- Terminal close retires one page/root/scalar per grant and releases admission only after a terminal-empty witness.

## Implemented source packet

### Raster Wasm caller

The Raster Wasm bridge no longer accepts or parses a whole envelope string. `RasterArtifactVcs`
owns the established `VcsArtifactApp<EditorApp<RasterPlayApp>>` runtime and exposes only the retained
page lifecycle:

1. `beginEnvelopeLoad(maximum_pages, maximum_bytes)` admits nonzero credits no larger than the
   framework's fixed 64-page / 262,144-byte operation authority and returns the by-value
   `(operation, generation)` handle.
2. `admitEnvelopePage(handle, Uint8Array)` checks the 4,096-byte page cap and calls
   `preflight_artifact_envelope_ingress_page` before the producer closure copies into the fixed
   `[u8; 4_096]` page. The callback is never entered on stale/saturated/oversized admission, so the
   JS caller retains its exact source owner.
3. `sealEnvelopeLoad` transfers the exact page authority to the retained decoder.
4. `pollEnvelopeLoad` gives maintenance one item / one page-byte grant, returns explicit progress,
   and acknowledges the exact operation-generation only after a replacement candidate is ready.
5. `cancelEnvelopeLoad` and `closeStep` route through the same retained runtime; there is no direct
   store constructor, direct dispatch, projection JSON, envelope JSON, bulk loop, or inline fallback.

### Raster-owned domain catalog and initializer

`RasterEnvelopeOwnedFieldCatalog` now owns snapshot, mutation, VCS, edit-history, and conflict field
authorities. Snapshot and mutation pack fields are admitted into the fixed 4,096-byte
`OwnedSchemaHexAuthority` before domain decode. Publication transfers the sole decoded owner; fault
and cancellation transfer it to `RasterOwnedRetirement`.

The retirement taxonomy is exhaustive for:

- the snapshot's schema/id/title, layer vector, asset map, and composed child handles;
- Pixel, Group, and Adjustment layers, including nested children and recursive `DslValue` params;
- all twelve Raster mutations, boxed layers, image-asset bytes, and all strings; and
- snapshot `Arc` roots and store/displaced-store ownership.

All recursive owners sit behind `ManuallyDrop` terminal assertions. A close grant advances at most
one nested owner, one string, one byte vector, one empty container backing, or one scalar shell.
The retained store initializer validates operation/generation and edit identities, clones one
admitted scalar/layer/asset per grant, seeds and replays history one entry or mutation per grant,
retires each displaced snapshot incrementally, and publishes only through checked
`generation.checked_add(1)`. Cancellation, stale authority, and fault all pump the same terminal
retirement path before returning their terminal result. Its entry guard observes cancellation and
the `StepContext` deadline/yield authority before either the active owner or retirement cursor is
advanced; a zero-fuel grant advances neither phase nor owner.

`RasterPlayApp` now supplies the Raster envelope catalog, member-store owners, retained
initialization job, and document-store disposer. There is no compatibility constructor.

## Focused fixtures and permanent mutations

Rust source fixtures cover:

- next-generation candidate publication followed by one-owner document-store close;
- cancellation and stale-generation fault reaching terminal-empty;
- a zero-fuel/deadline grant advancing neither initializer phase nor retained owner before a later
  cancellation pumps the same owner to terminal-empty;
- nested Group/Adjustment/`DslValue` and composed-child retirement with one-owner grant assertions;
- all twelve mutation variants, zero-fuel ownership, per-turn item/byte ceilings, and terminal-empty;
- exact 4,096-byte / 64-page / 262,144-byte caps and a page-size +1 rejection returning the same
  fixed owner contents; and
- 4,096-byte domain-string max/+1 admission.

The permanent verifier now rejects, independently: missing preflight; copy-before-preflight; fixed
page replacement with `Vec`; operation-generation handle erasure; unchecked generation increment;
missing deadline/yield admission; missing worker-cancellation admission; a weakened zero-budget
fixture; one missing mutation variant; recursive-value deep drop; missing nested-close fixture;
missing domain cap/terminal fixture; missing page +1 fixture; dynamic post-lift slice input;
whole-buffer string reintroduction; missing exact ACK; cancel-by-drop; bulk close; missing
fixed-registry +1 exact handback; and missing interrupted one-page close.

## Post-edit exact census

The same production scan now returns exactly **12** occurrences: the unchanged shared fail-closed
definition plus the unchanged eleven non-Raster callers. Raster has zero occurrences. The remaining
callers are Dag, Flow, Procedural3d, Fem2d, Procedural2d, Fem3d, Process3d, Cad, Puzzle5d, Puzzle3d,
and Shooting.

## Source gates

| Gate | Result |
|---|---|
| exact whole-buffer symbol census | **PASS**: 13 → **12** occurrences; Raster zero |
| edition-2021 scoped `rustfmt --check` | **PASS** on the Raster codec, editor, and Wasm bridge |
| Bun TypeScript/parser + permanent tool-job verifier | **PASS**: **309** self-tests clean |
| broad `bun ./📜️script.ts verify interactivity --format json` | **BLOCKED by concurrent unrelated source drift**: the rerun stopped in the live-reconcile self-test because the mounted `PatchTracker` predicate no longer matched; the Raster-focused verifier remained clean |
| live Raster retained-route predicate | **PASS**: neither deterministic ledger contains a Raster-specific failure |
| deterministic ledgers | **PASS**: byte-identical `📊️p8yw-raster-ledger-a.json` / `-b.json`, SHA-256 `ed38c717f71a6b0bc05da53925953fb33ad427b1c6788a3748b1da8db1ee3fbf` |
| full tool-job verifier | expected global **RED**: **0/884**, 18 failure classes; no Raster-specific failure |
| scoped and whole working-tree `git diff --check` | **PASS** |
| Cargo / Nx / native / Wasm / browser / runtime / network | **not run by instruction** |

The packet is **source-audit-ready, not accepted**. Rust typechecking, the Rust fixtures, Wasm
generation, and real browser/runtime behavior remain explicitly unverified until the serialized
build lane is opened. Phase 8 remains globally RED for the eleven raw callers and the ledger's
other declared cohorts.

## Independent RED remediation

The five source blockers in
`coordinator-independent-p8yw-raster-retained-ingress-final-audit-2026-08-23.md` are repaired in
the scoped Raster codec and permanent verifier. This is an implementation handback for independent
re-audit, not an acceptance claim.

### Recursive preflight and construction

`RasterSnapshotBoundsAuthority`, `RasterLayerBoundsAuthority`, and
`RasterDslValueBoundsAuthority` now census the recursive owner graph with retained fixed paths and
frames. They account source and simultaneous candidate item/allocation-capacity bytes before the
corresponding construction phase. `RasterSnapshotCloneAuthority`, `RasterLayerCloneAuthority`, and
`RasterDslValueCloneAuthority` then construct one admitted field, key, value, child, or asset field
per granted turn. All governed work calls `raster_reserve_unit` before allocation/copy/traversal;
zero fuel, insufficient low fuel, and an expired deadline advance no owner or cursor.

The production retained route contains no `serde_json::to_vec(source)`, recursive
`source.clone()`, `operation.diff(current)`, or `diff.apply(current)` admitted unit. Asset child
hashing/construction is also retained: MIME, capped byte owner, key, child ID, artifact ID, and each
dialect field advance separately before the exact child is published.

### Typed history replay and atomic candidate publication

`RasterMutationDigestAuthority` hashes the complete twelve-variant mutation taxonomy one bounded
scalar/string/page/layer unit at a time. `RasterMutationCandidateAuthority` cursor-clones the
current snapshot, locates nested targets with a fixed address, stages strings/layers/assets, and
applies vector shifts one swap per grant. The prior snapshot is unchanged until the complete
candidate swaps atomically into the retained initialization runtime; the displaced snapshot enters
the existing one-owner retirement path before the next history mutation.

Insert preparation has its own `BeginInsert` turn, so parent lookup and vector publication never
share a fuel opportunity. Initializer active-owner retirement also returns immediately after one
retirement action rather than continuing into another phase in the same step.

### Exact terminal ownership

An unclaimed completed store is now included in terminal retirement through
`ArtifactDocumentStoreDisposer`; cancel/stale/fault after `Complete` cannot declare terminal while
`candidate`, `mutation_candidate`, or `candidate_disposer` remains. The public generic initializer
wrapper uses `ManuallyDrop`, refuses Drop until exact candidate handoff/terminal retained close, and
the mounted ingress recovery path routes every non-taken terminal through
`retain_initializer_for_close(job)`. This proves fail-closed retained ownership rather than a silent
candidate destructor; the eventual runtime gate still has to exercise the mounted abandon path.

String and byte owners now compare/report allocation `capacity()`, as do vector backings. Recursive
retirement uses one fixed iterative frame array sized for the admitted layer-plus-value depth; it
does not allocate a boxed child cursor per recursive node. The frame capacity is checked before a
push. Empty layer/asset containers, nested values, child handles, candidate stores, and their shells
reach explicit terminal-empty witnesses before ordinary Drop. A dedicated fixture records that a
new empty `BTreeMap` releases no hidden allocation bytes at its shell step.

### Added focused source fixtures and mutations

Rust fixtures now cover:

- a 48-level snapshot clone with fuel `1` and exact source/candidate equality;
- an expired deadline that advances neither bounds, clone, nor mutation ownership;
- a small rename against a 40-level snapshot with unchanged source until atomic publication;
- cancel after `Complete` with a 24-level unclaimed candidate and exact disposer terminal;
- allocation-capacity string/byte retirement and fixed iterative layer/value depth;
- exact nested item/byte maximum and `+1` preflight failure; and
- allocation-free empty asset-map shell retirement.

The permanent verifier now discriminates removal of the recursive bounds authority, pre-work fuel,
typed mutation candidate/digest, completed-candidate disposer, capacity accounting, fixed iterative
frames, each hostile fixture, public initializer Drop refusal, and reintroduction of whole recursive
encode/clone or monolithic diff/apply.

## Remediation gates

| Gate | Result |
|---|---|
| exact whole-buffer symbol census | **PASS**: exactly **12**, one shared definition plus eleven non-Raster callers; Raster zero |
| scoped edition-2021 `rustfmt --check` | **PASS**: Raster codec, editor, and Wasm bridge |
| permanent tool-job verifier self-test | **PASS**: **318** self-tests |
| live tool-job source verifier | expected global **RED**: **0/884**, 19 declared failure classes; no Raster-specific failure |
| deterministic ledgers | **PASS**: byte-identical A/B, SHA-256 `0619e32db7b59923e5e4df3038249db97172d0ff717fbd9b39cb79e14c51be9b` |
| scoped and whole working-tree `git diff --check` | **PASS** |
| Cargo / Nx / native / Wasm / browser / runtime / network | **not run by instruction** |

Final remediation verdict: **source-audit-ready, not accepted**. Rust typechecking and execution of
the new Rust fixtures remain unrun; the serialized native/Wasm/browser gate remains mandatory.
