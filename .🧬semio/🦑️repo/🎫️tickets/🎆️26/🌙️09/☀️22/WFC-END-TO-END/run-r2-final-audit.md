# Final Source Binding And Ownership Audit

Read-only current-source review2026-11-04; no build queues or source edits.

## Grid3d

Reviewed editor root, new instance transient, fill completion and preview. CommitFill is present in enum, command bridge, command_id, retained tool roster, bounded proof roster, hidden preview action and Transient-only publication contract. Factory obtains request.port, emits completion callback and retains the result via CompleteWithEphemeral. Root installs bounded transient preparation, root retirement and store disposer. New transient SnapshotProjection uses current RetainedWorkOutcome signature.

Both original run identity and decoded live payload source are checked. Cached payload must be done and source-bound. Mounted render reads transient.snapshot and doc.render_operation_ref; absent/stale state paints an empty assignment rather than starting inference. RESIDENCY/LAST_FILL/cold solve imports are removed from editor preview. No remaining old residency APIs/callers found in current production/test Rust scan. Per-app store ownership provides isolation without window-id global collision.

Initial audit found old65,536byte command envelope inconsistent with full payload callbacks. An independent serialization sample of4096 air assignments produced148113byte payload and189101byte escaped payloadJson envelope, so the old limit demonstrably rejects a reasonable16³ controls case. Sent to native owner; owner is raising raw/output to1MiB and adding envelope regression, bounded against existing262KiB tick transport. Re-review final constants after that edit; not asserted fixed merely from intent.

## Graph2d And Graph3d

Render adapters use actual render_operation_ref canonical revision. render_body filters transient source and live run identity before projection. Preview payload decoder additionally binds payload source to run identity. Fill publication/finish stamps source_revision from original self.identity.base_revision, and SetSolve carries that revision to app-local transient. Same-ID/same-extent edits therefore hide prior cache; dismiss retains a matching completed transient.

No new roster/action is introduced for graph freshness: existing internal CommitFill and Transient lane remain. Source field was added across native transient snapshot, SetSolve/inverse/diff, payloads and authored fixtures. Current callsite scan found updated render_body argument count for both graph editors and tests. Helper payload_from_commit intentionally starts unbound and live job stamps it before publication; it must not be mistaken for a ready source-bound cache.

Both graph commands retain existing65,536byte raw and262,144byte output envelopes. Owner confirmed this is unchanged; full large ID/slot payloads can exceed it independently of the new revision field. This limits larger browser-control setup, not proof that legal schema maxima are supported. Do not infer comprehensive graph-scale acceptance from small default passes.

## Validation Scope

Static signature/ownership and callsite review only. No native pass asserted. Native owners are executing parser/reference checks separately. Strong remaining runtime evidence includes mounted cache/live freshness, completion callback delivery, two-app isolation, dismiss retention and observable pause/cancel with bounded setup.

## Final Grid3d Envelope Recheck

Stable reread confirms Grid3d raw constant now1,048,576, execution contract output1,048,576 and proof raw/output1,048,576. Producer/factory use the shared raw constant. Editor preview still has no RESIDENCY/LAST_FILL/cold solve remnants. This resolves the specific grid3d64KiB callback ceiling finding by source inspection; native receipt/admission tests remain separate.

Grid2d constants were still65,536/262,144 at the instant of this read while parent was authoring that change; do not treat this timestamp as final parent implementation state.

## Grid2d Final Tick Publication Fence

At audit time grid2d finish_success632-647 dispatches commit_effect before publish_tick; oversized final tick can therefore queue a cache despite faulting the visible run. publish_tick590-623 returns FillRun::Staged on both outcomes. Its reliable success witness is self.settled=Some(FillSettled::Complete), installed only after writer.finish/encode success. Failure clears settled and creates Fault publication.

Canonical local graph reference is graph2d fill631-638: publish final tick first, then dispatch only when settled is Complete. Graph failure625 names wfc2d-fill-payload-capacity. Root advised equivalent grid2d gating and named capacity fault.

Grid2d child unit module has identity() and close(job), imports fill::* and can invoke private finish_success directly. Use cloned ToolRunJobPort to inspect pop_effect and ensure rejected oversized finish leaves no effect/committed flag. Grid2dFillRunJob::new(identity(),Arc::new(pipes::document()),port.clone()) is the current constructor. Payload size must actually cross262144 encoded tick limit;100 ordinary short tile IDs cannot do so. Generate a measured bounded oversized vector rather than assuming a row count. No native queue started by audit.

## Browser Readiness Selector Recheck

Reviewed corrected scenario focus, Pause state and Complete abort branches against actual public DOM. activateWindow uses the correct Canvas dock wrapper data-slot=mode-dock-tab/data-window-id and child button role=tab; actual component Canvas/⚓️dock/🟦️.tsx119-145 publishes data-active and onSelectTab. Paused/Pausiert, ready completion and abort labels match framework. Five public action IDs and preview surface IDs remain aligned; new grid3d commit-fill is hidden and does not collide with the author form list.

Concrete remaining helper issue: runPanel blindly clicks a tab, which toggles an already-open panel shut. Shared chrome closePanels111-115 itself closes panels by clicking their tabs. The framework already exports idempotent openTab(id) from testing/chrome88-106; it checks actual visible panel bodies before clicking and waits for the requested body. Root advised import openTab and use openTab("framework.panel.toolRun") in runPanel. No new browser queue started; generated reports remain pending parent execution.

## Dependency Phase Cycling Versus Ticket Reports

Read-only infrastructure source trace: repo dependencies/💾️storage/🟦️.ts147-162 captures package manifests, frozen lock, scope configuration and declared patches for generation hash. Arbitrary ticket Markdown is not captured. Producer632-635 repeatedly fences store, workspace custody, sources and material descriptors. current-input614 hashes protected material files chunkwise; workspace-edges438 walks produced dependency entries. Repeated phase names alone do not imply restarts from ticket reports.

Actual retry is explicitly logged as Store dependency sync retrying over the delta (658). Current browser-r2/native-r2/final-native scans contain no such retry/incomplete markers. Historical bitmap-native log names a real unrelated draw TypeScript package.json change as retry cause. No evidence ticket audit reports invalidate the dependency producer.

## Departed Dependency Builder Diagnostics API Limit

Queried existing local ticket MCP client list separately for log,history,event,receipt,status. The first four filters return no tools. Status exposes cache_status,capacity_status,fleet_status,leases_status,remote_cache_status,remote_execution_status,service_status,session_status. Their owned schemas describe current ledger/capacity/leases/process/session holdings; none offers a departed dependency-build outcome or last-failure history argument. Therefore these API metadata do not establish why previous dependency builders departed. No extra build, restart, mutation or raw cache inspection performed. Continue observing the original run; use explicit retained process diagnostics if produced.

## Narrow Cancellation Interface Discovery

Read-only MCP tool metadata filters cancel/stop/terminate return none. session_close(force) exists but closes the entire agent session and its temp/sandbox ownership; it is not a one-PID job cancel and is unsuitable while another WFC native task uses that session. repo run --help and repo --help expose no single-running-job cancel command. Originating terminal interruption is the normal cancellation channel when its owner exposes it; non-TTY stdin control bytes must not be assumed equivalent to a console signal. No cancellation executed by this agent, and no broad session close recommended.

## Completed Payload Trace Precedence Defect

Confirmed current ToolRunState::Complete is nonterminal: framework tool-run lifecycle/🦀️.rs53-55 terminal set contains only Finalized,Aborted,Faulted. Hence all live fill helpers still select a ready-to-finalize Complete payload. Final tick publishers clone search history trace into completed payloads.

Four concrete renderers override authoritative finished data with that history:
- Grid2d preview commit_with_trace261-266 rewrites final assignment for each trace event without done check.
- Graph2d preview_layers_json208 chooses last trace tile before completed assignment.
- Graph3d render_body377-380 replaces assignments with each trace tile.
- Bitmap output layers_from_fill_payload126-134 overwrites completed pixels and adds discarded markers.

A valid done payload with final assignment A and a conflicting discarded history event B therefore paints B before Finalize, then changes on cache fallback/dismiss. Grid3d preview72 already avoids this via !payload.done around trace overlay. Root and bitmap/graph owner notified; guard only the thinking overlay on non-done payloads and retain partial progress visualization. No production source changed by audit.

## Completed Trace Regression Review

Read-only review of latest shared four-case consumers found no additional defect. Both graph tests construct matching32byte payload.source_revision, ToolRunIdentity.base_revision and explicit current render revision; Running/Complete states are asserted nonterminal. Actual render_body receives a live payload and is compared to a separately constructed fixture-expected transient through the complete preview tree. This is not an unbound/empty equality: graph2d layer names include selected tile ID, and graph3d instance labels include selected tile ID even for placeholder meshes.

Fixture independently chooses assigned0,traced1,expected1 for partial and expected0 for done, across observed/discarded variants. Grid2d likewise drives paint_commit through live ToolRunView with matching revision and asserts the exact nonempty assignment vector. Runtime done guards preserve partial overlay and stop completed history overwrite. Bitmap remains deliberately held until current admitted native source settles. No source edit or gate launched.

## Coupled Core Native Coverage

Latest source-only suite mapping is recorded in [Native Verification](run-r2-native.md): after the neutral headless gate passes, the single WFC gate must explicitly select engine-job, engine-oracle, engine-solve and engine library tests. Consumers do not execute dependency crates' unit laws. This covers publication/checkpoint, four driver fixture cases, three-turn Search supervision and facade grid driving. The exact four-package registered command is retained there. No such core gate or browser journey has passed yet.
