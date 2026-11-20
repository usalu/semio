# Native retirement receiving receipts

A fresh untyped native retirement could report an enclosing semantic receipt after its decoder had already posted the physical effects. The retirement entry point lent the decoder the current receiving accumulator without issuing a grant for it. The enclosing receipt handler therefore assumed the whole semantic receipt was new spend. The physical effects were debited twice and remained in the accumulator. The next forward receiving step in the same turn refused with `original receiving scope still owes its enclosing semantic receipt`.

The Job context now opens a semantic receiving scope before it runs a fresh, untyped retirement. The enclosing receipt can deduct the accepted decoder spend and close the scope. A typed retirement or recovery of an already-pending effect uses its reserved Source receipt and a separate local accumulator.

The native regression drives a real decoder and Source with consecutive receiving steps in a single turn. The language-neutral JSON fixture expects eight copied bytes, zero duplicate bytes and successful reopening. Before the fix the retirement posted eight duplicate bytes and the next scope refused. Three focused native tests passed after the fix. They cover fresh untyped retirement, typed retirement and cancellation after an interrupted effect, recovered in the next slice with the initiating Source dispatch retained. Both neutral cases also matched an independent SQLite calculation.

## Actor recovery across host epochs

The actor dispatcher borrowed a new Source dispatch token on each host budget slice. A claimed partial Work receipt retains the exact dispatch that produced it. Recovery across host epochs therefore failed before receipt reconciliation. The actor now retains its dispatch token while the decoder has pending custody and issues a fresh token at the next clean epoch.

The actor regression interrupts a real decoder effect after recording Work, advances the host epoch, settles the original receipt and verifies that the next clean epoch gets a fresh dispatch. It also closes the actor under its caller grant. The close path previously narrowed every native-owned step to one item even when the decoder needed two items to return its Source loan. The closure now uses the caller's admitted grant. The actor regression, repeated fresh-borrow escrow test, retirement binding test and independent SQLite fixture calculation passed. The broader actor decoder suite hit an older five-epoch admission assertion and aborted during fixture teardown. Its financial-512 case also refused its second-attempt assertion.

The Job library run passed 9 of 12 tests, including the regression. Failures remained in the oversized allocation, controlled retirement and one-quantum registry cases. The shared checkout changed during the suite runs, so the results give partial coverage.

The browser baselines exposed only the semantic failure tag, most recently through `original-turn.native-return-refused: semantic`. The browser's exact sequence 7 cause remains unverified. A temporary one-shot `[DEBUG] FEM WIT poll semantic failure` probe prints the typed cause at the WIT poll boundary. `semio-plugin-reactor-context` checks successfully for `wasm32-wasip2` with the probe and both runtime fixes. See [the probe inventory](./📓️debug-probe-inventory.md).
