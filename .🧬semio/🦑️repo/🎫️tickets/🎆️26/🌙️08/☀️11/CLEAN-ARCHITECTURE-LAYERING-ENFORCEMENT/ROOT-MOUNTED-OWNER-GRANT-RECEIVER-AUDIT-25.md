# Mounted Owner Grant Receiver Audit 25

## Concise Summary

The observed mounted FEM receiving chain has no caller-owned RetainedCloneGrant parameter. Public GenericPlugin ArtifactApp, ArtifactEditor and ArtifactViewer mounted hooks transport only an instance identifier and two scalar ceilings. Their wrappers preserve that shape. The neutral MeshJob close receiver now requires a typed grant and returns full typed progress, while the actual 2D session still calls its former scalar receiver. A coherent typed public mounted-job port must reach the actual host scheduling owner before this chain can meet the new contract. This is an exact source observation, not a compile or runtime result.

## Evidence and Receiver Chain

GenericPlugin source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`.

- ArtifactApp lines14676/14680, ArtifactEditor lines39181/39185, ArtifactViewer lines39946/39950: `fn mounted_job_maintenance_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>` and the same shape for `mounted_job_close_step`. Default bodies return Complete.
- EditorApp lines40654/40657 and ViewerApp lines41072/41075 forward the same parameters unchanged. Viewer adaptation at9230/9234 also forwards the scalar hooks.
- VcsArtifactApp maintenance_stage_step at34881 selects live_runtime_instance_id at stage15 and calls `A::mounted_job_maintenance_step(instance_id, maximum_items.min(1), maximum_bytes)` at34911. Its maintenance_step at35964 drives this stage cursor at36016/36027.
- VcsArtifactApp close_step at35334 calls `A::mounted_job_close_step(instance_id, maximum_items.min(1), maximum_bytes)` at35414. Complete is followed by the exact mounted_jobs_terminal_is_empty witness; false completion yields the explicit interactive-job.close-mounted-false-terminal Fault.
- PluginApp runtime interface at15665/15671 itself accepts scalar maximum_items/maximum_bytes for close/maintenance. It exposes separate scalar next_close_byte_demand and next_maintenance_byte_demand. These scalar demand queries do not carry independent authorization for copy, capacity or depth.
- AppRenderOperationContext at9793 contains app_instance_id, base_revision, generation, canonical_base_revision only. The prepare_snapshot_read hooks receive this revision identity plus snapshot. This context binds work identity; it does not contain a retained clone grant or cleanup control.
- PluginCloseStep at15635 contains Pending released_items/released_bytes, AwaitingInput reason, Blocked reason and Complete. It cannot represent full copied/capacity/depth retained progress without a public contract change.

Specific override files:

1. `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs`851/855 forwards scalar maintenance/close to crate::editor::fem2d::session.
2. `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs`906/910 forwards scalar hooks to crate::live_visual.
3. The matching 3D viewer `👁️viewer/🦀️.rs`65/69 forwards the same scalar hooks to crate::live_visual.

2D actual session: `.../◻️2d/.../✏️editor/🧵️session/🦀️.rs`.

- Public maintenance_step2493 calls retire_one; public close_step2497 handles instance pending/current retirement then drives that same registry cleanup.
- retire_one2457 selects an exact instance-owned retiring shell, checks mutable borrow, calls MountedState.close_step(maximum_items.min(1), maximum_bytes)2476, validates terminal_is_empty before returning shell/credit, and reports Blocked while the worker owns its shell.
- MountedState.close_step1490 returns zero progress on zero maximum_items; otherwise it invokes real self.cancel.cancel_now(), marks Closing, then drains its explicit close cursor. Its retained mesh at930 is Option<MeshJob>; mesh close call at1522 is `mesh.close_step(maximum_bytes)` with tuple destructuring.
- Working execution receives reactor JobBudget through BoundedJob step1152/2184 and constructs remaining fuel/deadline budget2201. A fuel/deadline execution budget is not an independent retained ownership grant.

3D actual numerical/live visual session: `.../🧊️3d/.../✏️editor/🧵️session/🦀️.rs`.

- Public maintenance_step3852 and close_step3868 receive the same scalars; retained state close helper3029 accepts maximum_bytes only.
- Numerical owner close1775 accepts maximum_bytes and delegates to individual children. Observed interactive children at1880/1896 call close_step(1, maximum_bytes). These are candidate outdated receiver sites in a concurrently advancing source, not compiler-certified exhaustiveness.
- Existing bounded numerical execution steps2953/3450 carry JobBudget. They do not populate mounted cleanup hooks with an externally granted typed grant.

Mesh source: `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs`.

- Job typed imports8; InteractiveJob close2324 forwards a RetainedCloneGrant.
- MeshJob.close_step2770 queries retirement_demand, refuses insufficient depth, yields on insufficient copy/release, executes retire_owner_turn with granted release limit, reports RetainedCloneProgress and calls step.admit(grant, terminal_is_empty()). Thus required dimensions cannot be reconstructed from maximum_bytes without invented authority.
- Existing GeneralJob source `🧰️framework/🔨️modules/🧵️job/🦀️.rs` reexports grant/progress43; close receivers178/1320 accept typed grant; InteractiveJobCloseStep1281/1283 carries typed progress and admit1291 validates it. This typed vocabulary already exists for transport but is not carried by the observed mounted hook chain.

## Minimum Coherent Public Port

Define the mounted maintenance/close request and result schema before language bindings: caller-owned RetainedCloneGrant with separate item, copy, capacity, release and depth authority; complete RetainedCloneProgress; explicit refused/blocked/awaiting states; exact terminal witness. Demand remains descriptive RetirementDemand and never manufactures authorization. Introduce the typed port at the actual host scheduling authority, thread it through PluginApp, VcsArtifactApp mounted stage, ArtifactApp/Editor/Viewer wrappers and Specific sessions to MeshJob unchanged except legitimate caller-approved narrowing. Return exact consumed progress through each wrapper; do not compress it into released_items/released_bytes. A separate mounted port can isolate this requirement from unrelated hundreds of scalar artifact owners, but the caller must genuinely supply it.

Retain real cancellation and worker ownership blocking. Demand/control is not a default grant, a scalar alias, a capacity=max(bytes) conversion, a constant depth or a demand-funded allocation. No no-op cancellation or progress implementation is acceptable.

## Original Law and Validation Obligations

Portable and native implementations must share a schema-first language-agnostic law corpus. Preserve the original binary64 geometry algorithms and original fixtures/cases. Require exact unchanged geometry/orientation/constraints/holes/triangulation and original cancellation/revision publication behavior, alongside independent zero/short copy, capacity, release and depth grant cases; refusal retains owner/cursor and consumed progress never exceeds each granted dimension; terminal completion proves all nested shells empty. Native allocations and retained capacity must be charged at original owner transitions, including nested vectors, displaced shells and registry retirement. Compare original expected geometry output against an existing independent third-party oracle in tests; do not replace original laws with a handpicked easy family. Exercise actual mounted receiving call paths in both bindings and record real cancellation/progress/runtime evidence. This audit ran no compiler or runtime tests and makes no whole OS or 3D runtime claim.

## Receipt and Scope

Complete source bodies for the seven explicit relevant files were captured twice under before/after names in `🗑️generated/receiver-audit-25/`; an additional observed GeneralJob body was captured. SHA-256 hashes, paths, byte sizes and body names are in receipt.json. The controlled snapshot limit was64 MiB/65536 files/30 seconds for explicit input captures; seven twice-captured files consumed7,667,286 bytes before the Job capture. Before/after captures preserve concurrent advances if present; equal hashes establish only equality at these observations. No archive raw guard or receiving-test40 archive was opened or hashed. No source, AGENTS, Git state or runtime gate was changed. The enclosing active ticket owner must remove generated receipts at ticket completion while retaining this report.
