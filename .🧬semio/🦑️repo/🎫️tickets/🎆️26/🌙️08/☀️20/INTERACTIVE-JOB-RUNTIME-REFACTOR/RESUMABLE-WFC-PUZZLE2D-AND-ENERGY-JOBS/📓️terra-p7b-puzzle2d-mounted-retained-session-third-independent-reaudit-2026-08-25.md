# Terra Third Independent P7b Mounted Retained Fill Re-audit

Date: 2026-08-25  
Auditor: Terra independent read-only source/static audit  
Verdict: **RED — do not accept P7b.**

## Scope and method

I reread the P7 master plan, P7b contract, both earlier Terra RED reports, and the updated Sol report. I independently traced the current production Puzzle2d editor/action route, `ArtifactBoardFillJob`, `BoardFillSnapshotIngress`, fixed registry, terminal/apply/close routes, localized UI, and source fixtures. This is a new report; it preserves both earlier RED reports.

No production source or shared verifier was edited. No Cargo, Nx, Wasm, browser, build, or runtime gate was run. The source-only results below are limited to the tree inspected on this date.

## Decision

The two prior P0s are genuinely closed: the dedicated fill branch no longer constructs a `BoardHost` or `RefCell`, and `ArtifactBoardFillJob` now advances an explicit field/byte cursor exactly once per worker step. The current worker, ingress, fixed-page handback, registry, typed terminal, and UI paths preserve the earlier green findings.

P7b remains RED because every mounted fill action still takes a whole `Puzzle2dConfig` clone before it can reserve, enqueue, poll, or close the retained session. That config contains unbounded `Vec<serde_json::Value>` and `BTreeMap<String, …>` owners. This is a production whole-snapshot copy and ordinary deep-drop on the UI/action route; the claimed minimal `Puzzle2dFillActionCtx` instead exposes the entire dynamic config and growable output vectors. It violates the P7b contract's action-only and no-whole-snapshot-copy requirements.

## P0 — Every fill continuation deep-copies the whole dynamic config before dispatch

`Puzzle2dPlayApp::handle` classifies all eight fill lifecycle actions at [editor component.rs:936](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:936). The early fill branch then unconditionally executes `let mut runtime = config.clone()` before action dispatch at [editor component.rs:1014](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:1014). It then makes whole-config `Puzzle2dConfigMutation::Snapshot` output at [editor component.rs:1022](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:1022).

This is not a fixed scalar runtime projection:

- `Puzzle2dConfig` derives `Clone` at [config component.rs:74](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs:74) and owns `lod_mode_by_pane` and `engagement_input_by_pane` `BTreeMap`s at [lines 86–88](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs:86), arbitrary `brush_candidates: Vec<Value>` at [line 92](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs:92), and more map/string authorities at [lines 124–134](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs:124).
- The alleged minimal context is in fact `&mut Puzzle2dConfig` plus growable `Vec<Effect>` and `Vec<Puzzle2dMutation>` at [set-fill-count component.rs:14](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs:14). It does not carry only the lifecycle scalars necessary for a mounted session continuation.
- The clone happens even for a stale-generation no-op or a poll that produces no config mutation; then the complete dynamic duplicate is ordinarily dropped at the end of the branch. When a lifecycle scalar changes, the full clone instead crosses the snapshot mutation boundary. Neither path has per-item/page admission, retained copy cursor, exact +1 handback, or a close witness for the cloned owners.

This conflicts directly with the contract: the UI action may admit/enqueue but may not perform the former whole mounted work ([P7b contract:34](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-WFC-PUZZLE2D-AND-ENERGY-JOBS/📓️p7b-puzzle2d-mounted-retained-fill-repair-contract-2026-08-23.md:34)); production must remove whole snapshot copies and preserve authoritative owners ([contract:88](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-WFC-PUZZLE2D-AND-ENERGY-JOBS/📓️p7b-puzzle2d-mounted-retained-fill-repair-contract-2026-08-23.md:88)); and every close opportunity is bounded to one scalar/page/slot/control owner with no ordinary populated `Drop` ([contract:101](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-WFC-PUZZLE2D-AND-ENERGY-JOBS/📓️p7b-puzzle2d-mounted-retained-fill-repair-contract-2026-08-23.md:101)).

The current permanent early-branch source predicate only rejects document clone, host construction, host sync, and delta derivation ([editor component.rs:1522](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:1522)). I reproduced it with an additional `config.clone()` injected before the existing clone: it still returned true. Therefore the existing default-host mutation is green but does not prove minimal action authority or reject this whole-config-copy regression.

## Confirmed closures and retained green findings

- The production fill branch begins before the ordinary document clone at [editor component.rs:1014](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:1014). Its exact slice has zero `BoardHost`, `BoardHost::default`, `RefCell`, document clone, host sync, fixture parse, or document-delta call. The normal host route begins only at [line 1025](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:1025).
- `ArtifactBoardFillJob::step` validates cancel, operation/generation, immutable `SnapshotRead` authority, and deadline around the one `capture_one` call; it consumes one fuel unit at [set-fill-count component.rs:669](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs:669). The capture callee has explicit node/handle/kind/template/rule field enums, seven byte-cursor reads, no production loop/iterator/clone/string rebuild, and only one `capture_one` call in the worker step.
- The ingress accepts individual scalar/byte owners and restores the exact owner on fixed-page refusal, for example node [Board component.rs:5431](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs:5431), handle [line 5509](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs:5509), kind handle [line 5629](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs:5629), and rule [line 5702](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs:5702). The incremental ingress close and terminal-empty `Drop` assertion are live at [lines 5730–5800](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs:5730).
- Lease transfer uses the process Interactive pool with `MountedWorkerJobSession`, fuel 1 and 7 ms at [set-fill-count component.rs:1323](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs:1323). Worker close returns the exact `SnapshotRead` through its witness one bounded owner at a time at [lines 710–747](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs:710), and its `Drop` asserts terminal-empty at [line 755](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs:755).
- Terminal handling consumes only `StepOutcome::Complete(candidate)` via `BoardFillResult::from_commit_candidate`; no production `take_result` path remains. Typed paged placement retains source edge kind and reserves the paired node/edge mutation destination before either owner is moved.
- The fixed eight-slot registry reserves before backing allocation, is generation-qualified, returns unmatched/lost checked-out owners to their slots, and uses a one-owner `pump_abandoned_session` close opportunity at [set-fill-count component.rs:1272](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs:1272). The current `begin_fill_job` invokes that pump once at [line 1450](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs:1450).
- All eleven live `try_push_owned` sites in the retained ingress/search region bind `Err(owner)` and restore it; the compatibility-discard mutation is rejected by the Board source law at [Board component.rs:12541](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs:12541).
- UI progress, cancel, retry, and fault controls remain localized in English/German, including the German accessibility fixture at [fill tool component.rs:114](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️component.rs:114).

## Faithful source mutations and static gates

I reproduced the 12 live source mutations without compiling or changing a source file. All 12 baseline predicates passed and all 12 mutations were rejected:

1. restore whole ArtifactView clone;
2. restore BoardHost sync/rebuild;
3. restore whole document delta;
4. restore `BoardHost::default` in the early fill branch;
5. remove the worker `ArtifactBoardFillJob` implementation;
6. restore mutable terminal `take_result`;
7. coalesce node X/Y capture;
8. restore a whole-string capture loop;
9. remove same-turn mutation destination credit;
10. restore whole-node ingress;
11. restore whole-text ingress; and
12. discard the compatibility `Err(candidate)` owner.

The mutation set is GREEN as a preservation gate, but it has a material blind spot: an injected additional whole `config.clone()` leaves the editor branch predicate green.

| Gate | Result |
| --- | --- |
| Edition-2021 `rustfmt --check --config skip_children=true` on Board and twelve scoped Puzzle2d Rust sources | PASS |
| Bun `JSON.parse` on the live Puzzle2d fill-config schema | PASS |
| Scoped `git diff --check` across Board and the P7b Puzzle2d source/schema packet | PASS |
| 12 faithful in-memory source mutations | PASS — all rejected |
| Production route/callee census | FAIL — unconditional whole dynamic `Puzzle2dConfig` clone in every fill action |

## Required closure

Replace full-config clone/snapshot emission for mounted fill continuations with an admitted, bounded fill-lifecycle projection or field-level event/mutation authority. It must leave unrelated maps, `brush_candidates`, strings, and config owners untouched; represent each necessary lifecycle copy/allocation with a retained owner and bounded close; and retain exact ownership on refusal/interruption. The action context must expose only those fixed lifecycle fields and bounded output authorities.

Add a production-slice law and hostile mutation that reject `config.clone()`, `Puzzle2dConfigMutation::Snapshot`, full `Vec<Value>`/map authority, or an extra dynamic config copy anywhere in the eight fill continuations. Re-audit after this closure.

Compiler, runtime, native/Wasm, WorkerPool saturation/panic/cancel/close stress, deterministic 1/2/4/default replay, allocation evidence, and watchdog execution remain deferred and are not acceptance claims in this report.
