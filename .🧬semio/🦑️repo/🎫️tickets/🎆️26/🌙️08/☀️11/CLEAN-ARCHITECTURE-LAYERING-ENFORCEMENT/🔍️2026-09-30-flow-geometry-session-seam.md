# Flow Geometry Session Seam Audit

Read-only baseline inspection while the owning agent designs the replacement seam. No source edits, builds, runtime test execution, or Git mutations.

Current framework `🌊️flow/📐️brep-geometry/🦀️.rs` directly imports the concrete S-owned Brep implementation and declares process-global KERNEL, MESH_CACHE, and TESSELLATION_JOBS. `kernel()` silently constructs Brep; `with_kernel` and tessellation APIs access these ambient singletons. This remains an unresolved implementation seam, not a neutral geometry port supplied by a host.

Concrete cancellation coupling: `🌊️flow/🖥️host/🦀️.rs:3826` invokes global `cancel_all_tessellations()` from one `FlowEvalSession::cancel_preview_evaluation`. The callee iterates every retained job and clears the global registry rather than restricting cancellation to the calling session. Source therefore permits one session's cancel to retire another session's local geometry job. This is established from the exact caller/callee ownership; no runtime pass/failure claim is made.

Current host retention aggregates every session's handles into the process-global FLOW_SESSION_GEOMETRY map before invoking ambient retain_geometry_handles. The intended review criteria for replacement are explicit supplied per-session geometry authority, explicit host/session lifetime and disposal, session-bounded retained-job cancellation, and no ambient mutable default concrete kernel. The owner and coordinator were notified; this report is a baseline finding and must be revisited after their settled seam handoff.

## Reviewable Replacement Source

The owner relayed reviewable replacement sources: neutral framework `flow/🌐️geometry/🦀️.rs`, Flow host/eval builders, concrete `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs`, and its TS BrowserSession wrapper. Generic constructors now carry no provider and report a missing port; concrete Session::port creates separate job registries over an explicitly shared kernel family. This removes the original global job-cancellation mechanism and correctly separates jobs even for the same geometry handle.

Actionable review findings sent to the owner/coordinator:

1. retain_geometry_handles and close compute merged claims under the shared claims lock, release it, then call kernel.retain. A concurrent peer can add a live claim after that snapshot and have its handle disposed by the stale snapshot. GeometryPort::dispose likewise releases the claim-check lock before disposing, admitting a peer claim between check and destruction. Shared claim admission and handle retirement need one consistent transaction/lock order.
2. Session.close has no terminal authority flag; retained clones can retain/tessellate again after close. The TS wrapper frees the same resolved BrowserSession on repeated close and leaves invoke usable afterward. Explicit close must be idempotent and terminal.
3. CAD SemioBrepKernel still exports the module-global semioBrepKernel instance, and registry defaultSpatialKernel returns it. SemioBrepEngine eagerly constructs the BrowserSession but exposes no complete-session close; SemioBrepKernel likewise lacks session teardown. This live composition retains an ambient mutable concrete default and does not yet bind geometry lifetime to a host/session.
4. The peer-claim refusal exists only in GeometryPort::dispose. Public Session::dispose_geometry, BrowserSession.dispose, and brep_invoke_json's dispose branch directly dispose the shared kernel handle without that refusal. Linked operators and ports sharing a Session kernel therefore retain a concrete bypass; disposal protection must cover every actual entrypoint.

These are current-source findings, not independently executed native failures. Source compilation and implementation-owner authority laws remain in progress. Later repairs must be reviewed before marking the seam settled.

## First Repair Review

Owner's next handoff now has an idempotent terminal AtomicBool, claims guards held across port/kernel/cache retirement, implicit handle claim before retained tessellation admission, TS close returning one transition promise with invocation refusal, removed unused CAD singleton/default, and explicit engine/kernel close APIs. These address the original snapshot race and ambient CAD default findings.

Remaining current-source findings sent to the owner: wire brep_invoke_inner dispose and retain still directly mutate the shared kernel without peer-claim protection/merged claim retention; direct import_solid_json and export_solid_json lack terminal-session checks; successful dispose_geometry no longer retires matching own retained jobs. The outer wire claims guard means fixes must avoid recursively acquiring that same mutex. Verification remains pending, and the original nine-law run's seven passes/two numeric-oracle failures are not summarized as green.

Second source handoff repairs wire/direct disposal and merged wire retention while holding claims; own retained jobs are now removed on disposal and direct import/export reject closed sessions. A remaining reproducible sequential ownership gap was sent to the owner: create a root handle through box, open and close an unused sibling port, then query the root handle. Creating/importing geometry does not currently register its producing authority's claim before return, so sibling close's merged-empty kernel.retain can dispose that still-live root-created handle before output retention publication. Linked operator creation through with_kernel has the same unclaimed-output gap. Creation/import must retain produced handles atomically under its authority or carry an explicit unpublished-operation lifetime.

## Creation Claim Repair And Composition Follow-Up

Latest source now claims successful wire response handles and deconstruct arrays within the shared claims transaction. Direct imports claim their returned handles, and with_kernel holds claims before the kernel write, captures Brep::live_handles before and after the operation, and claims newly registered handles before releasing the transaction. Raw kernel/cache access is private. This addresses the sequential unused-port-close gap in source; the owner added that scenario and root volume preservation to the native lifecycle law. No independent runtime execution was performed here.

One actual composition question remains pending owner handoff: a repository search of S and generic Flow finds with_geometry_port only in its two builder definitions and the RecordingPort test. Generic wasm tessellate/dispose require domain.host.geometry_port(), but no production supplied port construction is currently present in that searched scope. CAD's explicit BrowserSession bypass is separate evidence and does not establish injection into a production FlowHost or FlowEvalSession. The owner was asked to identify or complete the intended first-party host composition. This is an integration-in-flight observation, not a claim that the native authority laws fail.
