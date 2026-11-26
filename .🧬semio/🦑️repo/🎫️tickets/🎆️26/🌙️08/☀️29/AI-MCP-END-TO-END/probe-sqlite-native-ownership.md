# Probe SQLite Native Ownership

The neutral Probe SQLite suite reproduced one failure. Its numeric TEXT and source-ordinal test looked for the nested receiving call in the Probe snapshot adapter. Both operations delegate to the shared SQLite snapshot capability. The Probe supplies its pack receiver and canonical JSON writer. The shared decoder and encoder scope the original native owners, receive into nested slots, settle the retained body and move the received result from its slot.

The neutral test now follows that call chain. It exercises the same exact number words, shuffled SQLite row order and member ordinals with Bun's independent SQLite engine. Its source checks verify the two Probe callbacks and nested ownership in the actual decoder and encoder functions. Production code and fixtures were unchanged.

`SEMIO_TEST_ORPHAN_GUARD=off repo test-file` on the SQLite test file reproduced 5 passing tests and 1 failure. The same command passed all 6 tests and 133 assertions after the correction. The test runner's standalone orphan scan is disabled because the sandbox denies `ps`.
