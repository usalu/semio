# Sol Independent P3 Raster GPU Allocation-owner Final Re-audit — 2026-08-23

## Verdict

**REJECT — source-only.** The live allocation-boundary and publication-owner repairs close the two
source defects from the prior re-audit, but the required 38-mutation authority is not
mutation-complete. A faithful independent reconstruction accepts the mutation that removes
`gpu.reserve_engine_texture` from the EngineCanvas live route. The authored Rust test would
therefore fail its own per-mutation assertion if it were compiled.

This was an independent non-author audit. I made no production-source edits. Cargo, Nx, Wasm,
browser/runtime execution, network, and root lint remained closed.

## Inputs and scope

- `📓️sol-independent-p3-raster-gpu-checkpoint-remediation-reaudit-2026-08-23.md`.
- The superseding raster allocation-boundary section in
  `📓️p3i-browser-worker-implementation-audit-20260822.md`.
- Live framework raster draw/GPU/glue, product WGPU glue, EngineCanvas, and their
  working/staged/`HEAD` diffs.

The audit checked every prepared and external texture/view/renderer/bind-group allocation edge,
the full tuple claim, returned versus retained publication ownership, cleanup granularity,
permanent operation-generation exhaustion, the 38 mutations, and the permitted static gates.

## Source repairs verified

### Complete claim and immediate allocation ordering

`claim_raster_stage_tuple` validates the entire live tuple before returning a claim:

- the admission witness equals the independent expected witness;
- the reservation exists and matches key, all three witness scalars, width, height, byte credit,
  staged index, and nonce;
- the table candidate still equals that expected witness; and
- the admitted staged slot is still vacant.

The prepared upload retains its admission and claim before allocation. Fresh full-claim calls
immediately precede texture creation, view creation, and bind-group creation. The gaps contain no
other texture/view/bind-group, target-texture, or renderer allocation.

EngineCanvas obtains the live admission before realization. Fresh full-tuple validations
immediately precede the initial target texture, its view, `Renderer::new`, the resize
texture/view pair, and the final replacement texture/view pair. The table performs another fresh
claim immediately before its external bind-group allocation. Publication then revalidates the
reservation, claim/admission tuple, candidate, nonce/index, and staged vacancy before
`insert_vacant`.

### Exact returned and retained owners

An external pre-bind claim fault returns the same `RasterTextureAdmission`, `Texture`, and
`TextureView` in `RasterTextureStageFault::Returned`; EngineCanvas restores the two exact
surface roots and sends the admission through exact cancellation.

After bind-group creation, a publication fault returns no owner-erasing adapter. It moves the
admission, allocation claim, texture, view, and bind group into
`RasterTextureUploadCloseCursor` and reports `RasterTextureStageFault::Retained`. The prepared
publication-fault branch uses the same retained cursor. Cleanup first transfers these roots into
their retirement authorities, then releases at most one bind group, view, texture, key, or scalar
per grant. Terminal completion requires every source/admission/claim/GPU root to be absent.

The live slices contain no former `map_err(|(fault, _, _)| fault)`, cloned published
EngineCanvas view, raster `HashMap<String, RasterTexture>`, raster `u32` operation, unchecked
operation `fetch_add`, or `mem::forget` witness.

### Preserved accepted behavior

`RuntimeRasterOperationAuthority::begin` remains serialized by the current-owner mutex. It
issues `u64::MAX` once by permanently setting the exhaustion bit; exact release clears only the
current witness and cannot reopen or wrap the allocator. Matching cancellation still retains both
reservation and admission, interrupted upload still moves into a retained close cursor, and
raster-table terminality remains ordered before World authority close.

Fixed 256 slots, eight probes, 256-byte keys, 16 MiB per item, 256 MiB aggregate, 16 KiB upload
pages, vacant-only staged insertion, independent scene/preview/operation freshness, and
staged-versus-live last-valid publication remain intact.

## Blocking mutation finding

The Rust mutation array contains 38 entries. Mutation 8 replaces every EngineCanvas
`gpu.reserve_engine_texture` call with `gpu.realize_without_reservation`
(`📦️glue.rs:5912`). The live `retained_raster_contract` checks all later validation/allocation
markers and exact handback branches, but it never requires the reservation call itself
(`📦️glue.rs:5800-5893`).

I reconstructed the Rust predicate and all-occurrence `String::replace` semantics in Bun against
the live four source strings. Results:

- baseline: **true**;
- mutations declared: **38**;
- mutations denied: **37**;
- surviving mutation: **8**, missing EngineCanvas pre-realization reservation.

This is discriminating: the mutated EngineCanvas still contains every later validation and
allocation marker, so each guarded-allocation predicate remains true even though the live
admission constructor has been erased. It is not an incidental parser result. The Rust test's
`assert!(!retained_raster_contract(...))` would evaluate false for that entry; no build was run
to execute it.

## Required repair packet

1. Make the permanent predicate require exactly one live
   `let admission = gpu.reserve_engine_texture(&key, width, height, candidate, expected)?` in
   `EngineCanvasPresenter::realize_one`.
2. Require that reservation index to precede every EngineCanvas full-tuple validation and every
   target/view/renderer/replacement allocation marker.
3. Retain mutation 8 unchanged and rerun a faithful all-occurrence reconstruction proving baseline
   true and **38/38** denied.
4. Preserve the repaired live tuple, returned/retained owner graph, permanent MAX allocator, and
   bounded cleanup source unchanged.

## Permitted gate evidence

| Gate | Independent result |
| --- | --- |
| Rust-2021 `rustfmt --check` on framework draw/GPU/glue, product WGPU glue, and EngineCanvas | **PASS**, exit 0 |
| Rust-2021 `rustfmt --emit stdout` parser checks on the same five files | **PASS**, exit 0 |
| Interactivity self-test and plain DENY | **PASS**, exit 0; one recorded allowlisted test-only blocking bridge, zero DENY findings |
| Faithful raster predicate reconstruction | **PASS** baseline |
| Faithful 38-mutation reconstruction | **REJECT**, 37/38 denied; mutation 8 survives |
| Immediate allocation-boundary scans | **PASS**, all named prepared/external validations immediately precede their allocation edge |
| Ownership/negative scans | **PASS**, exact Returned/Retained roots present and selected erasure/clone/map/legacy witnesses absent |
| Scoped working/staged/`HEAD` `git diff --check` | **PASS**, all three exit 0 |
| Whole working/staged/`HEAD` `git diff --check` | **Concurrent RED** only for unrelated trailing whitespace in `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md:459` and the staged prior raster audit's blank line at EOF on line 102 |

No runtime or build result is claimed. Phase 3 remains **RED** for this mutation blocker and the
separately reported prepared-pixel producer, atlas/icon/glyph, dynamic EngineCanvas surface,
Vello/render/submit/present timing, opaque GPU destruction, complete GpuContext/realm teardown,
semantic codec/input, and native/Wasm/browser matrix residuals.
