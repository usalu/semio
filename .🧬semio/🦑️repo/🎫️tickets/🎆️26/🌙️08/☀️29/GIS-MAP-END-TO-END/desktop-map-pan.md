# Desktop GIS2D Middle-Button Pan

Each desktop middle-button move previously dispatched a durable `setCamera` command. Chrome received the trusted input on the canvas and generated 13 additional command pages during one drag. A dense before-and-after comparison also found 5,246 changed pixels above a 16-level intensity threshold. The old sparse comparison missed that repaint, so the camera command load and the screenshot criterion were separate findings.

The React map now uses the same local pan update for mouse and touch. Each changed sample advances the renderer camera and schedules tile refresh. Mouse release applies its final coordinates and publishes one camera command. Cancellation and lost capture retain the last move and release the continuous frame request. The pan accepts events from its owning pointer. A canceled primary-button marquee clears without updating the camera or selecting features.

## Verification

Three mounted regression cases reproduced the extra commands before the change. Each dispatched two camera commands from two move samples, where the fixture expects zero until the gesture ends.

All 56 tests across the three React TiledMapHost suites passed. The language-neutral fixture and Three.js vector calculation agree on the move and terminal camera coordinates. Mounted React tests also exercise a trailing release after cancellation or lost capture, and use a controlled frame clock to verify that cancellation returns rendering to idle. The Git whitespace check passed.
