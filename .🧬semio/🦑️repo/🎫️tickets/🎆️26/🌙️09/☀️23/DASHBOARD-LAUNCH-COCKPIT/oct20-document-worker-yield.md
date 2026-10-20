# Document Worker Yield

The native source worker queue capacity32 did not bound a UI poll: its consumer drained until Empty while a concurrent producer could replenish progress. The owned receiver now consumes at most32 messages per poll, counting cancelled/stale progress as work; the same constant retains channelcapacity. Exhausting a quantum while the operation remains active wakes the native event loop for the next poll. Terminal outcomes, committed saves after cancellation, pending requests and disconnected-worker retirement keep existing semantics.

Schema and ten language-neutral vectors were authored before the implementation. They cover empty/single, terminal-at/after32, 63/64/65 progress and cancelled cuts. The actual private controller law uses those vectors to assert exact1/2/3-poll settlement. Existing dashboard source-freshness selector gains a schema/Ajv plus independent SQLite poll-count law; its two JSON data inputs are explicitly declared as freshness, without changing runtime source entries or adding a target. Existing native source_ selector now includes four laws; prior three-body green run is historical and does not verify this change.

Current checks: schema envelope plus all ten independent SQLite DATA calculations agree, TypeScript syntax audit clean. These are data/source checks only. Actual Source/native controller bodies and wake responsiveness under installed PTY remain unrun. Low independent source review requested; High owns real1MiB installed editor/load budgets.

## Exact Owned Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🧪️tests/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🧪️tests/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🧪️tests/🥒️.feature`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧊️execution/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json`

Low independent source readback accepted bounded draining (including cancelled/stale messages), full-quantum native wake and unchanged terminal/committed-save handling. Confirmed frame() does not call poll_documents, so unit poll counts are not double-drained. Test-only unbounded channel intentionally permits more than32 pending messages to exercise the per-poll bound independently of production channel capacity. No actual law body pass yet.
