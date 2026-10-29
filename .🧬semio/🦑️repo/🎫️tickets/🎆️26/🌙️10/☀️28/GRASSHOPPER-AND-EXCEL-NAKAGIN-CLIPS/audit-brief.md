# Audit Brief (Haiku, read-only)

You audit one video. Edit nothing except your report `T/audit-<series>-<topic>.md`. No git modifying commands.

Inputs: `T/plan.md` (video row), `T/agent-rules.md` (quality bar), `T/storyboard-<series>-<topic>.md`, `T/scene-<series>-<topic>.md`, the scene file, the research files (`T/research-nakagin.md`, `T/research-grasshopper.md`, `T/research-casting.md`, `T/research-excel.md`), and the keyframes `T/🗑️generated/frames/<series>/<SceneClass>/{en,de}/*.png` with `layout.md`.

1. Look at EVERY keyframe PNG in both languages with the Read tool. Per frame, list concrete defects: overlapping or clipped text, content touching the frame edge, text smaller than ~16 px at 1080p, empty or cluttered stage, a value without object or unit, a value not drawn by its meaning on the Nakagin or canvas/workbook, a drawing that contradicts the caption, inconsistent counts between panel/tree/drawing, wrong colour linkage, caption longer than two lines, missing "teaching model" statement.
2. Fact-check every narration clause (en and de): Grasshopper component names and behaviour, casting rules (verified vs teaching assumption marked), Excel names and German function names/separators/error codes, Nakagin history (architect, years, counts), physics (U·A, H_V, degree days, units). Check German grammar and natural wording.
3. Didactics: is each concept introduced on the smallest example, is the typical mistake shown and fixed, does the arc hold, is anything from the plan row missing?

Report format: a summary verdict, then a table `frame/key | lang | severity (blocker/major/minor) | defect | fix suggestion`, then fact findings, then didactic findings. Be specific (coordinates, words). Return a 5-line summary.
