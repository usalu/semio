# Direct TypeScript Mutation Replay — 2026-11-05

Executed independently authored retained `🐍️mutation-replay-check.mjs` through:

```
repo tools exec --light node -- --experimental-transform-types .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🐍️mutation-replay-check.mjs
```

Final exit 0. Actual current production TypeScript modules were imported in place; no checked-in source was copied into the ticket and no compilation/build runner or Vitest process was started. The Node transform-types experimental warning is expected and does not change the result.

Discovered 136 current neutral mutation cases across all 36 declared families: 80 applied, 18 no-op and 38 rejected, with zero failures. The bounded helper checks:

- Every neutral JSON read parses equally under native JSON.parse and existing third-party jsonc-parser, with comments/trailing commas refused.
- Every case has its exact native Rust test binding and all production mutation wire tags are represented.
- Actual snapshot/mutation/diff decoders admit fixture data; actual mutation outcome status, diagnostic codes and refused target paths match committed expectations.
- Actual apply returns expected decoded after-snapshot, including durable content; accepted actual diff equals committed JSON and applies from before to after. Rejected cases produce no lanes/diff and unchanged before/after.
- Production snapshot encode/decode roundtrips both sides; snapshot and accepted diff canonical encoded bytes match the committed files exactly.

This is substantive direct production TypeScript/neutral-fixture parity, independent of pending Vitest scheduling. It is not a claim that the complete Vitest suite, native Rust laws, artifact/snapshot family conversion, browser UI or host publication gates have passed. Those remain with their owning runs.
