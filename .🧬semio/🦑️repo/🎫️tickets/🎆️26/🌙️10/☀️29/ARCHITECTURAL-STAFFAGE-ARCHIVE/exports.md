# Staffage Exports: Capability Matrix and Limits

Root `S` = `🎓️teaching/🏛️architecture/🧍️staffage`. Modules under `S/🔨️modules/📤️export/`, tests under `S/🧪️tests/`, fixtures under `S/🧫️fixtures/`.

## Capability matrix

| Format | Module | Browser | Node CLI | Status | Verification (independent reader) |
|---|---|---|---|---|---|
| SVG | `🖼️svg` (depiction worker) | yes | yes (one file per frame) | working | rsvg-convert (used for every raster export) |
| PDF 1.7 | `📕️pdf` `writePdf(pages, control)` | yes, pure TS, sync | yes (one page per frame) | working | `🧪️tests/📕️pdf-reopen`: qpdf --check clean; pdfinfo -box MediaBox = mm x 72/25.4 pt (2 pages of different size); pdftoppm 254 dpi: 1.80 m figure at 1:100 = 18 mm +-0.2 mm; hatch tiling pattern and 0.5 opacity render; pdftotext recovers "1,80 m · 2,5 m² 20 °C -5 <= ü"; Helvetica/Helvetica-Bold advances equal reportlab AFM metrics; deterministic bytes; progress + abort |
| DXF R2000 (AC1015) | `📐️dxf` `writeDxf(scene)` | yes, pure TS | yes (one file per frame) | working | `🧪️tests/📐️dxf-reopen` (ezdxf 1.4.5 via uv): version AC1015, $INSUNITS 6, $MEASUREMENT 1, `audit()` = 0 errors and 0 fixes, $EXTMIN/$EXTMAX = model bounds +-1e-6 = `ezdxf.bbox` of the polylines, layers ACI + snapped lineweight, polylines closed flag + coordinates, TEXT decoded; also a real depicted scene (walking-commuter standing, forces view) written by the CLI |
| DWG AC1024 | `🖊️dwg` `writeDwg(scene)` + Rust crate `📦️packages/🦀️rust` | via wasm (raw C-ABI) when `staffage_dwg.wasm` is present | via `staffage-dwg` CLI (`SEMIO_STAFFAGE_DWG_CLI`) or wasm | code complete, **not compiled yet** (see blocker) | TS wrapper tested (unavailable path, CLI-runner path, abort). Rust acadrust reopen test written (`🧪️tests/🖊️dwg-reopen/🦀️.rs`), **not yet run** |
| PNG sequence ZIP | `🎞️frames` `writePngSequenceZip` | yes (browser module `📤️downloads` rasterises) | yes | working | `🧪️tests/🗜️zip-reopen`: `unzip -t` clean, python zipfile `testzip()` None, all members stored, names `manifest.json` + `frame-000000.png`..`frame-000013.png`, PNG signature on each, manifest fps/count/times n/12/asset/view/scale |
| MP4 H.264 (CLI) | `🎬️cli` | no | yes: rsvg-convert -> ffmpeg libx264 yuv420p, `-r fps -fps_mode cfr` | working | `🧪️tests/🎞️frame-timing`: ffprobe h264, yuv420p, r_frame_rate 12/1, 14 frames read = 14 scheduled, duration within 1 frame, packet spacing 1/12 s |
| MP4 H.264 (browser, first party) | `🎥️video` `encodeClip(..,"mp4")` | yes (no MediaRecorder needed) | importable in Node | working, **uncompressed** | same test: 5 synthetic frames, ffprobe 10/1 fps, 5 frames, ffmpeg decodes the whole stream (`-f null`) |
| WebM VP9 + alpha (CLI) | `🎬️cli` | no | yes: libvpx-vp9 `yuva420p`, crf 30 | working | ffprobe: codec vp9, 12/1, 14 frames, `alpha_mode=1` tag; decoding with `-c:v libvpx-vp9` reports `yuva420p`; packet spacing within 1 ms (WebM time base is 1 ms) |
| WebM (browser) | `🎥️video` `encodeClip(..,"webm")` | MediaRecorder only: VP9, VP8, AV1 probed | no | code complete, **not exercised** (no browser run by this worker) | `videoCapabilities()` tested in Node (reports MediaRecorder codecs as unavailable, first-party MP4 as available) |

## Frame schedule

`frameSchedule(clip, fps)`: frame n at `t = n / fps` computed as a single division (exact, tested with `toBe`); looping clip `round(duration x fps)` frames, non-looping `+1`. The CLI uses the schedule for evaluation, rasterisation and the manifest, so file names, manifest times and video frames agree one to one. Raster frames use one fixed canvas aligned on each scene's world origin (`metadata.origin`) so the figure does not jump when scene bounds change; width and height are rounded up to even pixels for H.264.

## Honest limits

- **Transparency**: MP4/H.264 has no alpha; transparent pixels are composited over `--background` (default `#ffffff`) in the CLI and over the `background` option in the browser encoder. PNG sequence keeps alpha. CLI WebM keeps alpha (yuva420p; players must support VP9 alpha, ffprobe shows the `alpha_mode` tag and the pix_fmt is `yuv420p` unless decoded with libvpx-vp9). Browser WebM alpha is reported as `unverified` because it depends on the browser's encoder.
- **First-party browser MP4** is valid H.264 but uncompressed (`I_PCM`, repeats as `P_Skip`); size limits 4096 px edge, even dimensions, fps 1..120.
- **Browser WebM** is recorded in real time through MediaRecorder; the frame rate is only approximately constant. Use the CLI or PNG sequence when exact timing matters.
- **PDF**: uncompressed content streams (no Flate), Helvetica standard-14 with WinAnsi: `·`, `²`, `°`, `µ`, `×`, dashes map directly; `−` -> `-`, `≤` -> `<=`, `≥` -> `>=`, Greek letters spelled out, unknown glyphs `?`. Optional-content group per drawing layer; hatch fills are tiling patterns tinted darker than the fill; opacity through ExtGState; no creation date (deterministic bytes); the document `Title` is `en / de`, `Subject` carries asset, scale, time, view. No PDF/A or tagged structure.
- **DXF**: lineweights are snapped to the 24 values AutoCAD admits; layer colour must be ACI 1..255; non-ASCII text is written as `\U+XXXX` (decoded by AutoCAD and ezdxf's `decode_dxf_unicode`); only LWPOLYLINE and TEXT; no plot styles; `ACAD_LAYOUT` has Model and Layout1.
- **DWG**: the in-repo writer's `DwgLayer` only has name and ACI colour, so insertion units (metres, `header.policy.insertion_units = 6`) and layer lineweights (nearest entry of the DWG lineweight table) are patched into the snapshot before `encode_dwg`; entities: LWPOLYLINE (no bulges) and single-line TEXT (rotation 0, left aligned). Text style and linetype are the writer's defaults (Standard, Continuous); dashed linetypes are not written.

## DWG blocker (repo infrastructure, not the crate)

The crate `S/🔨️modules/📤️export/🖊️dwg/📦️packages/🦀️rust` (CLI `staffage-dwg <in.json> <out.dwg>`, rlib + cdylib with raw C-ABI `staffage_alloc/free/encode/result_ptr/result_len`, no wasm-bindgen) is its own Cargo workspace (own `Cargo.lock`, `cargo-features = ["trim-paths"]`) and depends by path on the in-repo AC1024 writer crates. `repo cargo --store=shared --configuration=prototype -- build --manifest-path <crate>/Cargo.toml` is refused before compilation by the source admission gate:

```
Rust crate Source refused before compiler: semio-framework-os-store-engine
(... reason "bytes", path 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🧯️close-host-drop-and-pair-merge/🦀️.rs,
 observedBytes 18130, maximumBytes 13708, remainingFiles 21)
```

The same refusal happens for the existing writer crate itself (`semio-s-artifact-stdio-dwg-write`), so the AC1024 writer is currently not buildable through `repo cargo` regardless of this crate. Earlier attempts hit `Authorizer source closure capacity exceeded: files=1982/2048, bytes=16778094/16777216` in the repo tool itself. `repo tools exec cargo` is refused by design ("requires the repository launcher and authentic enforcing store"). Consequences: the Rust crate and its acadrust test have never been compiled here; `staffage_dwg.wasm` does not exist; `writeDwg` reports `unavailable` with the exact missing artifact (`.../🖊️dwg/staffage_dwg.wasm ... or a staffage-dwg CLI runner via configureDwg({ cli })`), the CLI exits 3 for `--format dwg`. Also unverified: that the writer's dependency graph (os-kernel) compiles for `wasm32-unknown-unknown`. Once the gate passes: `repo cargo --store=shared --configuration=prototype -- test --manifest-path <crate>/Cargo.toml --test dwg_reopen`, then build `--lib --target wasm32-unknown-unknown` and copy `staffage_dwg.wasm` next to `🖊️dwg/🟦️.ts`; the skipped vitest case `encodes through the wasm module into AC1024` then runs automatically.

## Integration notes for the coordinator

- The package barrel `📦️packages/🟦️typescript/🟦️.ts` does not yet export `writeDwg`, `dwgAvailability`, `configureDwg`, `DwgUnavailableError`, `frameSchedule`, `writePngSequenceZip`, `videoCapabilities`, `encodeClip`; the downloads module probes `library.writeDwg`, so it needs the DWG export added there.
- `🎞️frames` imports `writeZip` from `🎯️targets/⚛️react/🔨️modules/📤️downloads/🗜️zip` (reused, not duplicated); moving that file under `📤️export` would be the cleaner dependency direction.
- `🎥️video` imports the first-party writer from `🧰️framework/🔨️modules/🖌️raster/🎥️video/🟦️.ts` (pure TS, no imports).
- CLI usage: `repo tools exec bun -- S/🔨️modules/📤️export/🎬️cli/🟦️.ts --asset walking-commuter --clip walk-cycle --view dark-solid --scale 100 --fps 12 --format mp4 --out <dir> --lang en`; ffmpeg runs through `repo tools exec ffmpeg --`, `rsvg-convert` (not a registered repo tool) from PATH.
- Tests: `repo tools exec vitest -- run --config S/🧪️tests/🎚️config/🟦️.ts 📕️pdf-reopen 📐️dxf-reopen 🖊️dwg-reopen 🎞️frame-timing 🗜️zip-reopen` (17 passed, 1 skipped = the wasm case). Support helpers in `🧪️tests/🧰️support/🧰️.ts`; python oracles use `uv run --with ezdxf|reportlab`.
