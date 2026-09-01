# Authored Fixture and Streaming Decoder Boundary

## Current Authored Fixture Debt

The nineteen strict diagnostics point to removed UI pool constructors and one removed public page byteAt method in UiDocumentStore. The old tests additionally assume reservePage/capture, whole-copy-ready before reader admission, counter-only page charges, and automatic cleanup of arbitrary faults. Those assumptions conflict with the actual shared ledger, registered early reader, one-page window and retained unknown-fault contract. They must be handcrafted against current real authorities, not silenced, deleted or supported with compatibility methods.

The old top-level UI resident capacity schema still declares maxResidentBytes/maxPages/maxOwners and its contract names reservePage/capture. Only the three old OwnedResidentPool tests import that schema; current actor composition imports the neutral value resident schema. The old declaration is therefore not the current source of runtime capacity authority. The migration must make the UI declaration refer to its explicit original Shard/shared neutral ledger and current metadata/child phase contracts, preserving the neutral capacity's bytes/slots/owners/data-control semantics rather than inventing another UI limit. All source and fixture declarations must change together.

The retained laws remain: exact original admission before constructor escape, genuine foreign lifetime/field rejection, clean saturation refusal without mutation, all cancellation prefixes, independent live read aliases, before/after write/read/source/finalizer faults, and terminal child work separated from parent observation/refund. Fault tests must now assert the original charged fault remains held; they cannot claim arbitrary exception graphs were retired merely because caller code stopped referring to them. Existing tests that waited for copy-ready must instead obtain and service the original early reader.

## Genuine Semantic Consumer

RetainedUiWireValueCursor currently transfers a whole Uint8Array, creates NumericIndex/Frame/Owned containers without the new ledger, and uses random-access subarray/TextDecoder at text boundaries. Replacing only its byte accessor with the early reader would neither remove those assumptions nor account for parser/output ownership.

The live consumer therefore needs a separately declared concrete decoder owner installed before allocation and a bounded same-ledger roster for symbol entries, text chunks, frames and output nodes/Surface bytes. The current parent payload has no decoder field/slot authority beyond its already inventoried children. Any new pointer, controller, fixed parser state or result registration must be inventoried and charged before construction. The existing seven-word slot can be reused serially, but cannot fund uncounted concurrent decoder descendants.

The semantic decoder must receive the original private reader before builder-ready, pull one scalar in a source turn, and apply it in a distinct parse turn. Text, varints and container headers may cross destination pages without holding a consumed source alias. Its parsed output is a typed retained owner, not a reconstructed whole operation buffer or arbitrary JSON compatibility object. Cancellation must retire the current reader alias and parse descendants before any parent/domain refund. Surface and final tree ownership must remain on the same explicit composition budget; successful parsing is not publication or semantic ACK.

This is a design inventory only. No decoder API, admission price, successful parser or live consumer is declared implemented here. Copied-range/reader foundations remain the latest separately tested runtime boundary.

## Preserved Authored Admission RED R158

The unchanged registered `OwnedPagedAdmission` selector fails both old tests at the removed public pool constructor:2 failed/733 skipped/735,3.26 seconds,start05:00:44,Nx1. The precise error is Invalid resident pool authority before either old test reaches its semantic assertions. Full output:`🧪️renderer-authored-admission-red-r158-2026-08-28.txt`. This is an authored-caller migration RED, not failure of the released genuine phased admission cohorts. No test, source or schema was changed by this diagnostic. No own process remains running.

The next first two authored tests will use original Shard/pool/scope/field admission, preserve zero-grant and foreign-authority conservation, accept a fragment before asserting cancelled caller-loss recovery, and distinguish a truly unoffered fragment from transferred input. Duplicate same-field admission must recover the same original payload/builder rather than invent a second payload. A clean refused native release remains retryable with the exact evidence held; arbitrary exception faults remain charged. Exact final child work must precede parent record cleanup on a later grant.
## Authored Admission Join R159–R160

The two existing `OwnedPagedAdmission` bodies now use the original captured return/response, shared Shard ledger, pool, scope, field-owned payload and thirteen-phase builder admission. Zero-item admission preserves the actual ledger snapshot. Repeated genuine lookup returns the same original builder; changed lifetime, structural field and a genuine foreign field reject without admission. Caller abandonment first captures the real input, then the original instance drives a finite close bound derived from the input/evidence/builder/payload/scope declarations. No source byte is read, one original cancelled receipt is issued, and the pool returns to its pre-scope snapshot, not global zero. The unrelated actor/source reservations remain funded.

The second law retains source release refusal and retries the same evidence. Actual terminal builder work is injected at 4096 bytes; the parent forwards all 4096 before a separate 64-byte domain unlink, then each original registration refund phase. No unknown fault is forcibly retired.

R159 executed 1 PASS / 1 FAIL / 733 skipped, 735 discovered, 3.31 s at 05:14:51, Nx1. The failure was only the authored diagnostic spelling (`paged-input-release-refused` versus actual `paged-evidence-release-refused`) after reaching the real refusal. R160 corrected that spelling and executed 2 PASS / 733 skipped, 735 discovered, 3.72 s at 05:15:16, Nx0. Logs: `🧪️renderer-authored-admission-r159-2026-08-28.txt` and `🧪️renderer-authored-admission-r160-2026-08-28.txt`. Command: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedPagedAdmission'`. Only the two authored Store bodies changed; no production, capacity, schema or peer source changed. Remaining authored cohorts and strict/broad gates are still open.

## Authored Migration R161–R177

All nineteen removed-API strict diagnostics have been eliminated by authored caller changes, not compatibility exports. Current R177 is Nx1 with exactly the seven preexisting tutorial joins; no retained UI, actor or authored fixture diagnostics remain. There are still 735 discovered tests: no existing test was removed. The combined seventeen `OwnedPaged|OwnedResidentPool` tests execute the original laws on current phased authorities.

| Run | Actual outcome | Scope and correction |
| --- | --- | --- |
| R161 | Copy1 PASS, 4.89 s,05:17:36 | Early reader, two windows, exact Copied proof, parent-owned close |
| R162 | Fault1 PASS/1 FAIL,4.38 s,05:17:37 | Read exception retained but the authored probe stopped before its two additional cell-handoff phases |
| R163 | PartialFault1 FAIL,4.40 s,05:17:37 | Authored assertion incorrectly demanded immediate builder-to-cell transfer despite the still-held page binding |
| R164 | Fault2 PASS,6.14 s,05:18:28 | Exact four reader close phases and original exception captured |
| R165 | PartialFault1 PASS,4.25 s,05:18:31 | Original builder.failure identity remains charged; page/parent cannot falsely retire it |
| R166 | Registration1 FAIL,5.66 s,05:19:47 | Authored neutral case spelling source-before instead of source-bind-before |
| R167 | Boundary1 PASS/BoundFault1 FAIL,6.01 s,05:19:49 | Same neutral case-name issue; full-grant terminal source boundary itself passed |
| R168 | Continuation1 PASS/1 FAIL,5.70 s,05:19:49 | Source exception belongs to original evidence parent cell, not builder's diagnostic result |
| R169 | Registration1 PASS,6.91 s,05:21:00 | Exact existing neutral case names |
| R170 | BoundFault+Boundary2 PASS,5.65 s,05:21:02 | Genuine after-bind fault held; 4096-byte terminal work observed on later64-byte turn |
| R171 | Continuation2 PASS,5.62 s,05:21:02 | Original evidence-cell fault identity, refusal/over-grant/after-work assertions |
| R172 | Cancel1 PASS,22.14 s,05:24:06 | Every prefix of declared257-byte reader/producer/evidence sequence |
| R173 | Pool1 PASS/2 FAIL,4.16 s,05:27:10 | Missing derived alias sum and imprecise instance-close versus payload-close fixture expectations |
| R174 | Strict9 diagnostics | Tutorial7 plus missing authored aliasTotal twice |
| R175 | Pool3 PASS,4.10 s,05:27:54 | Exact intrinsicReader+admission sum; alias remains readable through instance close until exact payload close |
| R176 | Combined17 PASS/718 skipped/735,23.99 s,05:28:13 | `--args='--run -t "OwnedPaged\|OwnedResidentPool"'` with inner quotes preserved; Nx0 |
| R177 | Strict7 diagnostics, Nx1 | Exactly unchanged tutorial joins; all nineteen old API diagnostics gone |

The R172 source asserts every generated prefix and emits a DEBUG census, but default Vitest output suppresses that console line; no printed numeric census is claimed. Its finite sequence derives directly from current reader admission/alias/close, page admission/binding/close, evidence admission/retirement, and byte count. Source reads and destination writes have separate grants. The test obtains the reader before producer work, consumes every page before the next window, and compares read bytes with Buffer. Cancellation never advances source consumption, never substitutes a cancelled token for an already installed Copied token, and leaves unaccepted input with its original source.

The shared-pool law uses two genuine clients and two privately owned pools on the SAME injected 65536-byte fixture ledger. Both observe the same exact Immer resource deltas. A genuine unsubmitted external backing reservation saturates the remaining data partition without allocating its backing or posting; UI page admission refuses without mutation while a separately granted control admission still succeeds. Its exact intrinsic/cell retirement restores the original shared snapshot. This is a reservation/conservation proof, not physical heap certification or posted-response retirement.

The canonical UI capacity schema now references `semio.value.resident.capacity.v1`; it no longer declares independent maxResidentBytes/maxPages/maxOwners. The UI contract points to each actual child declaration and distinguishes operation/instance close from payload close. The old resident neutral fixture and wire-pages capacity/admission/binding oracle are handcrafted to current shared authority. Native payload pattern/length/fragment rows are unchanged. A fresh framework TS/TSX consumer scan found the stale counter properties only in the remaining authored Store body before its replacement; actor/kernel scans found none. No production TS, neutral runtime, actor price, shared capacity, native source or command configuration changed.

Two post-R177 coherence refinements correct wire-pages fixture relative declaration links from four parents to three and make the existing constructor/binding helper consume the authored neutral binding rows directly. Those refinements require the following final broad/strict gate; R176/R177 remain their own captured outcomes. Original logs are retained as `🧪️renderer-authored-*-r161` through `r177-2026-08-28.txt`. No full735, live decoder, semantic publication or raw-input acknowledgement claim follows from these selected passes.
