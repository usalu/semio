# WP-RS1 — First-Party CPU Raster Tier (Every Guest), Starting With Shooting `photos:out`

Slice: RS1 (new in session 15). Coordinator: `main`. Goal outcome 1 (working os `s` frontend: raster exports in every guest tier).
Design: `📓️wp-s20.md` "Design — shooting photos:out" option A + coordinator decision (VectorScene input, first-party OpenType
reader over the ui `🔤️outline.ttf` faces, parley/swash + resvg as oracles only). Scripts/sets: `.tmp-ticket/wp-rs1/`.
Durable data: `.🧬semio/🌐hub/s15-rs1-*` (overlay `s15-rs1-overlay`, private build/target dirs inside it).

## Session 15

Started 2026-09-29 21:4x (Opus 5.5). Rules: `📓️session-15-preamble.md` (29–33) + session-14 rules 1–28. GUEST FREEZE ON.

| # | Item | State | Evidence |
|---|---|---|---|
| 1 | survey reusable first-party pieces | **done 21:5x** — reuse: stdio's own drawing→png CPU rasterizer (exact-area accumulation, resvg-oracled) becomes the framework tier; draw's png already routes through that leaf ("one png writer"); `🔲️pixels` RasterImage/PNG; `🧵️job` fuel model; ui T7a/T7b corpus | log 21:5x |
| 2 | first-party OpenType reader `semio-framework-fonts` (`🧰️framework/🔨️modules/🔤️fonts`, NEW dir) — cmap 4/12, hmtx, glyf/loca (simple + composite), GPOS kern (PairPos 1/2, ext 9) + legacy kern, embedded Anta/Share Tech Mono, `text_run` | **8/8 laws PASS 21:52** (overlay): cmap+advances = swash U+0020–024F both faces; EVERY Latin-1 pair kerning = swash-shaped (ui `PairKerning::em` reading); WG11 corpus 144 rows = swash per scalar/pair AND Chromium ±0.5 px; every glyph outline = swash scaler (area + bounds) | `.🧬semio/🌐hub/s15-rs1-logs/fonts-test-1.txt` |
| 3 | CPU tier `semio_framework_raster::cpu` (`🖌️raster/🧮️cpu`, NEW dir) + VectorScene grows `Paint` (solid/image), `FillRule`, `StrokeStyle` (join/cap/miter); GPU tier maps them to vello; 12 language-agnostic fixture scenes (`🧫️fixtures/🔣️.json` + `🧬️schema/🔣️.json`) → pinned RGBA8 digest law (every target) + resvg oracle law (native); oracle registries `🔮️oracles/🔣️.json` (swash, resvg; schema-valid) | **proof-1 22:40: native 14/15** — every unit law + resvg oracle 12/12 PASS (mean 0.00025–0.00302, far ≤ 0.00024), GPU tier compiles + its test PASS; digest law red by design (digests unpinned) → pinned from native; tolerances = measured × 1.5 (far 0.0005); fuel calibrated (256/pixel, 64/edge = StepBudget units). **wasm32-wasip2: fonts 8/8 PASS** (incl. swash laws); raster aborted at the unpinned digest law. proof-2 (R W S H M) QUEUED 22:43 | `…/s15-rs1-logs/proof-1.txt`, `proof-2.txt` |
| 4 | stdio drawing→png leaf onto the tier (+ Text in Anta at 16 = SVG initial size, Image bilinear) + law vs resvg incl. text/image; shooting `photos:out` → `shooting_scene_png` (drops the `semio-framework-os` dependency) + law vs resvg (default / ellipse / emblem) | **proof-1: stdio png+pdf leaf laws 12/12 PASS** (incl. new text/image law, old resvg + hayro laws unchanged); **shooting 5/6** — `ellipse` red (mean 0.079, far 0.053) = shooting set `canvas.background` = shot background, which the png leaf paints under the WHOLE canvas while the svg leaf ignores it → fixed in shooting (background only as the frame's fill; `canvas.background: None`); re-proof in proof-2 | `proof-1.txt` S/H |
| 6 | measure (release native, temporary `[DEBUG]` tests, scene = 64² image over the canvas + 48 filled+stroked cubic blobs + a text run) | **256²: 2.7 ms; 1920×1080: 59.5 ms; 4096² (the tier's bound): 838 ms**, canvas 64 MiB, max RSS 172 MB (whole test process); at the old 2M-unit fuel the longest step was 47 ms (1080p) / 123 ms (4096²) → fuel recalibrated, re-measure at 20M in proof-2 | `proof-1.txt` M |
| 5 | prepared set `wp-rs1/rs1-raster-tier.py` (+ `.set.json`: 12 created files, 25 hunks / 10 edited files; `--dry-run` default / `--write` / `--revert` / `--root`; backups `.🧬semio/🌐hub/s15-rs1-backup/<stamp>/`) | **dry-run clean on live 22:0x** (create 10 + apply 25, 0 conflicts; oracle files added after → recapture after proof) | `python3 wp-rs1/rs1-raster-tier.py` |

### Session 15 log

- 21:4x read preambles 15/14, S20 design + coordinator decision (VectorScene input; ONE first-party reader over the ui
  `🔤️outline.ttf` faces; parley/swash + resvg oracles only; ui-onto-reader = later row).
- 21:5x survey: stdio `🖊️drawing/…/📷️png/🔖️1.2` ALREADY carries a first-party CPU rasterizer (font-rs exact-area coverage, butt/miter
  strokes, nearest image blit, NO text, private to stdio, f32 canvas 20 B/px) with a resvg oracle law; draw's png, layout/gis/puzzle/
  procedural png exports all route through it (`encode_drawing(.., Png)`). → the CPU tier = that rasterizer generalised over
  `VectorScene`, moved to `🖌️raster/🧮️cpu`, memory-bounded (u8 premultiplied canvas + one accumulation row + current op edges),
  resumable (`CpuRasterJob::step(fuel)` / `progress()` / `finish()`), text via the new reader. `semio_framework_os::
  rasterize_svg_to_png_base64` (resvg) default `Options` has an EMPTY fontdb → native `photos:out` never drew its label (finding).
- 21:47 overlay `.🧬semio/🌐hub/s15-rs1-overlay` = `wp-t14/overlay.py create` (33 s); runner `wp-rs1/rs1-cargo.sh <lane> <capture> <cargo…>`
  (private `.rs1-build` seeded with registry units via `wp-t14/overlay-build-seed.py`, `.rs1-target`, `CARGO_NET_OFFLINE`).
- 21:52 fonts laws 8/8 PASS first run (17.8 s wall, 138 MB peak; capture above).
- 22:0x set script + base snapshots (`wp-rs1/set/base/`, mirrored `.🧬semio/🌐hub/s15-rs1-set/`); every edited live file predates the
  overlay clone (mtimes ≤ 21:12), so base = live. Cargo.lock stays out of the set (first cargo run adds the new package offline).
- 22:14 proof `wp-rs1/rs1-proof.sh proof-1 R F S H W M` relaunched detached (`wp-w2/w2-detach.py`, same queue stamp 215459);
  M = temporary `[DEBUG]` measure tests (256², 1080p, 4096², release), removed from the overlay before the final capture.
- 22:40 proof-1 DONE (lane 22:22–22:40): R 134.9 s wall; S 460 s (stdio cold in the private build-dir); H 310 s; W 36 s; M 3 runs.
  Findings: (1) shooting ellipse red → canvas.background double-paint (above); (2) stdio svg vs png leaves disagree on
  `DrawCanvas.background` (svg ignores it, png paints it) — follow-up for stdio (svg leaf should emit a background rect);
  (3) svg leaf writes `<image>` without `preserveAspectRatio="none"` (letterboxes) while the png leaf stretches — follow-up;
  (4) `semio_framework_os::rasterize_svg_to_png_base64` hands tiny-skia's PREMULTIPLIED pixmap to the PNG encoder as straight
  RGBA (semi-transparent pixels darken) and loads no fonts — follow-up for its remaining callers (raster svg import, animate
  presentation, os run dwg), which can move onto svg → `SemioDrawingFromSvg` → this tier.
- 22:43 digests pinned, tolerances tightened, fuel calibrated, shooting background fix, temporary `[DEBUG]` prints in the stdio
  and shooting laws; proof-2 `R W S H M` queued (detached, `wp-w2/w2-detach.py`).

