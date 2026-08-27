# Shared Generation and Compound Read Review

## Native Compound-Read Result

The two compound-envelope native laws now **pass: 2 passed, 0 failed, 874 filtered**, 0.02 s runtime after 1m37s compilation. The actual Rust serializer injection and foreign-authority rejection executed; all four fixture cases emitted debug evidence. Log: `🧪️coordinator-group-envelope-native-r1-2026-08-27.txt`. Generation-root, Store snapshot/revision/generation group selection, and mounted atomic child publication remain unverified or incomplete as distinguished below.

## Exact Source Inspection

The coordinator read the complete shared generation root implementation and all four native laws, the registered latest-wins test helper, and the new Store/VCS compound envelope capture and two native tests.

`GenerationPlayRoot` shares an immutable allocation, denies mutable cold-builder access when shared, and transfers its exact final allocation into a retained cursor. The cursor advances through generation entries, JSON object/array frontiers, and actual string bytes. Explicit cold retirement is a synchronous batch path, not an interactive step. Final-owner and unfinished-cursor Drop guards retain deep ownership while reporting a lifecycle violation instead of recursively destroying arbitrary content. Four native laws cover wire parity, shared-root survival, mutation refusal, zero grants, transfer between threads, and drop/unwind guards; none executed in the failed Flow build.

The new VCS compound read captures one private visibility decision before serializer callbacks. Each history iterator stores fixed selected endpoints and length; the cursor selects against that same exact authority. The envelope rejects a foreign history/cursor authority before emitting output. The tests inject a commit while the initial snapshot is being serialized, ensuring an already-captured read stays old while a fresh read sees the new history and cursor together. This establishes the intended source-level history/cursor observation contract; native execution is queued. Store snapshot/generation/revision selection and the real mounted atomic child-group/log/undo pipeline remain separate unfinished work.

The registered latest-wins helper supplies actual command binary encoding, registry admission, typed dispatch, key reservation, a document revision change, fresh worker creation, stale-key rejection, Store publication, cancellation, and bounded app close. It is deeper than a standalone registry test but is still an in-process fixture, not browser input or every real app. Its expanded ten-test native cohort is running separately.

## Allocation Boundary

The selected-copy allocation budget is local admission accounting, not a process-wide allocation lease or evidence that arbitrary contiguous allocation is below eight milliseconds. The executor's `📓️parameter-allocation-admission-seam-2026-08-27.md` correctly separates those questions. Existing payload-page credits cannot be borrowed for naked Vec/String backing whose lifetime continues after a job publishes. An allocation token must remain inseparable from its backing through publication and final retirement. Full-domain latency also needs bounded-page backing or executed maximum-allocation evidence; no latency exemption was authorized.

## Executed Source Checks

- Tool-job self-tests: 785 pass, 33 exact owners, 254 custom rows, 25 generic rows.
- Flow session source: one fixture, three hostile rejections, 42,405 semantic bytes, grants 1/64/4096.
- Flow app source: four recipe fixtures/four hostile rejections with 4,800-byte semantic labels; four parameter cases/ten hostile rejections; two Node Buffer byte-retirement oracles/three hostile fixture rejections; existing canonical and identity fixtures also pass.

These source checks do not execute the Rust state machines. The Flow native gate stopped at 171 upstream B-rep extension errors before running any selected test. See `📓️coordinator-exclusive-compiler-queue-2026-08-27.md` for the exact current queue.
