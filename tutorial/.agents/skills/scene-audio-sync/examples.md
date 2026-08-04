# Example: run scene-audio-sync on a NowIGetIt job

User: `Use scene-audio-sync on artifacts/6115aa2db012`

Agent:

```bash
SKILL=tutorial/.agents/skills/scene-audio-sync
JOB=artifacts/6115aa2db012
bash $SKILL/scripts/install_check.sh
python3 $SKILL/scripts/inspect_job.py $JOB --out $JOB/audio_work/inspect.json
```

Then the agent reads `inspect.json`, writes `$JOB/audio_work/transcript.json` + `transcript.md` (no external LLM), then:

```bash
python3 $SKILL/scripts/synthesize_audio.py $JOB/audio_work/transcript.json \
  --out-dir $JOB/audio_work --engine edge --voice en-US-JennyNeural --rate=-20%
python3 $SKILL/scripts/mux_sync.py $JOB/audio_work/transcript.json \
  --job-dir $JOB --publish --final
python3 $SKILL/scripts/verify_sync.py $JOB/audio_work/transcript.json --job-dir $JOB
```

## Copying to another repo

```bash
cp -R tutorial/.agents/skills/scene-audio-sync /path/to/other-repo/tutorial/.agents/skills/
python3 -m pip install 'edge-tts>=6.1.0' 'httpx>=0.27.0'
# ensure ffmpeg is installed (brew/apt/winget)
bash /path/to/other-repo/tutorial/.agents/skills/scene-audio-sync/scripts/install_check.sh
```

Point `--job-dir` at any folder that has `scenes/<id>/scene.mp4` (or list `video_path` in the transcript).
