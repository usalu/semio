# WFC Fill And Persistence Audit

Read-only source audit2026-11-04. No runtime/build launch. Paths below use root ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts and each artifact suffix /🏅️standards/🔖️1/🪆️subsets/✳️any.

## Blocking Findings

### Grid3d Executes A Complete Cold Solve During Render

🧱️grid3d/.../✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs paint_assignments(document,tool_run) lines135-146 calls solve(document).unwrap_or_default() whenever no live fill payload exists. render lines168-175 invokes it on boot and after dismissal. There is no StepContext/progress/cancellation on this call. A larger controls fixture therefore triggers complete solving before the interactive test starts, and render after abort starts another solve. This defeats the interaction requirement and can mask loss of the finished cache because a fresh deterministic solve looks identical.

finish_success in 🛠️tools/🪣️fill/🦀️.rs lines607-623 calls preview::commit_fill_result before Complete. preview commit_fill_result104-109 writes RESIDENCY and LAST_FILL, but paint_assignments never reads either as its finished source. Replace cold render fallback with revision-fenced committed results through an owned app/window transient path. An empty cache must paint unsolved, not initiate solving.

RESIDENCY is process-global Mutex<BTreeMap<String,...>> keyed only by window id (99,152-160); no removal was found in this preview module, despite its bounded-by-live-window docstring. LAST_FILL101 is also process-global and not document/revision keyed. It is a test observer, not a correct multi-app cache. Any replacement should use existing instance-owned ephemeral publication rather than expand this static.

### Grid2d Cache Has No Exact Source Fence

🔲️grid2d/.../✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs cached_commit242-251 checks only that assignment coordinates fit and tile IDs exist. paint_commit271-275 accepts live payload unconditionally or this cache. Same-size pin/rule/seed changes preserve these checks while invalidating the solution. A conflicting pin on an existing cell can therefore leave a visibly wrong prior assignment after the edit.

✏️editor/🦀️.rs CommitFill571-572 writes solve_json into addressed pane config. The payload carries no canonical source fence. tools/🪣fill commit_effect148-154 serializes solveJson only, and finish_success623-638 emits it before Complete. Tests preview/🧪️tests/🔬️unit/🦀️.rs103-106 cover pipes-to-terrain IDs, which cannot catch same-extent/same-ID edits.

Add a canonical source revision to finished cache transport; filter both live and finished rendering against actual render operation revision. Test conflicting pin, changed rule and changed seed with identical IDs/extents, plus unchanged revision retention after dismiss.

### Graph2d And Graph3d Finished Transients Are Unbound

◻️2d and 🧊️3d/.../✏️editor/🫧️transient/🦀️.rs define assignments/contradiction without source revision. Graph2d preview_layers_json204-213 looks up assignment by slot id without validating the source; Graph3d graph-view-and-editor render_body364-380 paints payload/transient without revision filtering. Thus a changed pin/rule/seed with the same slot/tile IDs can continue displaying the previous solve.

Graph2d retained completion is 🧾️command-enum-and-retained-routes/🦀️.rs237-251; Graph3d is 🧵️retained-commands/🦀️.rs145-156. Both decode payload then emit set_solve_mutations into the app-local transient. Normal document commands emit no transient invalidation in these match arms. No exact source-bound cache was found. This app-local/shared-across-windows lane is the right scope for finished assignments; bind it to the original document revision rather than convert it to persistent document edits.

## Canonical APIs Already Demonstrated Locally

Bitmap is the current working source pattern: ArtifactView::render_operation_ref().canonical_base_revision supplies render identity; tool_run.identity.base_revision supplies live-result origin; ArtifactCommandInputs.context.canonical_base_revision supplies command context. Bitmap output render accepts current_revision:Option<[u8;32]> and rejects unmatched live/cached results. Its completion command explicitly transports source_revision captured by the original job, not the current revision at callback delivery. Re-stamping an old result at callback time would conceal staleness.

For graph completions preserve original source identity in the fill payload/commit transport and SetSolve transient mutation. EphemeralEmit.transient is instance-owned/shared across panes and excluded from authored document history. Grid2d already has a window-config lane; if it stays pane-local, completion must address the cache consumed by the preview pane. Current solve_json write tests explicitly manufacture a preview view; real focus-switch/dismiss test is necessary.

## Publication And Persistence Qualification

All four nonbitmap fill adapters expose interactive child work with fuel/deadline checks. Graph2d finish_success629-633, graph3d649-653 and grid2d623-638 publish derived completion effects before finalization. These effects target preview lanes rather than document mutations. Abort-after-Complete therefore requires unchanged authored ledger, not necessarily absence of an ephemeral completed preview. No new document-writing solve bug was established.

Authored model/seed/pins persistence remains the intended contract; app transient previews need not survive reload. Grid3d cold re-solving on render currently makes screenshot retention after dismissal a weak witness: it does not prove the committed result is read. Regression must instrument/refuse cold solve in render, then prove the exact completed assignment remains after dismiss and that abort never triggers fresh solving.

Existing preview tests mainly construct solved transients or cached commits directly and compare scene JSON. Required new laws should exercise mounted render authority, unchanged versus changed revision, completion-to-cache routing across panes, dismiss retention, new run abort preserving prior cache, and two app instances with identical window IDs remaining isolated. Native execution remains owned by consolidated verifier.
