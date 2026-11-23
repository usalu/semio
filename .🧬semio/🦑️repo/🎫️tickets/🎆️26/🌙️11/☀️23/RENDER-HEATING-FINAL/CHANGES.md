# Render Heating Final

## Cleanup
Deleted ticket correction videos (ENERGY-CLIP-STYLE inspection mp4s) and `videos/final/de/speaker/verify/*.mp4`.
Kept SEMIO-ARTIFACT lossless mp4 fixtures.

## Render
```sh
bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render \
  --language de --video heating --profile final \
  --audience speaker --force --jobs 1 --output-root ./videos --no-play
```

Deliverable: `videos/final/de/speaker/rendered/Full_Heating_Demand_speaker_de_1080p30.mp4`
