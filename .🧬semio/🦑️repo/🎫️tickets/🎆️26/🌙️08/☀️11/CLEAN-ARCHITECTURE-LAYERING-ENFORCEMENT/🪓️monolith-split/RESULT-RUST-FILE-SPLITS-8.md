# Result: Rust file splits 8 (slice RUST-FILE-SPLITS-8, prefix `rf8`)

Executor: Sonnet, foreground, no sub-agents, no ticket tool, no modifying git command. Folder `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/rf8-tools/` (kept). Logs and sandboxes: `F/🗑️generated/rf8-*`. Date 2026-10-16.

## 1. Verdict

All nine files are split, installed in the repo and verified statically. **Nothing was compiled**: the Wave 6 rule applies (the launcher is refused for headroom), so the evidence is the stub-layer full-resolution rustc differential (identical on every target and configuration), a lossless line multiset, a test-fn multiset, warning, parse, path, duplicate-mount and taxonomy scans, plus the re-pointed source-text readers. The real-check commands are in section 8. Section 7 lists the limits of the static evidence.

| Target | Lines before | Parent after | Children | Largest child (lines) |
| --- | ---: | ---: | ---: | --- |
| `📜️statutes` (`semio-framework-repo-statutes`) | 1,700 | 48 | 10 | 816 (`🔍️analyze`) |
| `⌨️cli/🧭️repo-cli/🚦️verbs` (`semio-framework-repo-cli-verbs`) | 1,480 | 41 | 10 | 227 (`📊️code-metric-verbs`) |
| `📡️events` (`semio-framework-repo-events`) | 1,464 | 67 | 7 | 483 (`🗄️store`) |
| `📊️metrics` (`semio-framework-repo-metrics`) | 1,451 | 58 | 9 | 309 (`📈️report`) |
| `🚚️move` (`semio-framework-repo-move`) | 1,320 | 67 | 13 | 253 (`🗣️language-helpers`) |
| generation3d editor unit laws (`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`) | 3,372 | 370 | 8 | 533 (`🧮️engine-compute`) |
| `🧊️generation3d` test target root | 1,368 | 78 | 4 | 604 (`🧭️editor-structure-laws`) |
| `🧩️generation3d-example-geometry` test target root | 1,235 | 35 | 6 | 465 (`🚚️mesh-delivery`) |
| spatial-kernel `🌊️session` (`semio-s-spatial-kernel-semio-session`) | 1,676 | 125 | 6 | 778 (`🧠️session-core`) |

73 new folders, 73 new files. Every child is at or below 816 lines (limit about 1,000). Longest new path: 211 UTF-8 bytes (limit 240). Public paths are unchanged: every item a caller names stays reachable at the old crate path through an explicit `pub use`; no caller changed. The unit parent keeps `context` (334 lines), the shared test context that more than 100 other test files name by path.

Scope note: slice `sf3` has touched none of the three generation3d test files. Its tools (`sf3-tools/`) only edit plugin-artifact supports; `RESULT-SHARED-FILES-3.md` did not exist yet. The staged one-line doc edits in the unit file and in `🧪️tests/🔬️flow-operators/🦀️.rs` come from the TS preview-kernel split (a peer), not from sf3; they are in my baseline.

## 2. How each package was cut

Every `//#region` marker moved with its cluster (region counts per file are balanced). Docstring emoji are unique among siblings; folder emoji are all used as folder prefixes elsewhere in the repo (`rf8-folder-emojis.tsv`, built from `git ls-files`).

### Package 1 and package 3: new child modules (`#[path = "<folder>/🦀️.rs"] mod x;` plus explicit `use`)
Splitter: `rf8-tools/splitter.py` (the coverage-checked splitter of `frs-tools`, extended). Each original `use` is copied to the child that names it, a root name used by a child comes in through `use super::{…}`, and a child item another file names gets a `pub use` (public) or a private `use` (crate-internal) in the parent.

- **statutes** (10): `📚️catalog`, `🙈️ignore-directives`, `🔶️breach`, `📐️specification-text`, `🗂️sources`, `🔍️analyze`, `🩹️autofix`, `🔏️digest`, `🗜️deflate`, `🧷️breach-cache`: one child per region. `include_str!` of the catalog got one more `../`.
- **verbs** (10): one child per aggregate family: `🧰️verb-support`, `🔌️gateway-verbs` (`mcp`, `graphql`, `test`, `hook`), `🎫️ticket-verbs`, `🎯️planning-verbs` (todo, goal, contributor), `📁️artifact-verbs`, `🔎️query-verbs`, `📊️code-metric-verbs`, `⚖️statute-verbs`, `🧩️system-verbs`, `📜️history-verbs`. The file was a single region (`🚦️Verbs`), so the outer `🧭️RepoCli` and `🚦️Verbs` markers stay in the parent around the mounts. All 82 public handlers (82 before, 82 after) stay reachable as `semio_framework_repo_cli_verbs::<name>`.
- **events** (7): `📋️event-kinds` (kinds plus envelope), `🌨️payloads`, `📤️emit`, `🔏️sha256`, `🗄️store`, `📦️export`, `🔐️server-client`. The `🧪️Tests` region and the `tests` mount stay in the parent. Names only the unit tests need (`fs`, `Path`, `PathBuf`, `Write`, `Deserialize`, `STAGE_SCHEMA`, `Stage`) are `#[cfg(test)]`-gated in the parent.
- **metrics** (9): `🏷️vocabulary`, `📏️counting`, `🧩️numstat`, `📈️report`, `🕰️time`, `🔌️git-source`, `🧮️pipeline`, `📤️render`, `⏱️benchmark`.
- **move** (13): `🗄️workspace`, `🧾️plan`, `🎬️executor`, `🔤️casing`, `🗣️language-helpers`, `🔏️sealed`, `✏️rename`, `📁️folders`, `📄️files`, `📚️agents-docs`, `📑️sections`, `📥️integrate`, `🧲️extract`. The `🧪️Tests` region stays in the parent.
- **session** (6): `🧰️helpers`, `⚠️errors`, `🧊️tessellation` (types and the failed-envelope codec), `📤️media-export`, `🌉️generic-invoke`, `🧠️session-core` (`ShellAllocator`, `Session`, capture, state, port). The file has glob and compound imports, so the parent keeps all imports and every child starts with `use super::*;`. The three existing mounts (`construction`, `retained_membership`, `tessellation_admission`) and the two `#[cfg(test)]` laws plus `physical_session_custody` stay in the parent, which is where the ownership fixture expects them. Two orphan doc blocks (they document the next item in the original) travelled with that next item, otherwise rustc reports "doc comment doesn't document anything".

### Package 2: include fragments (`include!("<folder>/🦀️.rs");`), not child modules
Splitter: `rf8-tools/fragsplit.py`. **Decision:** the three generation3d files are law corpora whose exact cargo test names are pinned. `🧪️tests/🧫️fixtures/🔣️.json` lists every law as its full test path (`editor_domain::editor_laws::<fn>`, `preview_eval_tests_laws::laws::<fn>`), the composition script runs them with exact cargo filters, and the root's wrapper modules (`editor_domain`, `viewer_domain`) are named by 278 `crate::…` paths in other files. A child module would add a path segment and break all of it. A fragment is spliced back into the same module, so **every test path, visibility and `use` scope is unchanged and no item was widened**. Cost: the child is not a module, so it cannot carry `//!` (rustc rejects an inner doc in an included file with E0753). Its first line is an outer `///` doc (or a `//` comment when the first item is a `use`), starting with the unique emoji.

- **unit laws** (8 fragments): `🕹️command-surface`, `🪟️window-and-menu-laws`, `🧮️engine-compute`, `📏️surface-budget-and-retirement`, `🔁️extension-round-trip`, `📈️hot-path-and-fault-laws`, `🪄️mesh-component-laws`, `🚪️archive-and-brep-laws`. The parent keeps the header, `pub(crate) mod context`, `serial_execution` and the shared `use` lines. 76 test fns before and after.
- **generation3d root** (4 fragments): `🔭️viewer-laws`, `🪟️editor-mode-laws`, `🧭️editor-structure-laws`, `🎮️editor-command-laws`: the 56 `#[path = "."] mod …_laws { use super::*; … mod laws; }` wrappers. A nested `#[path]` in an included file is relative to the fragment's own folder (checked with a rustc experiment), so each wrapper's `#[path = "."]` became `#[path = ".."]` (56 lines). The root keeps `SnapshotRead`, the gumball helpers and the five `#[path]` mounts.
- **example geometry** (6 fragments): `📇️fixture`, `⏱️calibration`, `🧰️harness`, `🔬️examples`, `🌉️scene-bridge`, `🚚️mesh-delivery`. The parent keeps the docs, the imports, the includes and `geometry_session`. 18 test fns before and after.
- Relative `include_str!`/`include_bytes!` literals got one more `../` (4, 6 and 8 lines in root, geometry and unit).

### Visibility (child-module targets only)
A private item, field or inherent method moved into a child becomes `pub(super)` when its name is referenced in another file of the crate closure (token scan, deliberately conservative; the scan is gated by the owning type, which is why `BitReader.data` and friends stay private). Widened lines (each is one removed plus the same line with `pub(super)`): statutes 6, verbs 9, events 10, metrics 2, move 9, session 93 (including single-line structs such as `CachedMesh {pub(super) mesh, pub(super) valid}`), fragments 0. For session the scan over-widens because the type-check cannot confirm each use (section 7).

## 3. Source-text readers and fixtures re-pointed
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🏷️ownership/🟦️.ts`: the negative law (`no static KERNEL|MESH_CACHE|TESSELLATION_JOBS, no OnceLock`) now reads the whole module tree with `readModuleTreeSource`, so it covers the six new children (10 files, 2,201 lines) instead of the 125-line parent. The test-name group for `🦀️.rs` (the two `#[test]` fns) is unchanged because those laws stayed in the parent. Run: `sessionLaws()` passes, 19 exact native laws.
- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🔬️ownership/🟦️.ts`: a suite with `"tree": true` is hashed and law-checked over `readModuleTreeSource(path)` (the parent plus its `include!` fragments) instead of the single file.
- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧫️fixtures/🔣️.json`: `"tree": true` and the new tree `sha256` for the example-geometry suite (`6621461f…`) and the editor unit suite (`da6de14d…`). Law lists are unchanged (the test paths are unchanged). **The unit suite's old pin (`a03ab821…`) was already stale before my change** (it matched neither HEAD nor the live file).
- Checked, no change needed: no TS, JSON or Python file names the five package-1 files; `project.json` files of those crates have no source globs; the hub fixture `🌎️hub/…/🏷️type/🔣️.json` pins the session file as a historical "original input" (its sha is already different from the live file); `💥️extension-evaluate-fault/🟦️.ts` only mentions the unit path in a comment.
- `.vscode/launch.json` and `🧩️launch.seed.jsonc`: no new executable command, no change.

## 4. Verification

### 4.1 Lossless (live tree, `rf8-tools/lossless.py`, log `rf8-lossless-final.log`)
Stripped non-empty line multiset of the original versus all installed files of the target. Removed/added lines, all accounted for:

| Target | removed | added | explained by |
| --- | ---: | ---: | --- |
| statutes | 7 | 65 | 6 widened fns/fields, 1 re-based `include_str!`; added: 48 wiring, 10 docstrings |
| verbs | 13 | 93 | 9 widened fns, 4 moved `use` lines (replaced by child copies); added: 70 wiring, 4 continuation lines, 10 docstrings |
| events | 11 | 70 | 10 widened lines, 1 re-based `include_str!`; added: 39 wiring, 14 continuation lines of rustfmt-wrapped `use` lists, 7 docstrings |
| metrics | 2 | 73 | 2 widened items; added: 49 wiring, 13 continuation lines, 9 docstrings |
| move | 12 | 90 | 9 widened lines, 3 moved `use` lines; added: 64 wiring, 4 continuation lines, 13 docstrings |
| session | 93 | 131 | 93 widened lines; added: 19 wiring, 13 continuation lines, 6 docstrings |
| generation3d root | 56 | 64 | the 56 `#[path = "."]` lines became `#[path = ".."]`; added: 4 includes, 4 docstrings |
| example geometry | 19 | 31 | 19 re-based `include_str!` lines; added: 6 includes, 6 docs |
| unit laws | 12 | 28 | 12 re-based `include_str!` lines; added: 8 includes, 8 docs |

No other line was added or removed. Dedent was not used (nothing was lifted out of an inline module), and no file has a multi-line string literal (checked by the lexer).

### 4.2 Test-fn multiset (`rf8-tools/testnames.py`)
`#[test]`/`async_test` fn names, original versus parent plus children: unit 76 = 76, example geometry 18 = 18, session 2 = 2, every other target 0 = 0: **TESTNAMES OK**. The test **paths** are identical for the three fragment targets. For session the two laws stay in the parent, so their paths are identical too. No child module carries a test.

### 4.3 Stub-layer full-resolution differential (`rf8-tools/oracle.py`)
`rustc +nightly-2026-07-20 --emit=metadata` over a sandbox copy of the live tree versus a sandbox copy of the snapshot (same patches: globs from missing crates neutralised, `extern crate` lines dropped, a generated stub module for every external crate path, sandbox-only; nothing is written outside `F/🗑️generated/rf8-work` and the scratchpad). The compared key is the error code and message, the primary-span label and the module-placement-insensitive text (closure types, `.`/`..` path segments and module prefixes normalised). Results on the live tree after the final install:

| Target | Configurations | Errors baseline = current | Result |
| --- | --- | --- | --- |
| statutes | lib, `--test` | 106 = 106, 106 = 106 | IDENTICAL x2 |
| verbs | lib, `--test` | 790 = 790, 790 = 790 | IDENTICAL x2 |
| events | lib, `--test` | 92 = 92, 121 = 121 | IDENTICAL x2 |
| metrics | lib, `--test` | 75 = 75, 75 = 75 | IDENTICAL x2 |
| move | lib, `--test` | 129 = 129, 129 = 129 | IDENTICAL x2 |
| session | lib, `--test`, `wasm32-unknown-unknown` lib and `--test` | 70 = 70, 75 = 75, 70 = 70, 75 = 75 | IDENTICAL x4 |
| generation3d root plus unit laws (one crate, both installed) | `--test` | 901 = 901 | IDENTICAL |
| example geometry | `--test` | 184 = 184 | IDENTICAL |

The error counts are the stub layer's own noise (unresolved external macros, derives and glob names); the comparison is on the whole multiset, so a single new error shows. Logs: `rf8-run-live-pkg1-final.log`, `rf8-run-live-g3.log`; the unit laws also ran alone on the staged tree (`rf8-run-g3unit-stg.log`, 901 = 901).

Negative controls (mutating the current sandbox only): statutes, un-widening `contributor_line` gives `E0603`; verbs, un-widening `flag_or_arg` gives `E0603`; session, un-widening `CachedMesh.valid`, `begin_close` or `Session.state` gives **no** error, which shows the limit of the stub layer for session (section 7). For fragments the check is positive: a missing or mis-based include or `#[path]` target shows up as a new `couldn't read` error (and did in the first g3 run until the `/./` path normalisation was added).

### 4.4 Warnings (`oracle.py --warn`, native, `--test`, and wasm for session)
No new warning on statutes, events, metrics, move, session and the generation3d targets. One new warning on verbs, `unused import client_enum` in `🎫️ticket-verbs`, is a stub artefact: the only uses are inside `json!({…})`, which the stub does not expand (the original reports the same fn as unused). The real check closes it. Events lost its one baseline warning (an unused `Path`), because the test-only imports are now `#[cfg(test)]`.

### 4.5 rustfmt, parse, paths, duplicate mounts, taxonomy
- `rustc -Zunpretty=normal` (parses every `#[path]` module) accepts all parents and children: 0 errors for all eight lib roots (`rf8-parsecheck-final.log`); `rustfmt --check` reports 0 parse errors. The files were already not rustfmt-clean. Because `rustfmt` follows `mod` but not `include!`, the comparable number is the hunk count of each file with `skip_children=true` on the pinned nightly (`rf8-rustfmt-skipchildren.log`): original file → parent plus all children: statutes 18 → 18, verbs 10 → 10, events 27 → 27, metrics 6 → 6, move 6 → 6, unit laws 87 → 87, generation3d root 64 → 64, example geometry 24 → 24, session 31 → 28. So the split adds no formatting debt. Every `use` block I generated was laid out by rustfmt itself (the repo `rustfmt.toml`, private `use` before `pub use`, `super` first), and nothing else was reformatted. The include fragments are parsed by rustc during the differential (a parse error would be a new error).
- `rf8-tools/pathcheck.py` (`rf8-pathcheck-final.log`): every `#[path]`/`include!` target exists, each new file is reachable exactly once, the longest repo path is 211 bytes, no child exceeds 816 lines, `//#region`/`//#endregion` balance in every new file, every new file starts with an emoji doc or comment, no duplicate first emoji among siblings: **PATHCHECK OK**.
- `rf8-tools/dupmounts.py`: a file mounted more than once inside one crate closure: 0 before, 0 after: **0 new duplicate mounts**. The root closure grew from 61 to 73 mounted files (4 root plus 8 unit fragments).
- `rf8-tools/taxcheck.ts` (the repo's own `matchDirectoryKind` over the whole ancestor chain of all 73 new folders, `rf8-taxcheck-final.log`): 0 unresolved, 0 ambiguous. 52 resolve through `crate-module-children`, 18 through `members-of-tests`, 3 through `members-of-modules`. No taxonomy edit was needed.
- Module-tree reader (`readModuleTreeSource`/`listModuleTree`, the repo's own, which the readers above use): the generation3d root resolves 74 files with `tests: true` (62 before plus 4 root fragments plus 8 unit fragments; 5 with `tests: false`), and the session tree resolves 10 files and 2,201 lines.

### 4.6 TS checks run
- `sessionLaws()` (session ownership): passes.
- `testCompositionOwnership("generation3d-example-geometry")`: passes (hash, 18 laws, Cargo target).
- The editor unit suite: hash and all 75 law names match over the tree; the whole `generation3d-app-laws` target still fails in `testCompositionOwnership` because **6 other suites have stale sha pins** (section 7).

## 5. Files

New folders (73, each holds one `🦀️.rs`):
- statutes: `📚️catalog`, `🙈️ignore-directives`, `🔶️breach`, `📐️specification-text`, `🗂️sources`, `🔍️analyze`, `🩹️autofix`, `🔏️digest`, `🗜️deflate`, `🧷️breach-cache`
- verbs: `🧰️verb-support`, `🔌️gateway-verbs`, `🎫️ticket-verbs`, `🎯️planning-verbs`, `📁️artifact-verbs`, `🔎️query-verbs`, `📊️code-metric-verbs`, `⚖️statute-verbs`, `🧩️system-verbs`, `📜️history-verbs`
- events: `📋️event-kinds`, `🌨️payloads`, `📤️emit`, `🔏️sha256`, `🗄️store`, `📦️export`, `🔐️server-client`
- metrics: `🏷️vocabulary`, `📏️counting`, `🧩️numstat`, `📈️report`, `🕰️time`, `🔌️git-source`, `🧮️pipeline`, `📤️render`, `⏱️benchmark`
- move: `🗄️workspace`, `🧾️plan`, `🎬️executor`, `🔤️casing`, `🗣️language-helpers`, `🔏️sealed`, `✏️rename`, `📁️folders`, `📄️files`, `📚️agents-docs`, `📑️sections`, `📥️integrate`, `🧲️extract`
- unit laws (under `…/✏️editor/🧪️tests/🔬️unit/`): `🕹️command-surface`, `🪟️window-and-menu-laws`, `🧮️engine-compute`, `📏️surface-budget-and-retirement`, `🔁️extension-round-trip`, `📈️hot-path-and-fault-laws`, `🪄️mesh-component-laws`, `🚪️archive-and-brep-laws`
- generation3d root (under `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/`): `🔭️viewer-laws`, `🪟️editor-mode-laws`, `🧭️editor-structure-laws`, `🎮️editor-command-laws`
- example geometry: `📇️fixture`, `⏱️calibration`, `🧰️harness`, `🔬️examples`, `🌉️scene-bridge`, `🚚️mesh-delivery`
- session (under `🧠️semio/🌊️session/`): `🧰️helpers`, `⚠️errors`, `🧊️tessellation`, `📤️media-export`, `🌉️generic-invoke`, `🧠️session-core`

Rewritten parents: the nine target files. Edited elsewhere: the session ownership test, the composition ownership test and the composition law fixture (section 3).
Tools (kept): `F/rf8-tools/` (`splitter.py`, `fragsplit.py`, `plan_*.py`, `plan_common.py`, `frslib.py`, `rslex.py`, `stubgen.py`, `oracle.py`, `snapshot.py`, `install.py`, `lossless.py`, `testnames.py`, `pathcheck.py`, `dupmounts.py`, `folders.py`, `census.py`, `parsecheck.py`, `errdiff.py`, `taxcheck.ts`, `fixture_tree.ts`). Baseline snapshots, stages, sandboxes and logs: `F/🗑️generated/rf8-*`.

## 6. Decisions worth a second look
1. **Fragments instead of modules for package 2** (section 2). If the law fixture later stops pinning full test paths, the fragments can become `mod` children in one step.
2. **`glob_super` for session**: its import block is globs and compound `use {…}`; copying imports per child would not be faithful, so each child imports `super::*` and the parent keeps the whole block.
3. **Unit parent keeps `context`** (334 lines): it is the module path (`editor_laws::context`) the other 100+ test files import.

## 7. Limits, foreign findings, hand-offs
- **Session field/method privacy is not machine-proven.** The stub layer type-checks too little of that crate (70 baseline errors, error-typed receivers), so E0616/E0624 stay invisible, as the negative controls show. The cover is the conservative widening above (a private name referenced anywhere in the closure is widened) and the real check. On `E0603`, `E0616` or `E0624` in the real check, widen the named item to `pub(super)`; on `E0599` for a trait method, add the trait import to the child.
- **Six stale sha pins in `🧪️tests/🧫️fixtures/🔣️.json`, not mine.** The suites `🧵️preview-eval`, `👁️viewer/🔬️eval-chain`, `🎨️set-active-example`, `✏️editor/🔬️example-switch`, `✏️editor/🔬️tick-addressing` and `⏱️flow-eval-tick` no longer match their pinned files, so `source-check` and the `generation3d-app-laws` target fail until a peer re-pins them. Re-pin with the live sha of each file (or with `fixture_tree.ts` for a split suite).
- No real compile: the launcher is still refused for headroom, so type-level facts the stub layer cannot see (serde derives, trait coherence, macro-generated code, `json!` argument uses) are open.
- The new children carry no `🧲️Header` region (the repo's own `header_policy` would flag a missing header in each of them); the earlier fleet splits (async, number, grammar, channel) did the same, and Wave 5's rule asks for an emoji docstring first.
- The ownership fixture's `previousPath` checks, the hub type fixture and the composition `Cargo.toml` `[[test]]` paths are untouched.

## 8. Real checks for the compile queue
Acquire the fleet mutex first (`mkdir F/🗑️generated/cargo-mutex.lock`), one launcher call, release after. Package-1 crates are members of the root workspace; the session crate is a member of `✏️s/Cargo.toml`; the composition test crate is a member of `✏️s/🧑‍💻dev/🧩️composition/Cargo.toml`.

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-repo-statutes -p semio-framework-repo-events -p semio-framework-repo-metrics -p semio-framework-repo-move -p semio-framework-repo-cli-verbs --lib --tests --keep-going --message-format=short

cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-spatial-kernel-semio-session --lib --tests --keep-going --message-format=short --manifest-path ✏️s/Cargo.toml
  (and once more with --target wasm32-unknown-unknown --lib)

cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-composition-laws --tests --keep-going --message-format=short --manifest-path ✏️s/🧑‍💻dev/🧩️composition/Cargo.toml
```

Then the law runs: `bun ✏️s/🧑‍💻dev/🧩️composition/📦️packages/🦀️rust/📜️script.ts test --test generation3d-example-geometry` and `… test --test generation3d-app-laws` (after the six stale pins are re-pinned). The session ownership law: `sessionLaws()` from `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🏷️ownership/🟦️.ts` (passes today).
