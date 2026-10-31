# 🧭️ 1030 taxaudit: manifest registration and taxonomy audit

STATUS: YELLOW. Read-only audit, no source file edited. Four inconsistency groups need fixes (3 registry gaps, 1 stale TS test path, 1 stale taxonomy entry, 1 stale owner entry).

## Scope and method
- Input: `git status --porcelain -uall` (read-only), 48 manifest lines: 1 `D`, 4 `A`, 1 `??`, 1 `AM`, 2 `MM`, 13 `M`, 26 ` M` (parsed, saved to `🗑️generated/1030/taxaudit-status-manifests.txt`).
- Plus crates named in the brief that are tracked but not in the status list (`energy mut-membership-tests`, `fem 2d/3d snapshot-tests`), checked for the same registrations.
- Workspace resolution: effective workspace = own `[workspace]` or the nearest ancestor workspace that lists the crate in `members` and not in `exclude` (Cargo semantics, tomllib on Python 3.12).
- Tools (not part of the repo): `C/1030-tools/taxaudit/audit2.py`, `audit3.py`, `audit_manifests.py`. Outputs: `C/🗑️generated/1030/taxaudit-audit2.tsv`, `taxaudit-audit3.tsv`, `taxaudit-manifests-table.tsv`.
- Not run: `repo verify taxonomy report`. The 1028 gate log (`🗑️generated/1028/taxaudit-verify2.log`) is from 10/28 and covers the whole repo, so it does not answer for 1030. Allowlist coverage below is static only.

## Findings (fix list)

### F1. Stale reference to deleted crate `📬️publication-authority` (deleted manifest and `.rs`)
1. `🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts:66`: the array passed to `read(path)` still contains `"./../../../../../../✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/📬️publication-authority/🦀️.rs"`. The file is deleted (`git status`: ` D …/📬️publication-authority/🦀️.rs`), so the test throws ENOENT. Fix: remove that one array element; keep `…/🏠️host/🦀️.rs`.
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:17934`: `"📬️publication-authority",` in `semanticDirectoryMemberKinds."crate-module-children".memberNames`. Fix: delete this line. No other `publication-authority` process3d reference remains in Cargo.toml, Cargo.lock, members, excludes, or Rust source (`git grep`).
- Checked clean: no workspace member or exclude entry, no `Cargo.lock` entry, no `📋️build-owners.json` entry for the deleted package `semio-s-artifact-process-process3d-host-publication-authority`.

### F2. Missing `📋️build-owners.json` owner entries (Rust crates with a `Cargo.toml` have no owner root)
Owners carry one entry per crate root with `manifest` = `Cargo.toml` and `packages: []`. These roots have none:
| Crate | Root (repo-relative) | Status |
|---|---|---|
| `semio-s-artifact-energy-model-editor-inspector` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔍️inspector/📦️packages/🦀️rust` | new (AM) |
| `semio-s-artifact-energy-model-editor-mounted-tests` | `…/✏️editor/🧪️tests/🔬️mounted-bestest/📦️packages/🦀️rust` | new (??) |
| `semio-s-artifact-energy-model-snapshot-tests` | `…/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/📦️packages/🦀️rust` | modified (M) |
| `semio-s-artifact-energy-model-mut-membership-tests` | `…/🧬️schema/🧬️mutations/🧪️tests/🔬️membership/📦️packages/🦀️rust` | tracked, not in status |
| `semio-s-artifact-fem-2d-snapshot-tests` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/📦️packages/🦀️rust` | tracked, not in status |
| `semio-s-artifact-fem-3d-snapshot-tests` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/📦️packages/🦀️rust` | tracked, not in status |

Fix: regenerate `📋️build-owners.json` with its generator. If that is not possible, insert entries in name order (the file is sorted by `name`), shape `{"name","root","manifest": "<root>/Cargo.toml","packages": []}`. The 6 entries above are the wave-1030 set. Entries for the energy `mut-*` crates should sit next to `semio-s-artifact-energy-model-mut-*` (around lines 13687-13717).

Pre-existing, outside wave 1030 (for the coordinator, not required for this wave): `staffage-dwg` (`🎓️teaching/…/🖊️dwg`), `semio-framework-io-binary-source` (`🧰️framework/🔨️modules/🚪️io/🔢️binary/📥️source`), `semio-framework-os-renderer-wgpu` (`…/🧊️wgpu`).

### F3. Stale owner entry (root does not exist on disk)
- `📋️build-owners.json:16981` `semio-s-artifact-procedural-3d-initialization`, root `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/🌱️initialization/📦️packages/🦀️rust`. No such manifest in HEAD or the working tree. Fix: delete the entry at line 16981 (its `…/🧊️generation3d/🔨️modules/🏠️host/🌱️initialization` root is absent). Pre-existing, not from wave 1030.

### F4. Cargo.lock: no inconsistency found
- All 42 live status manifests resolve to a package in their effective workspace's `Cargo.lock` (`IN-LOCK`). The 6 workspace-root/virtual manifests have no `package` section and need no lock entry.
- The untracked `mounted-tests` crate and the tracked `mut-membership-tests` and `snapshot-tests` crates appear in `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.lock`. The `fem 2d/3d snapshot-tests` crates appear in their fem workspace lock.
- `🌎️hub/Cargo.lock:34925/35061` lists `semio-s-artifact-energy-model-editor-inspector` through the energy editor path. Consistent with `…/✏️editor/Cargo.toml` depending on it.

### F5. Workspace membership: no inconsistency found
- Every live manifest is a member of its effective workspace (root `Cargo.toml` for `🏛️/🛍️products` plugin-os crates, `🧰️framework` for `🔨️modules`, `✏️s` and the per-artifact workspaces for plugin crates). Standards crates with their own workspace are covered by the nearest ancestor.
- Every literal `members` entry of the 14 workspace roots resolves to an existing `Cargo.toml` (0 missing).
- All `workspace = true` dependencies of the 42 live manifests exist in the owning `[workspace.dependencies]`. All path dependencies and `[lib]`/`[[bin]]`/`[[test]]` paths resolve on disk.

### F6. `[[test]]` target names and paths: no inconsistency found
- No test target name in any live manifest is non-ASCII. Every `[[test]] path` exists. Checked by `audit2.tsv` and `audit3.tsv`.

### F7. Path length: no violation
- Tracked non-ticket paths: longest 197 chars / 210 UTF-16 units (limit 256). Zero tracked paths reach 256 units.
- New crate manifests: longest 151-169 UTF-16 units.
- On-disk untracked paths under `🧰️framework` reach 216 chars (repo-relative). Under 256, but only about 40 chars of margin for a deeper checkout root. Note only.

### F8. Taxonomy segment names: no slug violation found; allowlist coverage not verified
- All 82 directory segments on the new/changed manifest paths satisfy `testCaseSlugPattern` (`^[a-z0-9]+(?:-[a-z0-9]+)*$`) after the leading emoji.
- 21 of those segments are not literal in `🔣️taxonomy.json` (e.g. `🔍️inspector`, `🔬️mounted-bestest`, `⏱️geometry-phased-job-support`, `🩹️mesh-repair`). Most are governed by structure rules (`mutationDirectoryPattern`, `testCaseSlugPattern`, editor/viewer child dirs), not by an allowlist. Whether the gate accepts `🔍️inspector` under `✏️editor` and `🔬️mounted-bestest` under `🧪️tests` is not verified. Recommended: run `repo verify taxonomy report --scopes-from <file>` on the new dirs in the coordinator lane.
- Same-emoji siblings (e.g. `🔬️laws` and `🔬️unit`, `🧮️reduce-and-helpers` and `🧮️native-extent`) are common across the existing tree, so they are not treated as violations.

## Fix summary for the coordinator
1. Edit `🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts:66`: remove the `📬️publication-authority/🦀️.rs` array element.
2. Edit `🔣️taxonomy.json:17934`: remove the `"📬️publication-authority",` line.
3. Regenerate `📋️build-owners.json`, or add the 6 F2 entries and delete the F3 stale entry (line 16981).
4. Optional: run the taxonomy gate on the new dirs (F8).

## Not done (read-only lane)
- No edits, no builds, no cargo, no `git` modifying command. Temporary outputs are under `🗑️generated/1030/taxaudit-*`, scripts under `1030-tools/taxaudit/`.
