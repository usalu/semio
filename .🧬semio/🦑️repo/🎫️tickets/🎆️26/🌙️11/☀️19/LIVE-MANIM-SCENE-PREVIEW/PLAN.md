# Live Manim Scene Preview

## Goal

`R26-02/UPDATED-DOCS/UPDATED-USER-DOCS/UPDATED-TUTORIALS` — make authored teaching clip scenes previewable live while iterating.

## Intent

Authoring `🎓️teaching/.../🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` needs a shared, profile-aware Manim path that opens a realtime preview window (OpenGL `-p`) instead of only the offline curriculum compose pipeline.

## Approach

1. Add `live_scene(script, scene, …)` on the shared clip runtime.
2. Extend `🖌️render` CLI with `--live --script --scene` (preview profile, OpenGL live window by default, Cairo `-p` as fallback).
3. Keep offline `compose_videos` unchanged; live mode never writes curriculum provenance.
4. Extend existing clip tests for the live command contract.
5. Confirm runtime by launching fundamentals `Beat1_Leistung` with a live window.

## Constraints

- Scenes require `SEMIO_TEACHING_PROFILE` (and language/audience) — live path must set them.
- OpenGL is the realtime popup; Cairo only opens the player after the scene finishes.
- Repo MCP unavailable this session — ticket created manually.
