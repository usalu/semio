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
- Added retained rejection, saturation, stale-generation, cancel, and frame-close paths. Each close grant retires at most one page, source page, string scalar, metadata scalar, or credit.

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

The permanent interactivity predicate rejects 64 KiB pages, missing derived-key credit, missing page-slot backing credit, dynamic page slots, source clone, stale validation after consumption, growable Canvas queues, decode before queue preflight, frame-generation erasure, the former contiguous pixel clone, and missing ABA/bytes fixtures. All 12 adversarial mutations are required to fail while the live source passes.

## Permitted gates

- `rustfmt --edition 2021 --check` on all seven touched Rust source files: exit 0.
- `bun 📜️script.ts verify interactivity --self-test` and plain DENY both passed twice after the prepared-raster predicate landed: exit 0, one approved process-entry finding, zero unlisted findings.
- The final shared-baseline rerun after a concurrent P1t edit exits 1 on one unrelated unlisted finding: `DB engine replay_history removal did not reduce the seven residual waits to exactly six`. The prepared-raster self-tests and 12 mutations complete without a prepared-raster finding before that aggregate DENY failure. This packet does not edit the DB engine.
- Canvas selected-path forbidden scan: zero `pixels[..expected].to_vec`, growable pending-raster queue, `PendingRasterUpload` reconstruction, or producer source clone matches.
- Exact residual scan: one Infinite World `pixels.to_vec()` prepared-raster constructor remains at `world/🦀️component.rs:139`.
- Scoped working, staged, and combined-HEAD diff checks for the packet paths: exit 0.
- Whole working-tree `git diff --check`: exit 0.
- Whole staged check: exit 2 because the staged census snapshot still has a blank EOF and two concurrent unrelated files have whitespace findings. The combined-HEAD view contains the census correction.
- Whole combined-HEAD check: exit 2 only for the concurrent prior raster audit report blank EOF and `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md` trailing whitespace; neither file was edited by this packet.

No Cargo, Nx, Wasm, browser, runtime, network, root lint, or git-modifying command ran.

## Honest residuals

- The current `image` crate PNG/JPEG API still materializes one complete codec-owned RGBA backing before this producer can accept it. Its public PNG/JPEG decoders do not expose page/rect decode. This is an indivisible semantic-codec residual, not claimed bounded.
- Infinite World still clones cached reference pixels into the legacy contiguous raster upload at its sole prepared constructor. Its public byte-apply entrypoint has zero in-repository callers, but the source route remains reachable and is the next adjacent producer packet.
- Atlas, icon, glyph, surface, Vello, presenter/platform submission, GPU/cache/realm, and broader runtime residuals remain RED exactly as recorded by the accepted Phase 3 audits.
- Runtime behavior was not built or executed under this source-only restriction. Independent source audit is still required.
