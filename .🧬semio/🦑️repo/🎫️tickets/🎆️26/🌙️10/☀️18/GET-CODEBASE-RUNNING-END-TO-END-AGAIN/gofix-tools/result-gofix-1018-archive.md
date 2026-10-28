# Result: gofix (Go test fixes, applyAutofixes decomposition, devcontainer drift)

## Changes
1. `🌳️tree` (3 failures)
   - `🔦️search_test.go`: G1 fixture path was stale (`📚️library/🧪️tests/1️⃣g1-contract/🧫️fixtures/🔣️.json`); the fixture lives at `📚️library/🧪️tests/🧫️fixtures/1️⃣g1-contract/🔣️.json`. Path corrected (no fixture authored).
   - `TestComputeCompositeFingerprintFallsBackWhenRecursiveSubmoduleStatusFails`: the submodule worktree (`child`) inherits no repo-local `commit.gpgsign=false`, so the dev's SSH signing key was invoked; the test now sets `commit.gpgsign=false` in `child` (hermetic).
2. `⌨️cli` (7 failures): `🎲️g1_test.go` same stale G1 fixture path (6 tests, fixed). `🥾️bootstrap_test.go`: stale expectations removed/updated to the current truth:
   - `.kiro/settings/mcp.json`, `.windsurf/mcp.json` cases removed: both were deliberately deleted (commit 9c641044be4, 2026-09-23); the real generator (`📚️library/🤖️agent-clients/🟦️.ts`, `bun repo run workspace:generate-config`) derives only claude-code, vscode, cursor, codex. Nothing to generate, no generator mentions kiro/windsurf.
   - `.devcontainer/post-create.sh` case replaced by a `devcontainer.json` case (`postCreateCommand` = `bun repo run workspace:setup`, no `post-create.sh`); the script was deleted 2026-09-09.
   - Native `🐚️.sh` / `🔵️.ps1` cases now require `bun repo run workspace:setup` and `bun repo run @semio-tech/repo-client:build` (what the scripts do); raw `cargo build` is refused by the compiler ingress gate, so the old literal could never be right.
3. `🔗️graphql` `applyAutofixes` (1,047 lines) decomposed: `autofixState` + `loadAutofixState` + dispatcher `fix` + `finish` + 13 per-kind methods; files (lines): `🩹️autofix.go` 146, `🧱️autofix_sections.go` 124 (empty section, newline after region, end name, name mismatch, section summary), `📑️autofix_docstring.go` 312, `📝️autofix_summary.go` 253, `📋️autofix_requirements.go` 202, `💬️autofix_comments.go` ~130 (inline, block/jsdoc), `🪪️autofix_file_policy.go` 114 (emoji variation, license). Method bodies were moved by a go/ast tool (outer variables -> state fields; the only rewrites are `break`/`continue` that targeted the outer switch/loop -> `return`).
   - Proof of no behaviour change: package tests identical before/after (35 top-level results, only `TestSystemPolicy` failing before); plus a temporary differential test (removed) running the original function copy vs the new one over 19,690 cases (every autofix kind x 8-12 language samples x every line x column/scope variants, incl. panics and returned counts): 0 differences; all 13 kinds were hit with fixes > 0.
   - Not touched: the duplicate 1,045-line `applyAutofixes` in `💻️client/⌨️cli/🛟️autofix/🐹️.go` (gocomp/gocomptest scope).
4. `📜️statutes` / `📡️events` (found in step 4): fixture paths stale after fixture moves. `📜️statutes/🔬️_test.go` -> `📚️library/🧪️tests/🧫️fixtures/🔏️path-emoji-statutes/🔣️.json`; `📡️events/🔬️_test.go` -> `📡️events/🧪️tests/🧫️fixtures/*.json`.
5. Devcontainer drift (`TestSystemPolicy/the_checked-in_devcontainer_block_equals_its_derivation`): `.vscode/settings.json` (host canonical, Oct 14) gained five `git.*` keys (`git.enabled`, `git.autorefresh`, `git.autofetch`, `git.decorations.enabled`, `git.timeline.showUncommitted`, all false) that `.devcontainer/devcontainer.json` `customizations.vscode.settings` lacked. The derivation (`derived-config/🟦️.ts` `deriveContainerEditor`, `workspace:generate-config`) inherits everything not in `editor-overlay.json` omit, so the drift was in the checked-in devcontainer file. `node_modules` is a dangling symlink (deps not installed), so I did NOT run the generator; I hand-added the five keys at the top of the block in host order, which is what the generator emits. Go derivation test now passes. Run `bun repo run workspace:generate-config-check` once deps are restored to confirm.

## Verification (`go test -count=1 <pkg>`, one package at a time; logs in scratchpad)
| package (github.com/usalu/semio/...) | before | after |
|---|---|---|
| framework/filesystem/durability | pass | pass |
| repo/yaml, search, identity, model, languages, providers | pass | pass |
| repo/codebase, metrics, move, contributors, goals, todos, testrunner, hooks | pass | pass |
| repo/tree | FAIL (3) | pass |
| repo/graphql | FAIL (TestSystemPolicy devcontainer drift) | pass |
| repo/cli (⌨️cli) | FAIL (7) | pass |
| repo/cli/bin, repo/coordinator, repo/coordinator/bin, semio.tech/repo/test | pass | pass |
| repo/events | FAIL (6, fixture path) | pass |
| repo/statutes | FAIL (1, fixture path) | pass |
| repo/workspace | FAIL (`TestMcpStdioInitializeHandshake`) | FAIL, unchanged |
| repo/tickets | skipped (gotix still running, no result file) | skipped |
| 💻️client/⌨️cli (+ main) | skipped (gocomptest pending; 58 known failures per result-gocomp) | skipped |
| 💻️client/🔌️mcp, 📚️library | not runnable with plain `go test`: emoji directories are invalid import paths (need the repo overlay pipeline) | n/a |

`gofmt -l` clean and `go vet` (tree also with `-tags exhaustive`) OK in tree, graphql, cli.

## Open
- `workspace` `TestMcpStdioInitializeHandshake` runs `bun ./📜️script.ts dev mcp stdio cursor`; bun fails with `Cannot find module './🔍️discovery/🟦️.ts' from 📚️library/🟦️.ts` (the library monolith split in progress: `🔍️discovery/` has only subdirectories, no `🟦️.ts` yet, mtime 03:16) and `node_modules` is dangling. Not a Go problem; retest after the library split lands and deps are restored.
- Re-run `tickets` and `💻️client/⌨️cli` once gotix/gocomptest finish.
