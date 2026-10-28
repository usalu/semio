# Agent Rules For This Ticket

Ticket folder `T` = `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES`. Clip folder `C` = `C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip`. Read `T/plan.md` and `T/api.md` first; they are the contract. Facts: `T/research-villa-savoye.md`, `T/research-ifc.md`. Existing code inventory: `T/audit-current.md`, `T/audit-toolkit.md`.

## Repository Rules (from AGENTS.md)

- Never run modifying git commands (commit, stash, checkout, reset, restore, clean …) and no worktrees. Never edit `AGENTS.md`. Other agents edit nearby files at the same time: only touch the files you own, keep foreign changes.
- Temporary files, scripts, logs go in `T`; generated outputs (PNGs, logs, renders) go in `T/🗑️generated/<your-area>/`. Write a Markdown report in `T` for your work (name given in your brief).
- Concise code matching the surrounding style. No comments inside function/class bodies. Every docstring starts with a unique fitting emoji. Temporary prints start with `[DEBUG] `.
- No new runtime dependencies: the model is stdlib only; Manim is reached only through the existing BIM `ClipScene` and energy toolkit imports. NumPy/SciPy are allowed in tests only.
- English and German are both first-class: every visible string and every narration clause exists in both, numbers follow language punctuation (en `1,250.5`, de `1.250,5`). No default language.
- Do not ask questions; take the most appropriate choice and finish completely.

## Environment

- Python with Manim 0.22, NumPy, SciPy: `C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe`. Run Python files, never `python -` on stdin (hangs). Bash heredocs and `node -e` mangle backslashes on this Windows host: write files with the Write/Edit tools.
- Always `cd /c/git/semio` before shell commands; paths contain emoji, quote them. Long commands (> 2 min) run in the background.
- Keyframes: `python T/capture.py <topic-folder> <SceneClass> <en|de> [key ...]` writes one PNG per narration key to `T/🗑️generated/frames/<SceneClass>/` plus a `…-layout.md` overlap report. Look at the PNGs with the Read tool. Fix every overlap, clipped label, unreadable text and empty or cluttered screen.
- Existing tests: `python C/🧪️tests/🐍️.py` (unittest).

## Quality Bar (the owner's request)

"Every single clip has a proper example (not just abstract boxes but meaningful design context for architect use, Villa Savoye etc.) and nothing is abstract, but the values in the plot always are at the same time visualized by their meaning with a specific example. The current example is way too simple both rendered and conceptually."

- Never show a bare box standing for "a wall", a bare number without its object and unit, or an abstract grid standing for "FEM". Every concept is shown on a recognisable part of the Villa Savoye (pilotis on the 4.75 m grid, the first-floor slab, ribbon windows, the ramp, the roof garden, the curved ground-floor glass wall).
- Whenever a value appears (a dimension, an area, a volume, a load, a revision, a GUID), the drawing shows what it means at the same time: the dimension line on the drawing, the hatched area, the filled volume, the load arrows, the highlighted element. Changing a value changes the drawing (ValueTracker + `always_redraw` where it moves).
- Every number on screen comes from the model functions, never typed by hand.
- IFC names are real IFC 4.3 names (`IfcColumn`, `IfcSlab`, `Pset_ColumnCommon.LoadBearing`, `IfcRelVoidsElement` …) and the statements match `T/research-ifc.md`. Historical claims about the villa match `T/research-villa-savoye.md`; simplified values say "teaching model" once per clip.
- Captions stay short (two lines at most). The drawing fills the stage between title and caption; nothing overlaps the caption band.
