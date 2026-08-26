# Puzzle Command Cohort Retained Jobs — Sol Implementation Evidence

## Scope and status

This is the live implementation record for the Puzzle2d, Puzzle3d, and Puzzle5d command cohort. It is intentionally not an acceptance report while any production command still reaches `BoundedFirstStepCommandWork` for work that is not truthfully O(1).

Current semantic boundary: 79 of 99 commands have command-specific retained work or a truthful bounded O(1) implementation:

- Puzzle2d: 3 of 3.
- Puzzle3d: 39 of 53.
- Puzzle5d: 37 of 43.
- Remaining: Puzzle3d 14 and Puzzle5d 6.

All 99 command registrations are production-reachable through the owner-local `ArtifactOwnedToolJobFactory` registration and typed app dispatch. Registration alone is not counted as semantic completion.

## Focused Rust consumer evidence

On 2026-08-26, after the Puzzle5d board-event owner transfers were changed so every pending field is taken into a local before `self.push`, the exact focused consumer check was run against the live shared tree:

```text
cargo check --locked -p semio-s-plugin-puzzle --lib
```

Result:

```text
exit: 0
Finished `dev` profile [unoptimized] target(s) in 3m 38s
warning: `semio-s-plugin-puzzle` (lib) generated 156 warnings
```

The successful check compiled the current Puzzle2d/3d/5d retained command sources, including the Puzzle3d typed per-window config mutations, the two-grant scalar config family, cursorized `addObjectKind`, and the Puzzle5d board-event ownership fix. No Wasm or browser gate was run.

The exact same focused consumer command was rerun after the Puzzle5d engagement-input/control/abort routes, cursorized `addPartKind`, and typed overlap-budget mutation landed:

```text
exit: 0
Finished `dev` profile [unoptimized] target(s) in 5m 23s
warning: `semio-s-plugin-puzzle` (lib) generated 156 warnings
```

## Static and fixture evidence at this boundary

- `bun ./📜️script.ts verify interactivity`: exit 0, `DENY mode — clean` at the 77/99 boundary.
- Both language-neutral retained-job JSON fixtures parse with Bun.
- Focused `rustfmt --edition 2021 --check` parses the edited Rust sources. The editor files still have existing formatting differences, so formatting exit 1 is not claimed as a pass.
- Puzzle3d scalar field source evaluator: 20 production routes, 20 feature vectors, and hostile source markers for old reducer replacement and missing typed mutations.
- Puzzle3d `addObjectKind`: persistent decode, kind, representation, vortex-template, publication, and close owners, with 64/+1 and cancellation/fault boundary vectors.
- Puzzle5d engagement abort: distinct input, board-utility, world-utility, publication, and incremental-close transfers; separate fresh cancellation and fault vectors cover every boundary.
- Puzzle5d `addPartKind`: the production factory now reuses the bounded catalog/grip/target/create/connect cursor rather than the legacy board-brush reducer.

## Deferred before acceptance

- Replace the remaining 20 semantically complex fallback routes.
- Execute all hostile lifecycle/oracle vectors, not only parse/static evaluation.
- Re-run the official root static gates after the next stable semantic boundary.
- Run focused runtime tests and final production caller census after all 99 routes are semantically complete.

## Puzzle5d Clipboard and Import Reserved Routes

On 2026-08-26 the owner-qualified Puzzle5d `copy`, `cut`, `paste`, and `import-media` envelope was completed independently of the remaining command cohort. Each route now has its own `ArtifactOwnedToolJobFactory` registration and concrete resumable `InteractiveJob`; the shared framework proof is no longer substituted for the three clipboard factories.

The production envelope now enforces:

- route-contract preflight before the first copy, a retained 4,096-byte ingress page, and exact rejected `Vec<u8>` handback;
- retained 17-byte checkpoint state carrying stage, cursor, and monotonic progress;
- cancellation before each route transition;
- retained pagewise `CommitOutput` construction whose bytes exactly reproduce the admitted raw envelope before producer completion is published;
- bounded close and terminal-empty witnesses for the output writer/payload and every route owner;
- exact maximum admission and maximum-plus-one rejection with pointer-identity handback in `reserved_wire_exact_max_and_plus_one_preflight_return_the_original_owner`.

The root verifier now checks these production markers together with the shared host's exact owner/schema admission, base-revision and generation freshness, checkpoint monotonicity, mounted resume, raw-output equality ACK, cancellation-before-commit, completion checkout, bounded emit, terminal session close, and permit finish. Its language-neutral self-test fixture rejects 13 hostile mutations covering missing factories/registrations, resizable keys, missing fixed-page ingress, copy-before-preflight, missing max-plus-one law, missing cancellation, empty output substitution, premature completion publication, missing checkpoint progress, stale host admission, missing output ACK, and false terminal close.

Exact static evidence at the final source boundary:

```text
bun ./📜️script.ts verify interactivity tool-jobs --self-test
exit: 0
[verify interactivity tool-jobs] self-tests=449 clean.

bun ./📜️script.ts verify interactivity tool-jobs --format json --output .../📝️puzzle5d-reserved-full-coverage-2026-08-26.json
exit: 1 (unrelated repository cohorts remain red)
JSON selfTests: 449
JSON Puzzle5d clipboard/import failures: 0
JSON aggregate failures: 89

git diff --check -- 📜️script.ts <Puzzle5d editor>
exit: 0
```

The isolated native command used `CARGO_INCREMENTAL=0` and a ticket-local target directory:

```text
cargo check --locked -p semio-s-plugin-puzzle --lib --message-format short
exit: 101
first run: 106 errors
compiler-driven route corrections: `max_raw_wire_bytes` and all six retained import initializer fields
second run: 104 errors
```

The second run reports no error in the reserved state-machine implementation span. Native package acceptance remains blocked by the concurrent repository-wide synchronous trait migration: the first diagnostics are E0053 declarations returning futures where current traits require immediate results, beginning in Puzzle viewers/editors/schema/I/O and continuing across Puzzle2d, Puzzle3d, and Puzzle5d. Focused Rust execution, Wasm, browser, and 8 ms timing are therefore not claimed.

## 2026-08-26 De-Async Test Repair and Retained Checkpoint Restore

This section supersedes the older native/Wasm blocker wording above for the live Puzzle source boundary.

Fresh package evidence completed before the retained-test repair:

```text
CARGO_INCREMENTAL=0 cargo check --locked -p semio-s-plugin-puzzle --lib --message-format short
exit: 0
Finished in 3m19s

CARGO_INCREMENTAL=0 cargo check --locked -p semio-s-plugin-puzzle --lib --target wasm32-wasip2 --message-format short
exit: 0
Finished in 11m28s
```

Exact logs remain in this ticket as `📝️puzzle-native-check-2026-08-26.log` and `📝️puzzle-wasm32-wasip2-check-2026-08-26.log`.

The first retained native-test compile did not execute tests: Cargo reported 381 test-only lockstep errors and therefore 0 run / 0 passed. The saved diagnostic classes were 250 E0277, 105 E0599, 18 E0728, 3 E0433, 2 E0609, 2 E0308, and 1 E0594. Source-only repair now removes every saved stale `dispatch`/selection/load/fill helper await and synchronous `resolve_ready` wrapper, restores the removed BoardHost test drain API through one Puzzle-local extension trait, and repairs the Duration, `Arc<SceneConfig>`, `BuiltNode`, `UiText`, and `ComponentTree` test diagnostics. A retained native-test rerun is deliberately pending the shared compiler slot; no pass count is claimed.

The shared Puzzle retained factory no longer blanket-rejects checkpoint owners. All Puzzle2d, Puzzle3d, and Puzzle5d accepted factory routes now use one exact 112-byte `PZCP` v1 state carrying:

- operation, base revision, generation, and deterministic seed authority;
- route/tool and original wire-input fingerprints;
- full-width raw length, page, and byte cursors;
- semantic extent, preflight cursor, and custom work-progress cursor.

Restore validates the sealed fixed-page checkpoint and command owner before transfer, rejects empty/single/max+1/corrupt/stale/wrong-route state with exact owner handback, and reconstructs dynamic custom work deterministically one bounded replay step per host grant. Work progress now checkpoints before its bilingual preview, so custom cursor state is resumable rather than silently skipped. The accepted checkpoint owner remains retained by the job and is recursively retired with the command wire owner during incremental close.

The replay audit found every retained Puzzle5d allocation that could otherwise depend on the process-global id counter: `proximityConnect`, missing-id board `edgeCreate`, `addNode`, standalone/nested `addBrushPart`, and `createFastener`. `PuzzleCommandWork::bind_operation` now supplies exact operation/generation/seed authority to those workers. Fresh ids derive from that stable nonce plus a replayed local cursor, nested board brushes receive a disjoint two-id cursor range, and completion after a restored checkpoint cannot advance a global counter into a different semantic digest.

Restore also fails closed if deterministic replay reaches a later phase or passes the recorded custom work cursor without reconstructing the exact checkpoint state. This prevents a structurally valid but semantically unreachable checkpoint from yielding forever. Cancellation still wins before every replay unit, and interrupted close clears replay state before recursively closing both retained fixed-page owners.

Checkpoint payload-page backpressure is retained as explicit `checkpoint_pending` state. If the host cannot admit the single exact 112-byte page, the rejected page source is retired and the job yields without advancing any raw, preflight, or custom-work cursor; the identical state is retried before the next semantic unit. Successful publication alone clears the pending flag. Close clears the flag before owner retirement, so an interrupted unacknowledged checkpoint cannot strand replay state.

Recursive close now preserves the nested raw/checkpoint/work release counts and converts a nested owner's `Complete` into outer `Pending` while later owners remain. Zero-item grants cannot drop an empty fixed-page wrapper, raw scratch bytes retire up to (never beyond) the granted byte count, and only the fully terminal-empty outer job reports `Complete`.

Each Puzzle language-neutral fixture now declares `checkpointBytes: 112` and six exact checkpoint vectors: empty, single-byte, exact max, max+1, corrupt magic, and interrupted-close restore. Bun independently parses all three ledgers with exactly six checkpoint vectors each. The Rust/Serde oracle asserts every extent and expected result, the corrupt-magic mutation, and the interrupted work-progress close grant of one item plus one live framework page (16,384 bytes) for all three production catalogs. The codec round-trips full-width custom cursor state, the interrupted-close regression requires bounded retirement of both input owners, the divergent-cursor regression requires fail-closed replay, and a factory regression requires every owner factory to call both checkpoint validation and adoption.

Source-only evidence after these edits:

```text
bun ./📜️script.ts verify interactivity tool-jobs --self-test
exit: 0
[verify interactivity tool-jobs] self-tests=460 clean.

bun ./📜️script.ts verify interactivity tool-jobs
exit: 1
production-hosts=50 production-invocations=50 production-rows=774
admitted=174 remaining=765 factories=21 registrations=179 self-tests=460
```

The full verifier failure listed concurrent FEM forged-bounded rows, Procedural2d missing proofs, and repository-global remaining/import/store ledgers; it did not list a Puzzle-specific production failure. That is source/static evidence only, not native retained runtime, Wasm runtime, browser, or 8 ms timing acceptance.

The final source-only replay audit additionally passed both edited Rust files through `rustfmt --edition 2021 --emit stdout` with exit 0, so the retained session and Puzzle5d editor parse after the deterministic-ID and divergent-cursor repairs. Puzzle-owned `git diff --check` remains clean. A later root self-test retry did not reach the tool-job verifier: shared taxonomy loading failed at `discovery/component.ts:674` because eight `taxonomy-{inventory,plan,apply,verification}-{data,summary}` semantic directory kinds no longer resolved uniquely. The earlier 460-clean result remains the last completed self-test evidence; the later attempt is an external pre-verifier blocker, not a Puzzle pass or failure. Cargo/Nx reruns remain deliberately unstarted while the coordinator reserves the compiler slot for the shared ARC1 runtime test.

## Fresh Checkpoint Compiler and Retained Runtime Evidence

The queued serialized compiler sequence subsequently ran against separate ticket-local targets:

```text
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=.../🎯️target-puzzle-checkpoint-native cargo check --locked -p semio-s-plugin-puzzle --lib --message-format short
exit: 0
Finished in 23m23s
Puzzle warnings: 157

CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=.../🎯️target-puzzle-checkpoint-wasm cargo check --locked -p semio-s-plugin-puzzle --lib --target wasm32-wasip2 --message-format short
exit: 0
Finished in 15m52s
Puzzle warnings: 158
```

These are fresh native and `wasm32-wasip2` package/library compile results. They do not establish Wasm runtime, browser, or 8 ms timing acceptance.

The first retained-test retry reached Puzzle's lib-test target and reported 14 test-only errors: three stale ASCII `include_str!` filenames for the emoji source, one fixed-tree key comparison, two fixed-map lookups, six async app-dispatch helper returns, one unawaited VCS dispatch, and one attempted mutation through `Arc<SceneConfig>`. The exact repairs use the emoji filename, semantic `UiText::as_str`/fixed-map iteration, test-local future resolution, the existing async test await, and `Arc::make_mut`. No production route behavior was altered by these 14 test-lockstep repairs.

The second retained-test retry compiled the full Puzzle lib-test target successfully and began 26 filtered tests. It then aborted before a pass summary:

```text
running 26 tests
editor::puzzle2d::engine::brush::tests::board_fill_candidate_acceptance_exposes_every_retained_field_stage
thread has overflowed its stack
fatal runtime error: stack overflow, aborting
exit: 101 (SIGABRT)
```

The overflow arose from overlapping large inline fixed-page owner frames: the outer test reserved a `BoardFillCheckpoint` while nested job/session admission moved the same roughly 300 KiB retained state through result and exact-handback frames. The source repair preserves the mounted retained route, every field-stage assertion, checkpoint ACK, recursive close, and terminal-empty law. It moves checkpoint extraction and retirement into `retire_checked_out_fill_checkpoint`, whose frame begins only after admission has unwound. It does not skip the test or increase the harness stack. The edited test file parses with `rustfmt --edition 2021 --emit stdout`. A focused runtime rerun remains pending the coordinator's compiler-slot release, so the stack repair is not yet claimed as passing.

The official verifier baseline reported by the coordinator is now 466 clean self-tests. Puzzle verifier/Nx has not been rerun after the retained runtime gate and is not yet claimed.

The subsequent official source census initially appeared to find two Puzzle3d declaration gaps. A production-only verifier trace corrected that diagnosis: `setActiveTool` and `setActiveUtility` are framework-injected shared actions, and the verifier intentionally cannot derive plugin-owned production rows from their imported framework constants. They are therefore removed from Puzzle3d's `OpBinary::TOOL_JOB_IDS`, app-owned retained inventory, bounded proof list, and manifest dispositions. The typed command variants and direct handler remain because the shared framework actions still enter the typed Puzzle command surface. A regression now requires the Puzzle retained catalog to exclude both shared action ids.

## Focused Retained Rerun and BoardFill Descriptor Backing

The next focused command used the warmed isolated native target:

```text
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=.../🎯️target-puzzle-checkpoint-native cargo test --locked -p semio-s-plugin-puzzle --lib retained --message-format short
compile: finished in 9m04s
runner: 26 filtered tests
exit: 101 (SIGABRT)
```

Two independent failures were observed before the abort:

- `engagement_repeat_is_a_direct_retained_fill_request` reported `FAILED`. Production retained behavior was intact. The negative source assertion searched its own `include_str!` test body and therefore found the hostile fallback literal embedded in that test. The law now classifies only the production prefix before `//#region 🧪️Testkit`, and a hostile production-prefix replacement proves that a real fallback remains rejected.
- `board_fill_candidate_acceptance_exposes_every_retained_field_stage` still overflowed the harness stack. Moving checkpoint retirement to a sibling function was insufficient and is not accepted as the fix.

Compile-free layout inspection found the retained page items were already heap-backed, while each `BoardFillFixedPages<T, CAPACITY>` kept its descriptor table inline as `[Option<BoardFillPage<T>>; CAPACITY]`. The two dominant fields were 4,256 source descriptors and 4,000 virtual-handle descriptors; every move through `BoardFillJobState`, `BoardFillCheckpoint`, `BoardFillJob`, admission rejection, and nested `Result` inherited that maximum inline variant.

With narrowly expanded framework authority, the descriptor table is now an exact-length `Box<[Option<BoardFillPage<T>>]>` built through checked byte preflight, `try_reserve_exact`, exact resize, and a length equality gate. The const-derived item capacity is unchanged. Existing per-page allocation, pop, page retirement, and terminal-empty behavior is unchanged. APIs that already return a capture/job error now use the fallible descriptor constructor and preserve the pending owner on failure.

The new source-only shared law checks:

- exact 4,256 and 4,000 descriptor lengths;
- four-machine-word inline owner size for both largest descriptor tables;
- a 1 KiB snapshot bound and 32 KiB bounds for capture, ingress, placement, typed commit, encoder, state, checkpoint, job, checked-out outcome, mounted session, rejection, and nested transfer-result frames;
- empty-owner terminal state;
- overflow rejection before descriptor reserve.

The language-neutral fixture is `🧪️puzzle2d-board-fill-descriptor-backing.json`. Both edited Rust sources parse through `rustfmt --edition 2021 --emit stdout` with exit 0.

## BoardFill Runtime Boundary and Retained Failure Triage

The focused shared infinite-canvas lib-test command did not reach the descriptor law. It exited 101 on 48 unrelated pre-existing world/DAG lib-test errors, beginning with the missing `../../🌍️world/component.rs` fixture at `🌍️world/🦀️component.rs:13363`. The edited BoardFill production module produced warnings but no compiler error, and the infinite-canvas production library completed successfully as a dependency of the subsequent Puzzle command.

The next isolated Puzzle command was:

```text
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=.../🎯️target-puzzle-checkpoint-native cargo test --locked -p semio-s-plugin-puzzle --lib retained --message-format short
compile: finished in 12m30s
runner: 27 tests
exit: 101 (SIGABRT)
```

Before the abort, 12 tests reported `ok`, seven preview/fixture tests reported `FAILED`, and `engagement_repeat_is_a_direct_retained_fill_request` passed. The BoardFill field-stage test had not yet reported. The abort was independent: `retained_command_catalog_excludes_framework_owned_shared_actions` unnecessarily constructed a Puzzle3d app/store, whose Drop correctly rejected the missing terminal-empty shallow-shell witness at framework store `component.rs:14724`; a second destructor panic during unwind caused SIGABRT. The catalogue law now checks the two ids directly and does not construct an unrelated store owner.

Running the already-built binary directly, single-threaded and exact-filtered, then proved that the heap descriptor repair removed the stack overflow:

```text
semio_s_plugin_puzzle-62fb4e70155a1dc6 --exact editor::puzzle2d::engine::brush::tests::board_fill_candidate_acceptance_exposes_every_retained_field_stage --nocapture --test-threads=1
runtime: 4.34s
exit: 101
failure: Puzzle2d brush component.rs:934, `field cursor session entered invalid phase`
```

The law reached retained worker execution rather than exhausting the stack. Its remaining semantic error was test-side phase handling: unlike the existing mounted driver, the field-stage loop accepted only `WorkerJobPoll::Outcome`, even though `pump_one` checks out terminal outcomes and reports `WorkerJobPoll::Terminal`. It now inspects both checked-out phases and retains the exact checkpoint/preview ACK and close laws. This source repair requires a fresh binary before a pass can be claimed.

Exact-filtering the other failures exposed three independent source mismatches:

- The retained preview encoder's `candidate_ghost` cursor incremented from the mutated subfield after emitting the opening brace/numeric source field. It therefore skipped `targetVortexFullId` and `meshUrl`, producing invalid `{,"objectKindId"...` JSON. The cursor now captures the active subfield and advances only after the corresponding quoted field completes. This is the common cause of the six failed preview oracle/boundary laws; fresh execution is pending.
- All three language-neutral retained fixtures declared the interrupted-close byte grant as 4,096 while the live framework page is 16,384 bytes. All three now declare 16,384, and Bun parses the three ledgers successfully.
- The fill-envelope supersession law saved the replacement pointer created by the weight change, then intentionally superseded that unadmitted replacement again during mesh registration and incorrectly expected the first pointer after close. It now witnesses both distinct replacements, proves neither aliases the admitted owner, and requires the final mesh replacement to survive closing the old admitted envelope.

The edited Rust sources parse under `rustfmt`; whole-file `rustfmt --check` remains nonzero on existing formatting differences outside these narrow changes and is not reported as a formatting pass. No fresh native retained pass, fresh post-repair native/Wasm library pass, verifier, Nx, Wasm runtime, browser runtime, or 8 ms timing result is claimed at this boundary.

## Puzzle5d Import-Media Strict Source Audit

The admitted `import-media` route required a stricter audit than the production-row verifier provides. Its prior implementation parsed the whole media JSON, cloned/deserialized whole catalog rows, grew indexes and mutation vectors opportunistically, assembled every vortex grip in one iterator collection, compared a deep initial catalog clone at completion, and dropped nested typed allocations behind shallow vector retirement. Verifier admission alone was therefore not accepted as runtime or timing evidence.

The Puzzle-owned importer is now an explicit schema-first state machine. A one-framework-page media cap is checked before Serde. Root, object-kind, vortex, grip-kind, and compatibility fields fail closed on unknown schema keys. The canonical producer's optional `schema: "manifest"` plus empty `cableKinds`/`attractionKinds` arrays remain accepted; non-empty unmapped rows fail closed instead of being silently discarded. Every JSON collection is capped at 32 descriptors, nested vortex rows are censused before any semantic reserve, and the exact combined decoded cap is 1,184: at most 1,024 nested vortices plus five final 32-row catalog/compatibility owner domains shared between snapshot and incoming rows. Every catalog/index/compatibility owner is capped at 32 descriptors and the mutation owner at 65 descriptors. Existing typed catalogs load one row per turn; pre-reserved linear tuple indexes build one row per turn; each object-kind uses separate part construction, nested reserve, per-vortex, and publication cursors; a `catalog_changed` bit replaces the former deep equality clone. Compatibility mutations are assembled into their pre-reserved owner incrementally, and the catalog-replacement mutation is assembled in its own `CatalogMutation` turn before completion hands the already-built mutation owner to the completion authority.

The former synchronous `ArtifactEditor::import_media` fallback is now closed with `MediaError::NotImplemented`; it no longer retains a second live whole-document parse/clone/map/collect implementation beside the production reserved route. The canonical mapping and repeated-delivery/idempotence tests now invoke `PluginApp::import_media` on the registry-backed app, so they can only pass through the exact `Puzzle5dImportJobFactory`/`build_reserved_tool_job` path. Additional retained tests exercise the exact 16,384-byte dispatch and 16,385-byte pre-Serde rejection through that same production route.

Cancellation is checked before every stage. Each checkpoint is an exact 33-byte state containing stage, outer cursor, per-vortex nested cursor, decoded-item census, and progress; this removes the prior ambiguity where two distinct nested import positions could publish the same cursor state. The close path recursively retires imported JSON, the in-flight part, compatibility rows, every typed catalog row and nested string/vector owner, index keys, preassembled compatibility and catalog-replacement mutations, raw/media/port owners, snapshot authority, completion authority, and commit envelope. The replacement-catalog mutation now has its own recursive close law instead of being an unexpected variant after cancellation between assembly and completion.

Source-only evidence at this checkpoint:

```text
rustfmt --edition 2021 --emit stdout <Puzzle5d editor component> >/dev/null
exit: 0

bun -e <language-neutral strict ledger validator>
exit: 0
schema=semio.puzzle5d.import-media-retained-law.v1
stages=28 vectors=13
mediaBytes=16384 semanticItemsPerOwner=32 decodedItems=1184 mutationItems=65
semanticUnitsPerGrant=1 closeItemsPerGrant=1 closeBytesPerGrant=16384

bun -e <production-only import state-machine classifier>
exit: 0
missing=[] present=[] productionBytes=52276

git diff --check -- <Puzzle/plugin, authorized BoardFill source, Puzzle ticket evidence>
exit: 0
```

The language-neutral ledger is `🧪️puzzle5d-import-media-strict.json`. It declares empty/single, media max/max+1, decoded-items max/max+1, semantic-owner max/max+1, nested-vortex max+1, non-empty unmapped rows, cancellation at every stage, corrupt-row, and interrupted recursive-close vectors in English and German with no default locale. The Rust laws include the exact/max+1 pre-Serde boundary, a native 7.5 ms parse assertion, native/Wasm descriptor-layout assertions for every semantic vector and tuple index, recursive typed close, recursive preassembled replacement-mutation close, and hostile source mutations that remove the per-vortex, recursive-close, or catalog-assembly stage.

This is source/static evidence only. The native timing assertion has not executed, the end-to-end retained importer route has not yet executed, and the current source has not received fresh native/Wasm type-check evidence. Cargo and Nx remain intentionally idle for Puzzle while another owner holds the serialized compiler slot.
