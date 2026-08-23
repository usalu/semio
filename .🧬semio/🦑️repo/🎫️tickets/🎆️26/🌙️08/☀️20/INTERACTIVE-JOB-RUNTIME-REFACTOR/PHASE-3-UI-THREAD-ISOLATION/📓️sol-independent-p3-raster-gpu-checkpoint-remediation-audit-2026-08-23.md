# Sol Independent P3 Raster GPU Checkpoint Remediation Audit — 2026-08-23

## Verdict

**REJECT — source-only.** The remediation closes the earlier pre-realization reservation, exact `Texture` + `TextureView` handoff, independent-operation wiring, staged/live last-valid, and structural-matrix gaps. Two required invariants remain false in live source: the raster operation sequence can wrap and reuse an old operation number, and valid cancellation/interrupted-upload close bypass the retained scalar retirement authorities. A third ordering path creates a GPU bind group before the reservation/candidate/staged-slot generation is revalidated.

This is an independent non-author audit. No production source was edited. Cargo, Nx, Wasm, browser/runtime execution, network, and root lint stayed closed.

## Inputs

- `📓️p3i-browser-worker-implementation-audit-20260822.md`, including its latest raster remediation section.
- `📓️sol-independent-p3-raster-gpu-checkpoint-audit-2026-08-23.md`.
- The actual working, staged, and `HEAD` diffs and live sources for framework raster table/GPU/glue, product WGPU glue, `EngineCanvas`, browser worker, and OS-host close ordering.

## Accepted source evidence

The following narrow requirements are source-closed:

- `RasterTextureStore` uses 256 fixed slots, an eight-probe key lookup, a 256-byte key limit, a 16 MiB per-item cap, a 256 MiB table cap, and 16 KiB upload pages.
- `EngineCanvas::realize_one` reserves through `gpu.reserve_engine_texture` before `Renderer::new`, target texture creation, or Vello realization.
- `EngineCanvas` transfers both `Texture` and `TextureView` by value with `mem::replace`; failed staging restores the exact returned texture and view. No live `surface.view.clone()` was found.
- The admission witness is checked at EngineCanvas, table staging, submit, and ACK. `insert_vacant` prevents replacement, and staged entries remain distinct from live last-valid entries until commit; abort retains the live generation.
- The runtime authority derives scene revision and preview generation from the presentation authority rather than candidate packet fields.
- Upload work is page/row limited to 16 KiB per opportunity.
- Table entries, reservation fields, witness fields, texture/view/bind-group owners, and normal close paths have retained retirement cursors; OS-host ordering keeps raster-table terminality before world-authority close.
- Direct negative scans found no live raster `HashMap<String, RasterTexture>`, `surface.view.clone()`, raster `operation: u32`, or `mem::forget`. Borrowed `&TextureView` hits are ordinary glyph/swapchain inputs; the staged raster texture-view owner is by value.

## Blocking findings

### P3-R1 — Operation generation wraps and permits ABA

`RuntimeRasterOperationAuthority::begin` uses `AtomicU64::fetch_add(1)` and only rejects the returned value when it is zero (`📦️glue.rs:6573-6584`). At exhaustion, the call returning `u64::MAX` is accepted, the counter wraps to zero, the next call rejects zero after advancing the counter to one, and a later call reissues operation `1`. A stale operation-1 token can therefore collide with a newer operation-1 token for the same scene/preview pair.

The existing fixture exercises ordinary consecutive operations but not `u64::MAX`, exhaustion permanence, or reopen-after-wrap ABA. The structural predicate only checks that the authority and its `begin`/`matches` calls exist; it does not deny unchecked wrap.

Required repair:

1. Use a checked monotonic allocation transition that enters a permanent exhausted state before wrap and never reuses a number.
2. Preserve generation matching at EngineCanvas, table, submit, and ACK.
3. Add `MAX-1`, `MAX`, exhausted-next, release, and reopen attempts proving no operation is reused; add an adversarial mutation restoring `fetch_add(1)`.

### P3-R2 — Cancellation and interrupted close drop multi-field authorities in one grant

The fixed retirement types are present, but two live paths bypass them:

- A matching `cancel_engine_texture_admission` assigns `self.reservation = None` and returns immediately (`draw.rs:1737-1740`). That drops the reservation key and seven scalar credits together instead of transferring them to `RasterTextureReservationRetirement` and retiring one scalar/owner per grant.
- `close_upload_step` takes the upload value and admission, assigns `self.reservation = None`, constructs an entry retirement, calls `self.presenting.set(admission.witness)` (three witness scalars at once), and clears `self.upload` in the same opportunity (`draw.rs:2022-2039`).

These are ordinary cleanup paths, so the requirement that every cleanup grant releases at most one scalar or owned GPU resource is not met. The existing close fixture proves eventual witness-slot cleanup, not these full valid-reservation cancellation and interrupted-upload paths.

Required repair:

1. Transfer matching cancellation into `RasterTextureReservationRetirement`; never clear a populated reservation wholesale.
2. Give interrupted upload a retained transition cursor that moves one source owner/scalar per grant, then begins entry/candidate/presenting retirement without setting a whole witness in one grant.
3. Add valid cancel, close-before-first-page, close-mid-page, exact texture/view return, and terminal-empty fixtures with per-grant owner/scalar accounting; add mutations for both wholesale assignments.

### P3-R3 — Bind-group allocation precedes complete generation revalidation

`stage_gpu_bind_group` checks only `admission.witness == expected`, creates the GPU bind group, and only afterward calls `stage_admitted_texture`, where reservation nonce/key/dimensions, candidate witness, and vacant staged slot are validated (`draw.rs:1869-1900`, with the complete checks in `draw.rs:1757-1776`). A stale/missing/ABA reservation or occupied staged slot can therefore allocate a bind group before rejection; that bind group is then dropped while only the exact texture/view are handed back.

Required repair:

1. Validate the complete reservation/candidate/staged-slot authority before GPU allocation, with a retained claim preventing change between validation and publication.
2. If bind-group creation itself must be fallible/interrupted, retain its exact owner for staged publication or bounded close.
3. Add missing-reservation, nonce ABA, candidate change, and occupied-slot fixtures proving rejection before bind-group allocation; mutate the ordering and require denial.

## Discriminating probes and fixtures

An independent 11-rule live-source probe passed 9/11:

- PASS: fixed slot/probe/key/byte ledger.
- PASS: pre-realization reservation ordering.
- PASS: preflight before reservation publication.
- PASS: exact by-value texture/view handoff.
- **FAIL: independently monotonic operation**, due to unchecked `fetch_add` wrap.
- PASS: stale/duplicate checks at engine/table/submit/ACK.
- PASS: one-page upload.
- PASS: last-valid begin/commit/abort.
- PASS: vacant-only staged publication.
- **FAIL: one-owner/scalar cleanup**, due to the matching cancellation and upload-close assignments.
- PASS: terminal-before-world ordering and exact structural matrix presence.

The permanent structural predicate accepts the live sources. Reconstructing its Rust `String::replace` semantics with replacement of every occurrence denied all 13 declared mutations (13/13). The matrix is useful for the fields it names but is not discriminating for operation overflow, full reservation cancellation, interrupted upload transition, or bind-group-before-full-validation; those missing mutations explain why the predicate stays green despite the live failures above.

Direct fixtures cover table 256/+1 owner handback, key 256/+1, per-item bytes cap/+1, normal operation freshness, staged/live commit/abort, and witness retirement. There is no discriminating aggregate-byte +1 admission fixture, operation-wrap fixture, valid-reservation cancellation fixture, or interrupted-upload per-grant retirement fixture.

## Permitted gate evidence

- Scoped `rustfmt --edition 2021 --check --config skip_children=true`: **PASS** for framework `draw.rs`, `gpu.rs`, glue, product WGPU glue, browser worker, and full `EngineCanvas`.
- `rustfmt --emit stdout` parser check on the same sources: **PASS**.
- `bun 📜️script.ts verify interactivity --self-test --format json`: **PASS**, zero DENY findings and one approved blocking allowlist finding.
- `bun 📜️script.ts verify interactivity --format json`: **PASS**, same zero-DENY result.
- Exact 13-mutation raster structural reconstruction: **13/13 denied** with baseline accepted.
- Scoped working/staged/`HEAD` `git diff --check`: **PASS**.
- Whole working/staged/`HEAD` `git diff --check`: **PASS**.

No build or runtime result is claimed.

## Scope and residuals

This rejection is confined to the raster checkpoint remediation above. The larger Phase 3 boundary remains red for the already reported prepared-container, atlas/icon/glyph, dynamic EngineCanvas surface, Vello/GPU-runtime, realm, platform, and runtime-matrix work. Passing source gates do not establish runtime acceptance.

