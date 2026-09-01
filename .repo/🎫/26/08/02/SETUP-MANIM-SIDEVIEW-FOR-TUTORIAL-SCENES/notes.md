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

## Troubleshooting “icon does nothing”

1. **Reload Cursor** after settings changes (`Developer: Reload Window`).
2. Status bar → Python interpreter must be **`semio/.venv/bin/python`**.
3. After “Loaded a configuration file…”, a **scene picker** appears at the top — choose `Beat1_WinterGains` (easy to miss).
4. Or Command Palette → **`Manim: Runs a Sideview`** (same as the icon).
5. **`outputToTerminal` must stay `true`** — setting it to `false` breaks rendering silently.
6. **`defaultManimPath` must be `"manim"`** — Sideview does not expand `${workspaceFolder}`; using it creates a broken doubled path like `.venv/bin/${workspaceFolder}/.venv/bin/manim`. Manim is resolved from the selected `.venv` Python interpreter instead.
7. If stuck: Command Palette → **`Manim: Clear All Active Jobs`**, then click the icon again.

- `.vscode/settings.json` — correct `manim-sideview.*` keys + Mac venv Python
- `.vscode/extensions.json` — recommends `rickaym.manim-sideview`
- `manim.cfg` next to `scene_3.py` — low-quality fast Sideview renders
- `.vscode/launch.json` — `🛠️dev🎬manim scene_3` fallback

Render verified: `Beat1_TransmissionOpaque` → `media/videos/scene_3/480p15/Beat1_TransmissionOpaque.mp4`
