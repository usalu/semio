# Live cleanup zero-progress fault during browser archive load: analysis

Read-only analysis. Paths are relative to `C:\semio\🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\` (abbreviated `P/`). Nothing was run; every claim below is from reading code, and the one runtime fact that would confirm it is named at the end.

Abbreviations:
- `CT` = `P/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs`
- `PUMP` = `P/⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs`
- `LAD` = `P/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs` (unit step)
- `LAD2` = `P/🪜️maintenance-ladder/🦀️.rs` (stage table, demands, archive/replacement drivers)
- `DOCLAD` = `P/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs` (archive load state machine)
- `TURN` = `P/⚛️reactor/🔄️turn/🪜️entry-lifecycle-and-retirement/🦀️.rs`

## 1. The verdict and its arithmetic

- `runtime_live_cleanup_nonterminal_status` (CT:24-57): a reading counts as zero progress if it is `Progress(empty)` or any `Blocked` (CT:40). Only `AwaitingInput` clears (CT:36). Fault needs `stalled >= 256` (`RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT`, cleanup-vocabulary:200) and `credit_us >= RUNTIME_LIVE_STALL_CREDIT_US` (CT:19, quoted as 8000us).
- The count is per pump, not per turn. One reactor turn runs up to `LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN = 32` pumps (CT:456, TURN:123-134). 256 readings are therefore 8 turns, and 8000us is roughly one browser frame. The observed `41000us` is 8 or more turns of the archive making no externally visible progress. In wasm the wall clock includes every host/IO gap between turns.
- Only `contended` readings are ignored (CT:33). `contended` is set by `runtime_cleanup_reading_is_starved(progress, runtime_live_pass_budget_spent(cx))` (PUMP:143, PUMP:276-286), which requires `should_yield || items==0 || depth==0`, evaluated AFTER the step.
- The close-side verdict (`runtime_close_stall_verdict`, CT:170-195) treats `Blocked` and `AwaitingInput` as `ExternalWait` and clears. The live-side verdict treats `Blocked` as a stall. The two are inconsistent.

## 2. The grant the live cleanup gets

- Per mounted turn: `MountedOwnerTurnV1::admit` (mounted-owner:96-107) gives `min(policy.maintenance, cx.retained_grant())` on every axis. Browser hubs use `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` (mounted-owner:42-45): items 64, copy 1 MiB (covering 688,784 B), capacity 16 MiB, release 1 MiB, depth 4096.
- That 64-item loan is capped by the REACTOR TURN's remaining stock, which is shared by all 32 pumps of the turn. PUMP:282-286 documents it: "the repeated live pump of a quiet turn spends the item stock on its first pass and reads empty ladders for the rest".
- Inside the ladder, `available_items = min(loan.items, cx.retained_grant().items)` (LAD:50). While the archive (Stage 24) is demanded (`archive_pending`, LAD:44) the rules are:
  - archive rung skipped if `available_items < 16` (LAD:53)
  - every non-archive, non-dependency rung skipped if `available_items <= 16` (LAD:54)
  - dependencies are Stage 8, 14, 15 only (LAD:52)
- `DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS = 16` (LAD2:686).

## 3. Which reading is most plausible

(a) `should_yield` before any unit (LAD:34, LAD:46). Already classified as scheduling latency by `runtime_cleanup_reading_is_starved` -> `contended`. Not the cause on its own.

(b) grant items 0 (LAD:34, LAD:47). Same: covered by `stock.maximum_items == 0` (PUMP:284). Not the cause.

(c) every unit skipped as unfundable (LAD:53/54), stock in 1..=16. THIS IS A REAL GAP in the starvation classification: `runtime_live_pass_budget_spent` only fires at items==0, not at 1..=16, so these readings are counted as stalls. It can only fire on pumps 1..31 of a turn (pump 0 starts with the full 64 items, since `INTERACTIVE_COMPONENT` is 64 >= 16). Pumps beyond 0 reach this state when `live_cleanup_pressure` stays true (TURN:125 bypasses the stock check when under pressure). Because the next turn's pump 0 is funded, (c) alone cannot produce 8 consecutive turns of zero progress. It is an amplifier that inflates the count 31:1 relative to real waits, not the root cause.

(d)/Blocked "nothing advanced while not terminal" (LAD:79 returns `Progress(empty)` when `maintenance_terminal_is_empty()` is false and no unit moved). This is the most plausible root cause in the browser. The archive state machine returns `Progress(empty)` for every wait on another owner or on the host:
- `HydrateParent`: paged parent receiver not ready -> `parent.advance(cx)` (empty while pages are still arriving), DOCLAD:305-309; `close_receiver` pending, DOCLAD:329/333.
- `DecodeParent`: `RetainedHistorySourceStep::Progress|Blocked` with a zero receipt, DOCLAD:~292-297.
- `AwaitingMembers`: replacement `Initializing` (DOCLAD:442) and `try_begin_owned_document_members` false (DOCLAD:438).
- `AwaitReplacement`: `poll_artifact_store_replacement` Pending/Progress (DOCLAD:530) and `acknowledge_artifact_store_replacement` false (DOCLAD:537).
- Funding refusal in the archive driver: `plugin_turn_admitted` false -> `Progress(empty)` (LAD2:~655-658).
- Store-replacement driver likewise returns `Progress(empty)` when unfunded (LAD2:~452-455, ~436-445 `Initializing`), and its own `[DEBUG] store replacement zero reading` trace (LAD2:416-424) shows someone already saw empty readings from this lane.
In a native fixture all of these resolve within one synchronous turn; in the browser the paged-parent pages, worker decode, member open and replacement are completed by async host work across frames, so 8+ consecutive turns of waiting is normal. `document_archive_waits_for_host` (LAD:76, LAD2:993) only converts the wait to `AwaitingInput` when the archive has NO runnable target (`document_archive_target` None, i.e. `awaits_merge`/terminal); it does nothing for the runnable-but-waiting-on-IO phases above, where `archive_pending` stays true and the ladder reports an empty `Progress`.

`Blocked` can come from: Stage 3 (snapshot retirement waiting on its live media owner, LAD2:192), Stage 23 (`pump_worker_job_retirements` -> `Blocked`, LAD2:169; wasm worker-job custody is parked), and rungs reaching `Blocked` from `close_step`s (pump-owned `Worker::Blocked`, PUMP:63/110). `awaiting.or(blocked)` (LAD:79) surfaces a Blocked only when nothing advanced and no `AwaitingInput`. During an archive load Stage 23 (parked worker retirements) is the most plausible Blocked source; it is by definition "waiting on another owner".

Can the archive step itself be starved of funds? Unlikely as the primary cause:
- Pump 0 has 64 items (>= 16), 1 MiB copy, 16 MiB capacity; `covering_grant` widens copy/capacity/depth to the quoted demand within the stock ceiling (LAD:57, mounted-owner:35).
- Archive demand per phase is small except `HydrateParent` capacity (birth bytes) and `FillMember` copy (<= `OWNED_SCHEMA_DECODE_PAGE_BYTES`) (DOCLAD:138-160), both within the 16 MiB / 1 MiB policy.
- Demand-never-funded is checkable: `document_load_diagnostic` (LAD2:975-991) prints each load's `demand` and `funded` against the grant, but the current `[DEBUG]` line (CT:106) prints only `progress` and `LAST_MAINTENANCE_STAGE`, not this diagnostic.
- Real budget starvation exists only on pumps 1..31 (see (c)).

Verdict: (d) archive/replacement waiting on the host or a worker, amplified by (c) counting unfunded tail pumps. Starvation of the archive step by grant size is not indicated.

## 4. Is the classification semantically correct?

No, for three of the four inputs.

1. `should_yield` before any unit is not progress evidence. It is already handled by `contended`; but handled only AFTER the step through a heuristic on the leftover stock. The ladder itself returns an indistinguishable `Progress(empty)` (LAD:34, LAD:46, LAD:47).
2. Unfunded-by-remaining-stock (LAD:53/54, i.e. `1 <= items <= 16`) is the same scheduling latency but is not recognised (PUMP:284 only tests `items == 0`). It is misclassified as a stall.
3. `Blocked` ("waits for another owner") is by name and by the close verdict (CT:182) an external wait. Counting it as a stall at CT:40 is wrong and inconsistent with the close side; a real deadlock of two owners waiting on each other is better judged by "no owner of the instance advanced", which the live verdict already approximates via `instance_progress_epoch` (CT:29, document ops only at ops/document:187/218) but only on document-open boundaries.
4. A zero receipt from an archive phase that is waiting for host IO / replacement / worker is not a stall either. `Progress(empty)` is overloaded to mean "idle", "starved", "waiting" and "stalled".

## 5. Recommended clean root fix

Make the reading carry its cause instead of inferring it from receipts. Schema-first, no legacy path:

1. Introduce an explicit reading vocabulary at the ladder boundary (single owner, e.g. next to `PluginLifecycleStep`): `Progress(receipt != empty)`, `Starved { budget: Scheduling | Items }`, `Waiting { owner: Host | Worker | Replacement | Media }`, `Idle`/`Complete`. `AwaitingInput` already means host wait; fold `Blocked` into `Waiting` or keep `Blocked` but give it the same non-stall semantics as the close verdict.
2. Ladder (`LAD`): return `Starved` at LAD:34, LAD:46, LAD:47 and when every unit was skipped solely by the 16-item archive reserve (LAD:53/54), instead of `Progress(empty)`. Do not rely on the post-hoc `runtime_live_pass_budget_spent` heuristic.
3. Archive/replacement drivers (`DOCLAD` 305-309, 329/333, 438, 442, 530, 537; `LAD2` funding refusals): return `Waiting { owner }` (`AwaitingInput` with a reason) when the phase is waiting for the paged-parent receiver, decoder bytes, the replacement owner, or a worker; keep `Progress(empty)` only for a real zero-work turn that is expected to be followed by work.
4. Live verdict (CT:36-44): `Starved` -> `contended` (no debt, do not clear); `Waiting` -> `clear()` like `AwaitingInput`; only `Progress(empty)` with a runnable, funded, non-waiting rung is a stall. Keep the dual count+time rule, but make the unit "turns" not "pumps" (count once per reactor turn, not per pump: 256 readings should not be satisfiable in 8 turns) and price time in the same currency as the close verdict.
5. Guard the repeat pump (TURN:123-134): do not keep pumping a pressured instance once `cx.retained_grant().items <= DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS` while the archive is the pressure; this removes 31 of 32 fruitless readings per turn at the source.
6. Waiting needs its own bound so a hung host cannot wedge a load silently: a per-wait deadline owned by the archive load (cancellable, with progress shown by the existing load progress UI), reported as an archive-load fault with its cause, not `plugin.internal.zero-progress`.
7. Tests (language-agnostic vector in `🩺️runtime-fault-vectors.json`): (i) 256+ readings of `Waiting` over 100 ms never fault; (ii) `Starved` readings never buy debt; (iii) `Progress(empty)` runnable-and-funded for N turns and T us still faults; (iv) a simulated async host (receiver ready only after K turns) loads an archive without fault. Cross-check with a third-party async executor harness.

## 6. Cheapest confirmation before changing code

Run the browser build once with the `[DEBUG]` at CT:106 extended to print `document_load_diagnostic(grant)` (LAD2:975) and the last `Progress(empty)` source (add the phase name). Expected: `phase` in `HydrateParent` with `receiver_ready=false`, or `AwaitingMembers`/`Initializing`, `funded=Some(true)`, and `LAST_MAINTENANCE_STAGE` 24 or 14. If `funded=Some(false)` instead, the grant is the problem and Section 3 (starvation of the archive step) applies.
