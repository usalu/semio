# Compiler failure during source drift

The Procedural host descriptor build reported a changed plan after three failed compiler attempts. The actual host actor-input compiler had exceeded its library ceiling at 2,179, 1,638 and 1,833 MiB. The failed-plan recheck then reported edits in independent sources, obscuring the repeated compiler failure in the retry summary.

The compiler now retains the first resource cancellation or compiler error across both forwarded and suppressed output. A failed build whose plan also changed emits a structured diagnostic containing the original compiler failure and the actual conflicting input. Source retries and success custody checks remain in force. Two focused Bun tests passed with twelve assertions. SQLite independently selected all three expected outcomes from the neutral fixture.
