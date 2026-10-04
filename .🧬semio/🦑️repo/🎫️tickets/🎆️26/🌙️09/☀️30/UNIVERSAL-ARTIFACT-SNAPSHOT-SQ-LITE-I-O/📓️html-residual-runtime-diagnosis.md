# HTML Residual Runtime Diagnosis

After the first genuine 23-law baseline authorized five repairs, Root ran the mounted cohort again.

Nextest `800db4cb-5ad8-41e8-8a12-8eefb5df335d`: 23 laws executed, 21 passed, two failed, 43 outside the selection; runtime 291 ms, Nx 50.2 seconds. First-body normalization, exact normalized returned rows, and allocation-denied deep/wide linear retirement now pass. The late SQL metadata child still aborts from stack overflow on its declared 64 KiB stack.

The other failed law canceled the first binary UTF-8 validation stage. That stage borrows bytes and has no owned allocation; a zero settled ledger is correct there. The strengthened witness selects the distinct actual 100,000-byte entity materialization stage, after ownership admission, and separately asserts that borrowed validation stays uncharged. The repeated exact budget, 100,000-byte payload, 65,536-byte cancellation threshold, and positive refused-copy ledger assertions remain intact. Production ledger behavior was not changed to hide this fixture witness error.

Root’s focused late-SQL rerun `0accaa94-63aa-4dd3-9d13-11ae8f86db33` reproduced the stack abort: one executed, one failed, 65 outside the selection, 115 ms. Temporary `[DEBUG]` fixture stage logs placed after complete input-tree creation produced no marker before the abort. Root is narrowing that pre-marker path with a thread-entry marker and uses the exact binary path from Nextest metadata for stack-frame inspection. No conclusion about the cause is claimed yet. These temporary logs must be removed before final changed-source verification.

## Exact Worker Inspection

Root's entry-marker rerun `42f34bb9-0625-4c73-854f-ac569336b77b` executed one law and failed, with 65 outside the selector, 119 ms of assertions and 76 seconds in Nx. Its child still displayed only the stack-overflow abort. This absence of debug markers does not establish that the closure never entered: the child lacked `--nocapture`.

The actual installed nightly libtest source (`library/test/src/lib.rs:160–166`) registers a thread-spawn hook that copies the parent's output capture into spawned workers whenever `nocapture` is false. Its unwind-only normal test completion returns buffered output. A fatal stack-overflow abort can therefore lose prior `eprintln!` markers. The isolated child now adds `--nocapture`, preserving the full exact selector, one-test guard, depth 8,192, late byte frontier and 64 KiB worker stack. This is diagnostic output visibility only, with no ownership or behavior change.

Exact-symbol LLVM disassembly of the e5aeb owning binary proves the worker closure's prologue reserves `0x9a0 + 0x20` bytes, or 2,496 bytes. Its first normal action calls formatted stderr, before the leaf or deep-tree loop. The actual SQL reconstruction method reserves `0x3000 + 0x650 + 0x20`, or 13,936 bytes. These are actual compiled frames, not inferred sizes of source locals. Neither frame alone exceeds 64 KiB, and no cause is inferred solely from them. Generated read-only receipts are `html-late-sql-worker-exact-symbol-disassembly.txt` and `html-late-sql-reconstruction-exact-symbol-disassembly.txt`.

The old numeric-address disassembly is not treated as evidence because Root's owning linker replaced the same binary between symbol listing and address inspection. Exact-symbol disassembly avoids that stale-address mismatch. The diagnostic test parsed with rustfmt exit 0 (`html-late-sql-nocapture-diagnostic-parsed.rs`); no Cargo or native test ran in this executor. Root's fresh uncaptured focused trace is required to locate the last completed stage before a concrete repair.

## Independently Observed Fixture Frontier Mismatch

The literal late field is 32,768 repetitions of `界`: 98,304 UTF-8 bytes. An independent Buffer plus fatal platform TextDecoder accepts a prefix of 65,535 bytes but refuses 65,536, which ends inside a code point. The actual shared Rust `copy_text` loops back from its 65,536-byte chunk boundary until `text.is_char_boundary(end)` succeeds. Its two completed frontiers for this field are consequently 65,535 and 98,304.

At diagnosis, the staged law required `completed >= 65,536 && completed < 98,304`. Neither real frontier satisfies that predicate. Independent receipt: `🗑️generated/html-late-sql-independent-utf8-frontier.json`. This is a concrete witness-layout mismatch, not evidence that the completed-tree guard fails. A successful returned deep snapshot remained raw in the fixture's `result` when `assert!(reached)` failed, so that assertion's unwind could recursively drop the result and mask the original witness failure as a stack abort. The original child-abort receipts do not establish a completed-SQL-tree guard defect.

Any repair must retain the depth, stack and interior-copy obligation; it should observe an independently valid UTF-8 boundary and retire any unexpected successful returned snapshot before asserting failure, with production retirement visits measured before fixture cleanup. No fixture threshold or behavior was changed during this diagnosis.

The thread-entry rerun `42f34bb9-0625-4c73-854f-ac569336b77b` again executed one failed law with 65 outside the selection, 119 ms. Exact-symbol inspection measured the prior worker frame at 2,496 bytes, disproving a large closure frame explanation. The agent then verified that libtest installs output capture on spawned workers unless the child uses `--nocapture`; an abort loses those captured markers. Their absence is not evidence that the worker never entered. The exact isolated child now disables capture, preserving the exact-one selector, 64 KiB stack and depth 8,192, for the next focused diagnostic run.

## Witness RED, Correction and Current Source Receipt

The registered `@semio-tech/stdio-html-rs:test-snapshot-sqlite-source` route executed the new independent witness law before the fixture correction. It ran 15 tests: 14 passed, one failed, 146 assertions, Bun 971 ms and uncached Nx 7.1 seconds. The only failure was `HTML late-copy refusal has an independently valid interior UTF8 witness`, which found no valid interior prefix at or above the old 65,536 predicate. Receipt: `🗑️generated/html-late-utf8-witness-source-red.log`.

The closed neutral fixture and its schema now explicitly own `cancelUtf8ByteBoundary: 65535`. The original 65,536-byte target, 98,304-byte literal field, 8,192-node depth and 65,536-byte worker stack remain intact. The Source law uses independent Buffer and fatal platform TextDecoder to prove that the exact literal prefix is valid, the original target cuts a character, the alignment differs by fewer than four bytes, and the selected prefix lies inside the full field. Existing independent Bun SQLite queries still verify the exact schema and doctype byte lengths and relational ownership.

The Native law selects that exact completed byte boundary only inside `SqlLateCopyScope`, after the complete SQL tree has been built. It captures actual production retirement visits before fixture cleanup. Any unexpected successful candidate is explicitly retired before assertion, so a missed cancellation now reports the failed predicate without recursive drop of the completed deep result. All temporary Native lifecycle `[DEBUG]` markers have been removed; `--nocapture` preserves ordinary failed-test diagnostics and the exact-one child guard.

The corrected route executed 15/15 tests, zero failures, 151 assertions, Bun 886 ms and uncached Nx 5.9 seconds. Receipt: `🗑️generated/html-late-utf8-witness-source-green.log`. The current Native lifecycle facet parses with rustfmt exit 0 (`html-late-sql-witness-corrected-parsed.rs`). No Cargo or Native test ran in this executor.

Root's prior uncaptured diagnostic attempt stopped before assertions on a shared physical transfer type-inference prerequisite. It is not a fresh HTML feature verdict. A scoped raw-map guard comparison and restored production focused law remain Root-owned Native obligations. The Source correction does not by itself certify either production guard behavior or native small-stack execution.
