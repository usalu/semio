# P10bf Next Live Dependency Scout

Date: 2026-08-22  
Verdict: **Dagre is the next isolated Phase 10 runtime packet after the Diagram force audit.**

## Current boundary

The provisional dependency census is 141 identities: 63 Rust and 78 JavaScript. This is not an
acceptance count until the independent owned-force audit passes.

`dagre` has one source import and one manifest owner:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`

The lock row is `dagre@0.8.5` with transitive `graphlib` and `lodash`. No other live source import was
found outside dependency artifacts.

## Interactivity finding

The current private port constructs Dagre's graph, inserts every node and edge, invokes whole
`dagre.layout(graph)`, then maps the complete input into a new position map. The public
`calculateDiagramLayout` performs two more whole input maps, and `useDiagramLayout` calls that full
path synchronously inside React `useMemo`. It is therefore both a dependency-removal packet and a
Phase 8/10 interactivity repair; replacing Dagre with another synchronous full-layout function would
not satisfy the governing rule.

## Required owned implementation

Implement a deterministic persistent directed-layout job behind the owned Diagram layout interface.
At minimum its state machine must cursorize graph admission, stable node/edge ordering, cycle/rank
assignment, crossing reduction, coordinate assignment, direction transform, and output projection.
Every unit needs a pre-unit deadline/fuel check, cancellation/generation authority, bounded
mailbox/byte state, and replaceable partial previews. Large input capture must remain reference/O(1)
until scheduled work reads it. `useDiagramLayout` must schedule and publish complete generation-
matching results rather than run the layout inside render/useMemo. The existing synchronous function
may remain only as an explicitly named batch adapter that drives the exact same state machine.

Differential fixtures should cover DAGs, cycles, parallel edges, disconnected components,
self-edges, four directions, variable dimensions, deterministic reversed inputs, and adversarial
20k-node/20k-edge setup/timing/cancellation. Exact Dagre coordinates are not a required public
contract, but rank direction, non-overlap, edge ordering, determinism, and stable visual semantics
must be compared before deleting the old dependency.

## Gate

After implementation, independently audit current source and rerun focused/full UI and renderer
suites, typecheck/lint/primitives/format, dependency freeze/parity/frozen lock, and exact retired
`dagre`/`graphlib`/private-`lodash` reachability scans. Real browser RAF cadence and visual quality
remain separate browser gates.

No production source was edited by this scout.
