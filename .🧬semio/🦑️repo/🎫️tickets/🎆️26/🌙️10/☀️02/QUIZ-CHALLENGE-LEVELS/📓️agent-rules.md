# Working rules for every agent of this ticket

Ticket folder (`TICKET`): `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️02\QUIZ-CHALLENGE-LEVELS`.
Normative design: `TICKET/📓️design.md`. Read it fully before anything else. Background maps (read the parts you need):
`📓️explore-core.md`, `📓️explore-react.md`, `📓️explore-proctor.md`, `📓️explore-site.md`, `📓️explore-designs.md`,
`📓️explore-tests.md` (may arrive a little later). The repo's own rules are in `C:\git\semio\AGENTS.md`; they bind you.

## Repository rules that matter most here

- Windows host, Git Bash and PowerShell. Always `cd /c/git/semio` first. Paths carry emoji: quote them.
- **Never run a modifying git command** (no commit, stash, checkout, restore, reset, add, clean, worktree). Read-only git
  (`git -c core.quotepath=false status|diff|log|show`) is fine.
- **Other sessions edit the same tree right now** (adaptive layout: react `🎨️.css`, `▶️run`, `↕️sorting`, `🃏️matching`,
  `🗂️classification`, `🏁️results`, `🗳️crowd`, `🏆️leaderboard`, `🪟️chrome`; pets: `🐾️pets` product, taxonomy file; other agents
  of this ticket: see your brief). Do not stop because of them and do not revert their work. Change files with the `Edit`
  tool on small exact hunks; re-read a file right before editing it; never rewrite a shared file wholesale; never
  regenerate or reformat a file you do not own. If a file changed under you, re-read and re-apply.
- Never write files through shell heredocs or `node -e`/`python -` with inline code (this host mangles backslashes and
  quotes, and `python -` on stdin hangs): create a script file with the `Write` tool and run the file.
- Do not run `bun install`. Package manager `bun`, task runner `nx` (`bun nx run <project>:<target>`); on this busy host
  prefix nx with `NX_PLUGIN_NO_TIMEOUTS=true`. Permanent scripts only in the `📜️script.ts` files; do not create other
  permanent script files. Temporary scripts, logs and probes go into `TICKET` (tool output under `TICKET/🗑️generated/`);
  delete your tool output when you are done, keep scripts and reports.
- Greenfield: no backwards compatibility, no optional-for-legacy members, no migration code, no deprecations. Change
  every producer and consumer at once.
- Code style: concise; match the surrounding code's naming and idiom; **no comments inside definitions**; every
  docstring starts with a unique fitting emoji and may carry `@see` links; no runtime dependency on external libraries
  (third-party libraries only in tests as oracles).
- Test-driven and multi-implementation: TypeScript and Rust twins side by side, behaviour equal bit for bit where the
  design says so; every feature has a language-agnostic case over shared vectors whose expected values an independent
  Python reference produced with a third-party library.
- New directory names must be registered by hand in
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (member lists such as `members-of-modules`); another
  session is editing that file, so use `Edit` on a small anchor. Check with
  `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` from the repo root.
- Texts shown to learners exist in English and German (German informal *du*), no default language, no plural forms
  ("Label: n"), every i18n key used as a string literal.
- Accessibility: meaning never by colour alone; keyboard operable; live regions only where the design says.
- Never claim a test passes that you did not run, and never claim behaviour you did not observe. Temporary logs carry the
  prefix `[DEBUG] ` and are removed before you finish.
- Do not ask questions and do not stop halfway: decide, and write the decision down. If the design is silent or wrong on
  a point, choose what fits its intent, and record the choice under "Deviations and decisions" in your report.
- Big Rust builds can exhaust memory on this host and several agents share the machine: build only the crates you need
  (`-p <crate>`), one cargo invocation at a time, and retry once when a build is killed or a lock times out.

- Any change of `🧬️schema/🔣️.json` beyond prose raises `$defs/WireVersion` and `WIRE_VERSION` in `🟦️.ts`/`🦀️.rs`, and the
  fixture `🧫️fixtures/🤝️wire-version/🔣️.json` is recommitted from `.venv/Scripts/python.exe
  🧰️framework/🛍️products/❓️quiz/🧪️tests/🤝️wire-version/🐍️.py` (convert CRLF to LF). A feature text must not put `:` right
  after a `shared://` URI. Every fake transport in client tests answers `GET /instance` with `quizInstance()`.
- On this busy host set `SEMIO_TEST_BUDGET_MS=900000` when a test budget kills a run.

## Your report

Write `TICKET/📓️report-<your area>.md` (with the `Write` tool): what you changed (file list: created, updated, removed),
what you ran with the exact command and the observed counts, what is red or not run and why, "Deviations and decisions",
and "Notes for the next agent" (new function names and signatures, anything the other layers must know). Your final
message to the coordinator is at most 300 words: outcome, gates run with counts, open problems, and the report path.
