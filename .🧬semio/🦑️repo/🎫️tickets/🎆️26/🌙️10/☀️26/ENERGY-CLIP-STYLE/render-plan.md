# Physical Video Draft Render Plan

Current manifest selects physical_fundamentals: one institution intro plus eight complete beats (energy in daily life, power, conservation, heat pump, radiation, thermal mass, sensible/latent heat, recap). No chapter or clause filter is needed for the full video.

Fast complete draft invocation through the registered Python project:

```powershell
repo tools exec uv -- run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --python 3.12 python -B -m 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🖌️render.🐍️ --video physical_fundamentals -q l --resolution 640,360 --fps 10 --jobs 2 --log-output --no-play --output-root .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ENERGY-CLIP-STYLE/🗑️generated/draft
```

Use an existing provisioned environment when the environment scout provides it; avoid redundant dependency setup. 854,480 at10fps is the480p alternative;640x360 is faster for the requested immediate visual draft. Start with2 workers to avoid native text/memory contention, increase only if measured capacity permits. Rendering reports segment/chapter progress, supports keyboard cancellation, and reuses provenance-matching completed segments. Do not force rerender or skip caching outside runtime policy.

Expected final output is output-root/rendered/Full_Physical_Fundamentals_640x360p10.mp4. Copy the verified complete deliverable to ticket/physical.mp4 before generated cleanup. Runtime --play invokes Windows os.startfile (macOS open/Linux xdg-open); the retained file can also be displayed with app media playback.

Current physics speech folder contains only its authoring Python script, no canonical beat_N_clauses recordings. source_narration therefore returns an empty list and renders a silent draft; it does not synthesize speech or block rendering. If partial recordings appear concurrently, runtime rejects incomplete canonical sets. Current timing contract has beats:{} and retains source-authored timing defaults. Final concatenation now preserves audio when present.

FFmpeg resolution: SEMIO_FFMPEG_BINARY, then PATH, then existing imageio_ffmpeg package. Python project pins Manim0.22.0/numpy/imageio-ffmpeg and requires Python3.12. Current CLI validates even positive width,height and positive fps. No known source-level blocker to this complete draft; actual render failures must be diagnosed from retained attempt logs.
