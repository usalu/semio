# Current Closure Schema Delta Observation

Requested expected delta pin21219/5a9efb30e751fc2c2792fc61a6f0b44902182169fdbd30ef1b8a5ef84da9a84e was not the physical file read. Current read was21219/`e22765bbc30236711122fe85d71615d989a32e6fb5e513248b79f2bc17732083`. Retained prior body21057/235b02ee is available and used for structural comparison; no invented intermediate5a9 body comparison. No Native or Source evaluation.

Exact structural differences against retained235b body:

- `/definitions/Protocol/const/maximumExpandedBytes`: 134217728 → 268435456.
- `/definitions/PayloadRefusal/properties/expandedBytes/maximum`: 134217728 → 536870912.
- `/definitions/PayloadRefusal/properties/maximumOutputBytes/maximum`: 134217728 → 536870912.
- `/definitions/GenerationReadProtocol`: undefined → {"const":{"version":1,"maximumRetainedTokenUnits":128,"maximumTokensPerByte":1}}.

The same-size current intermediate drift is material: the global decoded cap rises128→256MiB, while diagnostic expanded/output admissibility rises128→512MiB. The latter remains bounded and accommodates that new cap, but increasing a work bound is policy change, not merely adding unused DATA.

Related worker is now24686/`b436d28c199b3ce7b42962641c404bd84f97bcee5b0376dcf040b43d646ba1a7`, compared with accepted17364/9ccff028. It contains a new streaming generation decoder/tokenizer, consumes GenerationReadProtocol, and removes index/frontier from the old generic buffered inflation branch. This is a substantive unreviewed new parser, not covered by prior two-file diagnostic clearance. Inference corpus remains2928/8145906f2503220d4b1dd46808bfc276f8afa37b3d1499d0ea096ae53b8bd6de. Existing seven compressed cases include index/frontier but their actual extraction helper does not automatically establish new parser grammar, retained-reference completeness or bound causality.

Disposition: do not claim current schema/worker cleared under prior Source201 seal. Coordinator notified immediately. A coherent current parser contract/corpus/actual-function regression review is needed before final pins; no safe phase/title/count update can be inferred solely from this schema addition. This bounded observation does not request peer freeze or Native cancellation and introduces no new feature requirements.

No Source/control/ledger edits or test execution occurred.
## Bounded Parser Own-Merits Follow-Up

Per coordinator follow-up, reviewed current worker24686/b436d28c parser body and schema21219/e22765 against their actual current callers rather than treating size/newness as a defect. No concrete unsafe reference-loss or malformed grammar acceptance defect was established in this slice. The parser keeps only128 units of each token for shape/index checks while incrementally scanning the whole decoded token for closure references; fatal UTF8, JSON state transitions, closed root/class key sets, safe numeric class indices, index range bounds, maximum depth, token-per-byte and reference cardinality remain checked. Its20-unit suffix carry covers every incomplete prefix of the longest21-unit Source reference marker; a recognized marker retains its full incomplete digest until completion. Decoded JSON escapes are scanned, including keys.

Important scope: payload still Brotli-decompresses the whole document up to GENERATION_POLICY_V1.maximumIndexBytes=536870912 before passing65536-byte slices to the parser. This bounds inflated bytes but does not prove streaming decompression or small total memory. Global non-index/frontier cap is now268435456; generation owner cap512MiB accounts for the larger refusal metadata maxima. Both remain bounded by actual remaining proof bytes. Diagnostics retain previous first-primary and no-authority rules.

Concrete actual-test integration defect: current inference actual() extraction omits readGenerationClosureV1 and GENERATION_POLICY_V1 while current payload accesses both for index/frontier. Existing selected compressed7-role test must fail those rows with a ReferenceError unless corrected; its required ERR_BUFFER_TOO_LARGE/context checks correctly prevent false success. Coordinator and course_log reviewer notified. Course_log owns new generation test/corpus selection and phase admission review, so no duplicate broader claim here.

No final current-cohort clearance is asserted before that caller wiring and current test registration are reconciled. No Source effects, CLI or Native execution.