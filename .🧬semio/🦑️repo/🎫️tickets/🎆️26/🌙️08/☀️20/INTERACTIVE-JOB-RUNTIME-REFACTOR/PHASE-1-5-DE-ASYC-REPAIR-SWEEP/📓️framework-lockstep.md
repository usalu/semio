# Shared Framework De-Async Lockstep

Date: 2026-08-26

## Scope

Owned shared roots:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- dependent implementation fleet under `✏️s/🔌️plugins/🗄️stdio`

Trinity, Writer, Raster, and the shared reserved-action route spans were excluded and not edited.

## Compiler-driven harness

`📜️script.ts` uses UTF-8 byte offsets, exact old-text checks, source hashes, a JSONL edit journal, compiler diagnostic/suggestion spans, and a monotonic diagnostic-count guard with exact rollback. It does not select edits by artifact or method name. The final harness self-test is:

```text
[DEBUG] framework lockstep codemod self-test: 19/19 passed
```

Journal: `📝️framework-lockstep-journal.jsonl`.

Accepted shared-store batches included 42+4+5 seed edits, 85+22+6 compiler repairs, and 401 all-target stale-await repairs. Accepted shared-plugin batches included 87+2 seed edits and 188+22+7+2 compiler repairs. A later compiler expansion record exposed the remaining `derive_artifact_facets!` adapter macro; its 14 generated methods were changed to synchronous lockstep and their stale awaits removed.

Accepted Stdio journal batches total 1,860 exact edits:

- initial implementation and stale-await repair: 1,368 + 192
- diagnostic-selected implementation signatures: 118
- direct E0728 await spans: 118
- E0277 `resolve_ready(sync_value)` wrapper span pairs: 64 edits for 32 wrappers

## Shared native gates

The native library gates are clean:

- `semio-framework-os-kernel --lib`: zero errors.
- `semio-framework-plugin --lib`: zero errors, including the final macro adapter repair; this was rebuilt as a dependency of the fresh Stdio gate.

The store all-target repair reached 44 residual errors: 40 non-lockstep store-test semantic/type errors and 4 unrelated channel errors outside the owned store file. No compatibility bridge was introduced.

## Stdio native library gate

The authoritative target directory is:

```text
/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/🎯️target-stdio-cold-lock
```

The exact final command was:

```text
FRAMEWORK_LOCKSTEP_TARGET='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/🎯️target-stdio-cold-lock' bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/📜️script.ts' repair-package semio-s-plugin-stdio '✏️s/🔌️plugins/🗄️stdio'
```

The script invokes `cargo check --locked -p semio-s-plugin-stdio --lib --message-format=json` with `CARGO_INCREMENTAL=0` and the target above. Final output:

```text
[DEBUG] semio-s-plugin-stdio compiler checkpoint: 64 errors
[DEBUG] semio-s-plugin-stdio repair 1: 64 -> 0 errors; 64 edits
[DEBUG] semio-s-plugin-stdio repair fixpoint: 0 errors
```

The full cold sequence was 2,299 errors, then 184 after the shared adapter macro repair, 66 after 118 direct stale-await edits, 64 after two concurrent JSON analyzer signature fixes, and zero after removing 32 compiler-proven stale readiness wrappers.

An earlier reused-target zero-error checkpoint was rejected after source inspection found an impossible sync-function await. A completely new target reproduced 2,299 errors and was used for every authoritative result above.

## Stdio test and repository-task gates

Native all-target classification command:

```text
FRAMEWORK_LOCKSTEP_TARGET='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/🎯️target-stdio-cold-lock' bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/📜️script.ts' report-package semio-s-plugin-stdio --all-targets
```

The first classification exposed:

```text
[DEBUG] semio-s-plugin-stdio report: 250 errors
[DEBUG] code E0277: 249
[DEBUG] code E0308: 1
```

The compiler-driven all-target repair accepted 249 edits for 250 → 78, 78 edits for 78 → 28, 28 edits for 28 → 1, and two nested one-for-one edits. A sixth one-for-one candidate was rejected and exactly rolled back when it increased diagnostics 1 → 3. The final compiler-suggested fixture await was then removed with its revealed fixture state already repaired concurrently. The fresh isolated classification now reports:

```text
[DEBUG] semio-s-plugin-stdio report: 0 errors
```

Canonical source-tree task:

```text
bun nx run '@semio-tech/stdio-plugin:test-quick'
```

The pre-repair Nx run proved taxonomy loading and dispatch but exited 1 on the same 250 lib-test diagnostics and 104 warnings. A post-repair execution is pending serialized Cargo validation.

## Wasm

Installed targets include `wasm32-unknown-unknown`, `wasm32-wasip1`, and `wasm32-wasip2`. The project-supported command is `bun nx run '@semio-tech/stdio-plugin:build-wasm-release'`; execution is pending serialized Cargo validation.
