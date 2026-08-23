# Independent Sol Audit — P3 Prepared Raster Producer — 2026-08-23

## Verdict

**REJECT (source-only).** The packet correctly removes the selected Canvas queue's second
`pixels[..expected].to_vec()` and introduces a useful paged handoff, but it does not yet satisfy
pre-materialization admission, governed StepContext advancement, exhaustive fixed-cap evidence, or
terminal ownership. No production source was edited during this audit.

Infinite World's cached `pixels.to_vec()` constructor and the external PNG/JPEG semantic codecs
remain honest RED residuals. They are not counted as defects in this verdict. The defects below are
inside the selected Canvas/Interpreter producer boundary.

## Exact caller and definition census

The production call census is complete. `queue_canvas_image_upload` has exactly eight in-scope
callers, excluding its definition and the four `#[cfg(test)]` calls:

| Owner | Line | Route |
| --- | ---: | --- |
| Scene | `Scenes/🧊️component.rs:1941` | Canvas image layer |
| Scene | `Scenes/🧊️component.rs:4158` | Paint/image layer |
| Scene | `Scenes/🧊️component.rs:6433` | Ink image block |
| Interpreter | `Interpreter/🧊️component.rs:1513` | fetched UI image apply |
| Interpreter | `Interpreter/🧊️component.rs:1638` | cached inline SVG |
| Interpreter | `Interpreter/🧊️component.rs:1650` | newly rasterized inline SVG |
| Interpreter | `Interpreter/🧊️component.rs:1658` | cached URL image |
| Interpreter | `Interpreter/🧊️component.rs:1680` | inline PNG/JPEG image |

No other framework Rust caller was found. The remaining legacy prepared-raster constructor is the
declared Infinite World residual at `infinite/🌍️world/🦀️component.rs:139`.

## Source invariants that are present

- `PreparedRasterProducer` declares 16 KiB pages, 256 ledger slots, 4,096 aggregate items, 32 MiB
  aggregate bytes, and 16 MiB per operation.
- Page slot storage is allocated to the exact derived page capacity after ledger admission. Each
  producer call splits at most one row-aligned page, and the selected Canvas queue no longer clones
  `pixels[..expected]`.
- The Canvas surface queue is a fixed sixteen-owner FIFO and returns its seventeenth producer.
- `PreparedRasterPages` carries frame generation by value into `RasterPages`; the accepted GPU
  consumer checks it against packet preview generation and reads one retained page for the current
  row.
- The dimensions-only Interpreter probe uses `ImageReader::into_dimensions`; it does not call
  `to_rgba8`.
- The existing close routine orders page owners, key characters/backing, page-slot backing,
  metadata, and ledger credit when that routine is actually driven to completion.

## Blocking findings

### 1. Aggregate admission occurs after full pixel materialization and traversal

`queue_canvas_image_upload` checks only surface FIFO availability, then calls
`decode_canvas_image(data_url)` and hashes the entire decoded pixel slice. Only afterward does
`PreparedRasterProducer::try_admit` reserve the 256-slot/4,096-item/32-MiB ledger credit. A saturated
process therefore still base64-decodes, image-decodes, allocates the complete RGBA backing, and
full-scans it before rejection. The admitted page authority is preflighted before page creation,
but the producer operation is not reserved before source materialization as required.

The raw source digest also scans the complete data URL before the FIFO preflight. Neither digest is
a codec-owned indivisible operation or a retained StepContext cursor.

### 2. Live Interpreter callers retain extra whole-raster copies outside the codec residual

`apply_ui_image_bytes` and the inline-SVG route create an RGBA `Vec`, then
`encode_rgba_png_data_url` calls `pixels.to_vec()` to create another complete RGBA owner, encodes a
whole PNG/base64 owner, and finally routes it through `queue_canvas_image_upload`, which decodes a
new complete RGBA owner. The sole external decoder backing is an accepted residual; this
encode/clone/decode round trip is not. Consequently the five Interpreter caller paths do not meet
the claimed no-whole-raster-clone/materialization boundary.

### 3. Raster page work bypasses StepContext fuel and deadline

`PreparedRenderJob::step` checks cancellation and generation, then calls
`producer.step(cx.generation().0)` before consulting `cx.should_yield()`. The producer has no
`StepContext` parameter, does not consume fuel, and performs `split_off`, boxed-page creation, and
page publication even when the step entered with zero fuel or an expired deadline. The later
measurement loop observes StepContext, but the raster producer path returns before reaching it.
There is no zero-budget/deadline fixture or mutation. One page per function call is not equivalent
to one governed work grant.

### 4. Claimed fixed caps do not have exact boundary or mutation evidence

The permanent predicate requires 16 KiB pages, 256 slots, and 32 MiB bytes, but does not require
`PREPARED_RASTER_PRODUCER_ITEMS` or `PREPARED_RASTER_ITEM_BYTES`. Its fourteen mutations cannot
detect removal or inflation of the 4,096-item or 16-MiB-per-operation limits, nor replacement of the
256-slot cap with an untested value.

The Rust fixtures cover row width `+1`, a single oversized-capacity source against process bytes,
FIFO sixteen `+1`, and reuse ABA. They do not prove exact success and `+1` handback for 16 MiB per
operation, 4,096 simultaneous items, or 256 live generation slots. Thus the report's exhaustive
cap/+1 claim is not supported.

### 5. Global queued owners and ordinary Drop bypass terminal close

`PENDING_RASTER_STATE` can own up to 256 surfaces, each with sixteen producers plus rejected and
closing owners. Production references only enqueue, frame-cursor drain, and lookup operations.
There is no surface/realm close pump for this authority; `PendingRasterUploadCursor::close_step`
drains only its local `closing` producer. `close_all` is test-only.

`PreparedRasterProducer`, `PreparedRasterPages`, `PreparedRasterRejected`, `PendingRasterQueue`,
and `PendingRasterSurface` also have ordinary implicit Drop paths. Ledger credit is released only
by `PreparedRasterPages::retire_metadata_step`; ordinary Drop frees page/key/source owners without
calling `PreparedRasterLedger::release`, leaving the fixed slot and aggregate credits occupied.
This fails the required cancellation/fault/realm-close terminal witness even though the explicit
happy close routine has the right local order.

## Mutation reconstruction and gates

| Gate | Result |
| --- | --- |
| edition-2021 `rustfmt --check` on all seven implementation Rust files | **PASS** |
| exact production caller scan | **PASS**: 3 Scene + 5 Interpreter; no omitted in-scope caller |
| selected Canvas clone/growable-queue scan | **PASS**: zero old `pixels[..expected].to_vec`, `PendingRasterUpload`, or growable pending queue |
| dimensions-only scan | **PASS**: `into_dimensions`, no RGBA in that probe |
| exact existing mutation enumeration | **PASS**: fourteen entries |
| isolated permanent raster verifier | **PASS**: live baseline accepted and all 14 authored mutations rejected |
| broad interactivity self/plain DENY after P1 stabilization | **PASS**: one allowlisted test-only bridge and two predeclared future records; zero unlisted findings |
| scoped working/staged/HEAD diff checks | **PASS** |
| Cargo/Nx/Wasm/browser/runtime/network | **not run by instruction** |

Passing the existing mutation matrix does not resolve findings 1–5 because the matrix has no
discriminator for them.

## Required repair packet

1. Split reservation from source admission. Reserve exact slot/item/byte/key/page credits from
   dimensions and declared source capacity before pixel decode/materialization; return the exact
   token and source owner on every failure.
2. Move Interpreter raster/SVG pixels directly into the reserved producer. Remove the
   RGBA-to-PNG/base64-to-RGBA round trip and `pixels.to_vec`; keep external codecs only as the
   explicit one-backing semantic residual.
3. Make producer advancement accept `&mut StepContext`, yield before work on zero fuel/expired
   deadline, consume exactly one unit for one page/scalar, and recheck cancellation/deadline after
   the unit. Cursorize both full-image digests under the same authority.
4. Add exact success/`+1`/pointer-handback tests and mutations for 16 KiB page, 16 MiB operation,
   256 live slots, 4,096 aggregate items, 32 MiB aggregate bytes, and FIFO sixteen. Mutate every
   constant independently and add zero-budget/deadline/cancel between every phase.
5. Register `PENDING_RASTER_STATE` with surface/realm close. Detach and retire one producer page,
   source backing, key scalar/backing, page-slot backing, metadata scalar, and finally ledger credit
   per grant. Make ordinary abandonment structurally shallow or terminal-asserted so no live credit
   or deep owner can bypass the close witness.

After repair, rerun the same source gates and an independent audit. Phase 3, Infinite World,
semantic codecs, presenter/platform submit timing, and runtime evidence remain RED.
