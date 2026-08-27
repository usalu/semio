# Source Authority Independent Review

## Executed Boundary

The registered derive crate first failed before test execution with E0382, a moved `workspace` value in fixture setup (`🧪️source-authority-registered-red.log`). Root review also rejected raw parent reduction before no-follow checks, ancestor marker traversal, permanent ticket-path arithmetic, and an invented fourteen-key fixture unrelated to the required metadata schema.

After the executor corrected those items, the coordinator compiled the unchanged `MutationSourceAuthority` production region into an isolated executable using the existing paired serde_json compiler artifacts. Seven independent filesystem cases passed: direct source, valid relative parent mount, symlink-parent erasure rejection, workspace-ancestor symlink rejection, and three malformed inner-workspace anchor cases. Evidence: `🧪️source-authority-independent/🧪️root-first.log`, retained `🧫️run-HrBOI7`. Region hash: `1ffa472e58c98b4d983fc8da22c88d27287c0f3887c2f2895e309b34611eda79`; full source stayed unchanged during the run.

An eighth adversarial case then reproduced a real remaining bug: `not-a-directory/../domain/🧬️mutations/➕️insert-page/🦀️.rs` was accepted when `not-a-directory` was a regular file. A valid filesystem path cannot traverse a regular file before `..`. The raw walker checked symlink status but not intermediate directory type, then normalization erased the invalid traversal. Evidence: `🧪️source-authority-independent/🧪️file-parent-red.log`, retained `🧫️run-IGki5l`; eight cases, one mismatch. The executor owns the correction and permanent neutral regression.

## Corrected Independent and Registered Replay

The executor corrected the raw walker to require every intermediate normal component to be a directory before reducing parent segments. The eighth neutral regression also compares native filesystem behavior; a separate Node read of the raw file-parent path returned `ENOTDIR` (`🧪️source-authority-independent/🧪️native-file-parent-oracle.log`).

The final unchanged-production-region replay passed all eight independent cases, zero mismatches, exit0. Evidence: `🧪️source-authority-independent/🧪️root-final.log`, retained `🧫️run-1KYmyR/🔣️results.json`; source unchanged=true, region SHA256 `e300419a243eda7579e19f417e6939545f0ef6271627abfeffe0c904107c4116`.

The real registered derive crate then passed3 tests, zero skipped, exit0: the two attribute tests and the source-authority test covering18 neutral rows. Command: `SEMIO_BUILD_BUDGET_MS=3600000 SEMIO_TEST_BUDGET_MS=180000 SEMIO_TEST_ARTIFACT_DIR=<ticket>/🧪️source-authority-registered-green-artifacts bun nx run @semio-tech/dsl-derive-rs:test-quick --skip-nx-cache -- --lib --build-jobs 1`. Transcript: `🧪️source-authority-registered-green.log`; nextest artifacts: `🧪️source-authority-registered-green-artifacts/semio-nextest-Tukiwm`.

This accepts FND-SOURCE-AUTHORITY-07 as the bounded private direct-declaration source/owner authority. It validates only descriptor object/owner at this stage, not the full fourteen-field schema, public metadata derive, mandatory trait, aggregate source proof or registry cutover. Those remain required.

## Limits

The standalone replay executes exact production helper code, not a separately reimplemented approximation. It does not exercise the public procedural macro, which is not wired yet, nor replace the registered crate gate. It uses native macOS filesystem behavior; no Windows/Linux execution is claimed. Each run retains compiler input, stdout/stderr, fixtures, runtime output and structured results. No actual `compose/**` path is read or created.

Local installed Rust API documentation independently confirms `proc_macro::Span::local_file()` is stable since Rust1.88.0: `/Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/share/doc/rust/html/proc_macro/struct.Span.html`, method `local_file`. Only newer installed compilers were executed; the minimum-version toolchain itself was not run.
