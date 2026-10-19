# 🧭️ AGENTS.md Rule Audit 2026-10-19 (read-only)

Scope: tracked files at `HEAD` (`bd9e4e02bd0`, branch `🐙ueli/⛳wip`), excluding `.🧬semio/**` except item 8. Working-tree changes are not audited. Nothing was edited, staged or committed. Evidence lists and scan scripts are in `C/aud-rules-tools/` (`evidence/`, `*.ts`).

## Counts

| # | Rule | Count (HEAD) | Severity |
|---|------|--------------|----------|
| 1 | Leftover `[DEBUG] ` logs | 1,068 log-call lines in 621 files; 86 in production code (36 files), 63 of those inside permanent `📜️script.ts` | high (prod), policy (tests) |
| 2 | Permanent scripts other than `📜️script.ts` | 5 hook scripts, 2 bootstrap scripts, 1 Windows `.ps1`, 1 stray `.cjs`, 35 build-library `.mjs/.cjs`, 23 Python oracles | high, mixed decision |
| 3 | `project.json` not calling `📜️script.ts` via `repo:script` | 144 implicit `run-commands` targets, 1 `repo:noop` (of 3,792 targets / 1,448 files) | medium, agent-fixable |
| 4 | Compat layers / shims | 1 explicit shim, 2 compat definitions/aliases, 2 public re-export shims, ~5 borderline | high for explicit shims |
| 5 | Comments inside bodies (recent files) | 4,439 lines in 426 files (7,738 files read) | medium, agent-fixable (large) |
| 6 | Docstrings not starting with emoji (recent) | 1,425 of 40,912 blocks (3.5%) in 663 files | low-medium, mostly Rust field docs |
| 7 | Runtime external deps in framework `dependencies` | 46 entries in 11 packages | decision |
| 8 | Tracked generated/stale files | 34 hash-named lockfile copies, 1 Mach-O binary, 5 junk files, 150 ticket code copies, stray dir + file | high, agent-fixable |

## 1. `[DEBUG] ` leftovers

- Test files: 982 log-call lines (`console.*`, `println!`, `eprintln!`) prefixed `[DEBUG] ` are used as permanent evidence prints. Owner: test suites across `✏️s/🔌️plugins/*`, `🧰️framework/*`.
- Production, agent-fixable (temporary by rule, remove): 86 lines in 36 files. Top:
  - `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/📜️script.ts` (12 lines, e.g. 65, 77, 92, 104, 203, 215, 230, 242)
  - `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts` (8: 60, 75, 89, 103, 508, 557, 591, 625)
  - `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts` (7)
  - `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts` (6)
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/🛤️lane/🟦️.ts` (5: 91, 104, 132, 149, 163)
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts:53`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:133,153` (env-gated `SEMIO_BUILD_TRACE_AUTHORITY`)
- Non-log `[DEBUG]` hits (assertion/expected strings, JSON fixtures at 62 hits, markdown/patch 102) are not counted above.
- Evidence: `evidence/item1-debug-logcalls-src.txt`, `evidence/item1-debug-hits-src.txt`.

## 2. Permanent scripts outside `📜️script.ts`

- Shell/PowerShell bootstrap (zero-touch, runs before Bun exists): `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh`, `…/🥾️bootstrap/🔵️.ps1`. Decision.
- Windows file-association registration: `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🖥️associations/🪟️windows/🔵️.ps1`. Decision, or move into a script.ts.
- Git hooks (`sh`, mode 755, identical 2,247-byte copies): `🧰️framework/🛍️products/🦑️repo/🪝️hooks/{post-checkout,post-commit,post-merge,post-rewrite,prepare-commit-msg}`. Git requires these entry points, so they need a decision (thin wrapper vs exception).
- Stray tracked: `extension-host-build-7a5OFB/oracle.cjs`, `entry.ts`, `library.ts` at repo root. Agent-fixable (looks like scratch output).
- Taxonomy-named JS modules implementing build logic: 35 `🟨️.mjs/.cjs` under `📚️library/🏗️build/`, plus further `.mjs/.cjs` under `📚️library/`. Decision: taxonomy modules vs scripts.
- Python oracles: 23 `🐍️.py` under `🔮️oracles/` in plugin artifacts. Decision.
- Evidence: `evidence/item2-*.txt`.

## 3. `project.json` executors

- 1,448 files, 3,792 targets. 3,647 use `repo:script` and all call `📜️script.ts`.
- 144 targets have no `executor` but a direct `options.command` (implicit `nx:run-commands`), all calling `bun …/📜️script.ts`. Agent-fixable: switch to `repo:script`. Breakdown: 95 in `✏️s/🔌️plugins/🗄️stdio`, 18 in `🧰️framework/🛍️products/🦑️repo`, 12 in `💻️os`, 10 in `🏗️fem`, and the rest scattered.
- 1 target `repo:noop`: `👴️leutwiler/💤️realparts-of-powers-z-n/📋️project.json::build` (no command). Decision: delete or keep.
- Form decision: every command is `bun <path>/📜️script.ts …` (the rule says bare `📜️script.ts <command>`). 1 form uses `bun --watch`. 8 targets quote paths.
- Evidence: `evidence/item3-project-json-deviations.txt`.

## 4. Compat layers and shims

- `@deprecated` in source: 0. Remaining `compat`/`legacy` hits are mostly tests and greenfield policy comments.
- Explicit shim (agent-fixable after checking callers): `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust/🦀️.rs:1-4`. Doc says "compatibility shim for existing launch targets until callers migrate"; body is `pub use dsl_lsp::lsp::{…}`.
- Compat definition (agent-fixable): `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scale.sty:210` ("Backwards-compatible four-argument definition used by the families still on the legacy layout").
- Compat alias (agent-fixable): `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📈️polynomial-internals/🦀️.rs:1-9` keeps `crate::polynomial` via `pub use … as polynomial;`.
- Borderline re-export shims (decision): `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🌐️browser-bundle/🏗️materialization/{🩹️preview2-shim-patches,🪢️host-shim-source}/🟦️.ts` (public package entries that re-export moved sources, `export * from`); `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🟦️.ts:5` (`export * as mainWindow`).
- Re-export-only modules (heuristic, ≤12 code lines all re-exports): 1,936 across all source, 491 in recently touched files. Most are legitimate barrels; the list is in `evidence/item4-reexport-only-all.txt`.

## 5. Comments inside function bodies (recent files)

- Scope: files changed in HEAD commits since 2026-10-18 (`bd9e4e0`, `8f91299`; 15,776 paths). 7,738 non-test source files read.
- 426 files, 4,439 lines (indented `//` or trailing `//`). Owner split: `🧰️framework/🛍️products` 2,600; `✏️s/🔌️plugins` 1,119; `🧰️framework/🔨️modules` 613; `🌎️hub` 64; `.storybook` 21.
- Top: ShellHost renderer files (`host-effects/🟦️.ts` 88, `focus-and-input/🟦️.tsx` 75, `panel-hosting/🟦️.tsx` 75, `spawned-session/🟦️.ts` 63); `🧰️framework/🔨️modules/🗣️dsl/🔍️lexer/🦀️.rs` 49; `🧪️test/🚧️mutation-gates/🟦️.ts` 50.
- Excluded: jcoprobe generated bundles (299 each). Rust `// SAFETY:` comments need a policy decision.
- Evidence: `evidence/item5-body-comments-touched.txt`.

## 6. Docstrings without a leading emoji (recent files)

- 1,425 of 40,912 blocks (`///`, `//!`, `/** */`) in 663 files.
- By area: `✏️s/🔌️plugins` 1,049; `🧰️framework/🔨️modules` 206; `🧰️framework/🛍️products` 136.
- Top: `✏️s/🔌️plugins/🏙️bim/…/📸️snapshot/🧱️entities/🦀️.rs` (180, Rust field docs), `🏛️architect/…/🧬️schema/🟦️.ts` (72), `🖱️ui/…/🗂️WindowChrome/🟦️.tsx` (24), `…/🧩️vscode/⚙️data-models/🟦️.ts` (22).
- Decision: whether field/member docs count as docstrings.
- Evidence: `evidence/item6-docstrings-without-emoji-touched.txt`.

## 7. Runtime external dependencies in framework `package.json` `dependencies`

46 non-`workspace:` entries in 11 packages:

- `ui-react` (15): `@dnd-kit/{core,sortable,utilities}`, `@react-three/{drei,fiber}`, `@xyflow/react`, `i18next`, `pdfjs-dist`, `react`, `react-dom`, `react-i18next`, `react-resizable-panels`, `three`, `three-mesh-bvh`, `xstate`
- `repo-coordinator` (5): `next`, `react`, `react-dom`, `pg`, `pg-boss`
- `infinite-world-r3f` (4): `@react-three/drei`, `@react-three/fiber`, `react`, `react-dom`
- `framework-renderer-react` (4): `@react-three/fiber`, `katex`, `react`, `react-dom`
- `presentation-react` (4): `pdfjs-dist`, `react`, `react-dom`, `reveal.js`
- `nx-bootstrap-tools` (4): `nx`, `@nx/js`, `@nxlv/python`, `typescript`
- `print` (3): `@napi-rs/canvas`, `pdfjs-dist`, `sharp`
- `canvas-react-renderer` (2), `course-react` (2): `react`, `react-dom`
- `ui-styling` (2): `tailwindcss`, `@tailwindcss/typography`
- `pets-react` (1): `react`

Decision: UI/3D/PDF/DB libraries are inherently external, so the rule needs a policy. Build-time tools (`typescript`, `nx`, `tailwindcss`, `@nx/js`) may belong in `devDependencies`. That edit touches `package.json`, which is owned by the deps owner.

Evidence: `evidence/item7-package-dependencies.tsv`.

## 8. Tracked generated and stale files

- 34 hash-named tracked lockfile copies (`.xxxxxxx`, ~650 KB each, `lockfileVersion 1`): root 3 (`.333ee2a`, `.3a0f1e4`, `.48ebeba`), `♻️mit-bestand` 3, `✏️s` 5, `🌎️hub` 11, `🎓️teaching` 3, `🏢️semio-tech` 9. Source of the copies not traced. Agent-fixable (remove, then stop the generator).
- Tracked binary: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/server` (Mach-O universal, 9.5 MB, mode 755). Agent-fixable.
- Generated bundles: `🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-bundles/{📞️out-callback,⚡️out-jspi-explicit}/jcoprobe.js` (`@generated by jco transpile`). Decision: keep as fixtures or regenerate on demand.
- Stale junk: `.DS_Store` ×2 under `✏️s/🔌️plugins/🗄️stdio/…/📸️snapshot/`; `🌉️mcp/…/🔗️remote/🦀️.rs.orig`, `…/🔗️remote/🧪️tests/🔬️unit/🦀️.rs.orig`, `…/🧪️tests/🔐️authenticated-hub-workspace/🟦️.ts.orig`. Agent-fixable.
- Stray root files: `micro_msg.txt` (tracked). Agent-fixable.
- Ticket copies: 150 tracked ticket files byte-identical to tracked code files. Largest groups: `…/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-webbridges2-fullpipeline-out/scalefixture2/interfaces` (14), `…/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/wfc-bridge-inputs/proposed/✏️s` (5), `…/☀️11/…/🪓️monolith-split/spd-tools/old-pp` (3), `…/FIX-FLOW-EXTENSION-BIM-WASM-ARTIFACT-NAME/jco-out/interfaces` (4), `GET-CODEBASE-RUNNING-END-TO-END-AGAIN/{jsi-tools/manual,c*-tools/specs}` (~10). Modified copies are not measured; `🪓️monolith-split` alone holds 6,605 tracked source-like files. Agent-fixable after confirming ownership.
- Evidence: `evidence/item8-hash-named-tracked.txt`, `evidence/item8-ticket-copies-of-tracked-code.txt`.

## Needs a user decision

1. Whether `[DEBUG] ` evidence prints in tests stay (982 lines) or are replaced.
2. Bootstrap `.sh`/`.ps1`, git hooks, Windows `.ps1`, `🟨️.mjs` build modules, Python oracles: exempt or move into `📜️script.ts`.
3. `bun <path>/📜️script.ts` form versus bare `📜️script.ts`; keep or delete `repo:noop`.
4. Public re-export shims (`preview2-shim-patches`, `host-shim-source`, `export * as mainWindow`).
5. Runtime external deps policy for framework packages (46 entries), and which build-time tools move to devDependencies.
6. Whether field/member docs count as docstrings (item 6); whether Rust `// SAFETY:` comments are exempt (item 5).
7. Jcoprobe bundles: keep as fixtures or regenerate.

## Agent-fixable without a decision

- Item 3: convert 144 implicit targets to `repo:script` (project.json only).
- Item 1: remove 86 production `[DEBUG]` log lines in 36 files.
- Item 4: remove the trinity shim after checking callers; fix the `.sty` legacy definition and the polynomial alias.
- Item 8: remove the 34 lockfile copies, binary, `.orig`/`.DS_Store` junk, `micro_msg.txt`, `extension-host-build-7a5OFB/`, and ticket code copies. Each removal needs an index change by the coordinator since agents may not run modifying git commands.
- Items 5 and 6 are mechanical but large (~4,400 and ~1,400 edits). Schedule them as separate slices.

## Limits

- Heuristics only. Items 5 and 6 are line/regex counts; item 4 re-export detection is by line pattern.
- `git grep` and `git ls-tree` at HEAD only. Shell `grep` (ugrep) was not used for emoji paths.
- Test-directory classification uses path patterns (`🧪️`, `tests/`, `.test.`, `_test.`).
