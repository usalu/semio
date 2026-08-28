# Runtime Component Copy R22

Exact Component-copy runtime selector:2 passed,92 skipped,0.109s,exit0. Actual fresh-field callback catches injected unwind while its source/candidate owner remains structurally stored in RecordDiffCursor; source Surface content survives and exact close completes across ten cancellation frontiers. Before-allocation quota refusal keeps the source and charges/allocates zero. This does not claim other fields or arbitrary producer unwind safe.

```text
15:[DEBUG] surface-ownership-oracle checks=19
23:[DEBUG] surface-component-copy turns=15 reported=81781 ledger-allocation=32768 actual-allocation=32768
26:test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 93 filtered out; finished in 0.01s
49:[DEBUG] surface-component-unwind frontier=0 retained-outside-callback=true terminal-close=true
67:[DEBUG] surface-component-unwind frontier=1 retained-outside-callback=true terminal-close=true
85:[DEBUG] surface-component-unwind frontier=2 retained-outside-callback=true terminal-close=true
103:[DEBUG] surface-component-unwind frontier=3 retained-outside-callback=true terminal-close=true
121:[DEBUG] surface-component-unwind frontier=4 retained-outside-callback=true terminal-close=true
139:[DEBUG] surface-component-unwind frontier=5 retained-outside-callback=true terminal-close=true
157:[DEBUG] surface-component-unwind frontier=8 retained-outside-callback=true terminal-close=true
175:[DEBUG] surface-component-unwind frontier=12 retained-outside-callback=true terminal-close=true
176:[DEBUG] surface-component-unwind frontier=16 retained-outside-callback=true terminal-close=true
177:[DEBUG] surface-component-unwind frontier=32 retained-outside-callback=true terminal-close=true
178:[DEBUG] surface-component-refusal actual-allocation=0 source-retained=true terminal=true
181:test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 93 filtered out; finished in 0.05s
185:     Summary [   0.109s] 2 tests run: 2 passed, 92 skipped
186:[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-dhuSs6

```
