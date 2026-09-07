# Clean Tutorial Media — deletion log

Scope: `tutorial/` only. Nothing outside that tree was touched.

## Round 1 (mp3 + mp4) — earlier this session

Totals deleted: **~12,006 files**
- **81** `.mp3`
- **11,925** `.mp4`

A full per-path list was not saved before deletion (mostly Manim `partial_movie_files` caches). Categories removed:

### Audio (`.mp3`) — examples / locations
- `tutorial/intro/intro_audio.mp3`
- `tutorial/intro/intro_physical_fundamentals.mp3`
- Beat / scene TTS under `tutorial/energy/demand/{Cooling,Heating,1_physical_fundamentals}/**` (e.g. `beat_N_audio.mp3`, `scene_N_audio.mp3`, intro mux mp3s)
- Any other `.mp3` under `tutorial/`

### Video (`.mp4`) — locations
- Final / HQ renders under `**/rendered/*.mp4` and topic folders (`Full_*.mp4`, `*_1080p60.mp4`, `*_vo_de.mp4`, …)
- Manim outputs under `**/media/videos/**/*.mp4`
- Manim partial caches under `**/media/videos/**/partial_movie_files/**/*.mp4` (vast majority of the ~11,925)

## Round 2 (wav) — this turn

Totals deleted: **24** `.wav` files. Full list:

### Cooling/3_transmission_humidity
- beat_1_audio.wav
- beat_2_audio.wav
- beat_3_audio.wav
- beat_4_audio.wav

### Cooling/4_solar_radiation
- beat_1_audio.wav
- beat_2_audio.wav
- beat_3_audio.wav
- beat_4_audio.wav
- beat_5_audio.wav

### Cooling/5_systemauslegung
- beat_1_audio.wav
- beat_2_audio.wav
- beat_3_audio.wav
- beat_4_audio.wav
- beat_5_audio.wav

### Cooling/6_lueftungssysteme
- beat_1_audio.wav
- beat_2_audio.wav
- beat_3_audio.wav
- beat_4_audio.wav
- beat_5_audio.wav
- beat_6_audio.wav

### Heating (legacy full-video mux wavs under media/)
- Heating/2_conduction/media/videos/merged_scenes_german/1080p60/FullConductionVideo.wav
- Heating/3_convection/media/videos/merged_scenes_german/1080p60/FullConvectionVideo.wav
- Heating/4_internal_heat_gain/media/videos/merged_scenes/1080p60/FullInternalHeatGainVideo.wav
- Heating/5_solar_heat_gain/media/videos/merged_scenes/1080p60/FullSolarHeatGainVideo.wav

## Verification

After round 2: **0** `.mp4` / `.mp3` / `.wav` remaining under `tutorial/`.
