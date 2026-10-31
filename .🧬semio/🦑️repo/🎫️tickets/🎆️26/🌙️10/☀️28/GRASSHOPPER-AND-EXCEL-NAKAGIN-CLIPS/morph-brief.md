# Morph Agent Brief (Sonnet)

You make one video sophisticated through morphs. Read `C:/git/semio/AGENTS.md`, `T/morph-plan.md` (binding), `T/agent-rules.md`, `T/api.md` (section 7 = morph toolkit, being built now by MORPH-TOOLKIT; sections 1–6 = model, drawings, engines, toolkits), your scene file, `T/scene-<series>-<topic>.md`, `T/quality-<series>-<topic>.md`, `T/sweep-<series>-<topic>.md`.

## Phase A — Morph storyboard (start now)

Write `T/morph-<series>-<topic>.md`: 8–14 morph beats mapped onto the existing clauses (you may merge or rewrite clauses; stay at 14–24 clauses, every clause en+de, captions ≤ 2 lines). For each beat: clause key, concrete object (exact part of the Nakagin drawing or the tool UI), abstract object (token, cell, list item, tree leaf, number, path chip, formula token…), direction, what carries over (colour, count, value, position order), duration, and the sentence the narration says while it happens. At least three concrete → abstract and three abstract → concrete beats; chain them (the same capsule becomes a leaf, then a path chip, then lights up on the tower again). Replace every fade-swap, cut or side-by-side "here is the abstract version" with a morph that shows identity.

## Phase B — Implement (when `T/api.md` section 7 says `Status: ready`)

Use `🎓️teaching.🏛️architecture.🔀️morph.🐍️` (import with literal `importlib.import_module`). Keep every number from the model/engines. A codex session edits the Nakagin model/drawings (and the Grasshopper tower/tree scenes and transcripts) concurrently: re-read your scene file immediately before every edit, use small Edit-tool diffs, never revert foreign changes, never edit `🏢️nakagin/**`, take counts from the model (they changed: tower A 76, tower B 64, branches with one or two items — check on disk). Use `proxy()`/`silhouette()` for heavy tower groups.

## Phase C — Verify in motion

`C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe T/capture_morph.py <series-folder> <topic-folder> <SceneClass> en [keys]` (PYTHONUTF8=1; run in the background, it plays animations at 4 fps) saves `<key>-<n>-mid.png` at every animation's midpoint and `<key>-end.png`. Read the mid frames of every morph beat: the transforming object must be readable mid-way (not a scribble), colours must carry, the ghost/tether must show where it came from. Then the German run. Then `T/capture.py` keyframes for both languages (layout). Then the quick presentation contract tests of your series. Write `T/morph-<series>-<topic>-done.md` (beats implemented, frames checked, open items) and return a short summary.
