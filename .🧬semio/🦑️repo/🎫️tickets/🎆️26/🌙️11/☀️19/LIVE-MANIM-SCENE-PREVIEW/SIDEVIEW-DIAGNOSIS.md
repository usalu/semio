# Manim Sideview Exit Code 1

## Cause

Manim Sideview spawns bare `manim` with no teaching env. Scene construct calls `tr()` → `language()`, which raises:

```text
ValueError: Set SEMIO_TEACHING_LANGUAGE to en or de
```

Logged scene at failure: `Beat5_Strahlung`. Sideview exposes no environment-variable setting (`defaultManimPath`, `commandLineArgs`, `terminalCommand` only).

## Fix

1. Launcher sources under `.vscode/manim-sideview/` (`run.py`, Unix `run`, Windows `run.cmd`).
2. `python3 .vscode/manim-sideview/install.py` installs `.venv/bin/manim-teaching` (or `Scripts/manim-teaching.cmd`).
3. `.vscode/settings.json` sets `manim-sideview.defaultManimPath` to `manim-teaching` and `commandLineArgs` to `-ql --disable_caching`.

Defaults (override by exporting before run):

| Variable | Value |
|---|---|
| `SEMIO_TEACHING_LANGUAGE` | `de` |
| `SEMIO_TEACHING_AUDIENCE` | `speaker` |
| `SEMIO_TEACHING_PROFILE` | `preview` |

## How to preview

1. Run `python3 .vscode/manim-sideview/install.py` once per venv.
2. Reload the Cursor window so workspace settings apply.
3. Open the fundamentals scene file.
4. Manim Sideview → select a beat → render.

## Preferred repo CLI (unchanged)

```bash
.venv/bin/python "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🖌️render/🐍️.py" \
  --live --language de --profile preview --audience speaker \
  --script "🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py" \
  --scene Beat2_EnergieImAlltag
```

## Verification

- `manim-teaching --version` → Manim Community v0.21.0 without a pre-set language.
- Wrapper injects `de` / `speaker` / `preview` via `setdefault` before `execv` of `.venv/bin/manim`.
