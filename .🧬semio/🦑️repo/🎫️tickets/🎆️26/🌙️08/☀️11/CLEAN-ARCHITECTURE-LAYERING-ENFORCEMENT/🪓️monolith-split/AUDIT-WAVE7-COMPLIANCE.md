# 🧾️ Wave 7 Compliance Audit (aud7c)

Repo root: `/Users/ueli/Documents/semio`. Paths below are repo-relative. Full lists are in `🗑️generated/aud7c-*` (TSV/JSON/text files next to this report).

## Scope and method

- Changed paths: `git status --porcelain` gives 24,845 entries. Excluding ticket folders (`.🧬semio/`) leaves 19,505 source paths. 14,976 of those have an mtime of 2026-10-17 or later and form "today's fleet changes". Most of the rest is `M`/`D` churn from earlier waves.
- Added lines came from `git diff HEAD -U0` (streamed), plus `git grep` against the working tree. Untracked files came from `git ls-files --others --exclude-standard`.
- Caveats:
  - Many git calls printed `error: daemon terminated`. Output was still produced, but some counts may be incomplete.
  - Rename detection was skipped (`renameLimit` warning), so renames show as delete plus add.
  - Many TS/JS sources are single-line minified files. Line-level classification is approximate for those files.

## Counts per rule

| # | Rule | Violations | Notes |
|---|------|-----------:|-------|
| 1 | `[DEBUG]` temporary logs in source | 6 source sites in 5 files (new); 21 test log lines (new); 19 of those are inside one JSON fixture | 120 new hits are inside assertion/panic messages and are exempt |
| 2 | `pub use … as …` aliases and re-export barrels (compat) | 105 new `🪞️mirror` barrels (405 files with the "path-preserving" docstring); 41 `Form*` alias lines; 6 TS procedural alias exports; 12 test-only alias lines | Child wiring and designed facades are not counted |
| 3 | New files without a leading emoji docstring; duplicate sibling emoji | 94 files with no docstring; 29 files with a plain `//` comment instead of a docstring (24 with emoji, 5 without); 2 duplicate-emoji sibling groups | Of 1,967 new `.rs`/`.ts` files today, 1,844 are compliant |
| 4 | Comments inside function bodies | 194 indented added comment lines; 62 top-level; 1 trailing (sample of 200 changed files, 177 with added lines) | Sample-based; see findings for file:line |
| 5 | Non-canonical `cfg(any(test, …))`; self dev-dependency | 157 added non-canonical `cfg(any(test, feature = …))` lines in 71 files; 0 self dev-dependencies | 373 added lines use canonical `feature = "testkit"` |
| 6 | Paths over 256 chars; new folder names | 12 repo-relative paths over 256 chars (all in the ticket folder); 121 non-ticket paths over 240 UTF-8 bytes; 18 PascalCase folder names, 20 dotted version folders | Dotted versions follow an existing convention; PascalCase in `🧱️elements` also follows one |
| 7 | Untracked junk outside ticket folders | 4 items: `Users/` tree (50 files), `.%F0%9F%A7%ACsemio/` (empty dir tree), `temp/` (76 files, gitignored), `.tmp` (0 bytes, gitignored) | `🏟️sandbox` story folder is legitimate and not counted |
| 8 | Launch routes to missing targets; new crates with tests but no route | 41 distinct missing route paths in `.vscode/launch.json` (51 occurrences); 34 in `.vscode/🧩️launch.seed.jsonc`; 4 new crates with `tests/` and no route | Launch configs also reference ticket logs; those are excluded |

## Findings

### Rule 1: `[DEBUG]` logs left in source

1. `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:52` (new, HEAD has 0). `reportDrainDiagnosticV1` writes `"[DEBUG] Original drain "` to fd 2. Fix: delete the function and its call sites.
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts:137,139,286` (new). Three `[DEBUG]` emitters: `Current Tools role`, `Current Tools generation names`, and `console.error("[DEBUG] Dependency sync incomplete …")`. Fix: delete the three report functions and their calls.
3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📤️receipt/🟦️.ts:17` (new). `[DEBUG] Authorizer receipt refusal` emitter. Fix: delete it.
4. `…/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts:89` (new `Current Tools generation authentication` emitter; the file also has two pre-existing `[DEBUG]` hits). Fix: delete the new one.
5. `…/🔌️nx/📦️graph/🟦️.ts:212` (new, high severity). Production code parses log lines that start with `"[DEBUG] Fixed UDRW image "` and derives `roots` from them. A debug tag is being used as a data channel. Fix: have the producer return structured data through the result channel and remove the prefix coupling.
6. `…/🏗️build/🕸️graph/🦀️rust/🟦️.ts:54`. The `phase` helper contains an empty `if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1"){}` stub. Low severity. Fix: delete the stub.
7. Test sources (new `eprintln!("[DEBUG] …")`):
   - `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🧪️tests/🔬️unit/🦀️.rs:71`
   - `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🎹️facets/🧪️tests/🔬️conformance/🦀️.rs:20`
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/📋️child-group-owner.json:20,29,45,54,63,72,81,90,99,108,109,110,119,128,137,138,139,148,157` (19 `eprintln!` strings embedded in a JSON fixture).
   Fix: remove them, or move the diagnostics into assertion messages.

Not counted as fleet findings: `🧰️framework/🔨️modules/🏃️process/📜️script.ts:18` and the drain/rust-graph hits pre-date this wave (present in HEAD).

### Rule 2: aliases and compat re-export barrels

1. `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/📦️packages/🦀️rust/🪞️mirror/🦀️.rs` (and 104 other new `🪞️mirror/🦀️.rs` files). Its docstring says it is a "path-preserving" carve. It re-exports `pub use …::standards::v1::subsets::any::schema::*` and friends so that old module paths still resolve. This is a compat barrel. Fix: update callers to the canonical crate paths and delete the mirror modules. If the fleet keeps them as a designed facade, document that decision and limit it to crate roots.
2. Forms playbook aliases, e.g. `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/📦️packages/🦀️rust/🦀️.rs:3-5`, with `pub use crate::playbook::PlaybookBlock as FormQuestion;` and similar for `PlaybookStep`, `PlaybookExpr`, `PlaybookVectorField`, `PlaybookBlockOption`, `PLAYBOOK_BUILTIN_KINDS`. 41 alias lines across about 40 files. `FormQuestion` is used in 80 files, so the aliases are load-bearing. Fix: choose one name and rename the callers.
3. TS procedural aliases, e.g. `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (`ProceduralSnapshot as Generation2dArtifact`, `parseProceduralSnapshot as parseGeneration2dArtifact`). The same pattern appears in the `generation2d/…/📸️snapshot/🟦️.ts`, `🧊️generation3d/…/🧬️schema/🟦️.ts`, and `…/📸️snapshot/🟦️.ts` files. Fix: keep one canonical name and update the callers (181 and 278 files reference the aliases).
4. Test-only aliases (review):
   - `pub use semio_framework_plugin_testing_caller as original_caller;` in three `🧪️tests` / `🩹️patches` / `🪝️mounts` files.
   - `pub(crate) use semio_framework_io_sqlite_snapshot_testing_backing as snapshot_test_backing;` in five `🧿️semio/🧪️tests/🪶️sqlite/…` files.
   - `pub use semio_framework_os_kernel as __kernel;` and `pub use semio_framework_value as __value;` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs`.
   - `pub(crate) use crate::{wire_fixture as wire, job_close_fixture as job_close, no_state_fixture as no_state};` in `…/🔌️plugin/🧪️tests/🪟️surface-fixture/🦀️.rs`.
   Fix: use the real names at the call sites.

Not counted (designed facades or child wiring): `pub use semio_s_artifact_norm_*_io_text as text;` in the `🚪️io/🦀️.rs` files, `pub use semio_framework_3d_rigid as rigid;` and similar in `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/🦀️.rs`, `pub use semio_framework_os_directory_*` in `📇️directory/🦀️.rs`, and `export * as core from "@semio-tech/cad-engine-ts/kernel";` in `✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript/🟦️.ts`.

### Rule 3: docstrings and duplicate emoji

1. 94 new `.rs`/`.ts` files with no docstring at all. The first line is `import` or `use`. Examples: `🌎️hub/🧩️compositions/🗄️stdio/📥️execution/🟦️.ts` and `🌎️hub/🧪️testing/🎛️run/🟦️.ts`. Fix: add a `/** <emoji> … */` or `//!` / `///` docstring.
2. 29 new files whose first line is a plain `//` comment, not a docstring. Examples: `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/⚡️quick-and-binding-laws/🦀️.rs` and `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🎭️fixture-harness/🦀️.rs`. Fix: convert to a doc comment.
3. Duplicate sibling emoji: `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔓️mutable-copy` (`🟦️.test.ts` and `🟦️.ts`) and `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping` (`🟦️.ts` and `🦀️.rs`). The TS/RS pair is the multi-implementation pattern, so this is low severity.

Full list: `🗑️generated/aud7c-r3-today.tsv`.

### Rule 4: comments inside function bodies (sample)

Sample of 200 changed `.rs`/`.ts` files (seed 7; 177 had added lines). Totals: 194 indented added comment lines, 62 top-level, 1 trailing.

1. `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/👇️hover-and-placement/🟦️.ts`. Comment lines 55-56, 100-103, 134-135, 146, 156-159 (indented, inside a body), 166-172, and 326-327. Fix: move the rationale into the docstring of the enclosing definition or delete it.
2. `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧬️kernel-contract/🦀️.rs`. Lines 44 and 97-120 repeat `// 🚫️async: E1 pure inherent-impl helper …` eleven times, with `#region` markers at lines 96, 109, and 111. Fix: move the marker to a docstring above the `impl`, or delete it.
3. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/⚖️laws-and-rendering/🦀️.rs`. Seven indented comment lines. Fix: same as above.

Full examples: `🗑️generated/aud7c-r4-examples.tsv`. Sample list: `🗑️generated/aud7c-r4-sample.txt`.

### Rule 5: `cfg(any(test, …))` and self dev-dependencies

1. Non-canonical gates: 157 added lines in 71 files. By feature: `artifact-app-testing` 99, `testing` 46, `integration-fixtures` 7, `media-turn-testing` 2, plus one each for `not(target_arch = "wasm32")`, `unix`, and `all(test, unix`. Biggest files:
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🖥️native/🎭️actor/🦀️.rs` (18)
   - `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/⏯️fill-run/🦀️.rs` (14)
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` (10)

   Fix: use `#[cfg(any(test, feature = "testkit"))]`, add `testkit = []` to the owner's `[features]`, and have each consumer enable `features = ["testkit"]` in its `[dev-dependencies]`.
2. Plain `#[cfg(test)]` (1,786 added lines) is normal test-module gating and is not counted.
3. Self dev-dependencies (`path = "."`): none in tracked `Cargo.toml`, none added. Pass.

Full list: `🗑️generated/aud7c-r5-noncanon.tsv`.

### Rule 6: path length and folder names

1. Over 256 chars (repo-relative), all in the ticket folder. These are source copies under `monolith-split/r7c-tools/orig-seq-tests/…` and `orig-wri…`. Example: `…/r7c-tools/orig-seq-tests/✏️s/🔌️plugins/🎬️sequence/…/🧬️compiled/🧪️tests/🔬️unit/🦀️.rs` at 267 chars. Fix: delete the VCS-tracked source copies from the ticket folder and keep only paths and hashes. The rule forbids copying source into the ticket folder.
2. The wider ticket folder also holds many `*-tools/orig*` source snapshots (for example `r5b-tools` with 481 status entries, `tc2-tools` with 471, `r7c-tools` with 129). Not all are source copies. Fix: same as above.
3. 121 non-ticket paths exceed 240 UTF-8 bytes (executor cap). Most are deep `🗿️artifacts/…/🏅️standards/🔖️1/🪆️subsets/✳️any/…` nests. Fix: shorten the nesting.
4. New folder names not matching `<emoji>️<kebab>`:
   - 18 PascalCase folders under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/` (e.g. `⚙️EngineCanvas`, `🌐️World3dHost`, `🏛️ShellHost`). HEAD has 1,054 PascalCase entries in that folder, so this is an established convention. The executor rule text and the convention conflict; the team should decide.
   - 20 dotted version folders such as `🔖️1.0`, `🔖️25.2`. HEAD has about 1,284 paths under such folders, so this is an existing convention. Not counted.

Lists: `🗑️generated/aud7c-r6-badfolders.txt`.

### Rule 7: untracked junk outside ticket folders

1. `Users/` at repo root. A 50-file stray tree at `Users/ueli/Documents/semio/.🧬semio/…/🗑️generated/r6-backup-gisterrain*`, created by a backup that used an absolute path. Fix: delete it. Move any needed backup under the ticket's `🗑️generated` folder using a relative path.
2. `.%F0%9F%A7%ACsemio/` at repo root. A URL-encoded misnamed copy of `.🧬semio/`, containing only empty directories. Fix: delete it.
3. `temp/` at repo root. 76 files from 2026-10-07. Gitignored. Fix: delete it, or confirm it belongs to another tool.
4. `.tmp` at repo root. A 0-byte file. Gitignored. Fix: delete it.

Not junk: `…/📖️stories/🏟️sandbox/🟦️.tsx` is a legitimate Storybook story folder.

### Rule 8: launch routes

1. `.vscode/launch.json`: 41 distinct non-ticket route paths do not exist (51 occurrences). `.vscode/🧩️launch.seed.jsonc`: 34 distinct. Examples:
   - Config `🧪️test🧩️puzzle2d🏷️inverse🦀️syntax` points at `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-handles/🎮️prepare/🦀️.rs`.
   - Config `🧪️test📜️history🧳️member🗜️syntax🔮️` points at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`.
   - Config `🧪️test🎒️pack🏷️retained-symbols🟦️source` points at `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🏷️symbols/🎮️decode/🧪️tests/🟦️.ts`.
   - Config `🧪️test🚀️launch🧬️placement🔮️source` points at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🧪️tests/🧱️placement/🟦️.ts`.

   Fix: retarget each route to the moved file or remove it. The seed is the source of truth, so fix the seed generator as well.
   Full list: `🗑️generated/aud7c-r8-missing.tsv`.
2. Delta against HEAD: `launch.json` +393 / −13 config names; seed +262 / −12.
3. New crates with a `tests/` directory and no launch route (4):
   - `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/📦️targets/📦️packages/🦀️rust` (`semio-framework-3d-mesh-tests`)
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📑️command-page/📦️packages/🦀️rust` (`semio-framework-plugin-command-page-laws`)
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/⚖️laws/📦️packages/🦀️rust` (`semio-framework-plugin-child-group-wire-laws`)
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🪟️surface-carriers/📦️packages/🦀️rust` (`semio-framework-plugin-surface-carrier-laws`)

   Fix: add routes that follow the existing grouping and naming.
   List: `🗑️generated/aud7c-r8-unrouted-crates.tsv`.

## Generated outputs

All in `🗑️generated/` next to this report: `aud7c-status.txt`, `aud7c-src-today.tsv`, `aud7c-r1-*`, `aud7c-r2-reexports.tsv`, `aud7c-r3-*`, `aud7c-r4-*`, `aud7c-r5-*`, `aud7c-r6-badfolders.txt`, `aud7c-r8-*`, plus the scan scripts `aud7c-*.py`. Logs and TSVs are audit output only; nothing was edited in the source tree.
