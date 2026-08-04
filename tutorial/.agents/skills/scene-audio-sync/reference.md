# Scene Audio Sync — reference

## transcript.json schema

```json
{
  "job_id": "6115aa2db012",
  "title": "Cooling Load with Sun Protection: Solar Radiation & Glass",
  "language": "en",
  "voice": "en-US-JennyNeural",
  "rate": "-20%",
  "pitch": "+0Hz",
  "scenes": [
    {
      "id": "scene_1",
      "title": "Raw Solar Power",
      "video_path": "scenes/scene_1/scene.mp4",
      "video_duration_seconds": 12.4,
      "target_duration_seconds": 12.4,
      "beats": [
        {
          "index": 0,
          "visual_action": "Sun rays approach the cyan glass facade",
          "narration": "Sunlight hits the building first as raw solar power.",
          "approx_seconds": 4.0,
          "audio_duration_seconds": 0.0
        },
        {
          "index": 1,
          "visual_action": "Highlight the window aperture",
          "narration": "Only the glass opening can turn that power into indoor heat.",
          "approx_seconds": 4.5,
          "audio_duration_seconds": 0.0
        }
      ],
      "full_narration": "Sunlight hits the building first as raw solar power. Only the glass opening can turn that power into indoor heat.",
      "audio_duration_seconds": 0.0
    }
  ]
}
```

Paths in `video_path` may be relative to `job_dir`.

After `synthesize_audio.py`, `audio_duration_seconds` fields are filled from measured WAV lengths.

## Narration style

**Do**

- Speak as a calm teacher: continuous, unhurried, concrete.
- Name what moves on screen in the same beat.
- Carry one metaphor across the whole video.
- End each scene so the next can open without a hard reset.

**Don't**

- “Welcome back”, “in this animation”, “as shown here”.
- Stack three clauses in one breath.
- Race to match a too-short clip — freeze-frame is OK; rushed speech is not.
- Invent formulas/labels that never appear in the scene.

## Pace guide

| Goal | Guideline |
|------|-----------|
| Words per beat | Usually 8–18 English words |
| Rate flag | `-15%` to `-25%` for edge-tts |
| Gap between beats | Script inserts ~0.18s silence when concatenating beat WAVs |
| Sync tolerance | Prefer rewording if spoken length drifts > ~1.2s from visual budget |

## inspect.json (script output)

Produced by `inspect_job.py`. Fields used when drafting:

- `scenes[].id`, `title`, `visual_description`, `camera_notes`, `visual_device`
- `scenes[].beats` from section/plan (may be empty — then invent beats from code/visuals)
- `scenes[].video_duration_seconds` from ffprobe
- `scenes[].code_excerpt` short text cues from `code_final.py` (labels)

## Portable job layouts

1. **NowIGetIt**: `artifacts/<job_id>/scene_plan.json` + `scenes/<id>/`
2. **Generic**: a folder of `*/scene.mp4` plus optional `section.json`; pass `--job-dir` and ensure transcript lists each `video_path`.

## Engines

| Engine | Flag | Needs |
|--------|------|-------|
| edge-tts (default) | `--engine edge` | network + `edge-tts` package |
| OpenRouter Gemini TTS | `--engine openrouter` | `TTS_API_KEY` or `OPENROUTER_API_KEY` |
| macOS say | `--engine macos` | macOS only; lower quality |

Prefer **edge** for portability across repos without this project's OpenRouter setup.
