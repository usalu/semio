# Agent Rules For This Ticket

T = `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS`.
A = `C:/git/semio/🎓️teaching/🏛️architecture`. N = `A/🏢️nakagin`. G = `A/🦗️grasshopper/🎬️clip`. X = `A/📊️excel/🎬️clip`.
Read `T/plan.md` and `T/api.md` first; they are the contract. Facts: `T/research-nakagin.md`, `T/research-grasshopper.md`, `T/research-excel.md`. Toolkit and scene pattern: `T/audit-toolkit.md`. Reference implementation to imitate: the BIM series `A/🏗️bim/🎬️clip` (scene structure, NARRATION, ClipScene, tests, README) and its ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES` (capture.py, layout_strict.py).

## Repository Rules (from AGENTS.md)

- Never run modifying git commands (commit, stash, checkout, reset, restore, clean …), no worktrees. Never edit `AGENTS.md`. Other agents edit nearby files at the same time: touch only the files you own, keep foreign changes, never revert someone else's edit.
- Never edit BIM, optimization or energy files (`A/🏗️bim`, `A/🎯️optimization`, `A/⚡️energy`) unless your brief names the file; import and reuse them instead.
- Temporary scripts go in `T/<area>/` (keep them); generated outputs (PNGs, logs, renders, .xlsx exports) go in `T/🗑️generated/<area>/`. Write the Markdown report named in your brief in `T`.
- Concise code matching the BIM style. No comments inside function/class bodies. Every docstring starts with a unique fitting emoji. Temporary prints start with `[DEBUG] `. Links to sources in docstrings where useful.
- Runtime code is stdlib + Manim (+ NumPy where BIM scenes already use it). `rhino3dm`, `openpyxl`, `formulas`, SciPy are test-only oracles.
- Schema first: data in `🔣️.json` validated by `🧬️schema/🔣️.schema.json`; code reads data, never hard-codes building values.
- English and German are both first-class: every visible string and every narration clause exists in both; numbers follow language punctuation (en `1,250.5`, de `1.250,5`); no default language. German Excel shows German function names and `;` separators; German Grasshopper keeps the English component names (the UI is English) but German narration and captions.
- Paths stay below 256 characters. Folder names are emoji + short English word.
- Do not ask questions; take the most appropriate choice and finish completely. Do not stop halfway.

## Environment

- Python with Manim 0.22, NumPy, SciPy, rhino3dm, openpyxl, formulas: `C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe`. Run Python files, never `python -` on stdin. Bash heredocs and `node -e` mangle backslashes on this Windows host: write files with the Write/Edit tools.
- Always `cd /c/git/semio` before shell commands; quote emoji paths. Commands longer than 2 minutes run in the background.
- Scenes need `SEMIO_TEACHING_LANGUAGE=en|de`.
- Keyframes: `python T/capture.py <clip-folder> <topic-folder> <SceneClass> <en|de> [key ...]` writes one PNG per narration key to `T/🗑️generated/frames/<series>/<SceneClass>/<lang>/` plus a layout report. Look at the PNGs with the Read tool. Fix every overlap, clipped label, unreadable text, empty or cluttered screen.

## Quality Bar

- Nothing abstract: every concept appears on the Nakagin (a capsule, a type, a core, a storey, a facade, the capital, the podium) or on the canvas/workbook that computes it — and both are on screen together whenever a value is discussed.
- Every value appears with object and unit and is drawn by its meaning at the same time (dimension line, hatched surface, lit capsules, plane axes, wire colour, reference frame). Changing a value changes the drawing (ValueTracker + `always_redraw`).
- Every number comes from the model/engine, never typed in the scene.
- Grasshopper names are real Grasshopper 1 / Rhino 8 names (Number Slider, Panel, Center Box, Orient, Series, Graft Tree, Flatten Tree, Simplify Tree, Path Mapper, Param Viewer, List Item …); Excel names are real Microsoft 365 names in both languages.
- Historical claims match `T/research-nakagin.md`; simplified values say "teaching model" once per clip.
- Captions at most two lines. The stage between title and caption is filled; nothing overlaps the caption band. Text ≥ 16 px at 1080p.
- Each video is 3–6 minutes, 14–24 narration clauses, each clause one visible change.
