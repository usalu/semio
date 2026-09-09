# Combined Native Runtime Verification

The ticket script now supports a combined build of catalog package targets to avoid recompiling common framework dependencies for each package. Library targets and named integration targets are built separately. It preserves Cargo JSON, validates each executable against its package and target, hashes before and after each exact assertion, records individual failures, and supports cancellation through a signal or the run directory cancel.json. This is temporary ticket infrastructure; the repository runtime helper and its behavior are unchanged.

Changed file: 📜️script.ts in this ticket. Bun parsed the updated TypeScript. Execution is pending completion of native800.

## Scope Validation

Executed the new scope-only command and checked every package, target and feature against Cargo metadata. Excluded the three groups already freshly verified (WFC, Block 3D, Equation). Results: {"groups":64,"laws":376,"batches":[{"target":{"kind":"lib"},"groups":62},{"target":{"kind":"test","name":"native_codecs"},"groups":2}],"issues":[]}. No test compilation was started while native800 remains active.

## Launch Registration

Added the combined regression command beside the existing zero-warning regression launch entry at order 411.51. Replaced the temporary native800-specific prerequisite with detection of a live unfinished native worker owned by this ticket, so the command remains usable after generated receipts are removed. Parsed both the ticket script and the launch JSONC object using Bun's TypeScript parser before guarded writes.

Files: .vscode/launch.json and this ticket's 📜️script.ts.

## Scope Validation

Executed the new scope-only command and checked every package, target and feature against Cargo metadata. Excluded WFC and Block 3D, which remain freshly verified. Equation is included again after its window-state changes. Results: {"groups":65,"laws":400,"batches":[{"target":{"kind":"lib"},"groups":63},{"target":{"kind":"test","name":"native_codecs"},"groups":2}],"issues":[]}. No test compilation was started while native800 remains active.

## Ownership Regression Coverage

Registered three existing causal DAG tests for duplicate ownership, causal drain order, and exact identity return, plus the two current Puzzle 3D window-isolation and app-serialization boundary tests. Verified each Rust function exists and parsed the updated ticket runner before the guarded write. Runtime execution remains pending.

## Scope Validation

Executed the new scope-only command and checked every package, target and feature against Cargo metadata. Excluded WFC and Block 3D, which remain freshly verified. Equation is included again after its window-state changes. Results: {"groups":65,"laws":411,"batches":[{"target":{"kind":"lib"},"groups":63},{"target":{"kind":"test","name":"native_codecs"},"groups":2}],"issues":[]}. No test compilation was started while the native strict worker is active.

## Ownership Regression Coverage

Registered six existing actor bridge laws and two UI list allocation-counter laws for the compiler pass 835 changes. Verified each Rust function exists and parsed the updated ticket runner before the guarded write. Runtime execution remains pending.

## Kernel Runtime Ownership

Cargo metadata confirms semio-framework-os is the host wrapper and semio-framework-os-kernel owns the directory/store tests. Extended the existing kernel library group with explicit sync and nine additional existing laws covering the boxed actor lifecycle, rejected-page retirement, directory descriptors, and canonical bounds. Updated two never-ready actor futures to return the same boxed actor type as production. Rust and TypeScript parsing passed; execution remains pending.

## UI Engine Runtime Coverage

Added the UI engine as an explicit library test target with wgpu-engine enabled. Six existing tests cover stale scene-cursor refusal, one-step close, Unicode layout publication, cancellation, partial close, and complete layout identity validation. The catalog parsed and every source function was located before the guarded write. Runtime remains pending.

## Scope Validation

Executed the new scope-only command and checked every package, target and feature against Cargo metadata. Excluded WFC and Block 3D, which remain freshly verified. Equation is included again after its window-state changes. Results: {"groups":67,"laws":437,"batches":[{"target":{"kind":"lib"},"groups":65},{"target":{"kind":"test","name":"native_codecs"},"groups":2}],"issues":[]}. No test compilation was started while the native strict worker is active.

## Freshness Audit

Current source modification times for the two previously verified package areas, used to decide whether their exact runtime laws need repeating:

[
  {
    "root": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine",
    "latest": [
      {
        "f": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪶️soft/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:52:10.011Z"
      },
      {
        "f": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪞️symmetry/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:52:10.008Z"
      },
      {
        "f": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪜️hierarchy/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:52:09.999Z"
      },
      {
        "f": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🩺️diag/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:52:09.996Z"
      },
      {
        "f": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️parallel/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:52:09.993Z"
      }
    ]
  },
  {
    "root": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
    "latest": [
      {
        "f": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:46:29.279Z"
      },
      {
        "f": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        "mtime": "2026-09-09T04:15:23.021Z"
      },
      {
        "f": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs",
        "mtime": "2026-09-09T02:15:37.747Z"
      },
      {
        "f": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🎮️command-roster/🔣️.json",
        "mtime": "2026-09-09T01:54:39.096Z"
      },
      {
        "f": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T01:53:59.250Z"
      }
    ]
  }
]

## Scope Validation

Executed the new scope-only command and checked every package, target and feature against Cargo metadata. All catalog groups are now included. The freshness audit found WFC and Block 3D source changes after their earlier passing runs; Equation is also included after its window-state changes. Results: {"groups":69,"laws":478,"batches":[{"target":{"kind":"lib"},"groups":67},{"target":{"kind":"test","name":"native_codecs"},"groups":2}],"issues":[]}. No test compilation was started while the focused WASI strict check is active.

## Action Bus and Return-Owner Laws

Added seven existing framework tests to the shared library target: retained page admission/transfer/retirement, typed factory dispatch, saturation handback, exact prefix rejection, independent return-message byte parity, borrowed cursor cancellation, and one-byte UI-patch retirement. All selectors were located in their current Rust source and the catalog parsed before its guarded write. Runtime is pending.

## Current Dispatch Scope

The native worker selection exactly matches the root shipping-scope oracle. The current runtime catalog contains 485 laws in 69 groups, including all previously verified packages whose sources have since changed. Native compilation and combined runtime execution remain pending.
