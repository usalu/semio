# Base64 Twin6 Terminal and Reentrancy Audit

Read-only source audit: defining Base64 TS module and Rust cursor. No production edits.

## Actual blocker

TS progress is user code and can synchronously reenter the same cursor. step checks refused only on entry; advance does not check it after checkpoint. A bounded actual Bun eval with text TWFu, grant100, allowance3 and first progress callback calling cursor.cancel() then returning true produced:

`[DEBUG] {"state":"complete","complete":true,"refusal":"Error: intrinsic bytes cancelled","written":3}`

This is runtime-confirmed contradictory success/refusal, not speculation. intoParts inside callback is also source-permitted: it clears text/output and marks withdrawn, while outer advance can subsequently allocate and alter state. Nested step can restart phase-zero processing. Rust mutable borrowing prevents equivalent safe callback reentry. Add a private executing guard covering step and any consuming/mutating operation; prevent recursive step/withdrawal/take during callbacks. If cancel is intentionally permitted during callbacks, check frozen refusal immediately after checkpoint and before reservation or further work, and ensure completion never wins over prior cancellation. Regression vectors must exercise these callbacks directly.

## Tagged outcome and remaining semantics

Root identified current parts refusal:unknown plus complete:boolean cannot distinguish Pending from Refused(undefined/null). Replace with one Pending/Complete/Refused(cause) outcome in both languages; no compatibility fields. Keep written separate in TS because its full-length typed buffer has a meaningful prefix. Rust vector len already represents that prefix. Completed parts may legitimately lack output after successful take; outcome Complete describes processing, not remaining custody. Keep exact one-take and consuming withdrawal semantics explicit.

Invalid TS grants currently throw RangeError before the catch and do not freeze refusal. This is distinct from advancement failures. Decide admission-error retryability explicitly; if first-refusal means every step exception, move grant admission into the frozen path and test subsequent valid calls. Existing input text is private, immutable and retained without full conversion. Quad primitive uses low24 data/count high bits and creates no per-quartet array/result object. It preserves public quad codec-before-output refusal. TS now includes Rust nonfinal padding precheck. Progress units remain UTF16 code units for TS text versus input bytes for Rust; ASCII transport shares counts, arbitrary Unicode error values/index are not identical cross-language.

A grant bounds byte/code-unit visits, not allocator latency; Decode-start1 may reserve the full admitted output. Retained typed-array identity does not establish physical VM allocation behavior. The synchronous collector drains the same class and intentionally relies on GC retirement on thrown error; no synchronous physical cleanup claim.

## Actual receipts

native-6 code0:14 Rust tests passed, no failures/ignored/filtered. source-6 code0:7 Bun laws passed,2859 expectations. strict-6 code0:diagnostics[]. Each captures21 selected bodies plus producer. Independently recomputed all22 UTF8 SHA256 hashes and compared current physical bodies: all exact at audit time. These passing rosters do not contain the reentrant cancellation regression above. New tagged-outcome publication will require fresh joins.
