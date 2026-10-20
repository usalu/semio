# Result gran-value: value tower interface/implementation split

Executor `gran-value` (wave 1020e), stage-only. Tools: `gran-value-tools/` (`staged/` carve + oracle, `analysis/` measurements, `data/` edge ledger). Generated data: `🗑️generated/gran-value/`. Nothing applied to the checkout, no cargo, no `bun repo`, no git.

READY: `gran-value-tools/staged/apply.py` (3 groups, 19 files, dry run valid on the checkout, overlay apply + oracle differential clean). **READY2: `staged/apply2.py` (3 more groups, 577 files), section 8.** Both wait for coordinator GO.

Tags: **VERIFIED** = ran or read; **INFERRED** = computed from manifests/text by me.

## 0. Answer

1. The value tower cannot be made cheap by splitting alone. Every edit in the bottom 8 crates rebuilds ~2,915 of 3,213 crates, and that floor is forced by the orphan rule and by `retirement` storing its cursors in `PagedList` (section 1.5). The plan therefore has two halves: make the floor cold (ring 0 is 6.4k lines, 8 crates, frozen by a law), and move every hot line out of it so that hot crates have small reach.
2. The audit graph under-counted. It resolved `workspace = true` through the root workspace only. Re-resolving through the owning workspace adds **14,243 edges (45,013 -> 59,161, +31 %)**. Corrected: mean reach **363 crates (11.3 %)**, longest chain **157** (audit: 261 and 122), **354 crates with reach >= 2,000** (same script on the uncorrected graph: 209). Use the corrected numbers below.
3. The value family is only **8.4 % of churn x reach** over the 366 hubs with reach >= 1,500. The hottest hubs are `plugin`, `os-kernel`, `plugin-reactor-turn`, `kernel`, `os-store-sync`, `job` (section 1.3). Value is still the right first carve because its three hottest crates (`value` facade, `value-derive`, `value-intrinsic`) sit in the global top 30 and reach 2,894 to 2,904 crates.
4. Churn-weighted expected rebuild per value-family edit: **2,903 -> 2,885 (apply.py) -> 2,828 (apply.py + apply2.py)**. The "2,465 for the full plan" first reported here was wrong (see section 8.3): the correct full-plan figure is about 2,761. Specific crates drop much further (`registry` 2,895 -> 2,197, `retained-map` and `paged-text` 2,895 -> 0, `derive-expansion` 2,902 -> 1, `dres` macro 2). The ring-0 floor stays at ~2,910 by design.
5. Three carves are staged now (section 5). Everything after them is planned with measured effects (section 4).

## 1. Measured current fan-in

### 1.1 Method

- Scoped manifest parse: the audit's crate list (`crates.json`, 3,213 manifests) re-resolved by `analysis/graph_fix.py` through the `[workspace.dependencies]` table of the workspace root that owns each crate (inline and table form). Remaining unresolved names: 17 externals (serde, tokio, ...). VERIFIED by run.
- Source-level evidence by module-tree walk from each crate's lib root (`#[path]` mounts, `include*!`, plus bin/test/example/bench/build roots): `analysis/tree.py`. No repo-wide grep or find.
- Churn from `git log --name-only` mapped onto mounted files (read-only).

### 1.2 Value family (corrected graph; prod lines = module-tree lines without tests)

| crate | prod lines | direct | transitive | chain above | commits |
| --- | ---: | ---: | ---: | ---: | ---: |
| `value-control-contract` | 157 | 24 | 2,919 | 157 | 6 |
| `value-allocation-return` | 81 | 3 | 2,918 | 156 | 4 |
| `value-paged-list` | 1,039 | 8 | 2,917 | 155 | 18 |
| `value-retirement` | 1,384 | 24 | 2,916 | 154 | 21 |
| `value-shared-issuance` | 139 | 14 | 2,911 | 153 | 3 |
| `value-native-encoding` | 378 | 6 | 2,909 | 153 | 11 |
| `value-native-decoding` | 579 | 7 | 2,908 | 152 | 16 |
| `value-ordered` | 966 | 2 | 2,905 | 151 | 22 |
| `value-intrinsic` | 2,661 | 8 | 2,904 | 150 | 29 |
| `value-clone-source` | 320 | 4 | 2,904 | 150 | 15 |
| `value-derive-expansion` | 2,811 | 1 | 2,902 | 149 | 13 |
| `value-derive` (proc-macro) | 1,117 | 1,909 | 2,901 | 148 | 31 |
| `value-retained-clone` | 2,135 | 7 | 2,901 | 149 | 21 |
| `value-persistent` | 918 | 113 | 2,897 | 147 | 3 |
| `value-native-owned` | 647 | 353 | 2,896 | 146 | 3 |
| `value-edit` / `registry` / `schema` | 725 / 317 / 103 | 6 / 6 / 5 | 2,895 | 145 | 3 / 2 / 10 |
| `value-factory` | 708 | 5 | 2,895 | 145 | 15 |
| `value-retained-map` / `paged-text` / `retirement-queue` | 1,281 / 270 / 146 | 1 | 2,895 | 145 | 14 / 7 / 10 |
| `value` (facade) | 2,854 | 2,386 | 2,894 | 144 | 35 |
| `value-snapshot-read` | 1,323 | 25 | 2,496 | 122 | 7 |

Full table: `🗑️generated/gran-value/fanin.md` (regenerate with `analysis/fanin.py`). Outside the family the same floor is held by `schema-registry` (1,188 direct), `hash` (445), `io-base64` (351), `native-build-compiler-resources` (3), `pack-json`, `diagnostic`.

### 1.3 Where the cost is (reach distribution, churn x reach)

- Crates with reach >= 2,800: 29; >= 2,500: 123; >= 2,000: 354; >= 1,500: 366; >= 500: 537; >= 100: 896 (VERIFIED, `analysis/hubs.py`).
- Churn x reach over the 366 hubs: total 11.78 M, value family 8.4 %. Top by that product: `plugin` 217 k, `os-kernel` 190 k, `plugin-reactor-turn` 113 k, `kernel` 108 k, `os-store-sync` 108 k, `job` 103 k, `os-spr`, `ui-contract`, then `value` 101 k, `value-derive` 90 k, `value-intrinsic` 84 k (`analysis/hothubs.py`).
- Cross-workspace: 43,085 of 59,161 edges (**72.8 %**) cross a workspace boundary, 31,725 of them into `🧰️framework` (`analysis/crossws.py`). 141 of the 232 `Cargo.lock` files (root + nested) carry the value family, so any dependency-edge change in a value crate stales 141 locks (VERIFIED by scan of the lock files).

### 1.4 Who depends on the facade and derive (census of all 2,479 direct dependents)

- `semio-framework-value` has 2,386 direct dependents, `value-derive` 1,909, union 2,479, both 1,816 (`analysis/census.py`).
- Production use: 2,104 (84.9 %). Test-only: 169 (6.8 %). **No use at all: 206 (8.3 %)**.
- Family-wide edge hygiene (normal edges from outside the family into it, bin/test/example/build roots included): 4,089 used, **439 unused, 300 test-only** across 477 crates (`data/hygiene-edges.json`).
- Derive users (production, crates): `RetireOwned` 1,368, `ToValue` 1,349, `FromValue` 1,337, `CanonicalJsonTree` 953, `RetainedClone` 628, `FactoryPayloadRetirement` 69, `NativeReceive` 3, `value_codec!` 2, `SnapshotProjection` 1, `RetainedEquality` 1, `owned_json_file!` 1. Grouped by edit affinity: retire 1,402, codec 1,351, canonical 953, retained 630, resources 1.
- Names used from the facade (crates): `ValueError` 1,549, `RetireOwned` 1,351, `ValueRefusalKind` 871, `DslValue` 654, `retained_clone::*` 361 (almost all control-contract names `RetainedCloneGrant/Progress/Step`), `RetirementDemand` 296, `retirement` 295, `NativeEncodeControl` 178, `paged` 52, `SharedUtf8` 42, `ValueType/ValueKind` 17, `ordered` 11, `retirement_queue` 7, `numeric_scratch` 5, `snapshot_projection` 4, `OriginalAliasBatch` 2, `paged_text` 2. Factory names: 111 crates.
- Consequence: only 630 crates need ring-1 `retained-clone` (the `RetainedClone` derive); about 230 more need another non-ring-0 owner (factory 111, text/paged 95, type 15, ordered 11, queue 7, projection 4, aliases 2); the other ~1,250 use ring-0 names and the codec/canonical/retire derives only.

### 1.5 The orphan chain (why the bottom is one chain) - VERIFIED by reading the code

`control-contract -> allocation-return -> paged-list -> retirement -> shared-issuance -> native-encoding -> native-decoding -> ordered -> intrinsic`, 8 to 9 crates, 6.4k prod lines without `ordered`.

- `retirement` genuinely uses `PagedList`: `🎮️controlled` stores `PagedList<Box<dyn RetirementCursor>>`, so `paged-list` cannot move above `retirement`, and `impl RetireOwned for PagedList` has to stay in `retirement` (orphan rule).
- `intrinsic` mounts the `ToValue/FromValue` impls of `PagedList` (`📋️list/🌐️wire`) and of `SealedShared`. Moving them into `paged-list` or `shared-issuance` would close a cycle through `retirement`. They stay.
- Only the `OrderedMap/OrderedSet` impls (`🗂️ordered/🌐️wire`, 4 files, ~125 lines) can move: `ordered` depends on `control-contract`, `retirement`, `native-decoding` and nothing above, so `ordered -> intrinsic` is acyclic. That is staged carve C2. The earlier ideas "retirement/paged-list inversion" and "retained-clone/paged-list inversion" are **rejected**: both close cycles.
- The facade is not a type crate: it mounts 2,854 prod lines of its own (`types`, `read`, `paged`, `shared_utf8`, `retained_utf8`, `bounded_clone`, `numeric_scratch`, `scratch_hash`, `retained_clone/paged`) and depends on 26 crates, 9 of which it never names (C1).
- `value-derive` hosts 9 derives and 2 function macros in one proc-macro crate; its closure is `hash` (unused), `native-build-compiler-resources` (only for `owned_json_file`, 1 user) and `derive-expansion` (2,811 lines).

## 2. Target structure

Rule: **dependents compile only against cold crates; hot code lives where few crates depend on it.** rustc embeds the dependency's crate hash, so any body edit in X rebuilds every dependent of X (INFERRED from rustc behaviour, not measured here). An interface layer pays only if the interface crate is edited rarely and no hot code sits below it.

| ring | crates | rule |
| --- | --- | --- |
| R0 ABI (cold) | control-contract, allocation-return, paged-list, retirement, shared-issuance, native-encoding, native-decoding, intrinsic (6.4k prod lines) | frozen by a law: a fixture of the public item names per crate (made by the static resolver), regenerated only inside a ticket named `value-abi-*`; no collection engines, no I/O, no new modules |
| R0 surface | `semio-framework-value` becomes a pure `pub use` aggregate of R0 (no mounted modules, no deps beyond R0) | not a compat layer: it is the import surface for the 2,100 users of ring-0 names |
| R1 structures (cold-ish) | ordered, clone-source + retained-clone, persistent, native-owned, factory, snapshot-projection, original-aliases | may depend on R0 and R1 below them only |
| R2 behaviour (hot, small reach) | edit, registry, schema, retirement-queue, retained-map, paged-text, snapshot-read, dictionary, resident, and the carved facade bodies (`value-type`, `value-text` for `paged` and `shared_utf8`, `value-read`) | the only ring where hot work happens |
| derives | `derive-codec` (ToValue, FromValue, value_codec), `derive-canonical`, `derive-retire` (RetireOwned, FactoryPayloadRetirement), `derive-retained` (RetainedClone, RetainedEquality, SnapshotProjection, NativeReceive), `derive-resources` (owned_json_file) | each derive emits against the crate path of its own ring (the `crate=`/`native_owned=`/`persistent=` keys already exist); the default root `::semio_framework_value` goes away |

Law to add with C6: ring order for `semio-framework-value*` crates in `crate-boundaries` (`🧬️cuts.json`), same mechanism as the existing independence rows.

## 3. Expected rebuild counts (corrected graph, `analysis/sim3.py` and `staged_sim.py`)

Transitive dependents per edit:

| crate | now | staged C1-C3 | + C4 hygiene | + C5-C7 full plan |
| --- | ---: | ---: | ---: | ---: |
| ring 0 (control-contract ... intrinsic) | 2,904-2,919 | same | -4 | -1 to -3 (floor, by design) |
| `value-ordered` | 2,905 | 2,895 | 2,891 | 2,799 |
| `value-registry` | 2,895 | 2,197 | 2,197 | 2,197 |
| `value-schema` | 2,895 | 2,585 | 2,177 | 2,177 |
| `value-edit` | 2,895 | 2,593 | 2,593 | 2,593 |
| `value-retained-map`, `paged-text` | 2,895 | 2,895 | 2,891 | **0** |
| `value-retirement-queue` | 2,895 | 2,895 | 2,891 | 2,223 |
| `value-original-aliases` | 2,898 | 2,895 | 2,891 | 2,165 |
| `value-factory` | 2,895 | 2,895 | 2,891 | 2,793 |
| `value-native-owned` | 2,896 | 2,896 | 2,892 | 2,823 |
| `value-derive-expansion` | 2,902 | 2,902 | 2,898 | **1** |
| `native-build-compiler-resources` | 2,902 | 2,510 | - | 2,510 |
| `hash` | 2,908 | 2,691 | 2,904 | 2,691 |
| `schema-registry` | 2,912 | 2,671 | 2,671 | 2,671 |
| derive groups | 2,901 (one crate) | 2,901 | - | not staged, see 8.3 (codec 2,785, canonical 2,682, retire 2,867, retained 2,688) |

Longest chain from `control-contract`: 157 -> 155 (staged), `ordered` chain 151 -> 145. Mean reach over all crates: 363.4 -> 362.6 (staged) -> 361.8 (full plan). Churn-weighted expectation per value-family edit (weights = commits per crate, derive and facade churn split over their successors): 2,903 -> 2,885 (C1-C3) -> 2,828 (+ apply2) ; the first-draft "full plan" column above (P4, 2,465) overstated the gain and is superseded by section 8.3 (about 2,761).

The honest reading: the full plan removes about 15 % of value-edit cost and makes the cheapest 8 crates cheap, but nothing below ring 0 can drop under ~2,700 while 1,349 crates derive `ToValue`. The remaining lever is outside value: the 354-crate waist (section 1.3) must be frozen or split the same way, and cargo incremental mode for proc-macro edits (macro output re-expansion is cheap when the tokens are unchanged) should be turned on by conc-cargo for the rebuild of derive users.

## 4. Ordered carve plan

| # | carve | files | measured effect | gate |
| --- | --- | --- | --- | --- |
| C1 | **dead-edges**: facade drops 9 unused deps (clone-source, native-decoding, paged-list, allocation-return, hash, edit, schema, registry, schema-registry), derive drops hash, edit drops native-encoding, persistent drops original-aliases | 4 manifests | registry 2,895 -> 2,197, schema -> 2,585, edit -> 2,593, schema-registry -> 2,671, hash -> 2,891; chain -1 | none (text proof: 0 references in the module trees, tests and bin roots of the 4 crates) |
| C2 | **ordered-wire**: ordered mounts its own wire impls, intrinsic loses the ordered edge | 6 files | ordered leaves ring 0; chain from ordered 151 -> 145; prerequisite for C5 | after C1 |
| C3 | **derive-resources**: `owned_json_file!` becomes proc-macro crate `semio-framework-value-derive-resources`; `value-derive` loses `native-build-compiler-resources` and the nightly tracked-path features | 9 files + 1 new crate | compiler-resources 2,902 -> 2,510; derive closure -> {derive-expansion} | after C1; fixes a latent test path |
| C4 | **edge hygiene**: delete 439 unused and demote 300 test-only normal edges into value crates (`data/hygiene-edges.json`) | ~477 manifests | -4 on every ring-0 crate, dependency cones of 477 crates shrink; schema 2,585 -> 2,177 | stage a manifest-only group from the ledger; lockref after |
| C5 | **facade to R0 aggregate**: carve the facade bodies into R1/R2 crates, drop its ring-1/2 deps, repoint the users of non-R0 names (paged 52, SharedUtf8 42, ValueType 17, ordered 11, queue 7, numeric_scratch 5, snapshot_projection 4, aliases 2, paged_text 2, Factory* 111) with the `value-repoint-tools` table mechanism (new table, owner crates already exist) | ~230 consumer crates + 1 new splitter | retained-map and paged-text 0, queue 2,223, aliases 2,165, ordered 2,799 | needs C2; dry run first; cargo-fix compile window |
| C6 | **derive split** into the 5 proc-macro crates, derive output rooted at the owner crate of its ring, default root removed | 5 crates, ~2,100 use-line rewrites (codemod) | derive-expansion 1, retained 2,688, canonical 2,682, resources 2 | after C5; cargo-fix; the one crate that edits ToValue/FromValue expansion pays 2,785 |
| C7 | **R0 freeze law** + ring order law in `crate-boundaries` | 2 fixtures | floor becomes cold | after C5/C6 |
| C8 | repoint the 11 alias crates (`pub use semio_framework_value as value`) and the glob re-exporter `replication` | ~12 crates | removes the last hidden fan-in | with C5 |
| C9 | cross-workspace: 72.8 % of edges cross a workspace; 141 value-bearing locks | - | each ring-0 edit stales 141 locks | decision for conc-cargo and lockref owners: one lock for the framework consumers, or a cached framework rlib set |
| C10 | the 354-crate waist outside value (`schema-registry`, `hash`, `io-base64`, `pack-json`, `diagnostic`, `dsl`, `async`, `replication-*`, `os-*`) | - | 91.6 % of hub churn x reach lives there | next executor, same method (census, orphan chain, ring law) |

Not done and not recommended: "retirement/paged-list inversion", "retained-clone/paged-list inversion" (cycles), moving the `PagedList` wire impls out of intrinsic (cycle), a published framework artifact before C9 is decided.

## 5. Staged first carves (C1-C3)

`gran-value-tools/staged/apply.py [--root DIR] [--group dead-edges,ordered-wire,derive-resources|all] [--diff] [--write] [--status] [--ledger DIR]`

- Default is a dry run. `--write` validates every anchor of every selected group first (exact-once replacements, single-line manifest entries, checksums of the two blocks cut from `value-derive`'s root in `anchors.json`), then writes each file atomically with first-touch backups in the ledger, reads every file back and rolls everything back on any mismatch. A second run on an applied tree refuses (anchors gone), so there is no silent re-apply.
- Splitter rules: regions and docstrings are moved verbatim (the 3 moved blocks keep their text); no compat re-exports (the 6 `owned_json_file!` call sites in `📓️print` and the `owned-json` test are repointed, the facade `pub use` lines stay untouched because none names these crates); the new crate's lib root is the existing `📸️snapshot/🦀️.rs` (feature gate, entry point and observer moved in, `expand` stays `pub(crate)`); manifests carry `role`/`id` metadata; path 118 chars.
- Consumers updated: print manifest + 3 source files (6 sites), derive manifest and test, intrinsic and ordered lib roots and manifests, 2 wire files (`crate::ordered::*`), framework workspace members.
- C3 also fixes a latent defect: the `owned_json` test called the macro with `../../🧫️fixtures/...`, a path that does not exist; it now uses `../../🧪️tests/🧫️fixtures/...`, the fixture `fixture_text!` already reads.

### Verification (overlay under `/private/tmp/gran-value/overlay`, 20 MB copy, one process at a time)

- Dry run on the checkout: 19 files (18 edits, 1 new crate manifest), valid (`🗑️generated/gran-value/apply-dryrun-real.txt`). Latest peer mtimes on the touched files: facade, derive root and persistent manifests 10-20 13:14, `🧰️framework/Cargo.toml` 19:22, the rest older; drift is refused anyway.
- `apply.test.py`: 2 tests (language-agnostic `cases.json`: 2 manifest edits + 6 refusals), cross-checked with Bun's TOML parser (`toml_keys.ts`): pass.
- `verify_manifests.py pre post`: all 8 changed/new manifests have exactly the expected added and removed dependency keys by Bun's TOML parser, and the workspace member list grows 408 -> 409 without duplicates.
- POS typeck oracle (`oracle.py`, `rustc +nightly-2026-07-20 --emit=metadata`, 24-crate value core family, lib cfg): pre 1,434 errors (1,187 stub-involved), post **1,434 errors, 0 new, 0 lost** (`oracle_diff.py`, key = code + normalised message + label; typeck canary E0308/E0616/E0624 reached on both sides, privacy 0).
- Sensitivity (seeded regressions on the post overlay): private `encode` in the moved native module -> **+1 E0603** (privacy); removing the `FromValue` import from the ordered root -> **+254 new errors** (resolution). So both failure classes would show.
- `rustfmt --check` with `skip_children` on the 10 changed `.rs` files: parse OK.
- Not verifiable without cargo: the proc-macro crate itself (`derive-resources` needs syn/quote), `--locked` lock state, wasm. Those belong to the compile window below.

## 6. GO sequence and coordination

1. cargo-fix is compiling the value family in its own overlay (`/private/tmp/cargo-fix`, group `value`, 16 steps, 55 packages) and has 15 lock-refresh steps queued. **Recommended: apply C1-C3 immediately after cargo-fix's `value` group is green and before `lockref/all.sh`**, so the 141 value-bearing locks are refreshed once. If lockref already ran, it repeats for those 141 workspaces.
2. After apply: `python3 apply.py --status`, `python3 verify_manifests.py <backup tree> <live>` is not needed (the ledger holds the backups); then re-run only the touched packages in the compile window: `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --manifest-path 🧰️framework/Cargo.toml --locked -p semio-framework-value-intrinsic -p semio-framework-value-ordered -p semio-framework-value-derive -p semio-framework-value-derive-resources -p semio-framework-value-edit -p semio-framework-value-persistent -p semio-framework-value -p semio-framework-print --lib --tests`, then `bun test` for `crate-boundaries` (new package registration) and the derive `owned_json` test.
3. testfix-real: its retained carve (`replication-mutation-retained-field`) is outside the value tree, no file overlap with C1-C3; its crate list already went to cargo-fix.
4. Open for the owner of the `rust-package-commands` execution plan (`✨️derive/📦️packages/🦀️rust/🧪️execution/🔣️.json`): `derive-resources` has no tests of its own; its coverage runs through the derive package's `owned_json` test (dev-dependency edge added).

## 7. Open items

- The audit file `audit-granularity-1020e.md` should be corrected with section 1 numbers (edges +14,243, chain 157, mean reach 363).
- C4 and C5 need staging after GO of C1-C3 (the data and the name table are measured, the writers are not).
- `crates.json` is the auditor's parse; `graph_fix.py` is the fix. Both live in `🗑️generated/gran-value/` and are regenerated by `analysis/*.py` with `cwd` = that folder.
- Reproduce the whole verification: `sh gran-value-tools/staged/overlay.sh` (about six minutes, read-only on the checkout).

## 8. READY2: second staged set (C4, C5-tail, C7-rings)

### 8.1 Groups, in order (`gran-value-tools/staged/apply2.py`, run after `apply.py`)

`apply2.py [--root DIR] [--group facade-tail,hygiene,rings|all] [--consumers FILE] [--diff|--files] [--write] [--status] [--ledger DIR]`. Same contract as `apply.py` (dry run by default, all anchors validated before the first write, atomic writes, ledger backups, rollback on a failed read-back). Order matters: `apply.py` first (the ring law needs `ordered` out of `intrinsic`), then `facade-tail`, `hygiene`, `rings`.

1. **facade-tail** (C5 reduced): the facade drops 5 re-exports (`ordered`, `snapshot_projection`, `paged_text`, `retirement_queue`, `OriginalAliasBatch`) and 5 manifest deps; the 37 consumer crates (66 files, 146 path rewrites, 39 manifests, 2 workspace declarations) import the owner crates. The rewriter is the proven `value-repoint-tools` lexer (copied into `staged/repoint/`, scoped driver `repoint.ts` that only plans), extended by one table field `crateAliases` because `retirement_queue` is a whole crate behind a facade alias. Includes the `protocol::value::ordered::...` alias-head sites of the os-flow crates. Refuses on any unresolved path, planner issue or residual reference.
2. **hygiene** (C4): 436 unused edges deleted and 281 test-only edges moved to `[dev-dependencies]` in 514 manifests, each re-proved against the module trees plus bin/test/example/build roots at plan time (`data/hygiene-edges.json` is the ledger). Skipped with a count, not guessed: 4 whose verdict drifted, 3 not a single plain entry, 15 edited by facade-tail. Entries named by a `[features]` line or optional are skipped.
3. **rings** (C7, law half): `crate-boundaries/🧫️fixtures/.../🔣️.json` rows of the 8 ring-0 and 8 ring-1 value crates gain the hotter rings as `forbiddenDependencies` (15 rows changed). Refuses to reformat a non-canonical fixture.

### 8.2 Verification (overlay `pre2`/`post2`, 93 MB, `sh staged/overlay2.sh`, one process at a time)

- `apply.py` then `apply2.py` on the overlay: 577 files edited; idempotence by refusal as before.
- `verify_manifests2.ts` (Bun TOML parser, 514 changed manifests): all parse; 736 removals from `[dependencies]`, 281 demotions with the key present in `[dev-dependencies]`, 42 additions that are exactly the owner crates, 0 unexpected changes. A first run found and fixed a prefix-match bug (`semio-framework-value` vs `-derive`) that had dropped a demoted dependency.
- `rustfmt --check` (skip_children) on the 78 changed `.rs` files: parse OK. `transform-tail.test.ts`: 6 pass (5 language-agnostic cases in `cases-tail.json` plus the rustfmt cross-check). `apply.test.py`: 2 pass.
- Ring law: `check_rings.py` 0 violations on `post2`; applying only `rings` on a tree without the `ordered` carve reports exactly `intrinsic -> ordered`, so the law is live.
- POS typeck oracle, 30-crate family (24 value core + 6 consumers: time-travel, manifest-mutation-inputs, os-store-one-item-batch, pack-record-encode, plugin-tool-run, replication-mutation-retained): pre 4,915 errors, post 4,915, **0 new, 0 lost**, canary reached. Seeded breaks: a misspelled owner item -> +3 errors (E0425, E0432); a stale facade path `value::retirement_queue` -> +1 E0432. Both would show.

### 8.3 What is not staged, and the correction of the full-plan figure

The earlier P4 model counted `derive-expansion` and the macro crates as having no dependents (reach 1 and 2) and let `retained-map` and `paged-text` drop to reach 0. With the consumers wired back in (the 11 crates using `retained_clone::ordered_map`, the 2 using `paged_text`, expansion feeding the macro crates) the full plan is **about 2,761**, not 2,465. The staged sets reach 2,828. What is left is thin:

| remaining carve | weighted gain | cost | decision |
| --- | --- | --- | --- |
| C5 body carve (facade `types`, `paged`, `shared_utf8`, scratch, `read` into crates) and dropping `persistent`/`native-owned`/`factory`/`retained-map` from the facade | ~25 points | 4 new crates, glue per crate, repoint of ~230 crates plus a nested `retained_clone::ordered_map` form the table cannot express; derive output (`#value::retained_clone::...`, `FactoryPayloadRetirement`) still needs the facade root | **not staged**: gain below the risk |
| C6 derive split into 5 proc-macro crates | ~16 points | an attrs library split, 5 crates, and rewriting derive imports in ~2,100 crates; `RetireOwned` (1,368 users) and `ToValue` (1,349) overlap almost completely, so the hot expansion (`retained-clone` 13 commits) still reaches ~2,867 | **not staged**: reach of the hot half does not fall |
| C7 R0 freeze (public-item digest fixture) | 0 now, protects the floor | needs a resolver-made fixture and an owner decision on the regeneration rule | open, ring-order half staged |

If you want C5-body or C6 anyway, say so; the reuse path is the same planner (`repoint.ts` with a new table) and the oracle family approach.

### 8.4 Totals (`analysis/stage2_sim.py`, corrected graph)

| crate | now | after apply.py | after apply2.py |
| --- | ---: | ---: | ---: |
| `retirement-queue` | 2,895 | 2,895 | 2,308 |
| `original-aliases` | 2,898 | 2,895 | 2,165 |
| `paged-text` | 2,895 | 2,895 | 2,726 |
| `ordered` | 2,905 | 2,895 | 2,787 |
| `registry` / `schema` / `edit` | 2,895 | 2,197 / 2,585 / 2,593 | 2,197 / 2,177 / 2,593 |
| ring 0 (`control-contract`) | 2,919 | 2,919 | 2,915 |

Churn-weighted per value-family edit 2,903 -> 2,885 -> **2,828**; mean reach over all crates 363.4 -> 362.6 -> 361.9; chain from `control-contract` 157 -> 155 -> 155.

### 8.5 Coordination

gran-hubs (`ac2b45f0e7a54ffe0`) was sent `🗑️generated/gran-value/touched-apply2.txt` (577 files: manifests, 66 `.rs` paths, one fixture; 186 of them under `🧰️framework/🛍️products/💻️os`). Edits are single-line and recomputed from the live tree, so disjoint line-level changes compose; whole-file rewrites by gran-hubs of any listed file should be reported and the file is dropped. `lockref` must run after both sets (the new owner dependencies and the removed ones stale the 141 value-bearing locks plus the 39 consumer workspaces' locks).
