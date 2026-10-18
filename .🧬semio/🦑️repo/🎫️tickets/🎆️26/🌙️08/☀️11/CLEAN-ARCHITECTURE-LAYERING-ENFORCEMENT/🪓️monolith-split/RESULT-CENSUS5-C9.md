# Result CENSUS5-C9 (prefix `c5c9`, package 9 of MONOLITH-CENSUS-5)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo/nx/bun repo. `F` = this folder. `OD` = `🧰️framework/🛍️products/💻️os/🔨️modules`. Tools: `F/c5c9-tools/` (descendant of `c5c-tools`; the predecessor's drafted specs `specs/dc.py` and `specs/dd.py` and the `FRAG["dcu"]` table were reviewed and reused, `dd` was extended, see section 2). Logs: `F/🗑️generated/c5c9-*` and `c5c-c5c9-*` (nothing deleted). Sandboxes: `/Users/ueli/.cache/c5c9/`.

## 0. Verdict

**DONE (static evidence; the real `cargo check` is for the coordinator).** All three files are split, installed and verified on the live tree. Snapshots of the predecessor were re-verified by sha256 (`dc` a897c7.., `dcu` 4b1505.., `dd` 2a9fa9..) and equalled the live files before the first edit; copies are in `F/c5c9-tools/orig/`.

| tag | file (tail under `OD/`) | before | root | children (max) | mechanism |
| --- | --- | ---: | ---: | --- | --- |
| dc | `📇️directory/🔌️client/🦀️.rs` | 2,404 | 108 | 8 (561) | `#[path]` modules + `pub use child::*`; the two inline cfg-gated `pub mod native`/`pub mod browser` bodies moved to files (`unwrap`), the cfg and the module names are unchanged |
| dcu | `📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs` | 1,295 | 14 | 3 (463) | `include!` fragments of a flat test module (37 tests before and after) |
| dd | `🗣️dsl/✨️derive/🦀️.rs` | 1,405 | 26 | 5 (386) | `#[path]` modules + `pub use child::*` |

Children (folder, lines):
- **dc**: `📜️http-and-transport-contracts` 208, `⚠️errors-and-event-pages` 254, `🔑️socket-grants-and-authority` 157, `🧰️wire-helpers-and-credentials` 351, `📡️directory-client` 561, `🌊️directory-stream` 171, `🖥️native-transport` 519 (`pub mod native`), `🌉️browser-transport` 128 (`pub mod browser`).
- **dcu**: `🔐️credential-wipe-fakes-and-event-page-laws` 417, `🛰️execution-target-admission-and-stream-laws` 408, `🚦️turns-receipts-cancellation-and-store-laws` 463.
- **dd**: `🔖️mutation-source-authority` 317, `🧾️mutation-leaf-descriptor` 158, `🪪️mutation-leaf-attrs-and-schema-index` 350, `🏗️document-diff-and-composite-expansion` 206, `🔁️mutations-expansion` 386.

Proc-macro note: `dd` is not the proc-macro crate root. It is the implementation owner that `🧩️expand/🦀️.rs` mounts as `mod component` (`semio-framework-os-kernel-dsl-derive-expand`, a plain lib). The `#[proc_macro*]` entry points live in `✨️derive/📦️packages/🦀️rust/🦀️.rs` (68 lines), which was not touched and still mounts nothing. The file split contains no `#[proc_macro*]` item.

## 1. Evidence (all run on the live tree)

| Check | Result |
| --- | --- |
| Lossless (`modsplit.py lossless dc/dd <live dir>`) | `LOSSLESS OK`, `missing 0`; extras are only wiring (`#[path]`, `mod`, `pub use child::*`), child docstrings, `use super::*;`, balanced region markers (dc regions orig/new 10/12 because `Client` and `Wire` span a cut) and, for dd, the two declared explicit `use` lines |
| Fragments (`chk.py live dcu`) | `CHK OK`: spliced sequence equal, `gone=0 extra(non-marker)=0`, `#[test]` fn multiset 37 = 37, no include literal rebased (the only path macro, `fixture_text!`, is manifest relative) |
| Structure (`vmod.py live dc dd`) | `VMOD OK`: each child mounted once, docstring emoji = folder emoji, unique among siblings and different from the root emoji, no sibling-folder clash, regions balanced, brace depth 0, rustfmt parses, paths 119 to 137 UTF-8 bytes (limit 240) |
| Definitions, ambiguity, clashes | `defscan.py`: 0 cross-file private leftovers (dc 28, dd 47 declarations); `ambig.py`: 0 glob-vs-glob candidates; `modclash.py`: OK |
| Names | `rgi.mjs` bad=0 for 16 names; `tax.ts`: 16 dirs, 0 unresolved, kinds `crate-module-children` (13) and `members-of-tests` (3) |
| Source-boundary closure (canonical `readRustSourceClosureV1`, `census-own.ts`, files / bytes / active lines, ratio of limits 32 / 262,144 / 4,000) | `semio-framework-os-directory-client` lib 10 / 186,054 / 2,505 (0.710) to 16 / 150,269 / 2,560 (0.640); `-directory-client-tests` test 7 / 106,744 / 1,727 (0.432) to 10 / 107,767 / 1,737 (0.434); `-dsl-derive-expand` lib 2 / 101,075 / 1,403 (0.386) to 7 / 104,053 / 1,446 (0.397). All inside the gate. Log `🗑️generated/c5c9-census-own-after.jsonl` |
| Scoped patched differential (`oracle5.py`, original vs live, lib and `--test`) | dc 65 = 65 errors and dcu 435 = 435 errors, warnings equal (3 = 3): **CLEAN**. dd 130 vs 132: the two extra errors are `unresolved import quote` of the two explicit `use quote::quote;` lines (the sandbox has no `quote` crate; the root's own import is already counted in both); no other difference. Log `🗑️generated/c5c9-oracle-all.log` |
| Typeck-level oracle (`posrun.py` family `c5c9`, labels `pre` (predecessor, unsplit tree), `post`, `post2` after the dd fix) | typeck canary reached; privacy 0 / 0, trait-scope 0 / 0, other 0 / 0, resolve 12 / 18 and type 5 / 9 (lib / test) identical to `pre`. `posdiff.py`: the only row differences are 3 rows whose stub trait is named `Tr` instead of `FromValue` (traits held by the root's private imports are visible to a child only through its `use super::*;`, which the stub layer does not follow) plus one extra `R: Tr` row at the same site; no row in the dd crate differs |
| Text readers (`readers.py`, a Python mimic of `readModuleTreeSource`/`sliceModuleTreeSource`, because `node_modules` is empty and no TS test can run here) | 19 of 19 assertions of the re-pointed TS readers hold on the unsplit snapshot and on the split tree; the `expand_mutations` .. `mutation_relative_path` identifier window is identical (421 identifiers, 20,296 bytes), the dd fn list is identical (61 names) |
| TS parse | `bun build --no-bundle` exit 0 for all 11 edited TS files |

## 2. Findings worth keeping

- **The canonical closure reader refuses a `quote!` that contains `include*!` in a file that has a glob `use`** (`Unsupported Rust compile token-output origin: quote`, `📖️syntax/🦀️rust/🟦️.ts`). Two dd children (`🪪️mutation-leaf-attrs-and-schema-index`, `🔁️mutations-expansion`) hold such `quote!` blocks (`include_str!(#dependency)`), so they cannot use `use super::*;`. They carry a direct `use quote::quote;` and an explicit `use super::{...};` list instead (declared in `specs/dd.py` through `glob=False, uses=[...]`, listed in `expect_extra`); each listed name was checked to have a real (not path-segment, not field) occurrence in the file. The other children keep the glob.
- The first draft named the leaf-descriptor child `🔣️…`, which the taxonomy resolves to `json-fixture-case`; it was renamed to `🧾️mutation-leaf-descriptor` (`crate-module-children`) before the final verification.
- The 962-line rule did not bite; no function over 1,000 lines. The largest child is 561 lines (the 524-line `impl DirectoryClient`).

## 3. Source-text readers

Search: `git grep -F` of the three path suffixes, the crate folders, 140+ distinctive item names (`DirectoryClient`, `LocalHubCredential`, `diff_text_tokens`, `expand_mutations` and so on) over `*.ts *.tsx *.json *.rs *.py *.mjs *.js`, plus a read of every candidate (log `🗑️generated/c5c9-readers.txt`).

| Reader | What it read | Action |
| --- | --- | --- |
| `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts` (2 sites), `🌎️hub/🧪️tests/🔗️execution-target-lease-checks/🟦️.ts` (+ import), `🌎️hub/🧪️tests/🧭️space-journey-checks/🟦️.ts`, `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts`, `OD/../📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts` (2 sites) | `readFileSync` of the directory client root | **edited**: `readModuleTreeSource(join(repoRoot, …/🔌️client/🦀️.rs), { crates: false })` |
| `🌎️hub/📦️packages/🦀️rust/📋️project.json` input `hubFoundationSources` and `🌎️hub/🧪️tests/🧫️fixtures/🧱️foundation-source/🔣️.json` `route.inputs` (the test asserts the two are equal) | the client root as a cache input | **edited both identically** to `…/🔌️client/**/🦀️.rs` |
| `OD/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🧫️fixtures/🔣️.json` `rustReceivers` | tree-sitter parse of the dcu root | **edited**: the three fragments are listed after the root so they stay parsed |
| `✏️s/…/🧩️puzzle/…/🖐️5d/…/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | `diff_text_tokens` window of the derive file | **edited**: `sliceModuleTreeSource(frameworkSourceUrl(…derive/🦀️.rs), 'fn diff_text_tokens(', 'pub fn expand_diff_binary(')` |
| `🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts` | `expand_mutations` .. `mutation_relative_path` window | **edited**: `sliceModuleTreeSource` |
| `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🧪️tests/🟦️.ts` | derive owner among the `owners` list (regex and rustfmt) | **edited**: the derive owner is read and rustfmt-parsed through `listModuleTree` |
| `OD/🗣️dsl/✨️derive/🧪️tests/{🚪️diff-codecs,🪆️record-owner,📤️macro-exports}/🟦️.ts` | tree-sitter top-level fns / `#[proc_macro_derive]` regex of the owner | **edited**: all files of the module tree are parsed (`listModuleTree`) or read (`readModuleTreeSource`) |
| `OD/🗣️dsl/✨️derive/🧪️tests/{🪆️record-owner,📤️macro-exports}/🦀️.rs` (Rust twins, `syn::parse_file` of the owner root) | top-level fn items and derive attributes of the owner | **edited**: a 20-line `owner_items` helper follows the `#[path]` mounts; used for the owner and the glue root (same result for the glue, which has no mounts) |
| `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/🧪️tests/🟦️.ts` | derive root | **no edit**: already `readModuleTreeSource` |
| `…/🎯️mutation-outcome/🟦️.ts` (`POLICY_DERIVE_OWNER`) | existence of the owner file and a regex on the glue | **no edit** |
| comments with line numbers (`🔌️client/🦀️.rs:374`, `✨️derive/🦀️.rs:1419-1473`), `.cursor/plans`, `Cargo.toml`, `📋️build-owners.json`, `🔒️dependencies.json`, launch files, `taxonomy.json` | prose, crate roots, manifests, folder names | **no edit**: no crate, target, command or owner row changed |

Not run (no `node_modules`): the TS tests themselves; the Python mimic and `bun build` parse checks above stand in for them.

## 4. Open risks

1. The wasm-only child `🌉️browser-transport` (`cfg(all(target_arch = "wasm32", not(target_env = "p2")))`) and the feature-gated `🖥️native-transport` (`ureq` + `sync`) are not compiled by a plain native `cargo check`. The companion crate `semio-framework-os-directory-client-tests` enables `ureq`, `sync` and `testkit`, so checking it covers `native`. `browser` needs a wasm32 check or stays unproven (its imports are explicit `use super::{…}`, resolution was reviewed by the differential only).
2. The two dd children with explicit `use super::{…}` lists: a name missing from the list would fail with `cannot find`; the typeck oracle and the resolution differential show no such row, the real compile is the confirmation.
3. `owner_items` in the two Rust test files is unrun code (standard `syn` 2 API, rustfmt parses it).
4. The hub project input widened from one file to `**/🦀️.rs` below the client folder (more Nx invalidation, never less).

## 5. For the coordinator

Compile (lib and tests): `cargo check -p semio-framework-os-directory-client --lib --tests`, `cargo check -p semio-framework-os-directory-client-tests --lib --tests` (carries features `ureq,sync,testkit`), `cargo check -p semio-framework-os-kernel-dsl-derive-expand --lib --tests`, `cargo check -p semio-framework-os-kernel-dsl-derive-tests --lib --tests` (uses the edited `owner_items`) and, for the proc-macro consumer, `cargo check -p semio-framework-os-kernel-dsl-derive`. Via the shared store: `bun repo cargo --store=shared … -- <cmd>`.
Run after a dependency install: `bun test` on the 5 hub/os TS readers of section 3 and `📤️macro-exports/🟦️.ts` (imports the other two derive readers).
