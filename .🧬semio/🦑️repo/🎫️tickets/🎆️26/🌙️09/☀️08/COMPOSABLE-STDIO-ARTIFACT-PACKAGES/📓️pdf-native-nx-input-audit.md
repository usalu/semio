# PDF Native Nx Input Audit

## Result

No actionable native cache-input gap is present in the current private PDF Nx graph. The earlier standalone graph capture is superseded by `🗑️generated/nx-root-pdf-native-proof/project-graph.json`; the bounded receipt copied from it is `🗑️generated/pdf-native-current-14-input-receipt.json`.

The current `@semio-tech/stdio-pdf-rs:build` graph has 76 inputs. It contains all 26 Cargo-policy environment inputs, including compiler and linker variables (`CC`, `CXX`, `CFLAGS`, `CXXFLAGS`, `LDFLAGS`), platform SDK controls, Cargo/Rust controls, and all six `SEMIO_*` policy inputs. It also contains `rustc -vV`, `cargo --version`, Bun and Node version commands, plus `node -p "process.platform.concat(process.arch)"` for host OS/architecture separation.

Its five script inputs start with the local PDF router and include the neutral artifact router and shared Cargo runner. This is the exact closure added by `nativeTargetCommandInputs` in [🟨️.mjs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:359); the shared compiler contract comes from `nativeCommandInputs` ([🟨️.mjs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:344)).

The graph’s 835 `nativeSources` entries have 815 PDF paths, zero JPG paths, and zero other artifact paths. This preserves sibling-format isolation while retaining PDF’s owned source topology.

No task graph was available in `nx-root-pdf-native-proof`; this review used its current project graph only. No Nx or native build was launched for this audit. The receipt proves configured inputs, not a native cache hit, cache restore, or artifact validity.
