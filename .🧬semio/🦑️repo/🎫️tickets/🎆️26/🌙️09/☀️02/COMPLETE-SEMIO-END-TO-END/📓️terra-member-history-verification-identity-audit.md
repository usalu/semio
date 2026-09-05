# Member History Verification and Semantic Identity Audit

## Scope and current verdict

Read-only current-byte audit of two deliberately staged, unmounted source packets:

- semantic document/owner identity corpus at `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛂️identity/{🧫️fixture,🧬️schema}/🔣️.json` and `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:245-330`;
- caller-retained history request corpus at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/{🧫️fixture,🧬️schema}/🔣️.json` and `…/host/📦️packages/🦀️rust/📜️script.ts:332-409`.

The semantic identity corpus is a good **source-only** specification for ordering identity admission before typed snapshot decode. The request/history lifecycle packet now derives its framing outcomes and models retained ownership in neutral data, making its bounded framing/handoff contract **source-supported**. Its Rust `MemberHistoryVerification` is staged at `open/📜️history/🦀️.rs`, but deliberately unmounted; no native, typed decode, semantic hydration, or public member-factory acceptance is claimed.

No Cargo or Nx task was run for this audit.

## Current-byte correction — owner oracle and visibility

The two owner-oracle REDs below are **source-closed** in the latest staged bytes; their sections remain as historical rationale.

- The neutral host oracle now imports a test-only, `import.meta.main`-guarded `inspectRetainedSprNeutral` from the replication script. That independent TS grammar derives exact/torn/paged-tail/bad-CRC outcomes from actual input bytes, CRC-32C, and BLAKE3 instead of trusting fixture `verifiedEnd` (`host 📜️script.ts:337-390`; replication `📜️script.ts:39-78`). It is independent of the Rust scanner and shares no production codec.
- Lifecycle data now has thirteen explicit holder traces with `retainedBytes`, `retiredBytes`, and `handoffs`; the oracle proves a valid one-time transfer, repeat denial, pre/post-transfer authority denial, and bounded terminal retirement (`open/📜️history/{🧬️schema,🧫️fixture}/🔣️.json`; host script `:391-425`).
- `MemberHistoryVerification` and `VerifiedMemberHistoryInput` are `pub(crate)`, not externally exported capabilities. The witness owns a bounded close path, while the verifier retains request ownership whenever its final check rejects (`open/📜️history/🦀️.rs:13-171`).

This is still staged source evidence. `open/🦀️.rs` does not yet mount `history`, and the retained scanner remains unmounted; the host target's native phase remains intentionally unavailable.

## Current Rust owner reread

The staged owner source has sound bounded internal mechanics:

- `MemberHistoryVerification` retains the admitted `MemberOpenRequest`, a fixed-memory scanner, one pending byte, and no copied history allocation (`open/📜️history/🦀️.rs:13-24`). It applies request authority before and after each copy/hash unit, charges the request copy and scanner push separately, rejects sequence-zero/header-only input, and retains a verified torn-prefix span without semantic publication (`:40-91`).
- `take_ready` checks authority before and after the one handoff credit, then moves request plus private-field span exactly once (`:94-103`). A failed check leaves the request in the verifier. `VerifiedMemberHistoryInput` only reads up to `span.end` through the original authority-checked request and has its own bounded `ErasedSnapshotRetirement` path (`:107-171`). This closes the earlier risk that a post-transfer semantic refusal could strand the input.
- The two staged native laws exercise five history shapes over grants 1/7/4096, exact retired bytes, pre/post-handoff authority failures, no second take, and terminal close (`:241-312`). They are source evidence only until the mount and exact selector compile/run.

### Historical RED — staged visibility initially exported a raw history capability before the semantic consumer existed (source-closed)

The requested handoff is described as private, but once parent `member_open` adds the script-required `pub mod history;`, the current `pub struct MemberHistoryVerification`, `pub struct VerifiedMemberHistoryInput`, public `take_ready`, and public `copy_verified_history_chunk` become externally callable (`open/📜️history/🦀️.rs:13-24,94-147`; parent `store/🦀️.rs:22-25`). That gives arbitrary crate consumers a raw committed-history reader before a typed semantic consumer has been deliberately chosen.

Current source restricts the types to `pub(crate)`, so no external crate receives a raw-history capability. Keep them crate-private until the internal semantic decoder consumes the witness; do not make a public raw-span/history API merely to satisfy this staging packet.

## Semantic identity contract

The current 29-row identity corpus requires exact document id/schema, full dialect `(kind, standard, subset)`, and an exact optional owner triple. Its strict mini-codec independently checks canonical LEB128, fatal UTF-8, bounded control-free identifiers, dictionary ordinal resolution, document/composition version 1, known composition presence bits, and exact payload EOF (`host 📜️script.ts:253-300`; identity schema `:3-41`). It covers raw and prefix-UUID ids, missing/malformed/tail-bearing records, altered doc/schema/parent/slot/dialect, dictionary capacity, duplicate documents, and composition pin-group capacity.

Two policy decisions are correctly explicit in the current fixture:

- `unowned` carries schema-required `expectedOwner: null` and switches both expected authority and composition overlay to the unowned form (`identity fixture:19`; schema `:32-40`; script `:308`). This supersedes the earlier ambiguous non-null fixture owner.
- `committed-last-wins` parses a structurally valid earlier foreign composition before parsing the final valid composition; only the final overlay is compared to authority (`script:320-323`). This preserves the existing last-committed composition meaning without skipping structural validation of an earlier row.

The future Rust semantic stage must use the same authority shape as `validate_member_history_identity`: parse the parent URI to an `ArtifactRef` and compare the full typed value, not merely raw URI text (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3153-3167`). It must reject a duplicate `REC_DOC`; current generic `decode_history_from` overwrites `log.doc_id/schema` and is not an admissible substitute (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:1565-1570`). It must also keep the stricter v1/exact-EOF/presence-bit behavior rather than routing through generic `decode_doc`/`decode_composition`, which permit format zero and do not require EOF (`history/🦀️.rs:699-707,1247-1276`).

### Identity residuals

The contract declares both a one-megabyte dictionary-byte cap and an 8,192 total-pin cap, but its fixed seven-entry corpus cannot cross the dictionary-byte cap, and the 29 rows only exercise pin-group capacity. Add neutral hostile rows whose per-case selected limits are below the fixture's actual bytes/pins, or a bounded generated fixture dimension, so both source and independent oracle demonstrate the two stated limits. This does not require a large fixture or an allocation-heavy test.

## Request + SPR owner contract

`MemberOpenRequest` is a suitable retained primitive: it owns sealed pages, exact expected identity/owner, operation/generation/expiry, applies `StepContext` checks before/during copies, and has explicit bounded retirement (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:39-201`). Its `admitted_expected` now safely returns `Stale` for closed/detached input before exposing identity (`:63-68`). The staged lifecycle fixture correctly names normal, torn, paged-tail, no-commit, bad-CRC, and cancel/stale/expired authority cases (`open/📜️history/🧫️fixture/🔣️.json:1-29`).

### Historical RED 1 — the independent owner oracle initially assumed, rather than derived, tail verification (source-closed)

`MemberHistoryInputScript` reconstructs and checks the 255-byte committed prefix, but for `torn` and `paged-tail` it assigns `row.verifiedEnd == history.length` and counts two abstract credits per byte (`host 📜️script.ts:350-389`). It never feeds the appended three-byte torn suffix or the 4,244-byte declared-but-incomplete frame to an independent framing state machine. Thus a faulty Rust operation that accepts a complete malformed tail, prematurely publishes a tail, or fails to distinguish a CRC-valid torn suffix from a complete invalid frame can still agree with this oracle.

Current source implements that repair in test-only `inspectRetainedSprNeutral`, imported through an `import.meta.main` guard. It derives each input span without invoking the staged Rust scanner or production parser.

### Historical RED 2 — lifecycle rows initially did not prove retention or exactly-once transfer (source-closed)

The lifecycle loop only mutates booleans and compares an expected error string (`host 📜️script.ts:390-402`). Rows contain no owner state, retained/retired-byte result, positive `Ready → witness` transfer, or repeated-take denial. Source-name checks at `:405-408` cannot substitute for those transitions.

Extend the neutral lifecycle trace with explicit states such as `input`, `pending`, `ready`, `witness`, and `closed`, plus exact retained and released bytes. It must prove:

1. cancel/stale/expired at each named pre-transfer point preserves the original `MemberOpenRequest` for bounded close;
2. an authority-valid ready transition moves request plus scanner span exactly once into a non-`Clone` private witness;
3. a second take fails without moving or publishing anything;
4. authority is rechecked immediately before that one transfer, and a failed recheck leaves the request in the owning operation;
5. close after each denial/success releases the declared input/identity bytes under positive grants and reaches a terminal-empty witness.

These are now visible in the thirteen-row neutral trace and its independent holder-state model. Native execution remains separately pending.

## Integration requirements after source proof

The staged `MemberHistoryVerification` should remain private and unmounted until it:

1. owns the original `MemberOpenRequest` throughout framing, separate bounded history copy and hash/scanner credits, and semantic identity parse;
2. checks context operation, generation, cancel and expiry before/after every suspension or page turn;
3. returns the exact request to its operation on every denial, never a detached `VerifiedSprSpan`;
4. makes `take_ready` the only move of request plus span, with a final authority check and a non-clone private witness;
5. performs no `P`/`Mutation` decode, store initialization, graph/map publication, or `MemberFactory` API change in this packet.

The earlier input-only target is `bun nx run semio-framework-os-host:member-history-input-check`; it remains source-oracle-only by design. Its historical assertion that the parent lacked a `history` module is superseded by the five-law foundation mount below. The identity source target is `bun nx run semio-framework-os-host:member-history-identity-source`. Neither source target, by itself, is end-to-end acceptance. The retained operation still needs a separately reviewed semantic/typed integration into the member-factory/public child-open transaction.

## 2026-09-04 five-law foundation mount — isolated source review

The current foundation packet mounts exactly three ancestors:

1. `protocol_format::retained` through
   [`format/🦀️.rs`](../../../../../../../../🧰️framework/🔨️modules/📡️replication/📐️format/🦀️.rs:13);
2. crate-private member-open `history` through
   [`open/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:4);
3. crate-private SPR semantic-ID primitive through
   [`spr/history/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:12).

The registered foundation script checks those exact declarations and selects
only two retained-framing laws plus the two request/witness laws and the one
tagged-ID law ([host script](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:493-516)). Dictionary is not mounted: its parent has no
`mod dictionary`, and no dictionary law is in the five-name selector. A
current-tree reference census finds `MemberHistoryVerification` and
`VerifiedMemberHistoryInput` only in their private module/tests; no
`MemberFactory::open`, `open_member_store`, or public child transaction calls
them. Thus this packet cannot, by source structure, hydrate a typed member,
alter the legacy factory path, or publish a store/graph entry.

The newly mounted ownership boundary remains narrow and sound at source level:
the verifier retains the admitted request through framing, rechecks the
request's operation/generation/cancel/expiry authority before and after every
copy or scanner advance, and transfers request plus verified span only in
`take_ready` after a second authority/fuel check
([history owner](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:46-104)). The witness stays
`pub(crate)`, non-clone, bounds copies to its committed span, and both owners
require their explicit bounded retirement paths before `Drop`
([same file](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:107-181)).

No source coupling defect was found in this five-law scope. The compile/runtime
terminal is pending provider session `23286`; even a green terminal would be
bounded native evidence for framing/request/ID ownership only. It would not
accept semantic document identity, dictionary lookup, typed decode,
`MemberFactory` integration, or publication.

### Coordinator-observed native terminal — session 23286

Provider session `23286` is terminal green. I independently inspected its
retained exact-law receipts under
[`member-history-foundation-exact`](../🗑️generated/member-history-foundation-exact):

- replication `protocol` receipt `7fa620f0…225a35` selects exactly two laws:
  real-writer committed/torn-prefix verification and hostile-frame denial;
  each reports one pass (the second covers 26 hostile frame cases, ten
  compressed grammar cases, and cancellation at every 256-byte boundary);
- OS-kernel receipt `938ea378…3cd29c` selects exactly three laws: five wire
  shapes under grants 1/7/4096 and a one-use handoff with 4,531-byte paged
  retirement; thirteen authority-transition traces; and twenty tagged-ID
  wires under the same grants. Each exact selector passed with 997 unrelated
  kernel tests filtered.

This is a non-vacuous **five-assertion bounded native pass**, clearly distinct
from this audit's source review. It accepts only retained SPR framing,
private request/witness handoff, bounded close, and tagged-ID cursor behavior.
It does not change the explicit RED/nonclaims for dictionary owner/index
integration, full semantic document/owner verification, typed snapshot decode,
member factory, store/graph publication, or an end-to-end child open.
