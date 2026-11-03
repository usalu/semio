# R3 media plan (W17, `media`)

T = ticket folder, A = `🎓️teaching/🏛️architecture`, CACHE = `.🧬semio/🎓️teaching/⚡️cache`. Tools: `T/🧰️r3-media/`.

## 1. Discovery

### Canonical commands (all 26 manifest videos, A/⚡️energy/🎬️clip/🔣️.json)

| Series | Videos (manifest ids) | Canonical command |
|---|---|---|
| energy | physical_fundamentals, ventilation, heating, cooling, energy_balance | `bun A/⚡️energy/🏗️authoring/📜️script.ts render --video <id> --language <en\|de> --audience speaker --quality l --fps 15 --no-play --log-output --jobs 1` |
| bim, excel, grasshopper, optimization | bim_*, excel_*, grasshopper_*, optimization_* | the same command, same script (family clips run through the energy runtime) |
| nakagin, typology, morph, parametric, fit, course | scenes inside the lessons above, no own video ids | rendered as part of their lesson |

`T/🧰️r3-media/render_video.sh <lang> <id>...` wraps it: serial, one run per language (lock folder, avoids partial clobbering of `s/<n>`), retry after a guard abort (finished segments are reused), temp and logs under `CACHE/media-logs`. `JOBS=<n>` raises the segment limit; keep it at 1 or 2 with large casts.

### Preview scripts of ticket 26/10/28 (`🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/render`)

| Script | Role | Verdict |
|---|---|---|
| `energy-chain.sh` | the energy command above, copies masters into `preview/` | superseded by `render_video.sh` (needed the old node shim, bun now works through `repo tools exec`) |
| `preview_any.py`, `preview.py`, `preview_many.py`, `painter.py`, `bim_fast.py` | guard-free direct Manim single-scene previews, no narration, no provenance, hard-coded `Scripts/python.exe`, Nakagin painter skips occlusion | not used for the course media; any file they wrote has no cast keys and counts as stale |
| `course_preview.py` | joins the 26 `preview/<Output>_speaker_<lang>_480p15.mp4` copies | replaced by `T/🧰️r3-media/course_preview.py` (freshness gate, provenance, poster, thumbnail) |
| `deliver.py` | exports 1080p30 `delivery/*.mp4` + `delivery.json` for bim/opt/gh/excel | unchanged; deliveries of old masters are stale until re-rendered (run only after the masters are fresh) |

### Outputs

`CACHE/videos/<lang>/<audience>/`: `s/<source_ordinal>/v/<Scene>.mp4` + `.render.json` (segment clip and its state), `<video_index>/v/<Chapter>.mp4` + `.render.json`, `rendered/<Output>_<audience>_<lang>_480p15.mp4` + `.render.json` (lesson master), `preview/` (copies and the course join), `delivery/`, `trace/`, `speech/`. Staffage casts: `CACHE/staffage/<32-hex cast key>/<cast>`.

### Versioning and cache keys

- Segment clip key = `segment_state` (runtime `⚙️runtime/🐍️.py:421`): settings, sha256 of every source and asset reachable by literal `import_module`, narration and timing, theme, audience, the segment, and `casts` = `staffage.casts_in(sources)`, i.e. the cast key of every cast id named in those sources. A clip is reused only if its sidecar equals the state exactly (`reusable_clip`), so a changed catalog asset, exporter, schema, environment preset or cast manifest entry re-renders the scene. The contract (`r3-contract.md`) folds asset revision, preset revisions and viewpoint/scale/clock/scenario into `cast_key`.
- Lesson master key = the logical states of its chapters, each embedding the segment states.
- Staffage casts are content-addressed; stale key folders are pruned after a day.
- Gaps found: (a) preview copies and direct-Manim previews carry no states, (b) nothing reported freshness over the whole course, (c) no posters or thumbnails existed, (d) `casts_in` is module-granular (every cast named anywhere in a reachable module, 20 casts for `Beat1_Leistung`), so one cast edit re-renders more scenes than strictly necessary; safe, only costly, (e) sidecar `sources` keys are absolute paths (a moved clone invalidates all).

### Traps and handling

| Trap | Handling |
|---|---|
| source-fingerprint guard aborts on concurrent edits | `render_video.sh` retries; finished segments stay reusable; `status.py` shows what remains |
| same-language parallel renders clobber `s/<n>` | lock folder `videos/<lang>/.media-lock`, one language at a time; en and de may run in parallel |
| `repo tools ffmpeg` and bun crash | frames, posters, thumbnails and durations through PyAV; joins through the bundled imageio ffmpeg (works here) |
| cast frame cap (600 frames) | not touched by media; a rejected cast fails the scene render |
| 42 GB with `--jobs 3` | `JOBS` defaults to 1; never above 2 |
| `repo` shim | not needed any more: `repo tools exec --light bun` works (bun 1.4.2) |
| encoding of emoji paths in manim logging | `PYTHONUTF8=1 PYTHONIOENCODING=utf-8` in `run.sh` |

## 2. Implemented (T/🧰️r3-media)

| File | Purpose |
|---|---|
| `run.sh` | run a script in the teaching uv environment (cross-platform, no `Scripts/python.exe`) |
| `lib.py` | plan, freshness (`reusable_clip(segment_state)`), provenance (`staffage_provenance` of each cast), PyAV posters (854x480) and thumbnails (320x180) |
| `status.py <lang>` | fresh/stale/missing per segment and master |
| `render_video.sh <lang> <id>...` | canonical serial render |
| `manifest.py <lang>` | `videos/<lang>/speaker/media-manifest.json`: per lesson and segment sha256, state hashes, cast keys and provenance records, media key (sha + states + provenance + version), poster and thumbnail (only for fresh files) |
| `course_preview.py <lang>` | gated join of fresh masters, chaptered, with poster, thumbnail, media key and cast keys |
| `purge_stale.py <lang> [--apply]` | remove preview files that no fresh master backs |
| `probe_casts.py` | list the casts a scene fingerprint names |
