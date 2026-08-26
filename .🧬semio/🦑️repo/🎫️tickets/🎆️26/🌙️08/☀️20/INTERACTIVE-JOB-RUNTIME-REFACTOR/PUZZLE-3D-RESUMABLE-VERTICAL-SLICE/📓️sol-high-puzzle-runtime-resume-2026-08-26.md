# Puzzle Retained Runtime Resume — 2026-08-26

## Result

Native production-library compilation passed once in the isolated ticket target after disabling incremental compilation, and the earlier `wit-bindgen`/`bitflags` `E0463` cache failure did not recur. No Puzzle retained operation reached dispatch or runtime execution in this session. The external exact-dispatch harness remained blocked by shared dependency compilation, first by six transient framework-builder `E0277` errors and, after that repair, by `semio-s-plugin-stdio`. A stdio migration reduced its error count from 3,859 to 2,299, but did not restore compilation. Consequently cancellation, stale/ABA generations, `max + 1`, close, replay, browser/Wasm, and the authoritative 8 ms gate remain unaccepted.

The static registration/source census remains 99 routes: Puzzle 2D 3, Puzzle 3D 53, Puzzle 5D 43. This is source and fixture evidence only, not runtime acceptance.

## Scope and Safety

- Reused `PUZZLE-3D-RESUMABLE-VERTICAL-SLICE`; no ticket was opened, closed, or reopened.
- Used only ticket-local build output and harness files.
- Used `CARGO_INCREMENTAL=0` and `🧪️target-puzzle-runtime-resume`.
- Did not use the coordinator-owned `target-p0-current`.
- Did not run modifying Git commands or create a worktree.
- Did not edit the concurrently maintained Puzzle oracle ledgers.

## Native Commands and Evidence

### Filtered unit-test attempt

Command:

```text
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<ticket>/🧪️target-puzzle-runtime-resume cargo test -p semio-s-plugin-puzzle language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle -- --exact --nocapture
```

Result: exit 101 before executing any test. The Puzzle crate's entire `cfg(test)` tree failed with 336 pre-existing compile errors. Representative failures included missing `std::time::Duration`, `.await` in synchronous testkit functions (`E0728`), references to removed `BoardHost::drain_events_json`, and mutation through `Arc`. Runtime assertions executed: zero.

### Production library check

Command:

```text
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<ticket>/🧪️target-puzzle-runtime-resume cargo check -p semio-s-plugin-puzzle --lib
```

Result at the time of execution: exit 0, `Finished dev profile` in 8 minutes 33 seconds, with 157 warnings. This crossed both `bitflags` and `wit-bindgen` successfully and shows the previous `E0463` was not reproduced in the isolated non-incremental target. It is compilation evidence, not retained-runtime evidence.

### Ticket-local external runtime harness

The harness uses the production `create_puzzle2d_app`, `create_puzzle3d_app`, and `create_puzzle5d_app` registries and calls the app-owned `VcsArtifactApp::dispatch_typed`. Its intended checks are:

- exact Puzzle 2D `ForceLayout`, Puzzle 3D `SetGridVisible`, and Puzzle 5D `SetGridSnapEnabled` dispatch;
- production `PluginApp::maintenance_step` and typed publication;
- wrong-generation result ACK rejection followed by exact-token ACK acceptance;
- terminal/fault result-page arrival;
- Puzzle 2D replay lane equality;
- immediate cancellation through incremental close to terminal-empty for all three dimensions.

The first harness compile reached the harness crate after compiling Puzzle production code, then failed only on a ticket-local import (`E0432` for `TypedOperationResultLane`). The import was corrected to `semio_framework_plugin::plugin_app_close_prelude::TypedOperationResultLane`.

The next attempt was blocked by six shared `semio-framework-plugin` builder `E0277` errors: `resolve_ready(E::app_schema())` was applied after `app_schema()` became synchronous and returned `Option<AppSchemaDescriptor>`. That shared issue was repaired by its owner.

The resumed attempt then failed before linking or running the harness because Puzzle's unconditional production dependency `semio-s-plugin-stdio` produced 3,859 compile errors (`E0053` and `E0277` were reported). Representative `E0053` diagnostics say stdio viewer implementations still return futures for `initial_snapshot`, `handle`, and `render`, while the current traits expect synchronous `Snapshot`, `Result<ViewEmit<_>, _>`, and `Result<ComponentTree, _>` values. This occurs across XML, STEP, STL, PDF, IFC, XLSX, and semio-kit viewers. A dedicated stdio repair pass changed 352 files and completed its report-package check attempt; the immediate post-repair harness retry still failed with 2,299 stdio errors of the same classes. Stdio cannot be omitted without changing the exact production dependency graph. Harness runtime records emitted: zero. Exact Puzzle dispatch calls completed: zero.

## Fixture Ledger Integrity

A Bun/JSON integrity check passed for all three retained ledgers. It checked exact current tool/vector counts, unique tool and vector IDs, the shared ordered 16-vector boundary prefix and its fingerprints, a valid production-catalog tool reference for every added domain vector, capacities, and the exact locale key set `de,en`.

```text
[DEBUG] puzzle2d fixture-parity tools=3 vectors=25 base=16 refs=accepted
[DEBUG] puzzle3d fixture-parity tools=53 vectors=107 base=16 refs=accepted
[DEBUG] puzzle5d fixture-parity tools=43 vectors=72 base=16 refs=accepted
```

Shared ordered prefix:

```text
zero, max, maxPlusOne, malformed, staleGeneration, wrongOperation,
abaGeneration, cancelWirePage, cancelWireByte, cancelPreflight,
cancelWork, cancelPublish, faultWork, retry, close, replay
```

Declared capacities matched in every ledger:

```text
rawBytes=8192 decodedItems=512 workItems=4096 outputBytes=262144
stepMicros=7500 semanticUnitsPerGrant=1
```

Ledger SHA-256 values at check time:

```text
puzzle2d 6eb7824e4d0dfe0716a7c47c064163d596eb2996f843e239b33bfee2b6e53775
puzzle3d f8cf73657fb1e57f1622b07383a73bbdad2b0fbf4249cc87f151b2cc0cfadaad
puzzle5d 68dfbdcf13916afd2fa6748f0443c1e14f3acc6a94edfd2e9d6ed7046d81a407
```

This validates ledger structure and identity only. It does not execute any fixture vector. The inline generic fixture oracle was updated to compare the canonical 16-vector prefix and its fingerprints while validating unique identities and production tool references over every expanded vector. The independent Bun check passes that contract. The Rust oracle remains unexecuted because the shared stdio dependency blocks compilation.

## Acceptance Matrix

| Evidence | Status | Boundary |
| --- | --- | --- |
| Static route census | 99/99 present | Not runtime acceptance |
| Retained ledger integrity | Passed with Bun | Vectors not executed |
| Isolated native production lib check | Passed once | Before later shared-source churn |
| Exact production Puzzle dispatch | Not reached | Blocked compiling stdio |
| Publication and result ACK | Not run | Harness did not execute |
| Cancellation and incremental close | Not run | Harness did not execute |
| Stale/wrong-operation/ABA | Not run | Fixture/source evidence only |
| `max + 1` rejection | Not run | Fixture/source evidence only |
| Replay determinism | Not run | Harness did not execute |
| Native timing | Not accepted | Harness timing records not emitted |
| Wasm/browser runtime | Not run | No claim |
| Authoritative 8 ms worker gate | Not run | No claim |

## Exact Remaining Blocker

The current exact-production external harness cannot compile through Puzzle's unconditional `semio-s-plugin-stdio` dependency, which reports 2,299 errors after the latest shared migration pass. Until stdio compiles on the same shared head, no truthful retained-session runtime acceptance can be produced from this harness. Once repaired, rerun the warm isolated harness first for the single Puzzle 2D exact-dispatch/terminal assertion, then extend acceptance to replay, cancellation/close, 3D, 5D, boundary vectors, Wasm/browser, and the authoritative timing gate.

## Ticket-Local Files

- `🧪️runtime-harness/Cargo.toml`
- `🧪️runtime-harness/Cargo.lock` (Cargo-generated)
- `🧪️runtime-harness/src/main.rs`
- `📓️sol-high-puzzle-runtime-resume-2026-08-26.md`
- `🧪️target-puzzle-runtime-resume/` build output retained in the ticket
- `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️component.rs` fixture-oracle expansion fix
