# 🔎️ Wave R11 Resume Audit Brief (read-only)

The previous coordinator session died at ~23:35 on 2026-10-08 while wave W1 agents were mid-work. None of them wrote
their `r10-exec-<label>.md` report. Your job: establish, from the files on disk only, exactly how far your work
package(s) got and what remains, so a Sonnet finisher can complete it without redoing work.

T = `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️08\BIM-PLUGIN` (Git Bash: `/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`).
P = `C:\git\semio\✏️s\🔌️plugins\🏙️bim`. S = `P\🗿️artifacts\🏢️model\🏅️standards\🔖️1\🪆️subsets\✳️any`.

## Rules
- READ-ONLY. Do not edit, create, or delete any file except your single report `T/r11-audit-<label>.md`.
- No cargo/bun/nx builds. No git modifying commands. `git -c core.quotepath=false status --short -- <path>` and
  `git -c core.quotepath=false diff --stat` are fine. Do NOT use the Grep tool (it is broken in this repo); use Bash
  `grep -r`, `find`, `ls`, `sed -n` instead. Paths contain emoji: always quote them in Bash.
- Bash cwd persists; always `cd /c/git/semio` first.

## Inputs
- `T/r9-audit-completeness.md` §5 — the spec of your WP(s) (mutation kinds, inference fields, UI, IO, laws and tests).
- `T/r10-wave-brief.md` and `T/r9-decisions.md` — the laws and rulings.
- The agent's own input scripts in T: `T/r10-<label>-*.{ts,py,mjs}` — they reveal intent (which leaves/fixtures they generated).
- The agent's last build logs: `T/🗑️generated/<label>/check*.txt` (highest number = last). Report the last errors.
- The sources: mutation leaves live in `S/🧬️schema/🧬️mutations/<emoji><kind>/` with `🦠️mutation`, `🔺️diff`, `↩️inverse`
  (and tests/fixtures); inferences in `S/🧬️schema/💡️inferences/`; snapshot entities in `S/🧬️schema/📸️snapshot/`;
  editor in `S/✏️editor/`; IO in `S/🚪️io/`; examples in `S/🧫️fixtures` or `S/📚️examples` (find them).

## Report `T/r11-audit-<label>.md` (≤ 200 lines)
1. Status line: NOT STARTED / PARTIAL / NEARLY DONE / DONE.
2. Table per WP item (each mutation kind, each inference field, each UI piece, each IO piece, examples, `.feature`
   scenario, third-party oracle): present? wired? (leaf dir exists; registered in the mutation enum/aggregate and
   KINDS; inference wired into `🕸️model-graph`; UI registered in editor mount/command table; labels en+de). Give paths.
3. Last compile errors from the newest `check*.txt` (verbatim, ≤ 20 lines) and whether the referenced file now exists.
4. Missing files referenced by `#[path]`/`mod` declarations in your area (check that every `#[path = "…"]` target exists).
5. A precise, ordered REMAINING TASKS list for the finisher (concrete files and kinds).
Final chat message: ≤ 6 lines with status + report path.
