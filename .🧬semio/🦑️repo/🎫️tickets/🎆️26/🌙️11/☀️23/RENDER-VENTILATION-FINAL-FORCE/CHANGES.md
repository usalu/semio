# Force Re-Render Ventilation Final

Deleted prior final Lüftung outputs (`rendered/Full_Ventilation_*`, `2/`, `s/10`–`s/31`, traces).

Re-render:

```sh
bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render \
  --language de --video ventilation --profile final \
  --audience speaker --force --jobs 1 --output-root ./videos --no-play
```

Deliverable: `videos/final/de/speaker/rendered/Full_Ventilation_speaker_de_1080p30.mp4`
