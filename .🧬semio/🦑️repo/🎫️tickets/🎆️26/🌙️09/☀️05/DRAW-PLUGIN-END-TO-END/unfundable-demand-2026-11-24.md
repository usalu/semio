# Unfundable demand ends the operation with a typed fault (2026-11-24)

Status: NOT APPLIED. No production file was touched. The coordinator asked to hold framework edits until `🗑️generated/e2e-r37.log` shows "draw e2e prepare" (at the last look it showed only `still working ... boot.loaded`, zero matches), then apply in one go. Nothing was compiled (no cargo allowed). Patches below are final text, to be pasted in order P1 to P5, then P6 (notices) and P7 (law).

## 1. Proven from code (read)

- Silent yield in the ladder: `🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs` `maintenance_ladder_unit_step`: when `plugin_grant_funds(demand, grant)` fails even after `covering_grant(loan, demand, cx.retained_grant())`, it does `cx.consume_fuel(1); continue`. One fuel per turn, no fault: this matches the measured "fuel 1,999,999 of 2,000,000" and the idle drain. The widening ceiling is the context's grant, so a demand above the whole-turn grant is never funded.
- `StepContext::original_turn_ceiling()` returns the grant and fuel the turn was issued before any loan narrowed it (doc: "a quote above it can never be funded"). It does not move when a turn is partly spent, so it is the right "full grant" to compare with, never `retained_grant()` (the remainder). `admit_original_construction_settled` already uses it for the construction quote only (typed `OwnershipLimit` refusal); the retirement quote of the admitted body is not checked.
- `drive_typed_operation_worker` (`🧵️instance/👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs`) knows `mounted_policy` and narrows every step with `original_plugin_turn_grant(policy, retained)`; for non-closing steps that is `policy.preparation`. It returns the step's `Err` as the whole actor's failure. `fault_typed_operation_worker(operation_id, &fault)` (same file, `pub(crate)`, currently without callers) is the existing per-operation fault settlement: cancels the lease, mints the fault page, stage -> `Publishing`, retained owners leave through the cancelled-publication close.
- `worker_step_demands` returns retirement demands only when `worker_outcome_pending || (terminal_seen && !worker_resume_pending)`, producer retirement when cancelled or terminal, and the rejected-session retirement; otherwise a one-depth item or the fault-capture demand. So "live step" = none of cancel, `terminal_seen`, `worker_outcome_pending`, a non-admission `session_rejected`. Close demands are therefore never faulted by the guard below.
- `plugin_grant_shortfall(demand, grant)` already exists in `🏛️ownership/🦀️.rs` and names axis, demanded, granted. It is the single comparison.
- The Draw quote in the stall (copy 875,008 = 2 x 437,504, capacity 437,888, depth 2) is the close quote of the admitted `ArtifactReservedRequestJob` (stage 18, cancellation cleanup), per `clipboard-2026-11-24.md` section 9.

## 2. Inferred, not proven

- Whether the browser turn's `original_turn_ceiling()` equals `INTERACTIVE_COMPONENT.preparation` (the clipboard note says so for the fixture; `local-query-stall` says the guest is issued the whole actor remainder and the plugin narrows with the policy). Therefore guard B (worker step) compares with the intersection of ceiling and policy, and guard A (admission, no policy in reach) with the ceiling only. If the browser ceiling is the whole stock, guard A alone does not fire in the browser; then admission needs the policy carried to `admit_reserved_work` (extra field on `ArtifactReservedToolJobRequest` set by the operation before stepping, or on the context), which is more than a local change and is left as the follow-up below.
- The retirement quote used at admission (`copy 2 x size_of body`, `capacity size_of body`, `depth 2`) is a lower bound of the real quote (real: 875,008 / 437,888 / 2 for 437,504 bytes). A body failing the bound certainly fails the real quote; a body between the two is not caught.
- Whether `Fault` from `admit_reserved_work` reaches the command reply: the `Err` propagates out of `drive_original_producer` (it also sets `user_cancel_requested`/`terminal_seen`), then guard C converts it into the operation's fault page.

## 3. Patches

### P1 `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/🦀️.rs` (after `plugin_grant_shortfall`; `use semio_framework::{Fault,FaultCode,FaultOrigin};`)

```rust
/// 🚫️ The typed, localized refusal of a demand that exceeds the whole grant any turn can have: it names the axis, the demanded and the maximum value, so the operation ends instead of yielding forever. See [`plugin_grant_shortfall`].
pub fn plugin_unfundable_demand_fault(subject: &str, demand: RetirementDemand, maximum: RetainedCloneGrant) -> Option<Fault> {
    let (axis, demanded, limit) = plugin_grant_shortfall(demand, RetainedCloneGrant { maximum_items: maximum.maximum_items.max(1), ..maximum })?;
    Some(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unfundable-demand"), format!("{subject} demands {axis} {demanded}, above the {limit} any turn can grant"))
        .with_param("axis", axis).with_param("demanded", demanded.to_string()).with_param("maximum", limit.to_string()))
}
```

### P2 `🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs` (after `worker_step_demands`)

```rust
    /// 🚫️ The fault of a live worker step whose quote exceeds `maximum`, the whole grant of a turn; retirement and close quotes are never faulted, they must settle.
    pub fn unfundable_worker_step(&self, maximum: RetainedCloneGrant, cancelled: bool) -> Result<Option<Fault>, ValueError> {
        if cancelled || self.user_cancel_requested || self.terminal_seen || self.worker_outcome_pending || self.stage != MountedTypedCommandFullOperationStage::Worker || self.session_rejected.as_ref().is_some_and(|source| !source.admission_pending()) { return Ok(None); }
        let demand = self.worker_step_demands(maximum.maximum_copy_bytes)?;
        Ok(semio_plugin_ownership::bounded_ownership::plugin_unfundable_demand_fault(&self.verb, demand, maximum))
    }
```

### P3 `🔌️plugin/🧵️instance/👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs` `drive_typed_operation_worker`, inside the loop, right after `let grant=...remaining_plugin_turn_grant...;` and before `cx.with_retained_loan`

```rust
            if !closing{
                let ceiling=cx.original_turn_ceiling().0;let maximum=semio_plugin_ownership::bounded_ownership::original_plugin_turn_grant(policy,ceiling);
                if let Some(fault)=operation.unfundable_worker_step(maximum,cx.is_cancelled())?{self.fault_typed_operation_worker(operation_id,&fault)?;break;}
            }
```
(`policy` is `mounted_policy.preparation` here; `?` converts `ValueError` and `Fault` into `PluginLifecycleFailure` as the file's existing `?` on `spent.checked_add` does; if `From<Fault>` is absent use `.map_err(Into::into)`.) The `operation` borrow ends before `self.fault_typed_operation_worker`; re-fetch with `self.tool_operations.get_mut` if the borrow checker objects.

### P4 `🔌️plugin/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs`, `maintenance_ladder_unit_step`, replacing `if !plugin_grant_funds(demand,grant){cx.consume_fuel(1);continue}`

```rust
            if !semio_plugin_ownership::bounded_ownership::plugin_grant_funds(demand,grant){
                if matches!(unit,MaintenanceUnit::Stage(0)){
                    if let MaintenanceTool::Worker(_,operation_id)=self.shared().maintenance_tool_selection(){
                        let fault=self.tools.tool_operations.get(operation_id).map(|operation|operation.unfundable_worker_step(cx.original_turn_ceiling().0,cx.is_cancelled())).transpose()?.flatten();
                        if let Some(fault)=fault{self.tools.fault_typed_operation_worker(operation_id,&fault)?;}
                    }
                }
                cx.consume_fuel(1);continue
            }
```
(`MaintenanceTool` is in the `🪜️maintenance-ladder/🦀️.rs` crate of `shared()`; if it is private to that module, expose a `pub(crate) fn maintenance_worker_operation(&self)->Option<u64>` next to `maintenance_tool_selection` and use it.) Close units (stage 18 and others) are untouched: they keep the old `continue`.

### P5 `🔌️plugin/📜️declaration/👷️job/🎟️request/🏭️reserved/🦀️.rs` `admit_reserved_work`, after `let binding=...;let bytes=size_of::<...>();`

```rust
 let lifecycle=RetirementDemand{copy_bytes:bytes.saturating_mul(2),capacity_bytes:bytes,depth:2,..Default::default()};
 if let Some(fault)=semio_plugin_ownership::bounded_ownership::plugin_unfundable_demand_fault("reserved work body",lifecycle,cx.original_turn_ceiling().0){return Err(fault.into())}
```
(admission refuses before mounting; `OriginalTurnFailure: From<Fault>` is used elsewhere in this crate family, e.g. `Err(Fault::new(...))` in `drive_worker_step`.) Add a guard C in P3: when the step result is `Err(failure)` with `failure.primary_fault().is_some_and(|f| f.code.0=="interactive-job.unfundable-demand")`, call `self.fault_typed_operation_worker(operation_id, fault)` and `break` instead of `failure=Some(..)`: replace `if let Err(fault)=stepped{failure=Some(fault);break;}` by

```rust
            if let Err(fault)=stepped{
                if let Some(unfundable)=fault.primary_fault().filter(|fault|fault.code.0=="interactive-job.unfundable-demand").cloned(){self.fault_typed_operation_worker(operation_id,&unfundable)?;break;}
                failure=Some(fault);break;
            }
```
(`stepped` is `Result<_, OriginalTurnFailure>`; `fault.primary_fault()` is on `OriginalTurnFailure`; if the loan wrapper converts to `PluginLifecycleFailure` first, use that type's equivalent.)

### P6 notice rows (en + de), same position as `plugin.original-stock-exhausted`, code `interactive-job.unfundable-demand`

- `🧰️framework/🔨️modules/🎠️kernel/📜️history/🦀️.rs` (tuple list), `🏷️notice-labels/🟦️.ts` (object list), `🧪️tests/🧫️fixtures/🧫️framework-notices/🔣️.json` (object list):
  - en: `This task needs more working memory ({axis}: {demanded}) than this app can ever grant ({maximum}) — it was stopped; use a smaller selection or document.`
  - de: `Diese Aufgabe braucht mehr Arbeitsspeicher ({axis}: {demanded}) als diese App je gewähren kann ({maximum}) — sie wurde gestoppt; eine kleinere Auswahl oder ein kleineres Dokument verwenden.`
- The kernel test `🧪️tests/🧪️framework-notices/🦀️.rs` iterates the fixture; add the case with params `axis=copy bytes`, `demanded=875008`, `maximum=524288` modelled on its `plugin.channel-mismatch` case (line 49).

### P7 Law (Rust, next to the typed-owner laws in `🏛️ownership/🦀️.rs` `mod typed_owner_work_tests`; no neighbouring fixture JSON exists for this module, so the language-agnostic data is inline constants and the kernel notices fixture of P6)

```rust
    #[test]
    fn a_demand_above_the_whole_grant_is_a_typed_fault_naming_axis_demanded_and_maximum() {
        let maximum = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 524_288, maximum_capacity_bytes: 16_777_216, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let draw = RetirementDemand { copy_bytes: 875_008, capacity_bytes: 437_888, depth: 2, ..Default::default() };
        let fault = plugin_unfundable_demand_fault("copy", draw, maximum).expect("875,008 copy bytes exceed 524,288");
        assert_eq!(fault.code.0, "interactive-job.unfundable-demand");
        assert_eq!((fault.param("axis"), fault.param("demanded"), fault.param("maximum")), (Some("copy bytes"), Some("875008"), Some("524288")));
        assert!(plugin_unfundable_demand_fault("copy", RetirementDemand { copy_bytes: 524_288, depth: 1, ..Default::default() }, maximum).is_none());
        let remainder = RetainedCloneGrant { maximum_copy_bytes: 1, ..maximum };
        assert!(plugin_unfundable_demand_fault("copy", RetirementDemand { copy_bytes: 524_288, ..Default::default() }, maximum).is_none(), "a partly spent grant is never the comparison: only the maximum is");
        let _ = remainder;
    }
```
Operation-level law (P2, needs a `MountedTypedCommandFullOperation` fixture; add to `🧪️tests/🔬️app-typed-command-full-operation/⚖️slot-and-publication-fairness/🦀️.rs` using its existing operation literal): a Worker-stage operation with `fault_capture`/admission quote above the maximum returns `Some(fault)` from `unfundable_worker_step(maximum,false)`, and the same operation with `terminal_seen=true` returns `None` (close must settle); then `app.tools.fault_typed_operation_worker(id,&fault)` leaves `stage == Publishing` and a queued Fault page. Admission law (P5): `admit_reserved_work` for a work type whose `size_of` body exceeds half the ceiling returns the fault and leaves `source` untouched (request still present, `receiving` empty). Neither is written yet: they depend on fixture constructors I could not compile-check.

## 4. Files changed

None (hold). Only this report. Planned: files P1 to P6 above plus the two operation/admission laws.

## 5. Not compiled / not run

Everything (no cargo). Open type-level risks: `?` conversions in P3/P4 between `Fault`, `ValueError`, `PluginLifecycleFailure`, `OriginalTurnFailure`; `MaintenanceTool` visibility in P4; `self.verb` type in P2; `FaultCode.0` is `String` (used as `fault.code.0` in `failure.rs`); `Fault::param` exists (`⚠️diagnostic/🦀️.rs:696`).

## 6. Follow-up if guard A does not fire in the browser

If the browser ceiling is the whole actor stock, carry the policy maximum to admission: add `maximum: Option<RetainedCloneGrant>` to `ArtifactReservedToolJobRequest`, set it in `drive_typed_operation_worker` before the step, and compare in `admit_reserved_work` with `min(ceiling, maximum)`. That touches the request type and every reserved factory, so it was not written.
