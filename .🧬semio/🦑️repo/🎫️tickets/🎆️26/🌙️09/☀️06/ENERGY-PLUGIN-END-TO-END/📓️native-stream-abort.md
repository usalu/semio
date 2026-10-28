# Native Stream Abort

The mounted empty abort still invoked the synchronous Store abort requiring cold snapshot and mutation capabilities. Its required behavior is removal of the unpublished open edit, restoration of the prior projection, unchanged redo authority, and no added transition, causal operation, or outbox message. Undo and Revert cannot implement that behavior.

The shared `abort.json` fixture and source-homed native test were authored before the replacement. `🧪️native-stream-abort.py` independently exercises the same output using a real SQLite transaction and rollback.

`OriginalStreamAbort` retains the actual incoming transaction text identity, original caller and decoder recipient. It prepares the selected native Report without the final open edit and prepares the actual applied metadata while preserving the existing redo roots. The prior projection, applied metadata, removal of the original edit and original open-transaction owner move in one funded publication. Displaced payloads retain their real retirement issuer.

Message removal has a defining retained cursor that visits one original hash bucket per turn and returns the actual prepared ticket and bucket. Publication unlinks that exact row without the synchronous identity table scan. History removal preflight authenticates the exact unborrowed generation before companion roots change.

The lower native law passed (`native-stream-abort-law-9.log`, one test, seven cases, 3.30 seconds). It covers the real native genesis projection, a retained open edit and nonempty original diagnostic row; publication restores the prior value and removes the edit, messages, applied ID and open transaction without adding redo, transitions or outbox messages. Birth denial, a foreign decoder, six cancellation prefixes, every positive selected-close currency denied by one, unchanged denied quotes, exact allocator/receipt equality, and full source/Store/decoder/caller retirement pass. The independent SQLite transaction oracle also passes.

The test exposed a genuine cancellation bug: an underfunded close attempted to enter the native retirement context before checking its selected demand. The owner now checks every selected currency first. The fixture was aligned with its defining native genesis value, 1.25, and asserts that initial value before constructing the stream.

Mounted publication retains the actual Abort owner and original transaction, synchronizes its fixed original publication receipt before continuing, and includes the source in terminal ownership and contextual Store retirement. The mounted graph check is in progress. The empty streamed commit path still needs its own native announcement producer; this report does not claim that the whole streaming subsystem or Energy goal is complete.
