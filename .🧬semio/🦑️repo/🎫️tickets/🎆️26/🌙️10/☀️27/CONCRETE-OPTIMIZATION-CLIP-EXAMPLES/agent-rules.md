# Agent Rules For This Ticket

Ticket folder `T` = `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES`. Clip folder `C` = `C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip`. Read `T/plan.md` and `T/api.md` first; they are the contract. Model facts: `T/model.md`, `T/model-refinement.md`. Shared toolkit: `T/toolkit.md`, `C/🎨️visuals/🐍️.py`, `C/🎨️visuals/🏠️room/🐍️.py`. Per-scene notes: `T/scene-<topic>.md`.

## Repository Rules (from AGENTS.md)

- Never run modifying git commands (commit, stash, checkout, reset, restore, clean …) and no worktrees. Never edit `AGENTS.md`. Other agents edit nearby files at the same time: only touch the files you own, keep foreign changes.
- Temporary files, scripts, logs go in `T`; generated outputs (PNGs, logs, renders) go in `T/🗑️generated/<your-area>/`. Write a Markdown report in `T` for your work (name given in your brief).
- Concise code matching the surrounding style. No comments inside function/class bodies. Every docstring starts with a unique fitting emoji. Temporary prints start with `[DEBUG] `.
- No new runtime dependencies: the model is stdlib only; Manim is reached only through the existing `ClipScene` and energy toolkit imports. NumPy/SciPy are allowed in tests only.
- English and German are both first-class: every visible string and every narration clause exists in both, numbers follow language punctuation (en `1,250.5`, de `1.250,5`). No default language.
- Do not ask questions; take the most appropriate choice and finish completely.

## Environment

- Python with Manim 0.22, NumPy, SciPy: `C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe`. Run Python files, never `python -` on stdin (hangs). Bash heredocs and `node -e` mangle backslashes on this Windows host: write files with the Write/Edit tools.
- Always `cd /c/git/semio` before shell commands; paths contain emoji, quote them. Long commands (> 2 min) run in the background.
- Keyframes: `python T/capture.py <topic-folder> <SceneClass> <en|de> [key ...]` writes one PNG per narration key to `T/🗑️generated/frames/<SceneClass>/` plus a `…-layout.md` overlap report. Look at the PNGs with the Read tool. Fix every overlap, clipped label, unreadable text and empty or cluttered screen.
- Tests: `SEMIO_TEACHING_OUTPUT_ROOT=<T>/🗑️generated/output python C/🧪️tests/🐍️.py` (unittest; the variable is normally set by `repo …/📜️script.ts test`).

## Quality Bar (the owner's request)

"Every single clip has a proper example and nothing is abstract but the values in the plot always are at the same time visualized by their meaning with a specific example. The current example is way too simple both rendered and conceptually. See energy videos."

- Every plotted value has its physical meaning on screen at the same moment: the room drawing shows that exact design (orientation, window size, overhang, insulation) and its consequence (sun rays passing or blocked, lamps dimmed, heat-loss arrows, stacked electricity bar with live kWh/a, price chip). Moving a marker moves the room (ValueTracker + `always_redraw`).
- No bare number without its object and unit; no normalized 0–1 axis without its physical range; no invented letters standing for designs without a façade thumbnail.
- Every number on screen comes from the model functions, never typed by hand. Signs and arithmetic shown on screen must be correct and self-explaining (no "÷ EER = −279").
- Captions stay short (two lines at most). The drawing fills the stage between title and caption; nothing overlaps another label, a line or the caption band.
