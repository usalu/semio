# Framework Test Companions (slice FRAMEWORK-TEST-COMPANIONS, prefix `ftc`)

Executor: Sonnet, foreground, no sub-agents, no git write, no ticket tool. Tools: `ftc-tools/` (`ftc-census.ts` = copy of `tcc-census.ts` with `EXTRA_MANIFESTS` and its own output prefix; `ftc.py` = snapshot / oracle / test-name multiset / input hashes; `ftcpos.py` + `ftcunion.py` + `ftcstub.py` + `ftclib.py` = fork of the POS union oracle with a baseline taken from a pre-edit snapshot and a `libNoTest` mode; `hooks.py`, `cutmounts.py`, `mkroot.py`, `mkcargo.py`, `reg.py`, `embedrt.py`, `privscan.py`, `fieldscan.py`, `autopriv.py`, `privloop.sh` = edit and privacy helpers; `dupmounts.py` and `finalcheck.py` = the final scans). Logs and data: `🗑️generated/ftc-*` (not deleted).

## 0. Verdict

COMPLETE for all twelve targets, statically verified. 12 crates, 11 new companion crates. Every touched crate and companion is inside the hard limits (4000 active lines, 32 files, 262144 bytes); `overTestUnitsNative` is 0 for all 23 crates. The test-name multiset is identical for every crate. Every moved embedded input is byte-identical. The union oracle reports 0 new privacy, resolve, type or other errors for every family. Duplicate-mount scan: 0 new.

NOT verified by a real build: no `cargo` run was possible. The launcher stops in `retireInactiveCargoStoreV1` / `inspectPersistentReceiver` (`Exact canonical persistent Native publication required`, probe `🗑️generated/ftc-cargo-probe-1.log`). The `Cargo.lock` files are not refreshed. The first compile window must start with the commands in section 6. Privacy completeness for the typeck-blocked families (see section 5) rests on definition scans, not on rustc.

## 1. Method

- Companion layout (precedent: `📡️spr/🧪️tests/🥣️laws-companion`): `<module>/🧪️tests/🥣️companion/🦀️.rs` (`#![cfg(test)]`, `pub use <lib>::*`, mounts the existing test files with `#[path]`, so the test files do not move and the TS source readers keep working) plus `<module>/🧪️tests/🥣️companion/📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`. A private module's tests mount under a wrapper file per module (a namespace with the glob of the public API plus the restated private imports), so `use super::*` in the moved test files still resolves. Tests that define their own `mod tests { use super::* }` are pulled in by `include!`. The parent loses its `#[cfg(test)]` mounts and its dev-dependencies, which move to the companion.
- Embedded inputs: `include_str!`/`include_bytes!` in the tests became run-time reads from the product directory (`CARGO_MANIFEST_DIR`-relative `text()/bytes()` helper, leak-and-cache). Fixtures did not move.
- Hooks: private items that a moved test needs became `#[doc(hidden)] pub` (fields, methods, one enum), or forwarders / type aliases inside a `#[cfg(feature = "testkit")] #[doc(hidden)] pub mod testkit`. No compatibility aliases. `cfg(test)`-gated production code (async, value read lease, ui-host browser seam) became `feature = "testkit"` gating.
- Oracle: the union oracle builds the original crate and the companion into one crate; `--test` applies to the companion only (the lib is built without `cfg(test)`, as cargo does for a dependency); the baseline is a pre-edit snapshot of the closure (`scratchpad/ftc-snap/<key>`, which includes dependency crates of the family). Stub noise cancels because both sides see the same externals.

## 2. Results per crate (native lib-test unit: files / bytes / active lines; census tool `ftc-tools/ftc-census.ts`, final run `🗑️generated/ftc-census-final.tsv`)

| Crate | Before | After (lib) | Companion(s) |
| --- | --- | --- | --- |
| `semio-framework-io-base64` (declared 4/32768/1024) | 7 / 47,936 / 755 | 3 / 18,179 / 337 | `semio-framework-io-base64-tests` 5 / 30,087 / 425 |
| `semio-framework-print` | 52 / 2,575,613 / 1,733 | 22 / 139,328 / 1,210 | `semio-framework-print-tests` 4 / 78,720 / 555 |
| `semio-framework-server` | 20 / 353,837 / 6,915 | 14 / 165,448 / 3,440 | `semio-framework-server-tests` 7 / 168,744 / 3,173 |
| `semio-framework-quiz` | 20 / 408,357 / 6,695 | 11 / 136,925 / 2,641 | `semio-framework-quiz-tests` 15 / 250,510 / 3,740 and `semio-framework-quiz-primitives-tests` 5 / 25,157 / 416 |
| `semio-framework-pack-json` | 42 / 477,267 / 5,758 | 9 / 228,027 / 3,381 | `semio-framework-pack-json-tests` 16 / 216,493 / 2,459 |
| `semio-framework-mesh-engine` | 25 / 369,219 / 2,718 | 16 / 200,664 / 2,745 (inputs read at run time) | none |
| `semio-framework-artifact-space-collection` | 35 / 636,836 / 2,127 | 26 / 185,307 / 1,948 (inputs read at run time) | none |
| `semio-framework-pixels` | 48 / 735,478 / 4,351 | 13 / 203,607 / 3,204 | `semio-framework-pixels-tests` 24 / 108,383 / 1,292 |
| `semio-framework-artifact-workflow-run` | 60 / 202,547 / 2,363 | 29 / 96,203 / 1,417 | `semio-framework-artifact-workflow-run-tests` 14 / 66,670 / 688 |
| `semio-framework-async` | 55 / 343,284 / 6,299 | 24 / 162,136 / 3,332 | `semio-framework-async-tests` 20 / 122,050 / 2,205 and `semio-framework-async-wasm-pool-tests` 5 / 9,645 / 207 |
| `semio-framework-ui-host` | 27 / 263,615 / 4,610 | 13 / 157,261 / 2,951 | `semio-framework-ui-host-tests` 14 / 98,026 / 1,616 |
| `semio-framework-value` (added by the coordinator) | 77 / 538,035 / 5,347 | 25 / 186,049 / 2,766 | `semio-framework-value-tests` 27 / 248,006 / 2,683 |

Tightest remaining units (ratio to the binding limit): `quiz-tests` 0.956 (bytes), `value-tests` 0.946 (bytes), `workflow-run` lib 0.906 (files), `pack-json` lib 0.870 (bytes), `server` lib 0.860 (lines), `async` lib 0.833 (lines). Under the `testkit` feature the `value` lib closure is 27 files / 190,329 B / 2,820 lines (`featclosure.ts`); the `ui-host` lib is 7 files / 132,059 B / 2,737 lines. The wasm labels are equal or smaller.

Residual in-lib test code, documented on purpose:
- `ui-host`: the lib still mounts `📥️enqueue`'s `input_admission_tests` and `input_root_tests` (and the lib copy of the observed allocator). The input root has a `cfg(test)` production hook (`interfere_after_load`), so moving it needs another feature conversion. The unit is at 74% of the line limit, so this is inside budget. The window-unit laws (21 tests) did move: the `cfg(test)` browser seam in `🪟️window` became `feature = "testkit"` and the seam module `linear_memory_test_import` is `#[doc(hidden)] pub`.
- `async`: the cooperative standalone test stays reachable from `🤝️cooperative/🦀️.rs` (test-only include).
- `mesh-engine`, `artifact-space-collection`: the brief allowed the embedded-input move alone; both land well inside the limits.

Foreign, not mine: `semio-framework-print-tests` carries a dev-dependency on `semio-framework-plugin`, whose closure is 125 files / 1,979,150 B (census row `devdep-lib`, 7.55x). The same dependency is already a normal dependency of `semio-framework-print` at HEAD, and `chart-mutations` uses it; the plugin slice owns that closure. `csvMismatch 20` in the census summary compares against the old `crate-census.csv` snapshot, which the peers' edits have outdated.

## 3. Defects found on the way

- `ui-host` window-unit used `semio_framework_job_worker` and `semio_framework_job_worker_authority`, which the HEAD manifest of `semio-framework-ui-host` did not list (the HEAD test could not have compiled). The companion manifest now lists them as dev-dependencies (with `semio-framework-job`, `-value`, `-async`, `ui_contract`). The oracle's baseline reports those as 8 unresolved-crate errors; the current side reports 8 stub-gap errors for the same sites (the stub of the authority crate lacks `WorkerJobInitializer` members and `BatchDriveConfig`, which the real crate defines at `🧵️job/👷️worker/🎟️authority/🦀️.rs:65,102`). They cancel 1:1 and are not real.
- 50 touched files contain `[DEBUG]` strings; every one of them is already in HEAD (assertion logs of the tests and one script), none was added by me.

## 4. Verification (static, Wave 6)

| Check | Result | Evidence |
| --- | --- | --- |
| Census, hard limits | 23 crates, 2,866 units, 0 error units, `overTestUnitsAllLabels` 0, `overTestUnitsNative` 0 | `🗑️generated/ftc-census-final.tsv`, `ftc-census-summary.json` (workflow-run lib counted separately: 29 / 96,203 / 1,417) |
| Test-name multiset | IDENTICAL for io-base64 13, print 39, server 157 (with the 5 dependency crates of its snapshot), quiz 192 (with `quiz-schema`), pack-json 102, mesh 46, space-collection 29, pixels 72, workflow-run 44, async 95, ui-host 73, value 81 | `🗑️generated/ftc-final-names-hashes.log` |
| Moved inputs byte-identical | 0 changed or missing of 3 + 29 + 1 + 0 + 22 + 10 + 9 + 23 + 17 + 13 + 11 + 40 inputs (sha256 of every non-`.rs` file of the baseline closure against the live path) | same log |
| Union oracle (`--cfgs test`, label `final`, ui-host `final2`) | NEW privacy 0, resolve 0, type 0, other 0 for iob64, mesh, spacecoll, print, server, quiz, packjson, pixels, wfrun, async, value; `NEW 0 / GONE 0` in all of them. ui-host: NEW 8 / GONE 8 = the stub gaps of section 3 | `🗑️generated/ftc-final-<family>.log`, `ftc-pos-<family>-final-result.json` |
| Duplicate mounts | 23 crates, 322 code files: 0 new. One pre-existing double edge (`📏️preflight/🫳️borrowed` in space-collection, also in the baseline snapshot) | `ftc-tools/dupmounts.py` |
| Touched-file scan (257 files) | 0 paths over 240 bytes, 0 `rustc -Zunpretty=normal` parse errors over all `.rs` files, 70 `.rs` files new to HEAD all start with an emoji docstring | `ftc-tools/finalcheck.py` |
| Taxonomy | 93 directories new to HEAD (`🥣️companion`, wrapper folders, `🪝️testkit`, `🧰️support`, ...): 0 unresolved, 0 ambiguous | `frs-tools/taxcheck.ts`, `🗑️generated/ftc-newdirs.txt` |
| Registration | members in `🧰️framework/Cargo.toml`, `📋️build-owners.json` rows, `📋️project.json` per companion, 6 launch routes per companion in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` in the existing order, parent `📜️script.ts` test lists repointed to the companions | `ftc-tools/reg.py`, `manifestcheck.ts` |

## 5. Hooks and oracle limits

`testkit` features: `server` (also `conformance`), `quiz`, `pack-json`, `pixels`, `async` (also `entrypoint`, `wasm-pool-host`), `ui-host`, `value`. Print, io-base64, mesh, space-collection and workflow-run needed no feature. All hooks are `#[doc(hidden)]`; plans: `ftc-tools/plans/*-kit.json`. `ui-host`'s `testkit` swaps the wasm linear-memory import for the in-process seam (test-only; do not enable it for a wasm release build). The `async` and `value` `testkit` features replace former `cfg(test)` production arms.

The stub-layer oracle could not reach rustc's typeck (privacy codes E0616/E0624/E0603) for pack-json, server, ui-host and value (stub gaps in winit, taffy and the serde macros, plus the authority crate above). For these four, privacy was checked by definition scans (`privscan.py`, `fieldscan.py`) and name-based widening. A real compile can still find a private field or method I missed there; the fix is a one-line `#[doc(hidden)] pub`.

## 6. Real checks for the first compile window

```
bun repo cargo --store=shared -- update --workspace --offline
bun repo cargo --store=shared -- check --offline --tests -p semio-framework-io-base64 -p semio-framework-io-base64-tests -p semio-framework-print -p semio-framework-print-tests -p semio-framework-server -p semio-framework-server-tests -p semio-framework-quiz -p semio-framework-quiz-tests -p semio-framework-quiz-primitives-tests -p semio-framework-pack-json -p semio-framework-pack-json-tests -p semio-framework-mesh-engine -p semio-framework-artifact-space-collection -p semio-framework-pixels -p semio-framework-pixels-tests -p semio-framework-artifact-workflow-run -p semio-framework-artifact-workflow-run-tests -p semio-framework-async -p semio-framework-async-tests -p semio-framework-async-wasm-pool-tests -p semio-framework-ui-host -p semio-framework-ui-host-tests -p semio-framework-value -p semio-framework-value-tests
bun repo cargo --store=shared -- test --offline -p <each companion above> -- --test-threads=1
bun tcc-tools/tcc-census.ts   # expect overTestUnitsNative 0
```

Then compare `cargo test -- --list` per companion with the multisets in `🗑️generated/ftc-final-names-hashes.log`. Per-crate script routes (`📜️script.ts test ...`) run both the lib and its companion packages.

## 7. Files

All touched files are listed in `🗑️generated/ftc-touched.txt` (257 existing paths; two listed paths were removed or renamed by peers). Main groups: the twelve parent crates' `🦀️.rs` mount cuts and hook widenings, `Cargo.toml`, `📜️script.ts`; their moved test files (embed edits only); the new companion trees `🧪️tests/🥣️companion` (also `🍰️companion-primitives`, `🍰️companion-wasm-pool`) with wrappers; `🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts` (new `testPackages` option); the registries named above.
