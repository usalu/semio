# Note Browser Startup Stall

Chromium received the Note availability snapshot at stream sequence zero. The browser context recorded successful plugin acquisition, the shard instantiated the real Note component, and activation returned idle. The first instance-open request stopped returning after a series of successful guest turns. The active shard stopped servicing its timer and CDP evaluation, while the three idle shards answered.

V8's active-shard CPU profile identified Wasm function 85473 at byte 34,964,851 as the hot frame. The exact served core's function body matches `RuntimeInstanceRegistry::next_entry_from`: its 16 occupancy words, first-word suffix, last-word prefix, and trailing-zero index produce the slot, instance ID, and borrowed cell. That lookup intentionally wraps. The original picker-effects scan called it without a wrap guard, so the first live instance with an empty picker outbox was visited forever. The scan now stops when a lookup wraps behind its cursor, before reading that instance a second time.

The six other cyclic lookups use one of three finite patterns. Close and live cleanup each take one instance. Result publication and typed operations stop at the first repeated slot. Displaced-surface and pending-effect processing already stop when the result lies behind the cursor.

## Verification

- The Playwright startup and CPU probe established the red case against attempt 51. A Playwright experiment serving the same attempt 51 component with only the compiled guard showed the instance-open request return after 37 continuation polls. The editor finished document loading and reported both Canvas and Navigator. The owned guest rebuild is still pending.
- SQLite independently sorted the shared fixture's occupied rows by slot and returned `[]`, `[7]`, `[1023]`, and `[1025, 511, 2047]`. [SQLite oracle](picker-scan-oracle.py)
- `repo cargo --store=shared --configuration=prototype -- test -p semio-plugin-tests-12-2 --lib picker_lifecycle::idle_picker_turn_scans_each_occupied_slot_once -- --exact` runs the native neutral regression. The fixture application fails fast if a visit is repeated, so a regression need not wait for a watchdog. The first run exited before compilation after a shared Cargo preparation schema changed. The retry reached compiler-disk admission and was canceled to free capacity for the owned guest rebuild. The companion test has yet to execute.
- A standalone Rust probe compiled the current picker scan and cyclic lookup in memory using `repo tools exec --light rustc -- -Awarnings -o <ticket generated binary> -` and ran under `repo capacity run --memory=64`. The probe failed on a repeated visit in the baseline without the guard (exit 101). The corrected functions passed all four neutral rows (exit 0) and matched SQLite. This probe used idle application doubles and the real scan and cursor code.

```sh
repo tools exec --light python -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️02/NOTE-INFINITE-CANVAS-APP/picker-scan-oracle.py' '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🧫️fixtures/🔣️.json'
```

## Changed Files

- [Picker-effects scan](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🎞️raw-media/📤️picker.rs)
- [Picker regression](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/📤️picker/🦀️.rs)
- [Neutral reactor fixture](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🧫️fixtures/🔣️.json)
- [Native test registration](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🪝️tests-12-2/🦀️.rs)
