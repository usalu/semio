# Dependency installer stderr receipts

The Windows m05 dependency attempt reported installer error reference `8ef44f48` on 21 November at 19:44 UTC. The original root-scope child exited with status 1 and no signal. Its stdout had 31 bytes and its stderr had 188 bytes. The machine record retained stderr SHA-256 `58adf23a19919199b83f98c9131227a698694f9f11abb15757f9bfa55442c0c0`.

The live dependency log and the separate failed-attempt mirror retained the same truncated diagnostic. Both end inside the opening resolve/download lines, after the output hashes. The local origin had no separate failure record named for `8ef44f48`. Its containment and stall files did not contain the missing installer stderr. The available evidence does not establish the underlying installer failure.

The producer still emits the existing machine record. It follows that record with a standalone JSON suffix of the already-redacted stderr. The additional line's authored limit is 400 bytes including its prefix and JSON escaping, which keeps it intact in the existing 400-character mirror. Both the frozen-lock refusal and the generic installer error use the original owned stderr sink, and an output failure leaves the primary error intact.

The neutral fixture reproduced the missing excerpt before the change. Its synthetic 188-byte case verifies that a terminal reason after the resolution progress survives the mirror limit. Existing cases exercise redaction, Unicode, ANSI controls, signals and sink failure. An independent SQLite recursive query finds the longest JSON suffix within the same byte budget. The focused diagnostic and producer failure cases passed three tests with 376 assertions.
