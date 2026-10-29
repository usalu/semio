# 🎬 Render How-To: BIM and Optimization Clips, and the Nine Grasshopper and Excel Videos

Read-only research for this ticket. Sources are the `CONCRETE-OPTIMIZATION-CLIP-EXAMPLES` and `CONCRETE-BIM-CLIP-EXAMPLES` tickets (ticket `☀️27`), the teaching script, the shared renderer and the deliver script. Nothing was rendered or edited.

Ticket root (T): `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS`

## 1. Environment

- **Workspace root:** `C:/git/semio`. Always `cd /c/git/semio` first and quote emoji paths.
- **Teaching script:** `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts`. It is a Bun script. It is normally started through `repo <script.ts>`, but the `repo` binary is missing on this host, so run it with `bun` directly (the CONCRETE-OPTIMIZATION `environment.md` says the same).
- **Required on PATH:** `bun` and `uv`. `script.ts` spawns `uv run --project 📦️packages/🐍️python --python 3.12 python <renderer>`. Its Python environment is `.🧬semio/🎓️teaching/⚡️cache/python` (Manim 0.22.0, NumPy, SciPy).
- **Repo shim (needed because `repo` is missing):** `SEMIO_REPO_BINARY` must point at a runnable command. The browser presentation layer (`🪟️presentation/🌐️browser/🐍️.py`, `_repo()`) reads this variable first, so the stale `native-repo-path` marker is ignored when it is set.
  - Shim files already exist in the optimization ticket: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd` (calls `repo-shim.mjs`).
  - The shim accepts only `tools exec --light node -- <args>` and runs `node` directly. Any other form exits with code 2.
  - Consequence: the render path only needs the `node` form. The chapter test (`🧩️chapters/🐍️.py` line 287, `tools exec --light python`) and the output test (`git` form) will fail through this shim. Use the shim for renders only, or write a sibling shim for those forms.
  - Use the shim as-is. Do not copy it into T unless a change is needed.
- **TMP and TEMP:** set both to a folder inside the workspace, for example `T/🗑️generated/tmp`, before running any test or render. Some runtime tests need this (team memory note; the exact tests were not traced here).
- **Python for deliver.py:** use the cache interpreter `.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe`. `deliver.py` imports `av` and `imageio_ffmpeg`, which are not in the documented package list (Manim, NumPy, SciPy, rhino3dm, openpyxl, formulas). Check the import before the first run. The interpreter used for the BIM and optimization delivery is not recorded.
- **Never run `python -` on stdin** on this host (it hangs). Run Python from files.

## 2. How the BIM and Optimization Videos Were Rendered (1080p30)

Source: `render-r2.md` in the optimization ticket, `🖌️render/🐍️.py`, the BIM `README.md` render section, and the `.render.json` certificates.

**Render commands (from the ticket, with the shim):**

```bash
export SEMIO_REPO_BINARY="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd"
bun "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render --video <id> … --language <en|de> --quality h --fps 30 --no-play
python deliver.py <en|de> <speaker|spectator>
```

- `--language` is required. `--audience` is optional; when omitted, the renderer runs speaker and then spectator, sequentially.
- `--quality h` selects the Manim high preset (1920×1080). `--fps 30` is required for 30 fps. The certificates record `fps: 30.0`. Without it, the Manim default for `-qh` is not 30 fps; this is a Manim default, not checked here.
- `--no-play` avoids opening a player.
- `--output-root <dir>` overrides the output location. Without it, `SEMIO_TEACHING_OUTPUT_ROOT` is used, and by default that is `.🧬semio/🎓️teaching/⚡️cache/videos`.
- Other useful flags from `🖌️render/🐍️.py`: `--log-output` (immutable per-segment attempt logs), `--chapter <Scene>` (re-render one chapter; the scene name is the manifest chapter scene, for example `Grasshopper_01_Anatomy`), `--jobs N` (global concurrent segment limit; default `min(4, cpus-1)`), `--force`, `--concat-only`.
- Ctrl+C or a stop cancels the whole process tree. Completed segments are reused by the next run.

**Video IDs used (from `🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json`):**

- BIM: `bim_information`, `bim_objects`, `bim_representations`, `bim_models`, `bim_exchange`, `bim_consistency`
- Optimization: `optimization_extrema`, `optimization_sampling`, `optimization_experiments`, `optimization_index`, `optimization_pareto`, `optimization_algorithms`
- New (already registered in the manifest, one chapter each): `grasshopper_anatomy`, `grasshopper_capsule`, `grasshopper_tower`, `grasshopper_casting`, `grasshopper_trees`, `excel_anatomy`, `excel_references`, `excel_program`, `excel_energy`

**Output locations.** `<lang>` is `en` or `de`; `<aud>` is `speaker` or `spectator`.

- Masters (renderer output, one MP4 plus a `.render.json` certificate per video):
  `.🧬semio/🎓️teaching/⚡️cache/videos/<lang>/<aud>/rendered/<Output>_<aud>_<lang>_1080p30.mp4`
  Example: `…/videos/en/speaker/rendered/BIM_information_speaker_en_1080p30.mp4` (about 95–157 MB each in the current cache).
- The manifest `output` value sets `<Output>`. For the new videos it is `Grasshopper_anatomy`, `Grasshopper_capsule`, `Grasshopper_tower`, `Grasshopper_casting`, `Grasshopper_trees`, `Excel_anatomy`, `Excel_references`, `Excel_program`, `Excel_energy`.
- Segment work and traces: `…/videos/<lang>/<aud>/s/<n>` (slot directories; the source of the concurrency failure below), plus `…/videos/<lang>/<aud>/` intermediates and `…/videos/trace`.
- Delivery (H.264 exports plus provenance): `.🧬semio/🎓️teaching/⚡️cache/videos/<lang>/<aud>/delivery/<Output>_<aud>_<lang>_1080p30.mp4` and `…/delivery/delivery.json`.

**Speaker vs spectator.** Both are variants chosen by `--audience` and implemented in the shared audience module (`👥️audience/🐍️.py`). Each audience has its own master set and its own delivery set. This ticket did not inspect the presentation differences.

**Durations.**

- Final clip length from the R2 delivery: 2:42 to 5:43 per video (for example, de spectator BIM exchange is 162 s and optimization algorithms is 343 s). The grasshopper and excel agent rules target 3 to 6 minutes each.
- Render wall-clock time was not recorded in the BIM or optimization certificates. The only measured figure is a single optimization scene estimated at about two hours for 1080p30 before the final fixes (`📌️important/📝️.md`, optimization ticket). Treat render time as unknown until measured. The `--log-output` attempt logs carry a `finished` epoch per segment, which can be used to derive duration.
- Per-language delivery size for the 12 BIM and optimization videos: en speaker 271 MB, en spectator 243 MB, de speaker 276 MB, de spectator 245 MB.

## 3. deliver.py: What It Does

File: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/deliver.py`. Usage: `python deliver.py <en|de> [speaker|spectator]`.

1. Finds the workspace root (the folder containing `🧰️framework`).
2. For each entry in `NAMES` (a map from manifest ID to output name), it exports the master with bundled ffmpeg (`imageio_ffmpeg.get_ffmpeg_exe()`): H.264 High, level 4.1, yuv420p, CRF 18, preset fast, 4 threads, `+faststart`, audio copied (`-c:a copy`). File: `<Output>_<aud>_<lang>_1080p30.mp4`.
3. Decodes the export and the master with PyAV and checks that the frame count matches the master and the size is 1920×1080. It aborts with `SystemExit` otherwise.
4. Writes `delivery.json` (schema `semio.architecture.delivery/v1`). Each record has: id, file, sha256 of the export, byte size, frame count, duration in seconds, the master path and its sha256, and the certificate path and its sha256. The file header records encoder settings, fps, and the audio declaration (AAC LC, 48 kHz, stereo, copy, no spoken narration).
5. The stdout encoding is reconfigured to UTF-8. The earlier exit code 1 (cp1252 console crash on an emoji path, after `delivery.json` was already written) is fixed by this.

**Gap for the new videos:** `NAMES` currently covers only `bim_*` and `optimization_*`. Before delivery, add the nine entries. The ticket copy should be made in T and kept as the ticket's own script (not edited in place in the optimization ticket):

```python
**{f"grasshopper_{name}": f"Grasshopper_{name}" for name in ("anatomy", "capsule", "tower", "casting", "trees")},
**{f"excel_{name}": f"Excel_{name}" for name in ("anatomy", "references", "program", "energy")},
```

The delivery loop then produces `Grasshopper_anatomy_speaker_en_1080p30.mp4` and so on, with `"id"` set to the manifest ID.

## 4. Known Failures and Fixes

| # | Symptom | Cause | Fix |
|---|---|---|---|
| 1 | `repo` not found, or `native-repo-path` points at deleted `repo.exe` | `repo` binary missing on this host; repo MCP server down | Set `SEMIO_REPO_BINARY` to the shim (see section 1). Shim only handles `tools exec --light node --`. |
| 2 | Shim exits 2 with `unsupported command` | A non-node `tools exec` call (python or git form) | Render path does not use those forms. For tests, add a shim form or skip those tests. |
| 3 | `FileNotFoundError: '<none>'` in Manim `combine_files` (PyAV opening the in-memory concat manifest); partials disappear | Two `render` runs for the same language at once share `videos/<lang>/<aud>/s/<n>` slots | Never run two renders for the same language at the same time. Different languages are safe (separate folders). The renderer's retry recovered some segments; the rest needed a rerun. |
| 4 | `ValueError` about browser work directory length (over 220 characters) or outside the workspace | Browser presentation work path rule in `_call` | Keep the default output root. Do not pass a deep `--output-root`. |
| 5 | deliver.py exits 1 after writing `delivery.json` | Printing an emoji path to a cp1252 console | Fixed in deliver.py (`stdout.reconfigure(encoding="utf-8")`). Keep it in the ticket copy. |
| 6 | Stale tail bytes or corrupt output after a scripted rewrite | Scripted rewrite of a file that a running Bun process has open (team memory: Windows loaded-file truncation) | Edit source files only with the Edit or Write tools, and never while a render is running. |
| 7 | Speaker certificate looks stale, or the video shows old text | Certificates hash the source files of each scene; an edited source invalidates them | Re-run that video (`--video <id>`); the renderer reuses only matching segments. |
| 8 | Render fails on a missing `uv` or Python environment | `script.ts` runs `uv run --project …`; cache Python is under `.🧬semio/🎓️teaching/⚡️cache/python` | Install or check `uv` on PATH; do not change the cache layout. Not verified here. |
| 9 | Bash tool times out on a long render | The Bash tool has a 2-minute foreground limit | Run the render with `run_in_background` and wait for the completion notice. Do not poll the output file. |
| 10 | A single scene fails after many succeed | Scene bug, not a pipeline bug | Re-run one video, or one chapter with `--chapter <Scene>`. Completed segments are reused. |

## 5. Command Sequence for the Nine New Videos (en and de, speaker and spectator)

Rules applied: en and de run in parallel (separate language folders). Within one language, speaker runs before spectator and nothing else renders that language. Logs go to `T/🗑️generated/render/`. The delivery step runs after each audience's render.

```bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
SCRIPT="🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts"
PY=".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
SHIM="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd"
VIDEOS="--video grasshopper_anatomy --video grasshopper_capsule --video grasshopper_tower --video grasshopper_casting --video grasshopper_trees --video excel_anatomy --video excel_references --video excel_program --video excel_energy"
export SEMIO_REPO_BINARY="$SHIM"
export TMP="$PWD/$T/🗑️generated/tmp" TEMP="$PWD/$T/🗑️generated/tmp"
mkdir -p "$T/🗑️generated/render" "$TMP"
# Step 0 (once): add the nine NAMES entries to "$T/deliver.py" as shown in section 3 (the ticket copy, not the optimization ticket file).

run_lang() {
  lang=$1
  for aud in speaker spectator; do
    bun "$SCRIPT" render $VIDEOS --language "$lang" --audience "$aud" --quality h --fps 30 --no-play --log-output \
      > "$T/🗑️generated/render/render-$lang-$aud.log" 2>&1 || return 1
    "$PY" "$T/deliver.py" "$lang" "$aud" > "$T/🗑️generated/render/deliver-$lang-$aud.log" 2>&1 || return 1
  done
}
run_lang en &
run_lang de &
wait
```

Run the whole block with `run_in_background`. It is a single shell with two background subshells and a `wait`.

**Verify after completion:**

- Nine masters per language and audience: `ls .🧬semio/🎓️teaching/⚡️cache/videos/<lang>/<aud>/rendered/` and look for `Grasshopper_*_<aud>_<lang>_1080p30.mp4` (5 files) and `Excel_*_<aud>_<lang>_1080p30.mp4` (4 files).
- `delivery/delivery.json` lists 9 records with matching frame counts and 1920×1080.
- Logs: `render-*.log` ends with no traceback; `deliver-*.log` ends with `[DEBUG] delivered 9 videos to …`.

**Run times.** Not measured. Expect about 2 to 3 hours per language if speaker and spectator run back to back at default `--jobs`, based on the single-scene estimate in section 2. Treat this as unverified.

**Cleanup at ticket close.** Per the ticket rules, delete the `T/🗑️generated/` folder after the logs are reviewed. Keep the scripts (`deliver.py`) and the reports in T.

## 6. Unverified or Open Items

- Whether the nine grasshopper and excel scene modules pass their tests. The ticket audits were not inspected here, and `script.ts test` runs every suite in the repository (heavy).
- Exact interpreter that produced the BIM and optimization deliveries, and whether `av` and `imageio_ffmpeg` are present in the cache Python.
- Render wall-clock per video (not recorded anywhere reviewed).
- Whether `uv` is installed on this host (required by `script.ts`).
- The presentation difference between speaker and spectator (not inspected).
