# Qualified Receipt Refusal Read-Only Audit

Inspected saved publication log, receipt88763, census88245 and actual owner Source. No protected live file/store or Native operation accessed.

## Observed Scope

Actual diagnostics log has nine stages: accepted,observed,witnessed,detached,underlying,staged,published,attached,verified. No settled stage. Saved final receipt successfalse/ownedCustodyClosedfalse. Parent's separately successful private ten-stage fixture remains separate evidence and does not settle this canonical asset.

Owner publishNativeImageReceiptCandidateV1 publishes candidate by rename, checks backing, syncs host base metadata, reads actual published image.json, checks bytes, then retains published.stamp and receipt. This is after-rename capture; a before-rename stamp hypothesis is contradicted by current Source.

finishNativeImagePublicationCandidateV1 first requires privately qualified token/current receipt/stamp, reads current receipt with complete bounded descriptor/name checks, then throws if either stamp differs OR bytes differ from JSON.stringify(state.receipt)+newline. Existing exact error reports neither predicate separately. No retained expected/current pair exists in the inspected saved log/receipt, so current cause cannot be narrowed to byte mutation, timestamp precision or another stamp field.

## Stamp And Precision

Owner stamp fields are dev,ino,mode,nlink,size,mtimeNs,ctimeNs, all from BigIntStats. atime is excluded. Reader compares lstatSync before/opened file.stat/after/name-after-close, so a persistent sync/async precision mismatch should normally fail an earlier reader predicate, not only finish. This does not rule out a delayed change; it prevents asserting such a cause without observations.

Saved census88245 shows diagnostics host receipt device16777232 and mtimeNs/ctimeNs1791994167767862100; mounted root device16777264 and mtimeNs/ctimeNs1792152612000000000. These illustrate host nanosecond versus mounted-root whole-second DATA. image.json remains host-side in the base directory during publish and finish. Root precision is not receipt precision. Historical private fixture success on another underlying filesystem does not independently explain canonical host failure.

## Minimal Diagnostic Evidence

Preserve exact existing refusal and all checks. Before throwing attach closed failure DATA (e.g. Error cause) computed from already held record/state, with version,stage=qualified-receipt,stampEqual,bytesEqual,expected/currentSHA256,expected/current byte lengths, and expected/current stamp fields dev,ino,mode,nlink,size,mtimeNs,ctimeNs as strings. Compute expected bytes once for comparison/diagnostics; no record content needs disclosure. Existing record.sha256 supplies current hash. Retain the expected published SHA alongside state.recordStamp if needed, or hash the already retained expected bytes on failure. Do not add extra protected reads, weakening tolerances or timestamp truncation.

For precise timeline, capture the same closed published stamp/hash/length at the existing postrename successful read, then failed finish from existing current read. This distinguishes inode replacement, chmod/link metadata, size/content change and time-only change without attributing causality. No full image/payload details needed.

Schema-first Source law: unchanged pair succeeds; each seven-field mutation independently refuses with exact field delta; same bytes/different stamp refuses; different same-length bytes/same stamp refuses; both differ refuse. Independent SQLite derives equality flags and SHA implementation may use Node crypto oracle already available. Extract actual finish comparison and preserve qualified-token admission and primary error behavior; diagnostic test must not fabricate Native success. Existing full owner/private proof remains necessary.

## Next Canonical Gate

Parent's proposed fresh census/planner/restart of only diagnostics follows existing canonical lifecycle. This audit releases no physical operation. Retained namespace/intent cannot be deleted or hand-settled after verified failure; current producer token/receipt/stamp and complete canonical settlement remain required. Diagnostic evidence should be available before assigning a timestamp fix or changing receipt admission.
