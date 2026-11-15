# Render Pipeline For Energy Final Videos (read-only audit)

Scope: how to render one or all energy videos today, preview and final, with concurrency, speech and subtitles, tests and the current machine state. Nothing was rendered, moved or deleted. All paths are relative to `C:\git\semio`. Emoji folder names are kept as in the repo.

## 1. Entry point and what it does

- Entry: `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts`.
  - `render` maps to `🎬️clip/🖌️render/🐍️.py` (`script.ts:33`).
  - `script.ts` provisions native tools (`script.ts:150`), sets `SEMIO_TEACHING_CACHE_ROOT` default `.🧬semio/🎓️teaching/⚡️cache` (`script.ts:144`), output default `<cache>/videos` (`script.ts:146`), then spawns `uv run --project 📦️packages/🐍️python --python 3.12 python <render.py> …` (`script.ts:168`). It sets `SEMIO_TEACHING_LANGUAGE` and `SEMIO_TEACHING_PROFILE` only for the child (`script.ts:154-156`).
  - Ctrl-C / SIGTERM kills the whole process tree (`script.ts:172-194`).
- CLI, `🖌️render/🐍️.py:33-46` (argparse):
  - `--language en|de` (required).
  - `--profile preview|final` (required; no default, `:40`).
  - `--audience speaker|spectator` (repeatable; omitted = both, `:34`, `:63`).
  - `--video <id>` (repeatable; omitted = every manifest lesson, `compose_videos` line 772).
  - `--chapter <scene>` (repeatable).
  - `--jobs N` (default `min(4, cpu-1)`, `:38`).
  - `--output-root`, `--resolution W,H`, `--fps`, `--force`, `--concat-only`, `--log-output`, `--no-play`/`--play`.
  - There is no `--quality` flag. The old `-q/--quality` was replaced by `--profile` and must not be used (ticket CONTRACT.md section 5, line 83).

## 2. Profiles (`🎬️clip/🎚️profile/🔣️.json`)

| Profile | Size | FPS | Morph | LOD | Other |
|---|---|---|---|---|---|
| `preview` | 854x480 | 15 | swap | preview | no antialias, no shadows, budgets capped |
| `final` | 1920x1080 | 30 | full | full | antialias and shadows on, no budgets |

Output names carry the height and fps (`🖌️render`/`⚙️runtime` line 886): `<Video>_<audience>_<lang>_480p15.mp4` for preview, `_1080p30.mp4` for final. If `--resolution` is given the name uses `WxH`.

## 3. Energy videos (manifest `🎬️clip/🔣️.json`, energy)

| `--video` id | Manifest line | Output stem (`Full_…`) |
|---|---|---|
| `physical_fundamentals` | 5 | `Full_Physical_Fundamentals` (line 46) |
| `ventilation` | 49 | `Full_Ventilation` (line 171) |
| `heating` | 174 | `Full_Heating_Demand` (line 356) |
| `cooling` | 359 | `Full_Cooling_Demand` (line 501) |
| `energy_balance` | 504 | `Full_EnergyBalance` (line 651) |

Chapter ids per video are in each family README (for example `Heating_01_Introduction` … `Heating_06_FinalCalculation`). Only the id is passed to the CLI. The chapter must belong to the selected lesson (`compose_videos` line 783-785).

## 4. Commands

Run from `C:\git\semio` (the workspace root). The README form in each family's `README.md` is the same:

Preview, one video, German, both audiences:

```
repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --language de --video heating --profile preview --jobs 4 --output-root ./videos --no-play
```

Final, one video:

```
repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --language de --video heating --profile final --jobs 4 --output-root ./videos --no-play
```

One audience only: add `--audience speaker` (or `spectator`).

All five energy videos, one call (manifest order is kept, `compose_videos` lines 786-791):

```
repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --language de --profile preview --video physical_fundamentals --video ventilation --video heating --video cooling --video energy_balance --jobs 4 --no-play
```

Same with `--profile final` for finals, and `--language en` for English chrome.

Caveats:

- `--output-root ./videos` in the README writes to `C:\git\semio\videos`, not under the cache. Omit it to use `.🧬semio/🎓️teaching/⚡️cache/videos`.
- Without `--video` the call renders all 26 lessons including BIM, grasshopper, excel and optimization.
- Each audience renders its own full set: the default writes both speaker and spectator (twice the work).

Current course deliverable (26 lessons, not a `script.ts` command). From the ticket `☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️r3-media/`:

```
repo tools exec uv -- run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --python 3.12 python <r3-media>/manifest.py en speaker
repo tools exec uv -- run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --python 3.12 python <r3-media>/course_preview.py en
```

(`☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/join-en.ps1:24-27`; `☀️03/.../work-plan.md:57` says final publication will invoke these two calls directly, and `FINAL-PREVIEW-READINESS.md:3` says `join-en.ps1` is historical.) Output: `.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/preview/Architecture_course_speaker_en_480p15.mp4` (`☀️03/.../ACTIVE-RUNS.md:86`). Stated format: 854x480, 15 fps, 48 kHz stereo, 3:59:47 (`RENDER-TIME.md:3`).

Stale scripts, do not reuse as-is:

- `☀️03/.../render-en.ps1:29` and `parallel-demand.ps1:36` pass `--quality l --fps 15` and no `--profile`. The current parser rejects `--quality` and requires `--profile` (`🖌️render/🐍️.py:33-46`; CONTRACT section 5). Replace with `--profile preview`.
- `ACTIVE-RUNS.md` and `RENDER-ADMISSION.md` describe runs from 2026-11-07/08 for BIM, GH and Excel. Those runs are not visible now (section 8).

## 5. Where outputs land

Root: `<output-root>` = `--output-root` or `SEMIO_TEACHING_OUTPUT_ROOT` or `.🧬semio/🎓️teaching/⚡️cache/videos` (`📁️output/🐍️.py:18-21`). Per run (`compose_videos`, runtime lines 776-780):

- Final lesson movie: `<root>/<profile>/<lang>/<audience>/rendered/<Output>_<audience>_<lang>_<480p15|1080p30>.mp4`, with `.render.json` and `.export.json` provenance and `posters/` (runtime 780, 886-898; posters from `_export.poster_policy`).
- Chapter masters: `<root>/<profile>/<lang>/<audience>/rendered/<Chapter>_<audience>_<lang>.mp4` (runtime 869-871, only with `--chapter`).
- Logical chapter clips: `<root>/<profile>/<lang>/<audience>/<index>/v/<Chapter>.mp4` (runtime 791, 797).
- Source beat segments (rendered once per source): `<root>/<profile>/<lang>/<audience>/s/<ordinal>/v/<Scene>.mp4` (runtime 779, 799).
- Manim working dirs: `<root>/m/` with `v`, `p/<scene>`, `i`, `t`, `x`, `logs` (`📁️output/🐍️.py:8-15`).
- Narration and timing: `<root>/speech/<identity>/` (`📁️output/🐍️.py:24-28`). Subtitle traces: `<root>/trace/<identity>/<scene>/🔣️.json` (runtime 603-607).
- Per-attempt logs with `--log-output`: `…/attempt-<ns>.log` plus `.json` with `command`, `pid`, `started`, `finished`, `exit_code` (runtime 214-226, 569). These are the best source for real render times.

## 6. Render time

- Published 480p15 speaker `Full_Physical_Fundamentals_speaker_en_480p15.mp4`: 1085.2 s of video (`☀️03/.../published-visual-audit.md:9`). Wall time is not recorded in any ticket I read.
- Energy wall times: not recorded in the tickets read. Measure with the `.json` attempt records above.
- Evidence of slow scenes in other families: `Beat6_Consistency` late frames at about 2.5 h each (`RENDER-TIME.md:7-11`); grasshopper tower 157-174 s per frame (`work-plan.md:204`). The 1080p30 BIM masters dated 2026-10-28 are the only 1080p30 course masters mentioned (`RENDER-TIME.md:3`). No energy 1080p30 final was found in the tickets read.

## 7. Concurrency, locks, reuse

- Within one process: `--jobs` is the global limit on concurrent segments (`🖌️render/🐍️.py:38`; `execute_jobs` runtime 237-260). A `ToolSession` owns all child processes and kills them on cancel (runtime 181-196, 162-178).
- One render per source: `source_plan` keeps the first job for each `source_ordinal` (runtime 98-103). Partial-source sources require their trace (runtime 808-813).
- Fingerprints: segment state = settings + profile fingerprint + sha256 of every authored input (runtime 409-420). A clip is reused only if its sidecar matches (runtime 549-554, 831-832, 889). Authored inputs are re-hashed after the render and after assembly; a change raises `Authored sources changed while rendering` or `… while assembling` (runtime 580-581, 855-857, 892-894).
- Atomic publish: outputs are written as `.partial` and renamed (runtime 247, 262, 846-851, 264).
- Exclusive log creation: `log.open("x")` (runtime 204).
- Same-language collision: there is no cross-process lock in the Python code. The segment and chapter directories depend only on profile, language and audience (runtime 776-791). Two processes with the same profile + language + audience write the same `s/<ordinal>` and `<index>` folders and clobber each other. This matches the stored memory note on same-language parallel renders. Different languages or audiences are safe.
- The only lock is a ticket convention: `.🧬semio/…/videos/en/.media-lock` (a directory) taken by `☀️03/.../render-en.ps1:10-15` and released in `finally` (`:37`). `join-en.ps1:9,18-19` waits for it to disappear. `parallel-demand.ps1` header says it does not take the lock (line 2).
- Stale lock now: `videos/en/.media-lock` exists, created 2026-11-03 23:43, and no render process is running (section 8). It will stall `join-en.ps1` forever until removed. Remove only after confirming that no `en` render is alive.
- Memory admission: `repo capacity run --memory=24576 --cpu=1 -- …` for the full suite (`ACTIVE-RUNS.md:76`); render admissions of 16384 and 8192 MB (`RENDER-ADMISSION.md:3, 21`). `parallel-demand.ps1:18, 26-28` admits a new render only when more than 8 GB is free and at most two run.

## 8. Speech, TTS and subtitles

- Narration is a list of clauses `(section, text_en, text_de)` in each scene, for example heating fundamentals `NARRATION` at `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:451-457`. The type is `Clause = tuple[str, str, str]` (`🔊️speech/🐍️.py:44`).
- Subtitles on screen: `subtitle_text` returns `tr(text_en, text_de)`, so the caption language follows `SEMIO_TEACHING_LANGUAGE` (`🎨️visuals/🐍️.py:2035-2040`; read-time sizing 2043-2048).
- Voice: `subtitle_clauses` returns `text_de.strip()` only (`🔊️speech/🐍️.py:88-90`), and `synthesize_clause_audio` sends that string to the TTS backend (`:265-298`). The voice is therefore always the German text, even for `--language en`. This is a mismatch with English captions that should be checked in a real render.
- Pronunciation or display split: none exists. The TTS input and the on-screen string are the same field (`text_de`). There is no spoken-form field and no replacement table for symbols; a grep for pronunciation, spoken and speech-text variants found nothing. So "Δθ" on screen is sent to the TTS as "Δθ"; the provider decides how it sounds. A "Delta Theta" spelling would be read literally. Reviewer F11 (`FEEDBACK.md:25`) asks for Δθ on screen and everywhere; the code cannot separate the two without a new field. The string `Δ` appears 7 times in the heating fundamentals file (count only; not all are narration).
- Timing: rendering with `VO_TRACE=1` (runtime 570) records when each subtitle appears; `attach_source_narration` mixes each clause at its measured start (runtime 584-592; `🔊️speech/🐍️.py:141-145`, clamped so clips do not overlap).
- Provider and key: default model `google/gemini-3.1-flash-tts-preview` via OpenRouter (`🔊️speech/🐍️.py:46, 82-83`). Keys come from `🎓️teaching/.env` or the environment (`:47, 58-85`).
- Trap: when no backend has a key, `synthesize_clause_audio` returns silent clips with length 0 (`:272-274`). The render does not fail; the video is silent.
- Audience: `speaker` overlays subtitles, `spectator` hides them, speech is the same (`🎬️clip/👥️audience/🔣️.json:4-5`; `🎬️clip/README.md:5-6`).

## 9. Tests for energy clips

- Full suite: `repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test`. It runs a fixed unittest list (`script.ts:37-126`) plus JS tests (`script.ts:138-140`), with `SEMIO_TEST_MANIM=1` and `SEMIO_TEACHING_PROFILE=final` (`script.ts:154-156`).
- Energy modules in that list: presentation, browser, audience, output and clip-level tests (`script.ts:87-99`); visuals tests (`:99-109`); ventilation (`:110-111`), heating (`:113`), cooling (`:114-117`), demand (`:70`, `:116`), and energy visuals/staffage tests (`:118-121`).
- One module, the ticket runner style (`☀️11/DECLARATIVE-…/run-tests.sh:18`): `uv run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --python 3.12 --group test python -m unittest <dotted.module>`, with `SEMIO_TEST_MANIM=1` and a test output root under the ticket's `🗑️generated`. Dotted names use the emoji folder names, for example `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🧪️tests.🐍️`.
- Language-agnostic JSON cases sit beside the tests (for example `🎬️clip/🧪️tests/🔣️.cases.json`).

## 10. Machine state (read-only checks, this host, 2026-11-13)

- No `manim`, `ffmpeg`, `🖌️render` or `script.ts render` process is running. `Get-CimInstance Win32_Process` and `tasklist` were checked.
- Alive, not renders: 8 `uv`/`python -m unittest` processes for `🔀️morph.🧪️tests.📍️placement` and `🖼️preview` (run through the ticket `run-tests.sh` in `☀️11/DECLARATIVE-…`). Their creation time is 2026-11-11 04:54 and 05:03, two days old.
- Alive, not renders: `repo …/📜️script.ts run` (root script, started 2026-11-13 04:14) and the `repo dev mcp` / `bun` MCP servers.
- `ACTIVE-RUNS.md` (`☀️03`) and `RENDER-ADMISSION.md` (`☀️03`) describe BIM, GH and Excel runs from 2026-11-07/08 with PIDs and reservations. None of those processes is alive. Treat them as history.
- `☀️11/ARCHITECTURE-COURSE-REDEPLOY-READINESS/readiness-report.md` describes a different host (macOS) where an `en` speaker render was running on 2026-11-10. It does not apply to this machine.

## 11. Things to fix before a final run

1. Remove the stale `videos/en/.media-lock` after confirming no `en` render is alive, or `join-en.ps1` will wait forever.
2. Do not use `render-en.ps1` or `parallel-demand.ps1` as written (`--quality`, no `--profile`).
3. Stop the two-day-old morph unittests if they are not needed; they compete for RAM and CPU with renders.
4. `🎬️clip/README.md:28` still says the complete lesson includes an institutional intro; the energy curriculum no longer has one (see the intro audit).
5. Decide the voice language for `--language en`: the TTS reads `text_de` always (speech 88-90), while captions follow `tr()`.
6. The `final` profile output path is documented but no energy 1080p30 final is evidenced in these tickets. Run one video as final and check its `.export.json` before a full batch.
