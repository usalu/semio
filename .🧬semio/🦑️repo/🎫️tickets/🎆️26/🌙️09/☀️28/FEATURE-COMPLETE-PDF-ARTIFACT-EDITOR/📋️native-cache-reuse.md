# Native Cargo Cache Reuse Audit

The all-fixtures capture used `.cache/b/r/m/u/l/pf748d4/t-ce555540`; the samples retry used `.cache/b/r/m/u/l/pac3065/t-ce555540`. Their Cargo build directories are the respective parent lanes. Both report compiling the same early framework crates. Cargo `Compiling` lines alone do not prove actual compiler-cache misses: the shared unit cache can restore across lanes through its wrapper.

The product identity includes root, manifest, mode, command, producer, full arguments, and toolchain. The ticket notes the retry added `--no-fail-fast`, which changes that identity. A different package selection likewise changes it. The logs do not print the frontend commands, so they do not establish whether `--lib` appeared in both; warning suggestions for `cargo fix --lib` are not evidence. The test-only source edit itself is not established as causing the broad rebuild.

At inspection pf748d4 was absent and pac3065 was present. No ordinary log inspected contains an identifiable eviction record, so the responsible action and timing are unverified. Do not redirect or copy lane contents: exclusive claims and identity receipts protect custody. Repeat the exact command and toolchain to select the same surviving lane. A one-package retry selects a distinct lane but may benefit from the shared unit cache. Let the active owner continue rather than duplicate a build.

## Narrow Change Ownership

The call in library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts:64 passes the argument array into product identity and separately to the compiler. The identity factory is library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🎯️product/🟦️.ts:11; existing language-agnostic fixture and independent hash oracle are under library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🎯️product/🧪️tests/🟦️.ts. A command-aware identity projection can exclude frontend `--no-fail-fast` for Cargo test while preserving original arguments for execution and diagnostics. Tests should distinguish arguments after `--`, retain compile-affecting `--lib`, compare keys, and verify executor arguments and diagnostic fidelity. Treat other flags individually. No build was started by this audit.

## Landed Identity Change

The compiler product key now excludes the exact Cargo frontend argument `--no-fail-fast` when the Cargo command is `test`. Argument validation still applies to the original sequence. Options after the first `--` and compile-affecting arguments such as `--lib` remain in the key. The compiler invocation receives the original sequence. Cargo documents `--no-fail-fast` as a test-runner option. [Cargo test reference](https://doc.rust-lang.org/cargo/commands/cargo-test.html)

The checked-in language-neutral identity cases and independent Bun hash oracle passed six tests and 44 assertions through the repo tool. The change is live. Actual reuse by a subsequent native build remains unverified.
