# Live Manim Scene Preview

Requires the `repo` launcher on `PATH` (staffage renders through it):

```bash
bash "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🐚️.sh"
export PATH="$HOME/.local/share/semio/launcher/bin:$PATH"
```

Preview one authored beat under the preview profile (480p15). Cairo opens the movie when the scene finishes:

```bash
.venv/bin/python "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🖌️render/🐍️.py" \
  --live \
  --language de \
  --profile preview \
  --audience speaker \
  --script "🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py" \
  --scene Beat1_Leistung
```

- Default `--renderer cairo` matches teaching morphs and opens the system player after the scene.
- `--renderer opengl` streams a live window when the scene graph stays within the OpenGL Cairo-surface shims.
- `--write-movie` also saves an mp4 during an OpenGL preview.
- Other beats: `Beat2_EnergieImAlltag` … `Beat8_Zusammenfassung`.

Confirmed: `Beat1_Leistung` rendered end-to-end and wrote

`/.🧬semio/🎓️teaching/⚡️cache/videos/live/preview/de/speaker/Beat1_Leistung/v/Beat1_Leistung.mp4`


# Manim Sideview (Cursor / VS Code)

Teaching scenes require an explicit language. Sideview has no env setting, so this workspace uses a `manim-teaching` shim:

```bash
python3 .vscode/manim-sideview/install.py
```

Then reload the window and run Manim Sideview on a beat. Workspace settings point `manim-sideview.defaultManimPath` at `manim-teaching` and pass `-ql --disable_caching`.

Defaults injected by the shim: `SEMIO_TEACHING_LANGUAGE=de`, `SEMIO_TEACHING_AUDIENCE=speaker`, `SEMIO_TEACHING_PROFILE=preview`.

Details: `SIDEVIEW-DIAGNOSIS.md` in this ticket.

## Empty Sideview player (0:00)

If the checkerboard shows and time stays 0:00 while the scene name appears, the render likely wrote outside Sideview’s webview root. The `manim-teaching` shim writes under `<scene>/.sideview/` so the player can load the mp4. See `SIDEVIEW-EMPTY-PLAYER.md`.
