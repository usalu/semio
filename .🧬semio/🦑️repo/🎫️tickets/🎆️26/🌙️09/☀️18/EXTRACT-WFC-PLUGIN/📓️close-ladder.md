# 🚪️ Slice L — the close-ladder fault, root cause and fix

Logs: `🗑️generated/close-ladder/`. Every cargo command foreground, `-j 4`, one at a time,
`RUST_MIN_STACK=33554432` for tests, `CARGO_PROFILE_WASM_DEV_DEBUG=false` for the wasm check. No
exit 137 occurred. No git-modifying command was run.

## 0. Result

| lane | before | after |
|---|---|---|
| `cargo test -p semio-s-plugin-wfc --test close_ladder` | 🔴️ **0 of 6**, every editor SIGABRT | 🟢️ **11 of 11** (the 6 G authored + 5 new viewer laws) |
| `wfc_close_cost_is_independent_of_the_retained_session` | never reached its assertions | 🟢️ reaches them: cold 22 / warm 20 / long 31 turns, ceiling 768, dilution **344 %** vs the required 140 % |

## 1. Root cause

**`plugin.internal.prior-outcome` was a MESSENGER, not the fault.** It is what
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:35010` raises once
`RuntimeCloseCleanupJob::step` has already returned `StepOutcome::Fault`; the real detail rides in
that outcome's retained payload, which the runtime only copies into `RuntimeCloseWorkerState::last_fault`
under the FRAMEWORK crate's own `#[cfg(test)]` — i.e. never, when the framework is a dependency. That
is why five slices could read the fault code and learn nothing from it.

I recovered the detail by temporarily printing `fault.detail.single_page()` in
`runtime_close_cleanup_pump_one` (`🔌️plugin/🦀️.rs`, right after `pump.faulted = faulted`), running the
lane, and reverting the edit. `git diff` on that file is empty again
(`🗑️generated/close-ladder/probe-wfc2d-1.log:756`):

```
[DEBUG] close cleanup step fault detail: app owner did not provide the required bounded disposer for document-store
```

That string is `🔌️plugin/🦀️.rs:15404`, inside `drive_artifact_owned_disposer`. `VcsArtifactApp::close_step`
(`🔌️plugin/🦀️.rs:29174-29182`) walks EIGHT owned store lanes, one per `close_owned_stage`:

```
0 document-store   1 config-store   2 draft-store   3 presence-store   4 transient-store
5 window-config    6 window-transient                7 interaction-store
```

Lanes 0–4 and 7 go through `drive_artifact_owned_disposer`, which **hard-faults the whole close** the
moment its `ArtifactDisposal<T>` (= `Option<Box<dyn ArtifactOwnedDisposer<T>>>`) is `None`. Lane 7 is
built by the framework itself (`🔌️plugin/🦀️.rs:22097`); lanes 0–4 are built from the APP
(`🔌️plugin/🦀️.rs:22092-22096` → `A::build_*_store_disposer()`), which `EditorApp<E>` forwards verbatim to
`E::build_*_store_disposer()` (`🔌️plugin/🦀️.rs:32322-32340`), whose `ArtifactEditor` defaults are all
`None` (`🔌️plugin/🦀️.rs:31586-31605`).

**All five wfc editors took those defaults.** Peeling that layer exposed two more, each a distinct
fail-closed gate with its own message, all reached the same way:

| layer | message | what was missing |
|---|---|---|
| 1 | `app owner did not provide the required bounded disposer for document-store` (`🔌️plugin/🦀️.rs:15404`) | `build_{document,config,draft,presence,transient}_store_disposer` |
| 2 | `artifact store has no owner-supplied bounded disposer` (`🏪️store/🦀️.rs:16036`) | `build_{document,config,draft}_store_owners` — the disposer is driven THROUGH the store's own owner catalog, so the two are one declaration in two halves |
| 3 | `presence close requires its installed local-root retirement factory` | `build_presence_local_root_retirement_factory` / `build_presence_peer_retirement_factory` / `build_transient_local_root_retirement_factory` |

This is exactly the "pattern all five copied": the five editors were authored from the assembly
template, which declared its stores and nothing about releasing them. Every other editor in the repo
that has ever been closed declares the full set — `🌀️procedural/…/🌀️generation2d/…/✏️editor/🦀️.rs:1333-1414`
and `🧩️puzzle/…/◻️2d/…/✏️editor/🦀️.rs:4889-4936` are the two precedents I copied the helper choices from.

**It is not a framework bug.** Procedural's identical lane is green through the identical
`close_to_retired` helper and the identical `VcsArtifactApp<EditorApp<…>>` wrapper, and it is green
precisely because its editors make these declarations. The framework's behaviour (fail closed on an
absent disposer) is the intended one: a store released by an unbounded `Drop` is the regression the
whole ladder exists to prevent. The only framework-side shortcoming is diagnostic — the app's own
fault detail is invisible to plugin authors because the capture is `#[cfg(test)]`-gated on the
framework crate. Recorded here, not changed: promoting that capture is a framework-owner decision and
would touch the shared `🔌️plugin/🦀️.rs` for every plugin in the repo.

Two things the fault was blamed on beforehand and that it was NOT: no `InferredField` / inference-job
state is involved (the ladder faults on the FIRST close step, before any job lane), and no refused
dispatch is involved (the editor laws boot, spend four idle turns and close — they dispatch nothing;
the coordinator's `B4` lead describes the same fault code arriving by the LIVE-maintenance route,
which is a different producer of the same code, `🔌️plugin/🦀️.rs:34620`).

## 2. The fix

Per artifact, in the editor's `impl ArtifactEditor` and the viewer's `impl ArtifactViewer`, with one
emoji docstring per group and no comments inside definitions. All values are the framework's own
generic helpers — no artifact grew a hand-written disposer.

| hook | bitmap | grid2d | wfc2d | grid3d | wfc3d |
|---|---|---|---|---|---|
| `build_document_store_owners` | `bounded_document_store_owners::<Snapshot, Mutation>()` | ← | ← | ← | ← |
| `build_config_store_owners` | `bounded_config_store_owners::<Config, ConfigMutation>()` | ← | ← | ← | ← |
| `build_draft_store_owners` | `bounded_document_store_owners::<NoDraft, NoDraftMutation>()` | ← | ← | ← | ← |
| `build_document_store_disposer` | `bounded_document_store_disposer::<Snapshot, Mutation>()` | ← | ← | ← | ← |
| `build_config_store_disposer` | `bounded_config_store_disposer::<Config, ConfigMutation>()` | ← | ← | ← | ← |
| `build_draft_store_disposer` | `no_draft_store_disposer()` | ← | ← | ← | ← |
| `build_presence_store_disposer` | `no_presence_store_disposer()` | ← | ← | ← | ← |
| `build_presence_local_root_retirement_factory` | `no_presence_local_root_retirement_factory()` | ← | ← | ← | ← |
| `build_presence_peer_retirement_factory` | `no_presence_peer_retirement_factory()` | ← | ← | ← | ← |
| `build_transient_store_disposer` | `bounded_transient_store_disposer` (real `BitmapTransient`) | `no_transient_store_disposer()` | `bounded_…` (real `Wfc2dTransient`) | `no_…` | `no_…` |
| `build_transient_local_root_retirement_factory` | `bounded_transient_root_retirement_factory` | `no_transient_local_root_retirement_factory()` | `bounded_…` | `no_…` | `no_…` |

`Presence` is `NoPresence` on all ten surfaces, `Draft` is `NoDraft` on all five editors, and
`ViewerApp<V>` supplies the draft lane itself, so the viewers take nine hooks instead of eleven.
Files (all `🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor,👁️viewer}/🦀️.rs` under
`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/<x>/`): `🖼️bitmap`, `🔲️grid2d`, `◻️2d`, `🧱️grid3d`, `🧊️3d`.

Where a peer slice had already declared one of these hooks (B1 had landed bitmap's
`build_transient_store_disposer` and `build_transient_local_root_retirement_factory`, B3 wfc2d's
transient root factory), I left theirs and added only the missing ones — the patch is idempotent and
I re-ran it after every peer write to the same files.

### 2.1 The viewers had the identical defect, and now have a law
The five viewers close through the same eight-lane ladder and declared the same nothing. Fixing only
the editors would have left five production surfaces that abort the guest on close with no test
looking at them, so `✏️s/🔌️plugins/🀄️wfc/🧪️tests/🚪️close-ladder/🦀️.rs` gained `WFC_VIEWERS`,
`viewer_close_reaches_retired` and five `wfc_viewer_close_ladder_law!` rows — the exact shape G gave
the editor laws, one law per artifact so an abort cannot hide its four siblings. The lane is 11 tests.

### 2.2 Also in grid3d: the three flagged comments inside definitions
- `💡️inferences/🦀️.rs` — both in-body blocks (the `CommitCandidate`-carries-two-payloads note in the
  `Solve` arm and the one-payload-page-per-step note in the encode arm) deleted; their substance is
  now the docstring of `impl InteractiveJob for Grid3dInferenceJob :: fn step`.
- `📸️snapshot/💾️binary/🦀️.rs` — the `ManuallyDrop` warning moved from inside `fn drop` to its docstring.
- crate root `🦀️.rs` — `// ---- Shims: flat access … ----` replaced by a docstring on `pub mod schema`.

The same `⚠️ ManuallyDrop` comment exists verbatim in the other four artifacts' `💾️binary/🦀️.rs`; the
brief named grid3d only, so the siblings are untouched and are listed here instead.

## 3. Commands and results

| command | result |
|---|---|
| `cargo test -p semio-s-plugin-wfc --test close_ladder -j 4 -- --test-threads=1` | 🟢️ `test result: ok. 11 passed; 0 failed; 0 ignored` (`ladder-final.log`) |
| `cargo test -p semio-s-artifact-wfc-bitmap --features component-app-assembly --lib -j 4` | 🟢️ `ok. 185 passed; 0 failed; 1 ignored` |
| `cargo test -p semio-s-artifact-wfc-grid2d --features component-app-assembly --lib -j 4` | 🟢️ `ok. 209 passed; 0 failed; 0 ignored` |
| `cargo test -p semio-s-artifact-wfc-2d --features component-app-assembly --lib -j 4` | 🟢️ `ok. 183 passed; 0 failed; 0 ignored` |
| `cargo test -p semio-s-artifact-wfc-grid3d --features component-app-assembly --lib -j 4` | 🔴️ `FAILED. 204 passed; 2 failed; 2 ignored` — **not mine**, see §4 |
| `cargo test -p semio-s-artifact-wfc-3d --features component-app-assembly --lib -j 4` | 🟢️ `ok. 237 passed; 0 failed; 2 ignored` |
| `cargo test -p semio-s-plugin-wfc --lib -j 4` | 🔴️ `FAILED. 13 passed; 1 failed` — only `descriptor_is_fresh`, **not mine**, see §4 |
| `cargo test -p semio-s-plugin-wfc --test boot_deadline -j 4` | 🟢️ `ok. 1 passed` |
| `cargo test -p semio-s-plugin-wfc --test idle_turns -j 4` | 🟢️ `ok. 2 passed` |
| `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-wfc --target wasm32-wasip2 -j 4` | 🟢️ `Finished` (75 warnings emitted, so expansion really ran) |
| `cargo check -p semio-s-plugin-wfc --lib --tests -j 4` | 🟢️ |

Close costs, final run: bitmap 10 / grid2d 15 / wfc2d 16 / grid3d 16 / wfc3d 18 editor turns;
bitmap-viewer 6 / grid2d-viewer 20 / grid3d-viewer 10 / wfc2d-viewer 13 / wfc3d-viewer 16. The cost
law measures `costs=[("cold", 22, 0), ("warm", 20, 6), ("long", 31, 32)]` against `ceiling=768`,
`dilution-percent=140` → measured **344 %**. Per the fixture's own `lawNote` those inherited procedural
numbers can now be re-measured against wfc's own runs; I did not change the fixture, because the
preview windows do not yet retain real evaluation state and the note asks for that first.

## 4. Two reds that are NOT this slice's, with evidence

1. **`semio-s-artifact-wfc-grid3d --lib`: 2 failures, both slice B4's in-flight `worldSelect` bridge.**
   `every_declared_action_bridges_to_the_command_it_names` ("command_id mismatch for action
   worldSelect: left `worldSelect`, right `pickCell`") and `both_pick_lanes_carry_the_same_cell_key`
   ("left `WorldSelect { cell_id }`, right `PickCell { cell_id }`"). The test file
   (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`, mtime 16:00) still says `worldSelect` is an ALIAS of `pickCell`,
   while the editor it tests (`✏️editor/🦀️.rs`, mtime 16:11) now bridges it to a `WorldSelect` command of
   its own. Nothing in the close path touches action bridging; the other 204 grid3d tests pass. **Owner: B4.**
2. **`semio-s-plugin-wfc --lib`: `descriptor_is_fresh` red.** `🛂️.descriptor.semio` is stale because the
   B slices added window actions, commands and labels after G's last `describe` (15:30). Store owners
   and disposers are runtime-only and appear nowhere in the descriptor, so my change cannot make it
   stale. `describe` takes ~5 min and would go stale again while B* are still editing. **Owner: the
   coordinator / G, after the B wave settles.**

Separately, three cargo runs died on a peer's in-flight
`🧰️framework/…/♾️infinite/🌍️world/🦀️.rs` breakage (`E0433 tool_run`, then twice
`E0502 take_reference_underlay_upload(state, url)`) — the same wall B4 recorded. I polled the file's
mtime until it was quiet for ~2.5 min and the runs went through; nothing in wfc could fix it.

## 5. Files changed

Inside `✏️s/🔌️plugins/🀄️wfc/`:
- `🗿️artifacts/{🖼️bitmap,🔲️grid2d,◻️2d,🧱️grid3d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — the owned-store declarations.
- `🗿️artifacts/{🖼️bitmap,🔲️grid2d,◻️2d,🧱️grid3d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` — the same, nine hooks.
- `🧪️tests/🚪️close-ladder/🦀️.rs` — five viewer laws.
- `🗿️artifacts/🧱️grid3d/{🦀️.rs, …/💡️inferences/🦀️.rs, …/📸️snapshot/💾️binary/🦀️.rs}` — the three comments.

Outside it: **nothing**. The framework instrumentation of §1 was reverted; `git diff` on
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` is empty.
