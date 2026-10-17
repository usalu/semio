# Result SPLIT-HARDENING-7 (prefix `sh7`): fixes from the wave-7 census-4 split audit

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Input: `F/AUDIT-WAVE7-CENSUS4-SPLITS.md`. Tools: `F/sh7-tools/`. Logs: `F/🗑️generated/sh7-*` (nothing deleted). Every finding was re-checked before it was touched.

## 0. Verdict

**DONE, not compiled.** The compile mutex line `COMPILE ADMITTED` is not in `📓️FLEET.md`, so no `bun repo cargo` call was made (section 7 has the queue command). All checks that exist without a compiler ran and are reported with their real results.

| # | Finding | Audit right? | Result |
|---|---|---|---|
| 1 | Class 1: unpaired `//#region` in 9 package-1 children | yes, all 9 (the audit's list is exact) | 10 children made balanced (21 comment lines added, nothing else touched); package 15 "group": not a defect, see item 4 |
| 2a | Class 3: architect inference `rust` leaf reads root only | yes | `concat!` over root + 3 children; probe proves it, before/after |
| 2b | Class 3: energy mutations `typescript` leaf reads root only | yes (47 children, the audit says 48 because it counts the root) | `concat!` over root + 47 children; probe proves it, before/after |
| 2c | Class 3 risk: package 3 parser projection | **real**: 107 of 110 and 402 of 412 exported types are invisible to the projection | `SourceStore.declarations` added, 5 + 3 call sites re-pointed; probe 509 differing names to 0 |
| 3 | Package 11 sibling emoji clash | yes, but the audit named the wrong tree (`🏭️vdi3805`); it is `🏛️en1992` | folder renamed `📦️design-effects` to `🧮️design-effects` |
| 4 | Package 15 "target 4 missing" | not a defect | the coordinator moved it on purpose; RESULT-CENSUS4-PKG-15 corrected |
| 5 | Class 4: 133 glob `pub use child::*;` | not defects by the stated criterion | 0 cases of a module that exists only as an old import path; nothing changed |

## 1. Class 1: region markers (package 1; package 15)

**Verified.** The audit's nine files are the unpaired ones, and the cause is that `c4pcheck.py` compared region *names* per file and never checked nesting. The base files are themselves cross-crate unbalanced (the W4 carve split regions between crates: for example `🧵️interaction` has 2 opens and 10 closes at HEAD), so no pre-split file is a clean model for each child; the rule used is IDE folding (pairing by nesting, labels ignored).

**Tool.** `F/sh7-tools/regionfix.py plan|apply|verify` splices the root and its children in mount order (the `#[path]` / `include!` lines of the root, children from `c4p1-tools/plans.py`), pairs every marker with a stack, and for each **child** adds:
- a repeated `//#region <label>` right after the header (`use super::*;` for modules, the first `///` line for `include!` fragments) for every region that is open when the child begins, or whose close sits in the child with no open anywhere (the label of that close is used);
- a repeated `//#endregion <label>` at the end of the child for every region still open there, and for opens that are never closed anywhere in the group (those close in their own child, not spread over the following ones).

Roots are never edited (the three roots `🎛️host`, `🧩️components`, `🧱️scene` were already unbalanced at HEAD and stay so). The plan also prints a `ROOT-CROSS` note if a pair ran between a root and a child; there was none.

**Changed (10 files, `W` = `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world`; +21 comment lines in total):**

| File | Added |
|---|---|
| `W/🌉️bridge/🎞️scene-bridge` | `#endregion 🌉️World3dSceneBridge` |
| `W/🌉️bridge/🖌️overlays-and-gumball` (spanned, had no marker) | `#region` + `#endregion 🌉️World3dSceneBridge` around the child |
| `W/🌉️bridge/🎥️camera-and-retirement` | `#region 🧭️WorldCameraSync` (pairs with its existing close; the HEAD label mismatch `World3dSceneBridge` / `WorldCameraSync` stays in the base, each child is label-consistent) |
| `W/🗃️model/🗂️registries` | `#endregion World3dState` |
| `W/🗃️model/🔖️interaction-transaction` | `#endregion 🧵️WorldInteractionTransaction` |
| `W/🧩️components/🪄️placeholder-mesh-and-gumball` | 5 closes: `🧵️WorldInteractionTransaction`, `WorldMeshBuffers`, `PlaceholderMeshAuthority`, `MeshHelpers`, `WorldRelocateGesture` |
| `W/🧵️interaction/🌪️vortex-relocate-and-terrain` | 4 opens (`WorldRelocateGesture`, `Environment`, `Terrain`, `PlaceholderMeshAuthority`), innermost = first existing close |
| `W/🧵️interaction/📜️plan-and-publish` | 2 opens `🧵️WorldInteractionTransaction` |
| `W/🎛️host/🧪️tests/🔬️unit/🖼️reference-and-pick-lane-laws` | `#endregion 🧲️PickTargetLane` |
| `W/🎛️host/🧪️tests/🔭️unit-2/🥽️brush-environment-and-hover-laws` | 3 opens: `🔖️WorldInteractionVerbs`, `TerrainTests`, `GlbAssetTests` |

**Package 15 group: no defect.** The 6 markers that "were lost" belong to target 4, see item 4. The other six targets' HEAD snapshots (`c4p15-tools/orig/{1,2,3,5,6,7}.rs`) contain 0 region markers, and none of their 44 live files (6 roots, 38 children) has one (counted from `c4p15-installed.json`).

**Verification (real results):**
- `python3 sh7-tools/regionfix.py verify` ends `UNBALANCED CHILDREN: 0` (all 31 children); `regionfix.py plan` after the apply prints no pre/suf for any file (idempotent). Log `sh7-regions-apply.log`, `sh7-regions-verify.log`.
- Before/after content comparison of all 31 children (before copy in `🗑️generated/sh7-regions-before.json`): only the 10 files above differ, after removing region lines they are identical, and every added line is a `//#region` / `//#endregion` line (21).
- `python3 sh7-tools/group-lossless.py` (line multiset of root + children against the HEAD blob, `pub(super) ` normalised): `lost=0` for 7 of 9 groups; `bridge` and `model` each lose 1 doc line (`/// (✏️editor/⚙️engine/🧲️picking/🦀️.rs:1108)` and `:86`) because a **peer** re-pointed that path text to `…/🧲️picking/🎨️target-style/🦀️.rs`; not caused here (my diff adds region lines only). Log `sh7-group-lossless.log`.
- `rustfmt +nightly-2026-07-20 --edition 2021 --check --config-path rustfmt.toml --config skip_children=true` on the 10 files, before copy against live: 127 hunks before, 127 after (no parse error; formatting debt unchanged, per file equal).
- `bun c4p1-tools/c4p1-tree.ts`: 32 files resolve over the seven roots, the four source-law assertions give the same answers as before.
- Scoped differential, world family, `POS_PREFIX=sh7w python3 pos.py run families/c4p1.json --cfgs lib,test --label s1` (log `sh7-pos-world.log`): lib 3071 = 3071 errors, NEW 1 / GONE 1; test 3560 = 3560, NEW 1 / GONE 1; privacy, trait-scope, resolve, type, other all 0; canary `E0308 E0616 E0624` reached on both sides. Identical to the CENSUS4-PKG-1 `a2` run (the NEW/GONE pair is the stub-flagged `E0308` re-key).

## 2. Class 3: source-text readers

### 2.1 Architect inference (package 9)

`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:90`: `rust: include_str!("🦀️.rs")` is now `rust: concat!(include_str!("🦀️.rs"), include_str!("🔀️adjacency-and-validation/🦀️.rs"), include_str!("🎁️outputs-and-reports/🦀️.rs"), include_str!("📊️status-search-and-analysis/🦀️.rs"))`, the three children in mount order. The test mounts (`🧪️tests/…`) stay out, as at HEAD.

**Probe** `python3 sh7-tools/reader-probe.py <descriptor> rust HEAD:<descriptor> 'pub fn build_report(' 'pub fn run_analysis(' 'pub fn validate_plugin('` resolves the `include_str!` list like rustc and compares with the HEAD monolith:
- before (`--index`, the state of the cut): 1 file, 209 lines; all 3 needles **MISSING**; 1,699 monolith lines missing (`sh7-probe-architect-before.log`).
- after: 4 files, 2,030 lines; all 3 needles FOUND; 1 monolith line missing, and that is the edited descriptor line itself (0 non-wiring lines lost; `sh7-probe-architect-after.log`).

### 2.2 Energy mutations (package 14)

`✏️s/🔌️plugins/🔋️energy/…/🧬️schema/🦀️.rs:140`: `mutations.typescript` is now a multi-line `concat!` of `🧬️mutations/🟦️.ts` and the 47 group leaves `🧬️mutations/<group>/🟦️.ts`, in the order of the root's `import type` lines (what `listModuleTree` returns). HEAD order cannot be reproduced: 8 groups interleave in the 2,548-line monolith (`sh7-tools/energy-head-order.json`). Same form as the BIM mutations descriptor.

**Probe** (`… mutations.typescript HEAD:<mutations 🟦️.ts> 'interface ChangeZoneVolume' 'interface ChangeBatteryCapacity' 'interface ChangeRunEndDay' 'interface ReplaceSurfaceVertices'`):
- before: 1 file, 357 lines; 4 of 4 needles MISSING; 1,942 monolith lines missing.
- after: 48 files, 2,598 lines; 4 of 4 FOUND; **0** monolith lines missing (`sh7-probe-energy-{before,after}.log`).

**Oracle** (family `sh7-tools/families/sh7energy.json`, `POS_PREFIX=sh7e … --label s1`, log `sh7-pos-energy.log`): lib 41 = 41 NEW 0, test 45 = 45 NEW 0, all categories 0, canary reached. **Positive control** (`--label s1mut`, sandbox-only `--mutate` of the first `include_str!` to a missing file): lib NEW 1, `couldn't read …` at line 142 (`sh7-pos-energy-mutate.log`), so the oracle does resolve these `include_str!` paths.

### 2.3 Package 3 parser projection: the risk is real, fixed

`schema parsers` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/`) reads each TypeScript leaf as one file through `SourceStore.read` and asks it `declares(name)`, `declaredOptionalFields`, `declaredTypes`, `aliasText`. After CENSUS4-PKG-3 the types of the architect schema root (750 lines) and diff root (1,012 lines) live in children; all 72 + 136 parsers and the `//#region 🚪️Parsers` stayed in the roots. Probe `sh7-tools/parser-probe.ts` (bun) compared the projection's view with the HEAD monolith:
- root file only (what `read` returns): `declares`/optional fields/declared types differ for **107 of 110** module exports and **402 of 412** diff exports (509 names in total);
- `SourceStore.declarations`: **0** differing names (`sh7-parser-probe.log`).

Practical effect today was nil (`check` stayed `fresh`, since all 72 + 136 parsers exist), but the first new export or drift regeneration would have misjudged a type as missing. **Fix** (three files, +31/−9 lines): `🧾️parser-render/🟦️.ts` (`SourceReader.declarations(path)`, `Context.declared`, three `context.own` type lookups moved to it), `🗂️documents/🟦️.ts` (`SourceStore.declarations`: the leaf plus the `listModuleTree` children below its directory for `.ts`, cached, falls back to the leaf when the tree cannot be listed; `read`/`write` still address the root file, so a projection write can never put child text into the root), `🟦️.ts` (five type lookups: `declares` x2, `declaredOptionalFields` x3). Parser-defined/import/region queries keep reading the root, where the parsers live.

Checks run: `bun ./📜️script.ts schema parsers check --under ✏️s/🔌️plugins/🏛️architect` before and after: both `fresh: pending=0 dangling=0 helperless=0 drift=0 orphans=0 unowned=177` (logs `sh7-parsers-check-{before,after}.log`; the `--under` flag still walks the whole repo, about 70 s). `bun test ./🟦️.ts` in `…/🪞️parser-projection/🧪️tests/🎯️ajv-oracle`: 156 pass, 0 fail, 710 expects (`sh7-parser-tests.log`). `tsc` was not run on these files.

### 2.4 Oracle for the architect crate

Family `sh7-tools/families/sh7architect.json` (`semio-s-artifact-architect-program`, base `b84bf526c7b`), log `sh7-pos-architect.log`: lib 4643 = 4643, NEW 0 / GONE 0; test 7714 = 7714, NEW 1 / GONE 1 (a stub-flagged re-key of a missing fixture `include_str!` path of the sandbox, `🛠️graph-tool/🔣️.json`; categories all 0); canary reached.

## 3. Package 11 sibling emoji clash

The clash is in `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/`, not under `🏭️vdi3805` as the audit says: `📦️design-effects` next to `📦️packages` (siblings now: `⚓️anchor-evaluation`, `📋️member-evaluation`, `📦️packages`, `🧰️helpers-and-annex-parts`, `🧮️design-effects`). Changed: folder renamed to `🧮️design-effects` (`🧮️` = U+1F9EE U+FE0F, RGI, already a folder prefix across 1,215 tracked paths of the repo, unique among the siblings and among the closure's docstring emoji), `#[path = "🧮️design-effects/🦀️.rs"]` in `⚖️checks/🦀️.rs`, first docstring emoji of the child to `🧮️`. No taxonomy row names this folder (`git grep -F design-effects` over json/ts/toml/md finds nothing); `bun c4p11-tools/c4ptaxcheck.ts` resolves it as `crate-module-children` (`checked=1 unresolved=0`). `listModuleTree` of `⚖️checks/🦀️.rs` lists all four children. rustfmt hunks 7 = 7 (child) and unchanged for the root. Oracle (`sh7en1992.json`, `sh7-pos-en1992.log`): lib 813 = 813, test 1345 = 1345, NEW 0 / GONE 0, all categories 0, canary reached. The git index still has the old path as `A` (an `AD` plus an untracked folder in `git status`); I did not touch the index.

A wider sibling scan over every added folder of the working tree (683 parent folders, `sh7-added-files.txt`) finds 22 other parents with a repeated sibling emoji. All lie outside the 11 audited packages (test folders such as `🧪️tests`, `🧪️testing`, `📦️packages/🟦️typescript`, `📦️units`, `🧬️contract`/`🧬️schema`); nothing was changed there.

## 4. Package 15 "target 4"

Confirmed as the coordinator's decision, not damage: `📓️FLEET.md` (entry "00:2x CENSUS4-PKG-15 done statically") says the coordinator moved the orphan `🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit` to `F/🗑️generated/coord-orphan-hub-dist-assets-test/`. The root and its five children are there (6 files, `ls`/`find` checked), `git status` shows the old root as `D`, and the older plugin-host shard copy that nothing mounts or reads is not restored. Changed: `RESULT-CENSUS4-PKG-15.md` got a correction block (6 roots + 38 children = 44 files live, not 7 + 43). The taxonomy member `🔬️dist-assets-bll2-7qi-unit` in `🔣️taxonomy.json` (line 14621) is now unused; I left that shared file alone.

## 5. Class 4: glob `pub use child::*;` (133 lines)

Checked as the brief asks: a defect would be a re-export into a module that exists only as an old import path. I parsed all 133 lines (41 parent files) and counted top-level items (`fn`, `struct`, `enum`, `const`, `static`, `type`, `trait`, `impl`, `macro_rules!`) in the parents. **22 parents now have 0 own items** (pure wiring); **all 22 had 18 to 99 items at HEAD** (`🗄️registers/*` 5 files, `⚖️checks` of din18599, din16798, `🏃️motion`, `🏭️reconstruction`, `🪣️fill`, the puzzle editor root, 8 fem engine modules, the hub `🪶️sqlite`, `🏛️trusted-catalog` and `🧾️descriptor` roots). They are the original modules split into children, so the module path stays its own namespace and the glob is the sanctioned wiring (`📓️EXECUTOR-RULES.md`), not a shim; the other 19 parents keep their own items. **0 defects, nothing re-pointed, nothing rewritten.** Log `sh7-class4.log`.

## 6. Not verified / limits

- Nothing was compiled (no `COMPILE ADMITTED`); the oracle runs are analysis-only `--emit=metadata -o /dev/null` runs with the stub layer, so borrowck, lint and linking are unchecked.
- `tsc` over the three parser-projection files was not run; `bun test` of the ajv oracle (156 pass) and the repo's own `schema parsers check` ran.
- The region repair is not a statement about the semantic correctness of the HEAD region labels; it only makes each child self-balanced.
- The en1992 oracle log may have been written twice: a background batch of this session also started the same family while I ran it by hand under a different prefix (`sh7n`). Both runs are deterministic and the log shows one coherent run (813/1345, NEW 0).

## 7. Compile queue (not run)

One launcher call under the fleet compile mutex, once admitted:
```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-architect-program -p semio-s-artifact-energy-model-schema -p semio-s-artifact-norm-en1992-checks -p semio-s-artifact-norm-en1992-inferences -p semio-framework-os-infinite-world-model -p semio-framework-os-infinite-world-components -p semio-framework-os-infinite-world-bridge -p semio-framework-os-infinite-world-interaction -p semio-framework-os-infinite-world-host --lib --keep-going --message-format=short
```
then the same with `--tests`. The `concat!(include_str!(...))` form compiles only with a real `rustc`; the oracle already reads every path.

## 8. Files

- Edited: the 10 package-1 children of section 1; `✏️s/🔌️plugins/🏛️architect/…/💡️inferences/🦀️.rs`; `✏️s/🔌️plugins/🔋️energy/…/🧬️schema/🦀️.rs`; `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/…/⚖️checks/🦀️.rs`; the three parser-projection files of section 2.3.
- Renamed: `…/🏛️en1992/…/⚖️checks/📦️design-effects/🦀️.rs` to `…/🧮️design-effects/🦀️.rs` (plus the docstring emoji).
- Result files corrected: `RESULT-CENSUS4-PKG-1.md`, `-3.md`, `-9.md`, `-11.md`, `-14.md`, `-15.md` (one short correction block each).
- Tools kept: `F/sh7-tools/{regionfix.py, reader-probe.py, parser-probe.ts, group-lossless.py, energy-head-order.json, families/*.json}`. Logs: `F/🗑️generated/sh7-*` and the oracle outputs `sh7a-*`, `sh7e-*`, `sh7n-*`, `sh7w-*`.
- No `[DEBUG]` line was added anywhere.
