# Why Manim Sideview Shows an Empty Checkerboard

## What you saw

`Beat6_ThermischeMasse` player at **0:00 / 0:00** with a checkered background. Scene name is present; no frames play.

## Not a render failure

Sideview log (20:15) shows exit code **0**, 79 animations, and:

```text
File ready at
.../.🧬semio/🎓️teaching/⚡️cache/videos/m/v/Beat6_ThermischeMasse.mp4
```

That file exists (~1.7 MB, ~91 s). The render worked.

## Root cause

Manim Sideview’s webview sets:

```text
localResourceRoots = <folder of the .py file>
```

Teaching `configure_manim()` writes media into the emoji cache under `.🧬semio/🎓️teaching/⚡️cache/...`, **outside** that folder. The webview URI is posted, but the browser blocks the file → empty player.

## Fix

`.vscode/manim-sideview/run.py` now forces:

```text
SEMIO_TEACHING_OUTPUT_ROOT = <scene-dir>/.sideview
```

so the mp4 lands at e.g.

```text
.../🧲️fundamentals/.sideview/m/v/Beat6_ThermischeMasse.mp4
```

which Sideview is allowed to load. `.sideview/` is gitignored.

## What to do

Re-run Manim Sideview on `Beat6_ThermischeMasse` (no Cursor reload required for the `run.py` change; the shim already calls it).
