# 📓️ `sdk-dropped-futures` report — 97 dropped futures in `semio-framework-plugin`, all fixed

## Result

Forced-rebuild (`cargo clean -p semio-framework-plugin` then fresh `cargo check`) count of
`unused implementer of \`std::future::Future\`` warnings in `semio-framework-plugin`: **0** (was 97).

| check | result |
|---|---:|
| `--lib` (forced rebuild) | **EXIT 0**, 0 target warnings, 18 warnings total (was 115) |
| `--lib --all-features` | **EXIT 0**, 0 target warnings, 20 warnings total |
| `--all-targets` | **EXIT 101, 1373 errors** — pre-existing `#[cfg(test)]` residue, NOT caused by this packet, see "Acceptance gate 3" below |
| `semio-framework-os-kernel --lib` | **EXIT 0**, 57 warnings (unchanged baseline) |
| `cargo test -p semio-framework-os-kernel --lib` | **779 passed / 0 failed / 0 ignored** (unchanged baseline) |

All numbers above were run by me this session, in the foreground, `CARGO_TARGET_DIR` pointed at the
session scratchpad (`.../scratchpad/target-drop`), pasted verbatim from the actual command output —
none reused from a prior run. sol independently re-verified the same four numbers before this
write-up and confirmed the tree matches.

## Files touched (all inside the granted `path_scope`, `🖥️host/**` untouched)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs`

## ⚠️ Attribution note — the coordinator's raw diff-vs-HEAD numbers include OTHER packets' work

The coordinator's message quoted a diff of "280 added `.await`, only 1 `let _ =`, 5 spawn, 8
`🚫️async` E-tags" over these same 4 files. I re-ran that diff myself and it is accurate as a
description of the CURRENT working tree vs `HEAD` — but `HEAD` predates several other already-landed
packets that share these exact files (`sdk-features` added 27 `.await`s here before I started;
`dispatch-group-split` restructured `CompositionCoordinator` dispatch here too; `sdk-final` added the
8 E1/E1-adjacent tags visible in the diff, all on `panel_tab_spec_to_definition` /
`peers_selecting`/`peers_hovering` and similar pure helpers — none of which this packet touched).

**What THIS packet actually added, verified against my own edit history, not re-derived from the
diff:**
- **~96 `.await` insertions** — 95 applied by a span-keyed script (see Tooling below) against
  rustc's own 97-warning JSON capture (2 of the 97 were deliberately excluded from the script and
  hand-fixed, see below), plus 1 more inside the bonus `let _ = member.checkout(...).await;` fix.
- **5 `Box::pin(...).await` wraps** (self-recursive async fns — these already carry an `.await`
  token each, counted in the 95 above, not additional insertions).
- **0 new `// 🚫️async: E<n>` tags.** Every one of the 97 sites' enclosing function was already a
  legal `async fn`/`pub async fn` — none needed an exception tag. The 8 tags visible in the file
  diff are `sdk-final`'s, not this packet's.
- **0 new task-spawn ("fire-and-forget, detached") sites.** The "5 spawn" the coordinator's grep
  matched are substring hits on the fault-message strings `"spawn-job"`/`"spawn-plugin-instance"`
  (from `sdk-features`'s `direct_unavailable_fault("spawn-job")` calls), not actual `.spawn(...)`
  task-detach calls — I checked the diff lines directly, they are not spawns at all.
- **1 `let _ = ...await;`** — the bonus fix at `🦀️component.rs`'s `cascade_checkout_to_children`
  (below), matching the coordinator's count exactly.

I flag this because misattributing `sdk-final`'s/`sdk-features`'s/`dispatch-group-split`'s edits to
this packet would corrupt the ticket's audit trail on exactly the kind of finding this packet exists
to get right.

## Tooling (R10-compliant, span-keyed, saved in the ticket folder)

`terra-sdk-dropped-futures-await-fixer.py` — adapted from the already-vetted
`terra-oskat-unused-future-fixer.py` (operates in raw BYTES throughout — reads bytes, indexes bytes,
writes bytes — so it does not repeat the byte/codepoint-offset confusion documented in
`📓️terra-alltargets-hard-report.md`'s "severe bug in a tool I wrote" section). Reads a
`cargo check --message-format=json` capture, extracts every "unused implementer of Future" warning's
primary span, and appends `.await` at the exact byte offset rustc names — except for a 7-line
exclude-list of sites that need special handling (below), which it prints as explicitly skipped
rather than touching.

**Tool bug I hit and caught before it mattered**: my first exclude-list match was a filename
*suffix* check (`file_name.endswith("🔌️plugin/🦀️component.rs")`), which silently failed for the
main SDK file because its `cargo`-reported path runs through the crate's package indirection
(`.../📦️packages/🦀️rust/../../🦀️component.rs` — the `../../` defeats a suffix match). 2 of 7
exclusions (the `⚛️reactor/💼️jobs` ones) matched fine since that file has no such indirection; the
other 5 (all in the main `🦀️component.rs`) did not, and got a plain `.await` appended instead of
being skipped. I caught this immediately by reading the resulting diff (not by re-running the tool
blind), confirmed the 5 sites were exactly the ones needing `Box::pin`, and hand-fixed each via
`Edit` before ever running a build. No corrupted build was ever compiled or shipped — this is
recorded per R10's "if you build a recovery tool... save it" spirit, and as a documented caution for
whoever reuses this script: **the exclude-list match must be a full relative-path match through the
package's `../../` indirection, not a bare suffix.**

## Pattern groups and treatment

All 97 sites' enclosing function was already declared `async fn` — none required an E1–E5 exception
tag. Every site was genuinely dropped work (category 1 in the packet brief), except two shapes that
needed a specific *mechanism* to await correctly rather than a bare `.await`:

| group | file | sites | shape | fix |
|---|---|---:|---|---|
| A | jobs | 2 | `spawn_job(...)` called from `start_job`/`restore_job`, no `.await` | plain `.await` |
| B | jobs | 2 | `executor.wake`/`run_until_idle` inside a `LocalKey::with` **sync** closure | `resolve_ready(...)` bridge (E0728 bars a bare `.await` here) |
| C | jobs | 3 | `ctx.progress`/`ctx.checkpoint` in `run_two_phase`, no `.await` | plain `.await` |
| D | host | 23 | `self.emit(Effect::X{...})` — every effect-emitting wrapper method | plain `.await` |
| E | builder | 1 | `register_job_kind(kind, run)` inside `try_build()`'s loop | plain `.await` |
| F | component | 2 | `visit(...)` topology tree-walk — 1 self-recursive call, 1 outer call | recursive call: `Box::pin(...).await`; outer call: plain `.await` |
| G | component | 10 | `require_declared_capability_or_record(...)` in every `ArtifactDeclarationBuilder` method | plain `.await` |
| H | component | 6 | `validate_panel_tab_spec`/`validate_arg_defs` top-level assembly calls | plain `.await` |
| I | component | 2 | `ui_tree_stamp_presence(...)` | plain `.await` |
| J | component | 12 | `push_log_entry`/`record_command` (the command-history log) | plain `.await` |
| K | component | 7 | `self.<store>.set_local_actor_id(...)` | plain `.await` |
| L | component | 1 | `self.absorb_created_children(...)` | plain `.await` |
| M | component | 4 | `set_merge_policy`/`reset_dispatch_report`/`apply_merge_policy` chain | plain `.await` |
| N | component | 4 | `self.stamp_and_cache_interaction_ui(child, state)` — self-recursive over `UiNode` tree | `Box::pin(...).await` |
| O | component | 1 | `self.cascade_checkout_to_children()` | plain `.await` |
| P | component | 2 | `presence_store.adopt_peer`/`remove_peer` | plain `.await` |
| Q | component | 1 | `self.store.detach_backbone()` | plain `.await` |
| R | component | 8 | `ensure_plugin_initialized`/`ensure_extension_initialized` — every WIT export entry point | plain `.await` |
| S | component | 4 | `push_dispatch_fault`/`push_os_fault`/`push_app_fault` | plain `.await` |
| T | component | 2 | `assert_extends_matches_primary_dependency()` | plain `.await` |
| **total** | | **97** | | |

(2+2+3 = 7 jobs; 23 host; 1 builder; 2+10+6+2+12+7+1+4+4+1+2+1+8+4+2 = 66 component — matches the
97/66/23/7/1 split in `📌️important.md` exactly.)

## Per-site behaviour table — what was actually missing

This is the part that matters: not "added `.await`" but what silently did not happen before.

| group | function(s) | behaviour that was silently missing |
|---|---|---|
| A | `spawn_job` | Under `#[cfg(feature = "component-guest-async")]`, `spawn_job` awaits `crate::reactor::host()`; dropping the call meant `start_job`/`restore_job` (the WIT `jobs::start-job`/checkpoint-restore entry points) never actually registered the job's task on `JOBS_EXECUTOR` under that feature — every job silently failed to start. |
| B | `LocalExecutor::wake`/`run_until_idle` | **The core of the whole job-slicing mechanism.** `step_job` calls these to re-queue the job's task and actually poll it. With both dropped, `run_until_idle` never runs a single poll — `state.outcome`/`state.progress` never populate — so `step_job` would **always** return `Running(None)`, and the stall guard would eventually fail **every job** with `job.stalled` no matter what it does. The module's own elaborate slicing-mechanics doc comment (lines 1–46) described a mechanism that, before this fix, could not have executed a single job to completion. |
| C | `JobCtx::progress`/`checkpoint` | Job progress bytes and checkpoint bytes were never recorded. `JobStep::Running(progress)` would always report `None` even mid-job, and checkpoint/restore would never actually skip the decoded phase on restart (since `checkpoint()` never wrote the checkpoint bytes `restore_job` later reads). |
| D | `HostAdapter::emit` (23 wrapper methods) | **Every effect-emitting host API in the SDK was a no-op.** `emit`'s body genuinely does work — `registry.emit(effect).await` (Poll backend) or a real cross-boundary `direct::host_async::emit(...)` call (Direct/wasm backend). Dropping it at each of the 23 call sites meant `send_message`, `publish_event`, `subscribe`/`unsubscribe`, `respond`, `set_timer`, `cancel_job`, `close_window`, `notify`, `clipboard_write`, `navigate`, `open_external_url`, `set_panel`, `set_active_utility`, `set_active_tool`, `patch_world3d_chrome`, `replay_shell_command`, `download_media_export`, `icon_render_export`, `load_document`, `open_plugin_instance`, `request_sync`, and `release_capability` **all silently did nothing** — no message sent, no event published, no window closed, no timer armed, nothing. This is the single widest-blast-radius finding: every plugin built on this SDK calls at least some of these. |
| E | `register_job_kind` | Plugin-declared job kinds were never registered into `KIND_REGISTRY` during `try_build()`. Every plugin-authored job kind (`jobs()` builder entries) would have resolved to `JobBody::UnknownKind` at `start_job` time — the entire "makes jobs authorable" design (module doc, line 1) was unreachable for any plugin-declared kind; only the hard-coded builtins (`job_io_run` etc., registered elsewhere) worked. |
| F | `visit` (topology walk) | `ui_tree_domain_topology` builds `DomainTopology.ordered` by walking a `UiTree`'s nested sections. With the recursive call dropped, only the TOP-LEVEL items of the first level got visited (or not even that, at the outer call site) — nested `UiTreeItemNode.items` children were never appended, so `HierarchyProvider::UiTree`'s self-derived topology silently lost every node below the first level. |
| G | `require_declared_capability_or_record` | **The entire capability-declaration validation for `ArtifactDeclarationBuilder` never ran.** This helper checks that a builder-declared capability (`schema`/`inference`/`composer`/`representation`/`subset_validator`/`codec`/`languages`/`document_codecs`) actually matches a declared capability's claim set, and records a `definition_error` on mismatch. Dropped at all 10 call sites, a plugin could declare a schema/inference/composer/etc. whose identity claims don't match ANY registered capability, and `definition_error` would simply never be set — malformed declarations passed builder assembly silently. |
| H | `validate_panel_tab_spec`/`validate_arg_defs` | Assertion-based validators (`assert!` panics on violation: duplicate ids, empty ids, malformed `Select` args, etc.) for an app's panel tabs, actions, app commands, window-kind actions, mode commands, and dialogs. Dropped at all 6 top-level call sites, these `assert!` guard rails never fired — malformed app definitions (duplicate action ids, empty tab ids, a `Select` arg with no options) passed `try_build()` without ever being checked. |
| I | `ui_tree_stamp_presence` | At `UiTreeBuilder::build()` (line 5471): this is the ONLY place selection/highlight ids get stamped onto a `UiTree` that is never later bound to a live `interaction_domain` (`stamp_and_cache_interaction_ui`'s own stamping is gated on `tree.interaction_domain` being `Some`). Dropped, a non-interactive `UiTree`'s `selected`/`highlighted` rows never got their presence stamped at all. At line 11952 (inside `stamp_and_cache_interaction_ui`, for domain-bound trees): the peer-mark/selection/hover presence for the CURRENT tree level was never actually written into `tree.sections`. |
| J | `push_log_entry`/`record_command` | **The entire command-log / undo-redo audit trail.** `record_command` is documented as "the single entry point every live dispatch logs through"; its own body calls `push_log_entry` (which does `self.command_log.push(...)`) without awaiting it — so even where `record_command` itself WAS reached, its actual log-append never happened. With all 12 sites dropped (9 via `record_command`, 3 direct `push_log_entry` backfill calls), no mutation, history action (undo/redo/checkpoint/alternative/revert), interaction change, shell command, config command, or transaction commit ever produced a `CommandLogEntry` — the history panel (`HistoryView`) and every undo/redo path built on `command_log` had nothing to read. |
| K | `set_local_actor_id` | Sets which actor id subsequent `Apply`/`AmendLast` edits get attributed to. Dropped at all 7 dispatch entry points (draft/config/main/interaction/transaction stores, both for regular and child-group dispatch), every subsequent edit on that store kept whatever actor id was set (or wasn't) from a PRIOR dispatch — a real multi-user attribution bug: edits could be silently mis-attributed to a stale or absent actor. |
| L | `absorb_created_children` | Newly created child artifacts from a composite dispatch were never inserted into `self.children` (the slot/dialect bookkeeping this fn performs). A parent that just created children via a composite gesture would not actually see them registered as its children afterward. |
| M | `apply_merge_policy` chain | `apply_merge_policy` itself dropped BOTH of its own two statements (`self.store.set_merge_policy(policy)` and `self.reset_dispatch_report()`) — meaning calling it did **literally nothing**: the merge policy was never actually changed on the store despite the caller believing it was, and the dispatch report was never reset to reflect it. `begin_dispatch_report` similarly never reset the report at dispatch start, and the trait-impl `set_merge_policy` never even reached `apply_merge_policy`'s body (dropped one level up too). |
| N | `stamp_and_cache_interaction_ui` (self-recursive) | Real async work happens in this fn (registry lookups, topology building) for `UiNode::Tree` nodes specifically — but `Stack`/`Section`/`Group`/`Field` nodes only recurse into their children to find nested `Tree`s. With the recursive call dropped, **any `Tree` node nested below a `Stack`/`Section`/`Group`/`Field`** never got its interaction state (selection/hover/peer marks/topology) stamped or cached at all — only top-level `Tree` nodes worked. |
| O | `cascade_checkout_to_children` | After a checkpoint checkout, child artifacts pinned to that checkpoint's `composition_pins` were never actually checked out to match — a checkpoint checkout would leave every child artifact on whatever version it happened to already be at, silently breaking the checkpoint's cross-document consistency guarantee. |
| P | `presence_store.adopt_peer`/`remove_peer` | Peer presence roster updates (a peer joining/leaving, or a peer's decoded presence pack) never actually mutated the roster — collaborative presence (who else is viewing/editing) would silently drift stale. |
| Q | `detach_backbone` | The store's backbone-channel detachment (severing the VCS backbone link, e.g. on instance teardown) never actually ran — a call site expecting the store to be detached would find it still attached. |
| R | `ensure_plugin_initialized`/`ensure_extension_initialized` | **The single most severe finding.** `ensure_plugin_initialized`'s entire body is `PLUGIN_INIT_ONCE.call_once(|| { ...; PLUGIN_BUNDLE_INSTALLER.get().install(); })` — a call gated to run exactly once, which populates the plugin's frozen bundle state (apps, capabilities, `PLUGIN_ASSEMBLY_ERROR`). Dropping the future means the fn's body — including the `call_once` — **never executes at all**. Every one of its 5 call sites is the first statement of a WIT export (`plugin_manifest`, `plugin_wire_list_artifact_inference_services`, `plugin_wire_artifact_infer`, `plugin_wire_list_artifact_mutations`, `plugin_wire_artifact_mutation_plan`): with the installer never invoked, these would have silently operated against an uninitialized bundle — `PLUGIN_ASSEMBLY_ERROR` would stay perpetually unset (looking like success) while no app/capability was ever actually installed. The extension variant (`ensure_extension_initialized`, 3 call sites: `extension_manifest`, `extension_activate`, `extension_invoke`) is the identical bug for the extension bundle path. |
| S | `push_dispatch_fault`/`push_os_fault`/`push_app_fault` | These encode a `Fault`/`DispatchReport` and push an `AppFrame::Error` into the response `frames` list. Dropped at all 4 call sites, a failed dispatch, an unsupported command, or an app-level fault would never actually produce an error frame in the response — the caller would receive no error signal for a failure that DID happen. |
| T | `assert_extends_matches_primary_dependency` | Contract-freeze §3/§4 check: `extends == dependencies[0].plugin_id`, meant to panic on violation. Dropped at both call sites (`.extends()`/`.depends_on()`), a malformed `ExtensionBundle` declaring a mismatched `extends`/primary-dependency pair would pass builder assembly without the intended panic guard. |

## Bonus finding — one more dropped future, NOT among the 97, invisible to rustc's own lint

`🦀️component.rs`'s `cascade_checkout_to_children` (same function as group O above) has a SECOND
call, two lines below the one rustc flagged:

```rust
let alternative_id = member.current_alternative_id().await.unwrap_or_default();
let _ = member.checkout(&pin.checkpoint_id, &alternative_id);   // was missing .await
```

`member.checkout(...)` is an `async fn` trait method (`🏪️store`'s `MemberFactory`-style trait). This
call was `let _ = ...`, which — unlike a bare statement — **completely suppresses rustc's
`unused_must_use` lint**: assigning a `#[must_use]` value to `_` counts as "using" it even though it
is dropped unpolled. I found this by noticing an EXACT sibling of this same call ten lines earlier
(line ~10697) correctly reads `let _ = member.checkout(&pin.checkpoint_id, &alternative_id).await;`
— identical code shape, one instance has `.await`, the other doesn't. Fixed to match its sibling.
**This is worth flagging to the program generally**: `let _ = <async call>;` is invisible to the
exact compiler lint this whole packet exists to close, and I did not do an exhaustive repo-wide sweep
for this pattern — I only checked the 4 files in my `path_scope` (found and fixed the 1 instance
above; also observed 2 more `let _ = <async call>;` sites in this same file, both inside test-only
code paths with genuinely no observable side effect either way — `let _ = Self::command_id(command);`
at a test fixture's `handle()`, and `let _ = <$validator as SubsetValidator>::validate(&payload);`
inside a macro-generated conformance test — I left both as-is since fixing them changes nothing
observable and they carry zero production risk; noting them rather than silently skipping).

## E-tagged sites: none added by this packet

Every one of the 97 sites' enclosing function was already declared `async fn`. None hit an E1–E5
barrier (no external-trait impl, no `const`/`extern`/proc-macro fn, no fn-pointer slot). The 8
`// 🚫️async: E1`/`E1-adjacent` tags visible in `🦀️component.rs`'s current diff against `HEAD`
predate this packet — they sit on `panel_tab_spec_to_definition`, `InteractionView::peers_selecting`/
`peers_hovering`, and similar pure-computation helpers documented in the ticket's own `sdk-final`
cross-packet finding (`📌️important.md` rule 27), landed before I started. I did not touch, move, or
add to any of them.

## The `resolve_ready` bridge sites (jobs.rs `executor.wake`/`run_until_idle`)

`⚛️reactor/💼️jobs/🦀️component.rs`'s `step_job` calls both inside `JOBS_EXECUTOR.with(|executor| {
... })` — `LocalKey::with`'s closure is a plain sync `FnOnce`, so a bare `.await` there is E0728
(illegal). The fix uses the file's OWN already-established bridge (`semio_framework::io::
resolve_ready`, already used twice in this exact file: the `thread_local!` initializer at line 281,
and `spawn_job`'s own `JOBS_EXECUTOR.with` call at line 333) rather than inventing a new mechanism.

**The argument for why this is safe, restated explicitly** (since an E5 bridge over something that
CAN genuinely suspend would be a live bug, not a fix): `resolve_ready` panics if its argument isn't
`Poll::Ready` on the FIRST poll. I read `LocalExecutor`'s full body
(`⚛️reactor/🧵️executor/🦀️component.rs`) to confirm neither `wake` nor `run_until_idle` has a
suspension point of ITS OWN:
- `wake` is pure `RefCell` bookkeeping (`inner.ready.push_back(id)` if not already queued) — no
  `.await` inside it at all beyond nothing.
- `run_until_idle` loops popping the ready queue and calling `future.as_mut().poll(&mut cx)`
  **directly** (not `.await`) on the DRIVEN job's future — that inner future may legitimately return
  `Poll::Pending` (that's the whole point of slicing), but `run_until_idle` handles `Pending` by
  storing the future back in its slot and moving on; it never propagates that pendingness as
  suspension of ITS OWN outer future. Its only two internal `.await` points
  (`self.waker_for(id).await`, `self.has_pending().await`) are themselves calls into equally
  suspension-free helpers (`waker_for` just constructs a `Waker` synchronously; `has_pending` is a
  synchronous `RefCell` read).

So `run_until_idle`'s outer future — the thing `resolve_ready` actually polls — always resolves on
the first poll, regardless of whether the job it's driving internally parks. This is the identical
reasoning already established and load-bearing at this file's line 333 (`resolve_ready(executor.
spawn(...))`), which this packet did not invent, only extended to two more call sites of the same
executor.

## Spawn sites and the one `let _ =`: no detaching was introduced

This packet added **zero** new task-spawn / detached-future sites. Every one of the 97 was
genuinely dropped work whose completion the caller needs before proceeding — none was a case where
"fire and forget, don't wait for it" was the correct intent (category 2 in the packet brief). The
`HostAdapter::emit` group (D) came closest to sounding like fire-and-forget by its own doc comment
("Fire-and-forget dispatch"), but that phrase describes not waiting for the EFFECT'S downstream
processing/response — the `emit` call itself (enqueueing into the registry, or the direct wasm-host
import call) still has to run to completion, which is exactly what `.await` provides; nothing here
warranted an actual `spawn`.

The single `let _ = ...await;` (the bonus fix, group O / `cascade_checkout_to_children`) is not a
fire-and-forget decision either — it discards the `Result<(), VcsError>` (matching its own sibling
line 10 lines above, which already discards the same call's `Result` the same way), while still
awaiting the future to completion. The `let _` is about the return VALUE, not about skipping
execution.

## Test coverage — did any of the 97 sit under a passing test?

**No test in this crate currently passes over any of the 97**, but the reason is structural, not
reassuring: `cargo check -p semio-framework-plugin --all-targets` (which is what `cargo test` would
also need to compile) is **RED with 1373 errors**, entirely inside `#[cfg(test)]` code, and entirely
pre-existing — confirmed matching, by exact signature, the residue `📌️important.md`'s `sdk-final` and
`dispatch-group-split` cross-packet findings already documented before this packet started
(`unresolved import crate::app::__semio_dispatch_PluginApp` / `__semio_dispatch_PluginApp is
ambiguous` / `HybridLogicalTimestamp` errors present; I did not re-derive these, I grepped my fresh
`--all-targets` output for the exact strings those findings named and both are present, 1373 total
errors matches the documented count exactly). I did not touch any test code with production
consequences, and did not attempt to fix this residue — it is explicitly out of this packet's
mandate per rule 25 (atomic packets finish clean or get redirected before start).

**I did find direct, concrete evidence that this residue independently manifests the SAME missing-
`.await` defect class inside this crate's own test code**, which I want to surface rather than bury:
`⚛️reactor/💼️jobs/🦀️component.rs`'s own `#[cfg(test)] mod tests` contains
`a_three_slice_job_returns_running_running_done_with_progress_each_slice` and siblings — tests that
call `register_job_kind`/`start_job`/`step_job`/`cancel_job` (all `async fn`, all functions I fixed
in production code above) **without `.await`**, inside an `#[semio_framework_async_macros::
async_test] async fn` test body. Unlike the 97 production warnings, these don't produce a silent
"unused" warning — because the test code immediately tries to `match step_job(...) { JobStep::
Running(...) => ... }` against the (undropped-but-untyped) future itself, which is a genuine type
mismatch. Confirmed in my fresh `--all-targets` capture: `jobs/🦀️component.rs:682:48: error[E0609]:
no field \`fuel\` on type \`impl std::future::Future<Output = jobs::JobBudget>\`` and similar at
lines 684, and (in the sibling `💡️infer` job kind file) `error[E0308]: mismatched types: expected
future, found \`JobStep\`` at 5 separate lines. These are part of the same pre-existing 1373-error
residue, not something I introduced or fixed — I am naming them because they are the clearest
available evidence that the exact mechanism this packet's production fix restores (job slicing via
`LocalExecutor`) was ALSO never exercised by a passing test even before the residue existed as
1373 errors: the test file's own call sites were already missing `.await` independently of the
production bug, so even a hypothetically-green test run would not have caught the production drop
at group B — the test and the production code shared the identical class of defect, not a
test-catches-production relationship. I did not check every one of the 97 production sites against
every test file individually (that would require the test target to compile, which it does not); the
above is the one file where I read the test module directly and can state this with confidence, not
an exhaustive claim about the other three files.

## Honest gaps

- I did not attempt to fix the 1373-error `#[cfg(test)]` residue (`--all-targets`) — explicitly out
  of scope per rule 25, needs its own dedicated packet as already recorded in `📌️important.md`.
- I did not do a repo-wide (or even crate-wide, outside my 4 files) sweep for the `let _ = <async
  call>;` footgun (invisible to rustc's lint) — only spot-checked the 4 files in my `path_scope` and
  found 3 instances (1 real bug, fixed; 2 test-only no-ops, left as-is and named above).
- I did not verify test coverage for the other 96 sites individually beyond the jobs.rs case above —
  the test target's inability to compile makes that unverifiable right now for any of them.
