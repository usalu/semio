# Terra Independent P7c3 Mounted Energy Product/Session Audit

Date: 2026-08-25  
Tree inspected: `18adc8cce3`, including the current dirty and untracked P7c3 sources  
Method: source/static only; no source changes, build, runtime, browser, Cargo, Nx, Wasm, shared script, or test execution.

## Verdict

**RED — P7c3 is not acceptable at source/static level.** The mounted product path retains a whole-`Model` cache/clone source, command request identities are discarded before session handling, and bounded retirement/close paths can strand or immediately drop deep owned state. These are production-path defects, not fixture wording defects.

This audit preserves, rather than supersedes, the accepted P2a1, P2c, P7c1, and P7c2 boundaries. The P7c2 typed packet/lease interface is present; the defects below occur in the new P7c3 product/session ownership layer.

## Confirmed Foundations

- The session declares one registered process kind (`ENERGY_SIMULATION_JOB_KIND`); its registry uses fixed `16` active slots, `32` shell slots, and `64` event slots. The `MAX+1` fixture contains `17`, `33`, and `65` hostile values.
- The schema is schema-first and expresses start, cancel, retry, discard, and adopt. Start requires either `en` or `de`; the editor exposes EN/DE controls, cancel/retry/discard/adopt actions, busy/status announcements, and keyboard guidance. The viewer has a separate read-only presentation path.
- The session has fixed typed P7c2 preview/checkpoint/commit/fault channels. Its normal channel collection validates the mounted identity and uses the P7c2 checkpoint transfer/ACK interfaces rather than a text packet.
- The capture implementation itself progresses one nested model/config record, one Unicode scalar, or one item at a time, and its preflight census checks capacity before the P7c3 shell is started. That incremental work is nevertheless fed by the prohibited cache described below.
- The four tier projection is retained in a fixed `[Option<_>; 4]` field and consumers use `try_borrow`, not an ordinary blocking borrow, for the visible projection.

## Blocking Findings

### 1. Production capture is cache- and whole-`Model`-based

`🧵️simulation-session/🦀️component.rs` imports and calls `with_energy_model_ref` both in `capture_one` (lines 894–901) and in the pre-admission census (`prepare_snapshot_read`, lines 1753–1763). That function is not an artifact-neutral retained-store read:

- `🔋️model/🦀️component.rs` defines `EnergyWorkingScene { model: Model }` and a thread-local `ENERGY_SCRATCH: RefCell<HashMap<String, EnergyWorkingScene>>` (lines 167–175).
- `energy_children_from_model` serializes the entire model for a cache key and inserts `EnergyWorkingScene { model: model.clone() }` (lines 193 and 205–207).
- `with_energy_model_ref` reads the cached whole `scene.model` (lines 226–233). Its own documentation says this cache can miss after store-level undo/redo.

Consequently the live admission path violates the P7c3 prohibition on cache, whole-route cloning, and serialization, and it does not capture through current document/store child reads. Incremental copying after this lookup does not repair the ownership/source violation.

### 2. Cancel/retry/discard/adopt lose their request identity

The public editor command variants carry `request: u64`, but `✏️editor/🦀️component.rs` lines 186–201 explicitly bind each request then discard it (`let _ = request`). `EnergySimulationEventKind` only retains `Start(config)`, `Cancel`, `Retry`, `Discard`, and `Adopt` (`🧵️simulation-session/🦀️component.rs` lines 99–104). A `SessionEvent` has event sequence but no caller command request or requested operation/generation.

`apply_event_one` therefore evaluates cancel/discard/adopt only against the current render identity (lines 1700–1748). A delayed command for an old operation on the same current app/document render can mutate the replacement session. `Retry` is also unsafe: if there is no matching retryable owner it cancels/retire attempts and then begins preflight with `current.config.unwrap_or_default()` (lines 1670–1698). This permits a stray retry to mint a new default-English operation instead of rejecting the stale request. The schema’s required request field is thus not live authority.

### 3. Capture close performs an ordinary whole-owner drop

The mounted close state machine handles P7c1 job and P7c2 channel/lease pages in earlier lanes, but close lane 11 executes `self.capture.take()` and counts that as one released item. `ModelCapture` contains the partially assembled `Model` and its dynamic backing. There is no `ModelCapture::close_step` or equivalent pagewise release driver. A cancellation, stale transition, document/app/window close, or fault during capture can therefore destruct the complete partial model in one grant rather than retiring one admitted semantic owner/backing page/control at a time.

The fixture only asserts one-item *capture construction* and does not exercise this close path. It cannot prove bounded terminal retirement.

### 4. Retirement capacity is checked after moving the new deep owner

In `reconcile`, the session calls `state.admit_job(checkpoint)` (around lines 1905–1910), consuming `self.capture.take().finish()` and moving the completed model into the new job. Only afterward does it call `registry.retire(previous.shell)` (around lines 1923–1927). If the retirement arena is full, it returns `Vec::new()`.

At that point the new shell remains in `pending`, but it has `job: Some` and `capture: None`; the pending-shell predicate accepts only a state with no job and some capture. It is neither current nor in the retirement queue and its deep job owner cannot be mounted or re-admitted. The rejection branch has the same ordering issue. Start/retry/discard event handling also pops an event before returning when retirement is full, losing the semantic action rather than retaining/rejecting its exact owner.

This directly violates pre-reserved recovery/retirement capacity and the MAX+1 exact-owner rule.

### 5. Normal terminal and lost-handle paths are not exact bounded recovery

`worker_step` marks a cancelled job terminal and returns `Done`; adoption likewise returns `Done`. The bounded job sets `clean_terminal = true`, after which `Drop` immediately returns. `maintenance_step` moves a state to retirement only when its `abandoned` bit is set, not when it has normally become Cancelled, Adopted, or FinalReady. Such a job therefore remains in `current` with its owned job until an unrelated discard/new-start/app-close happens.

Further, the non-clean `Drop` uses `self.shell.try_borrow_mut()` and silently does nothing on borrow failure. No fixed recovery record is published for this lost-handle case. That fails the required exact owner recovery for Drop/panic/lost-handle paths even though the normal P7c2 packet close machinery is available.

### 6. The advertised law/cap coverage is fixture/static-string coverage, not live authority proof

The session contains fourteen local tests, and the P7c3 fixture supplies hostile caps plus booleans. The `1/2/4/default` chronology law iterates local tier/fuel values; it does not run a mounted `EnergyJob`, a bounded job grant, or the shared renderer pool. The lower-tier freshness law compares a local number, and the accessibility law searches included source text. There is no mutation corpus that drives a stale request, retirement-full transition, capture close, lost handle, or exact P7c2 ACK ownership through the production registry.

The live `17/33/65` constants and registration were inspected, but their current tests cannot falsify the source defects above and are not acceptance proof.

## Static Gate Record

| Gate | Result | Scope |
| --- | --- | --- |
| `rustfmt --edition 2021 --check` | GREEN | P7c3 session/editor/viewer/model Rust sources plus the relevant P7c2 glue; included untracked P7c3 Rust files. |
| `git diff --check --` | GREEN | Tracked Energy P7c3 support changes. Git does not include untracked P7c3 files in this check. |
| `jq empty` | GREEN | P7c3 session schema and event fixture. |
| Static source/caller/registration census | RED | Production ownership and freshness paths above. |

## Deliberately Deferred Executable Gates

Per assignment, no Cargo/Nx/Wasm/browser/runtime execution or mutation script was run. Runtime confirmation remains required after the RED source defects are repaired, including actual one-grant capture/close behavior, EN/DE command identity refusal, 1/2/4/default pool routing, and every cancellation/fault/Drop/retirement-full boundary.
