# Trunk Output Boundary — 2026-09-13

The current native graph still resolves @semio-tech/framework-renderer-wgpu:wasm as cache:true with outputs:[]. Its package command runs a complete Trunk application build into the live renderer cache. This remains a confirmed unsound cache contract. No Trunk producer or serving source was changed during this follow-up.

The HTML input copies the active dev-profile plugin store, the separate live extension store and the complete asset source tree, alongside boot and worker bytes. The inferred prepare-variant-wgpu-dev/release tasks both depend on the same unspecialized wasm target; plugin/extension materialization is parallel to that dependency. A finite producer must not capture whichever active store happens to exist. TrunkServeScript also compiles when serving starts, and starts an asset server internally. Its installed CLI has no option to serve without the initial build.

The next implementation must separate renderer compilation, variant composition and live serving. Final deliverables need profile-specific owners; composition must consume the completed plugin/extension artifact owners; serving must consume the completed distribution and use Nx to schedule changed prerequisites. Preserve existing browser boot, worker, module routes, proxy behavior, readiness and cancellation, with native cache restoration and HTTP/runtime proof. Existing React production distribution and activation helpers should be examined for reuse.

Trunk 0.21.14 first probes the selected tool on PATH. With an explicit required version it accepts only a match; offline mode rejects missing/mismatched tools before consulting its download cache. Without a requirement it accepts any discovered version. Its default optimizer is Binaryen 123, whereas this repository's prepared optimizer is 130. Therefore simply preparing Binaryen without selecting it in Trunk does not establish compiler identity. The finite compiler must receive exact prepared tool paths/versions and prohibit hidden acquisition. [Pinned Trunk tool-selection source](https://github.com/trunk-rs/trunk/blob/v0.21.14/src/tools.rs).

The root setup deps wasm command already prepares wasm-pack and a wasm-bindgen CLI version read from Cargo.lock, in addition to WASI tooling. workspace:deps-trunk currently prepares only Trunk and the wasm32-unknown-unknown Rust target. Native optimizer selection and bindgen preparation need explicit graph treatment before claiming the Trunk compiler is qualified.

The former trunkrs.dev documentation URL returned unrelated content during this research and was not used as technical evidence. The pinned GitHub source and installed command help were used instead. No shared tool install, Trunk build or live-server mutation was performed.

## Finite Compiler Follow-Up — 2026-09-13

The native fixture now passes: 1m8s, cold/warm/deleted-output restoration for both dev and release, source invalidation, and last-successful-output preservation after a compiler error. Restored JavaScript/WASM executes in Node and matches an independently configured native Trunk build byte-for-byte. Trunk requires the optimizer pin `version_130`, confirmed by a rejected `130` pin followed by the successful release run. The compiler stages only renderer modules and aliases in Rust-package `dist/wasm-dev` and `dist/wasm-release`; it copies no ambient modules/assets. Temporary Trunk and uplifted Cargo directories are retired after each invocation while Cargo incremental state uses its configured store.

The package now declares its Rust manifest root so inference can select Cargo source/dependency/generator contracts. Both authored compiler targets are finite and cacheable. Locked Cargo synchronization, Trunk/bindgen preparation and Binaryen preparation are explicit uncached prerequisites. The locked-bindgen tooling regression passes (4.2s). Editor seed commands were corrected to wasm/wasm-release and regenerated. Matching-profile playground prerequisite changes are under verification. The old live Trunk server and its shared copy tree still require replacement; no server runtime success is claimed. The full repository suite has not yet been rerun after these compiler changes.

Pinned tool selection fields were checked against [Trunk tools source](https://github.com/trunk-rs/trunk/blob/v0.21.14/src/config/models/tools.rs) and exercised by the installed native CLI.

The extended compiler/consumer fixture passed in 1m21s, including both profile routes served by native Vite after Nx restoration. Browser boot generation boundary checks passed separately in 4.8s. A full run first passed the native compiler fixture, then correctly rejected the obsolete browser-boot consumer assertion; that fixture is corrected and a new full run is in progress. An actual repository `@semio-tech/framework-renderer-wgpu:wasm` build is also running through its seven explicit prerequisites (three synchronization/tool targets and four generators).

Native graph inspection confirms 89 local Cargo dependency owners, 27 compiler environment inputs and five runtime fingerprints for each profile. The WGPU package has a custom build.rs that scans source SVGs; inference intentionally retains its conservative engine source glob plus explicit native assets rather than claiming a fully minimal first-party source closure. No build-script source discovery optimization is claimed.

The extended actual-root repo:test suite passed on 2026-09-13: 4m36s, 0/2 cache hits. It includes both native Trunk profiles, poisoned Trunk environment isolation, native Vite HTTP bytes, all compiler restoration proofs, graph/editor/lifecycle and cancellation contracts (704 projects, 6903 targets). The actual full renderer build remains running; it is not yet qualified.

The full renderer invocation did not complete: Trunk reports Cargo exited on SIGTERM after 19m28s. No origin for the signal is established, and no full renderer publication is claimed. A fresh native retry is running. See 📓️wgpu-browser-serving.md for the new live consumer boundary.

Output accounting found that compatibility aliases duplicated the renderer JavaScript and its 76 MiB WASM file. The native fixture now requires exactly the compiler’s canonical JS/WASM pair and failed on the four-file publication (54.4s). The compiler no longer publishes aliases; the renderer library consumes the canonical module URL. Native restoration/oracle and actual publication tests are in progress. The existing native source resolver still returns a conservative fallback for any build.rs; this is why the current WGPU hash also includes broad engine sources despite its explicit icon/logo inputs. A future narrowing must preserve all custom build-script reads and have a native dependency oracle.

Canonical-only publication now passes its native fixture (2m56s): both profiles execute, restore after deletion, match independent Trunk bytes, invalidate on source changes and preserve previous output after compiler failure. The actual-root republish did not reach compilation: current taxonomy validation rejects references to removed configuration contracts (tailwind/eslint/dependency-cruiser/VS Code test CLI) in dependency and graph-generator prerequisites. The actual frame-worker regeneration is stopped by the same prerequisite issue. These unrelated active taxonomy changes have not been reverted or bypassed; actual-root alias retirement remains unverified.

## Canonical Repository Publication — 2026-09-13

The actual repository WGPU wasm target passed after the native dependency boundary change and concurrent taxonomy repair: 3m 7s task-run duration, 0/8 cache hits. Native generation and compilation both ran. The completed wasm-dev output directory contains only the canonical JS/WASM pair and the ownership receipt; duplicate aliases were retired. Current sizes and SHA-256 identities:

- .nx-artifact.json: 133 bytes; 7adf76ccf7a91cfb2ebfa38476c502ea3e56478acb0d90ea11c87743ef807e49
- semio-framework-os-renderer-wgpu.js: 177202 bytes; 2638808aa69612e8e1f1874c8fd24bd22be45166f2e6b39e38998c4838376906
- semio-framework-os-renderer-wgpu_bg.wasm: 79294972 bytes; 24277dd1a99cfa9af412c01ed8a0476c9ac7c281519ac591b57593c9fc6db7ab

An actual HTTP consumer and a repeated invocation are being checked separately. Release-profile publication has not been run at this checkpoint.

The actual HTTP consumer passed in 8.4 seconds after canonical publication. It served the published compiler, browser boot and frame worker bytes and closed its owned listener. The repeated repository invocation overlapped active source edits, rebuilt native dependencies and failed after 3m 26s; it is not an identical-input warm-cache measurement. Diagnostic details follow when reviewed.

The repeated repository build failed on two E0603 errors: ArtifactDialect was private in the concurrently changing renderer source. A follow-up comparison found the live WASM identity had changed during overlapping work, so the shared output cannot serve as an isolated failure-preservation witness. No current-source warm-cache or byte-preservation claim is made for that run. Controlled compiler failure-preservation remains covered by the native fixture. Native library incrementality passed independently (📓️native-incremental-publication.md).
