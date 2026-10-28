# Inner Geometry

The four additional selected paths now form the directed closed inner cycle 1 → 5 → 10 → 13 → 1. Only lines 286–289 of parametrization-geometry.tex changed; the existing selected 16-edge outer cycle, complete 240-arc graph, bend styles, graph size, ledger, and K3 worked example remain untouched.

## Test-First Evidence

- Authored language-neutral inner-geometry.case.json and inner-geometry.verify.cjs before modifying production.
- The first verifier run failed exactly on the previous selected inner edges: 1 → 10, 2 → 12, 4 → 10, 5 → 13.
- Replaced precisely those four data lines, with an in-memory assertion that no other line differed. No checked-in source was copied into the ticket.
- The same verifier now passes: seed selection, directed cycle closure, 16-edge outer cycle, all 240 distinct non-self base arcs, 20 selected arcs, and preserved shape metrics.

## Selection

Starting from LCG state 20261027, each shuffle advances state with (1664525 × state + 1013904223) modulo 2³². A full Fisher–Yates shuffle draws 16 times (including the trivial final swap). Four vertices are taken, sorted, and accepted only when all cyclic index gaps lie in [3, 5]. Attempt 19 selects [1, 5, 10, 13], with gaps [4, 5, 3, 4]. The independent auditor supplies third-party D3 shuffle validation in production tests.

## Status

Geometry is frozen and ready for DE/EN compilation and dark review. No other production files were modified by this agent.
