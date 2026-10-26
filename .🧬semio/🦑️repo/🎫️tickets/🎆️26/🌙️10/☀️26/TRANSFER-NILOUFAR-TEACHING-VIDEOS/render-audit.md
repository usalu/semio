# Teaching Video Render Audit

Source: `C:/git/semio.worktrees/🦢niloufar/⛳wip/tutorial`. Read-only exploration on 2026-10-26; no renders or dependency installations performed.

## Infrastructure and Commands

Five curriculum series live under `tutorial/energy/demand`: `1_physical_fundamentals`, `2_ventilation`, `3_heating`, `4_cooling`, `5_energy_balance`. Each has a `full_*_video.py` entry point. Run with Python from the configured environment, for example `python tutorial/energy/demand/5_energy_balance/full_energy_balance_video.py -q l --no-play --force`. Qualities l/m/h correspond to 480p15, 720p30, 1080p60. Other wrappers generally expose the same flags, with `--concat-only` on several wrappers. Inspect exact CLI after transfer before invoking.

These entry points render intro plus individual chapters or beats and concatenate through ffmpeg. Their deliverables are intentionally silent (`-an` in concat). No API keys are needed for silent rendering. Narrated deliverables are generated outputs, not required source assets.

Avoid the single full-series Manim scene: documented Manim mega-combine failure can raise InvalidDataError when combining hundreds of partials. Separate chapter renders then ffmpeg concat are the recommended pipeline.

## Dependencies

Tutorial pyproject requires Python >=3.11 and declares manim>=0.18, manimgl>=1.6.1, watchdog>=4, numpy>=1.26, gtts>=2.5, openai>=1. Tutorial animation source primarily targets Manim Community. ffmpeg and ffprobe are external executables. Current Windows PATH exposes Python 3.14 and uv, but no manim, ffmpeg or ffprobe. A supported pinned Python plus managed renderer tools are needed; do not assume all packages support the host Python 3.14.

## Outputs and Cache

Outputs normally live beside each series in `media/videos/full_*_video/{480p15,720p30,1080p60}/Full*.mp4`, with copied deliverables under `rendered/Full_*_<quality>.mp4`. Energy balance also writes an explicitly named NoAudio copy. Exact names differ by series.

Wrappers reuse existing MP4 files based on quality and size >1000 bytes, not a source-content fingerprint. Use `--force` for a verified new render after transfer. `--concat-only` still imports Manim because imports occur before argument dispatch.

## Windows and Transfer Hazards

Many entry points look for `.venv/bin/manim` then PATH; most chapter audio trace scripts directly require the POSIX path. Prefer the configured Python executable with `-m manim` through the repository tool interface.

Many chapter `build_full_video.py` scripts import backend.pipeline modules from hardcoded `/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt`. Those are not self-contained. Cooling chapter 1 is already self-contained through tutorial/tts_pipeline.py. The Physical Fundamentals build script also depends on that external backend. Silent full-series wrappers avoid those backend imports.

Intro and module paths explicitly include `tutorial`; moving source into the current teaching taxonomy requires updating those paths together. Concat list writing should normalize paths for ffmpeg on Windows and escape apostrophes. Keep generated paths below 256 characters; individual partials can add substantial depth.

## Verification and Scene Duration Estimate

No test-named files or nested AGENTS.md were found inside tutorial. `LAYOUT_CHECK=1` invokes shared `layout_conflicts`/`check_layout` at settled narration clauses, but diagnostics are nonfatal. Rendering must inspect diagnostics and final media properties rather than treating exit success alone as visual correctness.

Scene duration is content-driven, not a fixed length. manim_visuals.py declares spoken English 2.5 words/sec, German speech 2.2 words/sec, and silent-caption reading 1.6 words/sec with a 1.5-second minimum per nonempty caption. Estimate each beat from its German NARRATION word count divided by 1.6, then account for animation and staging time; measured timing manifests may lengthen that estimate. Exact series duration needs a renderer dry run or ffprobe of newly rendered media. Five full curriculum videos contain many chapters and beats, so do not assume a short test clip represents end-to-end render time. 1080p60 processes 18 times the pixel-frame volume per timeline second of 480p15, before renderer overhead; use low quality for initial validation and high quality for final delivery if requested.

## Optional Voice Pipeline

Per chapter, `python generate_audio.py` synthesizes by default (there is no --synth flag), then `--trace`, `--align`, `--report`; `--beat N` limits scope. `VO_TRACE=1` records subtitle starts. tts_pipeline.py uses a repository-owned Protocol and stdlib HTTP, defaults to OpenRouter Gemini TTS, and loads tutorial/.env. Optional NowIGetIt dotenv integration runs only when that external folder exists. Voice configuration is unnecessary for this task's silent rendering.

## Tooling Limitation

Cached `repo.exe` is MCP-only. `semio-repo.exe` offers CLI but no generic file read/exec surface; query and GraphQL content attempts stalled. Parent authorized direct filesystem read-only inspection to complete this audit. No modifying Git commands or worktree operations were used.
