# Exact Issued Patch Acknowledgement

## Contract

Each pending slot retains the exact issued ActorUiPatchReceipt, surface, and revision. The pending sequence follows the patch through the single turn handback slot. Issuance is provisional while output is prepared from a borrowed TurnResult. Only after successful preparation and the lifecycle clock verdict is the receipt committed and output published. An ordinary failure returns the typed patch to the same pending sequence and clears uncommitted issuance.

ACK and rejection first require the current exact Live guest lifetime, then the committed per-slot tuple. Parallel slots remain independent even for equal surface and revision. Duplicate, cross-surface, foreign sequence, stale guest, and uncommitted-output ACKs do not advance an owner. Exact external-patch ACK/rejection now marks its pending owner for bounded close; it previously had no reconcile ACK object and could remain mounted indefinitely. Close revokes issued receipts before slot reuse.

## Qualification

Neutral fixture/schema are under the pending module. The existing lifecycle command includes ten independent AJV/exact-comparison cases. Source13760 was RED on missing issuance implementation; source74113 is GREEN5 admission +10 patch cases. Native87787 is active with three new pending laws plus the existing handback and six lifecycle laws. Native behavior is not yet qualified.

The tests cover all fixture cases for both ACK and rejection, independent parallel pending slots with equal surface/revision, duplicate ACK, and output handback followed by a new exact receipt. The production reducer fault-injection law currently covers lifecycle ACK retention; a real pending patch through that failure path is still required.

## Boundaries Still Open

- Invalid/unmounted patch emission needs an audited bounded refusal path; returning it forever would obstruct other pending publications.
- Reconciler rejection while a producer is active needs a retained retry, not premature receipt consumption.
- WIT preparation currently clones effect values; remove avoidable copies with borrowed projection after correctness qualification. Silent null/numeric defaults remain unqualified.
- Effects/presence on failed output are not yet retained for retry. This report does not claim a fully durable outbound event transaction.
- Live task resumes still need exact captured native keys and stale-owner retirement.
- Full Hub86949 remains BUILD ACTIVE; actual GIS child build65957 ended SIGKILL. Neither is end-to-end runtime evidence.
## Native Qualification Update

Run87787/fV55Lv compiled the production code but failed at a real strict-clock verdict. The test driver's unconditional expect caused teardown to abort with the deliberately retained owner. The reducer now gives the deadline a distinct fault code and marks same-event retry safe only for lifecycle-only/empty input with no command page. The deadline remains strictly eight milliseconds; arbitrary command/intent input is not labelled replay-safe.

Run76209/rPbGNt was BUILD RED on a test helper accessor, now fixed. Run71639/qaPpDb compiled and passed the three cell laws and the first real lifecycle law. Its real close generation1/lifetime1 reached Retired and exact final ACK. That law logged one initial-open deadline retry and three empty-turn deadline retries (5.74 seconds total); this is functional retry evidence, not responsiveness qualification. The next native law failed because the negative actor-size fixture used exactly the allowed 4096-byte cap; it now uses the actual cap plus one.

Run31019/JyGTcC ended BUILD RED on the cap constant's private test-module scope, corrected to the actual parent module. Run70505/R35BWr is the current active expanded13-law retry. In addition to the neutral pending laws, it includes actual borrowed-output failure and real-clock delay with a second lifecycle actor, plus a real reconcile owner superseded by a second render generation. Native13 GREEN is not yet claimed.

Reconcile rejection now carries the exact retained publication generation. An older valid receipt can retire only its own pending owner without resetting the newer tracker. Current rejection that is temporarily capacity-blocked remains an explicit retained intent, driven one slot per turn. Foreign/duplicate feedback remains intentionally inert. New/future or absent-unwitnessed tracker generation remains refused. Exact terminal-witness classification remains an audit follow-up; no guessed surface identity was added.

Scoped Nx formatting27988 and diff-check are GREEN. The post-format source oracle14493 is GREEN5 admission +10 patch cases; earlier formatting67729 was GREEN. Full Hub86949 remains active; real GIS factory assembly is still compiling on the DB lane.
