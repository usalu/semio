# Raw Wire Retirement Repair

## Exact Ownership Change

`RetainedToolWireInput` now retires only the granted initialized bytes of its last page, keeps `admitted_bytes` equal to the sum of live page lengths, removes exhausted pages in place, and releases the empty backing allocation in a separate one-item/zero-byte step. Terminal emptiness includes zero backing capacity. Zero item grants block without changing allocation or byte ownership; zero byte grants block while initialized bytes remain. Closing immediately disables `page` reads, so a sealed decoder cannot resume against partially retired data.

Partial-page byte retirement reports zero released items. Only an exhausted page and the final backing allocation count as released owners. The native and source fixtures assert cumulative item counts equal the initial live pages plus the one admitted backing allocation when present.

The action-bus retained-number, Flow host-effect, Draw gesture, Writer, and Layout export consumers convert nested completion to outer pending until all outer fields are empty. The mounted latest-wins, typed-operation, generic retained-command, and Puzzle raw owners already preserve this distinction. This repair does not certify unrelated cleanup tails: Writer still has a whole command-retirement envelope and Layout retains unrelated whole-string cleanup.

## Evidence

The strict language-neutral fixture has five before/after-seal ownership cases and grants 0/1/64/4096. Its source oracle uses Node Buffer and SHA-256 and rejects four hostile fixtures. The post-implementation source rerun passed (`🧪️raw-wire-retirement-source-r2-2026-08-27.txt`). The mounted native law asserts actual capacity release, exact byte deltas, disabled closed reads, and zero-grant non-mutation, but has not yet been executed. The canonical native selector is `@semio-tech/framework-rs:test-wire-retirement-native`.

The authoritative launch seed includes source and native entries, regenerated through the plugin-registry target. The final item-accounting source rerun passed (`🧪️raw-wire-retirement-source-r3-2026-08-27.txt`). A canonical Flow-core `check --tests` exited with 18 test-build errors; its exact recovered diagnostics and source repairs are documented in `📓️flow-browser-declaration-and-check-2026-08-27.md`. Metadata compilation is not a runtime test.

## Remaining Admission Boundary

The constructor still reserves the declared page-owner capacity up front, and each fixed page constructor initializes its 4096-byte storage. This patch repairs cancellation/retirement grants and ownership only; it does not claim arbitrary ingress allocation or all command decoding is within the interactive time budget.
