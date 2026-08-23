# Phase 8yx — Process3d Retained Envelope Ingress Census

Date: 2026-08-23  
Owner: `/root` coordinator  
Verdict: implementation packet prepared; source and runtime acceptance are not claimed.

## Exact structural census

The production scan

```text
rg -n "reject_whole_buffer_artifact_envelope_ingress" --glob '*.rs' --glob '!target/**' --glob '!**/🧪️tests/**'
```

currently returns exactly **12** occurrences: one shared fail-closed definition and eleven live raw
whole-buffer callers. The Process3d caller is
`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:36`.
The shared definition and the Dag, Flow, Procedural3d, Procedural2d, Fem2d, Fem3d, Puzzle5d,
Puzzle3d, Shooting, and CAD callers are outside this packet.

The exact Process3d post-edit target is zero raw calls and a global count of **11**: one unchanged
shared definition plus ten unchanged non-Process3d callers. This count may be updated only after an
independent source audit accepts the packet.

## Current boundary

`Process3dSnapshotVcs::new(envelope_json: Option<String>)` receives a whole owned JavaScript string,
calls the deliberately rejecting raw-ingress placeholder, and attempts to construct
`ArtifactStore<Process3dSnapshot, Process3dMutation>` in the constructor. The same bridge exposes
direct synchronous store work behind decorative `async` methods for text dispatch, binary
dispatch, snapshot JSON, envelope JSON, and generation. None of these routes mounts the retained
application worker/session protocol.

The reachable snapshot owner graph is not scalar:

- `Process3dSnapshot` owns `Workshop`, three top-level strings/records, stock and flow child
  handles, `Vec<ProcessStep>`, `Vec<ArtifactChild<SemioBrepSnapshot>>`, and an optional cursor;
- `Workshop` owns machines; machines own identity/icon/catalog strings and capability collections;
- capabilities own parameter and rule collections with nested strings;
- process steps own identity/label/origin and `ProcessMeasure`; measure variants contain
  `WorkingSolid`, `Pose`, and nested component/tool owners;
- every child handle owns `child_id` plus the strings and collections reachable through
  `ArtifactRef`; and
- `Process3dMutation` has sixteen variants, including complete machine, capability, step, measure,
  child-handle, and string owners.

The handcrafted snapshot codecs are also whole-buffer implementations. Text encoding builds
formatted strings, `enc_child_list` collects and joins a whole temporary vector, structured fields
round-trip through whole `serde_json`, binary reads call `to_vec`, collection reads allocate from an
untrusted count, and pack encode/decode returns or consumes whole vectors. These helpers may remain
for explicitly non-interactive offline paths only if the retained Wasm/history route cannot reach
them; they are not an acceptable implementation of this packet.

## Required architecture

### Retained Wasm ingress

Replace the constructor string with the accepted paged lifecycle already established by the
retained envelope cohorts:

1. mount the Process3d editor application in the single worker scheduler;
2. begin an envelope load by admitting fixed operation/page/item/byte/output authorities;
3. preflight a fixed-size page before the producer copies any byte;
4. seal only after exact page ownership transfers;
5. poll one bounded unit through decode, snapshot construction, history replay, publication, and
   displaced-owner retirement;
6. ACK the exact operation-generation only after an atomic candidate publication; and
7. route cancellation, stale handles, rejection, panic, abandoned completion, and close through
   the same retained terminal disposer.

There must be no compatibility whole-string constructor, inline run-to-completion loop, direct
store constructor, direct projection/envelope serialization, independent thread/channel, or
cancel-by-drop escape hatch.

### Domain-owned schema authorities

Create a Process3d-owned field catalog and retained cursors for the complete snapshot, mutation,
VCS history, edit identity, conflict, and child-handle taxonomies. Construction and retirement must
advance one admitted semantic scalar, fixed byte page, fixed collection slot, or fixed control
owner per grant. The source remains unchanged until a complete candidate swaps atomically.

The catalog must explicitly cover:

- every Workshop machine, capability, parameter, and rule;
- top-level stock fields, `Stock`, `Pose`, and both child handles;
- every process step, origin, `ProcessMeasure`, and nested `WorkingSolid` variant;
- each tool-solid child handle and every `ArtifactRef` field;
- all sixteen `Process3dMutation` variants and their nested owners; and
- history edit IDs, parents, authors, messages, conflict owners, candidate/displaced snapshot
  roots, store roots, and Wasm control roots.

Do not derive retained credits from `size_of` estimates of `BTreeMap`/`HashMap` entries or other
standard-library allocation layouts. Dynamic collections on the retained route must use an owned
fixed/page authority with a cap and exact +1 handback, or an allocate-inspect-admit protocol that
retains and incrementally retires the exact rejected allocation. Requested `Vec`/`String`
capacities are not actual capacities: candidate credits must use observed capacity, and an
over-capacity allocation must either be admitted before publication or enter exact retained
retirement.

Every `Box`, `Arc`, shared control block, collection backing, string backing, and byte backing needs
an explicit owned credit and matching one-grant close action. Recursive traversal must use a fixed
stack whose capacity is proved from one **combined** nesting budget for machine/capability,
step/measure/solid, child-reference, and history/mutation frames. Independently capped nested
subtrees may not be summed into an undersized retirement stack.

### Incremental codecs and history

The retained route must not call `serde_json::to_string`, `serde_json::from_str`, `to_vec`,
`collect::<Vec<_>>()`, `join`, whole `ArtifactDsl`/`ArtifactPack`, snapshot `clone`, generic
whole-document `diff/apply`, or the current direct store dispatch/serialization methods.
Use a persistent field/parser cursor over retained pages. Validate lengths and collection counts
before allocation, retain incomplete token state across grants, and report progress/checkpoints
without cloning the candidate.

History replay must digest and apply typed `Process3dMutation` candidates. One history entry or one
mutation may itself require many grants; a grant may not scan a whole snapshot to locate an ID or
shift a whole vector. Fixed index/path authorities and one-swap vector motion are required. The
document base revision, operation generation, and edit parent must be validated immediately before
publication with checked generation arithmetic; stale/ABA candidates retire without becoming
visible. Last-valid content remains available until the replacement commits.

### Terminal ownership

All terminal variants require a terminal-empty witness. Close must drain admitted but unsealed
pages, active decoder fields, rejected allocations, candidate/displaced snapshots, mutation
candidates, history cursors, completed-but-unclaimed stores, control roots, and output/checkpoint
owners. Ordinary `Drop` must fail closed while any retained owner is live; it must not silently
deep-drop after a fixed number of turns. Cancellation after `Complete` is still an ownership path,
not a no-op.

## Permanent hostile fixtures

The implementation and permanent verifier must discriminate at least:

- fixed page/item/byte/output maximum and maximum +1 with exact same-owner rejection;
- zero fuel, insufficient fuel, expired deadline, and cancellation before/after every phase;
- stale, duplicate, saturated, wrapped-generation, wrong-page, wrong-operation, and ABA handles;
- allocator over-capacity for strings, vectors, and control/container owners;
- populated collection backing retirement, not only a newly empty collection;
- every snapshot field and every one of the sixteen mutation variants;
- a machine with maximum nested capabilities/parameters/rules and +1;
- a step with the deepest admitted measure/working-solid/child-reference path and combined-depth
  maximum +1;
- low-fuel typed history replay with source immutability until publication;
- cancel/fault after candidate completion but before caller takeover;
- exact one-owner-per-close-grant progress, interrupted close, and terminal-empty idempotence;
- panic/rejection at producer, decoder, candidate, replay, publication, ACK, and disposer seams;
- last-valid rendering while replacement is pending or cancelled; and
- page identity and deterministic progress/checkpoint output across native and Wasm execution.

The verifier must source-mutate away each required preflight, generation, capacity, control-backing,
combined-depth, history, completed-candidate disposer, zero-budget, and close mechanism and prove
the corresponding self-test fails.

## Acceptance gates

Source handoff requires scoped edition-2021 `rustfmt --check`, all permanent verifier self-tests,
the live verifier with no Process3d-specific failure, deterministic byte-identical ledgers, exact
raw caller census **12 → 11**, and scoped/whole `git diff --check`. Cargo, Nx, native, Wasm,
browser, runtime, stress, timing, cancellation, panic, and network gates remain serialized and may
run only after overlapping Rust source packets stop.

This census is a bounded implementation contract, not an acceptance report. Phase 8 and the master
ticket remain open.
