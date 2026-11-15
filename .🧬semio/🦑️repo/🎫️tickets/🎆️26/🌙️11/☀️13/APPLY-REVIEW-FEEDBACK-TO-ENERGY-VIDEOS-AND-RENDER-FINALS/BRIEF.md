# Common brief for content agents

You are a senior developer and building-physics author working in repo
`C:\git\semio`. Read `C:\git\semio\AGENTS.md` first and obey it.

## Environment rules
- ONE shared live worktree replicated to 50+ machines. NEVER run a git command
  that modifies the repo or working tree (no commit, stash, checkout, reset,
  restore, clean). Read-only git (log, diff, show) is fine.
- Other agents edit neighbouring files and sometimes the same file right now.
  Re-read a file immediately before each edit, edit with the Edit tool in
  small targeted replacements, never rewrite whole files, never revert
  changes you did not make, never stop because of concurrent changes.
- Windows host. In Bash always `cd /c/git/semio` first. Paths contain emoji:
  quote them. Never write files through Bash heredocs or `python -c`/stdin
  (backslashes get mangled, `python -` hangs): use Write/Edit and run script
  files. Ignore `.venv`, `site-packages`, `__pycache__`, `.ruff_cache`.
- Do not open/close tickets. Ticket folder TK =
  `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️13\APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS`.
  Your scripts go into `TK\<your-name>\`, tool output only into
  `TK\🗑️generated\<your-name>-out\`.
- No comments inside definitions; docstrings start with a unique emoji;
  concise code; greenfield (no compatibility layers); English AND German
  strings both maintained.

## Read before working
1. `TK\FEEDBACK.md` – the reviewer's items F00–F29 (German).
2. `TK\CONVENTIONS.md` – BINDING decisions on units, symbols, references,
   narration `$…$` math, DIN V 4108-6 notation. Follow exactly.
3. `TK\NOTATION-AUDIT.md` – line-level audit of unit brackets, unit-less
   substitutions, spelled-out symbols, references, symbol inventory (line
   numbers may have drifted; re-grep).
4. `TK\MAPPING.md` if it exists (timestamp → scene mapping); otherwise match
   feedback to scenes by content.
5. `TK\RENDER-PIPELINE.md` §4, §5, §9 for commands; the family `README.md`.
6. `TK\SPEECH-LEXICON.md` if it exists (how `$…$` in narration is spoken).

## What "done" means for your files
- Every feedback item assigned to you is resolved in the scene (visual AND
  narration, en + de), in the spirit the reviewer intends – a lecturer in
  building physics must find it correct and clear for architecture students.
- The global rules are applied to EVERY scene in your files, not only the
  flagged ones: units (no `[W]`; units on every number of a substitution;
  fraction bars), symbols as symbols (narration uses `$…$` TeX, no
  "Delta-Theta"), symbol conventions (Φ / Q̇ / Q, index V = ventilation,
  DIN V 4108-6 names), findable references (verify with WebSearch; record
  evidence).
- Physics is right: recompute every number you touch; values come from the
  running example (`🏗️authoring/📏️running-example/🔣️.json` + `🐍️.py`) – do
  not hard-code a number that the running example owns; if a shared value
  must change, change it there and say so in your report.
- Layout is clean: nothing overlaps, clips, or leaves the frame; consistent
  alignment; equations aligned at `=`.
- The family's quiz JSON (`<family>/❓️quiz/🔣️.json`) and `README.md` are
  consistent with what you changed (symbols, units, wording, references).
- Existing tests of the family pass, and tests/JSON cases that pin text,
  symbols or numbers you changed are updated (test-driven: adjust or add the
  case first). Run with
  `uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -m unittest <dotted.module>`
  (`SEMIO_TEST_MANIM=1`, `SEMIO_TEACHING_OUTPUT_ROOT=<TK>/🗑️generated/<your-name>-out`).
  The narration guard (`TK\narration-guard.sh`, once it exists) must be
  clean for your files.
- Visual verification is mandatory: render each changed chapter in the
  PREVIEW profile, German, into your PRIVATE output root:
  `repo tools exec bun -- "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render --language de --video <id> --chapter <Chapter> --profile preview --audience speaker --jobs 2 --no-play --output-root "<TK>/🗑️generated/<your-name>-out"`
  (never the shared cache; never two of your own renders at once). Extract
  frames at the changed moments with
  `.🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe`
  and LOOK at them with the Read tool; iterate until right. If the render
  aborts with "Authored sources changed while rendering", rerun. If the
  shared speech/caption layer is mid-change and breaks your render, wait a
  few minutes and retry; if a break persists and is clearly in a file you do
  not own, note it in your report and verify with what works (e.g. a direct
  manim render of the scene class).
- Staffage/devices only through handcrafted casts (`fitted_staffage`,
  `staffage_motion`); diagram glyphs may be drawn with primitives.
- Do NOT add roadmap (Fahrplan) scenes and do not edit the manifest order –
  another agent does that. Do NOT edit `🏗️authoring/🔊️speech/**` or the
  caption functions in `🏗️authoring/🎨️visuals/🐍️.py` (speech agent). You may
  add small shared drawing helpers to visuals if several scenes need them.
- Write `TK\<your-name>\REPORT.md`: per feedback item what you changed
  (file:line), references changed with evidence/URL, numbers changed,
  renamed chapters/terms, verification frame paths, open points.
- Do not stop halfway and do not ask questions: decide and finish. Final
  chat reply ≤ 10 lines: items resolved, changed files, report path, open
  points.

## Update 06:45 – capacity gate

The machine capacity gate currently reports 0 MB available although about 30 GB are free, so `repo tools exec bun -- …` renders queue forever. Until that is repaired use `repo tools exec --light bun -- "…/📜️script.ts" render …` (same arguments). Discipline instead of the gate: at most ONE render per agent at a time, `--jobs 2`, kill your own stale/queued renders first (`taskkill /PID <repo.exe pid> /T /F`). The caption layer for `$…$` and the narration guard (`bash <TK>/narration-guard.sh`, `NARRATION_GUARD_FILTER=<family>`) have landed. Card posters are now stored assets: a render never renders foreign lessons any more.
