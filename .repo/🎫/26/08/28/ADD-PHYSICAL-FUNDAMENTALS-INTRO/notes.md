# Add Physical Fundamentals Intro

Repo MCP was unavailable (`repo://goals` / `ticket_open` not registered); ticket folder created manually.
Goal: `🎯r2602🎯updateddocs🎯updateduserdocs🎯updatedtutorials`

## Scene

`Demo_Intro_PhysikalischeGrundlagen` subclasses `NGSIntro`:

- series: Gebäudeenergie · Grundlagen
- topic: Physikalische Grundlagen
- explain: Kraft, Leistung und Energie — die Sprache der Gebäudeenergie.

## Pipeline

- `full_physical_fundamentals_video.py` renders the intro first, then the body, ffmpeg-concat.
- `build_full_video.py` prepends the same intro (optional `intro_physical_fundamentals.mp3` mux).
- Launch: `🛠️dev🎬manim intro physical fundamentals`

## Smoke

Low-quality render (~15 s):

- `tutorial/intro/media/videos/intro_scene/480p15/Demo_Intro_PhysikalischeGrundlagen.mp4`
- ticket copies: `Demo_Intro_PhysikalischeGrundlagen.mp4`, `preview_final.png`, `smoke_render.log`
