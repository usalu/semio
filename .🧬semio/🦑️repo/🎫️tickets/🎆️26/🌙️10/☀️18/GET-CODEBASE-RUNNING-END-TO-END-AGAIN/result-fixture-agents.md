# Result fixture-agents: re-pin laws, fixtures and docs to the new AGENTS.md (2026-10-20)

Executor fixture-agents, foreground, no sub-agents, no builds, no git modification, AGENTS.md untouched. New AGENTS.md (commit fd14da55f18 plus staged extra line): launch.json rule, "package.json calls nx", the `📜️script.ts`-only rules and "no search tool" are gone; human devs use `repo dashboard`; agents use the `repo` binary; no IDEs.

## Census (git grep --untracked, outside tickets and .cursor/plans)

Pinned the removed text (fixed): the dashboard authority law, its fixture and its schema (protected launch sentence). Documented agents on the dashboard (fixed): root `README.md` and `🧰️framework/🛍️products/🦑️repo/README.md`.
Checked and left (they assert the ABSENCE of IDE launch files, still true under "MUST NOT use IDEs"): course `🧪️tests/🎛️dashboard` (4 pass), play `🧪️playfreshbuild` (`retiredLaunchFiles`), library `🧪️tests/🎛️dashboard-control` (+ `🔍️ide-census.json`), `⚡️caching/…/🔁️lifecycle` and `📝️derived-config` fixtures, `🧫️frozen-seal-ledger` (historic deletion rows), `🪡️planner-blockers` (synthetic path), hub capture law, `watch-policy.json`. No dead launch-projection code exists any more (grep for generateLaunch, writeLaunchCatalog, projectDashboardLaunch, launch.seed outside tests/fixtures: none; the authority law already asserts it).
Not touched: nested `AGENTS.md` files (rule), `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md` (user prompt history), per-directory AGENTS header tooling (Go `agents-docs-headers`, statutes: unrelated to the root rules).

## Changes

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🧭️authority/🔣️.json`: `$defs.ProtectedInstructions` (const launch sentence) replaced by `$defs.DashboardInstruction` (`path` const `AGENTS.md`, `text` const `All human devs are using \`repo dashboard\`.`).
2. `…/🎛️dashboard/🧪️tests/🧫️fixtures/🧭️authority/🔣️.json` (CRLF kept): `protectedInstructions` + `protectedInstructionCases` replaced by `dashboardInstruction` + `dashboardInstructionCases` (5 vectors: exact accepted; README, nested AGENTS, old launch sentence and appended `.vscode/launch.json` refused); the old launch sentence in `instructionCases` is now a short refused vector, plus an accepted vector for the dashboard sentence.
3. `…/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts`: test "protected instructions…" rewritten to "root agent instructions name the dashboard as the human surface and need no editor exemption" (schema + independent predicate agree, sentence occurs once in AGENTS.md, WHOLE AGENTS.md passes the IDE-refusal predicate with no exemption); the developer-documents test no longer strips an exempt sentence.
4. `…/🎛️dashboard/🧪️tests/🧭️authority/🥒️.feature`: scenario "Protected root instructions…" rewritten to the new contract.
5. `README.md`: agents no longer told to run `bun run dashboard run …`; they use `bun repo run <project>:<target>` (3 sentences: "Agents (Claude Code…)", "agents and CI run", "Git hooks…" example); "dashboard commands" wording now "which the dashboard lists as commands".
6. `🧰️framework/🛍️products/🦑️repo/README.md` (EN + DE): "agents and automation use the same daemon through the `semio` client" -> `bun repo` binary, never the dashboard; and the sentence "(`.vscode/` does not exist in the tree)" -> "(no editor configuration directory exists in the tree)" (EN + DE). The old wording was itself refused by the IDE-refusal predicate (`.vscode/` + slash), i.e. the developer-instructions law was red on this README before this change. A peer edited the same line concurrently (adds "`AGENTS.md` calls the human surface `repo dashboard`…"); my edits were anchored substring replacements and kept theirs.
Scratch (inputs, kept): `C/fixture-agents-tools/t.sh` (single-file bun test runner with the finite launcher envelope, `-t` filter, `ALARM` env), `C/fixture-agents-tools/docs-check.ts` (applies the law's own predicate to all 131 README/CONTRIBUTING/DEVELOPMENT/AGENTS docs).

## Verification

- `zsh C/fixture-agents-tools/t.sh <authority>/🟦️.ts -t "root agent instructions|active editor launch"` -> 2 pass, 0 fail, 18 expects (before the change: the old protected test failed, `Received: 1` occurrence of the removed sentence).
- `bun C/fixture-agents-tools/docs-check.ts` -> 131 developer documents, 0 refused outside ticket folders (the law's inventory excludes `.🧬semio/🦑️repo/🎫️tickets`; the 2 refused hits are scratch READMEs in old tickets). Equivalent of the "developer instructions" law.
- Full authority file (load average 36-60): 37 pass, 11 fail. The real "developer instructions agree…" test timed out at its own 300 s limit while walking ~520k directories under load (not an assertion failure; covered by docs-check). The other 10 fails are NOT AGENTS-related and pre-existing/peer drift:
  - 4 `🧊️execution/🛠️bootstrap` tests: ENOENT `📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️packages/🦀️rust/📋️project.json` (dashboard bootstrap moved into the library).
  - "Go repository implementation has a dashboard-owned source build": output `{workspaceRoot}/{workspaceRoot}/.🧬semio/…/repo` doubled prefix in the root `build` target outputs.
  - "authored scripts use one declarative build entry": tool `run-script` now carries `--dry-run={dry-run}` (command-registry G4 change); fixture vector stale.
  - "every active source directory is free of IDE configuration": `👴️leutwiler/💤️realparts-of-powers-z-n/🧘️lean/.lake/packages/mathlib/.vscode/**` and proofwidgets `.vscode` (a Lean package checkout inside the working tree; inventory has no boundary for `.lake`).
  - "all active owner manifests retain executable canonical scripts": 18,726 `duplicate owner` rows (project.json copies being walked twice; needs an owner look).
  - "native console width profiles" and "native dashboard resolves retained live controls": need the native `semio` binary.
- Course `🧪️tests/🎛️dashboard/🟦️.ts`: 4 pass, 0 fail. Play `🧪️playfreshbuild/🟦️.ts`: 0 tests in the file itself (imported by another suite); not run further.
- `docs-check` and the tests were run with `NO_COLOR=1`, `SEMIO_TEST_ARTIFACT_DIR` short path, finite launcher envelope; generated outputs deleted.

## Open items

1. AGENTS.md (staged working copy) also carries "You MUST NOT use `repo dashboard`." under the human sentence, absent from commit fd14da55f18. The law pins only the human sentence, so it is stable either way. If that line stays, `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md:213` ("Agents and editors start servers with `bun repo run @semio-tech/repo-dashboard-rs:run … -- run <id> --detach --wait-ready`") still routes agents through the dashboard client and needs a rewrite; owner = dashboard README peer (the repo README peer line already says an agent never starts the dashboard).
2. AGENTS says `repo dashboard` but `bun repo --help` has no `dashboard` command; the entry is the root script `bun run dashboard` (package.json frozen). README wording follows the real entry. A `bun repo dashboard` verb (or alias) is a CLI/package.json owner task.
3. The law "every active owner resolves executable commands through canonical scripts" (project.json calls only `bun ./📜️script.ts …`) and the first authority scenario (root `repo`/`dashboard` package scripts) came from the REMOVED `📜️script.ts`/nx AGENTS rules. They are still enforced and still pass/fail for their own reasons; whether to relax them is a design decision, not changed here.
4. The VS Code client module `💻️client/🧩️vscode` and `.devcontainer` IDE-free policy remain; "MUST NOT use IDEs" may justify retiring the extension: user decision.
