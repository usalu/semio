# Sol Independent P3 Raster GPU Checkpoint Remediation Re-audit — 2026-08-23

## Verdict

**REJECT — source-only.** The permanent operation allocator and retained cancellation/upload-close repairs close the first two prior rejection classes. The complete allocation claim is now checked before the prepared texture allocation and before the separate EngineCanvas bind-group allocation, but it is not checked immediately before every live texture and bind-group allocation as required. The prepared path performs texture and view allocations between its only complete claim and its bind-group allocation, and its publication-fault adapter drops the exact admission plus GPU owner instead of returning or retaining them.

This is an independent non-author audit. No production source was edited. Cargo, Nx, Wasm, browser/runtime execution, network, and root lint remained closed.

## Inputs and scope

- `📓️sol-independent-p3-raster-gpu-checkpoint-remediation-audit-2026-08-23.md`.
- The superseding raster repair section in `📓️p3i-browser-worker-implementation-audit-20260822.md`.
- Live framework raster table/GPU/glue, product WGPU glue, `EngineCanvas`, browser worker, and the relevant working/staged/`HEAD` diffs.

The audit rechecked permanent operation allocation, matching cancellation, interrupted upload retirement, the full allocation tuple, exact GPU owner flow, fixed caps/key/page limits, staged/live publication, presenter ACK/abort, and close ordering.

## Closed prior findings

### P3-R1 — Permanent operation generation is source-closed

`RuntimeRasterOperationAuthority::begin` is serialized by the strong `current` mutex (`📦️glue.rs:6632-6649`). It compares the atomic sequence rather than using `fetch_add`; on `u64::MAX` it atomically sets `exhausted` before publishing MAX (`📦️glue.rs:6651-6668`). `release` only clears the matching current witness and cannot clear exhaustion or advance the sequence (`📦️glue.rs:6679-6685`). Consequently concurrent callers cannot mint two operations, MAX is issued once, `next_operation` remains MAX, and release cannot reopen or create ABA.

The direct fixture covers MAX-1, MAX, occupied MAX, release, permanent exhaustion, empty current, exhausted=true, and next=MAX (`📦️glue.rs:6177-6194`). The fixture is sequential rather than multithreaded, but the live mutex makes the allocation transition single-file under concurrency. The operation-overflow mutation restores unchecked `fetch_add`; the structural predicate rejects it.

### P3-R2 — Matching cancel and interrupted upload retirement are source-closed

- Matching cancellation moves the live reservation with `take` and the exact admission into `RasterTextureReservationCloseCursor::cancelled` (`draw.rs:2015-2028`). It no longer clears a populated reservation wholesale.
- The close cursor owns separate reservation and admission retirement authorities. Each key root and each of eight scalars is released in its own `RasterTextureCleanupStep`; completion is asserted terminal-empty (`draw.rs:1512-1641`). The direct matching fixture observes exactly two roots and sixteen scalars with at most one released authority per step (`draw.rs:3919-3943`).
- Interrupted upload transfers the admission, allocation claim, GPU value, and row marker through `RasterTextureUploadCloseCursor` (`draw.rs:1806-1889`). Owner transfers report retained progress without releasing authority. The GPU retirement owner releases bind group, view, texture, key, and scalar fields individually. The table first retires any reservation, then creates and drives the upload-close cursor; it does not clear the upload or reservation wholesale (`draw.rs:2301-2329`).
- The direct interrupted-upload fixture exercises before-first-page and mid-page scalar/claim retirement and enforces at most one released authority per step (`draw.rs:3969-3988`). It constructs `value: None`, so it does not itself discriminate bind-group/view/texture root retirement; that part is established by the live retirement-owner source rather than a GPU-root fixture.

## Blocking findings

### P3-R3 — Complete claim is not immediate before every GPU allocation

The prepared raster path obtains its only full `RasterTextureStageClaim` at `draw.rs:2082`. It then allocates the texture at `draw.rs:2083-2092`, allocates the view at `draw.rs:2093`, and allocates the bind group at `draw.rs:2094-2104`. There is no second full reservation/key/dimensions/nonce/candidate/staged-vacancy validation immediately before the bind-group allocation. Thus two GPU allocations occur between the stale-state validation and bind-group creation, contrary to the required no-allocation-between-validation boundary.

The external EngineCanvas path does validate immediately before its later bind-group allocation (`draw.rs:2155-2169`), but its initial reservation at `EngineCanvas/🧊️component.rs:431` is followed by `Renderer::new`, target texture allocation, rendering, and replacement texture allocation (`component.rs:434-459`) before that first complete stage claim. Reservation-time checks and an immutable copied claim are not the requested immediate pre-allocation witness at those texture allocation edges.

Publication does revalidate the reservation, key/dimensions/nonce-bearing admission, candidate, staged index, and vacancy before `insert_vacant` (`draw.rs:2036-2051`). That protects publication, but does not repair the missing immediate pre-bind-group and EngineCanvas texture-allocation validations.

### P3-R4 — Prepared publication fault drops exact GPU ownership

`stage_claimed_texture` correctly returns the exact `RasterTextureAdmission` and `RasterTexture` on all revalidation/vacancy failures (`draw.rs:2036-2049`). `stage_gpu_bind_group` preserves this contract by returning the exact admission, texture, and view to EngineCanvas (`draw.rs:2173-2175`), and EngineCanvas restores the returned texture and view (`component.rs:460-464`).

The prepared path instead erases both returned owners with `map_err(|(fault, _, _)| fault)?` at `draw.rs:2130`. A candidate/reservation/staged-vacancy publication fault therefore ordinarily drops the returned admission, bind group, view, and texture rather than placing them in the retained close authority or returning them to a caller. This violates exact fault handback adjacent to the rejected allocation ordering.

## Preserved invariants

- Fixed table capacity 256, eight probes, 256-byte keys, 16 MiB per item, and 256 MiB aggregate remain present.
- Upload work remains limited to 16 KiB pages/row groups.
- `insert_vacant` protects staged publication; staged and live last-valid registries remain separate until commit, and abort preserves live.
- Raster witness freshness still uses independent scene revision, preview generation, and monotonic operation with duplicate rejection.
- External staging transfers `Texture` and `TextureView` by value. No live `surface.view.clone()`, raster `HashMap<String, RasterTexture>`, raster `operation: u32`, `insert_at`, or `mem::forget` witness was found in the audited sources.
- Presenter commit/abort and table terminal-before-world close ordering remain structurally present.

## Fixtures and mutation evidence

The permanent `retained_raster_contract` baseline evaluates true. A faithful reconstruction of the Rust all-occurrence `String::replace` semantics denied **17/17** declared mutations. This includes fixed caps/key, admission erasure, duplicate freshness, texture-view erasure/borrowing/clone, missing pre-realization reserve, operation-authority erasure/wrap/input derivation/match bypass, wholesale matching cancel, wholesale upload clear, renamed allocation claim, and close bypass.

The suite is not discriminating for the live blockers:

- it only asserts `upload_claim < texture_allocation` and `gpu_claim < bind_group_allocation`;
- it does not require a complete claim immediately before every texture/view/bind-group allocation or forbid intervening allocations;
- it does not mutate a second pre-bind-group validation away;
- it does not reject the prepared `map_err(|(fault, _, _)| fault)` owner erasure;
- its interrupted-upload fixture supplies no GPU value, so it cannot observe bind-group/view/texture retirement.

## Required repair packet

1. Revalidate and retain the complete reservation/key/dimensions/nonce/candidate/staged-vacancy claim immediately before each live texture and bind-group allocation edge. No GPU or renderer allocation may intervene between its applicable stale validation and allocation.
2. Keep the claim generation-armed across texture/view creation, perform a fresh full validation immediately before bind-group creation, and retain it through the existing publication revalidation.
3. On prepared publication failure, move the exact admission and `RasterTexture` into an observable retained fault/close cursor, or return them by value; never erase them in `map_err`.
4. Add ordering mutations that move/remove each immediate pre-texture and pre-bind-group validation independently, plus a mutation restoring prepared owner-erasing `map_err`.
5. Add a GPU-owner-independent semantic fixture or injected oracle proving interrupted close and publication fault retire/return bind group, view, texture, admission, claim, reservation, and scalars one authority per grant.

## Permitted gate evidence

- Scoped `rustfmt --edition 2021 --check --config skip_children=true`: **PASS** for framework draw/GPU/glue, product WGPU glue/browser worker, and `EngineCanvas`.
- `rustfmt --emit stdout` parser checks for the same six files: **PASS**.
- `bun 📜️script.ts verify interactivity --self-test --format json`: **PASS**, one recorded allowlisted blocking-bridge finding and zero DENY findings.
- `bun 📜️script.ts verify interactivity --format json`: **PASS**, the same zero-DENY result.
- Faithful raster predicate/mutation reconstruction: **PASS**, baseline true and **17/17 denied**.
- Scoped working/staged/`HEAD` `git diff --check`: **PASS**.
- Whole working `git diff --check`: **PASS**.
- Whole staged and whole `HEAD` `git diff --check`: **FAIL** on the pre-existing staged prior audit report `📓️sol-independent-p3-raster-gpu-checkpoint-remediation-audit-2026-08-23.md:102` because it has a blank line at EOF. No source file is named by that failure, and this independent audit did not alter the prior report.

No build or runtime result is claimed. Phase 3 remains red for this repair packet and the separately reported prepared-container, atlas/icon/glyph, EngineCanvas surface, Vello/GPU runtime, realm, platform, and runtime-matrix residuals.
