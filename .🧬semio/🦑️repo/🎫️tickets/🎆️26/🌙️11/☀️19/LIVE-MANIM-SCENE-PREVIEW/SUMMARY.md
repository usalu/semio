# Summary

## Done (reopen)

- Diagnosed Manim Sideview exit code 1: missing `SEMIO_TEACHING_LANGUAGE`.
- Added `.vscode/manim-sideview` launcher + `install.py` shim into `.venv/bin/manim-teaching`.
- Wired workspace Sideview settings (`manim-teaching`, `-ql --disable_caching`) and terminal env defaults.
- Documented in `SIDEVIEW-DIAGNOSIS.md` / `USAGE.md`.

## Files

- created: `.vscode/settings.json`
- created: `.vscode/manim-sideview/run.py`, `run`, `run.cmd`, `install.py`
- updated: ticket `USAGE.md`, `SIDEVIEW-DIAGNOSIS.md`

## Done (empty player)

- Beat6 render succeeded; Sideview webview could not load emoji-cache mp4 (`localResourceRoots`).
- Shim writes `SEMIO_TEACHING_OUTPUT_ROOT` to `<scene>/.sideview/`; gitignore updated.
- Documented in `SIDEVIEW-EMPTY-PLAYER.md`.
