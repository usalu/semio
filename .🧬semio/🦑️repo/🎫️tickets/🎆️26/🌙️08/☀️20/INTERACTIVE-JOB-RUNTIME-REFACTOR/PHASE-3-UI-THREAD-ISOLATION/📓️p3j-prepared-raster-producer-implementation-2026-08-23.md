# P3j Prepared Raster Producer Implementation — 2026-08-23

## Source status

The Canvas/Paint prepared-pixel producer group is source-audit-ready, not accepted. The already accepted raster reservation/table/texture ownership remains intact; the only consumer-side change is a page-source adapter that gives the existing upload cursor one admitted row page without reconstructing a contiguous image.

## Implemented boundary

- Added a 16 KiB `PreparedRasterPage` owner and a retained `PreparedRasterProducer` in the framework prepared-render module.
- Added a fixed process admission ledger: 256 generations, 4,096 simultaneous items, 32 MiB aggregate retained bytes, and 16 MiB per raster. Admission uses checked arithmetic for the producer/source roots, both simultaneous key owners, every page owner, the exact page-slot backing, source backing capacity, and derived page bytes before page materialization.
- Made fixed-slot generation identity a `(slot, epoch)` witness. Frame generation is bound once before the producer enters `PreparedRenderInput`; stale generation is checked before source consumption.
- Made one worker job opportunity split exactly one row-aligned page of at most 16 KiB. The source backing retirement and final by-value handoff each have their own subsequent opportunities.
- Added `PreparedRenderUpload::RasterPages`. The accepted raster upload cursor reads exactly the retained page matching its current row and still uses the accepted raster operation witness for reservation/publication.
- Replaced Canvas/Paint's growable pending upload list with a fixed 16-owner FIFO per admitted surface. Queue capacity and terminal ownership are checked before image decode.
- Removed the second complete `pixels[..expected].to_vec()` owner. The codec-produced RGBA owner moves into the retained producer; page backing is created incrementally after admission.
- Changed the Interpreter's dimensions-only path to `ImageReader::into_dimensions`, avoiding a complete RGBA pixel materialization merely to read dimensions.
- Added retained rejection, saturation, stale-generation, cancel, and frame-close paths. Each close grant retires at most one page, source page, string scalar, backing allocation, metadata scalar, or credit. Empty page-slot and key backings are explicitly released before the ledger credit; terminal emptiness witnesses both releases.

## Files

Production/API:

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️gpu.rs`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Scenes/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`

Permanent verifier:

- `📜️script.ts`

Reports:

- `📓️p3j-prepared-raster-producer-census-2026-08-23.md`
- this report

## Fixtures and verifier mutations

Semantic source fixtures cover two-page one-opportunity production and pointer identity, stale generation before consumption, row cap plus one exact raw owner return, process bytes plus one exact backing return, mid-producer cancellation, slot ABA, one-owner retirement, and fixed FIFO 16/+1 handback.

The permanent interactivity predicate rejects 64 KiB pages, missing derived-key credit, missing page-slot backing credit, dynamic page slots, source clone, stale validation after consumption, growable Canvas queues, decode before queue preflight, frame-generation erasure, the former contiguous pixel clone, missing ABA/bytes fixtures, and ordinary page-slot/key backing drop before credit release. All 14 adversarial mutations are required to fail while the live source passes.

## Permitted gates

- `rustfmt --edition 2021 --check` on all seven touched Rust source files after final backing-retirement reconciliation: exit 0.
- Before the final two backing-retirement mutations, `bun 📜️script.ts verify interactivity --self-test` and plain DENY both passed twice: exit 0, one approved process-entry finding, zero unlisted findings.
- The final shared-baseline rerun after a concurrent P1t edit exits 1 on one unrelated unlisted finding: `DB engine replay_history removal did not reduce the seven residual waits to exactly six`. The prepared-raster self-tests and all 14 mutations complete without a prepared-raster finding before that aggregate DENY failure. This packet does not edit the DB engine.
- Canvas selected-path forbidden scan: zero `pixels[..expected].to_vec`, growable pending-raster queue, `PendingRasterUpload` reconstruction, or producer source clone matches.
- Exact residual scan: one Infinite World `pixels.to_vec()` prepared-raster constructor remains at `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs:139`.
- Scoped working, staged, and combined-HEAD diff checks for the packet paths: exit 0 after the final reports and verifier mutations.
- Whole working-tree `git diff --check`: exit 0.
- Whole staged and combined-HEAD checks: exit 2 only for the concurrent prior raster audit report blank EOF and `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md` trailing whitespace; neither file was edited by this packet.

No Cargo, Nx, Wasm, browser, runtime, network, root lint, or git-modifying command ran.

## Honest residuals

- The current `image` crate PNG/JPEG API still materializes one complete codec-owned RGBA backing before this producer can accept it. Its public PNG/JPEG decoders do not expose page/rect decode. This is an indivisible semantic-codec residual, not claimed bounded.
- Infinite World still clones cached reference pixels into the legacy contiguous raster upload at its sole prepared constructor. Its public byte-apply entrypoint has zero in-repository callers, but the source route remains reachable and is the next adjacent producer packet.
- Atlas, icon, glyph, surface, Vello, presenter/platform submission, GPU/cache/realm, and broader runtime residuals remain RED exactly as recorded by the accepted Phase 3 audits.
- Runtime behavior was not built or executed under this source-only restriction. Independent source audit is still required.

## Five-blocker repair checkpoint

This follow-up repairs the five source blockers from
`📓️sol-independent-p3-prepared-raster-producer-audit-2026-08-23.md`. The packet is
audit-ready, not independently accepted.

- `PreparedRasterReservation::try_reserve_source` claims a fixed ledger slot and the declared
  source/key ownership before dimensions, hashing, or codec materialization. `claim` resizes that
  generation-tagged credit to the exact row-page/item/backing budget before RGBA decode.
- Canvas no longer performs either whole-buffer digest. A stable prepared key is requeued through
  the admitted authority; the sole codec `Vec<u8>` moves into `PreparedRasterPages` without
  split, clone, or reconstruction.
- Interpreter moves the codec-owned RGBA backing directly into the reservation.
  `encode_rgba_png_data_url`, `pixels.to_vec()`, and the RGBA→PNG/base64→RGBA round trip are
  absent. Its five semantic routes remain: fetched apply, cached SVG, new SVG, cached URL prepared
  key reuse, and inline PNG/JPEG.
- `PreparedRenderJob` checks cancel/generation/yield, consumes one fuel unit, and only then
  advances one raster page/scalar. Zero fuel or an expired deadline moves no page/backing.
- Exact fixtures cover 16 MiB/operation, 4,096 aggregate items, 32 MiB aggregate bytes, 256
  generation slots, 16 KiB pages, and FIFO 16/+1.
- FIFO checkout leaves the producer in its slot; take moves it once and checked-out Drop hands the
  exact generation-tagged slot back. Stale/ABA handback is rejected.
- `PendingRasterAuthorityClose` is registered in `OsHostRetirement` after frame-build close and
  before events/presenter/runtime. Terminal requires pending state and the detached close owner to
  be empty before ledger release.

### Census and verifier

The scope remains three Scene producers and five Interpreter semantic routes. Four Interpreter
routes invoke a producer helper; cached URL reuses its already prepared key rather than creating a
second producer. Infinite World remains out of scope.

The permanent predicate now reads the producer, draw/GPU consumer, Canvas authority, Interpreter,
renderer glue, and host close owner together. Its 24 mutations independently alter all fixed caps;
remove source/page credit; restore dynamic pages, whole backing clone, hash scan, contiguous clone,
or Interpreter round trip; move stale/yield checks after consumption; erase checkout/frame
generation; remove boundary fixtures; bypass backing retirement; and erase realm/host close.

### Permitted gates after repair

- edition-2021 `rustfmt --check` on the six touched Rust files: **PASS**, exit 0.
- `bun 📜️script.ts verify interactivity --self-test --format json`: **PASS**, exit 0; all 24
  prepared-raster mutations rejected and live source accepted.
- `bun 📜️script.ts verify interactivity --format json`: **PASS**, exit 0; DENY clean.
- Selected scan for `pixels.to_vec()`, `encode_rgba_png_data_url`, `source.split_off`, and the
  removed digest calls: **PASS**, zero matches.
- Scoped working and staged `git diff --check`: **PASS**, exit 0.
- Cargo, Nx, Wasm, browser, runtime, network, and root lint: **not run by instruction**.

### Remaining RED

External image/SVG codecs still materialize one codec-owned backing. Infinite World,
atlas/icon/glyph/surface/Vello ownership, platform render/submit timing, broader caches, and the
full browser runtime gate remain RED. Rust fixtures were not compiled or executed. Independent
source re-audit is mandatory.

## Five-blocker independent-rejection repair — 2026-08-23

This narrow follow-up repairs the two live blockers identified by
`📓️sol-independent-p3-prepared-raster-producer-five-blocker-reaudit-2026-08-23.md`. It is
source-audit-ready, not accepted.

### Live source ownership

- Inline SVG now enters `queue_canvas_image_upload_with` with the stable `ui-image`/node key and
  claims its fixed generation slot plus source/key workspace before `parse_svg_data_url_bytes`, SVG
  dimension probing, or pixel decode. The former `ui_image_digest` whole scan and digest cache are
  deleted. Base64 and plain/percent-decoded SVG remain distinct semantic routes and their admitted
  parsed backing is moved into the producer.
- Dimensions and decode closures now return exact owners. Dimension failure, claim saturation,
  decoder fault, output validation, and finalization retain the parsed/encoded source; postdecode
  failure retains both that source and the RGBA owner. No captured `RefCell<Vec<_>>` can unwind at a
  failed resize.
- The fixed reservation holds `2 * declared source bytes` from initial admission through terminal
  publication: one externally retained source authority plus one exact parsed/encoded workspace.
  `claim_with_retained` validates workspace capacity, preserves those bytes, and adds the exact RGBA
  backing, page slots, two key owners, and fixed owner items. Resize can no longer erase source
  credit.
- The redundant Scene `published_key()` clone and `src_key` allocation are removed. Finalization
  creates the sole published key while moving the original key into the producer, matching the
  admitted `2 * key.capacity()` census.
- `PreparedRasterProducer` and `PreparedRasterRejected` retain both decoded and encoded/parsed
  backings. Completion, cancellation, fault, FIFO rejection, and realm close retire at most one
  16 KiB logical source page or one backing/key/credit owner per governed grant before terminal.

The exact census remains three Scene producers and five Interpreter semantic routes: fetched bytes,
inline base64 SVG, inline plain/percent SVG, cached URL-key reuse, and inline PNG/JPEG.

### Permanent evidence

- `raster_simultaneous_source_decode_peak_exact_and_plus_one` exercises the live global ledger at
  the exact 32 MiB coexistence peak and rejects one additional source byte before decode.
- `retained_codec_source_moves_once_and_retires_one_page_per_governed_step` preserves the parsed/
  encoded backing pointer and advances it under `StepContext` one 16 KiB opportunity at a time.
- `inline_svg_saturation_rejects_before_parse_or_source_copy` holds all 256 generation slots and
  proves the live inline-SVG parse counter remains zero.
- The permanent predicate now has 30 prepared-raster mutations. New discriminators reject lost
  source-on-resize credit, pre-reservation SVG parse, reintroduced SVG digest, ordinary retained
  source release, rejected-source erasure, and a redundant publication-key clone. The live predicate
  also requires exact owner-returning dimensions/decode and the monotonic simultaneous credit
  expressions.

### Permitted gates

- Edition-2021 `rustfmt --check` on the three repaired Rust files: **PASS**, exit 0.
- The wider seven-file check reaches concurrent product-WGPU `glue.rs` formatting drift from the P2
  progress-overlay packet; the three repaired files remain clean and this packet did not format or
  edit that shared file.
- `bun ./📜️script.ts verify interactivity --self-test --format json`: **PASS**, exit 0; all 30
  prepared-raster mutations reject and DENY is clean.
- `bun ./📜️script.ts verify interactivity --format json`: **PASS**, exit 0; one approved
  process-entry finding and zero unlisted findings.
- Exact selected scans: zero `ui_image_digest`, SVG digest cache, redundant publication-key API,
  `src_key`, selected-path `pixels.to_vec`, PNG/base64 roundtrip, producer clone, or split matches.
- Scoped and whole working/staged/combined-HEAD `git diff --check`: **PASS**, exit 0.
- Cargo, Nx, Wasm, browser, runtime, network, and root lint: **not run by instruction**.

### Honest residuals

The already documented external image/SVG codecs remain indivisible whole-codec operations after
admission; no runtime timing claim is made. Infinite World retains its separate cached-pixel clone.
Atlas/icon/glyph/surface/Vello ownership and broader renderer/browser runtime gates remain RED.
Independent source re-audit is required before acceptance.
