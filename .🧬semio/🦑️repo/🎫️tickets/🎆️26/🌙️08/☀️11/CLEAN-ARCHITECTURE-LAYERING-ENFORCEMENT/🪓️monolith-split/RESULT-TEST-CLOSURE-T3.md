# 🧪️ Result TEST-CLOSURE-T3 (slice `tc3`, 2026-10-17)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. `F` = this folder. Tools: `F/tc3-tools/` (`census.sh <tag> [crates]` over `tc3-cgc7.ts` = scoped copy of `cgc7-census.ts` that also prints one `[TARGET]` row per lib/bin/lib-test/itest; `closure.ts` = per-input list of one target closure; `snap.py`/`restore.py` = pre-edit snapshots in `tc3-tools/orig/` with `index.tsv`; `rt_include.py` = include-site rewriter with sha256 log `rt_include.log.tsv`; `names.py` = test-name multiset; `proctor_carve.py`, `oracle_*.py`, `errdiff.py`, `verify.py`, `taxadd.py`, `unused.py`; copies of the ftc registration helpers `reg.py`, `mkcargo.py`, `useimports.py`). Logs and census outputs: `F/🗑️generated/tc3-*` (census tags `baseline`, `a1` .. `a12`, `final`). Touched files: `F/🗑️generated/tc3-touched.txt`.

## 0. Verdict

**DONE (static), UNCOMPILED.** All 11 target crates are inside the compiler source-boundary gate for their lib and for every test target (native cfg; census `final`: 15 crates, 0 lib-oversized, 0 test-oversized, 0 reader-refused). Nothing is compiled: `COMPILE ADMITTED` is not in `📓️FLEET.md`. Two crates sit at the edge and are called out as THIN: `semio-framework-3d-brep-boolean` (test closure at exactly 4,000 / 4,000 active lines, the gate refuses only above) and, before its split, `publication-mounted` (now 0.865). `semio-framework-plugin` (pi2) is untouched; `time-travel`, `child-group-wire-laws` and `publication-mounted` do not overlap pi2's test roots (checked in `pi2-tools/crate_map*.json` and `touched.json`). Scoped resolution differentials show 0 new local-resolution rows (one real defect was found by it and fixed, section 5).

## 1. Baseline (before the first edit, tag `baseline`; native; worst test target)

| crate | target | files | bytes | lines | ratio |
|---|---|---:|---:|---:|---:|
| semio-framework-expression | itest expression_conformance | 5 | 886,692 | 225 | 3.38 (876,880 embedded) |
| semio-framework-time-travel | lib-test | 15 | 472,244 | 2,003 | 1.80 (337,944 embedded) |
| semio-framework-plugin-child-group-wire-laws | itest child-group-wire-authority | 42 | 1,175,981 | 1,965 | 4.49 (934,110 embedded) |
| semio-framework-plugin-publication-mounted | lib-test | 26 | 283,026 | 2,381 | 1.08 |
| semio-framework-3d-brep-boolean | lib-test | 8 | 344,819 | 4,000 | 1.32 (128,128 embedded) |
| semio-framework-3d-mesh-tests | itest mesh-jobs | 6 | 339,178 | 779 | 1.29 (119,940 embedded) |
| semio-framework-3d-mesh-document | lib-test | 28 | 264,421 | 3,427 | 1.01 |
| semio-framework-os-kernel | itest sqlite_snapshot_native_admission | 31 | 288,289 | 2,225 | 1.10 |
| semio-framework-number | lib-test | 18 | 143,601 | 4,325 | 1.08 (lines) |
| teaching-proctor | **lib** / lib-test | 11 / 21 | 238,110 / 435,376 | 4,457 / 7,146 | **1.11 lib** / 1.79 |
| semio-framework-os-shell | lib-test | 0 (refused) | | | `Missing Rust source input: …/🖥️shell/🤖️generated/🟦️.ts` |

Note: `teaching-proctor` was lib-oversized (not only test-oversized), and `os-shell` had already been repointed by a peer when I re-measured (7 / 229,448 / 2,491, 0.88).

## 2. Approach

- **Embedded fixtures to run-time reads** (T1 precedent; fixture files untouched). Each rewritten `serde_json::from_str(include_str!(P)).unwrap()` became a helper call; the 127 sites rewritten by `rt_include.py` (publication-mounted, mesh-tests, mesh-document, os-kernel) are logged with the sha256 of the file they read (`tc3-tools/rt_include.log.tsv`); the 4 + 5 + 17 + 4 sites of brep-boolean, time-travel, child-group and expression were rewritten by one-off scripts of the same kind (anchors asserted); originals in `tc3-tools/orig/`. Helpers: `CARGO_MANIFEST_DIR` join (expression, child-group, publication-mounted, mesh-tests, mesh-document, time-travel, os-shell, brep-boolean inline); `crate::fixture_json(file!(), rel)` for os-kernel, whose test files are mounted by several test roots (the workspace root is the manifest ancestor that contains the `file!()` path, so a file resolves its fixtures relative to itself under any mount).
- **Split where embedding alone was not enough**: companion crate `semio-framework-number-tests` (number), companion crate `semio-framework-plugin-publication-mounted-tests` (publication-mounted receipt laws), and a three-crate carve of `teaching-proctor` (base / domain / main, tests stay in their modules, shared fixtures behind a `testkit` feature).
- **No cross-crate `cfg(test)` items**: the shared proctor fixtures are `#[cfg(any(test, feature = "testkit"))] #[doc(hidden)] pub mod fixtures`, enabled by the consumers' `[dev-dependencies]`. `F/ctc-tools/ctc-scan.ts --only <15 crates>` (`🗑️generated/tc3-ctc-*`): 25 hits, all `exact` and sanctioned (testkit gate + consumer dev-dependency), 0 unsanctioned. The only other gated records are the serde bridge of mesh-document (module-private, hidden).

## 3. Per crate: before and after (native test closure; census tag in parentheses)

| crate | before (files / bytes / lines, ratio) | after | what changed |
|---|---|---|---|
| semio-framework-expression | itest 5 / 886,692 / 225 (3.38) | 1 / 10,156 / 229 (0.06) (`a1`; lib-test 19 / 117,802 / 2,917, 0.73) | 4 fixtures (877 KB) read at run time via one `fixture(rel)` |
| semio-framework-time-travel | lib-test 15 / 472,244 / 2,003 (1.80) | 14 / 138,951 / 2,005 (0.53) (`a2`) | lifecycle-law (333 KB) read via `tests::law()`; the retirement tests call `super::super::tests::law()` |
| semio-framework-plugin-child-group-wire-laws | itest 42 / 1,175,981 / 1,965 (4.49) | 28 / 241,538 / 1,973 (0.92) (`a3`) | 17 sites via `crate::fixture_json(name)`; 859 KB semantic-text-wire corpus no longer compiled |
| semio-framework-plugin-publication-mounted | lib-test 26 / 283,026 / 2,381 (1.08) | 16 / 226,851 / 2,063 (0.865) (`a12`) + companion `…-publication-mounted-tests` lib-test 3 / 35,575 / 365 (0.14) | 23 fixture sites via `crate::fixture_json(rel)` (0.993 alone); receipt laws (33.5 KB) moved to the companion: the 7 `pub(crate)` receipt types, 3 methods, 2 source fields and 2 phase enums became `#[doc(hidden)] pub` |
| semio-framework-3d-brep-boolean | lib-test 8 / 344,819 / 4,000 (1.32) | 5 / 216,991 / 4,000 (1.00 lines) (`a5`) | 4 sites inline `std::fs` (no net line added); **lines AT the limit**: the committed `[DEBUG]` code (`debug_dump_face`, `debug_face_tessellation`, `KB1_*` env, about 45 lines) is in the count; any growth refuses, removing it gives 3,95x |
| semio-framework-3d-mesh-tests | mesh-jobs 6 / 339,178 / 779 (1.29) | 1 / 218,869 / 787 (0.84) (`a6`) | 43 sites via `fixture_json(rel)`; other targets unchanged (max 0.35) |
| semio-framework-3d-mesh-document | lib-test 28 / 264,421 / 3,427 (1.01) | 25 / 243,271 / 3,436 (0.93) (`a7`) | 15 sites via `crate::fixture_json(rel)` |
| semio-framework-os-kernel | itest 31 / 288,289 / 2,225 (1.10) | 16 / 241,783 / 2,235 (0.92) (`a8`) | 41 sites in 9 files via `crate::fixture_json(file!(), rel)`; the dsl `🛬️decoding` file (own test target in dsl-schema, 4.3 KB of fixtures) keeps its `include_str!` |
| semio-framework-os-shell | refused (missing input) | lib-test 6 / 221,144 / 2,493 (0.84) (`a9`) | see section 4 |
| semio-framework-number | lib-test 18 / 143,601 / 4,325 (1.08, lines) | lib-test 12 / 119,570 / 3,594 (0.90) (`a10`) + companion `semio-framework-number-tests` lib-test 13 / 25,089 / 759 (0.41) | unit laws of algebra, primes, rational, interval, modular, traits (701 lines) run from the companion against the public API; natural and integer laws stay (they read private `exact_div`, `KARATSUBA_THRESHOLD`, `mul_schoolbook_pub`) |
| teaching-proctor | **lib** 11 / 238,110 / 4,457 (1.11), lib-test 21 / 435,376 / 7,146 (1.79) | lib 3 / 72,101 / 1,375 (0.34), lib-test 5 / 125,802 / 2,074 (0.52), itests 0.03 and 0.50 (`a11`) | three crates, section 4; `teaching-proctor-base` lib 6 / 108,597 / 1,951 (0.49), lib-test 13 / 176,546 / 2,952 (0.74); `teaching-proctor-domain` lib 4 / 58,400 / 1,147 (0.29), lib-test 9 / 135,609 / 2,169 (0.54) |

wasm labels: all touched crates are native-only for these test closures; I measured native only (no `cfg(target_arch)` line was added or removed).

## 4. Details of the three structural changes

**os-shell.** The refusal was `include_str!("../../../🤖️generated/🟦️.ts")` in `🧬️schema/🧪️tests/🔬️unit/🦀️.rs`, a path under `🖥️shell` while the generated TypeScript mirror lives at `🧰️framework/🔨️modules/🐚️shell/🤖️generated/🟦️.ts` (gitignored `**/🤖️generated/`). A peer's uncommitted edit had already repointed the include, so the census read 0.88 here, but a clean checkout has no generated file and the reader would refuse again (and `typegen`, which writes the file, could not compile to create it). The test now reads the mirror at run time and panics with "run the `typegen` target first".

**number companion.** `🔢️number/🧪️tests/🥣️companion/` (root `🦀️.rs` with `#![cfg(test)]`, six wrappers `🧮️algebra`, `🔑️primes`, `➗️rational`, `📏️interval`, `🕰️modular`, `💍️traits`, each `pub use semio_framework_number::<module>::*;` + only the restated trait/type imports the rustc typecheck proved necessary + `include!` of the unchanged unit file; one import line in the algebra unit became `semio_framework_number::{Integer, ModInt, Rational}`), package `📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`. The six `// #region 🔖️Tests` blocks left the lib modules. Registered: `🧰️framework/Cargo.toml` member, `📋️build-owners.json` row `@semio-tech/framework-number-tests-rs`, six launch routes in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (series 993.029+ / 998.030+ / 999.030+ and 900.999+ / 925.101+ / 903.993+), `🧰️framework/Cargo.lock` entry.

**publication-mounted companion.** `…/🪟️mounted/🧪️tests/🥣️companion/` (root with `extern crate … as store/protocol`, its own `fixture_json`, wrapper `🧾️receipt/🦀️.rs` = `pub use …::mounted_child_group_receipt::*;` + 9 restated imports + `include!` of the unchanged receipt law file). Registered: root `Cargo.toml` member, owner `@semio-tech/plugin-publication-mounted-tests-rs`, six launch routes in both files, root `Cargo.lock` entry (the existing lock entry of the lib itself is stale since pi1; not refreshed).

**teaching-proctor.** The lib was 4,457 lines (over the gate by itself) and its unit tests mount per module, with private access (`outbox_cursor`, `limits`, `claim`, `Flight`, cli verbs), so the tests stay in their modules and the lib is cut along its dependency graph:
- `teaching-proctor-base` (`🔨️modules/🧱️base`): storage, catalog, config, crowd, presence.
- `teaching-proctor-domain` (`🔨️modules/🧠️domain`): actors, projections, queries (depends on base).
- `teaching-proctor` (unchanged package dir): instance, cli, the `proctor` binary and the two integration targets (depends on both).
Module files did not move (only the new lib roots `🧱️base/🦀️.rs`, `🧠️domain/🦀️.rs` and the package manifests are new). `crate::<module>` paths into the other crate became `teaching_proctor_base::<module>` / `teaching_proctor_domain::<module>` (mechanical, `proctor_carve.py`); the integration tests and the cli import from the owning crate (no re-exports). The cross-module test helpers (`catalog::tests::{fixture, fixture_path}`, `storage::tests::{scratch, Scratch}`, `actors::tests::{id, ADA, BOB, TENANT, envelope, admission, perfect}`, `projections::tests::{bus, submit, answer, play, command_id}`) moved verbatim into `🧰️fixtures` modules of their owner (`#[cfg(any(test, feature = "testkit"))]`, `pub`), `testkit` features: base `testkit = []`, domain `testkit = ["teaching-proctor-base/testkit"]`, enabled through `[dev-dependencies]` of domain and main. Test-name multiset: 115 tests, 115 distinct, identical before and after (`tc3-proctor-names-*.tsv`). `fixture_path` now joins `../../../../🧫️fixtures/📚️catalog/🔣️.json` from the base manifest dir. Registered: `🎓️teaching/Cargo.toml` members, `🎓️teaching/Cargo.lock` (two package entries, the proctor entry rewritten: axum, async, quiz, server, serde_json, base, domain, tokio, tungstenite, ureq; `rusqlite`, `replication`, `hash`, `dispatch-macros`, `serde`, `sha2` moved to the crates that use them), `📋️build-owners.json` rows `teaching-proctor-base` and `teaching-proctor-domain` (manifest = Cargo.toml, like `semio-framework-number`), `🏗️bootstrap/🟦️.ts` `PROCTOR_LIBRARY_PACKAGES`, `📦️packages/🦀️rust/📜️script.ts` test verb runs `[PROCTOR_PACKAGE, ...PROCTOR_LIBRARY_PACKAGES]`; no launch routes (the proctor routes run the script verbs). Taxonomy: `taxadd.py` added `🧠️domain`, `🧱️base` to `semanticDirectoryMemberKinds.members-of-modules` and `🧰️fixtures` to `members-of-members-of-modules`; `taxcheck.ts` over all new directories: 0 unresolved (number companion and publication companion directories resolved without a taxonomy change).

## 5. Evidence (commands run)

- Census: `tc3-tools/census.sh <tag> <crates>` (`bun tc3-cgc7.ts`, native); `final` over 15 crates: 0 oversized, 0 refused (`🗑️generated/tc3-census-final.{log,err,csv}`). Per-input lists with `bun tc3-tools/closure.ts <crate dir> LIB|test:<name>|<entry>`.
- Test-name multisets (`names.py`): proctor 115 = 115 (identical); publication-mounted 22 = 22 (identical, 7 files vs HEAD). All other crates only changed include sites (no test fn added, removed or renamed; number: the eight unit files are each mounted exactly once, six by the companion, two by the lib).
- Fixture bytes: no fixture file is modified (git: no ` M` on any `.json`/`.sql` under the touched trees); sha256 of each read file is logged per rewritten site in `rt_include.log.tsv`.
- Typecheck: `tc3-tools/oracle_number.sh` runs `rustc +nightly-2026-07-20 --test --emit=metadata -o /dev/null` on the number crate with the companion mounted via `extern crate self as semio_framework_number;` (the crate has no dependencies, so this is a real typecheck): base and current have 0 errors and 0 warnings apart from the sandbox's own `/dev/rmeta` temp-dir error; negative control (algebra wrapper without the trait import) gives 4 E0599. `oracle_proctor.py` (unioned three crates, baseline = HEAD): test cfg 374 -> 377 errors, lib cfg with testkit 171 -> 184, all new rows are unresolved external crates (`quiz`, `server`, `serde_json`) in the moved fixture files or the macro-generated `ProctorDeciders` import; one real defect found and fixed (`command_id` used by the projections tests, now exported from the fixtures). `oracle_single.py` (baseline = live tree with my snapshots restored): time-travel found a real defect (`crate::tests::law()` from the retirement tests; the lib root is the package glue, so the module lives at `component::tests`; fixed to `super::super::tests::law()`), then 89 -> 84 errors with no new row; mesh-document 37 -> 36, os-shell 130 -> 129, brep-boolean 90 -> 90, child-group 95 -> 95, mesh-jobs 1021 -> 980 (no new local row), os-kernel `sqlite_snapshot_native_admission` 119 -> 110 (only baseline `couldn't read` rows gone). publication-mounted + companion (`oracle_union.py`): 129 -> 140, all new rows are the duplicated external-crate imports of the wrapper.
- `verify.py`: rustfmt parse of all 105 touched or new files, 0 failures; 0 paths over 240 bytes; every new `.rs` file starts with an emoji docstring; `.vscode/launch.json`, the seed (comments stripped) and `📋️build-owners.json` parse; `Bun.TOML` parse of all touched manifests and locks.
- `reg.py check`: 6 named routes + 6 commands in both launch files for each companion; owners sorted and unique (checked after each insertion).

## 6. Open

- `brep-boolean` has no headroom (lines). Fix after the committed `[DEBUG]` removal, or move the differential law (needs the lib's `debug_dump_face` first).
- `publication-mounted` 0.865 and `child-group-wire-laws` 0.921, `os-kernel` itest 0.922, `mesh-document` 0.928 are the tightest remaining; none was squeezed.
- The dsl `🛬️decoding` test file keeps 4.3 KB of embedded fixtures (own test target in the dsl-schema crate; the os-kernel helper is not available there).
- The `typegen` mirror of os-shell still has to be generated before the schema test passes on a clean checkout (by design).
- `Cargo.lock` files were edited by hand (framework: +1; root: +1; teaching: +2 and the proctor entry); refresh with `update --workspace --offline` when the launcher admits. Mounted/number/proctor type correctness beyond the oracle limits (external crates unresolved) is unverified until a real compile.

## 7. For the compile queue

```
cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check --tests -p semio-framework-expression -p semio-framework-time-travel -p semio-framework-plugin-child-group-wire-laws -p semio-framework-plugin-publication-mounted -p semio-framework-plugin-publication-mounted-tests -p semio-framework-3d-brep-boolean -p semio-framework-3d-mesh-tests -p semio-framework-3d-mesh-document -p semio-framework-os-kernel -p semio-framework-os-shell -p semio-framework-number -p semio-framework-number-tests
cd /Users/ueli/Documents/semio/🎓️teaching && bun repo cargo --store=shared --configuration=prototype -- check --tests -p teaching-proctor-base -p teaching-proctor-domain -p teaching-proctor
```
Expect to check first: the proctor `testkit` fixtures (visibility of `Admission`, `command_kind`, `command_target` from `use super::*` in `actors/🧰️fixtures`), `perfect`/`bus` signatures, the os-kernel `fixture_json(file!(), …)` root discovery, the receipt wrapper imports of the publication companion, then `cargo test -- --list` per crate against the multisets above.

## 8. Found state

Fresh slice; no earlier tc3 files existed. Pre-existing uncommitted changes of peers in the same trees (staged `M`/`A` on `package.json`, `📋️project.json`, mesh contract json, taxonomy, launch files) were left alone; `tc3-tools/orig/` holds my pre-edit copies of every file I edited.
