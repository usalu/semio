# Native Dependency Preparation and Lock Sequence Review

Actual WGPU48039 log 11195 bytes SHA256 `f6f0b9a7e7e55af88b31d92daab18a92cc73f3347bf7aed6e5548b3b37df718a` ends Nx130 after10m3s, failing workspace:deps-cargo; actual wasm target was not run. It records composition preparations then refusal to update current S Cargo.lock under --locked, Cargo101. Six other completed prerequisites do not establish Trunk/native WGPU success.

Current native dependency sync enumerates discoverCargoWorkspaces and invokes cargo fetch --locked for each exact owner manifest. Shared runTool invokes prepareCargoWorkspaceInvocation synchronously before spawn; selected owner preparation performs member/composition recipes, which can alter authored manifest closure before Cargo reads the lock. Current sync has no explicit owned lock refresh between preparation and final locked verification. Separate existing cargo-lock invokes cargo update --workspace per selected owner through the same preparation boundary.

A zero-touch correction should use the existing owned preparation/lock owner and bind one current discovered owner/manifest closure to its corresponding lock, then retain final locked fetch verification. Do not catch arbitrary Cargo101 by stripping locked, do not substitute historical root/S ownership, and do not claim a completed preparation means the lock already matches. Composition should be stable across the final refresh/check; changed roster/manifest inputs need refusal or a fresh owned sequence. Runtime was sent this exact seam. No fetch, metadata, compile, file changes or live-job operations executed by the auditor.

At this source read the proposed current-actor wrapper had not appeared and childUrl/body allocation safety changes still remained owner-active; no repair pass inferred. Final neutral child execution remains separately qualified.


## Current Coherence Repair and Neutral Receipt

Source now iterates the same discovered admitted owner and awaits cargo update --workspace --manifest-path owner, followed by unchanged cargo fetch --locked --manifest-path owner. Shared runner still prepares each exact owner before process spawn. Lock writes belong to Cargo selected workspace, not manual file replacement. Final locked verification remains a refusal if preparation or concurrent state changes afterward. No unrelated package/profile/target flags added. This source does not attest broad network dependency-version immutability beyond Cargo update --workspace semantics.

`oct8-native-sync-coherence-red.log`: 7624 bytes SHA256 `f944fa3849549aa70e54860a4cfda295dd45973517de05a100344632913b5df4`.

`oct8-native-sync-coherence-green.log`: 1475 bytes SHA256 `6c8ded3dbb93a404297369a67d2ac476d4f71bad74b6bf075a26e16159dea249`.

Independent retainedGREEN footer Nx0/4.1s; actual negative lockedfetch messages are expected test observations, not GREEN failure. Test source proves generated local dependency initially invalidates lockedfetch without changing lock, then two uncached ordinary Nx syncs create the exact prepared package lock/metadata, final offline lockedfetch succeeds, repeated lock bytes stable and no target deliverables exist. Separate invalid-lock refresh/existing explicit lock target checked. RED Nx1/2.0s remains meaningful neutral coherence failure. This is tiny real Cargo/Nx evidence, not actual S/WGPU production or Trunk. No commands executed by this auditor.
