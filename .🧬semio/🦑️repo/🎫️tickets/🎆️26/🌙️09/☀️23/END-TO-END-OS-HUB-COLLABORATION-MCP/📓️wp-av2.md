# WP AV2 — Animate Video Export Host Capability `media.video-render`

Slice AV2 of session 14 (Opus executor, coordinator `main`), continues AV1 (`📓️wp-av1.md`, `wp-av1/`). Outcome 1: animate
`exportVideoFromDeck` produces a real video file from the running `s` app (local document + hub document). Rules:
`📓️session-14-preamble.md` (guest freeze from "CHAIN LAUNCHED" to "WINDOW 3 OPEN"). Scripts + captures: `wp-av2/`.

## Session 14

### Session 14b

Successor AV2 agent (2026-09-28 12:0x, after the usage cut + app restart). Guest freeze ON since CHAIN LAUNCHED 12:02:46.

| # | Item | Status | Evidence |
|---|---|---|---|
| b1 | Live tree carries no half-applied AV2 edits | **verified 12:1x**: 0/18 new files exist, 0 hunks "already carried", 0 hits for `VideoRenderExport`/`media.video-render`/`VideoRenderJob`/`video-render-export` under `🧰️framework`, animate, root `📜️script.ts` | §14b log 12:1x |
| b2 | Re-base the payload onto the overnight tree (Codex ~1 870 files) | **done**: 3-way merge (`git merge-file -p`, read-only) of snapshot `s14b-av2-snapshot` onto the live files, 14 conflicts in 5 files resolved by `wp-av2/av2-rebase-resolve.py` (all unions with Codex's icon-render-export wire twin + document-transfer task lane + plugin rustfmt); payload **67 hunks / 25 files + 18 new, dry run 0 problems** | `av2-patch.py make`, `av2-apply.py` |
| b3 | Host-only parts landable during the freeze | **none**: every host piece (VideoRenderHost, TaskManager/ShellHost/PluginRuntime, raster TS twin) imports the kernel TS twin (`🎠️kernel/🟦️.ts`, frozen, bundled by the chain); the root verify lane names files that only exist after the guest landing → all of it lands in window 3 as ONE compile-atomic set | — |
| b4 | Overlay proofs on the re-based overlay (TS laws, tsc, cargo overlay lane) | TS laws **5/5 PASS**, tsc **0 errors**; cargo hold 1 (12:46–13:18): kernel `video_render` **5/5**, raster `video` **3/3**, plugin-host `check --lib --tests` **exit 0**, plugin lib test **red** (6 peer test reds + 1 AV2: wire round-trip literal lacked `images` → fixed); animate steps + plugin re-run queued (pid 72586, 13th) | `generated/laws-14b-2.txt`, `tsc-14b-2.txt`, `cargo-14b/`, `cargo-14b-2/` |
| b5 | Window 3 landing (native + wasm32, landing rows) | pending (freeze) | — |
| b6 | Live: export from running `s` (local doc + hub doc), FFmpeg verification of the file | pending (after b5) | — |

#### Log 14b

- 12:1x read preamble (rules 1–21 + 14b), fleet tail, this report. Predecessor state: overlay cargo proof never ran (lane ticket
  `generated/cargo/lane-1.out` empty — killed by the cut); `payload/hunks.json` was stale (19:08, before the 19:1x–20:2x additions;
  apply crashed on a missing new file). Previous payload kept at `.🧬semio/🌐hub/s14-av2-payload-0927/`.
- 12:1x b1 verified (see table). Dry run of the re-made 09-27 payload on the live tree: 12 anchors gone (Codex overnight: plugin
  `🦀️.rs` rustfmt of the `downloaded` line, wire-turn `wireIconRenderExport` + `icon-render-export` case, TaskManager
  `documentTransfer` lane, ShellHost document-transfer tasks) — none "already carried".
- 12:2x b2: snapshot (base + AV2 overlay versions) → `.🧬semio/🌐hub/s14b-av2-snapshot/`; overlay re-synced (108 125 tracked, 3 054
  copied); new base `.🧬semio/🌐hub/s14b-av2-base/` = live versions; 3-way merge; conflicts resolved (unions); ShellHost's
  in-definition comment block on the `videoRenderExport` branch dropped (AGENTS.md); PluginRuntime decodes the program with the
  same `decodePackWire` the icon door uses; no-op `📦️packages/🦀️rust/🦀️.rs` dropped from the manifest.
- 12:2x b4: TS laws run 1 FAIL kernel-program (`Unexpected export`: the merge ate the `}` closing Codex's `wireIconRenderExport`
  → resolver fixed: live + `}` + AV2); run 2 **5/5 PASS** (program valid=3 invalid=23; job streams=2 refusals=8; raster 6 cases;
  host both tiers, 36 events; FFmpeg decodes 6 streams / 54 frames byte-exact). tsc run 1: `TS2300 Duplicate identifier
  downloadMediaExportBytes` (Codex imported it overnight for document export; the AV2 import dropped) → run 2 **exit 0**.
  Payload now **66 hunks / 25 files + 18 new, dry run 0 problems**. Scan of added lines vs the base for other duplicates: none real.
- 12:2x overlay cargo proof queued (`fleet-mutex.sh overlay av2 -- zsh wp-av2/av2-overlay-cargo.sh generated/cargo-14b`, pid
  60712; steps kernel/raster/plugin tests, plugin-host check, animate-artifact tests, animate-plugin check; private
  `s13-av1-build`/`s13-av1-target`).
- 12:3x window-3 tooling: `av2-apply.py --write` now saves pre-landing bytes to `.🧬semio/🌐hub/s14b-av2-prelanding/` and
  `--revert` restores them (refuses when any file changed since the landing; removes the new files + emptied dirs) — round trip
  on a scratch copy of the base: byte-identical. `av2-land.sh`: dry run → write → native-lane `check` of the 6 crates →
  red = immediate revert; green = native tests, TS laws on the live tree, tsc (`tsc/tsconfig-live.json`).
- 12:4x S20 (via main): io-matrix `mp4` IoFormat + `judgeMp4File` (ffprobe h264 + frames, ffmpeg null decode, oracle
  `ffprobe+ffmpeg`) + animate pin `mp4` IN THE TREE since 12:3x; oracle proven by S20 (x264 file ok, truncated copy refused,
  `wp-s20/generated/probe-mp4.ts`). Live animate row today: `exportVideoFromDeck` refused `not-ui-safe` (pre-landing) — window 3 flips it.
- 12:46–13:18 overlay hold 1 (`generated/cargo-14b/`): kernel `test -p semio-framework --lib -- video_render` **5 passed**; raster
  `-- video` **3 passed**; plugin `--lib -- wire_effect_round_trip` did not compile: E0063 `images` missing in the AV2 round-trip
  literal (predecessor added `VideoRenderProgram.images` at 19:1x, never re-ran this test) → fixed in the overlay; the other 6
  errors are peers' overnight test reds (window-kits include path, `from_media_export`/`segmented_closures` privacy, f64 deref) —
  a peer fixed them in the live tree 12:3x–12:49 (`test_from_media_export`), picked up by the next sync; plugin-host `check --lib
  --tests` **exit 0** (78 warnings, 5 m 05 s). animate-artifact was still compiling third-party deps at the 30-min hold limit →
  stopped (my pids 90318 + cargo 47818 only), remaining steps re-queued (`cargo-14b-2/`, lane pid 72586).
- 13:0x `av2-overlay-sync.py` now never overwrites the slice's edited files and lists the ones whose live copy drifted from
  the base; new `av2-rebase.py` (3-way merge of drifted files, base := live). 13:0x sync: 507 copied, 3 drifted (ShellHost,
  PluginRuntime, plugin `🦀️.rs` — peers editing right now) → re-based, 0 conflicts; payload still 66 hunks / 25 + 18, dry run 0.
- Coverage note: the Rust `Effect` gains `VideoRenderExport`; every exhaustive match (kernel, plugin, reactor, 🌐host, 🖥️host,
  imports, wire test) is in the payload; the wgpu shell matches with a catch-all (compiles; drops the effect with its
  `[DEBUG] wgpu-shell effect dropped` line) — wgpu video export is NOT in this slice (open item for WG11's successor).

### Session 14 (09-27, predecessor)

| # | Item | Status | Evidence |
|---|---|---|---|
| 1 | Read preambles, AV1 handover, overlay state | done | §Log 18:3x |
| 2 | Re-derive AV1 payload against the live tree (fresh overlay) | **done** (61 hunks + 12 new files, dry run 0 problems) | `wp-av2/av2-patch.py seed`, `av2-apply.py` |
| 3 | Capability `media.video-render` + event-sourced export job (design + code) | **done in overlay** (Rust + TS twins, host, animate) | §Log 19:1x–20:2x |
| 4 | Permanent laws + third-party oracle (decode produced MP4 independently) | **TS laws 5/5 PASS in overlay**; Rust laws queued (overlay lane) | `wp-av2/av2-laws.ts`, `generated/cargo/` |
| 5 | Overlay proofs (native cargo + wasm-free TS laws + tsc) | tsc **0 errors** (12 files, full graph); real-browser proof **PASS**; cargo queued #10 in overlay lane | `generated/tsc-2.txt`, `generated/browser/checks.txt` |
| 6 | Window 3 landing (native + wasm32, landing rows) | pending | — |
| 7 | Live: export a deck from running `s` (local doc) + verify file | pending | — |
| 8 | Live: export a deck from a hub document + verify file | pending | — |

### Log

- 18:3x started. No CHAIN LAUNCHED yet (coordinator log 18:1x only). AV1 left an overlay
  (`.🧬semio/🌐hub/s13-av1-overlay`, synced ~15:2x) with a complete but un-landed design: kernel
  `Effect::VideoRenderExport { filename, program: VideoRenderProgram }` (+ TS twin, WIT `video-render-export`, reactor/host
  codecs), raster video tier (first-party H.264 all-`I_PCM` encoder behind `VideoEncoder` + MP4 muxer, Rust + TS twins,
  raster Rust tests 6/6 green in overlay 16:08), browser host `🎥️VideoRenderHost` (canvas paint, WebCodecs tier behind a
  port, first-party fallback, Task Manager lane `export`, en+de), animate command rewritten to emit the effect (deck →
  presentation scene → `VideoRenderProgram`). Not done by AV1: `payload/hunks.json` never generated; two animate test files
  missing from the manifest; kernel test run failed on an unrelated overlay sync gap (`🔤️tokens/🦀️.rs` missing); report not
  updated past item 2a. Full base→overlay diff: `wp-av2/generated/av1-edited.diff`.
- 18:5x overlay re-synced to the live tree (`wp-av2/av2-overlay-sync.py`: tracked + gitignored generated sources such as
  `🔤️tokens/🦀️.rs`, which broke AV1's kernel test; 1189 files refreshed, 12 AV1-new files removed then re-seeded). AV1's work
  snapshotted to `.🧬semio/🌐hub/s14-av2-av1-snapshot/` and re-derived onto the synced overlay (`av2-patch.py seed`, 0 problems);
  payload dry run on the live tree: `hunks=61 files_edited=24 new_files=12 problems=0`.
- 19:1x design additions (overlay only): (a) kernel `MEDIA_VIDEO_RENDER_CAPABILITY = "media.video-render"` (Rust + TS);
  (b) `VideoRenderOp` becomes a `kind`-tagged enum `fill | stroke | image` + `images: [{url}]` (same-origin path or
  `data:image/…` only; crop/opacity admission) so a deck video can show the deck's picture; (c) event-sourced job:
  `VideoRenderJobEvent {started, progressed, cancelRequested, finished{done|cancelled|refused|failed}}` + `VideoRenderJobLedger`
  fold (Rust + TS twins, fixture `🎠️kernel/🧫️fixtures/🧵️video-render-job/🔣️.json`: 2 streams, 8 refusals); (d) first-party
  encoder `AvcPcmEncoder`: I_PCM IDR per new picture + all-`P_Skip` P picture per repeat (a still slide costs one picture:
  720p 2-frame case 2.78 MB → 1.39 MB), `VideoEncoder::repeat`, TS twin 30× faster (plane pass + indexed NAL scan: 720p
  picture 2.6 s → 87 ms warm); (e) permanent third-party oracle: `🖌️raster/🎥️video/🔮️oracles/🔣️.json` (FFmpeg,
  `third-party-cli`, `hostImplementation: typescript`) + case `🧪️tests/🎞️ffmpeg-decode` (`🥒️.feature` + `🟦️.ts` differential
  adapter: exact yuv420p plane digests, key frames, stream facts).
- 19:2x TS laws in the overlay: `bun wp-av2/av2-laws.ts --only kernel-program,kernel-job,raster-video,ffmpeg-decode` →
  **4/4 PASS** (program valid=3 invalid=23; job streams=2 refusals=8; raster cases=6 refusals=3; FFmpeg 8.1 decoded all 6
  fixture streams / 54 frames byte-exact = the encoder's claimed planes). Fixture regenerated by `wp-av2/av2-emit-raster.ts`
  only after the oracle agreed.
- 19:4x host `🎥️VideoRenderHost` rewritten on the kernel ledger: `runVideoRenderExportV1({filename, owner, program,
  capabilities})` appends `started` → (capability refusal `capability` | kernel program refusal) → `progressed` (≤ 1 per 100 ms +
  last frame) → `finished{done|cancelled|refused|failed}`; cancel = `cancelVideoRenderExportV1(taskId)` appends
  `cancelRequested`, the render loop reads the ledger before every frame; Task Manager rows = `ledger.running()`; image ops
  (`fetch` + `createImageBitmap`, same-origin/data only); first-party tier encodes each run's picture once then `repeat()`;
  platform tier = WebCodecs behind `WebCodecsVideoEncoderPortV1` (key frame per run + every 2 s); yields ≤ 8 ms of work.
  ShellHost resolves the plugin's declared capabilities from the generated registry (`PLUGIN_BUILD_TARGETS[].capabilities`
  = descriptor `capabilityRequests`); animate requests `media.video-render` (builder + `🔣️.json`). Host law
  (`🎥️VideoRenderHost/🧪️tests/🔬️unit/🟦️.ts`) PASS: both tiers, one IDR per run (`stss` [1,7,11]), platform key frames
  [0,12,24], refusals capability + frameRate (en + de texts), failed image load, cancel at frame ≥ 3 of 30, task list =
  fold of the log (`VideoRenderJobLedger.fold(log)` → 0 running, last job 6). 36 events.
- 19:5x animate (overlay): deck export = `video_render_program_from_deck` (overview of the source picture with every tile
  outlined, 3 s, then one 2 s slide per tile showing its crop, aspect kept, 5 % margin; 1280×720 @ 15 fps Medium preset;
  one scene per slide) named after the picture (`habitat-67.mp4`); a stated `scene` JSON keeps AV1's engine-capture path
  (`VideoProgramBuilder`, now `Fill`/`Stroke` ops); PDF/empty source → named fault `animate.video.export.source-kind`;
  packed-size estimate counts image URLs. Laws updated (`export-video-from-deck/🧪️tests/🔬️unit/🦀️.rs`, program-unit).
- 20:0x root `📜️script.ts verify video-render-export [native]` lane added in the overlay (4 TS laws + `tsc --strict` of the
  raster twin; native: kernel `video_render`, raster `video`, plugin `wire_effect_round_trip`, animate `export_video`). Per
  preamble rule 17 the nx target + launch row are R10's (RELAY pending at landing time, spec below).
- 20:0x overlay cargo proof enqueued: `fleet-mutex.sh overlay av2 -- zsh wp-av2/av2-overlay-cargo.sh` (pid 85998, queue #10
  behind t14 which holds since 19:26; private dirs `.🧬semio/🌐hub/s13-av1-build` / `s13-av1-target`, never the shared
  build-dir). Steps: kernel, raster, plugin, host, animate-artifact, animate-plugin → `wp-av2/generated/cargo/<step>.txt`.
- 20:1x tsc over the 12 overlay TS files (`wp-av2/tsc/tsconfig-av2.json`, overlay `node_modules/@semio-tech/*` → overlay):
  run 1 = 1 error (missing `VideoRenderJobRow` type import in ShellHost, fixed), run 2 **exit 0, 0 errors**
  (`wp-av2/generated/tsc-2.txt`, ~10 min).
- 20:1x RELAY S20 (via main): io-matrix needs IoFormat `mp4` judged by FFmpeg (ffprobe h264 + frames > 0, `ffmpeg -f null`
  full decode) and the animate pin set to `mp4` — its current `any` sniffs `.mp4` as text. Spec: see §S20 below.
- 20:2x **real-browser proof** (`bun wp-av2/av2-browser-host.ts --port 6591`: overlay host bundled, Playwright Chromium 1234
  headless, real demo figure, deck program of a 2×2 grid = 165 frames): WebCodecs H.264 supported in headless Chromium on
  this Mac; platform tier 926 942 B in 3.3 s, first-party 6 951 135 B in 0.5 s; FFmpeg: both `h264 Constrained Baseline
  1280x720 yuv420p 15/1 frames=165`, full decode ok, key frames platform 1,31,46,…,151 / first-party 1,46,76,106,136;
  5/5 slide pictures distinct with real content (luma σ 44–75); platform vs first-party decoded frames PSNR 37.2–45.4 dB
  (same picture on both tiers). Frames eyeballed: `generated/browser/frame20.png` (overview + outlines),
  `frame60.png` (tile). Capture `wp-av2/generated/browser/checks.txt`.

### §S20 — io-matrix `mp4` format (relayed 20:1x)

`IoFormat += "mp4"`, `formatOfFileName`: `[/\.mp4$/u, "mp4"]`; `judgeFile` case `"mp4"`: write bytes to a temp file,
`ffprobe -v error -count_frames -select_streams v:0 -show_entries stream=codec_name,width,height,nb_read_frames -of json`
→ ok iff `codec_name === "h264" && width > 0 && height > 0 && nb_read_frames > 0`, and `ffmpeg -v error -i <file> -f null -`
exits 0; oracle name `ffprobe+ffmpeg`. Fixture `🧑‍💻dev/🧫️fixtures/🚪️io-matrix.json` animate pin → `"format": "mp4"`.

### §R10 — verify lane (to relay at landing)

Project `workspace` (root `📋️project.json`), targets `video-render-export` (`bun ./📜️script.ts verify video-render-export`,
group dev, after `media-export-encoding`) and `video-render-export-native` (`… verify video-render-export native`, group
gate); en "Video render export laws" / de "Gesetze des Videoexports"; requires nothing (no hub, no serve, no browser).
