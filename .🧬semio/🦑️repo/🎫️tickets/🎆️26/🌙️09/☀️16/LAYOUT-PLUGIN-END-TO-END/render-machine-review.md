# Mounted Tool Machine Render Review

2026-10-28. Read-only diagnosis of gate16 eleven errors in SDK instance/render/staging/tool-machine. No implementation, tests or builds. Source is concurrently being repaired by Rust; conclusions describe the source read before a ready signal.

## Live Paths Must Be Preserved

follow_tool_machines has real typing, history, action, gesture and settle callers. settle_press is invoked by settle_tool_operation and actual command-routes/actions, so deleting either whole facade would remove live UI behavior. abandon_window_preview has only its definition in bounded instance scan, but that alone is insufficient to characterize every dynamic/public consumer; its backing field no longer exists.

Actual PluginApp::render forwards into instance/plugin-app-impl/render-surface. That renderer refreshes cache, selects tool_machines.overlay_or(snapshot), config_overlay_or(config), captures the addressed Window config and reads window_config_store.preview_or(authority.snapshot). These are concrete live provisional display consumers. Old follow_tool_machines still attempts to clone provisional WindowConfigMutation leaves and call removed window_preview.request. Old retirement pending/demand/step also references removed field. Merely dropping these calls is not evidence of semantic replacement.

## Genuine Replacement Exists But Wiring Not Established

plugin/tool-machine/follow defines OriginalToolMachineFollow. It captures original caller+native decoder, pending request and actual Store/ledger frontiers. It walks genuine retained ledger rows, projects WindowConfigMutation without clone into original retained input, drives WindowConfigOwnerRegistry preview_input/preview_leaf_step and OriginalWindowPreviewEpoch, and publishes a complete epoch. Window registry preview_or selects published visible epoch; epoch tests cover complete pair and cancellation of private pair. This is the correct native mechanism, not a reason to revive cloned previews.

A bounded production Rust call scan under plugin found OriginalToolMachineFollow only at definition/reexport; no mounted admission/advance/retirement owner/caller was established. refold_request appeared at runtime initialization and follower logic, not live request producer. This is a concrete wiring evidence gap relative to actual renderer consumption. Before removing old facade window path, wire actual mounted source/maintenance/retirement to the genuine follower or identify a supported external live caller. Reported immediately to Rust/root. No conclusion that current epoch is already connected can be supported by symbol presence alone.

## Scope Limit

Rust is inspecting genuine send_original for non-Clone PressLeaf and owns repairs. This review does not claim whole-facade deletion safe, compiler success, actual rendered preview success or regression-test execution.

## Exact Genuine Mounted Integration Seam

Reactor runtime/reactor-lifecycle/cleanup-pumps lines137-142 receives the actual cleanup job StepContext, admits MountedOwnerTurnV1 with its operation/generation and calls mounted_owner_maintenance_step. Instance/plugin-app-impl lines150-153 validates live instance/phase and turn.advance_lifecycle lends the same cx and loan to maintenance_step. Instance/staging/maintenance-ladder passes that same context through original_lifecycle_step into LadderCtx::maintenance_unit_step. Existing MaintenanceUnit::ToolOverlay in plugin/maintenance-ladder currently forwards only grant into tool_overlay_retirement_step. This is the precise integration point to forward genuine cx and derive actual retained delta.

RenderCtx/LadderCtx already borrow PendingReserved<A>, Store/config/window registry and ToolMachineRuntime. OriginalToolMachineFollow is A-generic while ToolMachineRuntime carries four projection types only; an A-generic instance pending owner slot can retain Option<Box<OriginalToolMachineFollow<A>>> without inventing a replacement context or forcing payload copies into runtime. The slot must participate in maintenance pending/demand, close rung and terminal witnesses. Constructor/admission frame and final shell release are funded; no implicit drop of a nonterminal follower.

Existing live follow_tool_machines requests should mark refold_owed and checked-increment refold_request rather than execute cold folds. Admission captures actual request, ledger revisions, Store generations/full contents and original receiving identity. Use admission_demands/admit for empty pending slot; demands/advance for current work; close_demands/close_step for explicit cancellation/instance close, followed by shell_close_demands and funded slot removal after terminal_is_empty. advance also routes changed frontier, publication or conflict into closure and leaves refold_owed for later readmission. Keep actual box and all children on every yielded/refused turn. demands has no cx and does not detect cancellation, so caller selecting a cancellation turn must quote close_demands instead of ordinary active-phase demand.

Both advance and close validate original SourceBinding and native decoder identity; maintenance and close cannot substitute a freshly fabricated caller or rebind the existing follower to another close operation. If normal close uses another source, retain/return through the original admitted donor before releasing it, using existing owned handoff machinery. The exact cleanup-job lifetime across close was not independently established here, so source identity preservation remains an integration obligation.

The current real renderer already consumes Window registry preview_or at visible epoch. OriginalToolMachineFollow projects genuine Window mutations, finishes private candidate previews, validates/publishes complete epoch together with document/config candidates and increments provisional_generation. Thus no renderer replacement is needed; mounted source owner and turn plumbing must connect this existing producer to that existing consumer. APIs and cancellation/source-lifetime constraints sent to Rust and Medium. No integration edit or execution claimed.
