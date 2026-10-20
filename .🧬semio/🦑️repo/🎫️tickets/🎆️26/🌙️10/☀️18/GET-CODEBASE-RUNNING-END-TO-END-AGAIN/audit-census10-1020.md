# Audit Census 10 (2026-10-20): Crate Gate, Hand-Authored Census, Function Census, Path Length

Auditor `aud-census10`, read-only on source. No builds, no `bun repo`, no git mutations. Outputs only under `🗑️generated/aud-census10-tools/`.

Working tree at HEAD `fd14da55f18` (on top of it: today's late uncommitted edits). Baselines: `audit-gates-1019.md` (crate gate, 2026-10-19 18:0x), `fnsplit2-census.md` (function census, 2026-10-19), `audit-monolith-census-8.md` (census 8, HEAD).

## 0. Method and deviations

- Brief named `C/aud-census8-tools/census8-wt.py`; that file does not exist. The working-tree census script is `C/🗑️generated/aud-gates-tools/census8-wt.py`. I copied it to `🗑️generated/aud-census10-tools/` and ran `python3 -I` there. Output: `census8-wt.json`.
- Crate gate: `crate-gate8-disk.ts` copied to `🗑️generated/aud-census10-tools/` (the original writes its shards next to itself, into `aud-census8-tools/`). Run as 8 shards (`SHARD=0..7 NSHARD=8`) with `bun 1.3.14`. Each shard took about 11 min (baseline about 4.5 min), probably because of load from the parallel census runs. Outputs: `crates8.disk.0..7.json`, `gates10-merged.json`.
- Function census: `fnsplit2-tools/census.ts` run in place (read-only) with `--min 600` and `--min 300`. Outputs: `census-600.md/json`, `census-300.md/json`.
- Path length: `pathcheck10.py` (new, `python3 -I`) over `git ls-files -co --exclude-standard`, 186,361 entries. Measured UTF-16 units of the repo-relative path (limit 256) and UTF-8 bytes per component (limit 255). Output `pathcheck10.json`.
- New-crate crosscheck: `crosscheck-new.py` (copy of the 10-19 script) run with `python3 -I`. Output `new-crate-crosscheck.json` and `crosscheck10.stdout.txt`.
- Not run: `cargo metadata` (builds forbidden for auditors). The workspace membership verdicts below are from manifest text only.

## 1. Headline

| Measure | Baseline gates-1019 (prod / test) | Now (prod / test) | Delta |
|---|---:|---:|---:|
| Crate targets | 3453 | 3468 | +15 |
| Over 1.00 (refused by gate) | 0 / 8 | 0 / 1 | test -7 |
| 0.90 to 1.00 | 13 / 151 | 4 / 135 | prod -9, test -16 |
| Refused by reader | 0 / 12 | 0 / 0 | test -12 |
| Hand-authored files over 1,000 lines | 0 (wt) | 1 (wt) | +1 (new, see 3) |
| Generated files over 1,000 lines | 3 | 3 | 0 |
| Functions over 600 lines (outermost) | 13 (census 300 run) | 4 | -9 |
| Functions over 300 lines (outermost) | 297 | 279 | -18 |
| Paths over 256 UTF-16 units | 0 | 0 | 0 |
| Components over 255 UTF-8 bytes | not measured | 0 | n/a |

Crate gate: no crate in prod cfg is over the gate. No crate is refused in either cfg. One test-cfg crate is over the gate.

## 2. Crates over 1.00 (refused by gate)

| Crate | Cfg | Ratio (dim) | Manifest |
|---|---|---|---|
| `semio-framework-actor` | test | 1.073 (bytes) | `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml` |

Baseline had 8 test-cfg over-gate targets. Seven of them are now at or under 1.00. Among the 8 baseline crates, `semio-framework-plugin-window-config` and `teaching-proctor-base` are no longer over. The new over-gate crate is `semio-framework-actor`, which is not in the baseline list.

## 3. Crates 0.90 to 1.00 (band)

Prod cfg (4): `semio-s-artifact-gis-gismap-demo-asset` 0.930 (bytes); `semio-framework-plugin-decl-views` 0.912 (lines); `semio-framework-os-renderer-wgpu-shell-actions` 0.908 (lines); `semio-s-artifact-puzzle-5d-interaction` 0.908 (lines).

Test cfg (135). The highest are `semio-framework-ui-retained-tree-tree` 0.988 (lines), `semio-s-artifact-puzzle-2d-mutations` 0.987 (bytes), `semio-framework-ui-tui-core` 0.985 (lines), `semio-framework-plugin-decl-views` 0.985 (bytes), `semio-framework-value` 0.982 (bytes), `semio-framework-job` 0.982 (bytes). Full list in `gates10-merged.json` (`flagged`).

Many test-cfg band crates are dimension "files" at exactly 0.906 (29/32 files). This is a single shared constraint, not 100+ separate problems.

## 4. Crates refused by the reader

None (baseline 12 test cfg). Refusals resolved in the working tree.

## 5. Hand-authored files over 1,000 lines (working tree)

- Hand-authored over 1,000: **1** (census 8 at HEAD: 17; baseline 0 in the working tree).
- New: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🟦️.ts`, 1,046 lines, not in the census-7 list or the census-8 list. `build-owners.json` has no root covering it (owner `(no owner)` in the census script's prefix match).
- Generated over 1,000 (header rule): 3, unchanged (`🦀️.rs` 1,096 lines in `bim/.../snapshot/values`; two `jcoprobe.js` at 10,981 lines each).
- 16 of census 8's 17 are now at or under 1,000 lines. 41 of the 44 census-7 list entries are no longer over.

## 6. Functions over 600 and 300 lines (fnsplit2 census.ts)

Over 600 (outermost, 948 files over 600 lines scanned; 4 found):

| Lines | Kind | Function | File:line |
|---:|---|---|---|
| 899 | ts function | `planArtifactCratesOnce` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🧮️pack/🟦️.ts:36` |
| 894 | ts method | `run` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts:24` |
| 870 | ts function | `applyTaxonomyPlan` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🚀️apply/🟦️.ts:36` |
| 656 | ts method | `run` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts:16` |

Delta vs baseline (13 over 600 in the 300-run): gone since baseline are `registerGisDocumentOpenTests` (941), `useShellActions` (773), `plugin_exchange` (759), `createGisMapInferenceWorkerV1` (710), `loadPluginModule` (688), `createBrowserBundleTests` (669), `useApplyHostEffects` (663), and the callback at `hub/tests/two-client-document` (637). Still over: `planArtifactCratesOnce` (899), `applyTaxonomyPlan` (870), `verify run` (656), and `script.ts run` (890 to 894, grew by 4 lines).

Over 300 (outermost): 279 (baseline 297). 319 functions total (baseline 348). 4,258 files scanned (baseline 4,291).

## 7. Path length (≤ 256 UTF-16 units, components ≤ 255 UTF-8 bytes)

- 186,361 entries checked. 0 repo-relative paths over 256 UTF-16 units. 0 components over 255 UTF-8 bytes. 0 paths over 256 when prefixed with the macOS root.
- 6 entries are in the git index but missing from the worktree (deleted files), skipped.

## 8. New crates: workspace members and build-owners

104 new manifests (committed today or uncommitted, excluding `.🧬semio/`). Baseline was 92.

- Nearest workspace lists them as members: **82**.
- Excluded by the nested `🧰️framework/Cargo.toml` `exclude` list (line 412): **19**. The root `Cargo.toml` lists them as members (checked by grep on two samples: course and reactor-turn; the other 17 not individually grepped). Baseline said 14 were "not listed", so this is the same class of finding, now expressed as an exclude entry.
- Workspace-only stubs with no `[package]`: **3** (`✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node`, `…/🧱️material`, `✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw`). No build-owner entry is expected for these; they are not crates.
- Build-owners: 101 of 104 covered by an owner root; the 3 stubs have none. Every real crate has an owner.
- Not verified: `cargo metadata` was not run (builds forbidden). Whether cargo accepts the 19 excluded-but-root-listed crates is an open item.

## 9. Open items

1. `semio-framework-actor` is test-cfg over the gate (1.073, bytes). New since baseline.
2. `🏗️build/🧪️tests/🕸️graph/🟦️.ts` (1,046 lines) is new over 1,000 and has no build-owner root. Needs a split or an owner entry.
3. The 135 test-cfg crates at 0.90 to 1.00 are mostly `files` at 0.906 (29 of 32), so they share one constraint (the 32-file cap). Adding files to any of them moves it toward the gate.
4. Function census: `planArtifactCratesOnce` (899) and `applyTaxonomyPlan` (870) are unchanged since baseline (age 2,774+ min, not touched today). `script.ts run` grew to 894 and `verify run` is 656.
5. Nested framework workspace excludes 19 new crates. Confirm with `cargo metadata` in a permitted run.
6. Crate gate shard runtime was about 11 min per shard; this is only a note for the next run.
