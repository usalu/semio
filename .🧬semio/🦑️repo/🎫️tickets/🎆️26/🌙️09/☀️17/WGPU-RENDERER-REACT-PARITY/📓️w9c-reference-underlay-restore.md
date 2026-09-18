# W9c — Reference underlay restore

## Root cause

W9c’s credit fix moved reference pixels out of `World3dState` with a one-shot
`mem::take` on `reference_pixels` (`take_reference_underlay_upload`). After the
first `ensure_world_plane_texture` call per surface, the decoded buffer was empty.
Later renders still published `textured=1` (the quad is unconditional) but offered
no raster producer, so `RasterTextureTable::get` missed and
`encode_prepared_world_textured` drew nothing. A single aborted present then
never healed because the payload could not be re-offered.

## Fix (world lane only)

`🧰️framework/…/♾️infinite/🌍️world/🦀️.rs`:

- `reference_underlay_upload` — **clone** width/height/pixels every render; state
  keeps the decode for sibling panes and for re-offer after
  `abort_presented_step`.
- `ensure_world_plane_texture` — take pixels **by value** into the producer (no
  per-frame `to_vec` on a borrow); refused `PreparedRasterProducer::try_admit`
  is retired locally (not `World3dBuildRejected::RasterAdmission`).
- `visible_references` pre-collection — avoids E0502 when offering uploads.

Law: `a_reference_underlay_is_offered_every_render_and_a_refused_admission_is_not_a_fault`.

## Verification run (2026-09-18)

| Step | Result |
|------|--------|
| `cargo check -p semio-framework-os-infinite --lib -j 4` | OK |
| `cargo test … a_reference_underlay_is_offered_every_render…` | OK |
| `framework-renderer-wgpu:wasm` (`--skip-nx-cache`) | OK after minimal PDF import fixes (peer breakage, unrelated to underlay) |
| `activate-puzzle3d-wgpu-dev` | OK after registering `🎞️frame-worker/🧩️lazy-install/🟦️.ts` in `wgpu-frame-worker` `sourceModulePaths` |
| `SEMIO_PROBE_OUT=underlay` probe vs stale :6213 | No plan (old worker); probe vs rebuilt worker not reached |

## Probe

When `activate-puzzle3d-wgpu-dev` is green again:

```bash
cd <ticket> && SEMIO_PROBE_OUT=underlay bun 🐍️w9b-projection-and-framing-probe.mjs
```

Expect floor plan in `🗑️generated/underlay/boot.png` (both panes), matching
`🗑️generated/w8b-boot-2/final.png`.

## Collateral (wasm unblock only)

- `stdio/pdf` 1.4 `🚪️io` — import `Lexer` / `decode_stream` from 1.7 modules.
- `stdio/pdf` 1.7 outline inference — `p.text()` not `p.text`.
