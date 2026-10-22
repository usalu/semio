# Cross-Target Standard Library Readiness

2026-10-21. Bounded static inspection only during lock 37397/source freeze. No commands, setup, Cargo, Source/Native products, private stores or toolchain directories were inspected or executed. Prior E0463 in Native filesystem check 30084 is supplied context, not a newly reproduced result.

## Decision

No currently inspected owned command provisions x86_64-pc-windows-msvc rust-std on a macOS host. Do not retry the same Windows-target Native check expecting `repo tools setup all` to fix it. Raw rustup is prohibited and would also violate the provider exact-generation contract.

## Existing Owned Boundaries

`repo tools setup [nx|bazel|all]` is the documented setup route. Actual parser `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts:164-178` accepts at most three words, limits its selector to nx/bazel/all, prepares the host Bazel distribution/modules and/or invokes the owned Nx bootstrap --version. It has no requested Rust target option. Calling it host-only understates its actual scope: it is build-backend setup, not a general target stdlib installer. Host Rust provisioning can occur in owned tool/compiler preparation, but does not widen its pinned target set.

`rust-toolchain.toml` pins nightly-2026-07-20, minimal profile, rust-src/llvm-tools-preview and exactly wasm32-unknown-unknown plus wasm32-wasip2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/📥️installation/🟦️.ts:42` reads that file and explicitly rejects any different components/target sets even though the generic JSON schema admits string arrays. Therefore simply appending the Windows triple to TOML would fail the provider contract.

The same installation owner at lines 44,49,55-56 downloads host rustc/cargo/rust-std plus declared target rust-std archives, seals the complete generation, and validates exact installed components and targets. Its host comes from the bootstrap provider using actual process.platform/process.arch (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/📥️bootstrap/🟦️.ts:20,35`). On macOS the host is an apple-darwin triple. Windows MSVC std is absent from the permitted extra targets. Directly adding it into a sealed generation is not an authorized recovery.

A second exact boundary is `repositoryRustCompilerIdentityV1` in installation line 33: accepted compiler targets are only the retained compiler host, wasm32-unknown-unknown and wasm32-wasip2. Thus support needs both a provisioning/request-contract change and admitted compiler identity target support, not a shell alias to rustup.

`repo cargo --store=shared --configuration=prototype -- check ... --target x86_64-pc-windows-msvc` is the owned consumption shape in the build skill. CargoRelayScript (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts:29-39`) forwards arguments through withRepositoryCargoInputsV1 and compiler-store custody. Build intent target validation/projection (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/🟦️.ts:19-31`) accepts a syntactically valid target and emits --target, but neither is a provisioning interface. Accepting a target argument is not evidence that its stdlib is installed or admitted.

## Next Native Plan Boundary

After lock/sync/cold, run the already authorized macOS Native owners first. Treat Windows-target local checks as blocked on the bounded provisioning/admission gap unless the owner intentionally implements and validates that capability after freeze. No existing target-install command was found in the inspected setup/Cargo contracts; no new feature was implemented or proposed for immediate execution.

An eventual macOS cross-check with Windows stdlib would establish only Windows-target compilation of the selected scope. It would not execute on Windows, prove the Windows process/filesystem behavior, or establish cross-platform test success. Actual Windows-host execution remains separate evidence.
