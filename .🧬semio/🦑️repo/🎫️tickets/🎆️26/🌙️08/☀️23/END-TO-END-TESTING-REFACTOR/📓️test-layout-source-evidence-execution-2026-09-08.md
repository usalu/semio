# Rust source-policy evidence execution — 2026-09-08

## Implemented boundary

The root policy now models a Rust source as distinct authored evidence entries: one production entry and the canonical external test entries reached by conditional `#[path] mod` declarations parsed by `inspectRustModuleGraphFacts`. Canonical test entries retain their repo-relative source identity. Mutation verification replaces a unique needle in its owning entry and preserves every other entry.

The Store artifact-envelope rejection-transfer gate is the first migrated consumer. Production ownership checks read only Store production source. The four public law checks read only declared canonical test modules. Its public dispatcher constructs evidence rather than reading an undifferentiated raw string. The extracted TypeScript hostile suite uses source-aware mutation.

A language-neutral JSON vector describes a minimal production/test module edge. Its TypeScript implementation verifies identity and source-aware mutation, then invokes `rustc --test` as the independent parser/compiler/runtime oracle.

## Evidence

- Direct live Store evidence probe: three exact declared canonical test paths; `exact=true`.
- Direct extracted hostile suite: 13 mutations accepted as mutations and rejected by the policy baseline.
- `bun test ./…/🔬️rust-policy-source-evidence/🟦️.ts`: 2 pass, 0 fail, 7 assertions. The second test compiled and executed the canonical edge with `rustc --test`.
- Public `bun nx run workspace:verify-interactivity -- tool-jobs --p2a1-only --self-test` was started twice through the repository bootstrap. Both runs remained silent inside shared workspace bootstrap for several minutes. The first was stopped before execution output; the second remained live when this report was written. No passing Nx claim is made.
- A broad standalone `tsc` probe is not a valid focused gate and reported existing workspace errors including missing generated Storybook inputs, Bun globals, and unrelated root signatures.

## Exact changed paths

- `📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️tool-job-artifact-envelope-rejection-transfer/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️rust-policy-source-evidence/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️rust-policy-source-evidence/🔣️.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-source-evidence-execution-2026-09-08.md`

## Multi-source suite integration continuation

`policyReadRustPolicySource` serializes the same evidence record with explicit per-path boundaries so existing multi-source hostile mutation matrices can keep their string replacement mechanics. `interactivityProductionSource` consumes only the production segment; law assertions see the full evidence view and therefore retain the canonical test path and body. All root interactivity audit reads keyed by `INTERACTIVITY_AUDIT_*` and the nine extracted suites were routed through this reader.

The Puzzle fill envelope suite now reaches its production assertions instead of reporting missing extracted fixture laws. Its remaining baseline failure is the previously audited unrelated fixed nested allocation/credit assertion; it was preserved.

Additional exact changed paths:

- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-preview-json/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-live-reconcile/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️interactivity-mounted-prepared-render/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-surface-lane/🟦️.ts`

## Correct repoRoot suite results after ancestor-edge resolution

Canonical evidence resolution accepts exact declared paths that ascend out of delivery ancestors before entering the semantic owner's `🧪️tests/<case>/🦀️.rs`. The isolation vector now also proves a law visible in the evidence view is absent from `interactivityProductionSource` (9 assertions total).

All nine suites were invoked with the real repo root in one process. Each former extracted-law failure advanced to preserved unrelated drift:

- Puzzle envelope: fixed nested allocation/credit baseline assertion.
- Puzzle P4e: spatial-owner and canonical bounded diagnostic-page baseline assertions.
- Puzzle preview: `fixture-cap` production mutation no longer binds.
- Live reconcile: `per-surface-credit-cap` production mutation no longer binds.
- Mounted frame: `dormant-production-authority` production mutation no longer binds.
- Mounted engine: `flow-whole-store-drop` production mutation no longer binds.
- Mounted prepared render: `missing-gpu-watchdog` production mutation no longer binds; the former `missing-input-drop-law` now binds.
- Mounted surface: `wide-worker-deadline` production mutation no longer binds.
- Mounted layout: `unbounded-renderer-budget` production mutation no longer binds.

No assertion or hostile mutation was removed or weakened.

The isolated public Nx retry used caller-provided plugin isolation, no-timeout, workspace-data, and cache paths. Nx reached project graph calculation, reported that its daemon could not compute the graph, attempted its fallback without output, and was interrupted after the bounded wait. No Nx test result is claimed and no shared daemon reset was run.

## Coordinator Nx Verification

The two source-evidence tests were rerun together with the 21-test layout suite through public `bun nx exec` using the ticket’s private minimal Nx graph. Combined result: 23 pass, 0 fail, 79 assertions. The evidence suite’s compiler oracle executed successfully. This does not claim the unrelated failing production interactivity suites passed. The executable filename now explicitly uses `.exe` on Windows.

## Full Repository Dispatcher Result

The real public `bun nx run workspace:verify-interactivity -- tool-jobs --p2a1-only --self-test` completed successfully. Runtime output reported `live-source clean; hostile-mutations=13`. Nx reported one executed task, zero cache hits, and a 5.7-second run after full project graph construction. This resolves the earlier pending/timeout observations for the P2a1 dispatcher.
