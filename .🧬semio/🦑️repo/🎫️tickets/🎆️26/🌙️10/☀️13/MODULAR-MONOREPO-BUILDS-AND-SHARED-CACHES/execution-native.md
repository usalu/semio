# Native Build Execution

Native producers consume the strict prototype/release mode and publish mode-qualified artifacts. The central .cargo/config.toml owns native prototype/release profiles; authored Cargo manifests retain WASM tuning. Prototype preserves incremental intermediates and avoids global optimization; release retains optimized profiles and thin LTO. Compiler profile dev/test identifiers remain Cargo compiler identifiers.

Native Cargo artifacts are captured privately from compiler messages, then atomically published to dist/build/<mode>. Native Cargo/WASM intermediates remain shared beneath the existing artifact state store and profile leases. WASM packages publish to dist/wasm/<mode>; package metadata semioWasmOwner records the exact workspace-relative source owner, selected mode, and canonical output. Existing package entrypoint projections remain leased active views and are excluded from task cache authority.

The neutral compiler capacity owner grants an exclusive all-core host lease by default, with explicit bounded one-worker shared claims for controlled single-thread compilers. Inherited children require an explicit positive admitted worker count. Pipeline jobs declare jobserver or reserved worker protocols: Cargo borrows the existing jobserver pool; non-jobserver compilers reserve actual tokens and receive that worker count. Current flow/surface/puzzle WASM jobs reserve the whole pool exclusively because wasm-pack may run unmanaged optimization and subsequent bundles. Component wasm-opt stages reserve their explicit budget. Cargo/WASM without GNU jobserver flags set CARGO_BUILD_JOBS to their admitted count; a real build-script NUM_JOBS fixture caught10workers for an admitted1 before the fix. Existing cancellation, progress, queue admission, artifact ownership, resource provenance, and atomic publication remain in force.

CMake configure/build/test lives in the canonical dedicated library/🏗️build/🛠️cmake/📜️script.ts owner. Root commands load it lazily, and root Nx targets invoke it directly. Presets explicitly pair all three platforms with prototype Debug and release Release. Configure and build share the same state lease, toolchain/configuration/architecture fingerprint, and admitted CPU count. Windows ARM64/x64/Win32 mapping is explicit.

.NET Debug/Release and Python wheel packaging publish separate dist/build/<mode> directories. .NET uses the admitted MSBuild CPU count. Python wheels carry the same authored token contract in either mode. Go builds use prototype -gcflags=all=-N -l and release -trimpath/-ldflags=-s -w, private capture and atomic publication. Go binaries use a dedicated directory per mode and owner, avoiding sibling deletion or ownership collisions. Rust CLI/MCP bootstrap runs use explicit prototype artifacts; WGPU native profiles map to selected compiler modes.

Cargo owner synchronization now publishes a content receipt after current source/manifest/member/recipe custody admission and the locked update/fetch pair. The receipt binds Cargo identity, configuration, relevant compiler environment, local manifests/member rosters, preparation code/schema, observed recipe inputs/outputs, and final Cargo.lock bytes. An unchanged receipt skips Cargo update and fetch. A manifest or lock edit reruns both commands. Selected preparation admits nearest native owners only from its actual local dependency manifests. It preserves source-declared unrelated exclusions during selected membership publication; whole-repository setup remains the owner of global membership reconciliation. An unrelated invalid owner is covered by a fixture and no longer forces a selected compiler to discover/read every workspace. This never substitutes old authority for fresh preparation observation.

## Validation

All commands used Bun and canonical Nx targets. Generated log names below are temporary evidence and are removed when the parent closes the ticket.

| Verification | Observed result | Evidence |
| --- | --- | --- |
| Initial mode policy/profile/preset tests before implementation | 0 pass, 3 fail | native-red.log |
| Synchronization unchanged input before implementation | 7 pass, 1 fail; expected0 dependency commands, observed2 | cargo-sync-red.log |
| Final native mode/compiler/storage/worker suite | 10 pass, 0 fail | native-budget-green.log |
| Full Cargo preparation/membership/closure/discovery/cancellation regression | 32 pass, 0 fail | cargo-preparation-complete.log |
| Final native artifact and pipeline suite | 79 pass, 1 skip, 0 fail | native-budget-green.log |
| WASM policy, mode publication, owner metadata, queue/cancellation suite | 10 pass, 0 fail; task succeeds after partial-budget patch | native-acceptance-green.log and native-budget-final.log |
| Go mode/schema/compiler/runtime suite | 2 pass, 0 fail | go-mode-green.log and native-acceptance-green.log |
| Managed prototype reflection/Python wheel runtime | 1 pass, 0 fail; both emit #ff344f | native-acceptance-green.log |
| Managed release reflection/Python wheel runtime | 1 pass, 0 fail; both emit #ff344f | managed-runtime-release.log |
| Actual Go repo MCP prototype build | Nx success, dedicated canonical output | repo-mcp-prototype.log |
| Actual canonical Go MCP initialization | protocol2024-11-05; server repo1.0.0; exit0 | repo-mcp-runtime.log |
| Actual extracted CMake release target | Nx success | cmake-owner-release-final.log |

The native mode suite compares schema admission with Ajv, executes CMake/Ninja in both configurations, runs both native Cargo binaries, observes warm Cargo fresh=true receipts, and changes Rust source before rerunning the binary. One recorded fixture showed prototype1151ms cold/120ms warm/386ms edited and release3109ms cold/114ms warm/2453ms edited. Both edited executables emitted cargo-mode-edited-runtime-ok. CMake warm builds reported no work to do.

The synchronization test uses actual Cargo update/fetch and locked Cargo metadata as the independent compiler oracle. It observes dependency command counts initial2, unchanged0, manifest edit2, lock edit2 and unrelated owner change2, and confirms updated package version0.2.0. Language-agnostic JSON vectors author all expected mode/profile/configuration, capacity, runtime token, and synchronization results.

The reserved-worker fixture now emits NUM_JOBS=1 and agrees with an independent Cargo -j1 runtime oracle. Cancellation of an active compiler is triggered after its observed child-start record, preventing a busy host from confusing queued cancellation with active child-tree cancellation.

The final artifact suite includes actual Cargo build-script resource provenance, durable receipts, relocation/ownership/cancellation behavior, concurrent pipeline workers, and the parent-provided reserved4workers/shared-successor/Cargo jobserver runtime. The one skip is the GNU make ≥4.4 FIFO-jobserver interoperability test because that GNU make version is unavailable on this host.

All executed compiler/runtime tests ran on native macOS ARM64. Windows/Linux presets and architecture policy were checked structurally, not executed on those hosts. Real Rust/WASM production package compilation and Windows/Linux runtime verification remain separate platform/integration work; the WASM suite verifies the actual driver boundary using controlled tool ports.

## Warm Owner Measurement

The real base64 target ran through Nx with --skip-nx-cache and --excludeTaskDependencies. The old full-roster execution recorded233scopes, discovery13.26s and complete preparation31.13s (base64-sync-cold.log). The selected closure recorded one admitted scope and two local packages; first receipt preparation1.81s and critical task18.8s (base64-scoped-cold.log). Its subsequent warm run recorded preparation1.47s, a current synchronization receipt, no Cargo update/fetch, compiler0.19s, critical task8.6s and total Nx9.2s (base64-scoped-warm.log). These are observed runs in a concurrently active workspace, not controlled benchmarks. The remaining selected-owner75-member custody/inventory and provenance work is visible in the measured task time; compile itself reuses shared intermediates.


Temporary route tracing observed outer Bun startup169ms, native owner policy153ms, inner Bun startup128ms, preparation launch5.47s (selected plan1.28s), and capture/publication0.99s including Cargo0.12s. The remaining owner cost primarily consisted of current75-member snapshots. One duplicate inventory traversal used for receipt input assembly was removed. Temporary route instrumentation was removed after diagnosis; existing opt-in Cargo preparation timing remains.


## Final Extended Acceptance

| Verification | Observed result | Evidence |
| --- | --- | --- |
| Bounded compiler capacity red/green | Before:0pass3fail; after:3pass0fail | capacity-partial-red-2.log,capacity-partial-green.log |
| Generic lease regression after fair shared cohort addition | Passed actual Bun/Node/Python SQLite races, FIFO/stale/cancelled tickets, path256 and nonce laws | capacity-lease-regression.log |
| Queued exclusive cancellation | Red:unhandled rejection1; finalgreen:no unhandled rejection, cancelled child terminated, successor admitted | queued-exclusive-red-2.log,driver-contracts-green.log |
| Unit-graph metadata cancellation | Red:actual metadata child remained alive; finalgreen:child stopped and successor admitted | driver-contracts-unit-red.log,driver-contracts-green.log |
| Final strict profile/driver cancellation contracts | 4pass0fail,48 assertions | driver-contracts-green.log |
| Go optimization plus neutral durability | 3pass0fail; actual rename+parent sync matches Python os.replace | go-durability-green.log |
| Actual CLI prototype/release | Both canonical builds passed, Nx8.9s/12.1s | cli-main-prototype.log,cli-main-release.log |
| Actual coordinator prototype/release | Both canonical builds passed, Nx6.6s/27.7s including release wait | coordinator-prototype.log,coordinator-release.log |
| Zero-touch six canonical owner/mode Go executables | 1pass0fail,30 assertions; actual metadata flags, CLI help, MCP initialization, isolated coordinator health | go-executable-acceptance.log |
| Actual root CMake prototype/release | Both canonical targets passed, Nx29.3s/4.8s | cmake-workspace-prototype.log,cmake-workspace-release.log |
| Config-only Cargo profiles | 1pass0fail; independent workspace has no native profile table; both runtime modes match and rustc threads=0 accepted; @iarna/toml oracle agrees | native-central-policy-green.log |
| Final native mode after central policy and explicit relay profile selection | 12pass0fail | native-relay-mode-green.log |
| Final native artifacts and WASM after central policy | Both Nx tasks succeeded; successful sibling output counts suppressed by Nx | native-central-final.log |

The capacity API is acquireCompilerCapacityLeaseV1({directory,signal,environment?,requestedParallelism?:1,onWait?}). Explicit1 enters a fair shared host queue cohort, then acquires one exclusive kernel-backed SQLite CPU token. At most host availableParallelism claims compute. A queued whole-host writer blocks late readers. Cancellation releases both claims; actual SIGKILL recovery sweeps stale owner identity and recovers the kernel lock. Only one-core partial requests are permitted, so claims cannot deadlock collecting multiple tokens. Default uncontrolled compilers retain full-host ownership. Generic artifact shared leases keep their existing behavior. Python wheels and controlled TypeScript phases use one-worker claims; native compilers use the shared agents/resource-leases directory.

Driver cleanup observes success and failure through then rather than creating a detached rejecting finally promise. Unit-graph execFile receives the original abort signal. Pass/job/pipeline schemas admit only prototype, release, wasm-prototype, wasm-release; unknown profile-to-mode mappings throw. CMake/.NET capacity and mutable-state waits now receive original invocation signals as well as process interrupts.

The CLI main module imports its existing first-party RunCLI implementation. Coordinator Unix/Windows durability implementations were moved into one neutral filesystem Go module; current coordinator and live root durability fragment import it. No source fallback or copied shim was introduced. Permanent executable acceptance receives the original process envelope and rebuilds all three owners in both modes through canonical owned APIs before asserting6coexisting binaries, optimizer metadata, CLI help, MCP2024-11-05 initialization and health=ok. Coordinator binds127.0.0.1:0 and writes only isolated ticket-generated service data.

The actual canonical CMake owner is workspace:cpp-build. Root CMakeLists declares C++23 flags and CTest but no library or executable. No other authored CMakeLists or cpp/cxx/cc implementation exists in the current repository. Thus actual root configure/build passes with no repository executable runtime to assert. The tiny owned CMake fixture supplies compiler/runtime proof for both modes.

## Central Native Cargo Policy

270 independent authored Cargo workspaces lacked prototype profiles. Native optimizer tables now live once in .cargo/config.toml, with mandatory repository compilerConfiguration ingress and explicit forwarding in native capture/relay, WASM, component passes and unit graphs. Neutral self-contained manifests may omit external configuration. Four top-level manifests keep specialized release trim-paths opt-ins because Cargo rejects that unstable property for a standalone manifest without its cargo-features declaration. Existing WASM tuning remains unchanged. Config content remains in Nx/provenance inputs.

The installed rustc1.99.0-nightly commit is9f36de775bc636c8e88c31a173c2bcb6995956a0. Pinned source confirms threads=0 selects host available CPUs and frontend workers acquire/release inherited jobserver tokens. Host/browser/WASI/Linux hints now use0. The actual independent Cargo fixture compiled both modes with0. [Pinned parser](https://raw.githubusercontent.com/rust-lang/rust/9f36de775bc636c8e88c31a173c2bcb6995956a0/compiler/rustc_session/src/options.rs), [pinned worker pool](https://raw.githubusercontent.com/rust-lang/rust/9f36de775bc636c8e88c31a173c2bcb6995956a0/compiler/rustc_interface/src/util.rs), [pinned jobserver](https://raw.githubusercontent.com/rust-lang/rust/9f36de775bc636c8e88c31a173c2bcb6995956a0/compiler/rustc_data_structures/src/jobserver.rs).

Two intermediate verification failures were source/test edit defects: a table name embedded in a hub manifest comment was left uncommented during profile extraction, and compilerConfiguration was inserted into the wrong fixture variable scope. Both were corrected; the final mode suite passes. Disk-full provisioning and concurrent source moves temporarily blocked separate launches; no passing result is inferred from those failures. All runtime proofs remain native macOS ARM64; Windows/Linux runtime is not verified on this host.

Managed runtime after original-signal/partial-wheel admission changes:1pass0fail, actual .NET and Python token #ff344f, Nx8.3s (managed-capacity-final.log).

## Permanent File Registration

These exact edited/created source-owner paths include shared files with parent/web edits. Cargo WASM profile manifests are individually enumerated.

```text
.cargo/config.toml
CMakePresets.json
Cargo.toml
go.work
✏️s/Cargo.toml
✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/Cargo.toml
✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/Cargo.toml
✏️s/🔌️plugins/🀄️wfc/Cargo.toml
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.toml
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/Cargo.toml
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/Cargo.toml
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/Cargo.toml
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/Cargo.toml
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.toml
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml
✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml
✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/Cargo.toml
✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.toml
✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/Cargo.toml
✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/Cargo.toml
✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/Cargo.toml
✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/Cargo.toml
✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/Cargo.toml
✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/Cargo.toml
✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/Cargo.toml
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/Cargo.toml
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml
✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/Cargo.toml
✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/Cargo.toml
✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/Cargo.toml
✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/Cargo.toml
✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.toml
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/Cargo.toml
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/Cargo.toml
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml
✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/Cargo.toml
✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/Cargo.toml
✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/Cargo.toml
✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/Cargo.toml
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/Cargo.toml
✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.toml
✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.toml
✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml
✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/Cargo.toml
✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/Cargo.toml
✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/Cargo.toml
✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/Cargo.toml
✏️s/🔌️plugins/📕️norm/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/Cargo.toml
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/Cargo.toml
✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/Cargo.toml
✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.toml
✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/Cargo.toml
✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/Cargo.toml
✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/Cargo.toml
✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/Cargo.toml
✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/Cargo.toml
✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/Cargo.toml
✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml
✏️s/🔌️plugins/🔱️trinity/Cargo.toml
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/Cargo.toml
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.toml
✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.toml
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.toml
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.toml
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/Cargo.toml
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/Cargo.toml
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.toml
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.toml
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.toml
✏️s/🔌️plugins/🧱️block/Cargo.toml
✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/Cargo.toml
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/Cargo.toml
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/Cargo.toml
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/Cargo.toml
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/Cargo.toml
✏️s/🔌️plugins/🪐️space/🫀️core/Cargo.toml
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/Cargo.toml
✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.toml
✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.toml
✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.toml
✏️s/🧑‍💻dev/🌊️flow/Cargo.toml
✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/Cargo.toml
✏️s/🧑‍💻dev/💡️services/Cargo.toml
✏️s/🧑‍💻dev/📐️cad/Cargo.toml
✏️s/🧑‍💻dev/🧩️composition/Cargo.toml
✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.toml
🌎️hub/Cargo.toml
🎓️teaching/Cargo.toml
🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts
📋️project.json
📜️script.ts
🧰️framework/Cargo.toml
🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🧫️fixtures/🔣️.json
🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧬️schema/🔣️.json
🧰️framework/🔨️modules/🏃️process/🏗️build/🧪️tests/🏗️managed/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🏗️build/🧪️tests/🏗️native/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🏗️build/🧪️tests/🧫️fixtures/🏗️native/🔣️.json
🧰️framework/🔨️modules/🏃️process/📋️project.json
🧰️framework/🔨️modules/🏃️process/📜️script.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🌊️relay/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧬️schema/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🧫️fixtures/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧬️schema/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/📦️publication/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🦀️cargo/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🧫️fixtures/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧬️schema/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🐹️go-build/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🐹️go-build/🧪️tests/🚀️executables/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🐹️go-build/🧪️tests/🚀️executables/🧫️fixtures/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🐹️go-build/🧪️tests/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🐹️go-build/🧪️tests/🧫️fixtures/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🐹️go-build/🧬️schema/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/📜️script.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🧪️tests/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🧪️tests/🧫️fixtures/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🧬️schema/🔣️.json
🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🐹️go-build/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema/📜️ticket.json
🧰️framework/🔨️modules/📁️filesystem/🛡️durability/📦️packages/🐹️go/go.mod
🧰️framework/🔨️modules/📁️filesystem/🛡️durability/📦️packages/🐹️go/🐹️_unix.go
🧰️framework/🔨️modules/📁️filesystem/🛡️durability/📦️packages/🐹️go/🐹️_windows.go
🧰️framework/🔨️modules/📁️filesystem/🛡️durability/🧪️tests/🟦️.ts
🧰️framework/🔨️modules/📁️filesystem/🛡️durability/🧪️tests/🧫️fixtures/🔣️.json
🧰️framework/🔨️modules/📁️filesystem/🛡️durability/🧬️schema/🔣️.json
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🐍️python/📜️script.ts
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🔷️dotnet/📜️script.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧪️tests/📇️catalog/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🦀️native/📜️script.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🐹️go/📦️main/go.mod
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🐹️go/📦️main/🐹️.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📜️script.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️synchronization/🧬️schema/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🧫️fixtures/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/go.mod
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🐹️go/go.mod
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🐹️go/🐹️.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🛡️durability/🐹️.go
```

Removed primitives (bodies moved into neutral filesystem owner):

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🐚️unix-durability/🐹️.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🪟️windows-durability/🐹️.go
```

Final explicit compiler argv check:12pass0fail, native mode suite Nx22.2s. Capture and relay share strict profile/configuration argument selection. Relay build/rustc/check/run/clippy compile commands select the declared mode; compiler test/bench APIs retain their compiler-specific profile semantics. Parent completed mode-neutral NativeBuildDriver/Plan/Pipeline naming and SEMIO_NATIVE_BUILD_GENERATION without compatibility exports. Final parent integration regressions use those current names.

## Original Compiler Control Integration

Concurrent contributor changed admission to require complete original cancellation, deadline, progress and continuation authority. Cargo, Go and WASM builders require CompilerAdmissionControl, compose local and original cancellation, delegate original ports, and grant children remaining original budget after admission. Profile leases carry that control. Repository factories now require explicit original control before preparation; a concurrent contributor tightened this ingress after the first green adapter run. CMake, .NET and wheel owners bind their original script invocation. All compiler host gates converge on .cache/framework/leases. Capacity and lease facades explicitly export current control APIs. Optional local cancellation reaches actual child signals while checking original and local control. Retired WASM shipProfile options were removed. No synthetic production control is created.

Actual Nx native-mode, WASM, Go-mode and capacity regression succeeded together: task duration 1m11s. WASM reported 10 pass, 0 fail, 102 assertions. Console evidence confirms both Cargo and Go mode runtimes, warm and edited correctness, reserved Cargo NUM_JOBS=1, central policy, neutral Go durability against Python, bounded concurrent one-worker children, Python SQLite exclusion, writer fairness, queued cancellation, crash recovery and actual child cancellation while its original caller remained live. Capacity fixture uses a separate generated lease directory for parallel isolation. Initial additionalSignal reference failure and shared-fixture collision were repaired before this green run.

Actual managed prototype acceptance succeeded through Nx and 5 dependencies (duration 1m21s), rebuilding .NET and wheel with original controls; actual .NET and Python runtime token both #ff344f. Initial generator admission failure did not recur after current Nx guard refreshed.

Narrow incremental compaction requires complete control and current compiler-profile exclusion before rescan/deletion. Remaining authority is checked before admission, during discovery and before each mutation. Narrow CLI receives original script control plus local cancellation. JSON fixture adds queued mutation cancellation; fast-glob independently verifies current/working/foreign-profile preservation. Initial gate failed missing checkResourceLeaseControl facade export, now repaired and rerun pending. No broad cache pruning executed.

Additional/current paths, union with earlier inventory for ticket registration:

```text
✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/📜️script.ts
✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/📜️script.ts
🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🟦️.ts
🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🧫️fixtures/🔣️.json
🧰️framework/🔨️modules/🏃️process/🏗️build/🧪️tests/🏗️native/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/📦️publication/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🐹️go-build/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🐹️go-build/🧪️tests/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🧪️tests/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts
🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts
🧰️framework/🔨️modules/📁️filesystem/🛡️durability/🧪️tests/🟦️.ts
🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🐍️python/📜️script.ts
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🔷️dotnet/📜️script.ts
🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🔒️incremental/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🔒️incremental/🧪️tests/🧫️fixtures/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts
```

Actual narrow compaction gate: 1 pass, 0 fail, 19 assertions, 653ms through Nx. It verifies real held-profile exclusion, canceled queued mutation, dry-run unchanged inputs and current/working/foreign-profile retention against fast-glob. Fixture temporary paths were shortened after current admission correctly rejected paths exceeding256characters. Both production previews passed using the shared current lease store and original control: prototype3superseded finalized sessions719291bytes; debug1session109173bytes. Current debug compiler cache is139GiB but all other sessions are retained, so narrow compaction cannot provide meaningful full-workspace disk headroom. Debug apply attempts failed before mutation due a concurrent facade transition and then graph availability. Broad cache pruning and working-directory deletion were not executed. Targeted completed-private-generation reclamation is documented separately in private-fresh-reclamation.md. Broad and narrow cache-prune serializer namespaces now match .cache/framework/leases.
