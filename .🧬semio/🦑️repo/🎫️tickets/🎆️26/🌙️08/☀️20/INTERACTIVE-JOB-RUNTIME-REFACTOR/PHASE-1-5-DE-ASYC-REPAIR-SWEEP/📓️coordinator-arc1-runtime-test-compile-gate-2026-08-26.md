# ARC1 Runtime Test Compile Gate

## Scope

The shared retained-command shell now has an ActionBus runtime test named `shared_retained_command_checkpoint_resumes_exact_cursor_and_cancels_with_bounded_close`. It interrupts after the first custom-work cursor, closes the original owner incrementally, restores the ARC1 checkpoint into a fresh job, proves that only the two remaining work turns run, validates the exact mutation, and exercises cancellation plus bounded close.

## Verified source gates

- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`
- Exit: `0`
- Result: `self-tests=464 clean`
- The hostile set covers a resizable checkpoint, missing resume constructor, lost checkpoint close owner, false terminal witness, skipped custom-work restore, unbounded checkpoint size, replay-from-zero, cancellation bypass, and the exact shared `FixedOperationRegistry` exemption boundary.

## Native runtime-test compile attempt

Command:

```text
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/🧪️target-root-arc1-runtime' RUSTFLAGS='-Awarnings' cargo test --locked -p semio-framework-plugin shared_retained_command_checkpoint_resumes_exact_cursor_and_cancels_with_bounded_close --no-run --message-format=short
```

Exit: `101`.

The crate reached `semio-framework-plugin` and then reported `321` test-target compilation errors. The failures are a broader retained test-target de-async/private-boundary backlog: unresolved renamed UI contract symbols, stale `.await` calls against now-synchronous APIs, async implementations of newly synchronous trait methods, private-field accesses from relocated test modules, and changed fallible UI builder results. The ARC1 runtime test has therefore not executed yet and remains an open gate. Production-library compilation evidence from the current framework cohort is separate and does not satisfy this test gate.

## Next repair

Repair the whole `semio-framework-plugin` test target rather than bypassing it, rerun the exact no-run command, execute the focused test, and then run the package's full retained test target. No runtime-pass claim is valid before those commands exit zero.

## Source-only test-target repair checkpoint

The saved rustc output contains `322` error-level rows, including the final abort row, hence `321` actionable diagnostics. Its primary-code census was:

```text
E0616 75
E0277 47
E0624 43
E0425 41
E0308 38
E0599 35
E0609 12
E0433 10
E0053 8
E0061 7
other 5
```

The source repair was performed while the shared compiler slot was reserved by another cohort, so this checkpoint makes no fresh compilation or pass claim. The repaired root-cause groups are:

- relocated crate-owned tests now reach retained registries, cancellation owners, immutable child/presence roots, and close cursors through crate-only visibility; no item was made externally public;
- stale awaits were removed from synchronous store reads, envelope reads, applied-edit/checkpoint accessors, child emissions, artifact builders/composers, and IO-entry constructors;
- `ArtifactEditor`/`ArtifactViewer` fixtures implement the current synchronous trait surface and return the current fallible fixed-capacity UI tree type;
- fixed-capacity UI fixtures use typed `Label`, `UiText`, `SurfaceId`, `TreeNode::try_new`, and owned `ComponentTree` publication;
- retained jobs implement `begin_close`, bounded `close_step`, and `terminal_is_empty`, and erased jobs import the trait that owns those methods;
- inference tests use `RetainedJobPayload` ownership and an explicitly test-only byte-count variant for maximum/maximum-plus-one admission rather than forging a production payload;
- presence tests use the current bounded peer-page wire path instead of the removed monolithic roster wire type;
- two malformed `app.await.handle_action(...)` chains and a retained-ingress `Result::expect` that demanded a forbidden `Debug` implementation were repaired during the final source audit.

Static checks:

```text
rustfmt --edition 2024 <six affected semio-framework-plugin Rust sources>
exit 0

git diff --check -- <six affected semio-framework-plugin Rust sources>
exit 0
```

Remaining category status before the next compiler turn: no known saved root-cause category remains intentionally unfixed in source. A fresh isolated `cargo test --no-run` is still required to expose post-repair cascades and is the next authoritative count; the historical `321` must not be presented as current after this source checkpoint.

## ARC1 fixture and oracle audit

The pre-audit ticket artifact `🧪️artifact-retained-command-checkpoint-v1.json` was historical and
stale: it described codec version `1`, a 34-byte layout, and no context digest, while production
uses codec version `2`, a 40-byte header, and an eight-byte context digest. It was not consumed by
the runtime test or any independent oracle and therefore supplied no acceptance evidence.

The production retained-command source now owns the authoritative language-neutral schema and
fixture:

```text
🧵️retained-command/🧬️schema/🔣️artifact-command-checkpoint.schema.json
🧵️retained-command/🧪️fixtures/🔣️artifact-command-checkpoint.json
```

The five fixture vectors are empty work state, one work byte, exact interrupted cursor, exact
512-byte maximum, and maximum plus one. The Rust consumer parses the fixture with Serde, encodes an
independent byte stream with the third-party `byteorder` crate, requires byte identity with the
production encoder, then round-trips every decoded field. The maximum-plus-one vector must be
rejected before any output is accepted. Separate hostile decode checks retain the context-digest
and reserved-byte drift laws. `byteorder` is an explicit dev dependency and was already present in
the workspace lock graph.

Source-only validation while the compiler slot remained reserved:

```text
bun -e <strict Ajv2020 ARC1 schema/fixture validation>
exit 0; cases=5; max+1-forged=reject; false-error=reject

rustfmt --edition 2024 🧵️retained-command/🦀️component.rs
exit 0

git diff --check -- <ARC1 source/schema/fixture/manifest>
exit 0
```

No Rust test pass is claimed. The focused no-run and runtime commands remain the next authoritative
gates after the compiler slot is released.

The final live-tree static verifier rerun after the v2 cursor/context repairs is fresh:

```text
bun ./📜️script.ts verify interactivity tool-jobs --self-test
exit 0; self-tests=466 clean
```

## Fresh interpreter-differential compile checkpoint

The first isolated differential command was:

```text
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/🧪️target-root-arc1-runtime' RUSTFLAGS='-Awarnings' cargo test --locked -p semio-framework-plugin memory_copy_ranges_match_the_language_neutral_fixture_and_wasmtime --lib -- --nocapture
exit 101; test not executed; 27 lib-test source diagnostics
```

The exact saved command/result/category evidence is
`📝️arc1-interpreter-differential-compile-2026-08-26.txt`. The fresh 27 errors were eight missing
render-fixture imports, one private identity-oracle reach-through, one unawaited async task
constructor, a four-error roster clone cascade, three typed `SurfaceId` fixture drifts, one fixed
close-registry access drift, and nine private lease-method reach-throughs.

All seven root causes are repaired in source. The cancellation and identity tests use narrow
`cfg(test)` crate helpers; the production cancellation lease methods remain private. The affected
sources are rustfmt-clean and `git diff --check` is clean. No post-repair compile claim is made:
the compiler slot was released to the Puzzle cohort immediately after the failed command, and the
rerun remains queued behind that cohort.
