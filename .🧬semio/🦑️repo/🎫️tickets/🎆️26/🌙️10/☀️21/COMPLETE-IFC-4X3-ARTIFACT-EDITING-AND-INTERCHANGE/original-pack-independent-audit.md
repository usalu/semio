# Independent Original IFC Pack Audit

Source-only review of the current original IFC receiving, graph binding, canonical Pack index/publication, shared writer and IFC4x3 normal route wrappers. No Rust execution or typechecking was performed: authentic native compiler ingress remains refused before rustc. The documented direct read/rg fallback was used because repo CLI/MCP is unavailable. No product files were changed.

## Actionable Finding

### P2 — Graph Binding Does Not Advance Native Progress

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/owned-graph.rs:120` calls `native.checkpoint()`, but accepted graph turns return at line 350 without calling `native.step()` or `native.advance()`. This graph has potentially millions of value/entity and binary scanning turns. `NativeDecodeControl::checkpoint` (`🧰️framework/🔨️modules/🌱️value/🛬️decode/🦀️.rs:115`) only replays the current completed counter; only step/advance increments it (line 127). The shared driver (`🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs:148-156`) records the retained physical wallet but also only checkpoints the native observer.

Therefore native progress stalls throughout graph assembly. A caller canceling after a later completed-work threshold cannot trigger during this expensive phase, even though callbacks continue. This is narrower than an unconditional inability to cancel: callbacks based on elapsed time or invocation count still work. After each successful graph turn, advance native work and attach that turn’s retained receipt if the post-work callback cancels. Add an actual Rust observer regression entering graph binding and canceling after a completed-counter increment; separately check retained partial graph closure. This remains a source inference, not an executed regression.

## Reviewed Custody And Grammar

The new successful writer drain explicitly persists `draining` after codec Complete changes to Closing; subsequent demands stay on cursor retirement. Final wire take is separate, and cancellation closure retains both cursor and final wire. The graph uses reverse binding with forward child indices and one-time `take`, rejects orphan/reused/missing/backward children, and checks value depth. Projection uses a deterministic DFS locator forest and immutable snapshot borrowing; its parent lookup bound matches the admitted depth. Original receiving compares source address/length and ledger/recipient on explicit mutation/closure. Normal route wrappers retain intermediate source/outcome and typed error results inside nested original owner frames.

No additional concrete source correctness/custody blocker was established in these reviewed paths. This statement is limited to source review and does not establish compilation, runtime correctness, native allocation accounting, or host interchange.

## Evidence Limits

The Rust receiving roundtrip, canonical reference equality, denied/canceled receiving and shared writer drain tests are authored Rust tests, currently unexecuted. The neutral SQLite identity/index witness independently validates its reference lookup decisions, not actual IFC Pack cursor execution or measured Rust work. The native IfcOpenShell roundtrip gate correctly requires actual native-produced output; no generated native result was available in this audit. Source probes and reference witnesses must continue to be labeled accordingly. Root-owned native STEP work and known foreign BIM/glTF codec boundaries were excluded.
