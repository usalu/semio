# Root CUT1 R3–R4 Review

## Outcome

The ticket-only recovery/probe declaration is reviewed, not compiler-ready. The next approved work is an isolated neutral root-primary/recovery-pin test/API preparation in the existing native resident crate. No cross-crate CUT1 mount, allocator change, native command or production edit is authorized by this review.

Root read the complete [hardening report](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️runtime-opening-cut1-hardening-r3-r4-2026-08-28.md), staged 403-line native tests, 115-line probe DTO, 177-line controller, schema and fixture. Root also parsed the complete retained R4 command/terminal/pre/post output and independently opened all eight current named files without following a final symlink. Descriptor/path identities stayed equal during each read, and all current SHA256s match the retained eight-source boundary. This is a new readback, not a new reference or native test run.

The retained R4 actually exited0: six CUT1 reference cases/32 transitions, ten Buffer vectors, three pin cases/25 transitions, six ordering negatives, eight null-observation negatives and eleven schema-hostile cases. The separately retained original Opening seven reference cases/39 transitions remained unchanged. Rust bodies, native layouts, allocator hooks and private recovery methods were not executed.

## Required Corrections Before Native Mount

1. The pin reference state holds aggregate next/found/pin counts. It does not identify two actual nodes or their original registration/root. The smaller neutral packet must use at least two same-type nodes and a foreign root, preserve per-node pin identity/counts, and reject wrong-node release and successor substitution. R4's count/order result is not expanded into that proof.
2. The staged native allocation law currently assumes every allocation's original charge is only its node Layout plus one slot/owner. Actual [reserve_record](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:668) uses the caller's domain envelope **plus** intrinsic RecordNode bytes/slot/owner. The future plan must distinguish allocated node Layout from total reserved resources, and compare the actual canonical domain envelope plus intrinsic charge. No zero envelope is selected merely to satisfy the staged law. The agent confirmed this mismatch; actual runtime registry envelopes remain a separate unmounted join.
3. Cross-crate cfg-only probe exposure, fixed-buffer setup and allocator integration remain undecided. A nominal lexical native-test roster cannot supply that missing source or compiler evidence.

The neutral primary slice may exercise ConsumerPage-only reservations without Record envelopes, but it must explicitly retain the actual envelope-plus-node rule for the later runtime join. It must reuse the original root, consumer list, single Release and allocator; no second ledger or parallel registration registry.

## Preserved Boundaries

Native resident e23ec406 and its e81bcca1 tests are unchanged. Existing resident25, Opening7, Store/backbone ownership, full actor ingestion, client construction and scheduler-tail quiescence remain separate. No cleanup, restoration, source move or capacity increase occurred.

Complete fresh readback, exact diagnostic controller and retained command evidence: [JSON](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️coordinator-cut1-r4-review-2026-08-28.json).
