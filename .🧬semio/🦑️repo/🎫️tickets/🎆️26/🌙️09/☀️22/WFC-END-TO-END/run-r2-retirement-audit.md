# Publication and Child Retirement Audit

Read-only source evidence during concurrent SDK migration. No queues or source edits.

## Confirmed Physical Box Release Omission

Engine job/📤️publication/🦀️.rs35–46 returns Box<Publication>. Its retirement_demands126–142 quotes retained publication, state/output builders and primary/secondary Vec capacity only. close_step145–180 retires those internals and delivered flag; terminal_is_empty183–185 means inner resources empty, not Box storage deallocated. No branch reports size_of::<Publication>() release. The Box remains allocated until its parent Option is cleared.

All five fill and five inference parents clear publication=None immediately after publication.terminal_is_empty(), often in the same inner closing turn. Representative grid2d fill869–877 and inference around691. That physically frees the Box without the released_bytes receipt or corresponding release admission. This is a concrete physical-budget defect, not merely a preferred style.

Required acceptance obligation: once publication inner state is empty, make parent Box retirement its own paid turn: quote release size_of::<Publication>(),depth1,item1; refuse unchanged when release/items/depth/work cannot enter; take/drop only after admission; report the exact release and1CPU/1item. Do not add Box release to an arbitrary last child turn whose frontier did not include it. Paid inner terminal state and parent container terminal state are distinct.

## Confirmed Parent Header Piggyback

Grid2d fill886–895 calls the child close under a retained loan, then removes child Option in that same turn and replaces copied_items with max(childItems,1). Parallel close_original_nested_step callers exist in every five fill and inference owner. When childItems>=1, this adds no parent item at all. It also consumes no independent parent CPU; child callback accounting cannot pay a second parent event implicitly. Even when childItems=0, synthesizing returned progress after removal differs from actually posting original physical effects.

Required clean route: if child is already terminal on entry, retire/drop parent Option header on its own admitted original metadata turn. Otherwise run one child close turn and return Pending without removing the Option, preserving its actual receipt/work unchanged. An inline child has no additional heap release merely for removing its Option; any Box child needs its exact Box allocation release separately. Use the actual child's storage type, not a guessed heap size.

## Tests Needed

Denied header retirement must leave Option/pointers/resources unchanged for zero work/items, insufficient depth and (for publication Box) insufficient release. At exact frontier, compare allocator release with receipt.released_bytes and original work delta1. A child final close with positive item/work progress must leave parent Option present until a distinct later turn; Pending with zero child progress cannot synthesize a paid parent event. The final complete parent state must be physically empty before Drop. RetainedWorkOutcome callers must preserve both result and actual work through original context admission.

Owners notified directly: root owns grid2d/coordinator; bitmap_finish_r2 owns bitmap/graphs; native_verify_r2 owns engine/grid3d migration. Current exact line numbers can shift under their fixes; this report records the reviewed source snapshot.


## Shared Publication Slot Helper Review

Read publication/📦️owner/🦀️.rs. No concrete defect found. publication_retirement_demands quotes exact size_of::<Publication>() only once internals are empty. close_publication_step leaves inner-close completion as Pending; a later call admits independent Box removal through close_original_retained_step, whose demand preflight refuses unchanged on missing work/items/depth/release. The supplied closure takes/drops slot and reports1item,exactrelease,1CPU. An absent slot returns Complete with zero effects. Canonical close_original_retained_step intentionally maps even RetainedCloneStep::Complete to InteractiveJobCloseStep::Pending, so the header-removal invocation can return Pending with slotNone, and the next invocation returns Complete. Tests must not require Complete on the physical header-removal turn.

## Grid2d Output Writer Migration References

Current grid2d inference encode_one407–421 still uses RetainedJobPayloadWriter, context.admit_payload_page, page.write/commit, and the run branch516 calls writer.finish. Canonical replacement is RetainedPayloadBuilder:
- advance_initialization(&impl JobPayloadStep)->RetainedWorkOutcome, builder source60; poll is_initialized before next phase.
- append_original(&mut impl JobPayloadStep,source:&[u8],cursor:&mut usize)->RetainedWorkOutcome, source98.
- seal(&impl JobPayloadStep)->RetainedWorkOutcome, source115; poll published readiness.
- take_finished(&mut Option<RetainedJobPayload>,RetainedCloneGrant)->RetainedWorkOutcome, source165; requires sealed original, empty output, no staged/incoming/reservation. Transfer is a separate1item/1CPU turn and resets ready/sealed.
Each returned outcome must go through receive_original_retained_work(cx,turn) exactly once to preserve work and effects, including failed turns; fixed consume_fuel(1) is not equivalent. Persist source chunk and append cursor across incremental admission turns; advance_encode only after full chunk consumed, not on a zero-progress append. Seal and ownership transfer must not be collapsed into free finish. Closure/retirement of the now-empty builder remains its own producer lifecycle obligation.

Exact canonical file: 🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🦀️.rs. No source edits or native claims.


## Grid2d Runtime Migration Source Review

Concrete blocker found: inference step_value599 gates retire_delivered only on publication.is_delivered(). Shared helper can finish inner cleanup and clear delivered=false while retaining Some(Box) for the separate header turn. The next step bypasses retirement and polls that empty publication at608–610, potentially staging the old preview again instead of returning Box custody. Fill has the same gate800 and helper775. Retain explicit retiring state across every close step; merely terminal_is_empty can confuse an initially empty staged payload with an already-delivered owner. Root and bitmap owner notified.

Output builder phase review otherwise found no concrete finalization defect: persistent output_chunk/cursor append until complete, separate source chunk release advances encode phase only after slot cleared; initialization,seal,take_finished all receive actual work; step_value612 checks output_payload before another encode invocation, preventing repeated transfer to occupied output. Source reads only, native compilation needed.

Separate already-empty-on-entry child/header removal now consumes1CPU and reports1item under nested close reconciliation. Publication uses exact shared Box helper. Inference close quotes output builder/payload nested demands and retains options across actual childclose. Source stock and physical output chunk release remain explicit. The report is not a native pass or lifetime budget sufficiency proof.


## Allocator Linkage and Tiny-Fuel Work Enforcement

WFC enginejob and oracle owned source searches found no global_allocator. Kernel's RequestedAllocator is explicitly cfg(test) at os/📦️packages/🦀️rust/🦀️.rs33–35; a downstream test linking kernel as dependency with mutation-testing does not compile kernel with cfg(test). Therefore that feature is not proof of tracing allocator linkage. OriginalJobFixtureCaller::admit compares measured requested/released bytes to nonzero root receipts, and publication Box allocator law measures exact release; the owning WFC test binary needs an actual installed canonical tracing allocator. No allocator was added by auditor. Neutral framework headless lifecycle tests use their own source/metadata receipt checks, not OriginalJobFixtureCaller or heap observation; do not describe those tests as physical heap equality evidence.

Exact tiny-fuel enforcement: context::OriginalWorkLoan.matches_original subtracts reserved-return work from physical actual work and requires result<=issued. consume_fuel383 increments actual work even after saturating remaining fuel at0. with_original_work_loan returns InvariantViolated if that check fails; drop still debits actual work to original source. Publication.close_step's initial work1 plus inner childturn.work can therefore cost2+ against a1fuel business retirement turn. It is not authorized bounded primitive overspend. Reserved semantic return credit does not cover publication cleanup. Refactor wrapper/child work into genuine separate turns or report only actual work performed under the available original work loan. Do not hide effects after mutation or widen per-step budgets to bypass this law. Parent notified with exact source evidence.


## All Ten Publication Callers and Nonempty Lifecycle Regression

Read-only current snapshot: all five fill and five inference step bodies gate on Publication.needs_retirement; all ten retire_delivered methods delegate close_publication_step, not manual same-turn Option clearing. Predicate is delivered||retiring. Accepted close marks retiring=true, which persists after delivered clears; terminal_is_empty excludes the retiring control flag so the shared owner helper can remove the physically empty Box on the next admitted turn. Newly staged publications start delivered=false,retiring=false and are polled normally, avoiding the empty-staged-payload ambiguity. No current caller mismatch found.

Shared lifecycle regression delivered_publication_keeps_its_retiring_phase_until_the_box_returns uses vector1, a real7byte Preview, through actual TestJobDriver fuel1. It counts exactly one Preview, requires eventual Complete, and witnesses inner terminal emptiness plus needs_retirement while Box is still present. This discriminates the previous repoll-after-delivered-clear bug and exercises descriptor receive/ACK through the real driver. It is not an identical-empty-tree or unbound-result test.

Separate header frontier test checks denied items/shortrelease/exactrelease with actual allocator equality, independent work/item receipt, and a later absent-slot Complete. These shared tests do not individually instrument all ten parent adapters; parent body regression tests remain their integration evidence. Native execution has not been observed by this auditor.

Publication close now initializes work0 and forwards actual childturn.work unchanged; its own Vec release/deliveredflag actions use work1. The prior fictitious parent1+childwork double charge is removed in inspected source. No frozen neutral edits, queues, or new feature expansion.
