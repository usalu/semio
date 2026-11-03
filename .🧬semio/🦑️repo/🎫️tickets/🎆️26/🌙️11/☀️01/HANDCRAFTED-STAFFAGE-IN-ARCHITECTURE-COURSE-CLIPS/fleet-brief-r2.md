# Fleet brief, round 2 (final verification and polish, 20 agents)

Every agent reads this first.

## Paths
- Repo `C:\git\semio` (Windows). Use the Bash tool (Git Bash) and always `cd /c/git/semio` first.
- S = `🎓️teaching/🏛️architecture/🧍️staffage` (handcrafted archive; catalog `S/🗂️catalog`)
- V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals` (Python staffage API; manifest `V/🧍️staffage/🔣️.json`)
- T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS` (this ticket)
- Context to read: `T/contract.md`, `T/summary.md`, `T/audit-final-static-r2.md`. Read the report of the agent that last touched your area: `T/migration-*`, `T/polish-f2-report.md`, `T/verify-v2-report.md`, `T/render-audit-r*.md`. Handcrafted asset rules are in ticket `26/10/29/ARCHITECTURAL-STAFFAGE-ARCHIVE` (`authoring-v2.md`, `content-worker-brief.md`).

## Rules
- Follow `C:\git\semio\AGENTS.md`. Never edit AGENTS.md.
- No modifying git commands.
- About 20 agents edit concurrently. Make minimal Edit-tool changes and never revert foreign changes. If someone else changed a line, build on it.
- Bash heredocs and `node -e` strip backslashes, so write files with Write/Edit.
- `python -` or `cat >` on stdin hangs. Write a script file into `T/🧰️<yourid>/` and run it.
- Do not leave background commands running when you finish.
- Memory: render ONE scene at a time, quality l, 15 fps, `--jobs 1`. Kill your own render if it exceeds about 4 GB.
- Generated output goes to `T/🗑️generated/<yourid>/`. Delete it when done (keep scripts and the md report).
- Docstrings start with a unique emoji. No comments inside definitions. Code is concise. en and de with no default language.

## Runners
- Python unittest: `T/m9_tests.sh <module args>` (uv, SEMIO_TEACHING_OUTPUT_ROOT preset). For clip tests pass the test module path the same way the earlier reports did.
- Staffage vitest: ticket-local alias configs (see `T/q2-report.md`), because `repo dependencies ensure` refuses (repo infra, do not fix).
- Catalog check: the command in `T/migration-g2-report.md` / `T/verify-v2-report.md`.
- Render: see the scripts `T/m9_render.sh`, `T/f2_render.sh` and the per-agent render notes in reports. Frames are extracted with PyAV (repo ffmpeg is broken), and SVGs are rasterised with PyMuPDF (no rsvg-convert).

## Deliverable
- Write `T/r2-<yourid>-report.md` (findings with file:line, what you fixed, test output counts, what is still open).
- Your final chat message is at most 5 lines and points to that file.
