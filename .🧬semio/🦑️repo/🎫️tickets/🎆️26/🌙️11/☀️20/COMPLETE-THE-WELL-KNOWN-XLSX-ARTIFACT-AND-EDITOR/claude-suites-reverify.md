# XLSX non-browser layers: re-verification

Status as of 2026-11-25. Logs are under `🗑️generated/claude-suites/` (delete when the ticket closes).

## Results

| Layer | Item | Passed | Failed | State |
|---|---|---|---|---|
| TS (`repo test-file`) | package-routing | 40 | 0 | green |
| TS | natural-file-drop | 6 | 0 | green |
| TS | natural-file-save-name | 14 | 0 | green |
| TS | picked-file-opening | 49 | 0 | green |
| TS | format-resolution | 3 | 0 | green |
| TS script | `validate-xlsx-routing.ts` (SEMIO_LANGUAGE=en) | all checks | 0 | green (exit 0) |
| Rust | assembly `every_bundled_artifact_definition_assembles_without_violations` | 1 | 0 | green (the `semio_framework_hash` blocker is gone) |
| Rust | `semio-s-artifact-stdio-xlsx-base-mutations` | 1 | 0 | green (rust-five-8) |
| Rust | `semio-s-artifact-stdio-xlsx-strict-mutations` | 12 | 0 | green (rust-five-8) |
| Rust | `semio-s-artifact-stdio-xlsx-base` | 46 | 1 | `closes_mid_reserved_commit` failed (rust-five-8); since then blocked by the source boundary gate, not re-run |
| Rust | `semio-s-artifact-stdio-xlsx-strict` | 21 | 1 | `closes_mid_reserved_commit` failed in rust-five-8; the single test passed afterwards (mid-s3.log, 1 passed); full package not re-run |
| Rust | `semio-s-artifact-stdio-xlsx-transitional` | 28 | 1 | `closes_mid_reserved_commit` failed in rust-five-8; not re-run |
| Format claims | `check-stdio-format-claims` | - | - | not verified: runs died on session restarts and host limits |
| Oracles crate (`🔮️oracles`), `🧪️tests` | - | - | - | not run: the oracles crate is excluded from the xlsx workspace; the `🧪️tests` directories hold no Cargo target of their own |

## Root causes and fixes

1. `queries overflow the depth limit` (strict, transitional lib tests). The new `assert_closes_mid_reserved_commit` acceptance future nests deeper than the default recursion limit of 128 for the strict and transitional editors. Fix: `#![recursion_limit = "256"]` in
   - `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/📦️packages/🦀️rust/🦀️.rs`
   - `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/📦️packages/🦀️rust/🦀️.rs`
   (the repository already uses this attribute in other crates). Verified: both crates compile and run their tests.
2. `closes_mid_reserved_commit` (base, strict, transitional): "original lifecycle child receipt differs from its whole producer receipt". Instrumentation placed in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs` showed that the failure arises inside `original_lifecycle_step` during `close_registered_fixture_app`: the close step posted `released_bytes +4544` and `copied_items +1` to the context, while its returned receipt claimed only 1 item and 0 released bytes, so `original_lifecycle_unposted` refused. This is a receipt under-report by a close producer, in code that other agents were editing in the same hours (`🧵️job`, `♻️retirement`, `🔏️seal`). After their edits, the strict test passed (1 passed, mid-s3.log) with no change of mine in that logic. I did not find or make a separate fix. Confirmation for base and transitional is open. The instrumentation (two `eprintln!` sites in `⚠️failure/🦀️.rs`) is removed; the file equals the committed logic apart from the repaired closure (the committed copy had an unbalanced `{`).
3. New blocker, base only: the repo-wide Rust source boundary (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`, 262144 bytes, 32 files, 4000 active lines) rejects the base lib-test unit before the compiler ("23 of 32 files, 284484 of 262144 bytes"). Production code of base is 113865 bytes; the overage comes from tests and embedded fixtures. `measure-base.ts` (crate-tree measure, test cfg) gives 36 files, 326848 bytes: large-grid `package.xlsx` 56 KB, `canonical-xml-save/🔣️.json` 46 KB, editor unit tests 38 KB, controlled-import test 16 KB, native-document test 14 KB. strict (126 KB) and transitional (172 KB) are within the boundary. Fix (source-only, not yet compiled): the two io law modules `🚪️io/🧪️tests/🔬️native-document` and `🚪️io/🧪️tests/🔬️controlled-import` (both use only the public API) moved out of the base lib-test unit into a new integration target `base_io_laws` of `semio-s-artifact-stdio-xlsx` (`📕️xlsx/🧪️tests/🔬️base-io-laws/🦀️.rs`, registered as `[[test]]` in `📕️xlsx/📦️packages/🦀️rust/Cargo.toml`, next to `model_laws` and `mutation_laws`). Their `crate::standards`/`crate::editor` paths now read `semio_s_artifact_stdio_xlsx_base::...`; the two `#[cfg(test)] mod` lines were removed from `🚪️io/🦀️.rs`; no shims. Measured with `measure-base.ts` (crate-tree counter): base lib-test unit 30 files, 224038 bytes, 2799 lines (limits 32, 262144, 4000); new target 7 files, 103171 bytes, 576 lines. The canonical-save editor tests stay in base.

## Environment findings

- Detached runs failed or stalled repeatedly because of other agents' in-flight breaks (window-kits `scene_surface` arity, `Slot` exports, `debug_state`, `guest-contract` Debug bound), a compiler ICE in `semio-framework-pack-format` (crash dump removed), disk-image attach failures, and session restarts. Each cleared on its own.
- A host kernel panic on 2026-11-25 led to the rule "no cargo from this agent": runs go through `claude-resource-guard.sh`.

## Unverified and why

- base, strict, transitional full packages after the receipt fix: no run since the panic.
- `check-stdio-format-claims`: never finished in this session; the earlier green result (2026-11-22) predates later tree changes.
- Oracles crate: separate crate outside the workspace, not run.

## Requested run

`T/run-all-xlsx.sh` (zsh): the five packages with `--lib --no-fail-fast`, then `-p semio-s-artifact-stdio-xlsx --test base_io_laws`, then `check-stdio-format-claims`. Output in `🗑️generated/claude-suites/all-rust.log`, `all-io-laws.log`, `all-claims.log`, `all-done`. The carve has never been compiled: expect to fix import or visibility slips in the moved tests on the first run.

## Run results after the carve (coordinator's run-all)

| Item | Passed | Failed | Note |
|---|---|---|---|
| `xlsx-base` lib | 40 | 0 | `closes_mid_reserved_commit` passes; 40 tests after the two io law modules moved out |
| `xlsx-base-mutations` | 1 | 0 | |
| `xlsx-strict` lib | 22 | 1 | paged-route test; fixed since by the paged-decoder agent |
| `xlsx-strict-mutations` | 12 | 0 | |
| `xlsx-transitional` lib | 29 | 1 | paged-route test; fixed since by the paged-decoder agent |
| `check-stdio-format-claims` | - | - | passes ("stdio format claims are current") |
| `base_io_laws` | - | - | not a test failure: the host warden cancelled rustc for `semio_s_artifact_energy_model_snapshot` (4003 MB, ceiling 1536 MB) |

## Why the energy model crates were in `base_io_laws`

`semio-s-artifact-stdio-xlsx` listed `semio-s-artifact-energy-model-mutations` as a dev-dependency, and dev-dependencies are shared by every test target of a package, so `base_io_laws` built the whole energy model. The only user was `🧪️tests/🔬️model-laws/🦀️.rs`: `extern crate semio_s_artifact_energy_model_mutations as mutations;`. The xlsx schema law it includes calls `mutations::demo_mutation_cases()`, which exists in the xlsx crate `semio-s-artifact-stdio-xlsx-base-mutations` (feature `testkit`, already a dev-dependency) and nowhere in the energy tree. The alias was a wrong crate (copy of the energy artifact's setup).

Fix, no carve of the energy crate needed:
- `📕️xlsx/🧪️tests/🔬️model-laws/🦀️.rs`: the alias now names `semio_s_artifact_stdio_xlsx_base_mutations`.
- `📕️xlsx/📦️packages/🦀️rust/Cargo.toml`: dev-dependency on the energy crate removed.
- `📕️xlsx/Cargo.lock`: the dependency line removed from `semio-s-artifact-stdio-xlsx` and every package no longer reachable from the workspace members pruned (85 energy packages plus `semio-framework-math-dense` and the three `semio-s-artifact-stdio-semio-*` model crates), 718 to 629 entries. A copy of the old lock is `🗑️generated/claude-suites/Cargo.lock.before`. Not compiled: verify with the next run (`--locked` would show a mismatch).

## Next run

`T/run-paged-and-io-laws.sh`: `-p semio-s-artifact-stdio-xlsx --test base_io_laws --test model_laws`, then the strict and transitional `paged_natural_route` lib tests. Logs: `final-io-laws.log`, `final-paged.log`, `final-done`. `model_laws` is included because its alias changed; it may show errors that the energy alias had been hiding.

## Run of 18:30 (before the quick-xml fix)

| Item | Result |
|---|---|
| `base_io_laws` | did not compile: E0433 `quick_xml` (4 sites in `controlled-import`). The only crate missing; `quick-xml = "0.42"` (the base crate's own dev-dependency) added to `📕️xlsx/📦️packages/🦀️rust/Cargo.toml`. The lock entry of the aggregate crate is left to cargo to re-derive. |
| `model_laws` | not reached (same cargo command failed to compile) |
| strict `paged_natural_route_refuses_a_transitional_workbook_with_the_strict_gate_of_the_whole_buffer_decoder` | 0 passed, 1 failed |
| transitional `paged_natural_route_opens_workbooks_larger_than_one_step...`, `paged_natural_route_refuses_a_genuinely_wrong_namespace...` | 0 passed, 2 failed |

All three paged failures carry one cause: `unwrap` on `OriginalTurnFailure` "original clock Source or action changed" at the test's own paged intake call (strict `✏️editor/🧪️tests/🔬️unit/🦀️.rs:80`, transitional `:85`). This is a framework turn-clock refusal in the paged decoder, owned by the paged-decoder agent, who has since changed it; not re-run yet. `run-paged-and-io-laws.sh` runs both cargo commands independently, so the paged step ran although the first failed.
