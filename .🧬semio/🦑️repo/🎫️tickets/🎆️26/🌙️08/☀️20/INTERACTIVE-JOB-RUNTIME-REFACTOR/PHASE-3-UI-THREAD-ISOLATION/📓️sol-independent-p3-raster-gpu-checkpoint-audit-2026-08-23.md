# Sol Independent P3 Raster GPU Checkpoint Audit — 2026-08-23

## Verdict

**REJECT — narrow source packet.** The fixed raster registry, page-sized upload, staged/live
publication, presenter ACK/abort integration, and presenter-before-World close order are present,
but the live packet does not meet the exact admission/freshness/retirement ownership contract and
does not pass its complete scoped rustfmt/evidence gate.

This was an independent Sol High source audit. I made no production edit; this report is the only
file authored by the audit. Cargo, Nx, Wasm, browser, network, root lint, and runtime timing were not
run.

## Blocking Findings

### 1. EngineCanvas transfers and realizes GPU owners before raster admission

`EngineCanvasPresenter::realize_one` creates or replaces the surface texture, performs the Vello
render, clones the published view, moves the rendered texture out of the surface, and creates a
replacement texture before calling `GpuContext::stage_engine_texture`. The staging API consumes the
`wgpu::Texture` and only then checks closing state, candidate occupancy, key length, live probes,
and staged probes in `RasterTextureTable::stage_gpu_bind_group`.

There is no retained `RasterTextureAdmission`/reservation token or equivalent preflight seam that
can be obtained before texture ownership and GPU work. Rejection returns the texture, but it cannot
make the required cap/key preflight precede ownership transfer/realization.

The view path is also not exact owner handback. The staging API borrows `&wgpu::TextureView` and
returns only `wgpu::Texture`; EngineCanvas first clones the old view, overwrites and drops the
original field owner, then reinstalls the clone on rejection. This preserves the same underlying
GPU view handle, but it does not transfer and return the exact Rust view owner requested by the
packet, and no fixture checks view-owner identity.

### 2. Operation freshness is not independently enforced

Raster entries and upload cursors carry `scene_revision`, `preview_generation`, and `operation`,
but the live route does not independently reject a stale operation:

- `ensure_raster_step` rejects only a lexicographically older revision/generation pair. Operation
  participates in exact duplicate/cursor equality, not a stale/fresh ordering check.
- `stage_gpu_bind_group`, the EngineCanvas route, calls `live.locate(key)` but never inspects the
  live entry's scene revision, preview generation, or operation before staging a replacement.

Thus an older operation can replace a newer live operation when the other witness values match,
and the EngineCanvas route has no live freshness comparison at all. Merely storing the operation
number is not an independent freshness authority.

### 3. Retirement is not one field/scalar per grant

`RasterTextureRetirementOwner::new` consumes `RasterTextureEntry<RasterTexture>` but retains only
the key, bind group, texture, and a two-step scalar counter. The entry's scene revision, preview
generation, operation, width, and height are discarded together during construction. At retirement
completion, candidate and presenting pairs are also both cleared in the same grant before the
retirement cursor is removed.

Bind-group and texture release are individually stepped, and table scanning is one slot per call,
but the full retained entry/witness does not have one-field retirement. The terminal predicate
therefore cannot witness every scalar owner being retired one grant at a time.

### 4. Permanent evidence and rustfmt claims do not match the live packet

The permanent `raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete` array has
**9** mutations, not the implementation report's claimed **10**. All 9 authored mutations are
denied by its predicate, but it has no vacant-insertion mutation and cannot mutate EngineCanvas
handback because `EngineCanvas/🧊️component.rs` is not an `include_str!` input. It also has no
256-byte-key mutation. The predicate can therefore pass while exact view return or key admission is
erased.

Edition-2021 rustfmt and parser checks pass for framework `draw.rs`/`gpu.rs` and product
`glue.rs`/`os_host.rs`. EngineCanvas parses under edition 2021 but its scoped `rustfmt --check`
fails on live import ordering and several existing compact expressions. Because EngineCanvas is a
modified production member of this packet, the complete scoped formatting gate is RED.

## Contracts That Pass by Source Inspection

| Contract | Result | Evidence |
| --- | --- | --- |
| Fixed registry | PASS | `RASTER_TEXTURE_TABLE_CAPACITY = 256`, eight probes, 256-byte inline UTF-8 keys, and boxed fixed slots; the production raster table has no `HashMap<String, RasterTexture>`. |
| Bounded upload opportunity | PASS | Exact checked `width * height * 4` admission; one row must fit; each `write_texture` slice contains whole rows totaling at most 16 KiB. |
| Staged/live last-valid behavior | PASS | Lookup exposes matching staged entries only during the exact presenting witness and otherwise falls back to live. Abort scans staged only; commit moves matching staged entries and retains the displaced live owner. |
| Presenter ordering | PASS | `begin_presenting` precedes `render_prepared`; exact prepared ACK constructs commit retirement; faults and stale/missing witnesses enter abort retirement. |
| Texture handback | PARTIAL | Every explicit `stage_gpu_bind_group` rejection returns the exact texture, and EngineCanvas reinstalls it. Exact view-owner handback fails as described above. |
| Vacant staged insertion | PASS | Engine staging uses `insert_vacant` after locating the slot and returns the exact texture if the slot changed; there is no debug-only overwrite path in this staging method. |
| Close before World | PASS | `OsHostRetirement::close_step` requires `presenter.close_world_owners_step()` and its terminal witness before `runtime.close_world3d_dynamic_step()`. Raster table close and terminal-empty participate in the presenter witness. |

## Independent Probes and Mutations

An independent requirement-level probe scored **8/11**. Fixed registry, bounded page write,
staged/live last-valid, begin-before-render, ACK commit, abort retention, texture restoration, and
vacant staging passed. Preflight-before-texture-ownership, independent operation freshness, and
one-field retirement failed.

A separate ten-mutation structural probe denied **10/10** mutations for capacity, page limit,
candidate generation, cursor operation equality, vacant staging, begin, commit, abort,
presenter-before-World close, and EngineCanvas texture restoration. That result proves those exact
strings are represented, but does not override the three requirement-level failures. The permanent
repository fixture itself remains only **9/9 authored mutations**, despite the report's 10/10 claim.

## Executed Gates

| Gate | Result |
| --- | --- |
| framework raster scoped rustfmt, edition 2021 | PASS: `draw.rs`, `gpu.rs` |
| product presenter scoped rustfmt, edition 2021 | PASS: `glue.rs`, `os_host.rs` |
| EngineCanvas scoped rustfmt, edition 2021 | **FAIL** |
| edition-2021 parser | PASS: all five audited source files |
| independent requirement probe | **FAIL: 8/11** |
| independent mutation probe | PASS: 10/10 denied |
| permanent raster mutation census | **FAIL: 9 authored, report claims 10**; 9/9 authored denied |
| interactivity verifier self-test | PASS: DENY mode clean; one approved blocking-bridge record |
| interactivity plain run | PASS: DENY mode clean; one approved blocking-bridge record |
| dynamic raster map/replacement negative scan | PASS for the scoped WGPU raster table |
| scoped working/staged/HEAD diff checks | PASS |
| whole working/staged/HEAD diff checks | PASS |
| Cargo/Nx/Wasm/browser/runtime | not run; unverified |

## Required Repair

1. Add a generation-keyed raster reservation that validates closing/candidate state, fixed key,
   probe capacity, and live freshness before EngineCanvas creates/transfers a texture or view.
   Consume that exact reservation when staging and return both exact owners on rejection.
2. Define and enforce independent operation freshness on both prepared-raster and EngineCanvas
   routes, including stale/duplicate/ABA fixtures that distinguish operation from scene revision
   and preview generation.
3. Retain every entry and witness field through a one-field close cursor; terminal-empty must cover
   the freshness/dimension scalars as well as GPU handles.
4. Format EngineCanvas under the manifest-selected edition and extend the permanent fixture to the
   actual ten-mutation cohort, including vacant insertion and exact texture/view handback (plus a
   discriminating key-cap mutation).

## Remaining RED Areas

This rejection is narrow and does not regress the previously accepted presenter-freshness packet.
Phase 3/5 remains RED for producer-side prepared pixel `Vec` paging, atlas/icon/glyph ownership and
whole-buffer writes, dynamic EngineCanvas surfaces, Vello realization/render timing, complete
`GpuContext`/realm retirement, platform submit/present and opaque GPU destruction, semantic codecs,
Cargo/type validation, native/Wasm/browser behavior, and runtime timing.
