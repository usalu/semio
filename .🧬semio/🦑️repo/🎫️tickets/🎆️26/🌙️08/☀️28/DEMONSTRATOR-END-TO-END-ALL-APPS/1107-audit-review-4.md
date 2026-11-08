# 1107 Audit Review 4 (aud-review-4)

Read-only review of `git -c core.fsmonitor=false diff HEAD` for the newest ports listed in the lane brief: neural engine, geometry brep and mesh, fem sparse helpers, os-inference, infinite board, tool-typing-edit, actor and mounted owner, ui-contract-values and ui-contract, ui-draw-contract cold paths, `OriginalEffectRun` and `with_effect`, `RetainedOwnerGate`, store command, history record decode, and the uses of `release_empty_page_leased` and `reserve_one_cold`.

No repo runs, no sub-agents, no edits outside C, no miri. Owners come from `C/1107-lanes.md`. Rules applied: `C/1107-brief.md` addenda W1 (wallets only at entry points with no turn), W2 (no per-call constant standing in for a turn), W4 (close that cannot progress must refuse or block, never Pending forever), and the runtime2 drivers table.

Scratch used: `C/🗑️generated/1107-audit-review-4/` (`rust.diff` is the Rust-only diff, 60k lines; `hits.txt` is an indexed list of added lines matching wallet, `with_effect`, `unwrap`, `expect`, `panic`, `work: 1|0` and `Progress(default)`). Kept, not swept.

Coverage limits are stated per item. Items marked "not read" were only grep-level.

## Findings, ranked by runtime impact

### 1. FEM 2D mounted job: uncharged wallets inside a job turn (missing charge, W1/W2). High, confidence high on the code.

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🧷️job-state-and-construction/🦀️.rs`
  - L409–423 `step_one_funded(grant)`: `reserve` wraps `owner.reserve_capacity_one_funded` in `AuthoringWallet::new(grant, 64).run(grant, 64, ..)`. It returns only `RetainedCloneProgress`. The wallet's `performed` is dropped, so the turn never sees that work.
  - L460 and L463: `step_one_funded` falls through to `self.step_one()` for every non-reserve stage, so the grant-funded call also runs `step_one()`.
  - `step_one()` (L466 onward) calls `semio_s_fem_sparse::reserve_cold_page` at L489, 496, 541, 553, 567, 583, 611, 617, 767, 773. Each call opens its own fresh `AuthoringWallet::new(grant, 64)` in `🔢️sparse/🤝️numerical-protocol/🦀️.rs` (`reserve_cold_page`, about L300–315).
- Callers that hold a turn: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🔁️mounted-state-step/🦀️.rs` L343 calls `csr_build.step_one_funded(cx.retained_grant())`. L64 calls `assembly_build.step_one()`. Both have `cx` in scope.
- Effect: the reservations run outside the turn's original effect, so they are not charged against the turn's fuel (W1). The fix is the pattern the same lane already uses in `reserve_funded_page`: `cx.with_original_effect(grant, |_, effect| ..)` followed by `receive_original_retained_work`. `reserve_cold_page` must take an effect or be deleted from turn paths. `step_one()` and `step_one_funded` must thread the effect down.
- Owner: statik (`ae8bea3f04b810457`) for the pane. The shared helper `reserve_cold_page` sits in the fem sparse crate; value (`a1bf0499e1a315f51`) has the wallet primitive.
- Related, medium: `reserve_funded_page` and `reserve_cold_page` return `Ok(default)` when the quote is not admitted. Callers in the loops (`cold_paged_owner`, `admit_one`, `assembly_build_partitions`) then see "no progress" and can loop or error with a misleading kind. Not traced to a spin.

### 2. UI draw retained retirement discards release results and reports zero progress (W4 spin and error swallowing). Medium-high, confidence high on the code, medium on the spin.

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🗑️retained-retirement/🦀️.rs` L186–200 and L262–285 (shared borders, clip scissors, scene passes, layers, glass regions). Each branch is:
  `let bytes = X.next_release_allocation_bytes().unwrap_or(0); let _ = X.release_empty_page_leased(bytes); return false;`
- `let _ =` drops the `Result`. `unwrap_or(0)` turns a failed demand into a zero-byte release. `return false` reports "not terminal" without any progress, so a retirement that cannot release just keeps being called.
- `release_empty_page_leased` itself does no work accounting: its `pay` closure always returns `Ok(())` (see item 9). The retirement therefore releases memory with no charge and no reported progress.
- Fix: propagate the error, return a typed refusal or `Blocked`, and report the released bytes through the retirement's `RetainedCloneStep`.
- Owner: reacthost (`a4d1224fa2b0af2e6`) per the table, since the wgpu target sits in the React host and UI lane. The draw retained-append file (`📥️retained-append/🦀️.rs` L42) and shared-border-finalize (`📏️shared-border-finalize/🦀️.rs` L32, L41, L229) use `reserve_one_cold` and `release_empty_page_leased` in the same cold style, not read in full.

### 3. Brep and mesh: admit-all authoring wallets on public synchronous entry points, reachable from turn-bearing kernel calls. High impact if reachable, confidence high on the stock, medium on reachability.

Stocks and budgets are the admit-all constants the brief forbids:
- `HalfedgeMesh::tessellate(&self)` in `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/⏱️tessellation-job-and-obj-export/🦀️.rs` ~L229–236. Stock `1<<30` for items, copy, capacity and release, depth `1<<10`, budget `1e12`. Per step `wallet.run(grant, 1<<20, ..)`. No cancel and no progress. Loops until done.
- `MeshTessellationJob::into_mesh` (same file, around L70–85 for the retirement helpers) and brep `TessellationJob::into_mesh` in `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/⏱️tessellation-job/🦀️.rs` L279–287. Stock `1<<30`, budget `1e12`, per step `run_settled(grant, 1<<20, ..)`, with `.expect(..)` on each result.
- `Body::compact(&mut self, keep)` in `🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🕸️topology/🔗️merge-and-remap/🦀️.rs` L88–95. Constants `COLD_COMPACT_STOCK` (`1<<30`) and `COLD_COMPACT_BUDGET` (`1e12`) at L6–8. Panics on refusal via `unwrap_or_else(panic)` and `.expect(..)`.
- Reachability, traced:
  - `Body::compact` is called from `ShapeValue::compact_unreachable` (`⚙️engine/🧾️conversions-and-registry/🦀️.rs` L162). That is reached from `🏷️validation-gate-and-labels/🦀️.rs` L44 (`dispose_sync`) and from `🔌️brep-kernel-implementation/🦀️.rs` L289 (`retain`, a trait method). Whether a plugin turn reaches `dispose` or `retain` is not traced.
  - `ShapeValue::tessellate` (`⚙️engine/🪅️shape-value/🦀️.rs` L180–190) runs `step(usize::MAX)` with no cancel point, then `into_mesh`.
  - `HalfedgeMesh::tessellate` is called from the lowpoly plugin's `tessellate_transfer_json` (`✏️s/🔌️plugins/💠️lowpoly/.../✏️editor/⚙️engine/🦀️.rs` L368). That helper feeds the UV window and the model window's tessellation export. Whether these run inside a turn is not traced.
- Breaks W1 (uncharged wallet if reachable from a turn), W2 (admit-all constant), and the "explicit finite stock" law. The `expect` and `panic` are production paths.
- The brep kernel also runs `boolean_sync` through `job.job.run_to_completion` with `usize::MAX` budget (`🔀️boolean-transform-and-copy/🦀️.rs` L27). That is an unbounded synchronous loop, with no progress and no cancel. It is the same W2 shape.
- Owner: port-e (`ac2ac4bb1b043a97d`, geometry) for brep and mesh. Lowpoly plugin: unassigned in the table.
- Fix: thread the caller's effect or cx down; for the one-shot façades, return a typed refusal when a turn is needed; size stocks from the document or the grant. Do not add budgets.

### 4. Neural engine: a native turn falls back to an uncharged cold wallet and swallows every error. Medium (dead today, a trap for the next caller), confidence high on the code.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🌱️cache/👁️read.rs`:
  - `cold_effect` (AuthoringWallet, `COLD_STEP_WORK = 1<<10`, one fresh wallet per call).
  - `controlled_retire_step(owner, control)` uses `control.with_effect` (correct).
  - `retire_value_controlled(owner, control)` loops `if controlled_retire_step(..).is_err() { retire_value_cold(owner); return; }`. It is documented as "cleanup that a refused turn cannot fund completes at the cold boundary". That is a W1 violation by construction: it spends an uncharged stock inside the turn's own receiving path, and it converts every error (including invariant faults) into a silent fallback.
  - No caller of `retire_value_controlled` exists in the tree (checked with `git grep`). It is dead today.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🌱️cache/🦀️.rs`: `impl Drop for NeuralCache` calls `retire_cold`, which runs `cold_cache_step` and `panic!`s on refusal (`COLD_CACHE_STEP_WORK = 1<<10`). A panic inside `Drop` aborts when it runs during unwinding. A drop inside a turn is also uncharged.
- Owner: port-j (`adca15350cb0b158c`, kernel-neural-engine-model).
- Fix: delete the fallback; return the typed refusal; make `Drop` non-panicking (log `[DEBUG]` only during development, otherwise drop the error explicitly with a reason).

### 5. SPR history fold: Pending ignored in a loop, and a physically unbounded stock. Medium, confidence high on the code.

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🧬️schema/🦀️.rs` L220–245 (`HistoryLog::fold`).
  - `let stock = RetainedCloneGrant { .., maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, .. }` and `AuthoringWallet::new(stock, 1 << 24)`.
  - `match done.output { RetainedWorkPoll::Pending { .. } => {} ... }`: a pending poll is silently accepted and the loop repeats. W4: a pending that never resolves spins forever. The cold path makes this synchronous, so it should be a typed refusal.
  - The wallet is never posted (`post`), so the physical stock is never debited; the `usize::MAX` stock makes the check meaningless anyway.
- Owner: store (`a9d14ef1566792fdf`).
- Fix: `Pending` becomes `Err(cold(..))`. Replace `usize::MAX` with the document's size budget, and `post(step.progress())`.

### 6. Drawing kernel boolean retirement: one wallet reused across steps without `post`. Medium-low, confidence high on the code.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🧱️kernel-impl/🦀️.rs` L186 (`AuthoringWallet::new(STOCK, 1 << 48)`) and the loop around it (`wallet.run_settled(grant, 1 << 24, ..)`). `STOCK` is a constant of `1<<22` items and `1<<28` bytes.
- `run`/`run_settled` debit only the computational work (performed receipt). The physical stock is debited only by `post`. This loop never calls `post`, so the physical stock is never checked across steps: a retirement could exceed `STOCK` in total without refusal.
- Owner: flow (`a657639eb968ca6ac`).
- Fix: `wallet.post(step.progress())` after each step, as `into_mesh` and `compact` already do.

### 7. `unsafe impl Send for NativeReceiveSlot<I, T>` without `T: Send`. High soundness concern, runtime impact depends on the receiver types. Confidence high on the impl.

- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs` (about L20194 of `rust.diff`): `unsafe impl<I:?Sized+'static,T:NativeReceive<I>> Send for NativeReceiveSlot<I,T>{}`. The struct holds `ManuallyDrop<Option<T>>`, `NativeOwnedStorage<T::Receiver>`, `Box<T::Receiver>`, and an `OriginalEffectAuthority` over `NativeSlotProducer<I,T,T::Receiver>`. None of these is bounded by `Send`, so a `T` containing `Rc` or raw pointers can cross threads.
- Fix: bound the impl on `T: Send, T::Receiver: Send, I: Sync` as needed, or remove it. Confirm each receiver type actually is Send.
- Owner: receivers (`a10faa9b6579c9a02`) or value (`a1bf0499e1a315f51`).

### 8. Paged list serde decode reserves without any effect. Medium, confidence high on the code.

- `🧰️framework/🔨️modules/🌱️value/📦️paged/🔣️serde/🦀️.rs` L50: `output.entries.reserve_one_cold(bytes)` inside `Deserialize` for a paged object. The allocation is not admitted by any grant, effect, or fuel. Any serde decode of such a type bypasses the turn's memory admission.
- Reachability inside a turn is not traced. Under W1, a serde path is not allowed to allocate outside a turn if it can be reached from one.
- Owner: value (`a1bf0499e1a315f51`).

### 9. `release_empty_page_leased`: release work is uncharged by design, and callers rely on it. Medium, confidence high on the code.

- `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs` (about L13800): `release_empty_page_leased(maximum_bytes)` calls `release_page(.., &mut || Ok(()))`. The `pay` closure never charges. The doc says "the cursor owner pays one unit per progressed step", but no code in the function enforces that. Callers: fem `close_paged_owner_step` (`🔢️sparse/🤝️numerical-protocol/🦀️.rs` L329, `🧮️analyses/🗂️model/🦀️.rs` L61), which returns `(0, released_bytes)` with no work unit; ui draw (item 2); `rows.release_empty_page_leased` in `🖱️ui/🧠️runtime/🎭️present/📦️resources/🤝️loans/🦀️.rs` L173.
- Also: `self.allocated -= step.released_allocation_bytes` can underflow if a caller passes a stale marker. Debug panic, release wrap. Not traced.
- Owner: value (`a1bf0499e1a315f51`). Fix: take the effect as `release_empty_page(effect)` and let the caller's effect pay, or pay one unit in `release_empty_page_leased` against an explicit work authority.

### 10. `OriginalEffectRun` leaks a live action on early return and panic. Medium-low, confidence high on the code.

- `🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🎟️authoring/🦀️.rs` L21–45.
  - `begin` calls `work.prepare_for_action(..)?` then `issue_performed_for_action(..)?`. If the second fails, the action stays open because the `?` returns before any cancel.
  - `OriginalEffectRun` has no `Drop` impl. A caller that returns early or unwinds between `begin` and `finish` leaves the action live. `AuthoringWallet::run` catches that only through `has_live_actions()` after the fact; the job and `with_effect` callers do not.
- `NativeEncodeControl::with_effect` (`🌱️value/🛫️encode/🎟️effect/🦀️.rs` about L17886) uses `open.effect()?` then `run(self, &mut effect)` then `open.finish(&mut self.action, &mut 0)?`. An early `?` from `effect()` or a panic in `run` leaves the action live.
- Fix: `begin` cancels on its own failure; `Drop` on `OriginalEffectRun` cancels when not finished, or the closure form becomes the only public API.
- Owner: runtime (`a8f570504dc3f4b48`) for the primitive; port-e (`ac2ac4bb1b043a97d`) for `with_effect`.

### 11. Mounted analysis page reservation checks its byte grant after the allocation. Medium-low, confidence high on the code.

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/🦀️.rs` around L175–178 (`MountedModelItems::admit_one`): `reserve_cold_page(&mut self.values, target)` is called, and only afterwards is `progress.allocated_bytes > maximum_bytes.min(MOUNTED_OWNER_PAGE_BYTES)` tested. The wallet grant inside `reserve_cold_page` does not carry `maximum_bytes`, so the page can exceed the caller's byte grant before the refusal.
- Owner: statik (`ae8bea3f04b810457`). Fix: pass `maximum_bytes` into `reserve_cold_page` as the capacity bound.

### 12. Panics and phase-machine `unwrap`s on production paths. Low-medium, confidence high on the grep, low on reachability.

- `paged_from_values` (`🔢️sparse/🤝️numerical-protocol/🦀️.rs` about L346): `assert!(reserve_cold_page(..).expect(..).progressed, ..)`. Panics on refusal inside a production helper.
- `Body::compact` (item 3) and the neural cold `panic!`s (item 4), and `into_mesh` `.expect(..)` (item 3).
- Actor wire turn: `🎭️actor/🖼️wire-turn/📬️typed-operation/📥️receiving/🦀️.rs` L31082 `self.source.as_mut().unwrap()` in the `SourceClose` stage. Owner port-g (`a4029c0add7a8f008`).
- Infinite board: `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧭️host/🪟️wasm-session-and-input-application/🦀️.rs` (around L46712–46723) `self.id_insert.as_mut().unwrap()`, `self.status_insert.as_mut().unwrap()`, `self.port_insert.as_mut().unwrap()`, each passed to `insert_step(.., 1, control, effect)`. The constant `1` is the work for one action; it is honest only if each `insert_step` records exactly one unit (not checked). Owner: unassigned in the table.

### 13. Quote versus recorded work, constant `work:1`. Low, confidence medium.

- Neural receive phase 0 (`🧠️neural/⚙️engine/📥️receive/🦀️.rs`): `NativeAdvanceSelection::Local{work:1}` for `phase==0`. Honest only if that phase records exactly one unit. Not verified.
- Neural evaluation (`⏱️evaluation/🦀️.rs`): many phases use `.unwrap()` on `body.topology`, `body.merge`, `body.entry`, `body.wave`. Reachable only through phase bugs. Owner port-j.

## Verified without a finding

- `RetainedOwnerGate` (`🌱️value/🎟️control/🧬️contract/🎟️gate/🦀️.rs`): the guard carries `PhantomData<&'a mut T>`, so `Guard<T>` is `Sync` only if `T: Sync`. The `Sync` impl is `T: Send` on the gate. The `Drop` releases the flag on unwind. No leak on close. No finding.
- `AuthoringWallet::run` (`🎟️authoring/🦀️.rs` L67–80): one debit of work from the performed receipt, `spent` set before any refusal, `has_live_actions` checked after. The work debit is single. Physical stock is not debited by `run` (see item 6).
- `NativeEncodeControl::with_effect`: one commit, the receipt's performed work. Callers report work = performed. No second charge found.
- `release_empty_page_leased` and `reserve_one_cold` in the persistent list: both are cold or leased by name, and the uses are listed in items 2, 8, 9.
- `reserve_funded_page` (`🔢️sparse/🤝️numerical-protocol/🦀️.rs` about L317–330) and the one-page funded path in `🎵️subspace/🌀️job-core`: they go through `context.with_original_effect` and `receive_original_retained_work`. Single charge, no wallet. Correct pattern.
- Tessellation `into_mesh` and `compact` both `post` progress after each step. Only the stock constant (item 3) is wrong.

## Not reached in this pass

- os-inference (`💡️inference/`): not read.
- ui-contract and ui-contract-values, including the one-page arena and ascending map keys: not read. `UiValueNativeReceiver` appears in the diff as a rewrite to `NativeReceiveSlot` (see item 7).
- tool-typing-edit (`🛠️tool-machine/⌨️typing/✍️edit/`): three hits, `advance_retained_read`, `retain_original_read`, `return_retained_read`, not read.
- actor and mounted owner: 12 hits; only the actor wire turn and the `poll_step` helpers read.
- store command: not read.
- job-native-encoding `catch_unwind`: the sites are in `🌱️value/🛫️encode/🦀️.rs` around L18594 and L18607, and in `effect_allocated_with_work` (`🎟️effect`). Read only as far as the pattern `catch_unwind(AssertUnwindSafe(..))` with state re-recorded after a panic. No UB found from `catch_unwind` alone, since no `unsafe` is inside the closure. The logic risk (a panicked closure recorded as performed) is unverified. The raw-pointer witness in job encoding is item 1 of review-3 and was not re-checked.
- Unsafe in tool-run `🎞️tick/🛬️receive/🏗️native` and `📊️progress/🦀️.rs` (raw `addr_of_mut!` drops with a `headers` phase counter): not read in full, owner port-d (`ac184d5a0692b2b49`).
- Array rollback `unsafe{ rows.as_ptr().cast::<T>().add(initialized).read() }` in `📥️default/🔢️array`: depends on `rows` being `MaybeUninit`; not checked.

## Owners (from `C/1107-lanes.md`)

| Item | Owner |
| --- | --- |
| 1 fem mounted step, wallets in turn | statik `ae8bea3f04b810457`; value `a1bf0499e1a315f51` |
| 2 ui draw retained retirement | reacthost `a4d1224fa2b0af2e6` |
| 3 brep and mesh admit-all wallets | port-e `ac2ac4bb1b043a97d`; lowpoly unassigned |
| 4 neural fallback and Drop panic | port-j `adca15350cb0b158c` |
| 5 spr history fold | store `a9d14ef1566792fdf` |
| 6 drawing boolean retirement | flow `a657639eb968ca6ac` |
| 7 NativeReceiveSlot Send | receivers `a10faa9b6579c9a02`, value `a1bf0499e1a315f51` |
| 8 paged serde reserve | value `a1bf0499e1a315f51` |
| 9 release leased uncharged | value `a1bf0499e1a315f51` |
| 10 OriginalEffectRun leak | runtime `a8f570504dc3f4b48`, port-e `ac2ac4bb1b043a97d` |
| 11 mounted byte grant | statik `ae8bea3f04b810457` |
| 12 panics, actor unwrap | port-g `a4029c0add7a8f008`; infinite unassigned |
| tool-typing | port-h `a1a9781a39ebfbe57` |
| ui-contract-values | uivalues `ac7cbe178126819e6` |

## Confidence and limits

- Items 1, 3, 4, 5, 6, 7, 8, 9 and 11: high on the code (read from the diff and the working tree). Reachability into a turn is traced for item 1 only.
- Item 2: high on the discarded `Result`, medium on the spin.
- Item 3: high on the constants and the `expect`s. Reachability from a turn is not traced for `compact`, `tessellate` and `into_mesh`.
- Item 10: high on the code, low on whether a caller ever unwinds mid-run.
- Nothing was run. No test reproduces any item.
