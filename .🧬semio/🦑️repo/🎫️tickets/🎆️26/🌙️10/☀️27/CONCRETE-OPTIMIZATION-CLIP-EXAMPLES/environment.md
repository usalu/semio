# Environment Notes

- Repo MCP servers (`repo`, `semio`) failed to connect this session. `native-repo-path` points at a missing `repo.exe`; the cached `.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo.exe` CLI works for `goal list` and `ticket open/close` but does not run scripts.
- The teaching script is a Bun script, so its commands run as `bun 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render|test …` (what `repo <script.ts>` would dispatch to).
- Teaching Python: `.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe` (Manim 0.22.0, NumPy 2.5.4, SciPy 1.18.1).
- `capture.py` renders one still per narration key in ≈ 50 s per scene (skip-animation mode) and writes a layout-conflict report from the energy toolkit's `layout_conflicts`.
