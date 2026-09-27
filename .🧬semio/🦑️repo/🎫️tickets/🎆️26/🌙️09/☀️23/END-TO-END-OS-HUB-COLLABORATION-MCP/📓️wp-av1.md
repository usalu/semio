# WP AV1 — Animate Video Export Host Capability (prepared, window 3)

Slice AV1 of session 13 (Opus executor, coordinator `main`). Outcome 1: every command in `s` works — animate
`exportVideoFromDeck` is `BatchOnlyPendingRewrite` (no host renders + encodes video). Deliverable: a PREPARED, verified patch
(`wp-av1/av1-apply.py`, `--dry-run` default / `--write`) that lands in window 3.

## Session 13

| # | Item | Status | Evidence |
|---|---|---|---|
| 1 | Read requirements (preamble 1–35, T13 §AV, draw PDF precedent) | done | — |
| 2 | Design: `Effect::VideoRenderExport { filename, program }` (kernel SSOT + WIT + TS twin) | in progress | §Design |
| 2a | First-party H.264 (all-`I_PCM` IDR) encoder + MP4 muxer, Rust + TS twins, one fixture | **green** (Rust 3/3 standalone, TS law, ffprobe+ffmpeg oracle 4/4) | `wp-av1/av1-oracle.py`, `generated/` |
| 3 | Patch script + payloads | pending | — |
| 4 | Overlay verification (native cargo + bun) | pending | — |
| 5 | Oracle (ffprobe / python mp4 box parser) | pending | — |
| 6 | Census law 0 unreachable for animate | pending | — |

### Log

- 14:59 started; report skeleton written. No tree edits before window 3 (rule 33).
- 15:2x overlay `.🧬semio/🌐hub/s13-av1-overlay` = tracked files of the live tree (`wp-av1/av1-overlay-sync.py`, 106 573 files; openrsync was
  too slow). Authoring: `wp-av1/av1-patch.py base|new|make` (base snapshots in `.🧬semio/🌐hub/s13-av1-base`) → `payload/hunks.json` +
  `payload/files/`; `wp-av1/av1-apply.py [--root] [--write]` (dry run default, all-or-nothing).
- 15:5x raster video tier (`🧰️framework/🔨️modules/🖌️raster/🎥️video/{🦀️.rs,🟦️.ts,🧫️fixtures/🔣️.json,🧪️tests/🔬️unit/*}`): first-party
  Constrained-Baseline H.264 encoder (every picture one IDR slice of `I_PCM` macroblocks, BT.601 limited range, exact) behind a
  `VideoEncoder` trait/port + progressive MP4 muxer (`ftyp`/`moov`/`mdat`, `avcC`, `stss` only when not all-sync) + `encode_video_runs`
  (progress per frame, cancellation before the next frame). **ffprobe + ffmpeg oracle 4/4 PASS** (codec h264, profile Constrained
  Baseline, size, yuv420p, r_frame_rate, nb_frames = nb_read_frames, duration, level 10/31, decoded rgb within 4 of every run colour).
  TS law PASS (bun); Rust law 3/3 PASS in a standalone scratch crate (`wp-av1/rs-check`, byte-identical `mp4Hex`).
