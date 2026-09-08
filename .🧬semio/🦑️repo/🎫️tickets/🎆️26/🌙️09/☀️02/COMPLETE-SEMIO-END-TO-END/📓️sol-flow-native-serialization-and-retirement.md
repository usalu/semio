# Flow Native Serialization and Retirement

## Scope

This lane repairs the Flow native-test closure after the first-party `DslValue` transition, preserves the browser session admission owner through uncertain open failures, and closes the remaining DAG host byte-credit gap beneath `FlowHostRetirement`.

## Source changes

- Flow artifact, extension, playbook, registry, VCS, and host tests now use the first-party `ToValue` / `FromValue` JSON contract. `serde_json` remains only as an independent RFC 8259 oracle in tests.
- The browser host classifies only an exact failed `Open` reply as a verified pre-admission rejection. The runtime releases that one JS session entry immediately.
- A transport, decode, or Wasm exception while `Open` may already be admitted is retained as the exact `(request, generation)` owner. Runtime close drives the whole host, acknowledges the eventual session-terminal receipt, and clears the uncertain owner only after `flow_bridge_terminal_is_empty() == 1`.
- Runtime close uses one retained promise, attempts the global host close after every per-session close settles, and reports the original session failure only after a successful global terminal proof. An unproven host close retains the owner and does not claim terminal emptiness.

The browser-runtime fixture and AJV oracle distinguish verified rejection from uncertain admission. The registered source gate is GREEN and executes both controlled bridge branches. The full browser gate capture `70750` proves shared two-session retirement, late-open sibling preservation, receipt/control backpressure, both new open-failure branches, exact terminal refusal, and cooperative close. It then terminates RED at the pre-existing generated Wasm leg with `RuntimeError: Unreachable code should not be executed` in `flow_bridge_send`; therefore no freshly generated Wasm claim is made.

## Native status

The authoritative Flow native command is registered as `⚖️gate🌊️flow🧹️session-close🦀️native`. Session `61322` is active against the ticket-local `flow-session-close-native-target`; at the last observation it was waiting in Cargo without a new compiler diagnostic while other workspace Cargo owners held the shared package cache. No native pass is claimed.

## Remaining deepest-owner work

`FlowHostRetirement::close_page(maximum_items, maximum_bytes)` still delegates first to zero-argument `DagHostRetirement::close_step()`. The latter can remove a full `DagNodeSpec`, `DagFixtureEdge`, graph node/handle semantic bag, event vector, or cached icon key/value in one turn. The required repair is an owner-retaining DAG cursor that accrues bounded byte credit against the exact owned payload before terminal destruction or opaque-scene transfer. Its nonterminal Drop path must retain ownership rather than destructing the remaining graph.
