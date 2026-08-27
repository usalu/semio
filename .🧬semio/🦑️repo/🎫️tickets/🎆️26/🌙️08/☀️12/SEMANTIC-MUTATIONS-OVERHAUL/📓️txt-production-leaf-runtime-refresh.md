# TXT Production Leaf Runtime Refresh

## Scope

`🧪️txt-production-leaf-runtime/📜️script.ts` is a ticket-only, actual-source Rust compiler/runtime harness. It mounts the current TXT snapshot, diff, mutation support, mutation root, and five current leaf sources. It is not a registered STDIO integration result.

The insert-line mount is exact and canonical:

```text
📥️insert-line/🦀️.rs
```

There is no historical-name fallback. Insert-line and released remove-line both mount only their canonical `🦀️.rs` sources.

## Fresh Artifact Contract

The harness refuses dependency and artifact defaults. Supply the three environment values below:

```text
SEMIO_TXT_RUNTIME_DEPS=<absolute ticket derive-contract-target/debug/deps>
SEMIO_TXT_RUNTIME_ARTIFACTS=<JSON artifact map>
SEMIO_TXT_RUNTIME_TEST_COUNT=<released current roster count>
```

`SEMIO_TXT_RUNTIME_NATIVE` is optional. When supplied it must be an absolute, non-symlink directory below the same fresh `debug` directory; when omitted the compiler receives no native search path.

The artifact map has exact absolute, non-symlink paths. It requires the coherent `rlib` and `rmeta` pair for `semio_framework_os_kernel`, `semio_framework_schema`, `serde`, `serde_json`, and `serde_core`, plus the `semio_framework_async_macros` dylib. Each library is passed to one compiler invocation in both formats.

The harness rejects paths outside the supplied dependency directory, a native directory outside its sibling fresh `debug` directory, symlinks, missing artifacts, compiler/runtime signal or error outcomes, nonzero statuses, and an unexpected test roster. It records artifact SHA-256 values and production source fingerprints before and after compilation in the retained run directory.

## Readiness Check

The environment-contract invocation intentionally omitted required variables. It exited with the expected required-environment diagnostic, confirming the no-default gate; this is not a compiler or runtime pass. Retained output: `🧪️txt-production-leaf-runtime/🧪️environment-contract.log`.

## Actual-Source Result

After the released remove-line canonical mount, the harness compiled and ran the actual mounted production TXT sources with no `-L native` argument. All compiler, listing, and runtime statuses were `0`, with no signal or spawn error.

```text
30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The required canonical descriptor/provenance tests for both insert-line and remove-line were in the listed 30-test roster and passed. The production fingerprint was identical before and after compilation:

```text
5a8ffbf9f61c7816d59396d2adc2df1cc269bf7e366b263f08baec5c15d16670
```

The exact fresh artifacts and their SHA-256 values are retained in `🧪️txt-production-leaf-runtime/🧫️run-eWDjFR/🔣️artifacts.json`; generated fixture, compiler output, list, test output, and executable are retained beside it. This is actual-source compiler/runtime evidence only, not a registered STDIO integration result.

The coordinator independently reran the retained 30-test binary through the installed Nx CLI, with exit 0 and all 30 tests passing: `🧪️txt-production-leaf-runtime/🧪️root-replay-30-direct-nx.log`. The initial outer-router invocation was blocked by concurrent generator taxonomy validation before reaching the binary (`🧪️root-replay-30.log`). Subsequent set-line, line-ending, and trailing-newline conversion work requires a new source compile and must not reuse this checkpoint as current-source acceptance.

## Final Five-Leaf Result

After all five canonical cutovers and shared glue mounts, the Nx-wrapped Bun harness compiled fresh current production sources and ran:

```text
33 actual production tests + 1 ticket workspace-token oracle = 34 passed
```

The listing required all five metadata tests: set-trailing-newline, set-line-ending, insert-line, remove-line, and set-line. The separate harness test compared each actual `MutationLeaf::PROVENANCE.workspace_token` to the independent Node SHA-256 result. Its token input is the frozen `semio.mutation-source-provenance/v1` domain plus NUL, then big-endian `u64` byte lengths and the UTF-8 canonical no-follow workspace path and root-manifest taxonomy locator with slash separators. Runtime retained `[DEBUG] token oracle pass counts=5/5`.

The wrapper invocation completed with compiler/list/runtime status `0`, no signal or spawn error, no native search argument, stable sources, and stable paired-artifact hashes. Source fingerprint before, after compilation, and after runtime:

```text
e069722c35cf9158c7ddbc46ca8061defcf867f1e811479619b2ee8cfa4b45b5
```

Retained exact invocation inputs, artifact hashes before/after, generated fixture, compiler output, test list, and runtime output: `🧪️txt-production-leaf-runtime/🧫️run-y3WKmY`. This remains an actual-source compiler/runtime result, distinct from the root-owned registered STDIO Cargo gate.

## Path-Guard Replay

The final source runtime claim above predates the harness path-guard hardening and is retained only as bounded compiler/runtime evidence. The current harness lexically rejects control characters, backslashes, colons, empty or traversal components, and ASCII-case `compose` components before any manifest taxonomy filesystem operation. It then walks every root-manifest taxonomy component and all canonical source/descriptor prefixes from the canonical no-follow workspace with `lstat`, rejecting symlinks at every level. The production source fingerprint walker likewise rejects a `compose` child before `lstat` on that child.

Seven virtual locator negatives were exercised without materializing a forbidden path: mixed-case `CoMpOsE`, direct and nested `..`, backslash, colon, control byte, and an empty component. Retained results: `🧪️txt-production-leaf-runtime/🧫️run-Z5cboo/🔣️path-guard.json`.

The hardened harness then repeated the current-source compiler/runtime execution with the same fresh artifact pairs: 33 production tests plus the one independent token oracle, all 34 passing. Compiler/list/runtime status remained `0`, sources and artifacts remained stable, and the token oracle reported `5/5`. Retained run: `🧪️txt-production-leaf-runtime/🧫️run-Z5cboo`.
