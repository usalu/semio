# Cargo workspace tests under `repo test-file`: toolchain selection

## Cause
- `registeredToolExecutableV1("cargo")` resolves `~/.cargo/bin/cargo`, the rustup shim (registry `cargo` entry: `SEMIO_TOOL_CARGO`, then `~/.cargo/bin`).
- The shim picks the channel from the root `rust-toolchain.toml` (`nightly-2026-07-20`) only when the cwd is inside the checkout. The cargo workspace tests spawn it in scratch roots under `SEMIO_TEST_ARTIFACT_DIR`, so rustup found no override and no default: "rustup could not choose a version of cargo to run".
- The suite route (`runProcessTestSuiteV1`, used by `repo test`/project targets) already adds `pinnedRustToolchainV1(...).environment` (`RUSTUP_TOOLCHAIN` read from the registered `cargo` pin). The provider route of `repo test-file` (`runProvidedFileV1`, `test-file-cargo-preparation-storage`) only added the preparation storage, so it lacked the pin.

## Fix
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔬️test-file/🏃️run/🟦️.ts`: `runProvidedFileV1` now also passes `pinnedRustToolchainV1(undefined, environment).environment`. One source (registry pin), same as the suite route; no change to `~/.rustup`; an incoming disagreeing `RUSTUP_TOOLCHAIN` is refused by the existing function.

## Results (`SEMIO_LANGUAGE=en repo test-file`, logs in `🗑️generated/cargo-tf-N.log`)
| File | Result |
|---|---|
| `🧪️tests/🟦️.ts` | 20 pass, 0 fail (was 7 fail) |
| `🧪️tests/🪆️nested-owners/🟦️.ts` | 4 pass (the unknown-package message failure seen earlier is gone; concurrent edit) |
| `📤️arguments/🧪️tests/🟦️.ts` | 6 pass |
| `🛠️preparation/🔁️replan/🧪️tests/🟦️.ts` | 9 pass |
| `🛠️preparation/🧪️tests/📥️original-control/🟦️.ts` | 13 fail, NOT toolchain related |

The package-scoped workspace selection change (`cargo-package-ambiguity.md`) causes no failure in these files.

## Remaining failure: original-control
Pre-existing routing mismatch, not caused by the toolchain or the selection change. The file is run by the project target `original-control-contract-check` (`📜️script.ts`), which spawns it as a child of a live Original process invocation with `cargoContractEnvironmentV1("cargo-original-control-contract")` (private preparation storage). Under `repo test-file` it runs plain with the checkout's live storage (`providedTestEnvironmentV1`), so it gets "Live Original process issuance required" and "A test reached the live repository store of the checkout". Fix direction (not done): route that file in the test-file table to its project target (or give the provider the private contract environment), as a separate ticket.
