# Draw Retained Mutation Workset

## Recorded defect and law boundary

The Native 4 census separated eight retained-authority failures from the repeated-shape identity packet. The first failure was the live production refusal `drawing-store.mutation-aggregate-byte-capacity` for an exact 4,096-byte rename. Seven later lifecycle laws could not reach their cancellation, rollback, duplicate, or terminal-owner boundary because the same aggregate gate rejected first.

The retained packet now selects the original eight laws plus two explicit clone/neutral-fixture laws. It keeps source document admission at `D = 262,144`, field admission at `P = 4,096`, container structural checks, and every terminal owner assertion.

## Causal repair

The removed aggregate added borrowed source and mutation ownership, preallocated arena components, duplicate scratch storage, candidate storage, and unplanned clone work under one document ceiling. It double-counted the arena and assigned no operation-specific credit to `DuplicateLayer`.

`DrawingMutationWorksetPlan` now records independent boundaries:

- source census remains `S ≤ D`;
- the mutation digest records its largest field and retains `M ≤ P`, while its borrowed owner census remains independently bounded;
- one arena is measured once through `DrawingMutationArenaOwner::retained_totals`;
- candidate authority storage is `Q`;
- operation-specific cloned backing is `C`;
- the live candidate workset is exactly `A + Q + C`;
- the derived architectural maximum is `2D + P + A + Q`.

Create, duplicate, fill, and stroke copies are constructed and measured before `BindOverlay`. Duplicate first locates its immutable source, completes the real clone and ID/name rewrite, then walks the completed retained clone to record actual String and Vec capacities plus the temporary boxed clone authority. Any allocation or clone-capacity refusal therefore occurs before overlay or source handoff. Scalar mutations keep clone credit zero and use arena pages already counted in A.

Container rebuild still consumes only `maximum_container` and `container_slots` from the workset. The rollback and exact-owner predicates are unchanged.

Arena bootstrap now derives its claim from the configured one-owner arena totals multiplied by four. The native law compares that claim to the four actual allocator-returned owner totals and keeps the existing exact-capacity, plus-one refusal, fault-injection, and terminal retirement cases.

## Neutral contract

The language-neutral `drawing.mutation-admission/v1` schema and fixture live beside the owned Drawing authority. Four cases declare the document and field limits, source/mutation profiles, expected disposition/fault, and logical work classes:

- exact 4,096-byte rename: applied with `fixed-arena` and `overlay-page`;
- 4,097-byte rename: rejected with `drawing-store.mutation-field-capacity` and `none`;
- D+1 source: rejected with `drawing-store.preflight-byte-capacity` and `none`;
- duplicate subtree: applied with `fixed-arena` and `clone-subtree`.

The fixture does not encode Rust allocator capacities. Native laws compare the real workset and clone traversal capacities, preserve source pointers, and round accepted/rejected carriers through `serde_json 1`.

## Validation

The permanent Draw JavaScript publication audit now validates the admission schema and fixture through strict Ajv in addition to its existing publication authority checks.

```text
NX_DAEMON=false SEMIO_TEST_ARTIFACT_DIR=.../draw-mutation-admission-ajv \
  bun nx run @semio-tech/draw-js:publication-authority-audit --skip-nx-cache
```

Result: 2 Bun tests passed, strict Ajv accepted all four mutation-admission cases, the owned publication oracle passed, all five hostile publication mutations were refused, and Nx completed successfully in 281 milliseconds.

Both edited Rust files parse with `rustfmt --check`; the production authority retains its pre-existing formatting delta. This agent ran no Cargo command. Root owns the focused and full native receipts.

## Native 9 census and repair

Root's full Draw Native 9 receipt was Nextest `4785d48a-c0ff-45b6-9a35-3da38aa82520`: 280 run, 252 passed, 28 failed, zero skipped in 2.136 seconds; Nx completed in 26.8 seconds. The failures separated into 15 editor/live-envelope laws, nine retained-mutation laws, and four committed structure-diff laws.

The retained failures exposed three exact state-machine defects:

- duplicate ID/name rewriting cleared its scratch fields at phase 13 but never advanced to phase 14, so every duplicate remained in `PrepareOwnedValue` forever and never reached clone-work planning;
- rebuild close called rollback again after the rollback had already completed and its source owner had been handed back, producing `drawing-store.container-rollback-owner-missing` on the next bounded close grant;
- derived-string accounting emitted an internal page-capacity name before the canonical mutation field authority observed the same 4,097-byte field.

The repair advances the duplicate frame to phase 14, drives rollback or forward close only while `rebuild_close_phase == 0`, and leaves the field-size refusal to `DrawingSemanticDigestCredit::observe`, which returns `drawing-store.mutation-field-capacity`. No item, byte, container, pool, or stack capacity changed.

The editor failures were stale raw-dispatch assertions against commands already classified as migrated retained jobs. Each affected law now uses the production settle/ACK ladder, checks that the pre-publication `InvocationResult` does not leak an operation, checks exactly one Artifact result page and one completion for mutation commands, and then reads the published snapshot. Camera and scratch-only commands are also settled before render or fixture close. The route census includes app-level actions, where `addLayer` is actually declared, in addition to window actions and commands.

The create/reorder committed diffs now spell schema-defined floating fields as JSON floating values (`1.0`, `0.0`, and so on), matching canonical `serde_json::to_value` output. Integer indices remain integers.

The two valid live-envelope laws now print `artifact_store_replacement_refusal` if a later full run still refuses the candidate. This preserves the Ready assertion while retaining the typed refusal leg needed for a bounded follow-up.

Validation after the repair:

```text
bun nx run @semio-tech/draw-js:publication-authority-audit
```

Result: Bun 2/2 passed; strict Ajv accepted the neutral fixture; the owned publication oracle passed; all five hostile cases were refused; all four mutation-admission cases passed; Nx completed in 1.0 seconds. Receipt: `🗑️generated/astra-runtime/draw-native9-repair-oracle.log`. `rustfmt --emit stdout` parsed the owned authority and editor test source; both committed diff files parse as JSON. No Cargo command was run by this agent.

## Native 11 census and eight-failure follow-up

Root's full Draw Native 11 receipt was Nextest `22575bb4-1d2b-4b3a-8d69-f305047278f9`: 280 run, 272 passed, eight failed, zero skipped in 2.464 seconds; Nx completed in 27.6 seconds. The exact receipt is `🗑️generated/astra-runtime/draw-native11/run.log`.

The two camera failures were fixture-context refusals before capacity measurement: production requires an addressed canvas-window view for window-config commands. They now use the existing live-instance `inline_selection_app` context, retaining the document-immutability and exact camera assertions. The opacity law now uses the same real canvas context and first witnesses the framework-owned selection before requiring one retained Artifact publication.

The two duplicate failures reached the new structural admission correctly: the one-element handcrafted root vectors had capacity one and could not rebuild to two elements. Their positive fixtures now pre-admit exactly the destination slot; the negative unadmitted-destination case remains unchanged and still requires `drawing-store.duplicate-destination-capacity` with exact source-owner return.

The cancellation law now distinguishes its actual atomic boundary. Every named phase before commit must still reject stale/cancelled authority and preserve the last-valid source. `Complete` is after overlay commit, so a separate law now requires a late cancelled or stale context to finish the exact arena-owner return and retain the already-published duplicate. This preserves both sides of the invariant instead of expecting rollback after publication.

The two live envelope failures showed that the initializer cloned decoded layer mutation destinations with serialized-length capacity, then a valid replay rename required a larger name and faulted `drawing-store.mutation-overlay-destination-capacity`. The initial store clone now materializes `id`, `name`, `blendMode`, and Boolean `operation` as mutation-ready fixed field destinations, one field per bounded clone step. Ordinary duplicate/create clone ownership remains on the prior capacity-preserving path, including the explicit unadmitted duplicate rejection law.

All three touched Rust files parse through `rustfmt --edition 2021 --emit stdout`. This agent ran no Cargo command. The next native receipt must cover the renamed precommit law, the new committed-boundary law, both live initializer laws, both duplicate laws, and the three editor runtime laws before the packet can be called green.


## Draw native 12 and remaining fixture boundary repair

Root's full Draw run `a961ece7-67b4-4575-b161-e98933819437` executed 281 tests: 277 passed, four failed, zero skipped in 2.451s (Nx 36.7s); receipt: `🗑️generated/astra-runtime/draw-native12/run.log`. Every retained mutation cancellation, duplicate, late-cancel, descriptor, and diff law passed. The remaining failures were fixture protocol mismatches:

- Both camera laws dispatched through the addressed `drawing-canvas` window, then rendered with `ViewModel::default()`, which correctly resolved the default canvas config `[512,512,.75]`. They now render with the same addressed view that owned the window config mutation.
- The selection law sent a framework-reserved `interactionSelect` admission through the typed-operation settle helper. It now settles the returned admission with `settle_framework_reserved_admission` and includes the canonical `method: "pick"` wire field.
- The stale replacement ACK expected `Err`, but `acknowledge_artifact_store_replacement` specifies a non-owning or stale handle as `Ok(false)`. The law now asserts false and still proves the subsequent exact handle returns true and retires the owner.

Only test source changed for these four repairs. The file parses with `rustfmt --edition 2024 --emit stdout`; no Cargo command was run.
