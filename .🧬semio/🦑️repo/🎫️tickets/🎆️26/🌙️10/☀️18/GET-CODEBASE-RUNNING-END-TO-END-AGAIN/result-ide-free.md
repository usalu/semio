# result-ide-free (executor ide-free, wave 1021a)

READY (staged, rehearsed on a copy, NOT applied): `C` = this ticket folder, tools `C/ide-free-tools/`. Nothing was written outside `C/` and `/private/tmp/ide-free/` (overlay copy). No `bun repo`, cargo or bazel was run; no git-modifying command. Load guard honoured (every heavy step waited for load < 30, `nice -n 10`, one process at a time).

Brief: audit `audit-ide-1021a.md`. Order of the sections below follows the brief items (1) to (5).

## 0. How to apply (after the coordinator's GO)
```
cd /Users/ueli/Documents/semio
python3 C/ide-free-tools/apply.py                                   # dry run of the default groups (read only), safe any time
python3 C/ide-free-tools/apply.py --write                           # plans code guard leftovers agent-client; refuses paths under the frozen library/process trees
python3 C/ide-free-tools/apply.py --write --allow-frozen            # only after chain17 is green (library tree edits restart a running chain)
python3 C/ide-free-tools/apply.py --group extension --group manifests [--write --allow-frozen]   # decision D1, opt-in, see section 5
python3 C/ide-free-tools/merge-lock-sync.py                         # joins the manifest rows to the round-5 lock-sync batch (section 5)
python3 C/ide-free-tools/apply.py --group plans --undo --write      # plans are fully reversible; other groups restore from C/🗑️generated/ide-free/backup/<stamp>/
```
Engine (`apply.py`): anchored edits (counts per anchor), sha256-guarded whole-file writes, sha256-guarded deletes and moves, creates; all-or-nothing with rollback; drift refusal; `--dry-run` is the default; every path <= 256 UTF-16 units (longest 213); refuses while a dependency lock/sync/ensure or `show projects` runs; refuses writes under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/` and `🧰️framework/🔨️modules/🏃️process/` without `--allow-frozen`. Re-run is idempotent (second run on the copy: 1046 x `already`). Rebuild after drift: `python3 C/ide-free-tools/build-plan.py` (read only on the live tree).
Combination rule: the groups `code`, `guard`, `leftovers`, `agent-client`, `plans` edit some shared files (taxonomy.json, .gitignore, the MCP descriptions); apply them in one invocation (the default). `extension` needs `leftovers` applied first or in the same invocation.

Plan size: 162 anchored edits, 45 sha-guarded whole-file writes, 340 created `🎫️ticket.json`, 61 deletes, 438 moves (431 plans + 7 directory renames).

## 1. Plans become ticket-owned files (group `plans`, all-or-nothing)
Result: `.cursor/plans/` (429 tracked + 2 ignored plans) and `.codex/plans/declarative-actions.md` leave the repository root; `.cursor` and `.codex/plans` are removed when empty. Mapping: `ide-free-tools/staged/plans-map.json` (rebuilt by `build-plans-map.py`).
- Naming: a plan keeps its name and sits in the ticket folder (`<ticket>/<name>_<id>.plan.md`, the convention of the 111 plans already archived by `ticket_close`); `.codex/plans/declarative-actions.md` becomes `declarative-actions.plan.md`. Longest destination path 213 UTF-16 units.
- Evidence (the repository history is synthetic: no plan was ever committed together with its ticket, and only 16 of the 429 plans were referenced by any `🎫️ticket.json`):
  - 15 plans move to the ticket whose `plan.id` names them (T1);
  - 75 plans move to the ticket their own ticket section/todo names (T2, 53 by rule, 22 curated; 28 mentions that only name a ticket as context were rejected, list in `build-plans-map.py` `REJECT`);
  - 1 plan (`neural_schemas_and_operators_f692e604`) is deleted: its ticket already owns the swept copy and the `.cursor` copy is the stale spelling (flow/module vs flow/modules);
  - the remaining **340 plans have no ticket**. Each gets its own closed archive ticket (the precedent of ticket `RELOCATE-COMPLETED-CURSOR-PLAN-FILES-INTO-TICKET-FOLDERS`): title = titleized plan name, description = plan overview, goal `AI-OPTIMIZED-REPO`, `plan` = {client cursor, id, local}, folder dated by the day the plan entered version control (96 on 26/05/23, 49 distinct days). Decision for the coordinator: this is 340 new ticket folders; the alternative is one `ARCHIVED-CURSOR-PLANS` ticket (not implemented, say so and I add `--archive-mode single`).
- 7 `🎫️ticket.json` swap `plan.source` (absolute `.cursor/plans/...` path, some from other machines) for `plan.local`, exactly what `ticket_close` does.
- `.gitignore`: `!.🧬semio/🦑️repo/🎫️tickets/**/*.plan.md` rescues ticket plans from generic rules (two plans, `target_schema_*`, were silently ignored by `target*` and were never tracked; they now move into their archive tickets and stay trackable).
- Schema (`🎫️tickets/🧬️schema`): `TicketPlan` states the ownership rule (a ticket owns every `<name>_<id>.plan.md` of its own folder, no plan lives in a client directory of the repository, `source` is only a client-owned file such as a home-directory plan or a Kiro spec); `McpClientKind` description: Cursor and generic carry no plan attachment.
- Code (group `code`): the Cursor branch is deleted from Go `🗺️plan-source`, its Rust mirror `💾️documents`, the client CLI `🗺️plan-resolution`; Cursor loses `plan_id` on every MCP surface (two Go factories, `💻️client/🔌️mcp` repository, Rust `🔌️mcp` repository incl. the exhaustive `match`, the surface fixture `extraTicketArgument: null`, the closed argument schema description, Rust unit test, Go protocol-contract test, capability-listing feature) and the three `arg_plan_id` Cursor descriptions; model comments say client-native. Library: commit bullet area `.cursor/plans` becomes `ticket plans`, `cursor-plan-snapshot` population removed from `🔣️taxonomy.json`, `taxonomy-document` (four to three populations) and `audit-leaf-and-reference-policies`, the frozen coordinate path for `puzzle_5d_design_parity_61aab236` follows its plan to `PUZZLE-DESIGN-PARITY`, tests updated (`historical-document-evidence`, micro-commit metrics, tickets unit, client ticket-plan: Cursor is refused; the live GitHub test reads the plan from its archive ticket), skill `micro-commit/SKILL.md`, formatter `IsIdeConfigurationPath` (`.cursor` has no exception any more).
- Kept on purpose: Kiro (`.kiro/specs/<id>` is still a plan source; `.kiro` is already forbidden by the authority guard, say so if it should go too), historical ticket artifacts that quote old `.cursor/plans/...` paths (ticket workspace is excluded from reference scanning), `PlanClientTag("cursor")` (the 120 existing tickets carry `client: cursor`).

## 2. Guards (group `guard`, item 5)
- `ide-census` (library `dashboard-control`): fixture `directories` += `.cursor`, `.windsurf`; new closed field `directoryPaths: [".github/hooks"]` (schema `required` + property); classifier `retiredIdePath` matches a contiguous path sequence (the physical walk now classifies the relative path, not only the entry name); the SQLite oracle gets the same clause (`?8`); 7 new path cases (retired: `.cursor`, `.cursor/plans`, `.windsurf`, `.github/hooks`, `.github/hooks/repo.json`; accepted: `.github/workflows`, `.github/hooks-notes`).
- Dashboard authority (`🎛️dashboard`): `.cursor` is a forbidden directory outright (the plans carve-out is gone from schema patterns, test regexes and fixture rows), `.github/hooks` is a forbidden path, instruction vectors updated; Go formatter mirrors it.
- Without these two checks the census would have caught row 1 of the audit: it was blind to `.cursor`.

## 3. Redundant leftovers (group `leftovers`, item 3)
- `.claude/settings.json`: five LSP plugins removed (`mcp-apps` stays).
- `.github/hooks/repo.json` (empty) deleted and `.github/hooks` removed; guarded by the census.
- `.gitignore`: removed `.idea`, `*.suo`, `*.ntvs*`, `*.njsproj`, the VisualStudio per-user block (`*.rsuser`, `*.suo`, `*.user`, `*.userosscache`, `*.sln.docstates`, `*.userprefs`), `.vs/`, `*.vspscc`, `*.vssscc`, `*.vsp`, `*.vspx`, `.mfractor/`, `.localhistory/`, `.vshistory/`, `.ionide/`; the native/.NET build-output rules (`[Bb]in/`, `*.log`, `*.pdb`, ...) and the fixtures rescue stay (header renamed `NATIVE-BUILD-OUTPUT`). `**.vscode-test`, `*.vsix`, `# *.code-workspace` go with the extension (section 5).
- 20 tracked `tsconfig*.json`: `"$schema"` (editor completion only) removed. NOT removed: `"plugins": [{ "name": "next" }]` in the coordinator tsconfig, `next build` rewrites it (framework-managed).
- `pyproject.toml`: `jupyter`, `notebook`, `ipykernel` removed (0 tracked notebooks). `uv.lock` still lists them and their transitive packages: regenerate it with the python lock command (not hand-editable).

## 4. `editor` becomes `agent-client` (group `agent-client`, item 4)
Fail-closed codemod over an explicit allowlist of 44 files (providers, hooks, client CLI, MCP): `EditorProvider(s)` -> `AgentClientProvider(s)`, `AllEditorProviders`/`GetEditorProvider` (Go and Rust snake case), `EditorForClient` -> `AgentClientFor`, `EditorClient`/`EditorKind` schema defs -> `AgentClient`/`AgentClientKind`, fixture key `editors` -> `agentClients` (Go, Rust, JSON together), `formatVSCodeHookOutput` -> `formatCopilotHookOutput` (Go, Rust, schema `CopilotHookOutput`), prose "editor provider(s)", "eight editors", "IDE-native", "per-IDE". Directory renames: `🎆️editors` -> `🎆️agent-clients` (Go), `🎆️editor-providers` -> `🎆️agent-client-providers` (Rust incl. the `#[path]`, and the client CLI), test `🪝️editor-hook-output-format` -> `🪝️agent-client-hook-output-format` (+ fixtures, feature tags), taxonomy member names follow. gofmt-normalised where the original was gofmt-clean. 0 leftover `editor` lines in the allowlist (`staged/agent-client-leftovers.txt`).
Kept (they mean a text editor or an IDE): `.editorconfig`, `core.editor`, `stream_editor` (sed), `🖋️editor-overlay`, `🖊️editor-policy`, "Editor-owned configuration is forbidden" (statutes), developer-tools "external source editor", `"editor": "system"` fixtures. Not undoable by anchors (whole-file writes): restore from the backup directory.

## 5. Decision D1: the VS Code extension leaves the repository (groups `manifests` + `extension`, opt-in)
Recommendation (b) of the audit, as the brief asks. The two groups only work together and are NOT part of the default apply.
### 5a. Manifest rows for the round-5 lock-sync batch (recorded as requested)
| file | edit |
|---|---|
| root `package.json` | workspace row `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript` removed; `@types/vscode`, `@vscode/test-cli`, `@vscode/vsce` removed |
| `♻️mit-bestand`, `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech` `package.json` | the same workspace row (with the `../` prefix) removed |
`python3 C/ide-free-tools/merge-lock-sync.py` writes `staged/lock-sync-plan-merged.json` = the lock-sync plan + these 8 edits (group `manifests`); `python3 C/lock-sync-tools/apply.py --plan C/ide-free-tools/staged/lock-sync-plan-merged.json --group manifests` dry run: clean, 17 edits + 1 create. One lock/sync cycle then carries every manifest edit (any framework manifest edit rebuilds all six scopes anyway). `lock-sync-tools/run.py` runs `apply.py` with its own default plan: the lock-sync owner swaps `lock-sync-tools/staged/plan.json` for the merged file right before ROUND5 GO (not done here, it is that owner's file).
Predicted lock delta with the merged batch (`lockdelta.py --overlay`, `staged/predicted-lock-delta.json`): every one of the six locks +1 workspace (`build-parsers`) and -1 workspace (the extension); root lock additionally the 4 Nx rows and the 3 VS Code tool rows leave the root workspace row (7 changed rows); no third-party package is added or re-versioned; the extension-only packages (`@vscode/test-electron`, ...) simply leave the lock.
### 5b. The `extension` group (module and every dependent)
Deletes the 59 tracked files of `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode` (history keeps them; they move to a separate repository by the owner) and cuts the dependents: `📋️build-owners.json` entry, `🔒️dependencies.json` (4 tools + 3 user paths), owner-projection policy + its twin fixture (`readme-license-owner-authority`), `tool-configuration-ownership` fixture/test/validators (`vscode-test` loader and `vscode-test-cli-config` contract), `repo-source-ownership`, process `capacity` fixture (census 55 -> 53 files, 95 -> 93 literal calls), `repo build` target map and synopsis (3 scripts + registry), library `project.json` input, `web-build` test (extension build config assertions and the watch regression block, plus `extensionSource`/`watch` fixture keys), `cache-contracts` command-inputs test (two extension calls), the dashboard authority test of the extension, READMEs, `.gitignore` VSCODE block. Removed rows are listed in `staged/extension-removals.txt`.
Kept on purpose: the historical `remaining-package-purity-authority` census (snapshot of 2026-08-27, `decisionState: non-authoritative`), the lifecycle fixture `forbiddenArgs` entry `@semio-tech/repo-vscode:build-vsix` (a guard against resurrection), the taxonomy member name `🧩️vscode`, and the module's own `AGENTS.md` (agents do not edit `AGENTS.md`: the owner deletes it, the rest of the directory is already empty after the apply).
**Not verified at runtime (no GO):** run after the apply, in this order: `tool-configuration-ownership`, `readme-license-owner-authority` (+ owner-projection-policy), `repo-source-ownership`, process `capacity/scope`, `web-build`, `cache-contracts/command-inputs`, dashboard `authority`, `historical-package-owner-identity`. Risk points: `publisherVerifications` becomes `[]` in the readme-license fixture; the cases/entries shift by three rows; `cases.json` keys `watch` and `extensionSource` are removed from the web-build fixture.

## 6. Verification (overlay `/private/tmp/ide-free/root`, a copy with the whole batch applied)
VERIFICATION_PLACEHOLDER

## 7. Remaining decisions and open items
1. 340 archive tickets or one archive ticket (section 1).
2. D1 timing: take `extension` + `manifests` into the round-5 cycle or later (section 5); until then the default groups stand alone (no default group touches the extension).
3. Kiro `.kiro/specs` as plan source (section 1, kept).
4. Frozen trees: 30 library/process paths change (`code` 6, `guard` 3, `leftovers` 1, `agent-client` 1, `extension` 19); apply only after chain17 is green, with `--allow-frozen`.
5. `uv.lock` regeneration; module `AGENTS.md` removal by its owner; goal `REPO-VSCODE-EXTENSION` (due 2026-02-15, still open) is status only, not touched.
6. Rust edits (tickets `💾️documents`, MCP `repository`, hooks, providers) are not compiled here (cargo is gated); the Go side is vetted and tested on the copy.

## Files
`C/ide-free-tools/`: `apply.py` (engine), `blib.py` + `s_plans.py`, `s_leftovers.py`, `s_code.py`, `s_guard.py`, `s_agent_client.py`, `s_extension.py`, `s_manifests.py` (declarative sections), `build-plan.py` (regenerates `staged/plan.json`), `build-plans-map.py` (regenerates `staged/plans-map.json`), `merge-lock-sync.py`, `overlay.py`, `census-harness.py`, `verify.py`, `staged/` (plan, map, 45 staged whole-file writes under `staged/new/`, predicted lock delta, removal and leftover listings).
