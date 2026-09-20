# Raster Owner Review

Root reviewed the first disjoint TypeScript pool model before production wiring. The natural-resolution contract and page-sized copies are appropriate, but the first lease representation is not sufficient for exact ownership.

A ready acquisition returns slot, epoch, identity and contentDigest. Two live panes therefore receive value-identical lease tokens. Calling release twice with the first pane's token decrements the second pane's credit as well; zero-lease eviction can then retire pixels that the second pane still owns. A refcount without a unique acquisition identity cannot reject this duplicate terminal event.

The executor must add an exact, bounded acquisition owner or one-use lease registry, and test duplicate release, copied-token release, two-pane survival, upload aliases, last-owner retirement and refusal at the lease-credit limit. Rust move semantics help for internal handles, but cannot replace validation at a token-based browser/prepared boundary.

The first model also increments slot epochs without an exhaustion guard and retains caller-provided identity objects by reference. The next revision must refuse exhausted epochs before mutation and own immutable metadata so caller mutation cannot change an admitted identity.

These findings were sent to SolChrome before shared World/prepared/renderer wiring. No production failure is claimed: the new model is not wired into the active renderer yet. Existing source-backed full-resolution design remains in terra-raster-quality-audit.md; the execution report is astra-sol-raster-quality.md.


## Model repair review

SolChrome repaired duplicate release with a unique one-use lease ID. Each ready slot now has an explicit active capacity of 64; the 65th acquisition refuses without minting an ID or mutating slot authority. Slot epochs and lease IDs refuse u64 exhaustion. Admitted identity and pool limits are copied and frozen. Page access returns only a bounded copy in the neutral TypeScript model; the planned Rust pool exposes immutable scoped borrows. This also closes the follow-up finding that a returned mutable Uint8Array could invalidate the precomputed content digest.

The executor reports 10/10 focused Vitest laws passing. This validates the neutral state machine, not production full-resolution behavior. Shared production wiring was released after checkpoint15 renderer WASM compilation, and its native/GPU/browser acceptance remains pending.

## Rust core API review

The first registered pool revision caused three borrow errors during Native26; the executor repaired those before further wiring. Root also caught incorrect cross-slot credit subtraction; the revised slot stores its own exact reserved_bytes and a new uneven-slot law requires cancelling4 bytes to preserve another live16-byte lease. Native27 then stopped in an unrelated in-flight reference tint API, so the new pool laws are not yet runtime-accepted.

Additional core findings sent before caller integration: wrong-mode seal must refuse before replacing a Writing slot; writer transfers need the neutral model’s exact cursor to reject copied/replayed pages; mesh-paint seal requirements must be validated before admission. The four-slot pool also needs GPU-commit handoff that releases CPU leases so a fifth distinct visible asset can progress. Finally, decrementing a retirement integer while retaining one contiguous Vec is not physical page-sized retirement; the implementation and contract must accurately bound actual resource release. These findings remain under executor repair.

## Native core verification

Native28 compiled the coherent Rust pool successfully. UI33 then found a test helper’s writer binding was immutable despite the new cursor-returning API; Astra made that binding mutable. UI34 executed632 tests:631 passed and one failed. The sole failure was the uneven-slot fixture attempting to push4 bytes into a2-pixel/8-byte row, correctly rejected by row alignment. Astra changed the8-byte small image from2×1 to1×2 so one4-byte row is a valid partial image; all exact credit assertions remain unchanged. UI35 is the rerun.

## GPU Commit and Recovery Review

UI 39 passes 632 tests at its sampled boundary, but the new exact GPU byte-credit, replacement, teardown, and fifth-asset stress laws were not yet added at that point. Sol is adding them before the next coherent integration marker.

A concrete recovery concern is visible in the current World producer: `world_raster_upload` takes a pending lease or reacquires the CPU identity, while the reference request filter skips any URL already present in `reference_pixels`. Once CPU storage has been evicted and the GPU witness retired, an identity-only World entry has neither source of pixels and suppresses a new decode request. The required regression is more than four distinct assets, CPU eviction, GPU retirement, then the same URL becoming visible again and recovering without losing natural dimensions. Both stepped and direct builder paths must use the same readiness policy.

A second authority question remains under validation: the World pool is process-global while GPU residency is currently keyed by texture key and identity. If multiple renderer tables/devices can coexist, a resident texture in one must not suppress upload in another. The implementation owner is checking table ownership and the reused-texture acknowledgement path. These are review findings, not claims that the final integration fails its future tests.

## Texture Storage and Shader Color Contract

The new pooled upload format initially selected `Rgba8Unorm` for image/paint NoColorSpace profiles. The verified S2 painted and S3 reference WGSL currently expects sRGB texture storage and explicitly reconstructs NoColorSpace samples using `world3d_linear_to_srgb`. Changing storage to Unorm without updating that shader/profile contract applies an extra transfer. Reference instances currently use appearance.y=0, which selects the recovery branch. Sol is assigned a consistent texture-format/shader policy and rerunning the actual Three/production WebGPU pixel fixtures against that policy. String-level shader tests and the old pixel output do not validate a changed texture format.

## World 23 Runtime Findings and Root Repair

World 23 executed 236 selected tests: 233 passed, 3 failed. The replacement stress law failed at replacement 64 because removed World reference entries entered the old opaque quarantine. That production quarantine has no drain, so the new pooled CPU leases remained live until the 64-owner slot cap. This is a real ownership leak, independently confirmed by the final Terra audit.

WorldSceneRaster now contains only bounded identity metadata and one optional lease; the pool retains the allocation. Root removed raster variants from the opaque quarantine and releases their bounded metadata directly on registry replacement/rejection/removal and one entry per dynamic-close grant. Draw quarantine is unchanged. The existing schema-first 256-plus-replacements law exercises the correction.

The six-resident producer law also exhausted prepared-input permits because its synthetic frame loop dropped inputs without stepping abandonment retirement. It now drives the production bounded abandonment close route between iterations, with a 64-step ceiling. The remaining stale-aim interaction failure now includes its authority census for diagnosis; its limit is unchanged. World 24 is running.

The final Terra audit separately found that browser row transport still receives a complete ImageData and duplicate Uint8Array from the decoder before streaming strips. Thus the current report overstates strip-only browser ownership. This is queued for an execution-agent repair; pixel and UI checks do not establish bounded browser decode ownership.

World 24 confirms the reference replacement leak repair: the 256-plus replacement law passed. The six-resident law then reached its recovery assertion, which inspected an obsolete pending-URL set that the actual reserve API does not populate; root replaced that assertion with the real fetch-owner token, URL, kind, and return-to-retirement checks.

The stale-aim law exposed a second real runtime regression: its new ReferenceSelect fallback scanned all 1,024 fixed registry slots, including every vacant slot, one frame grant at a time. Root added a 16-word occupied bitmap to the interaction registry and made the object pick cursor advance between occupied slots. Empty reference picking now reaches completion without visiting all vacancies; token/revision validation and stable slot order remain. World 25 verifies these changes.

World 26 passed all 236 selected World/terrain tests (243 tests outside this target were skipped). The pass includes repeated reference replacement, six pooled GPU identities through two CPU slots with same-source recovery, stale-aim/middle-click completion, and all S3 reference-visual laws. The interaction registry bitmap adds exactly 128 inline bytes (owner 40 → 168), now recorded in the fixed-slot memory fixture; the bounded-thread-stack law passed. Full renderer/browser acceptance is still pending.
