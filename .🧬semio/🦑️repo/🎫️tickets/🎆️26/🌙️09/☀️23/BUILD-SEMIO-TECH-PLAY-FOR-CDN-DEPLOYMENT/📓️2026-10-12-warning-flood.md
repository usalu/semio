# 2026-10-12 Warning flood in the cold release build

Goal: remove the compiler-warning flood (log volume, rustc suggestion rendering) at the source. Result: **-95.0 % warnings** on the hub wasm workspace check and **-94.7 %** natively, with the build green apart from an in-flight peer edit (see "Open").

## Measured (cargo check, `--message-format json`, count of `level=warning` diagnostics, shared build-dir, private `CARGO_TARGET_DIR`)

| Check | Before | After | Reduction |
|---|---:|---:|---:|
| `cargo check --manifest-path 🌎️hub/Cargo.toml --workspace --exclude semio-hub --exclude semio-hub-auth-client --target wasm32-wasip2` | 18,529 | 920 (clean, green run) | 95.0 % |
| same, native (no `--target`) | 18,761 | 998 (clean, green run) | 94.7 % |
| same wasm, last run after the final `#[cfg(test)]` splits (blocked by a peer, see Open), like-for-like over the 212 packages that finished | 8,783 | 455 | 94.8 % |
| same native, last run, 594 finished packages | 8,989 | 501 | 94.4 % |

The "920 / 998" runs are the last fully green hub runs (exit 0, wasm and native). The later runs also include 139 more import splits (`unused_imports`, below) but are cut short by the peer error, so only the like-for-like rows are comparable.

Per lint (hub wasm, before to green run): `unused_qualifications` 7,417 to 19, `non_upper_case_globals` 6,088 to 0, `unused_imports` 2,180 to 320 (to 99 in the last partial run), `unused_variables` 1,785 to 7, `non_snake_case` 288 to 3, `elided_lifetimes_in_paths` 56 to 0, `unused_braces` 40 to 0, `unused_mut` 38 to 0, `deprecated` 13 to 0, `dead_code` 533 to 494 (unchanged on purpose, see below).

### Plugin workspaces (`✏️s/🔌️plugins/<p>/Cargo.toml --workspace --target wasm32-wasip2`)

No pre-change run of these existed, so "before" is a proxy: the sum of the pre-change hub counts of the same packages (the shared artifact and framework crates compile identically in both). "After" is measured directly.

| Workspace | packages checked | before (proxy) | after |
|---|---:|---:|---:|
| 🗄️stdio | 85 | 3,370 | 218 |
| 🀄️wfc | 44 | 1,634 | 41 |
| 📕️norm | 83 | 3,357 | 217 (+2 errors, peer) |
| 🔱️trinity | 39 | 1,614 | 39 |
| 🧱️block | 53 | 1,957 | 65 |

Only about 40 % of each workspace's packages exist in the hub, so the proxy under-counts the real "before"; the true reduction is at least the figure these rows give (roughly 93 to 98 %).

## Fixes by cause

### Generators (source of 6,088 + ~1,800 + 20 warnings)

1. `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🦀️.rs`: the borrowed-metadata statics were named `__DSL_BORROWED_<CamelName>_FIELDS/_ENUM/_VARIANTS/_VARIANT_<n>_FIELDS`. New `borrowed_table_ident` emits `__DSL_BORROWED_<UPPER_SNAKE>_...` (an underscore before every uppercase letter after the first, so `HTTPServer` and `HttpServer` stay distinct). The names are private statics in one file, nothing referenced them externally (`rg __DSL_BORROWED` finds only this file). Test: `tests::borrowed_tables_are_upper_snake_named_and_stay_distinct` in `🧪️tests/🦀️.rs` (3/3 pass with the two existing tests); the cross-check is rustc's own `non_upper_case_globals` lint reporting 0 across all 160 hub crates.
2. `🧰️framework/🔨️modules/🌱️value/✨️derive/🧵️canonical/🦀️.rs` (`CanonicalJsonTree`): the tagged struct-variant arms bind every field in the node, child and key methods but each method uses only some of them; that produced ~1,780 `unused_variables` on the *field declarations* of user types (the bindings reuse the field idents' spans). The three generated methods now carry `#[allow(unused_variables)]` (the same attribute the file already puts on the per-variant view impls). Applying rustc's own suggestion (`text: _`) there would have rewritten user struct definitions, so these were excluded from the mechanical pass.
3. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`: `framework_reserved_job!` generated an unused `new` per job type (20 `dead_code`); `#[allow(dead_code)]` on that generated item.
4. `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📽️projection/🟦️.ts` (+ the ignored `🌎️hub/🧩️compositions/🌊️flow/🤖️generated/🌊️flow-dag/🦀️.rs` it produces): generated manifest constants emitted `#![allow(dead_code)]` (18 `dead_code` per consumer).

### Mechanical source fixes (about 2,780 files)

`warning-flood/fix-warnings.py` (kept in the ticket folder, with `run-check.sh` and `rename-snake.py`) applies rustc's `MachineApplicable` suggestions from the JSON diagnostics, not `cargo fix`, because `cargo fix` reverts files on a failed verify pass (it would clobber concurrent edits) and applies the field-ignore suggestion to struct definitions. Safety rules:

- a diagnostic is applied only if the file is flagged by **every** compilation unit that compiles it (units and their source files come from the `compiler-artifact` `.d` files), across wasm32-wasip2 and native; this vetoes cfg-dependent items and files that are `#[path]`-included into several crates;
- each edit re-validates the exact source text at the byte range right before writing and aborts the file if its mtime or size changed meanwhile (peers edit concurrently);
- `unused_qualifications` and `unused_imports` are never applied in the same pass: rustc reports both for the same `use` (the qualified path makes the import look unused), and doing both broke `DslValue` in three files (found by the first wasm run, repaired by hand);
- `unused_imports` whose names also occur in the file's test code (`#[cfg(test)]`, `#[path = "🧪️tests/..."]` modules, sibling `🧪️tests` dirs) are kept for tests: whole `use` statements become `#[cfg(test)] use ...;` (445), grouped imports are split into a normal and a `#[cfg(test)]` statement (137). No import used by `use super::*` in a test module was deleted.

Applied (edit spans): `unused_qualifications` 7,452, `unused_imports` 1,581 removed + 445 gated + 137 split, `unused_variables` (underscore prefix) 621, `unused_braces` 80, `unused_mut` 37, `unused_parens` 14, `elided_lifetimes_in_paths` 55 (`<'_>`), `deprecated` (`fetch_update` to `try_update`) 8.

`non_snake_case` (288): editor files carried functions named like `bmpEditor_example_snapshot` (a rename codemod artefact, not generated). `rename-snake.py` renamed them to rustc's suggested snake case within 54 files; all references are file-local, the build confirmed.

## Not changed on purpose

- `dead_code` (about 494): real unused or only-test-used items and helpers in files that several crates `#[path]`-include; deleting them blind risks the test and plugin builds. They are 2.7 % of the old volume. Candidates by size: `semio-framework-plugin` 136, `semio-s-artifact-stdio-semio` 65.
- `unused_extern_crates` 23 (`extern crate self as <name>;` derive-path idiom), `private_interfaces` 11, `unused_must_use` 7, `unused_assignments` 7, `unused_doc_comments` 6, `async_fn_in_trait` 4: real code-review items, left as the owners' call.
- 31 grouped imports that are test-used but not splittable (`as _`, nested braces).

## Verification

| Check | Result |
|---|---|
| hub wasm32-wasip2 workspace check (green runs `hub-w7`, earlier `w3`, `w4`, `w5`) | exit 0 |
| hub native workspace check (`n7`, earlier `n2`, `n4`, `n5`) | exit 0 |
| after the final cfg-test splits (`w8` to `w11`, `n8` to `n11`) | exit 101, only `cannot find value 'control'` in three shared `admission/🦀️.rs` files (norm registry contract, tsv iana, csv rfc4180), see Open |
| plugin workspaces stdio, wfc, trinity, block wasm32-wasip2 | exit 0; norm exit 101 (same `control` file, peer) |
| standalone `--lib --tests` of stdio-semio, pdf, bmp, cad, puzzle-5d | pdf and bmp exit 0; semio only the `control` error; cad 152 peer errors (`PluginCloseStep`, `SnapshotRetire...` unresolved, none in touched files); puzzle-5d 3 `crate::editor` errors from a `component-app-assembly` cfg that is off in a plain test check, in lines I did not edit |
| hub `--workspace --tests` native | errors only in peer-owned files (`🏃️run`, `generated-test-contracts`, `control`) |
| derive crate `cargo test --lib` | 3/3 pass |
| root workspace `--workspace --lib` native | 761 warnings but hundreds of errors from the ongoing migration (`optional_json_to_dsl`, arity changes); the 2 errors in files I touched are `semio_framework_job` unlinked in `os-run` and a non-feature crate, not edit-related |

## Open

- The three `admission/🦀️.rs` files (`📕️norm/📇️registry/🧬️contract/🪶️sqlite/🫳️admission`, `🗄️stdio/🗿️artifacts/📑️tsv/.../🛂️admission`, `📊️csv/.../🛂️admission`) lost their `let mut control=SqliteSnapshotControl::new(&mut progress,limits);` binding in a peer's in-flight edit (not in my edit logs; the only parts of the csv and tsv files I changed are `std::mem::` qualifications). They block the wasm and native hub checks and the norm plugin workspace until that edit lands. After it does, re-run the two hub checks and the final warning count will be about 4 % below the "920 / 998" rows.
- Whoever lands the `admission` fix should rerun the hub wasm check once; the fixer script can be re-run on top (`--run label=json:basedir`, add `--apply`) to take the remaining unit-vetoed cases.
- The shared private cargo target dir `.🧬semio/🦑️repo/⚡️cache/warn/target` was deleted when done.

## Files touched

Generators: `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🦀️.rs`, `.../🧪️tests/🦀️.rs`, `🧰️framework/🔨️modules/🌱️value/✨️derive/🧵️canonical/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📽️projection/🟦️.ts`. Mechanical edits: about 2,780 `.rs` files under `🧰️framework`, `✏️s/🔌️plugins`, `🌎️hub` (per-pass logs `plan*.tsv.applied` are in the session scratchpad, the scripts are in `warning-flood/`).
