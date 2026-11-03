# 🎬 Architecture Course Clip Render Pipeline: Audit

Read-only audit of 2026-11-01 (about 01:47 local). No file was modified except this report. No render or test was executed, so every command below is unverified until run. Other sessions edit the same files live (see section 9), so re-check the fingerprint facts before relying on them.

Workspace: `C:\git\semio`. Shell: Git Bash. Use `W=/c/git/semio` in each command.

Shorthand used below:

- `E` = `🎓️teaching/🏛️architecture/⚡️energy`
- `A` = `$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring`
- `C` = `$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip`
- `CACHE` = `$W/.🧬semio/🎓️teaching/⚡️cache`

---

## 1. Layout

| Path | Role |
| --- | --- |
| `A/📜️script.ts` | Bun entry point. Commands: `render` and `test`. Spawns `uv run --project A/📦️packages/🐍️python --python 3.12 [--group test] python ...`. |
| `A/📦️packages/🐍️python/pyproject.toml`, `uv.lock` | Runtime deps: `manim==0.22.0`, `numpy==2.5.4`, `imageio-ffmpeg==0.6.0`. Test group adds `fonttools`, `formulas`, `jsonschema`, `openpyxl`, `rhino3dm`, `scipy`, `shapely`. `package = false`. |
| `A/📦️packages/🐍️python/.venv` | A local Windows venv (`Scripts/`). `script.ts` does not use it. See the environment note below. |
| `C/🎬️clip` related: `C/README.md` | Canonical source sequences, audiences, segment/clause selection, cache layout. |
| `C/⚙️runtime/🐍️.py` | Planner, fingerprints, Manim invocation, excerpts, chapter/lesson assembly, presentation publish. |
| `C/🖌️render/🐍️.py` | CLI for `--video`, `--chapter`, `--audience`, `--quality`, `--fps`, and so on. Calls `compose_videos`. |
| `C/📁️output/🐍️.py` | `output_root()`, `speech_root()`, `MANIM_DIRECTORIES`. |
| `C/👥️audience/🐍️.py` | `speaker` (captions on) and `spectator` (captions off). |
| `C/🔣️.json` (in `E/🎬️clip/`) | Course manifest: 26 videos, each with chapters and segments. |
| `C/🪟️presentation/` | Lesson window compositor. Uses a Node/Playwright step (`🌐️browser/🟨️render.mjs`). |
| `C/🧪️tests/`, `C/📁️output/🧪️tests/` etc. | Unit tests listed in `script.ts`. |

Environment note: `script.ts` sets `UV_PROJECT_ENVIRONMENT` to `$SEMIO_TEACHING_ENV`, or `$CACHE/python` by default. `$CACHE/python/pyvenv.cfg` exists and uses CPython 3.12.11 from uv. The `.venv` inside the package is a separate local venv and is not used by the scripts.

Runtime tools: `bun` (`/c/Users/Ueli/.bun/bin`, 1.4.2). `uv` 0.11.6. `repo` is on PATH. `ffmpeg` is not on PATH, so the runtime uses `imageio_ffmpeg.get_ffmpeg_exe()`, or `SEMIO_FFMPEG_BINARY` if set.

---

## 2. Output locations and defaults

Set by `script.ts` and `📁️output/🐍️.py`, all relative to the workspace:

- `SEMIO_TEACHING_CACHE_ROOT` defaults to `.🧬semio/🎓️teaching/⚡️cache`.
- Render output root defaults to `$CACHE/videos`. Test output root defaults to `$CACHE/tests`.
- `SEMIO_TEACHING_CACHE_DIR` (fonts) defaults to `$CACHE/fonts`.
- `UV_PROJECT_ENVIRONMENT` defaults to `$CACHE/python`.
- Staffage casts: `$CACHE/staffage/<32-hex key>/<cast>`.

Render layout for `--language en --audience speaker`:

- `$CACHE/videos/en/speaker/s/<source_ordinal>/v/<Scene>.mp4`: one canonical source beat.
- `$CACHE/videos/en/speaker/<video_index>/v/<Chapter>.mp4`: one chapter.
- `$CACHE/videos/en/speaker/rendered/<output>_speaker_en_480p15.mp4`: one lesson. Quality `-ql` gives `480p15`, `-qm` gives `720p30`, `-qh` gives `1080p60`. With `--fps` the rate changes to match.
- `$CACHE/videos/en/speaker/rendered/<Chapter>_speaker_en.mp4`: when `--chapter` is used.
- `$CACHE/videos/speech/<lang>/<identity>/`: clause audio and `timing.json`.
- `$CACHE/videos/en/speaker/trace/<identity>/<Scene>/🔣️.json`: narration timing trace.
- Each `.mp4` has a `.render.json` sidecar, which is the provenance record.

---

## 3. Commands

### 3.1 One lesson at low quality (speaker only)

```bash
W=/c/git/semio
cd "$W" && bun "$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render \
  --video heating --language en --audience speaker --quality l --fps 15 --no-play --log-output --jobs 2
```

Valid video IDs (26): `physical_fundamentals`, `ventilation`, `heating`, `cooling`, `energy_balance`, `bim_information`, `bim_objects`, `bim_representations`, `bim_models`, `bim_exchange`, `bim_consistency`, `grasshopper_anatomy`, `grasshopper_capsule`, `grasshopper_tower`, `grasshopper_casting`, `grasshopper_trees`, `excel_anatomy`, `excel_references`, `excel_program`, `excel_energy`, `optimization_extrema`, `optimization_sampling`, `optimization_experiments`, `optimization_index`, `optimization_pareto`, `optimization_algorithms`.

Omit `--audience` to render both speaker and spectator. Repeat `--video` to select several lessons.

### 3.2 One chapter

```bash
bun "$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render \
  --video heating --chapter Heating_05_Solar --language en --audience speaker --quality l --fps 15 --no-play
```

### 3.3 One source scene, no lesson, no fingerprint guard (direct Manim)

This is the fastest route for one beat. It is what the prior `preview_any.py` does. It has no guard, so it is fine while shared sources are being edited, but the result is not a certified artifact.

```bash
W=/c/git/semio
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
MEDIA="$W/.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/preview/media"
SCRIPT="$W/🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py"
cd "$W" && PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
  UV_PROJECT_ENVIRONMENT="$CACHE/python" \
  SEMIO_TEACHING_LANGUAGE=en SEMIO_TEACHING_AUDIENCE=speaker \
  SEMIO_TEACHING_OUTPUT_ROOT=.🧬semio/🎓️teaching/⚡️cache/videos \
  SEMIO_TEACHING_CACHE_DIR=.🧬semio/🎓️teaching/⚡️cache/fonts \
  uv run --project "$P" --python 3.12 python -m manim -ql --fps 15 --disable_caching \
  --media_dir "$MEDIA" "$SCRIPT" Beat6_ThermischeMasse
```

Find the output with `find "$MEDIA" -name 'Beat6_ThermischeMasse*.mp4'`.

Set `SEMIO_TEACHING_LANGUAGE`/`AUDIENCE` explicitly. Speaker shows captions. Energy scenes hard-code German captions (`set_vo_language("de")`), so `en` videos show German captions by design.

For the full pipeline's segment behavior (narration attach, trace), use the guarded route in 3.1. Direct render skips the trace and narration.

### 3.4 Still frame

Last frame (Manim `-s`), from the same invocation as 3.3:

```bash
W=/c/git/semio
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
MEDIA="$W/.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/preview/media-still"
SCRIPT="$W/🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py"
cd "$W" && PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 UV_PROJECT_ENVIRONMENT="$W/.🧬semio/🎓️teaching/⚡️cache/python" \
  SEMIO_TEACHING_LANGUAGE=en SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT=.🧬semio/🎓️teaching/⚡️cache/videos \
  uv run --project "$P" --python 3.12 python -m manim -ql -s --disable_caching --media_dir "$MEDIA" "$SCRIPT" Beat6_ThermischeMasse
find "$MEDIA" -name '*ENDFRAME*.png'
```

Frame at an arbitrary time `t` from a rendered clip:

```bash
W=/c/git/semio
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
FFMPEG=$(cd "$W" && uv run --project "$P" --python 3.12 python -c "import imageio_ffmpeg; print(imageio_ffmpeg.get_ffmpeg_exe())")
"$FFMPEG" -hide_banner -loglevel error -nostdin -y -ss 12.5 \
  -i "$W/.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/s/1/v/Beat6_ThermischeMasse.mp4" -frames:v 1 /c/git/semio/<ticket>/🗑️generated/frame-12.5.png
```

Replace `s/1` with the actual source ordinal, and `<ticket>` with your ticket folder.

### 3.5 Prior preview helper (direct, guard-free, from one import snapshot)

Ticket: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/render/`.

- `preview_any.py <script-path-under-🎓️teaching> <Scene> <Output> <en|de> [patch-module]`. Writes `$CACHE/videos/<lang>/speaker/preview/<Output>_speaker_<lang>_480p15.mp4` plus `.json`. Example for the energy beat in 3.3:

```bash
W=/c/git/semio
TICKET="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
PY="$W/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
cd "$W" && PYTHONUTF8=1 "$PY" "$TICKET/render/preview_any.py" "🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py" Beat6_ThermischeMasse Physical_Beat6 en
```

- `preview.py <series> <topic> <Scene> <Output> <lang>` (Grasshopper and Excel only; `series` like `🦗️grasshopper`). Shell helper `previews.sh <jobs> <lang>...` runs the nine Grasshopper and Excel previews.
- `preview_many.py <patch-module> <lang> <list>` runs many in one process. Lists: `bim.list`, `bim-opt.list`, `opt.list` (format `<script-path> <Scene> <Output>`).
- `painter.py` is a preview-only patch for the Nakagin tower. It skips occlusion.
- `course_preview.py <en|de>` joins the 26 per-video previews into one chaptered file. Output: `$CACHE/videos/<lang>/speaker/preview/Architecture_course_speaker_<lang>_480p15.mp4`.
- `run.sh`, `run-fast.sh`, `energy-chain.sh` drive full `script.ts render` runs and call `deliver.py`. They hard-code `Scripts/python.exe` and a `SEMIO_REPO_BINARY` shim from another ticket (`26/10/27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd`). These are Windows-only.

Portability: the helpers hard-code `.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe`. To make them cross-platform, replace it with `uv run --project "$P" --python 3.12 python`.

---

## 4. Source-fingerprint guard

Implementation: `C/⚙️runtime/🐍️.py`.

### 4.1 What is fingerprinted (`segment_state`, line ~414)

Each `state` is `{settings, sources: {abs posix path: sha256}, segment}`. Sources:

1. `⚙️runtime/🐍️.py` itself.
2. `source_dependencies(script)`: the segment script plus every `.py` reached through literal `importlib.import_module("🎓️teaching.…")` calls, followed transitively. Only the first argument must be a string constant. `import` and `from … import` statements are not followed. Non-Python files are skipped.
3. Fixed: `🏗️authoring/🔤️fonts/🐍️.py`, `🎨️visuals/🐍️.py`, `🔊️speech/🐍️.py`, `interaction.source_files()`, `🪟️presentation/🖱️interaction/🎬️scene/🐍️.py` and `🔣️.schema.json`.
4. Assets: every file with an asset extension (`.json .pdf .svg .png .jpg .jpeg .webp .ttf .otf .mp3 .wav .m4a .flac .ogg`) under each owner directory of a fingerprinted `.py`. Directories named `🧪️tests` are skipped. This is a recursive `rglob`, so the whole `🎨️visuals/` tree is included.
5. `timing.json` per source speech root, existing clause recordings, `theme.source_files()`, `audience.source_files()` (audience `🐍️.py`, `🔣️.json`, `🧬️schema.json`).

`clip_state` hashes each existing file with `sha256`. Missing files are skipped silently.

Presentation has its own `presentation_state()` (`🪟️presentation/🐍️.py` `source_files()`, which adds typography, geometry, browser and hierarchy sources).

### 4.2 Where it aborts

- `render_segment`: after Manim finishes, `segment_state != state` raises `Authored sources changed while rendering <Scene>`. Also re-checked after narration attach.
- `excerpt_segment`: `Authored inputs changed while selecting <Scene>`.
- `compose_videos`: `Authored inputs changed before assembling …`, `… while assembling …`, and `… while assembling <video>`.
- `publish_presentation`: `Presentation inputs changed while rendering <video>`.
- Reuse: a clip is reused only if its sidecar equals the current state exactly. Any change re-renders. The abort is the detection that the render itself raced an edit.

### 4.3 Gaps and traps

- Staffage TypeScript and its catalog are not covered (see section 5).
- `sources` keys are absolute paths (`resolve().as_posix()`). A clone or move to another absolute path invalidates every sidecar. This matters for zero-touch on other machines.
- Cost: `segment_state` is recomputed many times per run (per segment, post-run, per excerpt, per chapter, per presentation). Each call re-reads all fingerprinted files, including all asset files under the owner directories. Adding a large staffage tree multiplies the cost. Memoize per run, or fingerprint by `(size, mtime_ns)` with a content hash fallback.
- Two same-language renders share `videos/<lang>/<aud>/s/<n>` slot directories and clobber each other (`teaching-render-concurrency` memory). Different languages are safe.
- `--jobs` is a global segment limit within one process, not across processes.

---

## 5. Would staffage dependencies or a cache interact with the guard?

### 5.1 Current state of staffage

- `A/🎨️visuals/🐍️.py` line 38 imports `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️` through `importlib.import_module`. The module is `A/🎨️visuals/🧍️staffage/🐍️.py` (untracked, created 01:43 today). It exposes `staffage`, `staffage_motion`, `staffage_height` and `ensure_casts`.
- `staffage/🐍️.py` reads the staffage archive at `_SOURCES = 🎓️teaching/🏛️architecture/🧍️staffage`:
  - `_source_digest`: hashes `🔨️modules/**` and `🧬️schema/**` (excluding `node_modules`). This is the cast key.
  - `_catalog_bytes`: reads `🗂️catalog/*/<slug>/🔣️.json`.
  - `_EXPORTER`: `🔨️modules/📤️export/🎭️cast/🟦️.ts`, run during render as `repo tools exec bun -- <exporter> …` for missing casts. Output goes to `$CACHE/staffage/<key>/<cast>`, content-addressed.
  - `_repo()`: `SEMIO_REPO_BINARY`, then `repo` on PATH, then `.🧬semio/🦑️repo/native-repo-path`. That marker is absent here, but `repo` is on PATH.

### 5.2 Answers

Adding a dependency on staffage sources:

- Python imported with `importlib.import_module("🎓️teaching…")` from a fingerprinted module is already covered, transitively, provided a segment script reaches it. `from x import y` and plain `import` are not followed. Use `import_module` or add the file explicitly.
- TypeScript, catalog JSON, and schema under the staffage root are NOT covered. The owner-directory asset rule never reaches `🧍️staffage/` root because no fingerprinted `.py` lives there. A staffage TS edit during a render would not abort it, yet it changes cast keys and output.
- Fix: expose `source_files()` in `🧍️staffage/🐍️.py` (returning the module, its manifest, every file under `🔨️modules/📤️export/🎭️cast/`, `🔨️modules/**` and `🧬️schema/**`, and the used catalog entries), and add it to `segment_state`'s list next to `theme.source_files()`. Keep the list lean. Exclude `node_modules`, `__pycache__`, `.🧬semio`, and any cache path.

Adding a cache under `.🧬semio/🎓️teaching/⚡️cache/`:

- A cache written DURING a render must NOT be fingerprinted. Fingerprinting it aborts every render with "Authored sources changed while rendering". The staffage cache is already outside the guard, which is correct. Keep it that way. The same applies to `videos/…/s`, `trace`, `m`, `t`, `x`, `v`, `p`.
- A cache READ as input and produced BEFORE the render (like `speech/<lang>/<id>/timing.json`) is fingerprinted by content. That is the existing precedent.
- Caches are machine-local and absent on a fresh clone. A dependency on cache CONTENT violates zero-touch unless a bootstrap or the render generates it on demand (staffage does).
- Relocation: honor `SEMIO_TEACHING_CACHE_ROOT`. Keep cache paths short (MAX_PATH 256).

---

## 6. Python tests

### 6.1 Everything (listed modules, then `.cjs`, then one `.ts`)

```bash
W=/c/git/semio
cd "$W" && bun "$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" test
```

Output root: `$CACHE/tests`. Sets `SEMIO_TEST_MANIM=1`. Bun `require`s these before Python starts:

- `🏗️bim/🎬️clip/🧪️tests/🟨️.cjs`
- `🎯️optimization/🎬️clip/🧪️tests/🟨️.cjs`
- `🦗️grasshopper/🎬️clip/🧪️tests/🟨️.cjs`
- `📊️excel/🎬️clip/🧪️tests/🟨️.cjs`
- `🏢️nakagin/🧪️tests/🟨️.cjs`
- `🏗️bim/🎬️clip/🧱️model/🧪️tests/🧬️loss-selector/🟦️.ts`

Arguments after `test` are APPENDED as more unittest modules. They do not filter.

### 6.2 One family (direct, quick)

All modules have the prefix `🎓️teaching.🏛️architecture.`. Use the same environment as `script.ts`.

```bash
W=/c/git/semio
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
cd "$W" && PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
  UV_PROJECT_ENVIRONMENT="$W/.🧬semio/🎓️teaching/⚡️cache/python" \
  SEMIO_TEACHING_OUTPUT_ROOT=.🧬semio/🎓️teaching/⚡️cache/tests SEMIO_TEST_MANIM=1 \
  SEMIO_TEACHING_CACHE_DIR=.🧬semio/🎓️teaching/⚡️cache/fonts \
  uv run --project "$P" --python 3.12 --group test python -m unittest \
  "🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧪️tests.🐍️" \
  "🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧪️tests.🎭️presentation.🐍️" \
  "🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧪️tests.📇️registry.🐍️"
node "$W/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🟨️.cjs"
```

Replace the module list with one of the sets below.

Module lists (copied from `script.ts`):

- Energy (`⚡️energy`): 30+ modules. Examples: `⚡️energy.🏗️authoring.🎬️clip.🧪️tests.🐍️`, `⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🐍️`, `⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️`, `⚡️energy.🏗️authoring.🎬️clip.📁️output.🧪️tests.🐍️`, `⚡️energy.🏗️authoring.🎬️clip.👥️audience.🧪️tests.🐍️`, `⚡️energy.🏗️authoring.🔤️fonts.🧪️tests.🐍️`, `⚡️energy.🌬️ventilation.🎬️clip.📦️volume.🧪️tests.🐍️`, `⚡️energy.❄️cooling.🎬️clip.🧪️tests.🐍️`, `⚡️energy.📊️demand.🎬️clip.⚖️regulations.🧪️tests.☀️sun.🐍️`. Copy the full set from `script.ts` lines 16-18.
- BIM: `🏗️bim.🎬️clip.🧪️tests.🐍️`, `🏗️bim.🎬️clip.🧪️tests.🎭️presentation.🐍️`, `🏗️bim.🎬️clip.🔗️consistency.🧪️tests.🔄️lifecycle.🐍️`, `🏗️bim.🎬️clip.🧩️models.🧪️tests.📏️footer.🐍️`, `🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🐍️`.
- Nakagin: `🏢️nakagin.🧪️tests.🐍️`. Morph: `🔀️morph.🧪️tests.🐍️`.
- Excel: `📊️excel.🎬️clip.🧪️tests.🐍️`, `📊️excel.🎬️clip.🧪️tests.🎭️presentation.🐍️`, `📊️excel.🎬️clip.🧪️tests.📇️registry.🐍️`.
- Grasshopper: `🦗️grasshopper.🎬️clip.🧪️tests.🐍️`, `🦗️grasshopper.🎬️clip.🧪️tests.🎭️presentation.🐍️`, `🦗️grasshopper.🎬️clip.🧪️tests.📇️registry.🐍️`.
- Optimization: `🎯️optimization.🎬️clip.🧪️tests.🐍️`, `🎯️optimization.🎬️clip.🧪️experiments.🧪️tests.🧊️consumed.🐍️`, `🎯️optimization.🎬️clip.🧬️algorithms.🧪️tests.🧩️widgets.🐍️`, `🎯️optimization.🎬️clip.🧬️algorithms.🧪️tests.📈️comparison.🐍️`.

The `.cjs` files `require("ajv/dist/2020")`, which resolves from the root `node_modules/ajv` (root `package.json` pins `ajv` 8.20.0). Run them with `node` from the workspace root. This works standalone because they execute on load.

Tests in `🧪️tests/` under energy write under `$CACHE/tests`. Do not run the full energy list when only one family changed.

---

## 7. How the family clips import energy authoring visuals

The bim, excel, grasshopper, optimization, nakagin and morph folders do not carry their own runtime. Their `🎬️clip/🐍️.py` wrappers call `importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")` and `…🖌️render.🐍️`. Their scenes import shared helpers the same way:

| Import target (module string) | Used by |
| --- | --- |
| `…⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️` | every family wrapper (`compose`, `main`) |
| `…⚡️energy.🏗️authoring.🎬️clip.🖌️render.🐍️` | every family wrapper `main`, `video_main` |
| `…⚡️energy.🏗️authoring.🎨️visuals.🐍️` (`_art`) | bim visuals, excel visuals, grasshopper visuals, optimization visuals, nakagin `🎨️visuals` |
| `…⚡️energy.🏗️authoring.🔤️fonts.🐍️` (`_fonts`) | bim, excel, grasshopper, optimization |
| `…⚡️energy.🏗️authoring.🎨️theme.🐍️` (`_theme`) | bim, excel, grasshopper, optimization |
| `…⚡️energy.🏗️authoring.🎬️clip.🎥️camera.🐍️` | bim visuals and camera tests |
| `…⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️` (via visuals) | any scene that reaches visuals |

Grasshopper and optimization wrappers are the ones that run several videos at once, so `🦗️grasshopper/🎬️clip/🐍️.py` always selects all five videos. For one video, call the renderer directly:

```bash
W=/c/git/semio
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
cd "$W" && UV_PROJECT_ENVIRONMENT="$W/.🧬semio/🎓️teaching/⚡️cache/python" PYTHONUTF8=1 \
  uv run --project "$P" --python 3.12 python "$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🖌️render/🐍️.py" \
  --video grasshopper_capsule --language en --audience speaker -q l --fps 15 --no-play
```

Note: the family `🐍️.py` wrappers add all of their videos as `--video` arguments. Passing `--video` again does not filter.

---

## 8. Concurrency and current state (snapshot 01:47)

- Untracked: `A/🎨️visuals/🧍️staffage/`, `A/🎨️visuals/🔣️icons/`, `🧍️staffage/🔨️modules/📤️export/🎭️cast/`, `🧍️staffage/🗂️catalog/👤️humans/standing-adult/`, `🧍️staffage/🗂️catalog/🔌️appliances/`, `🧍️staffage/🧪️tests/🎭️cast-export/`, `🧍️staffage/🧫️fixtures/🎭️cast-export/`.
- Modified: many energy clip scripts, `A/🎨️visuals/🐍️.py`, `A/🎨️visuals/🧪️tests/🐍️scenes.py`, `🧍️staffage/🧬️schema/`, and several `🎯️optimization`, `🏗️bim`, `📊️excel` clips.
- Consequence: any render started now will abort if one of those fingerprinted files changes mid-run. Two same-language renders will clobber `s/<n>`.
- `A/🎨️visuals/🧍️staffage/🐍️.py` is an active file. Do not rely on its current API.

---

## 9. Recommendations for agents

1. Verify with 3.3 (direct, guard-free) for one beat, and 3.1 (guarded) before delivery.
2. Run the family test subset in 6.2, not the full `script.ts test`, after a family-local change.
3. Before any staffage change that affects renders, add `source_files()` to `🧍️staffage/🐍️.py` and add it to `segment_state`. Do this together with memoizing `segment_state` per run.
4. Never add a render-time output under `$CACHE` to a fingerprint.
5. Replace hard-coded `Scripts/python.exe` in the prior helpers with `uv run --project`.
6. Render one language at a time per output root.
