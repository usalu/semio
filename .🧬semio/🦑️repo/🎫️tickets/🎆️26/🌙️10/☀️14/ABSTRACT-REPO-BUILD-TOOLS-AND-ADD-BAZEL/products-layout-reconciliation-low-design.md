# Products Layout Reconciliation Design Review

Design-only review from Root’s reported actual SERVICES reservation-policy refusal. No protected storage observation, runtime, journal or receipt mutation performed. Exact physical mismatch fields remain unknown; no generation age, actor or historical receipt attribution is established.

A contained canonical reconciliation operation before reader charging is preferable to weakening reservation equality. Observe under SH, close it, obtain actual whole-image EX, then reobserve authenticated layout, allocation journal, receipt identity and current physical headroom. EX must fence every relevant opener/consumer; a private flag or inferred quiescence is insufficient. Maintenance must not require the stale equality before reaching repair.

Capacity arithmetic must derive state and artifact budgets from disjoint physical byte allocations, including filesystem metadata, unchanged floor and allocation-unit rounding exactly once. A state reservation and artifact remainder must not double-count one physical pool. Before shrinking artifact reservation, prove current committed artifact consumption and retained owners fit the new amount; if not, refuse without effects. No overall capacity increase, data deletion or policy bypass follows from the mismatch alone.

Schema-first journal operation should bind exact allocation id/owner/kind, old amount and current layout identity/epoch to the new derived amount. One authoritative committed event updates the reservation without retire/re-reserve vacancy; same-id replacement without an old-value precondition risks ABA. Derived receipts must be reconstructible from that committed event after publication interruption. Do not claim atomicity across independent databases/files unless the actual journal is the single authority and recovery proves all derived outputs.

Required portable fault laws include mismatch/current/no-op, active readers busy, cancelled before commit, insufficient headroom/consumption, wrong owner/id/old amount/layout, and crashes before event commit versus after commit before receipt publication. Preserve arbitrary primary across cleanup and actual Original/phase checks around awaits. Existing format/authority remains canonical; no legacy fallback, migration script or permissive admission path is justified.

Direction is conditionally sound; concrete eligibility requires protected current facts and actual Source/event schema review. No runtime acceptance claim.

## Protected Actual Diagnostic Review

Read-only review of G/cache-charge-protected-77268.json confirms reported closed imageSH diagnostic: receipt and authenticated journal sequence2 identify sole committed artifact-publication allocation bbb43f06491d92400b1c8dc3f941d125, owner repository:products-store,25,768,755,200 bytes. Current policy maximumArtifactBytes24,692,916,224 differs by1,075,838,976; no persistent-state allocation appears in this observed journal. Receipt SHA db818a2e6f61db8bb8f2f927af6b5a5dd5b257c662bbcdec9c0364d30d438f26. This confirms exact amount mismatch, not material consumption, free space or eligibility to shrink.

Current layout helper derives artifact budget as filesystemBytes minus policy.chargeBytes, and store reservation checks receipt/allocation against that exact derived amount. Equality refusal is correct. The absence of a state allocation does not prove absence of state files, orphaned preparation, binding/intent or other retained material; EX reconciliation must authenticate those facts separately. Native observation/allocation bytes are quota commitments, not an actual artifact byte census.

Before event eligibility, prove current used artifact data and required metadata fit the smaller budget and state charge fits the same physical image without overlap; preserve non-recomputable namespaces. Exact oldamount/sequence/layout identity must be reobserved under canonical EX and bound to event precondition. No current diagnostic authorizes data removal, raw receipt rewrite or reserve recreation.

## Concrete Proposal Review

cache-charge-atomic-layout-proposal.md addresses the core conditions: authenticated image maintenance owns EX, immutable detached expected preimage binds whole ledger/backing/receipt, shrink-only same identity/phase transition, bounded allocation marker survives checkpoint, conservative BigInt whole-filesystem used/available checks, and state orphan/intent refusal. Journal is sole authority; derived receipt publication/recovery does not claim cross-file atomicity. Direction is conditionally clean for schema-first Source work, with physical effects still held.

Required recovery law: append an unrelated valid allocation event after adjustment, checkpoint, then recover an old receipt. Marker epoch describes the historical transition; it must not be equated to current whole-ledger epoch after later valid events, nor cause reapplication. New Native API/types must remain repository-neutral and authenticate exact contained owned receipt target. Current openimage maintenance internally releases own SH before EX, then closes/reopens a fresh reader; no operation on a previously closed image or reader-upgrade is permitted.

## Revised Current-State Contract Supersedes Marker Design

Root’s revised approved direction removes marker/history fields. Current checkpoint State and its unique committed allocation matching current policy are authoritative. Under explicit authentic EX only, a missing receipt or closed stale receipt with SAME id/owner is a deterministic derived projection; malformed/unknown/ambiguous data refuses. Ordinary SH never repairs. Event CAS binds exact sequence and entire old allocation row, then shrink-only amount; unrelated events/checkpoint do not destroy recovery authority. This supersedes earlier marker recommendations rather than adding a second compatibility branch.

Keep Native API repository-neutral: repository owner validates its current disjoint declaration; Native must not import repository-specific types/declarations merely to recompute request layoutSha. Authenticate domain-neutral Native policy/layout facts at their owning layer. Later unrelated event plus checkpoint plus receipt repair remains required. Proposal leftover stale epoch terminology should mean sequence/current layout, not a retained historical marker.

## Early Concrete Source Review

Common Opening now runs Products layout preparation before constructing the partition wrapper, passing the actual registered Native image; ordinary store, maintenance and DevelopmentState entries therefore share it without a Store↔Opening dependency cycle. Strict Native WeakMap admission remains intact. Generic rebudget snapshots authority references/request, explicitly checks actual kernel proof and exact old allocation tuple/sequence, and uses the current smaller policy.

Two concrete integration checks sent executor: EX callback should carry custody.control as its actual writer phase instead of outer control, validating incoming refinement; and receipt directory durability currently uses raw directory fsync, which needs authenticated Windows metadata integration or explicit refusal rather than a POSIX-derived all-platform success claim. Final fault/entry model and held Source reviews remain pending.

## Updated Layout/Journal Reread

EX work now carries custody.control with incoming refinement. Derived receipt uses mandatory port.syncMetadata, replacing raw directory fsync. Generic journal marks dirty/cache invalid before potential append/checkpoint mutation, awaits actual barrier plus backing/control postchecks before cache/return/unlock, and preserves bodyprimary through barrier/release errors. Common Opening still supplies actual registered image before wrapper and covers all three entry paths. Narrow read is coherent; final fault models/strict/physical eligibility remain separately pending.

## Final Production And Private Law Narrow Verdict

Held production review is clean: authenticated kernel proof, exact allocation CAS/current-State receipt derivation, common Opening before wrapper across entries, actual custody phase and mandatory barrier. Executor reports final Source6/282/996ms and seven-root strict89770 actual0/5.825s; this audit does not rerun them.

Prepared exact retirement-identity/📜️script.ts --rebudget issues genuine120 and forwards same wire to permanent law requested50/runner60. Diagnostics host funds only private64/65MiB image. Two SH readers protect against third nonwaiting EX using busy DATA/no callback; then fresh quiescent EX shrinks60→32 sameid, checkpoints/later row recovery, Node payload SHA and exact known allocation/image cleanup. Ancestor removal is blocked on unresolved handles. Narrow review is clean for this exact released argv. Existing launcher includes-mode parsing is not a closed general mode API.

Physical law scope is generic Native CAS, sharing, payload and current-row recovery. It does not physically qualify repository-specific receipt repair, state orphan/headroom eligibility, Windows metadata or power-loss durability. No execution or physical release was performed by this audit.

## Bounded Receipt And Explicit Open Mode Review

Production bounded reader now reads only initial FD size<=2048 through fixed-size readSync loop, refuses short reads and one-byte tail growth, verifies opened/named pre/post stamps, and preserves primary across close failure. New exact --open ticket mode uses genuine60/current bootstrap factory, refuses18 inherited overrides, calls normal Products store with current policy and verifies expected sameid/owner/amount/current journal/bounded receipt after admission and full close. No direct CAS/receipt rewrite is used by this launcher. Narrow review is clean.

One retained helper issue reported: --observe is still executable and labels itself no-rebudget despite current common opener potentially reconciling. Historical receipts remain valid historical evidence; current executable mode must be disabled or honestly relabeled before claiming read-only behavior. No audit execution was performed.

Postedit guard reread confirms historical --observe branch removed; only default Source model or exact --open is admitted. Executor retained actual --observe pre-factory refusal. Prior historical observations remain preserved. Combined strict is pending a separate fixed cleanup unresolved symbol; no pass reclassification.

Final exact --open readiness verdict is clean after historical mode removal. Executor reports Source7/351 actual0 and final combined strict20123 actual0/8.918s; separate fixed helper was resolved in current Source without executor attribution. This is readiness for Root’s explicit canonical release, not execution authorization or canonical success.
