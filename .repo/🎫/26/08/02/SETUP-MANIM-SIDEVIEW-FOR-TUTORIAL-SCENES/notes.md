# Manim Sideview — How to Open scene_3.py

## One-time

1. Install recommended extension **Manim Sideview** (`rickaym.manim-sideview`) if prompted.
2. Select Python interpreter: `.venv/bin/python` (status bar → Python version).

## Open & preview

1. Open `tutorial/energy/demand/Cooling/2_transmission_humidity/scene_3.py`.
2. Click the **rotation icon** in the editor title bar (or Command Palette → `Manim: Runs a Sideview`, or `Ctrl+'` then `r`).
3. Pick a scene when prompted:
   - `Beat1_TransmissionOpaque`
   - `Beat2_TimeLag`
   - `Beat3_VentilationHeat`
   - `Beat4_SensibleVsLatent`
4. Sideview panel shows the live preview. Switch scenes with `Manim: Render a New Scene` (`Ctrl+'` then `c`).

## Config touched

- `.vscode/settings.json` — correct `manim-sideview.*` keys + Mac venv Python
- `.vscode/extensions.json` — recommends `rickaym.manim-sideview`
- `manim.cfg` next to `scene_3.py` — low-quality fast Sideview renders
- `.vscode/launch.json` — `🛠️dev🎬manim scene_3` fallback

Render verified: `Beat1_TransmissionOpaque` → `media/videos/scene_3/480p15/Beat1_TransmissionOpaque.mp4`
