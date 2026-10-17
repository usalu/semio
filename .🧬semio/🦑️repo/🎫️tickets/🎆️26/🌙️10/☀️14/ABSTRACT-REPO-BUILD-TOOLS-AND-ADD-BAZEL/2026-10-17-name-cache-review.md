# Bounded Name Cache Read-Only Review

Reviewed the three exact nameCacheReplacements in ticket source-cost/📜️script.ts lines56–60 against current production publication/📚️witness/🟦️.ts. No Source edits, tests or Native operations executed.

## Result

No correctness defect found in the proposed narrow replacements under existing closed immutable directory entry assumptions. Parent should retain permanent full-witness/fault/control regression coverage; the pure selection projection does not execute directory guards, filesystem work, observer/control checks or complete-census acceptance.

- selectedBytes starts empty with selected; the same splice index inserts the associated optional bytes and the same pop evicts it. Missing first-entry bytes remain undefined until the original comparison position requires them. All materialized keys equal Node Buffer.from of their corresponding immutable selected string.
- entryBytes is created only after existing unsafe-name/dot guards. Cursor comparison still encodes incoming before cursor, while selected comparison still materializes left selected before right incoming when needed. Cursor bytes are scoped to each enumerate invocation, so they cannot authorize reuse across pages/censuses.
- Buffer.compare retains byte ordering, including replacement encoding for lone surrogates. Distinct strings with equal encoded bytes still stop at the same insertion index, retain original selected[at]===entry.name duplicate test and insertion tie order. Do not replace this exact-string test with key equality. Existing cursor <=0 behavior for byte ties is preserved, including its eventual incomplete-census refusal where applicable; optimization must not silently fix/change that behavior.
- The original duplicate error, count checks, nameWindow=64, read order, enumeration scans, total equality, exact checks and recursive visit order remain unchanged by all three replacements. No await/yield/progress/check/charge/observer call is added, deleted or moved.
- At callbacks and IO boundaries selected keys are at most64 names. At256 UTF16 units, UTF8 payload length is at most768 bytes/name, giving49152 selected payload bytes and conservative50688 bytes for66 selected/transient/cursor/incoming key slots. Array/object/Buffer allocator overhead is additional and must not be described as bounded by those payload figures. The original65th transient insertion is immediately trimmed with its key; no extra logical name is retained.
- Cached Buffers remain private and unmodified; return continues to expose original selected strings/count only. Global compare for unrelated journal ordering remains unchanged.

## Current Source Drift

A direct bounded file SHA read during review observed production W SHA256 `8fa2b07b0ebee40361e19aeee9a87e953aca9d9232be8d87bbdc3ee466a60c6d`, with no selectedBytes token yet. Historical plan hash c823a977d7644c46898e9789246e96db4a863608fb690a5d35d2ee795ffdcf5d therefore does not bind this current W. Parent was notified before patch. Existing unrelated EOF work must be preserved. Git diff attempted for read-only context failed because its daemon terminated; this review uses direct current-file observations and exact replacement text, not a Git freshness claim.

## Verification Needed

Use current-source baseline/RED/proposal and permanent actual-W regression corpus, including byte-equal distinct strings, exact duplicates, max768-byte key, eviction near64/65 boundaries, reverse-order input and repeated pages. Then run complete independent Python stream equality, existing directory/ancestry/I/O/fault/descriptor/Windows modeled laws and strict checks as applicable. A separately measured whole-W run is needed before elapsed gain claims; deterministic encoding-call reduction alone does not establish Tools deadline causality or authorize Native retry.
