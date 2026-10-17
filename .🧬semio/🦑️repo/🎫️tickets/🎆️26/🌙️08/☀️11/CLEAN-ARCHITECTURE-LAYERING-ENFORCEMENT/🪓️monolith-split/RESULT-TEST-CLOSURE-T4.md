# 🧪️ Result TEST-CLOSURE-T4 (slice `tc4`, 2026-10-17)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. `F` = this folder. Tools: `F/tc4-tools/`. Logs: `F/🗑️generated/tc4-*`. Sandboxes: `/Users/ueli/.cache/tc4/`.

## 0. Verdict

**DONE (static), UNCOMPILED.** All seven crates are readable by the canonical reader and inside the source gate (4000 lines / 32 files / 262,144 B, embedded bytes counted) for their test closure (`tc4-census-after2`): worst ratios hub-artifact-creation 0.58, hub-stdio 0.48 (was refused), hub-directory-laws 0.95, hub-directory-backend-laws 0.44, procedural-2d-io-sqlite 0.28, procedural-3d-io-sqlite 0.28, composition-laws 0.84 (was refused, then 5.53 once readable). A full-repo census with the fixed reader (`tc4-census-full.log`, 2,842 crates) shows 0 reader-refused crates (the audit had 9). `COMPILE ADMITTED` is not in `📓️FLEET.md`; nothing was compiled. Open: the composition `sequence-runtime` suite hash in the ownership fixture no longer matches because a peer rewrote `sequence-runtime` imports (not this slice); the new test targets and the shared-helper move are unverified by any compile (section 4).

## 1. Baseline (`tc4-census-baseline`, scoped copy of `cgc7-census.ts`: `F/tc4-tools/census.sh <tag>`, `tc4-cgc7.ts`, `ONLY=` the seven crates)

| crate | test closure before this slice's edits |
|---|---|
| semio-hub-artifact-creation | readable already: 12 files / 151,283 B / 1,092 lines (0.58) |
| semio-hub-stdio | REFUSED: `Missing Rust source input: 🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json` (target `native_openable_provider`) |
| semio-hub-directory-laws | readable already: 18 files / 250,014 B / 2,653 lines (0.95) |
| semio-hub-directory-backend-laws | readable already: 7 files / 115,037 B / 1,099 lines (0.44) |
| semio-s-artifact-procedural-2d-io-sqlite | readable already: 9 files / 61,768 B / 164 lines (0.28) |
| semio-s-artifact-procedural-3d-io-sqlite | readable already: 9 files / 58,256 B / 164 lines (0.28) |
| semio-s-composition-laws | REFUSED: `Missing or ambiguous Rust module: laws` (target `generation3d-app-laws`) |

Five of the seven were already readable and inside the gate when this slice started (a peer fixed their inputs after the audit: the audit row for them quoted moved `🧫️fixtures` inputs). Nothing was edited for them.

## 2. Per crate

### semio-hub-stdio

Cause: the fleet-wide move `<owner>/🧫️fixtures/` -> `<owner>/🧪️tests/🧫️fixtures/` left `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/📇️native-openable-provider/🦀️.rs` pointing at seven old fixture locations (old directories exist but are empty; the files live under `🧪️tests/🧫️fixtures`). Fix: 7 `include_str!` paths re-pointed (hub trusted-catalog compiled-dependencies, stdio catalog x4, publication, plugin-builder topic-contributions). Fixtures untouched. Checked with `tc4-tools/incl.py` (all include sites of the three test targets resolve).

### semio-s-composition-laws

Cause 1 (reader divergence from rustc): `generation3d/🦀️.rs` does `include!("🎮️editor-command-laws/🦀️.rs")` (and three siblings); the included files hold `#[path = ".."] mod x { #[path = "🏅️standards/…"] mod laws; }`. rustc resolves module mounts inside an `include!`d file relative to the INCLUDED file's directory (verified in a sandbox with `rustc +nightly-2026-07-20`: `couldn't find file sub/../x.rs`). The canonical reader (`readRustSourceClosureV1`) passed the INCLUDER's directory as the module base. Fix at the owner: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts` now passes `dirname(target)` for the included file, plus a language-agnostic closure case in `🧪️tests/🧫️fixtures/🔣️.json` ("an included file resolves its module mounts from its own directory"). `bun test` of that suite: 44 pass; negative control (old base) fails exactly the new case.
Cause 2 (moved fixtures): six `include_str!` sites (4 in generation3d editor laws, the gumball protocol twice) pointed at old `🧫️fixtures` locations of the flow/brep/mesh fixtures (now `🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧪️tests/🧫️fixtures/`) and `World3dHost/🧫️fixtures` (now `…/🧪️tests/🧫️fixtures/`). Re-pointed.
After both: reader accepts the closure: 106 files / 1,449,759 B (600,574 of them embedded fixture bytes) / 12,612 lines (5.53x over, binding bytes).

## 3. composition-laws: splitting the oversized target

Gate counts per cargo unit (`📥️descriptor` reads one closure per `target.src_path`), so the single integration target `generation3d-app-laws` was re-cut into six `[[test]]` targets in the same package (no companion crates needed; no `Cargo.lock`, members, build-owners or launch.json change):

| target (root file under `🧪️tests/🧊️generation3d/`) | files | bytes | lines | worst ratio |
|---|---:|---:|---:|---|
| `generation3d-app-laws-viewer` (`🔭️viewer-laws`) | 23 | 215,704 | 3,364 | 0.84 lines |
| `generation3d-app-laws-editor-modes` (`🪟️editor-mode-laws`) | 20 | 181,112 | 2,580 | 0.69 bytes |
| `generation3d-app-laws-editor-structure` (`🧭️editor-structure-laws`) | 18 | 206,334 | 3,138 | 0.79 bytes |
| `generation3d-app-laws-editor-commands` (`🎮️editor-command-laws`) | 24 | 191,195 | 3,018 | 0.75 lines |
| `generation3d-app-laws-editor-unit` (`🧮️editor-unit-laws`, new) | 14 | 211,585 | 2,956 | 0.81 bytes |
| `generation3d-app-laws-editor-surface` (`🕹️editor-surface-laws`, new) | 22 | 188,919 | 2,755 | 0.72 bytes |

What was done (all generated from the pre-split snapshot `tc4-tools/orig/` by `tc4-tools/restructure.py`, idempotent):
- The four `include!`d law fragments (`🔭️viewer-laws`, `🪟️editor-mode-laws`, `🧭️editor-structure-laws`, `🎮️editor-command-laws`) became crate roots; the old root `🧊️generation3d/🦀️.rs` became the shared support fragment every root `include!`s (projection read, gumball readers, serial lock, flow operators, brep extension, publication authority, and the new `runtime_fixture`). Law blocks moved verbatim between roots (`#[path = ".."]` wrappers unchanged, so every law keeps its module path).
- Embedded fixtures are read at run time (T1 precedent): all 46 `include_str!` sites (32 fixture files, 600,574 B; one is the 215,594 B editor source read by the fold-contract source-text law) now call `crate::runtime_fixture::text("<repo-relative path>")` (cached, `CARGO_MANIFEST_DIR` + `../../../../..`). 19 `const X: &str = include_str!(..)` became `fn x() -> &'static str` (plus the 2 uses of the shared `KEYBOARD_REACHABILITY_FIXTURE_JSON`). Fixture files are untouched (byte-identical; no fixture file appears in the diff).
- The nine identical 61-line `use` headers (`editor_domain` and eight structure blocks) now `include!` one shared `📥️editor-imports/🦀️.rs` (also reduces repeated code).
- `editor_laws` (the editor test support: `context`, `serial_execution`) no longer `include!`s its eight law groups from inside; each root mounts `editor_domain::editor_laws` either as the support alone or through its own mount file `<root>/📚️editor-laws/🦀️.rs` (support + the groups it owns), so module paths `editor_domain::editor_laws::<test>` are unchanged.
- Six helpers that several groups share were defined inside single groups (`mesh_data_from_json`, `aabb_of_positions`, `test_serial` from engine-compute; `node_move_operations_json`, `declared_actions` from command-surface; `node_eval_status` from extension-round-trip); they moved verbatim to `…/🔬️unit/🧰️shared-helpers/🦀️.rs`, included by the support. The scoped oracle found this need (unresolved `declared_actions`, `node_eval_status`, `node_move_operations_json`, `test_serial`, `crate::publication_authority`); `publication_authority` is now mounted by the shared fragment.
- Registry: `🧪️tests/🧫️fixtures/🔣️.json` re-pointed (57 -> 64 suites: the tree suite of the editor unit file became eight per-group suites, because the module-tree reader skips `include!` targets that sit outside the mounting file's directory; `target`, `targetPath`, `sha256` of every app-laws suite recomputed; laws unchanged, 285). `Cargo.toml`: one `[[test]]` replaced by six. `🧪️tests/🔬️ownership/🟦️.ts` and `📦️packages/🦀️rust/📜️script.ts`: new `inTargetFamily` so `--test generation3d-app-laws` (and every existing launch route / project target) selects the whole family `generation3d-app-laws-*`; a single slice is `--test generation3d-app-laws-<slice>`. `.vscode/launch.json` is unchanged for that reason.

## 4. Evidence (commands run)

- Census: `F/tc4-tools/census.sh baseline|r1..r5|after1|after2` (scoped copy of `cgc7-census.ts`, `ONLY=` the seven crates, native). Full repo: `tc4-cgc7.ts` without `ONLY` (9.5 min): 0 refused, 11 oversized lib, 1 oversized test-only (`semio-framework-plugin`, 380 files, not this slice).
- Reader: `bun test ./🧪️tests/🟦️.ts` in `native-build/📥️source`: 44 pass; with the old base the new case fails (`Missing or ambiguous Rust module: laws`). rustc cross-check in `/Users/ueli/.cache/tc4/inc` (module mount of an `include!`d file resolves against the included file's directory).
- Test names: `tc4-tools/names.ts` (module-path + `#[test]`/`#[async_test]`/`#[tokio::test]` walker, validated against the 285 declared laws: all found) over the old root with the snapshot vs the six new roots: 293 vs 293 entries, multisets identical (`tc4-names-old.txt`, `tc4-names-new.txt`).
- Ownership law: `tc4-tools/ownership-run.ts generation3d-app-laws`: ok (sha256 of every suite file, every law fn present, Cargo targets and `targetPath`, project.json `--test` selections); per-target law counts 61/44/35/51/42/52 = 285.
- rustfmt (`+nightly-2026-07-20`, `skip_children=true`) parse of the 44 generated/edited `.rs` files: 0 failing.
- Scoped differential `tc4-tools/oracle.py` (rustc nightly-2026-07-20, `--test`, extern-crate lines removed and external globs neutralised identically; baseline sandbox = snapshot of the unsplit target; output into `/Users/ueli/.cache/tc4/sb` only): baseline 789 distinct rows; the six roots 799. `oracle-diff.py`: 33 rows are in files that were created by the move (imports fragment, shared helpers, new roots) and repeat a message the baseline already had in the old location; 0 genuinely new rows after the helper and `publication_authority` fixes (before them: 6 real new rows).
- `ctc-scan.ts --only semio-s-composition-laws,semio-hub-stdio`: 0 hits, 0 missing mounts.
- Hub stdio: `tc4-tools/incl.py` resolves all include sites of its three test targets.
- Limits: the oracle cuts off before type checking (missing external crates), so type and method errors in the moved code are not covered; the split moves blocks verbatim and the only code changes are the include sites, six moved helper fns and `runtime_fixture`.

## 5. For the compile queue

```
cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- test -p semio-hub-stdio --no-run
cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- test -p semio-s-composition-laws --no-run
```
Check first: unused-item warnings (roots carry `#![allow(dead_code, unused_imports)]`), `runtime_fixture::text` panics only on a missing fixture, serial lock behaviour per binary (six processes instead of one; the lock is process-global state only). Run the whole family with `bun repo run @semio-tech/s-composition-laws-rs:test-generation3d-app-laws`.
Not this slice, noted: (1) the ownership fixture's `sequence-runtime` suite sha256 no longer matches (peer import sweep over `🧪️tests/🎬️sequence-runtime/*`), so `source-check` without `--test` fails there; (2) `🔬️ownership` imports `@semio-tech/framework-compiler-ts/syntax/module-tree` (a peer's alias sweep); (3) the module-tree reader (`📚️compiler/📖️syntax/🌳️module-tree`) has the same includer-directory base for `include!`d files that the source reader had; it only matters for `#[path]` mounts inside included files, and no tree suite of this crate depends on it any more.
