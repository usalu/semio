# Handle rules (wave 2, gap 10)

Finding: all 12 reported paths already have rules; none is stale (no `stiffenerSpacing`/`layup`).
- EN 1992 (9): `bars[].y|z|length`, `tendonPoints[].x|z`, `studRings[].radius` are member-relative literals of `members[].bars[]...`, `members[].tendonPoints[]...`, `members[].studRings[].radius` (rules exist). `members[].longitudinal[]`, `members[].punching`, `members[].studRings[]` are entity source refs (containers of existing rules), not writes.
- EN 1996 (2): `walls[].openings[]`, `walls[].loadCases[].concentrated[]` are entity refs; leaf rules exist.
- EN 1995 (1): `cases[].numbers[]` is an entity ref; `cases[].numbers[].value` exists.

Fix: `🧰️scripts/handle-paths-check.py` matches member-relative suffixes and container refs, exits 1 on a real gap. Result: 0 for all ten codes.
Test: `✏️s/🔌️plugins/📕️norm/⚖️compliance/🧪️tests/🔖️handle-paths/{🐍️.py,🥒️.feature}` (language-agnostic, text-based, all artifacts); passes.
No artifact source touched, so no latest.md rebuild line. No cargo run.
