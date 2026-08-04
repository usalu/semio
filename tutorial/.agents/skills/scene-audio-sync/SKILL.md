---
name: scene-audio-sync
description: >-
  Writes continuous slow narration transcripts across animation scenes, synthesizes
  per-scene audio, and muxes it in sync with rendered clips. Use when the user asks
  to add/regenerate scene audio, sync voiceover to animation, prepare a transcript
  for a job, or invokes scene-audio-sync / "make audio for scenes".
disable-model-invocation: true
---

# Scene Audio Sync

Agent-owned narration + TTS + mux. **You** write every transcript (no OpenRouter / planner / codegen LLM). Scripts only synthesize speech and sync video.

## Prerequisites

Install once per machine/repo (see [skill.toml](skill.toml)):

```bash
# Python deps (from anywhere)
python3 -m pip install 'edge-tts>=6.1.0' 'httpx>=0.27.0'

# Or from this skill directory (modern pip):
#   python3 -m pip install .

# System binary required
command -v ffmpeg && command -v ffprobe
# macOS: brew install ffmpeg
```

Verify: `bash scripts/install_check.sh`

## Inputs

Resolve the job first. Accept any of:

- Explicit job id / path: `artifacts/<job_id>/`
- Open files under a job folder
- User phrase like “audio for 6115aa2db012”

Expected layout (NowIGetIt-compatible; also works if paths are passed explicitly):

```
artifacts/<job_id>/
  scene_plan.json          # or transcript will stand alone
  scenes/<scene_id>/
    section.json
    code_final.py          # optional, used for visual cues
    scene.mp4              # preferred silent/render clip
    scene_vo.mp4           # existing VO (may overwrite)
```

## Progress checklist

Copy and update while running:

```
- [ ] 1. Inspect job + all scenes
- [ ] 2. Write master transcript (agent-authored)
- [ ] 3. Synthesize per-scene / per-beat audio
- [ ] 4. Mux + sync each scene to animation
- [ ] 5. Verify sync + optional final stitch
- [ ] 6. Report paths + durations to user
```

## Step 1 — Inspect

```bash
python3 scripts/inspect_job.py <job_dir> --out <job_dir>/audio_work/inspect.json
```

Read `inspect.json` fully. For each scene note: id, title, visual_description, beats (if any), code cues, **measured video duration**.

## Step 2 — Write the transcript (YOU do this)

Do **not** call OpenRouter, the repo planner, or any external chat model to draft narration.

Write `<job_dir>/audio_work/transcript.json` yourself using the schema in [reference.md](reference.md).

Rules (voice must be slowly continuously understandable):

1. Read **all** scenes before writing any line — one continuous lesson, not isolated clips.
2. Slow pacing: short sentences, one idea per beat, spoken rate ~120–140 wpm equivalent (prefer fewer words over denser jargon).
3. Continuity: scene N may briefly recall scene N−1; never re-introduce the whole topic.
4. Sync to animation: each beat’s line must match what is on screen in that window (labels, transforms, equations).
5. Prefer measured `video_duration_seconds` as the budget. If video is silent/short, size narration to the visual beats and allow freeze-frame on mux.
6. Per beat: `narration` is what is spoken **while** that visual action happens.
7. Full scene narration = beats joined with spaces; no stage directions, no “in this scene…”, no “as you can see”.
8. Language: match job/user language (default `en`).

Also write a human-readable `<job_dir>/audio_work/transcript.md` mirroring the JSON for review.

## Step 3 — Synthesize audio

Default engine: **edge-tts** (no OpenRouter). Voice defaults: slow rate.

```bash
python3 scripts/synthesize_audio.py <job_dir>/audio_work/transcript.json \
  --out-dir <job_dir>/audio_work \
  --voice en-US-JennyNeural \
  --rate=-20%
```

Produces:

- `audio_work/scenes/<id>/audio.wav` (full scene)
- `audio_work/scenes/<id>/beats/beat_XX.wav` when multiple beats
- updates transcript with measured `audio_duration_seconds` / per-beat durations

If edge-tts is unavailable and user provides `OPENROUTER_API_KEY` / `TTS_*`, pass `--engine openrouter` (optional fallback only).

## Step 4 — Mux + sync

```bash
python3 scripts/mux_sync.py <job_dir>/audio_work/transcript.json \
  --job-dir <job_dir> \
  --publish
```

Behavior (matches repo philosophy):

- If audio longer than video → freeze last frame until narration ends
- If video longer than audio → pad audio with silence
- Never cut the voice mid-sentence (`-shortest` only as last-resort fallback inside the script)
- Writes `scenes/<id>/audio.wav` + `scenes/<id>/scene_vo.mp4` when `--publish`
- Optional burn-in subtitles: `--subtitles`

## Step 5 — Verify

```bash
python3 scripts/verify_sync.py <job_dir>/audio_work/transcript.json --job-dir <job_dir>
```

Fail / warn if |video−audio| > 1.2s after mux (post-mux clip should cover both). Re-edit transcript words (shorten/lengthen) and re-run steps 3–4 rather than speeding voice unnaturally.

Optional final concat of all `scene_vo.mp4`:

```bash
python3 scripts/mux_sync.py <job_dir>/audio_work/transcript.json --job-dir <job_dir> --final
```

## Step 6 — Report

Tell the user:

- Path to `transcript.md` / `transcript.json`
- Per-scene: audio path, VO path, audio duration vs video duration
- Any scenes that needed freeze-frame or rewording

## Hard rules

- Agent authors all narration text; scripts never invent dialogue.
- Prefer slow, clear voice (`--rate=-15%` to `--rate=-25%`) over fast catch-up.
- Keep one voice for the whole job.
- Do not delete existing `scene.mp4` sources; publish VO beside them.
- Portable: skill must work when copied to another repo that has `ffmpeg` + deps from `skill.toml`.

## Additional resources

- Schema + style examples: [reference.md](reference.md)
- Install / deps: [skill.toml](skill.toml) (pip: [pyproject.toml](pyproject.toml))
- Copy/run examples: [examples.md](examples.md)
