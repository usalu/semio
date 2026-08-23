# Sol Independent P8 Draw Semantic Digest and Aggregate Credit Final Re-Audit — 2026-08-23

## Verdict

**REJECT — Draw retained-load source cohort.** The schema-complete retained SHA-256 remediation is materially sound, but the live aggregate reservation still does not count every simultaneously retained source and derived allocation, the applied/redo commit seam still clones a prepared identifier inside the shared runtime, and the advertised aggregate and cancellation/stale fixtures do not exercise the claimed boundaries.

This is an independent Sol High source re-audit. Terra admission was scheduler-limited, so this report does not claim or imply a Terra verdict. I made no production edit. The only file authored by this audit is this report.

The verdict is narrow. Full Phase 8 remains **RED at 0/884 admitted commands and 18 global failure classes**. Cargo/native, Nx, Wasm, browser, network, runtime timing, hostile-valid-payload timing, and the full command migration remain **RED/unverified**.

## Blocking Findings

### 1. The aggregate ledger under-credits live original and derived owners

`DrawMutationAggregateReservation::admit` derives `mutation_items` and `mutation_bytes` by multiplying semantic digest totals by `structural_copies.max(1)` (`owned/component.rs:2599-2617`). Every nonstructural mutation therefore receives exactly one semantic copy of credit. That is not the live ownership shape:

- the decoded typed mutation remains retained in the envelope;
- `RenameLayer`, `SetLayerBlendMode`, and `SetLayerBooleanOperation` allocate a replacement `String` while the mutation's source `String` remains live;
- fill and stroke replacement retain the original mutation style while constructing a second stop/dash/string owner graph;
- the candidate retains the displaced old field in `DrawOwnedRetirement` until bounded retirement.

The single `mutation.items`/`mutation.bytes` credit cannot simultaneously represent the original typed mutation and its derived replacement. Structural multiplication by two only for Create/Duplicate does not close the nonstructural path.

The accounting inputs are also semantic lengths/counts rather than exact allocation ownership. `DrawSnapshotBoundsAuthority` adds `String::len` and logical item counts; it does not count String/Vec capacity, BTreeMap node backing, the typed mutation container/Box backing, or allocator-returned capacity. Candidate skeletons use `Vec::with_capacity`, and rebuild/fill/stroke authorities use `try_reserve_exact`, but no post-reservation capacity check reconciles the actual admitted backing. `backing_bytes = container_slots * size_of::<DrawLayerNode>() + source_items * size_of::<DrawLayerNode>()` is a coarse formula rather than a per-owner ledger.

The aggregate therefore does not establish the requested exact **4,096-item / 262,144-byte** simultaneous-owner boundary before clone/rebuild. A nonstructural mutation can be admitted with only one field copy credited and then create the second retained owner.

### 2. Applied and redo history still clone an uncredited second identifier

The Draw initializer now prepares applied ID and actor on separate grants and prepares a redo ID on its own grant (`owned/component.rs:3756-3789`, `3857-3865`). This closes the former outer multi-field grant only superficially.

`CommitApplied` and `CommitRedo` pass the prepared owned ID into `ArtifactStoreInitializationRuntime::push_applied` / `push_redo`. Those shared functions immediately call `id.clone()` for the revision record and retain the original ID in the applied/redo vector (`store/component.rs:10948-10963`). The two `String` owners are created and retained in one commit grant, with no Draw or shared-runtime derived item/byte reservation. This is the same simultaneous derived-owner class the prior re-audit required the split preparation to eliminate.

Per-entry validation itself is now one-owner: `ValidateEditMeta { edit, meta }` advances one metadata element per grant. That positive change does not cure the commit-time clone.

### 3. One-item execution and the adversarial fixtures do not prove the claims

Two live cursors still perform more than the named single opportunity:

- `DrawDuplicateRewriteAuthority::step` builds a temporary `Vec`, copies both ID and name, creates and installs a new ID, reserves the name suffix, and appends it in one grant (`owned/component.rs:2702-2717`). Charging fuel after those operations does not split them into retained field/page opportunities.
- snapshot bounds use `source.assets.iter().nth(self.asset)` (`owned/component.rs:825-829`), rescanning up to the current index in one nominal asset grant rather than retaining a BTreeMap range cursor.

The new fixture names do not discriminate the ownership defects:

- `retained_draw_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback` calls `exact_for_test`, which directly fabricates totals with every derived category set to zero (`owned/component.rs:2642-2663`, `4492-4512`). It does not drive the live preflight formula at the exact aggregate boundary. Its 700-layer case proves only one over-cap item rejection; it has no live aggregate byte boundary/+1 and checks only `source.id`, not exact recursive mutation/candidate/container owner handback.
- `retained_draw_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid` advances a nested candidate to a phase and directly calls `close_candidate` (`owned/component.rs:4516-4565`). It never cancels the `StepContext`, never calls the initializer's cancel authority, never supplies a stale operation or generation, and cannot prove stale-before-mutation or cancellation at any advertised stage.
- the digest fixture is useful but not independently discriminating for every field. Several changes combine two semantic changes (for example Image key+width, Boolean operation+child order, Trace source+threshold, and Create parent+index), so omission of either combined field can remain green. The verifier mutations require marker strings, not a digest collision assertion for the removed field.

The permanent verifier reflects this weakness. Its Draw predicate is a conjunction of source-string presence checks (`📜️script.ts:1770-1820`), and the aggregate/cancel mutations rename fixture or fault strings (`📜️script.ts:3306-3318`). They do not mutate the `structural_copies.max(1)` formula, reinstate the shared `id.clone()`, or turn the named cancellation fixture into its current close-only form.

## Accepted Source Evidence

The following remediation is source-valid and should be preserved:

| Requirement | Result | Evidence |
| --- | --- | --- |
| Schema-framed SHA-256 | **PASS structurally** | `DrawSemanticDigestCredit::observe` emits a domain byte, `u16` tag, big-endian `u64` length, and value into repository-owned incremental SHA-256; `seal` folds the terminal digest into the initialization digest. |
| Fourteen mutation variants | **PASS structurally** | Exact variant discriminants and payload cursors cover visible, locked, opacity, blend, rename, transform, fill, stroke, Boolean, trace, create, duplicate, delete, and reorder. |
| Recursive layer semantics | **PASS structurally** | Layer variant/base/transform/fill/stroke/shape/path/text/image/group/Boolean/trace cursors include explicit optional/variant/count boundaries and deterministic child/item order. |
| One semantic digest field/item per call | **PASS narrowly** | Digest authorities advance one framed field, stop component, dash, point component, segment component, or child boundary per `step`; field input is capped at 4,096 bytes. |
| Forbidden initializer seams | **PASS** | The owned live initializer has zero `snapshot.clone()`, `operation.diff`, `diff.apply`, `operation.encode_op`, serde reconstruction, or whole metadata scan occurrences. |
| Atomic candidate publication | **PASS structurally** | A completed mutation candidate replaces `runtime.current` only after terminal handoff; the displaced last-valid snapshot enters retained retirement. |
| Recursive close shells | **PASS structurally** | Snapshot, mutation, candidate, clone, rebuild, style, digest, and initializer owners use retained terminal witnesses and Drop assertions. |
| Per-entry metadata validation | **PASS** | `ValidateEditMeta` checks one metadata entry per grant. |

These passes do not override the exact aggregate/derived-owner and adversarial-evidence blockers.

## Census and Executed Gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check --config skip_children=true` on Draw owned/editor/glue | **PASS** |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | **PASS: 245 self-tests clean** |
| Draw retained-route predicate in full tool-job verifier | **PASS mechanically**; no Draw-named failure |
| Full tool-job verifier | Expected global **RED**: 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, **18** failures, 245 self-tests |
| Broad interactivity self-test/plain DENY | **PASS**; one recorded allowlisted blocking bridge, zero unlisted |
| Placeholder census | **PASS for Draw only**: repository 14 occurrences = one shared fail-closed definition + 13 live callers; Draw zero |
| Forbidden initializer scan | **PASS**: zero whole snapshot clone, whole operation encode, diff/apply, serde reconstruction, and whole metadata scan in the owned file |
| Deterministic Draw ledgers | **PASS**: `p8yt-draw-tool-jobs.json` and repeat are byte-identical, 312,305 bytes, SHA-256 `c6285afecde02b6005349bc05f24009996ab9c3a4842ce34fd5c9f1008617472` |
| Scoped and whole working/staged/HEAD `git diff --check` | **PASS** |
| Cargo, Nx, native, Wasm, browser, network, root lint, runtime timing | Not run by instruction; **RED/unverified** |

## Required Repair Before Another Re-Audit

1. Replace semantic-total multiplication with a schema-first retained owner ledger that separately counts original typed mutation owners, candidate copies, derived replacements, displaced owners, every String/Vec/Box/map/container backing, fixed cursors/pages/indexes/digests, and actual reserved capacities before clone/rebuild. Reject exact aggregate item/byte +1 with every owner retrievable.
2. Replace `push_applied`/`push_redo` ID cloning with owner-moving prepared dual-ID authority or reserve both exact String owners before either is constructed. Commit must move the two already-admitted owners without cloning.
3. Split duplicate rewrite into retained material-page, ID-generation, ID-install, and name-suffix opportunities; retain an asset-range cursor rather than `iter().nth` rescans.
4. Add live exact-boundary fixtures that drive `DrawMutationAggregateReservation::admit` through real source/mutation/candidate/container ownership, including nonstructural original+derived item/byte +1 and actual capacity reconciliation. Assert exact recursive handback and unchanged last-valid candidate.
5. Make cancellation/stale fixtures drive `DrawStoreInitializationAuthority` with real cancellation and mismatched operation/generation at each phase. Add verifier mutations for the one-copy formula, shared `id.clone()`, fake `exact_for_test` boundary, close-only cancellation fixture, duplicate multi-field step, and asset rescanning.

Until those exact paths close, the Draw retained-load source cohort remains **REJECTED**. Full Phase 8 remains **RED: 0/884, 18 failure classes, runtime unverified**.
