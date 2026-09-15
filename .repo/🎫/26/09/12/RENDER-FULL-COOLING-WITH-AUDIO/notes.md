# Render Full Cooling With Audio

Rebuilt missing full Cooling deliverables after files were deleted from disk.

## Deliverables
- `tutorial/energy/demand/Cooling/rendered/Full_Cooling_Demand_WithAudio_1080p60.mp4` — intro + Teile 1–6, 1080p60, AAC, ~27:29
- `tutorial/energy/demand/Cooling/rendered/Full_Cooling_Demand_1080p60.mp4` — same as WithAudio (alias)
- `tutorial/energy/demand/Cooling/rendered/Full_Cooling_Demand_NoAudio_1080p60.mp4` — silent HQ concat

## Notes
- Repo MCP unavailable; ticket folder created manually.
- Reused existing clause MP3s; HQ re-render with `VO_TRACE=1`; mux `libx264 -preset slow -crf 15`, AAC 320k/48k.
- Teil 5 window in this build: ~11.8–18.5 min (section ~397.8s).
