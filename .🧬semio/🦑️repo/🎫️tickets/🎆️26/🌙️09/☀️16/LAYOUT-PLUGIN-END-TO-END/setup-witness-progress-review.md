# Setup Witness Progress Review

Read-only source audit during live canonical setup 24933. No live filesystem/cache inspection, effects, jobs, source changes, controller action or cache action was performed. Existing setup-live-review.md supplied the phase context. No file-count, completion fraction or ETA is inferred from telemetry.

## Concrete Performance Finding

There is a clearly superlinear directory enumeration path in the witness owner, although this inspection does not establish that it dominates the live run.

Owner: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🟦️.ts`.

The metadata logical frame at line 79 opens and enumerates the entire current directory for every selected page, retaining only the next lexicographic nameWindow=64 entries beyond cursor. It visits those children, advances cursor, then reopens and scans that same directory from its beginning. It performs a final empty-page census too. For a stable directory with N entries, this entails N*(ceil(N/64)+1) name reads, plus end-of-directory reads. Per-name selection insertion is bounded by64. Thus broad-directory name enumeration grows quadratically in that directory width; it is not a whole-tree rescan after every file.

The content census directory branch uses the same full-directory-per-page algorithm at line41. The complete content census is executed twice at line51, after the preliminary logical frame at line24. This repeats the broad-directory cost across all three passes. It is a concrete candidate for a future traversal-owner performance improvement that preserves complete witness, stable ordering, all existing custody/identity checks, cancellation, bounded resident windows, and unchanged caps. No live alteration or shortcut is proposed.

Other repeated work is bounded-depth ancestry checking: metadata verify at line72 walks the current chain before/after each visit (76,79,80), and content verify at line32 does likewise (38,45,47). With maximumDepth64, this is bounded-depth per-entry overhead, not an unbounded growing history or repeated whole-tree search. The private observer also rechecks live publication custody, as documented in setup-live-review.md; this review did not weaken or bypass it.

## Progress and Cancellation Boundaries

Metadata frame lines64–69 define check, io, step and progress. check requires issued resource control, checks resource lease control/cancellation, and invokes the private publication observer. Each lstat/readlink goes through io with checks before and after. Every directory read goes through step with checks around the read. Enumeration yields every64 names through progress; progress publishes after256 charged work units and always yields a continuation. Verify yields every64 ancestry entries. Every recursive visit checks on entry and validates its named facts and chain after work. The metadata work budget is67108864; resident rows4160; path256; depth64.

Content census lines26–33 similarly check issued control and cancellation before/after each bounded io operation and around async step/sync batches. Directory enumeration checks before each read; its step performs full checks before/after at most64 reads, then progress may publish/yield. Descriptor acquisition and closure receive checkpoints through owned at line33. Each file stream at line45 reads chunks of65536 bytes, checks control before/after every read via io, hashes at most64 chunks per synchronous batch, then yields and checks progress. Thus one batch hashes at most4194304 content bytes; a large file is not read or hashed in one uninterrupted whole-file step. Symlink payload is bounded to65536 bytes. Per-entry metadata frame/path sizes and chain depth are bounded.

The two full content streams are deliberately complete comparison passes (line51), not an uncontrolled retry loop. The source rejects mismatched censuses. No unbounded CPU loop without cancellation checkpoints was found in these selected traversal/content paths. Individual synchronous filesystem calls can still take wall time; this source inspection does not measure kernel call latency or promise an absolute cancellation-time bound.

## Conclusion and Limits

Actionable source-level performance candidate: repeated full-directory lexicographic paging in both metadata and content traversals. Existing cancellation/progress checkpoints are present around bounded metadata/content steps. Nothing here establishes a hang, lease recursion, a current census count, or a safe ETA. The shared resource/mode rows remain telemetry, not a new lease acquisition. Preserve canonical setup24933 and await its genuine terminal/stage evidence.
