# Result value-fix: value crate family compile (static phase done, compile pass pending the dependency sync)

Executor `value-fix` (Sonnet 5.5, foreground, no sub-agents, no modifying git command). Logs and the batch runner under `🗑️generated/value-fix-*`; the relocated test lives in `value-fix-tools/`.

## 1. Status (honest)

- `value-retained-map` **lib compiles** (verified, `🗑️generated/value-fix-5.log`: 30 errors to 0; its `--tests` target then reported 4 errors, fixed afterwards, NOT recompiled).
- The rest of the family has **no compile verification** after the retained-map pass: the coordinator paused every `bun repo cargo` run (dependency sync starved by concurrent `bun repo` processes) and the sync was still running at 00:24. Everything below the "static" heading is checked only by the rs-sweep path resolver, `rustfmt` syntax, and the crate-boundaries law.
- Pass/error count: retained-map lib 30 -> 0; retained-map tests 4 -> (fix applied, not re-run); all other crates: not compiled.
- Teaching proctor check (`check --manifest-path 🎓️teaching/Cargo.toml`): NOT run.

## 2. Fixes

### Compiled (retained-map pass, fw16 errors)
1. `🧬️retained-clone/🗺️ordered-map/🦀️.rs`: the `use super::{...}` list gained `nested_retained_clone_{close_demands,demands,grant}`, `retained_clone_close_demands`, `retained_clone_grant_admits` (the cursor submodules reach them as `super::name`; the glue lost them when the monolith module became a crate).
2. `🔗️source/📦️packages/🦀️rust/🦀️.rs`: `RetainedCloneBorrowAuthority::next_take_demands(body)` composes the four existing `next_take_*_demand` quotes into one `RetirementDemand` (the K-vs-usize errors were cascades of this missing method).
3. `➕️insert-cursor/🦀️.rs`: inherent `close_demands` is `pub(super)` (the `edit_owner!` macro in the sibling `♻️retirement` calls it).
4. Manifest: `value-snapshot-read` carried two unused dev-dependencies (`artifact-reference`, `os-io`) added by the rs-sweep dependency fixer; `os-io` belongs to the root workspace and made every framework cargo invocation die with "member of the wrong workspace" (it also killed rg-core's fw17 pass). Removed twice; rs-sweep confirmed it will not re-add (it now has a workspace-boundary guard).

### Static (not compiled yet)
5. `🧪️tests/🔬️unit` of retained-map: `crate::value::observe_retirement_allocations` -> `crate::observe_retirement_allocations`.
6. `🧮️scalar/🧪️tests/🦀️.rs` (value-retained-clone tests, 3 E0061): `advance(source, body, grant)` takes the body bytes; the three calls pass `normal.maximum_copy_bytes`.
7. `value-tests` (companion) `♻️retirement/🧪️tests/🔬️unit`: 10 `value::observe_retirement_allocations` paths -> `crate::observe_retirement_allocations`.
8. `value-edit` named-root receive test: `crate::native_flat_record_receive!` -> `semio_framework_value_native_owned::native_flat_record_receive!` (the macro's `$crate` is native-owned; the two remaining `crate::` call sites are inside native-owned itself and are valid there).
9. `value-factory` owned tests: `crate::retained_clone::{RetainedCloneSource,..}` -> `semio_framework_value_clone_source::{..}` (declared dev-dependency).
10. `value-tests-2`: `crate::native_decoding::NativeDecodeProgress` -> `crate::NativeDecodeProgress`.
11. `value-snapshot-read`: the test `registered_owned_serializer_rejections_preserve_exact_read_and_options` needs `os-io` (a layer above; os-io depends on os-store-read which depends on snapshot-read) and cannot compile inside the value crate. It is removed from the mounted file `os/store/🔗️read/🧾️return/🧪️tests/🦀️.rs` and preserved verbatim in `value-fix-tools/owned-serializer-rejections.rs.txt`. OPEN: it must be re-mounted in an `os-io` (or higher) test closure by the owner of that layer; no coverage was silently dropped, the text is kept.
12. `mutation-diff`: independence law (`forbiddenDependencies` includes `semio-framework-value`) was violated by a staged manifest line and 22 `semio_framework_value::` paths in its source; now `semio_framework_value_intrinsic::` plus `semio_framework_value_clone_source::RetainedCloneRef`, manifest dependency on the facade removed and `value-clone-source` added.

### Derive expansion no longer needs facade re-exports (coordinator scope)
13. `value-derive-expansion` `⚙️expansion/🦀️.rs`: new container keys `#[value(native_owned = "path")]` and `#[value(persistent = "path")]`. Resolution: explicit key, else the authored `crate` root (a custom root supplies everything, as in the tests), else `::semio_framework_value_native_owned` / `::semio_framework_value_persistent`. New `declared_persistent_crate`.
14. `🛬️receive` (struct and enum expansion): every `#value::native_decoding::...` (47 sites) is now `#native::native_decoding::...`; the generated carrier types receive `native_owned=` so nested derives keep the same owner root.
15. `value-derive` `🟰️equality`: `RetainedEquality` derive accepts `#[value(...)]`; `#v::RetainedEquality{,Cursor,Fields,Step}`, `#v::RetainedRecordEquality` and `#v::retained_equality::equality_box` (24 sites) now point at `#persistent::retained_equality`, the generic bound too.
16. Facade `🧰️framework/🔨️modules/🌱️value/🦀️.rs`: removed `native_decoding`, `NativeReceive` (trait), `retained_equality` and the four `RetainedEquality*`/`RetainedRecordEquality` re-exports. `NativeDecodeControl` and `NativeDecodeProgress` stay as plain re-exports from `value-intrinsic` (they are part of the `FromValue` trait signature, so every `FromValue` derive names them through the value root like `DslValue`); this is a deliberate deviation from "delete the seven flat names": 5 of the 7 are gone, 2 are `FromValue` contract types. The derive macros `NativeReceive` / `RetainedEquality` stay exported by the facade package (macro namespace only).
17. Consumers: `energy-engine-calendar` and `energy-engine-air-exchange` gained `semio-framework-value-native-owned = { workspace = true }` (the other derive users already had it: energy model/schedule/snapshot, artifact-reference, os presence snapshot, os store child).
18. `value-native-owned` lib: `#[cfg(test)]` re-exports of the derive macros and the intrinsic names that `#[value(crate="crate")]` expansion needs (`NativeReceive`, `RetireOwned`, `FromValue`, `ToValue`, `ValueEdit`, `ValueShape`, `DecodedValue`, `NativeEncodeControl`).
19. Law fixture `crate-boundaries/🧬️cuts.json`: the two rows that named the removed facade lines are deleted.

## 3. Verification

- `bun repo cargo ... check -p value-retained-map -p value-retained-clone --lib --tests` (log `🗑️generated/value-fix-5.log`): lib of retained-map compiled (30 -> 0); retained-map `lib test` reported 4 errors (item 5); the run ended with `Original Cargo workspace topology changed` (a peer edited `🧰️framework/Cargo.toml`), so retained-clone was not reported.
- `bun C/rs-sweep-tools/main.ts --pkg <35 family crates>`: 13 findings (items 7-10) -> 0 after the fixes (last run: 2 findings, fixed after).
- `rustfmt +nightly-2026-07-20 --check --config skip_children=true,disable_all_formatting=true` over the 16 edited Rust files: parse OK.
- `bun test ./🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts`: 126 pass, 1 fail. The failure (`os-store-one-item-batch` test closure 335,667 bytes against 262,144) is not in the value family and pre-dates this task.
- Not run: any compile after item 5, all other value-family crates, wasm32, teaching proctor.

## 4. Defect found in the build tool (frozen library, not touched)
`bun repo cargo ... check` with all 36 family packages in one invocation dies with `Original compiler pressure owner required` thrown by `recoverCompactCompilerHeadroomV1` (`⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/♻️recovery/🟦️.ts:15`): the check `original.control.signal!==control.signal` fails on the capacity-recovery path. Smaller invocations avoid the path. Post-freeze fix needed. Also: the source-admission phase alone took more than 590 s for 43 units under load 40, so family checks must run detached with a long alarm (`🗑️generated/value-fix-run.sh <tag> <packages...>`, `ALARM=3300`).

## 5. Next command (after the sync exits 0)
`ALARM=3300 nohup 🗑️generated/value-fix-run.sh 9 <packages in 🗑️generated/value-fix-pkgs.txt> &` in groups of at most ~8 packages (retained-map, retained-clone, snapshot-read, persistent, registry, schema first); then the same for `value-native-owned`, `value-edit`, `value-native-decoding-tests`, `value-derive`, `value-derive-expansion`; then the `rg-core` downstream list (pack-json, tool-run, replication, job, job-payload); then `check --manifest-path 🎓️teaching/Cargo.toml -p <proctor crates>`. Expect new errors in the derive-using crates because item 13-18 changed what `NativeReceive` / `RetainedEquality` expand to.
