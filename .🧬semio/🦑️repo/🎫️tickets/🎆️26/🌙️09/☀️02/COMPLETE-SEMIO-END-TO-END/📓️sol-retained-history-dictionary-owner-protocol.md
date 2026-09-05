# Retained History Dictionary And Record Owner Protocol

## Current Scope

This is the next unmounted implementation packet. Existing retained framing, input-witness and tagged-ID Rust bytes remain frozen while the coordinator repairs Flow. No Cargo or production mount is authorized by this document. The new neutral gate must initially fail at missing owner implementation after its independent records/ownership model succeeds.

## Exact Existing Seams

- `store/🧩️composition/🚪️open/📜️history::VerifiedMemberHistoryInput` owns the original sealed `MemberOpenRequest` and its exact committed-prefix span. Its private request retains operation, generation, expiry, expected artifact and owner. Its checked copy cannot expose a recoverable suffix.
- The dictionary/record owner is a child module of that history owner, so it can retain the existing witness without exposing a public raw request, raw span or separately authenticated range.
- `spr/📜️history/🛂️identity/🪪️id::RetainedHistoryIdV1` handles fixed-buffer tagged IDs and exact dictionary-index handshakes. It currently lacks a non-mutating completed-state observation: calling `finish()` on an incomplete ID permanently rejects. A multi-field record driver needs `is_complete()` (closed state observation, no byte exposure), added only at the coordinated API batch. Duplicating tagged-ID framing in the Rust record owner is not allowed.
- Existing `decode_history`, `apply_dict_record`, `decode_doc`, `decode_composition` and `parse_document_spr` remain whole-input/allocating or permissive paths. None may be called inside a bounded step.

## Owned State And Admission

`MemberHistoryDictionaryOwner::begin(input, schema, limits, context)` consumes the actual verified witness. Rejection returns exactly that witness; it never drops or reconstructs the original request. The schema is a static codec identity, not an externally supplied descriptor. The request's full expected artifact/owner and context are checked before admission, after each bounded unit, before Ready, and immediately before a private handoff.

The owner contains the witness; frame/payload cursors; one retained input byte; one tagged-ID cursor; dictionary range pages; aggregate counters; provisional record facts; and the last complete composition's match facts. No `P`, mutation, erased value, typed history, store or app is constructed.

Dictionary values are ranges into the same immutable verified input, not copied strings. Each page contains exactly 64 `(offset, byte_length)` entries (16 bytes per entry, 1,024 index bytes/page); at most 128 pages admit 8,192 entries. Page allocation is bounded and charged before one page is admitted. A failed record retains any provisional page for bounded close but does not publish its entries. Every lookup resolves through this owner's committed entry count and checked input witness; a caller cannot supply replacement bytes or an arbitrary offset.

## Record Grammar And Authority

The second pass derives every record boundary from the already verified SPR prefix. It never trusts a caller's offset list. Identity/dictionary records must use the uncompressed semantic profile; compressed identity payloads reject until a separate bounded decompressor exists. Uncommitted/torn suffix bytes are neither scanned nor indexed.

Dictionary deltas require format 1, exact prior committed entry count, bounded declared count, canonical lengths, valid UTF-8, aggregate byte capacity and exact payload EOF. Dictionary text may contain general text; tagged identity use separately applies the ID cursor's control/length rules. Entries become lookup-visible atomically only after the entire delta succeeds. Invalid UTF-8, a wrong base or a trailing field poisons the owner without partial visibility.

Exactly one document record is required. Document id and schema must match the immutable request/codec. Every composition record is fully parsed with format 1, known presence bits, exact EOF, bounded groups and pins; only the last complete committed composition supplies final owner/dialect facts. An earlier malformed composition always rejects. A well-formed foreign earlier composition may be superseded by a valid later one. Parent identity is compared as the full parsed artifact reference, together with slot and child id, never as a truncated id.

Resource limits are operation-wide: dictionary bytes sum over all admitted deltas, and pin groups and references sum over all parsed committed composition records, even if an earlier overlay is superseded. Total record ceilings also apply. Last-wins authority does not reset spent resource credits.

## Fuel, Cancellation And Retirement

Input copying, byte grammar work, dictionary-byte lookup/ID feeding, fixed-page admission and record publication are separate bounded units. A pending byte or partially decoded UTF-8/ID survives yield. Zero fuel/deadline makes no owner transfer. Cancellation, missing clock, expiry or operation/generation mismatch becomes sticky rejection and preserves the original input plus every allocated index page.

Close first disables lookup/handoff, retires fixed scratch under byte grants, retires at most one dictionary index page under exact 1,024-byte credit, then delegates original input/identity retirement to the witness. A page is released only after its entire byte credit is consumed. False terminal/over-budget nested witnesses reject. No blind close loop, `forget`, generic decoded-value authority or implicit nonterminal Drop is permitted.

Ready is not a document: it is a private retained semantic witness with the same original request and index ownership. The future operation may move the request into the proven Flow snapshot decoder while retaining the index owner in its active-state enum; the dictionary cannot be read until the exact same request is returned. Full context/parent-generation checks still precede typed-store hydration and the app-owned final transaction. That later integration remains open.

## Required Neutral And Native Evidence

The independent oracle constructs actual SPR records with third-party canonical LEB128/CRC, recomputes their full BLAKE3 commit chain, and feeds the shared independent framing inspector before interpreting any semantic payload. Hostile semantic records have valid framing, so a framing rejection cannot vacuously replace dictionary/identity enforcement.

Required vectors include multiple dictionary deltas; exact base and all-or-nothing publication; entry/aggregate-byte limits; invalid UTF-8/trailing payload; out-of-range lookup; duplicate document; wrong full owner/dialect; earlier malformed versus well-formed foreign composition; per-record and operation-wide pin limits; a two-page index; uncommitted dictionary suffix; and cancellation/authority loss before allocation, mid-delta, after record publication, at Ready and after handoff. Expected traces include private/visible entry counts, handoff count, original input ownership, allocated index pages and exact retired bytes.

No source model is native or public-member evidence. The final registered native gate must discover exactly the owned laws in one kernel test binary, record its hash and nonzero assertion counts, and leave all prior full-fleet RED qualifications intact.

## First Executed TDD Frontier

`bun ./📜️script.ts nx run @semio-tech/framework-os-host-rs:member-history-dictionary-check --skip-nx-cache -- --oracle-only`:

- Session `16280`: owned schema RED (`strictRequired`: conditional `value` lacked a local `properties` declaration); corrected without disabling strict AJV.
- Session `1377`: the independent model passed 4 accepted / 15 denied committed histories plus 11 lifecycle traces at three grants; intended final RED was missing `🗂️dictionary/🦀️.rs`, before any Cargo dispatch.
- Session `55062`: after audit repair, independent model passed 4 accepted / 26 denied histories, all 11 lifecycle traces at three grants, and exact input/index retirement up to 3,025 bytes. Final gate remains intentionally RED at missing Rust owner.

The enlarged fixture has an aggregate group-limit counterexample and same-byte-length changes to every one of the ten request identity coordinates. Expected document/dialect/owner values are derived from that retained request, not independently supplied expected strings. Document/child changes update both coordinates together, preserving the existing request-admission invariant; this final refinement was made after `55062` and requires a fresh source terminal. Every history row pins input and retired byte totals; the two-page case pins 905 input bytes + 72 identity bytes + 2,048 index bytes = 3,025 bytes. Dictionary values remain ranges into the actual independently verified committed prefix.

Source/native commands are registered through the existing host task router/project and launch seed at orders 419.997/419.998. Generation `40282` exited 0 for these entries. A later aggregate-foundation entry is being generated in `71275`; freshness follows its terminal. No dictionary Rust owner, member factory or public app entrypoint changed in this packet.

Final post-audit source `66812` exited 1 only at the intended missing dictionary Rust owner after **4 accepted / 26 denied** committed histories and **11 lifecycle traces × 3 grants** passed. Every lifecycle row now pins its own 315 input bytes and exact 387/1,411 total retirement bytes; it no longer accepts a self-derived total. Prior `14799` passed the same model before this final lifecycle pinning. The source gate deliberately remains red and is excluded from the independent five-law native foundation batch.

The coordinator subsequently released the shared target for the previously audited foundation only. Its three parent module mounts and native attempt are tracked in the main persistence report. This does not mount or implement the dictionary owner, add the tagged-ID completeness API, or activate any member factory.

Terra's final current-byte reread source-qualifies aggregate bytes/groups/pins, request-derived full identity, and all eleven fixture-pinned close traces, with no remaining model blocker (`📓️terra-retained-history-dictionary-record-audit.md`). This does not change the gate's missing-owner RED. Generation `71275` exited 0; freshness `71079` subsequently exited 1 solely because `.vscode/launch.json` became stale after the backend's new GIS entry. The backend owns the next coordinated generate/freshness; this lane does not hand-edit generated launch or launch a competing generator.

Backend-coordinated final generation `72742` and freshness `39009` both exited 0, preserving foundation 419.999, GIS 411.142, and the four concurrent Norm entries exactly once. This is shared generation evidence reported by its execution owner, not a duplicate run by this lane.

## Private Range Index Source

The new unmounted `🗂️dictionary/📇️index/🦀️.rs` stages `RetainedDictionaryIndex`, not the missing record owner. It owns at most 128 fixed 64-entry pages (1,024 bytes/page), retains provisional entries until full delta publication, verifies base/count/aggregate bytes and range overflow/monotonicity/verified-prefix bounds, and blocks every lookup after denial or close. A scalar range is not an input authority; only the future parent holding the actual verified request may read it. The primitive never duplicates tagged-ID grammar and does not decode records or UTF-8.

Empty general dictionary text is valid in the current producer/reader (`replication/📖️dictionary`), so the index deliberately accepts zero-length ranges. Identity use remains nonempty through the tagged-ID decoder. Two additional neutral rows pin that distinction: an unused empty string is accepted; referencing an empty string as document identity rejects. This avoids inventing a blanket dictionary restriction. The record owner must validate UTF-8 before staging each range, exact EOF before publishing a delta, and identity grammar before accepting any resolved identity; it must not treat index admission as text or record authority.

Close erases exact granted bytes inside one fixed page, releases no page until all 1,024 bytes were charged, releases at most one page per step, then reports terminal-empty. Provisional pages are equally retained. Two local Rust laws cover two-page atomic publication/late rejection, cumulative byte capacity, malformed ranges and exact 2,048/1,024/zero-byte retirement at grants 1/7/4096. They have not been mounted or executed, and are excluded from the now-green five-law foundation batch. The overall dictionary gate remains RED at the absent parent owner, as intended.

## Bounded Dictionary Payload Cursor Source

`🗂️dictionary/🧾️record/🦀️.rs` now stages a second private, unmounted primitive, `RetainedDictionaryDelta`. It consumes at most one wire byte per fuel unit; keeps canonical numeric and four-byte UTF-8 state; emits one retained Begin/Entry event at a time; refuses another byte while an event is unconsumed; and requires exact payload EOF. It does not read input, allocate strings/pages, resolve tagged IDs or publish a dictionary. Scalar events must be adopted by the future actual input owner. Partial UTF-8 bytes survive failure/cancellation and close under exact byte credit. Two local unrun laws use the neutral dictionary rows and hostile tail/nonminimal/UTF-8/cancellation cases. Syntax-only `3a6ceb` exited 0; no typecheck or native result is implied.

Record-owner integration must still pin retained scratch bytes in its neutral retirement totals. Existing full-owner corpus totals deliberately cover input/identity/index storage; an incomplete UTF-8 prefix additionally retains one to three cursor bytes, a fully buffered invalid scalar can retain all four, and a partially decoded ID may retain its own bounded scratch. That exact accounting must be represented before claiming a complete record owner. The model cannot silently reuse input/index totals as proof of scratch retirement.

Post-empty-text source `26072` reached **5 accepted / 27 denied** histories and eleven traces at three grants, then the intended absent-owner RED. A new BOM identity vector is being run as TDD: JavaScript's default TextDecoder can strip a leading BOM while Rust UTF-8 preserves it. The independent oracle must preserve exact bytes before identity comparison; a BOM-prefixed document id must not become the unprefixed authorized id. This change is only in the new dictionary model, not the already accepted five-law Rust inputs.

BOM TDD **67749 RED** reproduced the exact false authorization: `bom-does-not-rewrite-identity` returned null instead of identity denial. The new dictionary oracle now uses `ignoreBOM:true` to preserve the decoded BOM scalar, aligning exact byte identity with Rust. No existing mounted Rust primitive changed. Private range-index syntax-only **cdd7c1 GREEN**; final scoped diff check **00c8fd GREEN**. Neither is native index/record coverage.

Final BOM source **12633** exited 1 only at the absent full owner after **5 accepted / 28 denied histories + 11 lifecycle traces × 3 grants**. The general dictionary remains independent of the new narrow payload-cursor corpus.

## Record-Local Fixture And Source Terminal

The cursor now owns strict `🧾️record/🧬️schema/🔣️.json` and `🧾️record/🧫️fixture/🔣️.json`: **34 exact wires**, each with literal offset, completed-entry count, diagnostic and retained scratch byte count. Rows cover canonical multi-byte base/count/length; nonminimal/overflow encodings; held Begin/Entry before push and EOF; one/two/three/four-byte UTF-8; malformed/truncated/surrogate/out-of-range scalars; BOM/control/unused empty general text; capacity; and cancellation with zero through four scratch bytes. Every row executes at grants 1/7/4096 in the independent Bun/AJV/third-party LEB128/UTF-8 oracle.

`bun ./📜️script.ts nx run @semio-tech/framework-os-host-rs:member-history-record-check --skip-nx-cache -- --oracle-only`:

- **67187 RED exit 1**: independent 11 accepted / 23 denied rows passed, then the intentional source assertion rejected the Rust tests' missing local fixture binding.
- **99946 GREEN exit 0**: both staged Rust laws now consume the exact record-local fixture through one helper, retaining the earlier range and every-four-byte-scalar cancellation coverage. The source oracle passed 11 accepted / 23 denied × 3 grants and its binding/owned-law checks.

The unexecuted Rust helper checks consumed offset and fuel, sticky held-event errors without further consumption, exact fixture scratch count and bounded close totals. It does not replace the whole-owner scratch accounting requirement. The registered native command fails closed before Cargo unless the dictionary parent exists and mounts the cursor; the parent does not yet exist. Source/native launch entries are owned at orders 419.9981/419.9982. This is source-only cursor evidence, not native record/index, full dictionary, MemberFactory, public Flow or persistence activation.

Canonical generation **88895 GREEN** and freshness **27122 GREEN** now include those two entries; backend generator ownership was coordinated and released. Root retains the shared native target. Next staged dependency is record payload metadata observed from the already authoritative framing machine, not a parallel LEB/frame parser. The immutable `VerifiedMemberHistoryInput` remains the sole input-reading authority, and any metadata range is only a scalar observation until consumed with that exact witness.

## Existing-Verifier Record Observation

The unmounted `replication/📐️format/🔎️verification/🧾️record/🦀️.rs` extends the existing `RetainedSprVerification` in a child module. `observe_record_header()` only reads its existing stage, kind/flags, parsed canonical lengths and checked bounds; it never consumes bytes or implements another framing decoder. The non-clone scalar result carries frame/payload bounds and optional compressed raw size. No payload is copied or decompressed. Repeated observations do not advance the scanner; before a complete header, after a complete frame/finish, and on error/cancellation it produces no usable observation. A header observation alone deliberately proves neither its eventual CRC nor any commit. The dictionary driver must already own the verified immutable input, and must not accept a caller-supplied range.

The dedicated strict schema/11-row neutral corpus has exact absent/present observations for empty payload, multi-byte body/raw length, pending raw length, trailer traversal, completion, invalid flags/CRC and cancellation. `retained-record-observation-check --oracle-only` ran **16009 RED** on owned unsigned CRC coercion, then **65069 RED** at the intended missing Rust source after the oracle passed. After staging the accessor and exact fixture-bound Rust law, **54316 GREEN exit 0** passed all 11 rows (five scalar observations) and source guards. This is source-only. The new child module is not mounted, the exact native law has not run, and root's current Flow compilation is unaffected.

The future driver charges copy, existing framing feed, dictionary payload feed, range admission/publication and tagged-ID feed separately; it retains the current byte across each stage. The metadata module plus non-poisoning ID completion query remain a coordinated mounted batch, not an excuse to shadow either wire grammar.

Terra source-qualified this seam. Its `raw_bytes: u64` is only the canonical declared scalar, never an allocation credit; the future semantic owner must apply its own decompression/raw-byte limits before allocation. Pre-trailer coordinates do not establish CRC or commit validity. Source/native launch orders 419.951/419.952 were generated by **17126 GREEN**; **58798 GREEN** proved freshness. No parent Rust mount changed.

## Ordered Events And Combined Scratch Retirement

After independent audit, the record-local fixture now pins ordered `Begin(base,count)` and `Entry(offset,length)` events for all 34 rows. A compact fixture-owned count/stride expands the 128 empty entries, without consulting parsed output. Both Rust and the independent model compare the full event sequence. Final record source **38374 GREEN exit 0** retains 11 accepted / 23 denied rows at three grants. Terra reread found no new record parser defect; native execution remains pending.

Whole-owner TDD **4251 RED** reproduced the old 378-byte total against the correct 379 bytes for malformed UTF-8: one prefix byte was retained outside input/index storage. The shared independent UTF-8 scalar model now advances during actual committed dictionary traversal and retains that byte. Seven new strict neutral traces pin combined original-input, identity, provisional-index and scratch ownership, including malformed/truncated/full-invalid scalars and cancellation after one/two/three UTF-8 bytes. **78186 RED** found an incorrect fixture input estimate (303, not 304): replacing the 16-byte schema entry with four bytes removes 12 bytes. Literal input/retirement expectations were corrected accordingly, without weakening checks.

Final whole-owner source **8973** exited 1 only at the expected absent `🗂️dictionary/🦀️.rs`, after **5 accepted / 28 denied histories + 11 lifecycle + 7 exact scratch/input/index traces × 3 grants** passed. These new cancellation milestones occur after the dictionary byte was consumed; pending-copy and tagged-ID scratch retention still need their own full-owner traces. The actual Rust owner, dictionary/record mounts, semantic handoff and public member integration remain open. Scoped diff hygiene **f27906 GREEN**; current mount census found no dictionary/record/query mount in the frozen production parents.

Final current-source reruns: metadata **64233 GREEN exit 0** (11 exact rows/five observations), payload cursor **58867 GREEN exit 0** (11 accepted/23 denied with every ordered event at three grants), and scoped diff hygiene **b1d48e GREEN**. No native command or mounted Rust edit was made during the coordinator's current Flow build. The next coordinated mount must still run these exact Rust laws rather than crediting their source oracles as native evidence.

## Actual Unmounted Dictionary Owner

The prior missing-owner condition is superseded by staged `🗂️dictionary/🦀️.rs`, its fixed semantic sequencer at `🛂️identity/🦀️.rs`, and production-writer laws at `🧪️tests/🦀️.rs`. The parent remains unmounted. The operation owns the actual `VerifiedMemberHistoryInput`, private range index, existing framing scanner, pending wire/lookup bytes, payload cursor and tagged-ID cursor. It reuses the existing scanner and metadata observation rather than parsing another framing grammar. Every ID byte is delegated to the existing tagged-ID cursor; the semantic sequencer only handles DOC/composition fields, canonical counts and full request/owner comparisons. No typed snapshot, mutation, history, store or public member is produced.

Begin returns the original input on admission failure. Every unit and final private handoff checks operation, generation, cancellation, clock and expiry. Deltas become visible only after exact payload EOF and a complete frame from the existing scanner; final handoff requires the original committed end/sequence/chain and exact unique document/final composition identity. Cumulative dictionary bytes, pin groups and pins never reset for later records. Failure poisons lookup but retains all owners. Close separately retires pending bytes, UTF-8/ID scratch, fixed pages and the original request. No production caller is wired.

The two staged native laws use production SPR framing and the actual retained input verifier before dictionary admission. They bind 33 histories, 11 authority transitions, seven payload-scratch traces and nine pending-copy/ID traces at grants 1/7/4096. The ordinary two-delta case additionally pins 14 ordered Begin/Entry/publication visibility events and ten exact ranges. Raw-DOC rows construct raw tagged IDs in both first-party writer and independent oracle; native cancellation predicates inspect real payload/ID state, not a nonexistent production transition.

Registered `member-history-dictionary-check --oracle-only` terminals:

- **50966 RED exit 1**: 33/11/7 model and actual owner/law bindings passed; next failure was the frozen missing `RetainedHistoryIdV1::is_complete()` API.
- **39760 RED exit 1**: enlarged 33/11/7/9 model passed; its TDD guard rejected missing native pending-copy/ID fixture binding.
- **16487 RED exit 1**: all nine native rows and ordinary event/range bindings are staged; the full model passed again, then the missing non-poisoning completion query failed before Cargo.

Nine fixture-owned totals cover one pending original byte, a separate dictionary lookup byte, partial ID text, completed 11-byte ID, partially retired 10-byte ID, raw ASCII and partial multibyte ID scratch. The oracle retains and zeroes these buffers rather than deriving retirement from expected answers. Staged cancelled native rows require zero additional fuel/offset, sticky denial with a fresh uncancelled context, no handoff and exact fixture input/index/scratch retirement.

### Remaining Integration Authority

Root's Flow build retains exclusive mounted-source/target ownership. `verification::record` and `history::dictionary` remain unmounted and the completion query is absent. This lane ran no Cargo or mounted edits. A coordinated batch must add the non-mutating query with native proof and execute the exact owner, record, index and metadata laws; source models do not qualify compilation or native ownership.

The staged `begin(input, schema: &'static str, ...)` tests an expected schema but does not establish its selection authority. Before factory integration, derive it from the closed selected MemberFactory/declaration together with the exact request ref/owner, behind a private expected-identity input; never accept an external schema string. This private source-only witness does not prove that link. Typed decoding/replay/initialization, public child admission and final app transaction remain open.

## Coordinated Mount And First Native Attempt

After root Flow `60581` reached a terminal native assertion failure, the coordinator explicitly released the source freeze. The replication retained-verifier parent now mounts the scalar `record` observer; crate-private member history mounts `dictionary`; and tagged-ID `is_complete()` checks only error absence and the existing terminal stage. No MemberFactory or public caller was added. Eleven new strict neutral completion rows cover empty/tag/length/partial raw input, pending/partial/complete dictionary, UUID prefix/completion and cancellation. The existing exact ID law probes twice and compares retained bytes, numeric state, diagnostic and fuel without calling destructive `finish()` while incomplete.

ID source TDD **7897 RED** passed 20 wire rows plus 11 completion boundaries, then rejected the absent production query/native binding. Final source **50041 GREEN exit 0** closes that exact source obligation. Dictionary source **68455 GREEN exit 0** and record-observation source **83730 GREEN exit 0** passed current mounted-source guards and their complete neutral corpora. None is a Rust assertion result. The private post-handoff dictionary witness now retains the checked schema, makes authority denial sticky, poisons lookup, and still owns bounded close; its two lifecycle rows cover denial replay under a fresh context.

Root subsequently released the shared native target. **70042 ACTIVE** executes the registered `member-history-dictionary-check` without `--oracle-only`, exactly two owner laws through one kernel test binary. Environment matches launch order 419.998: absolute ticket `🗑️generated/native-openable-provider-sol-target`, `🗑️generated/member-history-dictionary-exact`, one Cargo job, both Rust wrappers disabled. All mounted bytes are frozen through this attempt. No native owner, metadata, record or index result is claimed until terminal evidence is captured.

Additional authority limits from independent audit remain explicit: pin fields are currently grammar/count checked, not authorized against actual checkpoint membership; the later typed `validate_composition_pins` boundary must reject syntactically valid foreign/non-admissible pins. `MemberOpenRequest` has no space scope, so the selected-factory schema/ref/owner binding must be constructed inside an already authorized document/space context. This scanner-level witness cannot substitute for either check.

## Native Owner Terminal: GREEN2

**70042 completed exit 0**. The registered gate selected exactly two laws from one kernel test executable after a **1m43s** test-profile build. Receipt root: `🗑️generated/member-history-dictionary-exact/exact-cargo-laws-DbP7cw/00`. Executable `semio_framework_os_kernel-774d47594b46b30f`, SHA-256 `c3bdb2cacc68c3de832f319e5766ca71c14f1bacbdf13366a45285f135725abe`.

- `os_store::component::member_open::history::dictionary::tests::member_history_dictionary_is_atomic_and_bounded_by_neutral_records`: **1 passed, 0 failed, 0 ignored**, 0.39s. Runtime output confirms 33 production-writer histories at three grants; the law also asserts fixture-owned ordered visibility events and exact ranges.
- `os_store::component::member_open::history::dictionary::tests::member_history_dictionary_retains_every_denied_owner_until_exact_close`: **1 passed, 0 failed, 0 ignored**, 0.10s. Runtime output confirms 11 authority transitions, seven payload scratch cases and nine pending-copy/ID scratch cases at three grants, including literal complete retirement and sticky denial.

The kernel binary reported 1,004 filtered tests per exact invocation; the runner independently discovers each full name exactly once and records two executed assertions, so this is not a zero-test/filter success. The real mounted verifier/observer, input owner, index, payload cursor, ID cursor and semantic sequencer were used by these laws. Dedicated primitive/query/metadata laws were not separately executed in this run and are not credited as such. The shared target was immediately released to root; no subsequent Cargo or deletion occurred. This closes this bounded dictionary-owner native packet, not selected-factory/space/pin authority, typed history hydration, MemberFactory or public child opening.

## Canonical Composition Flag Follow-up

Selected-factory native session 24681 exposed that the older owner/fixture required the additive `REC_COMPOSITION` overlay to be critical although production `encode_history` declares and writes it noncritical. The retained profile now requires exact flags: document/dictionary `2`, composition `0`. The neutral corpus contains 36 CRC-valid histories, adding exact critical-composition and noncritical document/dictionary denials.

Registered session **8902 GREEN**, exit 0, reran both dictionary laws on the final source. Receipt `member-history-dictionary-exact/exact-cargo-laws-X2Egwr/00/receipt.json` records assertions 2 and the kernel executable SHA-256 `1c09a11c36663be9d06f8f806ff36aa19075e4f3beea4b91d84d743be38b1fb4`. This supersedes the old flag portion of 70042 while preserving all other bounded owner evidence and nonclaims.
