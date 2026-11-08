# 1107 Audit Review 11

Read-only audit of lane `review-11`. No cargo, no edits, no git modifying commands, no sub-agents. Static reading only: nothing below is compile- or test-verified.

Paths are relative to `/Users/ueli/Documents/semio/`. Abbreviations:
- `TR-P` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/`
- `TM-P` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/`
- `TR-F` = `🧰️framework/🔨️modules/⏯️tool-run/`
- `AB` = `🧰️framework/🔨️modules/🎯️action-bus/`
- `SEALED` = `🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs`

Crates reviewed: `semio-plugin-tool-run` (`TR-P`), `semio-plugin-tool-machine` (`TM-P`, new), `semio-framework-action-bus` (`AB`). Also `semio-framework-tool-run` (`TR-F`) because it carries the same `admit_sealed` logic.

## Answers to the brief's direct questions

1. **Is work now uncharged after the `admit_sealed` surplus rollback?** No. On the current constants the surplus path is dead code, and the sealed admission over-debits by one unit. See F2.
2. **Is the `duplicate_sealed` debit removal correct?** Yes. The single debit is inside `try_duplicate`. See F3.
3. **Does `selected_entry_required!` hide errors or half-update state?** Mostly no for the macro itself. Two call-site patterns half-move state or drop errors. See F6 and F7.

## Findings (highest first)

### F1. Single-charge violated in the tool-run native tick (high)
`TR-P` `🎞️tick/🛬️native/🦀️.rs:101`:
`cx.with_original_effect(...)` returns the effect's `performed` work, and the same line then runs `cx.consume_fuel(work.max(performed))`. Here `work = 1 + compared_items + compared_bytes + moved_items` on `Ok` and `1` on `Err`.
- Failure: `compared_bytes` is a byte count used as work units. A presentation advance that compares 4 KiB debits about 4000 fuel after performing a handful of actions. The turn runs out of fuel early, and the next turn backs off or refuses on WorkLimit.
- The law in the brief is "fuel debited == turn.work". This debits `max(heuristic, performed)`, a second charge that is not the receipt.
- Fix: debit `performed` only. Delete the heuristic. The constant `1` on `Err` is the W2 anti-pattern.

### F2. `admit_sealed` quote does not match actual admission work (high, predicted red)
- `TR-F` `🦀️.rs:42-50` and `👥️entities/🦀️.rs:22-27` both quote `SealedShared::<T>::birth_work_demand()`, which is `1 + prepare_birth_work_demand() = 1 + 3 = 4` (`SEALED:87`).
- The actual admission work is `1 + 2 = 3` on success: `SEALED:103` (`work:1+turn.work`), `🔗️arc-custody/📦️original/🦀️.rs:33` (passes `turn.work`), and `🏗️allocation-return/🔗️shared/📦️original/🦀️.rs:21` (`work:2`).
- Consequence 1: `surplus = turn.work - quote` is 0 on every current path. The rollback (`entities 🦀️.rs:25`, `TR-F 🦀️.rs:48`) never runs. Every sealed admission over-debits one unit. That is conservative, not uncharged.
- Consequence 2: the new test `entities 🦀️.rs:112-120` (`sealed_birth_quote_is_exact`) asserts `turn.work == birth_work_demand()` at line 116. Under the arithmetic above that is `3 != 4` and fails. The existing fixture assertion at `SEALED`'s sibling `🎟️work/🧪️tests/🦀️.rs:35` asserts the same equality for `SealedShared::admit`, so it is probably red too. The toolsplit's "exactness" claim is false.
- The rollback itself is untested. `cold_take_exclusive` (`SEALED:42-46`) performs a physical `try_unwrap` with no debit, and `effect.actual()` still reports the freed allocation's size. If the rollback is ever reached, it is uncharged physical work and a stale retained-capacity marker.
- Fix: make the quote and the admission agree. Either quote 3 or have admission report 4. Then delete the rollback or give it a test that reaches it.

### F3. `duplicate_sealed` debit removal is correct (verified by reading)
`SEALED:109-114`: `try_duplicate` debits `duplicate_work_demand()` (3) with `record_retirement_work(effect, ...)` before the physical duplicate, and returns `work = effect.performed_work() - before`. A refused debit returns `Err` before any alias is made. `entities 🦀️.rs:30-32` discards `turn.work` but does not need it. The old pre-record is gone and nothing else pre-records the duplicate quote in `TR-F` `📊️progress/🪟️presentation/` (`capture`, `update`). No uncharged path found. Not proven by a run.

### F4. Retained-command and action-bus construction work is a constant (medium)
- `AB 🚪️admission/🦀️.rs:88` and `:127`: `work: u128::from(result.is_ok())`. A successful construct reports 1 whatever it performed. A failed construct reports 0 even if owner births allocated before the error (`admit_owner`, line 72, reserves quota only). This is the W2 pattern: a constant is honest only for exactly one action.
- Per-call constants: `:96` `copy_bytes: complete.copy_bytes + 64*size_of::<usize>()` and `items + 3`, `total + 3` at `:96`, `:98`, `:103`. `total + 3` can overflow in debug for large `total`.
- Caller: `🧵️retained-command/🏭️producer/🦀️.rs:26` passes a settled quote whose nested work is not in the `work` argument (1).

### F5. Local admissions can be dropped mid-frame (medium, unverified)
- Callers pass `&mut Default::default()` as the admission: `producer 🦀️.rs:26`, `💡️inference/🫴️receiving/🦀️.rs:43,101`, `♾️infinite/.../📥️fill-job-and-snapshot/🦀️.rs:42,389`, `🚪️snapshot-ingress/🦀️.rs:29`, `🧪️testing/🎟️caller/🦀️.rs:100`.
- The helper returns `Ok(false)` at `AB 🚪️admission/🦀️.rs:114` when `Pending` consumed no fuel. The caller then drops its local admission.
- `OriginalConstructionAdmission` holds a `NativeReceivingEffect` (`🧵️job/🏭️construction/🦀️.rs:37`). Its `Drop` asserts `phase == Empty` (`🌱️value/🛬️decode/🏭️construction/📦️effect/🦀️.rs:141`). A frame still mid-receive at that return panics.
- The callers2 note says "a lost local admission is harmless". That holds only if the frame is `Empty` at every `Ok(false)` return. I did not verify this. Needs a test that reaches `Pending` with no fuel.

### F6. `selected_entry_required!` at take-then-fail sites (medium, unreachable today)
`TR-P 🧵️instance/📤️publication-and-member-refresh/🦀️.rs`:
- `119-125` (`refresh_tool_run_member`): `member` is taken at 120. The `?` at 123 returns early with `member` still held, dropping it, and `progress` from 122 is never retained (uncharged).
- `152-163` (`settle_tool_run_member`): the same at 162.
- `ToolRunMemberEmissionState::drop` asserts the emission is empty (`👥️member-owners-and-visitors/🦀️.rs:275-277`), so dropping a member that still holds emission panics.
- Reachable only if the selected entry disappears during `refresh_member_overlay`. Nothing I read does that, so I do not claim a live failure.
- Fix: put `member` back before any `?`, and retain `progress` first.

### F7. Errors discarded and turned into rejection or fault (medium)
- `...publication-and-member-refresh/🦀️.rs:185-188`: `_ => return self.reject_tool_run_publication(...)` discards `Some(Err(e))` from `visit_member`. A WorkLimit or allocation failure while decoding member ops rejects the whole composite publication as a conflict. The error and its `retained_progress` are lost, so the work is not charged.
- `...:183` (`Ok(None) | Err(_)` on the revalidate job build) and `...job-admission...:18` (`Ok(None) | Err(_) => fault_tool_run`) discard the `Fault`. `:142` (`Err(_) => fault_tool_run`) discards it too. Faults become "run faulted" with no diagnostic.
- `Ok(None)` from `build_tool_run_job_slot` means "no job builder", which is correctly a fault. The `Err` arm is the problem.

### F8. State is mutated before a fallible debit (medium)
- `...publication-and-member-refresh/🦀️.rs:26`: `finalize.phase = Closing` is set before `cx.consume_retained(...)?`.
- `:33`: `finalize.publication = Some(publication)` is stored before `cx.consume_retained(p)?`. If the debit fails, the admitted publication exists with its receipt uncharged.
- `:51-55`: `publication.acknowledge_granted(child)` runs before `cx.consume_retained(...)?`.
- `...job-admission-and-finalize-turn/🦀️.rs:57` and `:67`: `job.outcome_received = true` is set before `consume_retained(...)?`. On the next turn `:45-48` sees `outcome_received` and only acknowledges. It never calls `complete_tool_run_job`, so a failed debit at `:67` leaves the run uncompleted for good. The `retained_work_can_enter` guard at `:66` probably makes this unreachable, but the order is wrong.

### F9. Leftover re-export alias (forbidden, medium)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🦀️.rs:46`: `pub(crate) use semio_plugin_tool_machine as tool_jobs_tool_machine;`
- `:52`: `pub(crate) use super::tool_jobs_tool_machine as tool_machine;`, with `pub use tool_machine::{...}` at `:53`.
- This is the alias the toolsplit said it removed. It is still present in this file (modified 17:07). It is the only remaining `tool_jobs_tool_machine` hit in `🧰️framework`.
- Also `TM-P 🦀️.rs:27-29`: `extern crate semio_os_kernel as store|protocol|pack`. These are three crate-name aliases in the new crate. `TR-P 🦀️.rs:8-10` has the same pattern, which predates this wave. Remove them with the rest.

### F10. Unfunded close quotes stall without a typed refusal (low, W4)
- `TR-P 📤️publication-and-member-refresh/🦀️.rs:76`, `:80`, `:93` return `Ok(false)` / `Ok(())` when `tool_grant_funds` fails. No ceiling check exists, unlike the settled helper's `AB:98`.
- `TR-P 🎞️tick/🛬️native/🦀️.rs` `close_with`: `if !funds(cx,demand){return Pending}`.
- `TR-P 🧵️instance/🧹️member-retirement-and-close/🦀️.rs:96`: `unwrap_or_else(... Progress)`.
- The copy demands are small, so these are probably funded in practice. The W4 rule still requires a typed refusal or `Blocked`.

### F11. Production `unwrap`/`expect` in `TM-P` (low)
- `TM-P 🦀️.rs:339`: `self.typing.open(window).expect("a listed run is open")`. Holds only if `typing.windows()` lists exactly the open runs. Not verified.
- `TM-P 🔁️follow/🦀️.rs:42` and `:73` use `self.windows.as_ref().unwrap()`. `:44` and `:59` use the typed `invalid`. `:73` is in `demands()`, guarded only by the `closing` check at `:66`.
- `TM-P 🫧️host/🦀️.rs:53`, `:64`, `:109`: each is preceded by a guard that makes it safe, so these are proven, not faults. They still violate the no-unwrap rule.

## Dependencies and manifests (30 manifests reference `semio-plugin-tool-machine`)
- No cycle found among the direct dependencies of `TM-P`. `semio-plugin-tool-run-operations` depends only on the framework `semio-framework-tool-machine`, not on the plugin crate. Not a full transitive check.
- `TM-P` and `TR-P` carry `testkit` only as `[dev-dependencies]`. The new crate has no production testkit.
- Companion test crates list `semio-framework-job` with `features = ["testkit"]` under `[dependencies]`: `tests-05-3`, `tests-06-2`, `tests-02-2`, `tests-05-2` (present at HEAD, so pre-existing, not from this wave), and `tests-05`, `tests-06` (HEAD not checked).
- `AB` feature `original-wire-testing` enables `semio-framework-job/testkit`. I did not check whether any production crate enables it.

## Checked and clean
- No `[DEBUG]`, `#[allow`, `println!`, `eprintln!` in the three crate trees (outside `🧪️tests`).
- `tool_run_slot_lost` and the `selected_entry_required!` macro body (`TR-P 🦀️.rs:88-97`) are correct.
- `selected_entry_required_tests` (`TR-P 🦀️.rs:100-109`) exercises a stub `Ledger`, not the real `ToolRunLedger::apply_event` removal path. It does not prove the real path.
- `retain_original_receipt` (`📒️driver-and-ledger/🦀️.rs:113`) is single-slot. Callers that retain twice in one turn would fail. Not seen in the three crates.

## Not checked / limits
- The construction stall (lane `construct`) was excluded as instructed.
- The plugin retained-command crate was checked only at `producer/🦀️.rs`.
- `StepContext::with_original_effect` (`🧵️job/⏱️context/🦀️.rs:269`) was read only for its signature and effect sequence. I did not verify `receive_original_retained_work`.
- No compile, test or run. All verdicts are static.
- "Lowered quotes": the `git diff` of the tracked files in `TR-P` and `TM-P` showed no obvious lowered work constant. Not exhaustive.

## Scratch files left behind
I wrote two lists into the lane's generated folder: `🗑️generated/1107-review11-manifests.txt` and `🗑️generated/1107-review11-edited.txt` (under the ticket folder). The brief forbids `rm` in `🗑️generated`, so I did not delete them. The coordinator should remove them.

## Top finding
F1: `TR-P 🎞️tick/🛬️native/🦀️.rs:101` debits `max(heuristic, performed)` fuel, a second charge that breaks the single-charge rule and can exhaust a turn early.
