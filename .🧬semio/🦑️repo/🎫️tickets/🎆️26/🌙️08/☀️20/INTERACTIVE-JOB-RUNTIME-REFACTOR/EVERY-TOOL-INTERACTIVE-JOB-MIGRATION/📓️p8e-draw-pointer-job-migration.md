# P8e Draw Pointer Job Migration

## Outcome

Draw's `canvasPointerDown` action is classified `Migrated` and enters the production `ToolOperationSpec → ToolJobFactory → InteractiveJob` worker path. Its document-sized trace hit-test no longer flattens and scans the whole layer tree in one reducer call. It advances through zero-delay `DispatchAction` continuations, each of which re-enters the same production action-bus path.

## Bounded and Resumable Work

- `TRACE_POINTER_WORK_PER_STEP` is exactly 32.
- A work unit is one tree enter/visit, one path segment, or one polygon point.
- Tree traversal is resumable through owned index paths; path and polygon bounds retain their cursors and partial extrema.
- Equal-generality candidate replacement and reverse document traversal preserve the former pick ordering.
- The ordinary gesture branch remains the existing FSM transition. A focused test covers every pointer-down utility and asserts at most one microstep; this chart has no raised/eventless pointer-down chain.
- In-progress trace previews expose generation, cursor, completed work, and pending work through the existing gesture preview contract; every continuation advances the preview sequence.

The follow-up compile-shape audit repaired the continuation boundary: `TracePointerJob::advance`, candidate evaluation, transform/bounds calculation, and utility-reset wrapping are synchronous functions. The async session boundary awaits continuation dispatch and the existing async trace-layer/mutation builders. Escape and utility-change handlers also await the gesture session they invoke. Empty paths retain the former fallback bounds, while non-empty paths and polygons remain cursor-bounded.

## Freshness and Cancellation

Every initial pointer-down advances an input generation. Trace continuations carry that generation, and a stale continuation emits neither mutations nor another continuation while leaving the live job untouched. A newer pointer-down, Escape, or utility change cancels the pending trace job immediately. Completion alone emits the trace mutation and utility reset.

Focused source tests were added for:

- the exact 32-unit per-step ceiling against a 256-segment path;
- synchronous compile shapes for the bounded reducer and its owned geometry/wrapping helpers;
- stale input-generation rejection without disturbing the live job;
- one-microstep pointer-down FSM settlement.

## Validation

- `bun ./📜️script.ts verify interactivity tool-jobs --format json`: PASS — 774 production rows, all 774 bounded, zero batch-only/forbidden/deleted rows, 1 production factory, 1 registration path, 1 dispatch path, and zero failures.
- Focused `rustfmt --check` on the four touched Draw Rust files: PASS; this also parsed the Rust sources.
- Focused continuation call-site audit: PASS — the reducer/helper chain is synchronous, all async session/trace-builder calls at the boundary are awaited, and both cancellation handlers await `step_gesture`.
- Exact stale-policy search: PASS — no Draw `canvasPointerDown` batch-only declaration or opaque-reducer ledger row remains.

Cargo checks and Rust test execution were not run because P4 exclusively owns the Cargo lane. The four focused tests above are therefore source-added and awaiting the P4 Cargo release gate.

## P8i Ownership and Boundedness Repair

This section supersedes the pre-audit ownership/preview description above.

The final source audit findings are repaired in source:

- Every genuinely synchronous reachable Draw schema, geometry, gesture guard/action, preview, and mutation-builder helper is now a synchronous `fn`; only the typed app handler remains async at the framework boundary. This removes the future-as-value chain rather than adding scattered awaits.
- `DrawSession` no longer stores a trace generation or trace job. Trace state lives in a process-wide `OnceLock<Mutex<BTreeMap<(document_id, generation), TracePointerJob>>>`, so worker hops preserve the job and equal generation values in different documents cannot alias.
- The registry has no global 32-session eviction. Completion, Escape, utility change, or a newer trace for the same document removes only that document/generation. Focused tests cover two documents with the same generation and forty simultaneously active documents.
- Hidden `canvasPointerDown` continuations carry only fixed-size generation/completed/pending counters. The frontier stays in the scoped registry; it is never serialized into an ever-growing payload.
- Root and group expansion use `Roots { next }` and `GroupChildren { path, next }` cursors. One work unit enqueues at most the cursor plus one child, including 20,000-root and 10,000-child adversarial trees.
- Authoritative `DrawConfig` fields publish the live generation and completed/pending counters. Render invalidation therefore observes progress from config, not a fresh/default thread-local session.
- Production-handler tests move initial and continuation calls across separate OS threads, reject stale config generations, time the complete wide-tree handler plus continuation encoding under the 8 ms watchdog, and assert a sub-512-byte checkpoint representation.

The exact interactivity verifier remains green at 774/774 bounded rows and zero failures. `rustfmt --check` parsed the touched sources and reported formatting differences only; it was not used to rewrite shared files. Cargo/test execution remains deferred to P4.

## Files

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-escape/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-active-utility/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `📜️script.ts`
