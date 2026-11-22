# Archive replay worker retirement

The fresh native editor finished decoding the parent Pack and replaying the forward and inverse history. It then refused the Store replacement initializer retirement because the worker's current receipt was shorter than the effects observed in the original recipient.

`archive-worker-close.gdb` traced the initializer's replay closure. Its final native close posted six items and 810 copy bytes. Four items and 728 copy bytes redeemed previously held return credit, leaving a current receipt of two items and 82 bytes. The same callback advanced the next retirement rung, which posted and reported one item, 2,448 copy bytes and 2,400 capacity bytes. The worker compared only that second report against all effects posted by both rungs.

The replay initializer now reports the exact current receipt of the native close and yields after any paid completion. The worker authority, live session and parked session wrappers subtract checked held returns before comparing current child effects to the reported receipt. Their existing underreporting refusal still applies to current effects. The native text and Pack reopen law reaches the repaired production route.

The independent SQLite ledger verified the missing current receipt as `[2, 82, 0, 0]` after subtracting the authentic held return and the next retirement step from the total observed recipient delta.

The v2 test linked after both source edits but used prior owner builds. The v2 symbolized trace `archive-worker-close-v2.gdb` measured the same old dual-rung total and the executable's close wrapper still subtracted the full recipient total directly. Its ordinary worker path already had the current-receipt helper before these edits. Native validation must therefore use a candidate that recompiles both `semio-framework-job-worker-authority` and `semio-os-store-initialization-replay` from the live sources.

The composed Store replacement initializer also needs the whole accepted original recipient delta, including held returns. It now checks the current worker receipt against its own phase grant, then reports the whole delta to the composed path. That path compares and settles the current two items and 82 bytes and restores the four items and 728 bytes already held at the Source in its report. The change is in `plugin/🗃️document/🧰️store-replacement-initializer-and-retirement/🦀️.rs`.
