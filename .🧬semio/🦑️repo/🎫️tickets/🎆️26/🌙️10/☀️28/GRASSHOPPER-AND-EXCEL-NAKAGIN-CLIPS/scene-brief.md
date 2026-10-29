# Scene Agent Brief (shared)

You are a scene agent for one video. Your video row is in `T/plan.md` (Videos table). Read `C:/git/semio/AGENTS.md`, `T/agent-rules.md`, `T/plan.md`, `T/api.md`, `T/audit-toolkit.md`, the research files, and the reference BIM scene `C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py` (structure, NARRATION, `step`, live values, helpers) plus one optimization scene in `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/`.

## Phase 1 — Storyboard (start immediately)

Write `T/storyboard-<series>-<topic>.md`: learning goals (3–5), the misconceptions from the research files you address, then a table of 16–24 narration clauses: key, English, German, what is on screen (tool side: canvas/workbook; building side: Nakagin drawing), which value is shown and how it is drawn by its meaning, the animation. Didactic arc: question/hook on the Nakagin → concept introduced on the smallest example → generalised → typical mistake shown and fixed → recap legend of terms (en/de). Every clause is one visible change. Say "teaching model" once. Both languages are authored natively, not translated word by word.

## Phase 2 — Scene (when the API sections you need are `Status: ready` in `T/api.md`)

Check `T/api.md` for readiness; while waiting, refine the storyboard against the actual model/engine code as it appears on disk. Then implement your scene class in your topic file (class name from the plan, `TITLE {en,de}`, `NARRATION [[key, en, de], …]`, `construct` → `self.present("<topic>")` → `present_<topic>`), using only the ClipScene, toolkit widgets, Nakagin drawings and engine/model results. Every number from the engine/model. Values that move use ValueTracker + always_redraw.

You may add a canvas (Grasshopper) or cells/blocks (Excel) that your video needs to the shared model JSON, only inside the entry for your topic, keeping the schema valid and the series tests passing; never change other topics' entries. If you need a toolkit/engine feature, implement it as a small private helper in your scene file or ask the owner via SendMessage only if it is a bug.

## Phase 3 — Verify

Capture keyframes in both languages: `C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe T/capture.py <series> <topic> <SceneClass> en` (and `de`). Look at EVERY PNG with the Read tool. Fix overlaps, clipped or tiny text, empty or cluttered screens, wrong values, wrong German. Repeat until the layout report has 0 strict findings or each remaining finding is justified. Then run the series presentation tests (`G|X/🧪️tests/🎭️presentation/🐍️.py`).

Write `T/scene-<series>-<topic>.md`: what was built, clause list, keyframe review notes, test result. Return a short summary.
