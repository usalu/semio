# Store Decoder Concrete Retained-Owner Repair Design

## Status and Scope

Read-only source design, 2026-08-27. This follows [retained-field review 48](./📓️store-decoder-retained-field-review-48.md). No production source, test source, schemas, controller, launch configuration, Cargo target, or registry state was changed or executed. No native RED/GREEN claim is made. Root owns the separately prepared grammar RED and compiler scheduling.

The requested SPR edit findings are both confirmed in source: final assembly removes earlier required owners before later missing-field errors; finishing one nested mutation retirement can report the entire edit Complete while other mutations and strings remain.

Additional directly observed defects matter to the same repair: Fresh VCS close can drop a non-Snapshot active variant on a failed pattern match; published reservations are subsequently cancelled as if still live; mutation/history reservation errors discard their returned tokens; callback unwind poisons the field registry and makes retained owners inaccessible through its current methods.

Keep the existing concrete authorities, catalogs, targets, field registry, completed registry, and retirement cursors. Do not introduce a parallel decode/transaction API, generic serde fallback, cold drain, global queue, ownership-forgetting guard, or second completion certificate. These are field-decoder repairs, not Flow mutation retirement, diff application, channel ingress, Interaction, or lifecycle changes.

## Reviewed Sources and Fingerprints

All line references below refer to the observed source revision; symbol names are authoritative when unrelated work shifts lines.

| Input | Observed SHA-256 |
| --- | --- |
| Store `🦀️component.rs` | `d9c8ce77be44113b217687d5bba4f3da6c55b7feb0d99bbe3fa2c002fe269beb` |
| OS VCS `🦀️component.rs` | `94c13ef40c7d13a505fbf66af5f4704ad3f894531c17534fa5ced8d7c94dc1b2` |
| Review 48 | `c7cc9c60e2c3ba01c1c8c5cc462fa4065cd39eaf1b40895cf79266e0e9f70dd6` |
| Existing owned-schema-record native tests | `8e8ccaffa0c23179b33ea55d87ac19df0b5c631362318b0d41a2b6326962cda9` |
| Existing owned-schema-record neutral vectors | `534b6c794937c18dabff738c788a48d0ff25d73b843bed15942336fcae9c0d83` |
| Existing owned-schema-record vector schema | `83337d3c9d9353ae92fb712eeea303877418e1440dc1ea0b78139d269ff133ef` |

[Store source](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs), [VCS source](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs), and [existing grammar tests](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️owned-schema-record/🦀️.rs) were read directly. Existing grammar tests execute record cursors, not FreshFieldDecoder/FreshVcs/SPR edit ownership; they must not be counted as proof of these repairs. Existing Store envelope tests around 20504–20805 use `TestEnvelopeFieldDecoder`, which only counts accepted tokens and flips a terminal flag; retain those tests, but do not reuse that stand-in as the ownership subject.

At release, the shared Store whole-file hash had changed to `0ed0d7a78c833c1081825c598de3a5dde36ecc858a2e1448c5695899358efd0d`; the inspected decoder symbols shifted by 37 lines. The original hash is retained above, not overwritten. A fresh read still finds the SPR sequential takes/early Complete and both Fresh active takes. The release decoder cohort, current lines 5743–8953 inclusive (envelope schema constants through FreshFieldDecoder Drop), hashes to `2f9cef0c87a17220ef89593ccda9dd0798b916e28f78ca98f3718df1a186f59b`. This is a release fingerprint, not a claim of an initial-to-final cohort hash comparison; no initial full cohort copy was captured. OS VCS remained at the same observed hash. This task made no Store source writes.

## Existing Contracts Worth Preserving

- `OwnedSchemaRecordCursor` finishes only after its outer trailing state sees EOF. `ArtifactEnvelopeDecodeAuthority::step` calls field `finish_record` only after this and checks the field decoder's terminal witness.
- The persistent job retains its page cursor and exact field-registry lease separately. Release first closes fields, returns the lease, waits for exact registry detachment, then closes source pages. Registry detachment is a handoff, not completion of the detached owner.
- Fresh VCS stages a completed snapshot in `ArtifactEnvelopeFreshSnapshotTarget`; FreshFieldDecoder stages completed VCS in `ArtifactEnvelopeFreshRecordTarget`. Neither is an application-state publication.
- `OwnedSchemaBoundedArrayAuthority` owns its live entry authority, ledger reservation, rejected entry, ledger, and active retirement. Its entry callback is already borrowed in place. Its output-take guard is stronger than Fresh VCS's unchecked assignment from `take_values()`.
- `ArtifactOwnedSprMutationArrayAuthority` also borrows mutation accept/publish callbacks in place. This is a useful local pattern, not a blanket correctness certificate.
- `OwnedSchemaStringAuthority<N>` stores partial bytes inline, with a hard N-byte limit. Fault/Cancelled states are terminal-empty; `cancel()` marks the shallow scalar taken without allocation. A String is allocated only by `take_string()`. Do not describe an errored inline scalar as a leaked deep collection.
- Real cleanup uses `ArtifactStoreVcsRetirement`, `ArtifactStoreDecodedEditRetirement`, `ArtifactStoreHistoryMetadataRetirement`, and the actual supplied snapshot/mutation factories. Keep their one-owner-at-a-time handoff; no replay or whole-value Drop substitute.

## Concrete Failure Inventory

| Source boundary | Observed behavior | Required repair |
| --- | --- | --- |
| FreshFieldDecoder accept, 8737–8801 | `self.active.take()` precedes String extraction, VCS accept/publish `?`, and Empty-array acceptance. VCS Err or RecordComplete drops the local boxed authority. Publish success drops it without an empty witness. | Borrow `self.active.as_mut()`; preserve it through every fallible callback and unexpected step; clear only a validated terminal-empty child. String destination/extraction and Empty terminal checks stay explicit. |
| Fresh VCS accept, 8416–8508 | `self.active.take()` surrounds Snapshot and all four history arms. History `?` exits lose the concrete ledger/entry owner; snapshot errors lose phase/reservation even though its boxed decoder is a separate field. History completion stores `None` if `take_values()` fails. | Borrow the exact variant in place. Validate destination vacancy, returned output presence, and terminal child state. Reject unexpected RecordComplete explicitly. Retain a transferred output in its destination before any later failure; never keep it solely in a fallible local. |
| Fresh VCS snapshot publication, 8420–8448 | FieldComplete/TokenComplete is accepted without checking snapshot terminal state, target output presence, or consumed reservation. | Validate all three. A normal published snapshot leaves value in target, no live target reservation, and terminal-empty snapshot authority. A false terminal remains owned and faults. |
| Fresh VCS final assembly, 8512–8519 | Snapshot, edits, changes, checkpoints, alternatives are sequentially taken with later `?` checks. | Preflight every required slot and output vacancy before any take; perform only infallible struct assembly after that. |
| FreshFieldDecoder final assembly, 8803–8838 | Schema, id, VCS sequential takes; no explicit active/reservation/output consistency preflight. `terminal=true` alone returns RecordComplete. | Check all required slots and phase consistency first; require full own-empty witness when reporting terminal completion. |
| SPR edit final assembly, 6231–6240 | Id, forwards, inverse, startedAt are moved before the later sequenceNumber check. Missing inverse/startedAt/sequenceNumber drops earlier mutations on the local stack. | Preflight all five required values in existing diagnostic order, plus empty output slot and no active child. Preserve all earlier owners on Err; optional strings move only in the final infallible assembly. |
| Fresh VCS close, 8554–8557 | `if let Some(Snapshot {..}) = self.active.take()` removes Edits/Changes/Checkpoints/Alternatives too; pattern mismatch drops that live history owner. | Inspect the variant by reference first. Retire only the snapshot owner in this step; leave non-Snapshot active history untouched for `close_pending_history`. |
| FreshFieldDecoder/Fresh VCS/SprMutationArray cancellation | After a publisher transferred its value and then returned Err/panicked/false terminal, its target reservation is already consumed, but the caller still has a phase token. Close unconditionally cancels that stale token. Several sites remove it before the fallible cancel. | Distinguish still-reserved from already-published using the existing target's real fields. Cancel only the matching live reservation, retain the token until successful cancellation, and retain/retire an already-published value. A genuine mismatch is an error with authority still retained. |
| FreshFieldDecoder begin VCS, 8708–8711 | Reservation is installed before `catalog.begin_vcs`. A construction unwind leaves target.reserved but no active variant. | Close must drain the target's exact live reservation even with active=None; do not infer absence of a reservation from absence of active. This uses the existing target fields, not a second reservation API. |
| SPR mutation publication, 6011–6035 | Clears its phase reservation; takes target value before checking `self.values`; missing destination loses the value. Capacity refusal restores value, but later cleanup must still reconcile publication state. | Preflight destination availability/capacity before taking the target value. Keep publication reservation state coherent across Err, false completion, and unwind. Do not replace a retained target/output or retirement slot. |
| History ledger handoff/cancellation, 7210–7231 and 7306–7310 | Failed `insert_reserved` preserves value in rejected but discards returned reservation. Cancel takes reservation then uses a missing-ledger check and discards cancellation's returned token on Err. | Preflight ledger presence before consuming the token. Restore both components of `Err((reservation,value))`; restore the exact token from cancel Err. Use current OS VCS return-owned APIs, not a new API. |
| SPR edit close, 6356–6368 | A completed retirement is dropped, then self.terminal=true and Complete returned, although it may be only one popped mutation and other vectors/strings remain. | Child completion returns bounded Pending. Continue remaining vectors, strings, and output. Only the final own-empty check sets terminal and returns Complete. |
| SPR edit active close, 6341–6353 | Drops active after Complete without a per-variant empty witness. | Mutation child requires its own terminal-empty witness. String cancellation is an inline proof; EmptyMetadata has no deep owner and needs an explicit shallow-cancellation treatment, not a fabricated parsed-success witness. |
| Final close guards, Fresh VCS 8568–8573; FreshFieldDecoder 8896–8905 | Sets terminal/returns Complete without checking all own fields, including reservations. FreshFieldDecoder removes schema and id together after only a byte-total check. | Whole-owner emptiness must be the only Complete condition. Release at most one owned String per item grant; report bytes/items in Pending before a later Complete. An orphan live reservation cannot be hidden by terminal=true. |

The four Fresh VCS history variants all require coverage, including histories that appear before initialSnapshot. Field order is not fixed by the schema.

### Why the SPR edit findings are real but differently reachable

The final assembly gap may be masked for ordinary well-formed input by the required-field cursor. That does not prove typed-owner safety under a false/missing-output child or an explicitly exercised private assembly boundary. Native tests must distinguish grammar rejection from assembly rejection, and execute the actual private assembly method/branch without replacing it.

The premature Complete defect is reachable after a genuine partial edit parse. For example, decode id, one or more forwards and inverse mutations, then encounter invalid sequenceNumber. Close first hands one mutation to its factory. Once that child reports Complete+empty, current code reports the whole edit Complete although its vector shells, remaining mutations and id String still exist. This does not require a malformed private state.

## Repair Shape: Existing Owners, No New Public API

### Retain active callbacks in place

Use a borrowed match on each established active slot. A small private result/action enum or boolean may distinguish progress from validated child completion; it must not own the child. Do not use a take/restore guard as a substitute: ordinary `?` restoration is easy to miss, and unwinding skips manual restoration.

Accept-phase terminality is not the same as publish-phase terminality. An accept FieldComplete may still own the value pending publication. Before switching to publishing, verify the enclosing token really is the field terminal. A nonterminal TokenComplete cannot consume the actual terminal token without rejection. Pending must leave token, owner and phase available for re-offer. Treat RecordComplete from a nested field/entry/publisher as a typed retained error, never as record success.

On publish completion, require target has the exact output, its reservation was consumed, and publisher is terminal-empty. If it lies, retain both the active authority and any already-transferred target value for close. A second attempt must not republish or overwrite the first value. After a post-commit Err/panic, the next action is close, not a repeated commit.

For Strings, preflight field id and destination vacancy before `take_string()`; require Some output. For histories, preflight destination vacancy, obtain Some output, establish it in the actual field slot, and then check the now-empty child's terminal witness. No arbitrary-object or skipped-missing-output acceptance.

### Preflight complete records, then move

Use ordinary borrowed presence/phase checks first, preserving current missing-field diagnostics. Only after every check succeeds may the actual owners be taken into an immediate struct assignment. No `?`, user callbacks, unknown-field checks, or later required-field checks should follow the first take. Prepare any fallible capacity admission before moving owners. Fresh envelope assembly also creates an empty `ArtifactEditMessageLedger`, whose constructor allocates fixed-capacity tables; prepare that empty shell before the move when enforcing allocation admission. Do not conflate allocation-abort behavior with a recoverable Result/unwind path, or invent a new allocation guarantee for the current Box constructors.

Exact required slots:

- Envelope: schema, id, target.vcs; active=None; target.reserved=None; no competing pending completed/retirement owner.
- VCS: target snapshot, edits, changes, checkpoints, alternatives; active=None; snapshot reservation absent; output vacant; snapshot authority terminal-empty before publication.
- SPR edit: id, forwards, inverse, startedAt, sequenceNumber; active=None; output vacant; no concurrent closing retirement.

Do not check optional actor/description/coalesceKey/finishedAt as required. Preserve their current omitted-or-null behavior. Empty forwards/inverse arrays are valid required values, distinct from absent arrays.

Assembly tests must separately prove each later missing slot leaves every earlier owner in the original slots with unchanged identities. Grammar tests alone cannot execute this law.

### Bounded close and exact reservation state

Keep one child retirement active at a time. Every delegated Complete must be followed by that child's terminal-empty check before dropping its wrapper. Child completion returns Pending when any enclosing owner/slot/reservation remains. Blocked, Err, and unwind retain the same child for a later grant; no replacement, forgotten owner, or terminal-bit shortcut.

For copyable envelope/snapshot/mutation reservation tokens, inspect without taking; clear the phase token only after successful cancellation or validated prior publication. For non-Copy history reservations, use the existing return-owned error value to restore the slot after failure. No cancellation token should disappear because a ledger is missing or refuses the exact generation.

Once active callbacks are closed, cancel any target-held reservation left by constructor unwind; then retire any target value. A post-publication target with no reservation is not stale cancellation: it is an owned-value cleanup phase. A mismatched live reservation is a retained diagnostic, not permission to clear somebody else's token.

Use the existing real close grants: the envelope job normally gives one item and 4096 bytes; a history entry is capped at 4096 and a history ledger at 64 entries. Respect zero item grants without releasing owners. Byte-short grants may refuse progress for an individual String/entry; a subsequent sufficient grant must progress. Report actual per-step releases, not a count of future scheduled work. Do not promise eventual progress under a forever-insufficient byte grant.

FreshFieldDecoder should release schema and id one at a time rather than dropping both on a one-item grant. Inline partial String/EmptyArray cancellation is shallow and can be accounted separately; it is not a bounded deep-retirement implementation. Preserve explicit bytes bounds for completed heap Strings.

## Unwind and Publication Boundaries Requiring Explicit Coordination

### Field registry poisoning is an existing decoder blocker

`ArtifactEnvelopeFieldDecoderRegistry::with_owner` (6700–6712) runs `f(owner)` while holding its mutex. Callback panic leaves the exact boxed decoder in its slot but poisons the mutex. `with_owner`, `take_returned_ticket`, and `try_admit` currently treat Poisoned like WouldBlock. Consequently a shallow lease return can succeed atomically, while no bounded pump can detach the retained owner afterward.

Minimal existing-registry repair: distinguish WouldBlock from Poisoned at this registry's access points. Recovery may use the poisoned guard only after verifying the registry bookkeeping invariant is unaffected by the owner callback. The callback is passed the decoder, not the slots/free list; it cannot legitimately mutate registry bookkeeping. Do not apply this policy indiscriminately to other locks. Preserve exact ticket/generation checks and execute native unwind→lease-return→detach→close laws. This is an additional source boundary that root/runtime must explicitly include; no registry code was changed here.

The alternative of catching and rethrowing inside the mutex solely to avoid poisoning still needs the same owner-retention tests and adds panic machinery. Prefer the narrowly justified existing-registry poison recovery, not a new owner API or silently blocked forever behavior.

### Completed-ticket rejection needs a retained retry phase

FreshFieldDecoder removes pending_completed before `completed.try_admit`, but that concrete registry returns the exact owner on ordinary admission refusal; this transfer is already recoverable and is not an arbitrary callback. On successful admission followed by duplicate completion publication, `try_request_close(ticket)` may be contended. Current code returns Err and loses the newly admitted ticket locally. The completed registry still owns the record, but `try_next_close_ticket` enumerates only its closing mask; an unmarked new record has no guaranteed normal close-pump route.

Minimal repair: retain the admitted-but-unpublished ticket in a private phase/slot of the same FreshFieldDecoder until publication succeeds or its close request is acknowledged. Retry a contended close request on later grants, never construct/admit the envelope again. Clear this shallow obligation only at a real handoff. If the acceptance law requires actual nested completion before terminal failure, retain the ticket through its `ticket_reclaimed` witness and drive the existing detached completed-record owner to empty in the test; do not call a close-mask handoff “disposed.” A private phase is not a parallel public owner API.

### What the current by-value contracts cannot guarantee

`publish_vcs_reserved`, `publish_snapshot_reserved`, and `publish_mutation_reserved` consume the value and return `()`; `ArtifactOwnedValueRetirementFactory::retire_owned` consumes T and returns a boxed retirement. If an arbitrary implementation panics after taking T but before retaining it, its caller cannot recover T by restoring its own active enum. Catching that panic afterward does not recreate the owner.

The concrete Fresh targets only validate their exact reservation/vacant slot and then store the value, with no callback between validation and assignment. For valid callers, test both publisher failure before the commit while the authority still owns its value, and failure after the commit while the real target owns it. Do not inject a by-value target that discards T and then assert the caller can repair it. False/mismatched reservation must be rejected while the publishing authority still owns the value; do not exercise target assertions as if they were recoverable Result errors.

Similarly, the concrete Store retirement constructors wrap the exact owner in retained state; domain factories must meet that handoff contract. Panicking arbitrary consuming factory/target implementations are an explicit unresolved contract limitation, not covered by borrowed-active repair. If root requires recovery from those implementations too, the shared existing consuming contract needs a separate owner decision. This design does not authorize or propose a second API.

## Schema-First Neutral Fixture Proposal

Proposed domain location, not created in this read-only task:

`🏪️store/🧪️tests/🧬️owned-field-retirement/`

Use canonical `🔣️vectors.json`, `🧬️schema/🔣️.json`, and `🦀️.rs`. A later ticket controller is only `📜️script.ts`. Leave root's separate `🧬️owned-schema-record` grammar fixture unchanged.

The fixture schema should separate:

1. Raw source/pages and expected syntax/schema diagnostic or successful normalized record.
2. Scripted leaf-catalog fault schedule: boundary (begin/accept/publish/close), before/after actual target commit, event (Pending/Err/panic/RecordComplete/falseComplete/Blocked), one-shot occurrence, and owner IDs.
3. Close grants and exact expected conservation checkpoints: owner location, live IDs, released IDs, reservation presence, completed-ticket/field-ticket handoff, and terminal expectations.
4. Private assembly-slot cases, explicitly labeled non-wire white-box cases; do not claim Ajv rejects an internally missing output slot.

A minimal owned test-domain snapshot can be `{"owned":["snapshot-a","snapshot-b"]}`; its mutation can be `{"owned":["forward-a"]}`. These are explicitly proposed probe-domain payloads, not Flow/SPR mutation wire schemas. Give them strict, authored schemas and actual leaf authority implementations through the existing snapshot/mutation catalog interfaces. They need no Mutation trait or fake semantic descriptor: these Store decoder generics require Send and exercise owned decoding, not mutation dispatch. Their counted owner tokens must be genuinely retained in the authority/typed value and released one at a time by the supplied retirement, not represented only by a Boolean “owner present.”

Use this handcrafted baseline with real SPR edit and VCS field names:

```json
{"schema":"s","id":"i","vcs":{"initialSnapshot":{"owned":["snapshot-a","snapshot-b"]},"edits":[{"id":"e","forwards":[{"owned":["forward-a"]},{"owned":["forward-b"]}],"inverse":[{"owned":["inverse-a"]}],"sequenceNumber":0,"startedAt":"t"}],"changes":[{"id":"c","editIds":["e"],"savedAt":"t"}],"checkpoints":[{"id":"k","changeIds":["c"],"authors":[],"timestamp":"t"}],"alternatives":[{"id":"a","name":"A","checkpointIds":["k"]}]},"editMessages":[],"conflicts":[]}
```

The three metadata entries above use actual OS VCS serde fields. Omitted Option fields are intentional. Metadata types currently use serde without deny_unknown_fields; do not claim strict nested metadata rejection beyond their actual contract. For a malformed later metadata entry use a missing required field or wrong type, not an invented strictness guarantee.

Handcrafted case roster to author before repairs:

| Case | Exact variation/schedule | Expected retained boundary |
| --- | --- | --- |
| C01 | Baseline, sufficient bounded grants | Exact successful record; every probe ID transferred once; completed owner later explicitly closed. |
| C02 | Baseline with whitespace-only suffix, split pages inside a String escape | Success only after EOF and field/page close, not at VCS completion. |
| C03 | Baseline followed by ` true` | Outer trailing-token rejection after VCS is complete; all target-owned IDs retired. |
| C04 | Remove final `conflicts` field | Outer missing-required rejection; no completed-record admission. |
| C05 | Insert `"unknown":0` after vcs | Outer unknown field after all prior deep owners are complete. |
| C06 | Change final `conflicts` to `[0]` | Concrete EmptyArray rejection while completed VCS remains owned. |
| C07 | Change sequenceNumber in the edit to `"bad"` | Partial SPR edit with forward-a, forward-b, inverse-a and strings retained; first child retirement must not finish the edit. |
| C08 | Change sequenceNumber to `2147483648`, and separately `0.5` | Actual i32 token conversion rejects; same retained-prefix cleanup. |
| C09 | Append a second malformed Change `{"id":"broken","editIds":[]}` | First history entry already in real ledger, second entry authority faults on missing savedAt. |
| C10 | Analogous later missing timestamp Checkpoint / missing name Alternative | Both other concrete metadata-history arms retain prior entries and active authority. |
| C11 | Start a second edit, then make its second forwards payload invalid | Real bounded ledger + real SPR edit + real mutation array + active domain mutation authority all coexist during failure. |
| C12 | Put edits, changes, checkpoints, alternatives before initialSnapshot; cancel midway through each | Snapshot close must not remove/drop the non-Snapshot active variant. |
| C13 | Snapshot and mutation accept: one-shot Err, panic, RecordComplete, premature FieldComplete, false final TokenComplete | Existing concrete parent retains phase/owner; source tokens never advance incorrectly. |
| C14 | Snapshot, mutation and VCS publishers: one-shot Err/panic before and after actual commit | Exact owner remains either authority-owned or target-owned, never both/neither; no stale cancellation after commit. |
| C15 | Publisher returns terminal step with no output, or output plus live child | Reject without dropping live authority or transferred output; no final assembly of missing fields. |
| C16 | begin_vcs and begin_mutation panic after parent reserves | Existing reservation remains visible and is cancelled by bounded close; no fabricated leaf owner. |
| C17 | Close child returns Blocked, Err, false Complete, then valid Pending/Complete | Parent retains child, does not overwrite retirement, and does not report whole Complete early. |
| C18 | Cancel at every established owner/publication boundary | Real registered job closes and returns exact field lease; detached owner and pages also reach empty. |
| C19 | Zero items; one item with zero/short bytes; one item with 4096 bytes | No deep release on zero grant, no over-budget release, refusal followed by progress. |
| C20 | Pre-seeded completion cell and one-shot completed-registry contention | New rejected ticket remains tracked through retry and actual completed-owner cleanup. |

C13–C17 are schedules applied to actual leaf callback seams; they are not alternative parser models. A VCS fault wrapper may delegate all valid work into actual FreshVcs and inject before/after delegation solely to exercise FreshFieldDecoder's seam. It must retain the wrapped FreshVcs; it cannot replace it with the existing Boolean-only test decoder.

Third-party checks should use Ajv2020 for the actual domain vector/payload schemas and a raw JSON parser such as jsonc-parser for source syntax/duplicate/trailing expectations where appropriate. Compare successful native serialization with the expected neutral record. Neither JSON Schema nor JavaScript arithmetic proves ownership or cleanup; only native runs of the real authorities and real owner counters establish those. Preserve diagnostics separately from the third-party acceptance result when schema and callback rejection differ.

## Concrete Native Law Roster

All names below are proposed tests, not executed results.

| Native test | Required subject and assertions |
| --- | --- |
| `fresh_field_string_faults_preserve_prior_completed_owners` | Real FreshFieldDecoder + OwnedSchemaStringAuthority: semantic-byte overflow, cancellation, stale generation, invalid escape. Earlier VCS/String fields survive until bounded close; inline scalar safety is not misreported as deep retirement. |
| `fresh_field_vcs_callbacks_retain_on_error_and_unwind` | Real FreshFieldDecoder and delegating FreshVcs wrapper; accept/publish pre/post-commit Err and panic; retain exact boxed child identity and target IDs. |
| `fresh_vcs_all_active_history_variants_survive_failure` | Actual FreshVcs + all four OwnedSchemaBoundedArray variants + real entry decoders; completed prefix then malformed entry; no active take-before-error. |
| `fresh_vcs_close_preserves_non_snapshot_active_variant` | Histories before initialSnapshot and histories after published snapshot; zero grant then close snapshot; assert history variant/ledger/entry identity unchanged until its own close phase. |
| `nested_publishers_reject_false_terminal_and_record_complete` | Actual FreshVcs, SPR mutation array and their real targets; Pending/TokenComplete/FieldComplete/RecordComplete discipline, missing output, output plus nonempty authority. |
| `spr_mutation_publication_errors_preserve_value_and_reservation` | Real mutation array: destination preflight, capacity refusal, before/after commit fault; exact target value and phase token maintained; no cancellation of consumed reservation. |
| `history_entry_handoffs_restore_returned_reservations` | Real bounded array and actual ArtifactHistoryLedger: failed insert/cancel retains both returned token and value; missing-ledger preflight cannot consume the token. Distinguish deliberately invalid state retention from eventual cleanup of coherent state. |
| `required_record_slots_are_preflighted_before_any_owner_take` | Actual SPR finish_record, Fresh VCS completion branch, FreshFieldDecoder finish_record; each required-slot omission preserves every earlier owner. White-box fixture cases labeled explicitly; output occupancy never replaces an owner. |
| `spr_edit_child_completion_is_not_record_completion` | Actual partial edit from C07 with two forwards/one inverse and Strings. After each child retirement, remaining slots still live => Pending, not Complete. Every token retired exactly once. |
| `active_and_retirement_false_completion_keeps_exact_child` | Real close methods; false Complete must retain same child and reject; later valid close succeeds. Include SPR active mutation-array terminal witness. |
| `constructor_unwind_keeps_existing_target_reservation_cancellable` | Actual begin_vcs/begin_mutation call sites with controlled catalog panic; no active authority required to find/cancel the target reservation. |
| `registered_decoder_unwind_is_recoverable_without_poison_livelock` | Real registry + real job + retained concrete decoder; catch panic around borrowed job.step, begin_close, return exact lease, detach exact generation, close detached owner, then pages. No poisoned-lock Contended loop. |
| `completed_publication_rejection_retains_retry_ticket` | Actual completed registry/completion and FreshFieldDecoder; duplicate publication plus contention/retry; no unmarked/orphan admitted record, no double admission. Detach and close actual completed owner. |
| `late_envelope_failure_retires_completed_vcs_before_terminal_fault` | Real pages→record→job→FreshFieldDecoder→FreshVcs chain for C03–C06; no result publication on failure; completed target and source pages conserved through close. |
| `all_field_retirements_obey_zero_refusal_and_positive_grants` | Run every failure schedule with exact grants; counters check each step and final empty state, including two completed schema/id Strings under a one-item grant. |
| `successful_owned_decode_preserves_exact_edit_and_history_order` | Baseline and reordered fields; actual decoded Edit vectors/ledger order/nullable Option values; completed record taken or retired explicitly, not left live on test exit. |

The native harness must build actual `OwnedSchemaDecodePages`, `OwnedSchemaRecordCursor`, `ArtifactEnvelopeDecodeAuthority`, `ArtifactEnvelopeFreshFieldDecoder`, `ArtifactEnvelopeFreshVcsAuthority`, `artifact_owned_spr_edit_decoder`, mutation arrays, ledgers, field/completed registries, and real Store retirement cursors. Only the domain leaf catalog/factory and fault schedule are controlled test implementations.

Use `catch_unwind(AssertUnwindSafe(...))` around a borrowed retained subject, not around a closure that owns and drops it on failure. Assert counters after subsequently driving the existing close protocol. Keep IDs and source/lease identities in every assertion. A Drop counter should report premature disposal without itself panicking, but current concrete parent Drop assertions can still abort a defective RED path through double panic. Run those initial RED cases in isolated native test processes with retained output/exit status; an abort is a failing RED observation, never a passing test or permission to forget the owner. Do not weaken Drop assertions or use mem::forget to make the suite exit.

## Minimal Source Packet and Coordination

After root's grammar RED/source freeze, reserve these exact Store regions for one repair owner:

1. FreshFieldDecoder/FreshRecordTarget, 8583–8916: borrowed active logic, completion preflight, reservation-aware close, private admitted-ticket phase and truthful terminal.
2. FreshVcs/FreshSnapshotTarget, 8196–8581: borrowed Snapshot/history arms, output validation, final preflight, non-Snapshot close preservation and orphan-reservation handling.
3. SPR mutation target/array and edit authority, 5897–6408: publication destination/reservation preflight, typed-step validation, assembly preflight, and enclosing close progression.
4. Bounded history array, 7189–7337: exact return-owned reservation restoration and output/terminal discipline. Keep current concrete metadata decoders; do not replace domain edits with serde.
5. Field registry, specifically try_admit/with_owner/take_returned_ticket around 6664–6740: narrowly justified poison recovery needed for actual unwind laws. Obtain explicit root/runtime inclusion if “decoder/catalog regions” was not intended to cover this registry.
6. New domain test/fixture files plus one cfg(test) mount adjacent to existing Store test mounts. Existing grammar fixture/controller remains frozen and disjoint.

The completed registry's current admission/close-request/detach APIs suffice for the private retry-ticket phase; no new public registry API is proposed. OS VCS ledger's existing return-owned insert/cancel API likewise suffices; OS VCS production changes are not requested.

Do not widen this packet to Store production mutation behavior, StoreHistory, group history publication, SPR channel, PluginCommandIngress, Plugin lifecycle, Interaction, shared DSL, Flow mutation/diff/retirement, or generic serde. The by-value consuming-contract limitation is a separate root decision if broader unwind guarantees are required.

## Acceptance Sequence and Evidence

1. Freeze and fingerprint the exact reviewed source and actual neutral inputs/controller before the first run. Author schemas and handcrafted vectors first.
2. Add the native laws against the current concrete authorities, and execute bounded isolated REDs in the root-scheduled compiler slot. Keep the root grammar RED separate.
3. Apply only the reserved owner fixes. Replay unchanged neutral/native cases, then the existing grammar/envelope tests.
4. Require exact owner conservation through failure, no premature Complete, no over-budget disposal, recoverable callback unwind at the existing seam, and successful record order/shape preservation.
5. Record per-case native outcome, isolated RED exits, tool versions, before/after source/schema/controller hashes, retained logs, and any unresolved consuming-callback limitation. A source scan or Ajv pass is never a native ownership pass.

This task produced this design document only. All native tests, proposed fixture files and implementation steps above remain unexecuted/unwritten.
