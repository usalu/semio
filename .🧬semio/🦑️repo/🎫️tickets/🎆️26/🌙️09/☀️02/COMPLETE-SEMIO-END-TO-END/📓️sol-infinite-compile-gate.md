# Infinite Generic Inference Compile Gate

## Scope

Repaired the five `E0283` diagnostics in `semio-framework-os-infinite` that prevented the GIS plugin test target from compiling. No runtime behavior, schema, serialization shape, dependency, or public type changed.

## Reproduction

The infinite Rust crate has no standalone Nx project or package-local `📜️script.ts`. The narrowest existing compliant route is the GIS Rust package's `test-quick` Nx target, which invokes its existing package `📜️script.ts` and builds `semio-framework-os-infinite` as a dependency:

```text
CARGO_TARGET_DIR='<ticket>/🗑️generated/infinite-compile-cargo-target' bun nx run @semio-tech/gis-plugin:test-quick --skip-nx-cache
```

Before the fix the command exited 1:

```text
error[E0283]: type annotations needed
error[E0283]: type annotations needed
error[E0283]: type annotations needed
error[E0283]: type annotations needed
error[E0283]: type annotations needed
error: could not compile `semio-framework-os-infinite` (lib) due to 5 previous errors; 578 warnings emitted
```

The Nx wrapper abbreviates JSON compiler diagnostics, so the exact diagnostics were read from the ticket-local Cargo fingerprint with:

```text
jq -r 'select(.code.code == "E0283") | .rendered' '<target>/debug/.fingerprint/semio-framework-os-infinite-781a1f4e173c4a48/output-lib-semio_framework_os_infinite'
```

The five diagnostics mapped to three expressions in `🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`:

- line 1701, `obj.insert("x".into(), Value::from(nx))`: two diagnostics;
- line 1702, `obj.insert("y".into(), Value::from(ny))`: one diagnostic;
- line 5343, `obj.insert("ghost".into(), Value::Bool(ghost))`: two diagnostics.

## Root Cause and Repair

The first-party `JsonObject::insert` contract accepts `key: impl Into<String>`. Calling `.into()` before passing the literal creates an unconstrained intermediate target type. The host dependency graph contains multiple `From<&'static str>` implementations, so rustc cannot infer which conversion precedes the outer `Into<String>` bound.

The repair passes each string literal directly:

```rust
obj.insert("x", Value::from(nx));
obj.insert("y", Value::from(ny));
obj.insert("ghost", Value::Bool(ghost));
```

This is the native generic contract: `&'static str: Into<String>` is resolved by the callee. It removes redundant conversions without adding annotations, weakening types, introducing compatibility code, or changing the emitted JSON keys and values.

The DAG file already contained substantial unrelated concurrent modifications before this packet. It was re-read immediately before editing, and only these three expressions were changed by this lane.

## Test Strategy

This packet is a compile-contract correction with no semantic change. The existing DAG layout and label-overlay tests remain the behavioral coverage, and no new language-neutral fixture or independent oracle is required because the JSON output is byte-for-byte semantically unchanged.

The red reproduction is the test-first failure. The warm rerun rebuilt the modified infinite crate successfully and produced:

```text
debug/deps/libsemio_framework_os_infinite-781a1f4e173c4a48.rlib
debug/deps/libsemio_framework_os_infinite-781a1f4e173c4a48.rmeta
```

Both artifacts were written after the edit at 01:22 on 2026-09-03. Cargo then advanced to the downstream `semio-s-plugin-stdio` rustc process, proving the former infinite gate was crossed.

The enclosing GIS Nx run did not reach assertions: after approximately 14 minutes compiling the downstream stdio crate without diagnostics (approximately 17 minutes total for the warm run), the task-owned run was interrupted to bound this packet. Its one orphaned task-owned rustc process was terminated explicitly. Therefore no GIS test pass is claimed.

## Verification

1. Red command:
   `CARGO_TARGET_DIR='<ticket>/🗑️generated/infinite-compile-cargo-target' bun nx run @semio-tech/gis-plugin:test-quick --skip-nx-cache`
   - Exit 1.
   - Five `E0283` diagnostics in `semio-framework-os-infinite`; 578 warnings.
2. Diagnostic extraction:
   `jq -r 'select(.code.code == "E0283") | .rendered' '<fingerprint output>'`
   - Exit 0.
   - Confirmed the three literal-key expressions above.
3. Warm command after repair:
   same isolated Nx command.
   - `semio-framework-os-infinite` compiled successfully and emitted its `.rlib` and `.rmeta`.
   - The enclosing command was manually interrupted while compiling downstream `semio-s-plugin-stdio`; process exit 1 from interruption, not from a compiler diagnostic.
4. `git diff --check -- '🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs'`
   - Exit 0.

## Changed Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-infinite-compile-gate.md`

## Residual Blocker

The requested five-error infinite compile gate is repaired. Full GIS assertions remain unverified in this packet because the downstream stdio dependency did not finish compiling within the final bounded interval.
