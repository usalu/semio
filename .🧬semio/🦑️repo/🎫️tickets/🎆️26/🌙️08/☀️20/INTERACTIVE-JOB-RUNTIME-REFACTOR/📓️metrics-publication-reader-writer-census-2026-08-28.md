# Metrics Publication Reader and Writer Census

Historical read-only WGPU source census, corrected below to distinguish accepted input from observed build input. The later approved single-enqueue helper extraction is recorded separately; no native execution or guarded-state repair is claimed here. Paths below are in the existing WGPU target unless stated otherwise.

## Actual Current Accesses

| Authority | Writers | Readers / consumers |
| --- | --- | --- |
| RuntimeMailboxInner.next_revision | glue::completion11821; native/guest spawn_interaction_reserved12441/12454; native/guest spawn_dispatch_reserved12467/12584; native/guest spawn_frame_deferred_reserved12480/12571; native try_spawn_frame_maintenance_reserved12502; native maybe_reload_native_plugins15013 | Completion.revision is consumed by apply_pending_step12602ff and its applied_revisions stale check. Reserved futures retain their revision through completion. |
| RuntimePresentationAuthority.scene_revision | mark_scene_changed11693, called after enqueue11832 and finish11843 | current11701 loads scene/input separately; witness_for11705 delegates to it. |
| RuntimePresentationAuthority.input_generation | observe_input_generation11697 via RuntimeMailbox12007, called by winit_app::redraw_core188 | FrameJob boundary reads through presentation_witness_for at glue13239/13311; native maintenance reads witness_for12522; AppPresenter BeginGpu14622 and Acknowledge14752 call current directly. |
| Completion ready/in_flight | enqueue/reserve/reserve_interaction/cancel/finish in runtime_mailbox_core; producers through glue11826ff,12369ff,12415ff | apply_pending_step12591ff locks then removes front before applying; has_lossless_capacity12373 reads under blocking lock. |
| Snapshot sink revision | render_snapshot::SnapshotSink::next_revision97 and winit_app206 | Its Arc snapshot publication is a different authority, produced after a frame. It cannot stand in for immediate input/metrics invalidation or mailbox admission. |

The direct existing presentation test at glue10847 checks candidate identity independently; static source laws at10921ff/11048ff pin two mark_scene_changed calls. They must be migrated semantically with the authored reader/writer cutover, not silenced.

## Concrete Observer Boundary Proposal

Use one exact guarded committed state for the operations' actual shared write sets, reusing the mailbox's existing completion storage/ownership boundary. The former phrase “committed input generation” was ambiguous: RuntimePresentationAuthority.input_generation is observed build input, not accepted EventQueue input. Scene2/buildInput7 is legitimate after a standalone scene change and must remain so. Do not invent a paired scene/build-input transaction, parallel revision ledger, or independently successful CAS pair.

| Actual operation | Fields it writes | Fields it does not advance |
| --- | --- | --- |
| Single runtime enqueue | Actual ready queue and its scene invalidation; completion revision is currently minted before the extracted helper | Observed build input |
| Reserved runtime completion finish | Actual ready/in-flight state and its scene invalidation | Observed build input |
| Observe new frame build input | Observed build input from the selected build generation | Scene revision and completion queue |
| Proposed full metrics commit | EventQueue accepted input, frame generation/scheduler invalidation, surface pending metrics/generation, exact mailbox completion and scene invalidation | Observed build input; redraw advances it later |

Every affected writer must use the same short-lived committed-state access for its own write set, with checked successors where the operation mints a generation/revision. Every affected reader must obtain a complete state for the operation through that guard or decline with explicit Busy/Poisoned. This is not a requirement that independent operations advance all tuple fields together.

The original EventQueue and surface lane are owned exclusively by the UI callback; their prepared mutations occur while the same mailbox commit guard is held. This yields a reader-observable old-or-new mailbox/presentation tuple only if the complete input/surface write set occurs before releasing that guard. A test interlock after each field write must show a foreign reader cannot observe a half-committed tuple. It must return Busy, never spin/block. After release it must see all new fields and exactly one completion. On preflight failure it must see all old fields and the original source.

No new Arc allocation or root-to-root owner promotion may occur in the callback. Existing presenter authority handles would need to refer to this same preadmitted committed state, preserving its exact final-reader retirement owner. That construction and physical metadata must join actual Opening funding; it is not supplied by the current numeric input root tag.

Reader migration must precede taking live owners. In particular AppPresentPhase::Acknowledge currently takes cursor.witness before reading current; a new Busy result must be checked before that take so the exact witness/frame remains installed. BeginGpu must return pending on Busy before issuing GPU work. Frame/maintenance readers must preserve their installed owners on Busy rather than marking an otherwise valid candidate stale. This changes observation/admission only; no claim of fixing the separate irreversible GPU callback/rollback problem.

Existing current() returns an independently loaded scene/build-input tuple; that independence alone is not a defect, as the unchanged native independence law demonstrates. The real single-enqueue defect under test is completion visibility before that same enqueue's scene invalidation. The snapshot sink publishes a later frame and cannot represent this frontier. A coherent authored receiver/read/write cutover plus real held-lock, poison and intermediate-observer laws remains required. The separately mounted helper/test packet preserves current production behavior and has not yet executed natively.
