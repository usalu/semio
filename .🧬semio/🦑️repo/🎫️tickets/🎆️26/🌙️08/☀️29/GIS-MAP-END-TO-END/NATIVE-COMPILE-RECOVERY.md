# Native GIS2D Host Compile Recovery

## Prior Host Attempt

The earlier native GIS2D owner completed two Cargo attempts that reported E0425 for `semio-renderer-wgpu-shell-actions` and E0599 for `semio-plugin-instance-app`. The first and second compiler failures preceded a new source-admission pass. The third attempt became the terminal owned-command failure with exit code 1 and no signal.

Line 33459 of the earlier launch log identifies the third attempt's bounded Cargo diagnostic file as `.cache/b/r/m/u/l/qe7b736/semio-cargo-logs/cargo-artifacts-BK8Wpm.log`. Cargo elided 121,143 diagnostic lines and the replayed tail contained warnings and the timing report. The original lane directory was absent when recovery started, so the specific compiler error could not be recovered from that attempt. The retained launch log alone does not identify a current Rust defect.

## Current Host Attempt

The managed root command is `SEMIO_RUNTIME_DIAGNOSTICS=1 SEMIO_LANGUAGE=en repo ./📜️script.ts dev playground gis2d native prototype`. It published one complete native runtime component and then started the owned native host build. The staged native `gis/📦️component.wasm` is 138,519,858 bytes and has SHA-256 `125c04c15010f7bb6d961cfb0eed61f1bf887c3977b58bc985c05096bc26621f`. The digest matches the shared GIS component exercised by the browser.

A read-only monitor captures the new bounded Cargo logs from the managed lane into the ticket's generated directory while Cargo writes them. It keeps the files open for reading so an ordinary lane retirement cannot remove the diagnostic evidence before inspection.

The pinned Node native launcher fixture passed on the current sources. It verified loopback readiness before child launch, forwarding through both GIS raster and vector routes, and listener teardown. The fixture does not start the compiled GIS window.

## Reproduced Compiler Failure

The retry captured Rust E0425 in the owned diagnostic stream, at line 121719 of the ticket copy `🗑️generated/native-gis2d-cargo-diagnostics/qe7b736-cargo-artifacts-0jFy8o.log`. The compiler reported `S_SPACE_INDEX_DOCUMENT_SCHEMA` out of scope in `semio-renderer-wgpu-shell-actions`, at `🔄️backbone-and-check-in/🦀️.rs:247`. The earlier E0599 package, `semio-plugin-instance-app`, had emitted its completed Cargo artifact in this attempt.

The selected program's `os.space-index` topic supplies a `SpaceIndexContribution` with a string `document_schema`. The Space plugin's declaration contributes `s.space`, and the WGPU foreground path already opens its document with the contributed value. The background check-in path also already used that value to admit its bindings, then referenced the removed shell constant when opening the actor. The fix moves the same contributed field into the actor configuration, preserving the schema selected at admission.

The failing Rust diagnostic preceded a typed Cargo source-plan drift and the Cargo owner automatically entered a new source-admission attempt under the existing launcher. The fallback follows that owned attempt before considering a new process.

A second source-plan drift exhausted the first child’s local attempts. The root launch reissued the managed session, republished the GIS native runtime, and started a fresh owned Cargo host build in the same native lane. The monitor retained its diagnostic stream separately from the reproduced failure.

The reissued compiler stream reported E0432 for the MostGeneralDialect and most_general_dialect imports in semio-plugin-runtime-ops-document. The live artifact-reference producer already publicly reexported both symbols from its selection module. Those producer files had newer modification times than the attempted source plan. The owner subsequently reported typed plan drift again and reissued the launch through the normal recovery path. The recovery made no change to the dialect selector.

The next owned Cargo attempt emitted completed artifacts for both semio-framework-artifact-reference and semio-plugin-runtime-ops-document, passing the E0432 point after reissue.

The fresh native build compiled semio-renderer-wgpu-shell-actions with the contributed-schema fix. Its compiler artifact event is at line 47049 of the recovery launch log and reports fresh=false, confirming Cargo compiled the selected shell crate in this attempt. The binary publish and live interaction checks were still pending at that milestone.

## DWG Source Drift

After compiling WGPU shell actions, Cargo reported E0432 for DwgDrawingExtents and DwgEntityProjection and E0599 for DwgLogicalLayer::to_native in semio-s-artifact-stdio-dwg-read. The captured compiler lines are 127376, 127382, and 127402 in the EjCIMO diagnostic file. The live reader and all three producer changes have the same 19:58:25 UTC modification time. The live public geometry and snapshot modules declare the types and method the reader now uses. No DWG source was modified for native recovery; the checked owner rechecks the failed source plan.

The native recovery launch log recorded the typed failed-plan drift at line 52371. The root settled the old native host child, republished the runtime and reissued compilation at line 52651. The separately retained ePPnwZ Cargo file records that reissue.

## Public Kernel Facade Export

The reissued Cargo build stopped at E0422 in semio-plugin-host-wit. Line 94539 of the ePPnwZ diagnostic records that the WIT conversion could not find PluginDocumentTarget under semio_framework::kernel. The mounted kernel producer declares the owned type and the Effect document field. The framework facade exposes individual kernel items and omitted this one. The two existing native WIT conversions and the wire-effect round-trip fixture already consume the facade path. The fix explicitly reexports the producer type alongside the other kernel items. The next native owner attempt must verify the WIT crate and publish the host.

The inner retry then reported E0433, E0425 and E0422 in semio-plugin-interpreter-runtime for the two poll-limit types. Those diagnostics begin at line 94022 of the cjbegU capture. The kernel-identity producer defines both public types and its Cargo manifest mounts the defining module directly. The consumer expected those types at semio_framework::kernel, whose explicit identity export list omitted them. The same facade now explicitly reexports OwnedCorePollLimits and PollRequestLimits from the owning crate. The patch preserves the previously added PluginDocumentTarget export.

The U1zRfD retry compiled the framework facade, native WIT host, and interpreter runtime with both facade corrections. The recovery log records their fresh=false artifacts at lines 84799, 84939, and 84949 respectively.

## Native Tree-Level Drag MIME

After compiling both facade consumers, the U1zRfD build reported E0425 at line 99495 because the new native WGPU row-drop route referred to TREE_ROW_REORDER_MIME in ui_contract. The reader changed at 20:45:02 UTC. The Rust component contract had no declaration for the payload, while the TypeScript interpreter and the positional-drop WGPU fixture already used application/x-semio-tree-row-reorder. The Rust component contract now defines that exact same MIME next to its tree props. Its existing public export reaches the native events crate without a dependency or wire change.

The previous native root reached its checked retry boundary and terminated with ordinary exit status 1. The terminal child recorded four source-plan attempts and the parent propagated the typed input change. The native recovery log records both errors at lines 87786 and 87803. No native host binary or window had been published. A new governed launch then started from the live workspace with the source fixes and the updated early admission handling.

## Fresh Native Launch

The fresh managed root entered the updated bootstrap and the component preparation phase. Its active stdio preparation child resolved and captured owned package recipes for semio-hub-stdio-component. A separate governed stdio compiler was already preparing the same model. At this point no duplicate Cargo compile or native host compile had started under the new root.

The native launch routing fixture passed again on the live launcher. It confirmed loopback readiness before invoking the child, raster and vector forwarding, and listener teardown.

