# 1107 Audit: Native Pane Fixture vs Real Host (lane aud-fixture)

STATUS: READ-ONLY AUDIT, COMPLETE. No repo runs, no cargo, no edits outside this folder. Every claim below is from source reading with file:line; nothing was executed. "Not verified" marks what a run would have to confirm.

Root: `/Users/ueli/Documents/semio/`. Short names used below (all relative to root):

- `FW` = `🧰️framework/`
- `OS` = `FW/🛍️products/💻️os/`
- `PL` = `OS/🔨️modules/🔌️plugin/`
- `LAWS` = `PL/🧪️testing/🧪️artifact-app-laws/`
- `REACT` = `PL/⚛️reactor/`
- `HOSTWAKE` = `PL/🖥️host/🔔️wake/🎟️root/`
- `CORE` = `FW/🔨️modules/` (job, actor, trace)
- `WGPU` = `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/`

## 0. Scope and where the "real host" actually lives

- The `FW/🛍️products/💻️os/🖥️host/🔌️plugin-host/` crates (backbone-document, backbone-ports, space-catalog, host-lifecycle, workflow-store, extension-install, workflow-reconcile) hold storage, backbone and extension install. None of them contains the turn loop, grants, wake or close. The real host path is therefore in `PL/⚛️reactor/` (opening, turn, exchange, cleanup) and `PL/🖥️host/` (wake root, post-turn relay, native frame caller under `WGPU`, browser TS source under `CORE/🎭️actor/🎟️retained-turn/🟦️.ts`).
- Fixture side read: `LAWS` (root `🦀️.rs`, `🎟️fixture-dispatch-steps-and-document-loading`, `🏗️registered-app-construction-and-typed-operation-settlement`, `🪧️pane-smoke-law`), `PL/🧪️testing/🎟️caller/`, `CORE/🧵️job/⏱️context/🦀️.rs` (`renew_turn`), `CORE/⏱️trace/🧮️memory/🦀️.rs` (heap witness).
- Pane smokes checked: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/` `📐️cad/…/🪧️pane-smoke`, `🏭️process/…/🧊️process3d/…/🪧️pane-smoke` (bearbeiten), `🌍️gis/…/🗺️gismap/…/🪧️pane-smoke` (verfolgen), `🪵️sourcing/…/🗂️curation/…/🪧️pane-smoke` (aussuchen), `🏗️fem/…/🧊️3d/…/🪧️pane-smoke` (statik), `🌀️procedural/…/🧊️generation3d/🧪️tests/🪧️pane-smoke` (generator), `🧑‍💻dev/🧩️composition/…/🧊️generation3d/…/🪧️pane-smoke` (composition generation3d). Energy uses `🔋️energy/🗿️artifacts/🔋️model/` mounted tests (panes.sh line 6). No assert_pane_smoke caller for the aggregator (puzzle3d) was found, so its smoke path is unverified.
- Coverage note: `1105b-tools/panes.sh` runs only 5 native smokes (cad, process3d, energy, fem3d, gis). sourcing, generation3d (procedural) and composition generation3d are smoke files that panes.sh does not run. Aggregator is not run by panes.sh at all.

## 1. Summary (what changes a pane result)

1. Turn renewal re-grants lifetime stock every turn (fixture), where the host debits one lifetime budget. The fixture can pass any operation the host would refuse (D1, D2).
2. The fixture's clock is frozen at 0 and its deadline is u64::MAX, so no deadline, lifetime or turn-ceiling is ever enforced natively (D4, D5).
3. Heap-equality assertions in the fixture read a witness that the pane test binaries never install, so they see (0,0). Any non-zero retained result page is a false FAIL. The handover's "energy 48 B retained vs 0 measured" is this symptom (D8).
4. Four of seven smokes load the bundled text constant, bypassing the shell's example command and its LoadDocument effect. Three use the by-example door (D13).
5. Maintenance, publication receipts, birth before a runtime-hosted load, wake/timer delivery, and the whole JS/JSPI/WIT/frame-worker path are not on the fixture path at all (D6, D7, D9, D10, D12, D14, D15).

## 2. Divergences

Legend. "False pass" = fixture passes where the host would refuse or behave differently. "False fail" = fixture fails where the host would pass. Panes: G=generator (procedural), K=koordinator (cad), A=aggregator (puzzle3d, unverified), E=energie (mounted), Au=aussuchen (sourcing), B=bearbeiten (process3d), V=verfolgen (gis), S=statik (fem3d). All pane smokes use the same primitives, so "all" means every smoke that reaches that primitive.

### D1. Turn renewal restores full lifetime stock (fixture) vs one debited lifetime (host)

- Fixture: `CORE/🧵️job/⏱️context/🦀️.rs:441-452`. `renew_turn` sets `cx.retained = current_used + grant` (line 445), resets fuel to `fuel` (line 444), and in the no-return-budget branch adds `refill` to `original_total_fuel` (line 449), so the lifetime total grows every renewal. Called every turn from `LAWS/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs:22, 38, 253, 287`, `LAWS/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs:200`.
- Host: `HOSTWAKE/🦀️.rs` (file `🦀️.rs` in `HOSTWAKE`) `GuestRelayOriginalRoot`: `remaining_work` line 22 and `remaining_grant` line 23 read the debited remainder; `debit_grant` line 25 subtracts every spent receipt from `grant_remaining`; `issued_budget` line 27 refuses a scope whose input differs from the remaining work/grant. Native frame: `WGPU/🎟️native-caller/🦀️.rs:42` (`issue`: fuel and remaining grant from the caller's own remainder), `:51` (`receive_raw`: `self.fuel.saturating_sub(work)`, recipient cumulative, refused if it exceeds immutable stock). Browser: `CORE/🎭️actor/🎟️retained-turn/🟦️.ts:58` (`issue` uses `#remaining` and `#fuel`), `:59` (`receive` debits).
- Effect: false pass for any operation whose total work exceeds the host's lifetime (native initiating event: 67,108,864 fuel, 33,554,432 items, 30 s; `WGPU/🎟️native-caller/🦀️.rs` `initiating_event`; browser `maximumElapsedUs`). The fixture never runs out. Panes: all 7 smokes, through dispatch, settle and load loops.

### D2. Fuel per turn: three values in the fixture, host lanes differ

- Fixture step context: `LAWS/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs:155` (`fixture_retained_turn`, fuel 1,000,000) and `:167` (`StepBudget::new(1_000_000, …)`).
- Fixture renewal: `PL/🧪️testing/🎟️caller/🦀️.rs:27` (`turn_fuel`, 2,000,000 from `PL/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json`, key `turnFuel`). Renewal at dispatch/settle uses the 2,000,000 value (`fresh_fuel`, fixture `:17, :34`, settle `:197`), while the step context starts at 1,000,000. Only the first turn of a context gets 1,000,000.
- Host lanes: Interactive 2,000,000, UserVisible 6,000,000, Background 20,000,000, Maintenance 80,000,000 (`CORE/🧵️job/🕰️clock-identity-and-budget/🦀️.rs:168-180`; `CORE/🎭️actor/📖️vocabulary/🆔️identities-and-budgets/🦀️.rs:353-356`).
- Effect: fixture maintenance runs under Interactive-sized fuel, while the host maintenance lane has 80M. Fuel-heavy steps can fail the fixture where the host passes (false fail, WorkLimit or turn bound), and maintenance throughput in the fixture is not representative (false pass on retirement debt). Panes: all. Not verified: whether any smoke actually hits the 1M-vs-2M gap.

### D3. Grant sizes: pane path matches, caller-backed laws do not

- Pane path: fixture uses `fixture_mounted_policy()` = `PL/🏇️mounted-owner/🦀️.rs:36-40` INTERACTIVE_COMPONENT (32 items, 32 KiB copy, 16 MiB capacity, 1 MiB release, depth 4096). Host per-turn maintenance uses the same `cell.mounted_policy` at `REACT/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs:167-168` (grant read at `:254`). Match for the pane path.
- Divergence: `PL/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json` `grant` = 65,536 items, 128 MiB copy, 256 MiB capacity, 512 MiB release, depth 4,096, and `retirementLoan` = 1 item. That is 2,048 times the per-turn item count of the host. `OriginalFixtureCaller::admit` (`PL/🧪️testing/🎟️caller/🦀️.rs`) uses it. Any law built on the caller rather than the pane step context runs with a grant the host never gives a turn.
- Effect: false pass in caller-backed laws (more items per turn than the host allows). Pane smokes: not affected on this path.

### D4. Clock frozen, deadline unbounded (fixture) vs real lifetimes and lanes (host)

- Fixture: `LAWS/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs:167` builds the context with clock `|| Some(0)` and deadline `u64::MAX`. `:158-166` document this as "unfunded by default". `:267` `plugin_settle_store_constructor` uses real wall time of 30 s. `fixture_pending_effects` (`:171-174`) reuses the same frozen context.
- Host native: `WGPU/🎟️native-caller/🦀️.rs:27` and `:34` (absolute deadline = now + `maximum_elapsed_microseconds`, 30 s for the initiating event, checked against remaining clock). `HOSTWAKE/🦀️.rs:14` (`new`: absolute deadline = now + `maximum_lifetime_microseconds`). Lane wall budgets 4/16/50/200 ms (`CORE/🎭️actor/📖️vocabulary/🆔️identities-and-budgets/🦀️.rs:353-356`).
- Host browser: `CORE/🎭️actor/🎟️retained-turn/🟦️.ts:46` and `:58` use the `clock()` bigint and `deadlineUs`. Boot deadline `WGPU/🎞️frame-worker/🔌️boot-and-plugins/🟦️.ts:157` = `performance.now() + 30000`.
- Effect: false pass for every deadline, lifetime and clock-monotonicity path (no native test can see a timeout). False fail only where a real-time 30 s settle loop (`:267`) is slow on a loaded box.

### D5. Per-turn guest execution ceiling is never measured natively

- Host: `REACT/🔄️turn/⏱️turn-execution-and-poll-entry/🦀️.rs:6-66` accumulates only time spent inside guest `poll` calls (`with_turn_execution`, `poll_kernel_output` at `:87-98`). The file's own comment (`:6-13`) records a browser turn killed at the 5 s ceiling for a 13.9 ms native turn (JSPI suspension, throttled tab). `guest_turn_executing_us` (`:27-34`) returns `None` when the clock is lost.
- Fixture: no call into `with_turn_execution`, `poll_kernel_output` or `guest_turn_executing_us` from `PL/🧪️testing/` (grep empty). Fixture calls `PluginApp` trait methods directly.
- Effect: false pass natively for any turn-length ceiling. Browser-only failure mode (see section 3).

### D6. Cooperative yield and event-loop turns

- Browser: `WGPU/🎞️frame-worker/🎟️original/🟦️.ts:31` (`admit` loop yields with `setTimeout(resolve, 0)` between admissions) and `:51` (`close` loop does the same). Frame ticks are block-time accounted in `WGPU/🎞️frame-worker/⏱️steps-and-reporting/🟦️.ts:52-67`.
- Fixture: synchronous loops with `should_yield` from a frozen clock. `LAWS/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs:270` and `:426` only yield when the caller says so. `🏗️registered…/🦀️.rs:201` returns `WorkLimit` on `cx.should_yield()`. Yielding never happens with `Some(0)` and unbounded fuel.
- Effect: false pass for starvation, ordering between frame ticks, and maintenance interleaving. Browser-only in practice.

### D7. Instance birth and cancellation issuer: partly fixed, still missing on two routes

- Host birth: `REACT/🏃️runtime/📦️instances/🚪️opening/🦀️.rs:60`. `plugin_advance_actor_opening` runs `advance_instance_birth` (bind identity, then `admit_cancellation_issuer`) under the opening's own loans and source retirement (lines 46-75), before publication. The opening's own test (`:143-145`) asserts no caller binds or admits the issuer itself.
- Fixture birth helper: `LAWS/🎟️…/🦀️.rs:6-13` `ensure_fixture_cancellation_issuer` runs the same `advance_instance_birth` loop with a fresh StepContext and no opening loans.
- Called: `dispatch_fixture_action` (`:18`), `load_document` (`:251`), `settle_registered_typed_operation` (`🏗️…/🦀️.rs:197`).
- Not called: `dispatch_fixture_command` (`:33-46`), `dispatch_fixture_response_action` (`:107-112`), media helpers (`:58-104`), `plugin_load_document` (`:279-296`, calls only `plugin_settle_store_constructor` at `:280`).
- Effect: false fail for the runtime-hosted load route and for addressed command or continuation dispatch if an app needs its cancellation lease. No pane smoke calls these routes directly (pane smokes use dispatch_original_fixture_command, which goes through dispatch_fixture_action, and load_document or load_document_text). Remaining divergence: birth runs without the opening's loans (false pass on loan accounting in the opening).

### D8. Heap-equality witness is absent in the pane test binaries

- Fixture assertions: `LAWS/🦀️.rs:26-30` `take_fixture_result_page` (`assert_eq!((heap.requested_bytes, heap.released_bytes), (p.retained_capacity_bytes, p.released_bytes))` at `:28`), `:34-38` `close_fixture_result_page`, `:41-55` `retire_fixture_owned`, `:58-75` `present_fixture_mounted_page`, `:78-90` `retire_fixture_acknowledged_mounted_page`.
- Witness: `CORE/⏱️trace/🧮️memory/🦀️.rs:164` `observe_heap_allocations_on_this_thread` returns the thread-local counter. It is zero unless a `#[global_allocator]` HeapWitness is installed in the binary. The only installed witness in the runtime is `REACT/🏃️runtime/🧰️ops/🦀️.rs:7-8` and it is `#[cfg(test)]`, i.e. only that crate's own tests. grep of the 7 pane smoke directories listed in section 0 finds no `global_allocator`.
- Effect: in the pane binaries every heap observation is (0,0). The assertion holds only when the receipt is zero. Any non-zero retained result page (for example 48 B) is a false FAIL. The handover symptom "energy result page 48 B retained vs 0 measured" (`1105b-HANDOVER.md` update 23:39, `PL/🧵️instance/🔌️plugin-app-impl/🦀️.rs:407`) matches this. Browser: no counting allocator on wasm32; `CORE/⏱️trace/🧮️memory/🦀️.rs:206-209` only reads `memory.size`, so heap receipts are never checked in the browser. Host has no equivalent assertion. Heap accounting is therefore a native-only check and only as good as each binary's allocator. Not verified: which pane binary (if any) links a witness through a dependency.

### D9. Maintenance scheduling differs

- Host: one maintenance step per host tick on the mounted policy, with a pressure burst when the store reports pressure (`REACT/…/🧯️cleanup-pumps/🦀️.rs:164-172`; step at `:168`; `cell.maintenance_pressure` at `:172`).
- Fixture: dispatch loops run maintenance only when the action is still pending (`LAWS/…/🦀️.rs:26`, `:42`, `:52`, `:61`, `:71`). Settle runs maintenance only when `!has_runnable_typed_operations()` (`🏗️…/🦀️.rs:203-209`), so runnable typed work starves maintenance. `drain_maintenance_pressure` (`LAWS/…/🦀️.rs:422-434`) runs only in the typing-run law.
- Effect: false pass when retirement debt builds during runnable typed work (fixture never drains it), and different receipt sequences versus host. Panes: K, A, Au, B (typed operations).

### D10. Publication receipts and refused publications

- Host: `REACT/🔄️turn/⏱️turn-execution-and-poll-entry/🦀️.rs:110-111` (`plugin_exchange_turn` calls `collect_original_publication_receipts` and `retain_refused_publication`).
- Fixture: `🏗️…/🦀️.rs:212` (`advance_typed_operation_publication`) and `:263` (`retain_registered_ownership_receipts`). No call to the host's collect or retain-refused path (grep of `PL/🧪️testing/` for both names is empty).
- Effect: the refused-publication retention path never runs natively in pane laws (false pass). Browser exercises it.

### D11. Turn bounds the host does not have

- Fixture: `OriginalFixtureCaller::maximum_turns()` = 32,768 and `maximum_idle_turns()` = 4,096 (`PL/🧪️testing/🎟️caller/🦀️.rs:31`, caller.json `maximumTurns`, `maximumIdleTurns`). Settle enforces both (`🏗️…/🦀️.rs:200-201`). Document load: `LAW_DOCUMENT_LOAD_MAXIMUM_TURNS` = 1,000,000 (`LAWS/…/🦀️.rs:326`). Host bounds by fuel and deadline only.
- Effect: false fail if a legitimate settle needs more than 32,768 turns. Not observed; theoretical with the fuel gap in D2.

### D12. Document load door bypasses the runtime exchange

- Fixture: `LAWS/…/🦀️.rs:241-262` `load_document` calls `PluginApp::begin/poll/acknowledge_document_archive_load` directly, with `renew_turn` and maintenance each poll (`:253-255`).
- Host: archive commands go through `REACT/🏃️runtime/🔄️exchange/🔀️plugin-exchange/📦️document-archive-commands/` inside `plugin_exchange` (`REACT/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs:113`), with runtime ops (`REACT/🏃️runtime/🧰️ops/📄️document/`).
- Fixture runtime variant `plugin_load_document` (`LAWS/…/🦀️.rs:279-296`) also bypasses `plugin_exchange` for the poll (`:288` calls it, but without receipts; `:289` polls through `ops_document`).
- Effect: archive-door faults and runtime receipt mismatches are invisible natively. Panes: all that load documents (all 7).

### D13. Example door vs bundled text constant

- Text constant (`assert_pane_smoke`, never runs the example command or LoadDocument effect): cad K (`/…/📐️cad/…/🧪️tests/🔬️unit/🪧️pane-smoke/🦀️.rs:10`, `PRIMARY_TEXT` at line 10), process3d B, gis V, sourcing Au.
- Example door (`assert_pane_smoke_by_example`, `LAWS/🪧️pane-smoke-law/🦀️.rs:32-57`, dispatches the example command and crosses LoadDocument): procedural G, fem3d S, composition generation3d (1 each).
- Effect: four smokes never execute the example command or the published LoadDocument effect. A pane whose example catalogue is broken still passes (false pass). Energy and aggregator: not checked.

### D14. Wake and timer delivery never reaches the fixture

- Host: `REACT/🔄️turn/📨️event-dispatch/🦀️.rs:128-132` (`Event::Timer` removes the armed timer and calls `TASK_EXECUTOR.wake`). Executor wake contract at `REACT/🧵️executor/🦀️.rs:454`. Post-turn relay: `PL/🖥️host/🔀️post-turn-relay/`.
- Fixture: grep for `wake`, `Timer`, `TASK_EXECUTOR` in `LAWS/` returns nothing. Fixture steps call trait methods and never deliver events.
- Effect: any progress that depends on a timer or relay wake (async decode, debounced input) is never exercised natively, so a pane can pass while its wake path is broken (false pass).

### D15. WIT, JSPI and frame-worker path skipped natively

- Host browser: component-model wrapper and JSPI suspension (see the comment block `REACT/…/⏱️turn-execution-and-poll-entry/🦀️.rs:6-13`). WIT lowering of an original turn: `CORE/🎭️actor/🎟️retained-turn/🟦️.ts:84` (`originalTurnWit`). Frame worker: `WGPU/🎞️frame-worker/🎟️original/🟦️.ts:10-51`, with receipt decoding at `:26`.
- Fixture: calls kernel `PluginApp` methods directly. No WIT encode or decode, no frame worker.
- Effect: WIT lowering and receipt-decoding mismatches are invisible natively (false pass), browser-only.

### D16. Linear memory ceiling

- Browser: `CORE/⏱️trace/🧮️memory/🦀️.rs:206-209` (`guest_linear_memory_bytes` reads `memory.size` on wasm32). Native: `:212-214` returns `None`. Native fixture never checks a linear-memory ceiling.
- Effect: a pane can exceed the guest's linear memory only in the browser (browser-only).

### D17. UI patch page budget

- Host: `REACT/🔄️turn/⏱️turn-execution-and-poll-entry/🦀️.rs:121` (`turn_patch_budget_bytes`) and `:142` (`take_turn_patch_page`) per turn.
- Fixture: pane law checks state through `probe` closures (`LAWS/🪧️pane-smoke-law/🦀️.rs:6-29`), never a UI patch page. Rendering helpers exist in `LAWS/🔎️declared-verb-probing/` but the pane law does not call them.
- Effect: patch-page byte budget and reconcile ordering are browser-only (false pass natively).

### D18. Identity authority budget: equal for pane path

- Fixture: `LAWS/…/🎟️…/🦀️.rs:230` `EntityIdentityAuthority::new(1usize << 24, …)`. Host bootstrap: `🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs:231` uses the same 1<<24. Host checkpoint restore uses `semio_plugin_reactor_surfaces::checkpoint::RESTORE_AUTHORITY_BYTES` (`PL/⚛️reactor/🦀️.rs:309`), which no pane smoke reaches. No divergence on the pane path.

### D19. Cancellation never fires in pane laws

- Fixture: `root_cancel_token()` (`LAWS/…/🦀️.rs:167`) is never cancelled. The pane law has no cancel step.
- Host: `REACT/🏃️runtime/📦️instances/🚪️opening/🦀️.rs` cancels an opening on `cx.is_cancelled()` (around `:47-48`, `opening.closing`).
- Effect: cancel paths are untested by the pane smokes (false pass for cancel-on-close and cancel-during-open).

### D20. Production mounted policy not confirmed

- The fixture hard-codes `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` (`LAWS/…/🦀️.rs:149-151` and cad smoke line 10). The host reads `cell.mounted_policy` / `runtime.mounted_policy` (`cleanup-pumps:254`, `:525`). The production value was not traced in this audit. Not verified.

## 3. Real-host behaviour no native pane test exercises (browser-only)

- JSPI suspension, WIT lowering and the component wrapper: D5, D15.
- Guest turn executing-time ceiling (`GUEST_LIFECYCLE_TURN_CEILING_US`, 5 s incident): D5.
- Timer and executor wakes, post-turn relay: D14.
- Frame-worker admit/close yields, block-time ledger, boot 30 s deadline: D6, D4.
- Browser-clock lifetime debit (`OriginalOperationSource`, `TS :46-59`): D1, D4.
- Wasm linear-memory ceiling: D16.
- UI patch page budget and reconcile: D17.
- Refused-publication retention on the live turn: D10.
- Native-only heap receipts are not checked in the browser at all: D8.

Native tests do exist for some of these in other crates (for example the `#[cfg(test)]` witness in `REACT/🏃️runtime/🧰️ops/🦀️.rs`), but none is on the path of the eight pane smokes. This audit did not enumerate the whole workspace test suite.

## 4. Fix direction (not applied, read-only)

- Fixture renewal: make `renew_turn` restore only per-turn fuel and per-turn grant and debit a lifetime budget (match `GuestRelayOriginalRoot.debit_grant`), or route pane laws through the host root instead of renewal (D1, D2, D3).
- Clock and deadline: give the fixture the real clock and the lane wall budgets (D4).
- Witness: install the HeapWitness global allocator (non-test) in each pane test binary, or make the fixture refuse to assert heap equality when no witness is installed (D8).
- Route: switch the four text-constant smokes to `assert_pane_smoke_by_example`, and add a LoadDocument-effect check (D13).
- Birth: call `ensure_fixture_cancellation_issuer` from `dispatch_fixture_command`, `dispatch_fixture_response_action` and `plugin_load_document` (D7).
- Maintenance: drain maintenance on the same cadence as the host, including while typed work is runnable (D9).
- Add one native test that runs a turn through `with_turn_execution` and one through a timer wake (D5, D14).

## 5. Not verified (needs a run, which this audit did not do)

- Whether any smoke reaches the 1M/2M fuel gap (D2) or the caller-backed grant (D3).
- Which pane binaries, if any, link a HeapWitness transitively (D8).
- The production `mounted_policy` value (D20).
- Aggregator (puzzle3d) smoke location and its primitives.
- Whether `assert_pane_smoke` paths hit `should_yield` at all under the frozen clock (D6).
