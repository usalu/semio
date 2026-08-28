# Render Heating Intro And Concat

1. Rendered `Demo_Intro_Heizlast` at 1080p60 (matches existing heating mp4s; 4K would force re-encode of the body).
2. ffmpeg-concat intro + existing `FullHeatingDemandVideo.mp4` → `FullHeatingDemandVideo_with_intro.mp4` (no re-render of modules).

Outputs under `tutorial/energy/demand/Heating/media/videos/full_heating_video/1080p60/`:
- `Demo_Intro_Heizlast.mp4`
- `FullHeatingDemandVideo_with_intro.mp4` (~16.5 min)
