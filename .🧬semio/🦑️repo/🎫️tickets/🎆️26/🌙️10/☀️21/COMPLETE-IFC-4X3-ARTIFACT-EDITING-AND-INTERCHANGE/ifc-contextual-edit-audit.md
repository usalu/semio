# Contextual Snapshot Edit Lifecycle Audit

Read-only2026-10-21. Implementation readiness pending; this is a contract checklist until source lands. No source edits or native builds.

## Framework Baseline

ArtifactCommandWork and BoundedArtifactCommandWork are defined at C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔖️checkpoint-and-work-contract/🦀️.rs. Existing bounded work has consumed guard set before reducer, depth1 work demand, terminal frame release Some(size_of::<Self>()), reports_progressfalse, no checkpoint state. New contextual work must preserve failure single-consumption and frame accounting; use current original StepContext rather than create child/fresh context.

Shared snapshot factory has original build request tool match event.action_id, exact owner/publication contracts, bounded raw bytes and no checkpoint ownership. Keep these. Extent currently performs E::snapshot_edit_is_admitted before scheduling. New semantic admission must remain bounded scalar validation rather than scan graph.

Shared SnapshotEditingEditor::snapshot_edit_emit currently runs ReplaceSource as genesis load, special edit before generic rules, then exact publication/inverse byte admission. Default contextual hook should delegate this exact behavior; IFC override should yield ordinary SetEntityArg Emit only after full successful contextual validation and still check publication limits. Error/cancel/limited must not return Complete Emit; no failed mutation publication.

Existing shared editing tests in editing/🧪️tests/🔬️unit/🦀️.rs cover persisted-path admission, malformed source, original preparation owner/cancel/close, but no contextual one-shot work fixture was present at baseline. Necessary new tests: default semantic parity, originalcx identity+cancel+fuel, repeated step after success/error, tool mismatch, admission extent, no emission after validation failure, terminal frame size/close emptiness and explicit one-shotLimited refusal.

## Fresh Implementation Review

Implementation landed during audit. Shared SnapshotEditingEditor now declares snapshot_edit_emit_in_context(event,snapshot,cx) with default exactly delegating existing snapshot_edit_emit. New SnapshotEditWork replaces only the boxed old bounded reducer in build_snapshot_edit_tool_job; factory keys, payload schema, owner/publication contracts, raw byte limits, no-checkpoint policy, original command/snapshot/context/completion ownership and tool-id checks remain unchanged.

Consumed guard is factored through pub(super) snapshot_edit_contextual_once: repeat refusal first, consumed=true before reducer even if it errors. Work stores only static tool_id, bool and PhantomData; no heap-owned cursor. work_demands depth1, terminal_frame_release_bytes Some(size_of::<Self>()), reports_progressfalse match old BoundedArtifactCommandWork. Default terminal_is_empty/close are valid for this empty-owned-state frame. Extent still uses exact existing admission and work-capacity accounting. No new resumable checkpoint claim: Limited is terminal failure and consumed guard forbids retry in same work.

IFC override performs originalcx semantic scanning via semantic_edit_controlled, cancellation/yield checks and1fuel each graph lookup/uniqueness scan iteration. Admission was changed to bounded canonical path+known field+string/Null event validation without a graph scan. Resolved semantic edits still call check_publication_limits then emit ordinary SetEntityArg; default persisted edits fall back to existing emission, retaining source genesis-load and complex-edit behavior. No direct state writes or separate publication/inverse authority introduced. Errors propagate through OriginalTurnFailure and do not produce Complete Emit.

### Actionable Exact-Fuel Boundary Gap

At reviewed IFC editor lines325–329, callback tests should_yield before consume_fuel(1), but no final originalcx check follows semantic_edit_controlled. Its last current-value lookup can consume the final fuel unit, return successful mutations and then return Complete Emit although originalcx.should_yield now true. The new shared neutral fixture deliberately expects zero emission when consume exhausts original fuel, but its bespoke reducer checks should_yield after consume; actual IFC hook lacks that postcheck. Add a final cancellation/yield check after semantic planning and before Complete, preferably recheck after publication byte/inverse admission too; add an exact-budget test against actual IFC contextual override rather than only helper closure. Author notified directly; no source fix by audit agent.

### Tests Inspected

New shared fixture editing/🧪️tests/🧫️fixtures/🎟️contextual-snapshot-edit.json has normal delegate, initially exhausted, initially cancelled and consume-to-zero cases. Shared test confirms identical StepContext address/cancel-token node, remaining original fuel, no returned values for stopped reduction and repeat refusal. It exercises helper with a bespoke closure, not SnapshotEditWork full factory or actual default/IFC hook. Missing tests remain actual tool-id rejection, extent/work/frame release contracts, default trait behavior across source+special+generic edits, actual semantic exact-budget/failed-inverse zero publication and terminal host completion.

No native test execution or typecheck was performed. Full function line confirms pub(super) helper visibility is correct; an initial truncated snippet hid that qualifier, the mistaken alert was retracted immediately and is not a defect claim. No additional proven wrong API/signature found in new work implementation.

## Final Read After Author Corrections

Re-read actual source after readiness notification: IFC contextual hook now checks originalcx.is_cancelled/should_yield immediately after semantic planning and AGAIN after check_publication_limits, before returning Emit. The exact-final-fuel finding is resolved in source. Inspector projection likewise rechecks original control before publishing fields. No outstanding concrete guard/tool/frame/default-delegation defect established in current shared contextual implementation. Test gaps and native execution limits above remain accurate.

## Fresh Extensible Work Hook Review

Current shared SnapshotEditingEditor now declares snapshot_edit_work_capacity() default one point-invertible item and snapshot_edit_work(tool_id) default boxed SnapshotEditWork. Shared factory uses E capacity work_items in payload and default extent, and E work hook for original work owner. Ordinary editor behavior therefore retains the same consumed-before-reducer contextual emission, depth1 entry demand, reports_progressfalse, exact fixed-frame release and no checkpoint resume. Units remain staged forward+inverse rows; default capacity is2rows, not1row. Store actual emitted gesture footprint remains independently derived by mounted publication; these hooks do not replace Store authority.

IFC override chooses retained IfcDetailsWork with capacity computed from maximum version catalog attribute count (2*attributes+3 point-invertible primitives, converted by capacity to staged rows). That is a declared bounded upper estimate, not measured actual row count. It does not itself reject source entities with arbitrary extra malformed tuple arguments; an oversized type shrink can emit more primitive removals than this estimate, while actual Store still independently checks emitted leaves. Enforce planner's actual resulting rows against declared capacity if this is intended as a command cap, and test malformed oversized tuples as well as normative templates.

Concrete fresh custom-work regression reported directly to parent and High: work.rs final owned-copy branch invokes unlimited normalized_reference_edit rather than controlled variant; native_argument_edit/owned_entity_edit then returns Complete after only cancellation check, bypassing editor contextual full-details planner, final should_yield guards and check_publication_limits. owned_entity_edit itself supplies unlimited closures to GUID/constituent planning. Retained cloning is bounded, but this final phase is not thereby original-context controlled. The final custom work path needs controlled normalization and every expensive helper, final fuel+cancel checks, and explicit publication item-bound validation without replacing sequential Store inverse authority. This finding concerns the new IFC custom override; default shared hook remains correct by source inspection.

Native source API checks found Canceled is the actual ValueRefusalKind spelling; shared work type visibility permits trait default construction. Source owner uses captured Arc snapshot and retained clone cursors; no direct snapshot state mutation occurs. Exact close demands/frame and actual retained batch execution still need author validation; no new native compilation claim.

## Latest Controlled Custom Work Resolution

Fresh work.rs now calls normalized_reference_edit_controlled with work_tick(originalcx); native_argument_edit/owned_entity_edit and retype/GUID helpers receive the same callback. Prepared mutation vector moves into self.pending before final checks, retaining its owner for failure close. Final path checks cancellation, charges work_tick, checks yield, validates publication bytes by borrowing pending, then checks cancel/yield again before taking mutations into Complete. Prior unlimited/final-guard bypass is resolved by source inspection. Pending vector is included in terminal_empty, demand and close branches. Sequential borrowed pending validation followed by take has no obvious new Rust move/borrow error.

Still unverified: exact command-capacity boundary. Current IFC4 maximum catalog allAttributes=21; formula allows45point-invertible mutations/90staged rows. An existing malformed IFCWALL with1000args switched to IFCCARTESIANPOINT can plan999removals plus name/value changes, exceeding this estimate. No source resulting-row guard compares emitted primitives to command capacity. Actual Store merged footprint remains independent and may admit this much larger batch; document semantic underdeclaration accurately.

Regression evidence remains limited: work.rs native unit exercises work_tick with original cancellation token and exact fuel; it does not instantiate IfcDetailsWork or drive retained copy/finalbranch. Full-details native tests drive controlled planner plus sequential inverse/apply, not shared factory and mounted Store. Therefore no executed proof of actual customwork cancellation/finalpublication/retirement or capacity boundary can be claimed from these tests. Native build remains gated; source syntax is not runtime validation.

## Authoritative Recheck Correction

A subsequent fresh read found work_tick/final branch had reverted to cancel-only plus one final fuel charge; the stronger guards described above were not then present. Main owns patch. Latest exact source/test scaffolding and safe base-independent row declaration are recorded in ifc-retained-final-accounting-audit.md. Treat earlier resolution as historical until final fresh verification.

## Fresh Main Pending/Rows Patch

Checked declared inverse-row arithmetic and actual cap validation now exist in current custom work; pending/publication_validated state preserves planning across publication yield. No obvious borrow/type defect. Exact0fuel native helper test expectation is stale and was reported to main. Latest full details in ifc-retained-final-accounting-audit.md; actual branch execution remains unverified.
