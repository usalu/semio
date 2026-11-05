# Native History Receiving Compiler Check

The supported testing-laws request 15906 failed before its selected test ran. Store-codec pair receiving expected RetainedWorkOutcome from RetainedHistorySourceDecode::step and close_step, while the compiled dependency exposed Result at the two call sites (four E0609 errors).

The current authored History source returns RetainedWorkOutcome for both methods. Its public wrapper still uses the earlier Result shape directly, so current source coherence needs a focused compiler check before concluding that the earlier failure was only a concurrent snapshot difference. The next supported check selects the actual History package manifest. No fake work counters or alternate decoder authority are introduced.

The previous testing-laws request 91522 failed at workspace selection, and 15906 selected the correct package manifest. Neither executed a native fixture law.

The focused History request 27957 confirmed two current E0308 wrapper mismatches. Both calls now project the child's result after retaining its actual work counter, including refusal. The wrapper also forwards delegated retirement and decoder-recipient work, resets its counter on each turn, and exposes last_turn_work independently of physical receipts.

Replacement request 39957 passed the real History package library check (exit 0). This resolves the demonstrated compile mismatch; it does not establish a native test result or full caller work propagation outside this wrapper.

SDK request 21579 cleared the mounted-instance source admission and then found E0308 in the transition decoder's native actor-admission refusal branch: it attempted to return OriginalReceivingPrimary from a Result whose error is ValueError. The public step already refuses to enter decoding while a fault is retained. The native refusal now asserts that first-fault invariant and retains the original native error in its existing typed RetainedHistoryOpMetaFault slot, together with the actual receipt and work. No native error is flattened or discarded, and the public error API is not widened.

Focused record-decoder library check 13249 passed with exit 0. A new genuine native regression reuses the neutral transition corpus, refuses only the original actor allocation, checks that the native cause and receipt remain retained, verifies that a second step creates no allocation or work, and funds fault/partial/dictionary closure under the same decoder. Companion native request 60949 is live; no native regression verdict has returned.

The separate transition TypeScript oracle first failed after its independent byte/SQLite comparisons on obsolete source-substring assertions (including the removed private SharedUtf8 admission spelling). It now tests only the neutral wire and SQLite results, with explicit truncation checks. Native custody is covered by the real Rust laws instead of source-name matches. The first incorrectly specified Bun operand was refused before execution; the corrected explicit ./ path runs the owning test file.

The neutral transition oracle now passes twenty-six assertions. Native request 60949 compiled the companion test target until its new fixture callback failed the required Send bound because it captured Cell<bool>. The fixture now uses AtomicBool; replacement 84373 is live. No native regression execution is claimed yet.

A separate host-plumbing request stopped in Store codec receiving after newly work-bearing History transfer methods were still used as plain Results. A fresh inspection found that the shared tree already captures actual ready/fault/partial transfer work before inspecting results and receipts. Root preserved those concurrent fixes and is checking the actual Store codec package rather than duplicating edits.

Both replacement native requests are now terminal: actor-refusal companion 84373 and Store codec library 85415 each failed before their intended verdict on the same twenty-nine borrowed DSL model receiving API errors. Native selection owns that prerequisite's actual work/failure transport migration. The actor refusal regression is authored and parses, but has not executed.
