# Retained History Dictionary and Record Audit

## Current verdict — source-staged RED; no native or end-to-end credit

Audited 2026-09-04 without Cargo/Nx. The provider subsequently staged the
dictionary neutral fixture, schema and independent host model at
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary`.
There remains no dictionary Rust owner or completion query in the current tree,
so the newly registered source target is intentionally RED and there is no
native assertion or end-to-end credit. The protocol and the model are useful
implementation contracts, not execution evidence.

## Current neutral corpus and model

- The fixture defines nineteen committed-prefix cases and eleven lifecycle
  traces under grants 1, 7 and 4096, including atomic delta tails, wrong base,
  UTF-8, missing lookup, duplicate document, identity mismatch, last-complete
  overlay selection, aggregate dictionary bytes/pins, a second index page, and
  an uncommitted suffix
  ([fixture](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧫️fixture/🔣️.json:1)).
- `MemberHistoryDictionaryScript` validates the schema with AJV, makes actual
  SPR-like framed bytes and asks the independently maintained retained-SPR
  inspector for the committed range before running its separate LEB/UTF-8/CRC/
  BLAKE3 model
  ([host script](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:493)).
  It is a neutral oracle, not production decoding. Its registration correctly
  requires the two future Rust laws and a non-poisoning ID completion method
  before it can report a green source gate (lines 683-689).

### Current source-oracle REDs

1. **`pinGroups` is not aggregate.** The model checks only the current
   composition record (`groups > limits.pinGroups`) and never retains a
   group-total ([host script](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:612)). Its current
   `aggregate-pin-limit` vector exercises aggregate pins, not aggregate groups.
   Two individually valid composition records can therefore cross the declared
   operation-wide `pinGroups` bound. Add an aggregate group counter, an
   underflow-safe remaining-cap check before record acceptance, and a
   two-overlay `pinGroups = 1` hostile row.
2. **Request authority is not bound by the neutral model.** The ten-field
   `requestIdentity` fixture is only summed for retirement accounting (line
   632), while semantic comparison uses a separate `expected` object (lines
   621-622). A same-byte-length substituted request artifact/dialect/parent/
   slot/child could therefore leave the oracle green. The model needs to derive
   or assert the request's artifact+dialect and complete owner coordinate
   against `expected` before scanning, with same-length mismatch vectors. The
   document schema is a separate required owner input because it is not carried
   in the ten request fields.

### Coordinator-observed source terminal

Provider session `1377` reached the independent model and reported four
accepted / fifteen denied corpus cases with all eleven lifecycle traces across
three grants. It then failed as intended on the absent dictionary Rust owner
and absent `RetainedHistoryIdV1::is_complete()`. This is useful TDD evidence
for the model only; it is neither a source pass nor native evidence. The
provider accepted both REDs above and is amending the staged model before any
owner/mount/Cargo work.

## 2026-09-04 reread — aggregate/authority corrections landed, lifecycle-byte RED remains

The two earlier neutral-model findings are source-closed in the current staged
bytes. `MemberHistoryDictionaryScript` now retains `state.pinGroups` and checks
each overlay against the remaining aggregate capacity before adding it
([host script](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:556,622)); the
`aggregate-group-cap` fixture row is a two-overlay hostile at limit one. It
also constructs record expectations only from the ten retained request fields
([same script](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:540-555)), then rejects each same-byte-length coordinate mutation through the
document/owner/dialect equality check. The document/child fields are mutated
together only to preserve the separately required admitted-request equality;
the unchanged parsed history then proves that the derived request authority,
not merely that internal equality, governs admission.

The fixture now pins `inputBytes` and `retiredBytes` for every normal/hostile
history case, and the model checks both after each grant
([fixture](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧫️fixture/🔣️.json:10),
[model](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:665-668)). The second
source-oracle terminal was coordinator-reported as four accepted / twenty-six
denied histories and eleven lifecycle traces under grants 1, 7 and 4096, then
the intended missing-owner RED. That is still only a neutral source model;
there is no Rust owner, parent mount, native law, or publication evidence.

One material neutral-fixture hole remains. The eleven `lifecycle` rows pin
entries, pages, handoffs, and outcome but not their close-byte totals
([fixture](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧫️fixture/🔣️.json:41)). Their model check derives
retirement from the same mutable model expression (`inputBytes + 72 + pages ×
1024`) instead of comparing a fixture-owned expectation
([host script](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:681)). Thus a self-consistent
close-accounting defect can pass cancellation-before-allocation, provisional,
published, ready, and witness traces. Add required `inputBytes` and
`retiredBytes` to every lifecycle row and compare the model output to those
external values before an owner treats its close accounting as covered.

### Current-byte correction — lifecycle close accounting source-closed

The lifecycle accounting RED above is superseded. Its JSON schema now requires
`inputBytes` and `retiredBytes` on every lifecycle row
([schema](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧬️schema/🔣️.json:51-58)); all eleven rows independently pin
the 315-byte retained input and the appropriate 387- or 1,411-byte terminal
retirement ([fixture](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧫️fixture/🔣️.json:44-55)). The model compares both
values after each of grants 1, 7, and 4096
([host script](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:710)). This restores an external accounting assertion across pre-allocation,
provisional-page, post-publication, ready, and post-handoff cancellation
states; it is no longer self-derived.

The dictionary packet is now **source-model qualified only**: aggregate
dictionary bytes, groups, pins, request-coordinate authority, atomic delta
publication and exact lifecycle retirement are modeled with an independent
AJV/LEB/CRC/BLAKE3/UTF-8 oracle. There is still no `dictionary/🦀️.rs`, no
`RetainedHistoryIdV1::is_complete`, no parent mount, no native law terminal,
and no semantic/typed/member-factory/publication acceptance.

## Reviewed frozen prerequisites

- [`RetainedHistoryIdV1`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛂️identity/🪪️id/🦀️.rs:11)
  is a fixed 256-byte tagged-ID cursor. It charges one input or dictionary byte
  per unit of caller fuel, constrains raw and dictionary IDs, validates UTF-8 and
  Cc controls, and requires the caller to supply the exact requested dictionary
  index before dictionary bytes are accepted (lines 36-42, 82-107, 110-149).
  It is explicitly not dictionary authority.
- The accompanying neutral source gate is registered as
  `member-history-id-check --oracle-only` in
  [`📜️script.ts`](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:436),
  with strict AJV plus independent LEB128, fatal UTF-8 and UUID formatting
  (lines 439-489). This validates the 20 tagged-ID vectors only; the module is
  deliberately unmounted and its native law is not executed.
- [`MemberHistoryVerification`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:13)
  owns a sealed `MemberOpenRequest`, copies one framed-history byte at a time,
  rechecks operation/generation/cancel/clock authority around work and handoff,
  and transfers a private verified span exactly once (lines 46-104). Its own
  header comment correctly makes no semantic-identity, typed-decode, or
  publication claim (lines 1-2). Both owners use bounded retirement rather than
  `Drop` cleanup (lines 147-181).

## Material integration blocker

`RetainedHistoryIdV1::finish()` rejects and permanently latches `Malformed`
when the cursor is not `Done`
([id cursor](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛂️identity/🪪️id/🦀️.rs:138)). A record owner must never use `finish()` as an
end-of-record probe. Before it accepts a multi-ID record, it needs a
non-mutating `is_complete()` (or an equally narrow read-only completion query),
then calls `finish()` only after completion. Re-parsing a wire prefix or
reimplementing tag/LEB/UTF-8 grammar would create divergent authority and is
not acceptable.

## Required admission properties when staged source appears

1. A single owner must retain the original `VerifiedMemberHistoryInput` and
   return/retire that same owner on every denial, cancellation, expiry,
   generation mismatch, or close. It must not expose a detached span, copied
   suffix, or public dictionary lookup.
2. Charge aggregate dictionary bytes, range-index bytes, pin count and tagged-ID
   work across the entire committed prefix. A later composition record may be
   the selected value only after every earlier record parsed successfully; it
   must not reset capacity accounting or publish an earlier partial delta.
3. Admit a dictionary delta atomically only after canonical format/base/count,
   exact payload EOF, UTF-8/control rules and aggregate caps all hold. A failed
   delta must retain its provisional page solely for bounded close and leave no
   lookup-visible entries.
4. Each tagged ID must obtain its dictionary entry only through the exact
   pending index from `lookup()`, feed that entry once through
   `begin_dictionary`/`push_dictionary`, and require the non-mutating complete
   query before record completion. No shadow LEB/UTF-8 parser is permitted.
5. Native proof must execute the record owner under grants 1, 7 and 4096 and
   cover cross-page input, aggregate bytes/pins, wrong index/base, partial
   UTF-8, cancellation at wire/dictionary/page boundaries, complete-ID-required
   rejection, one-use handoff and exact bounded retirement. The existing
   tagged-ID and framing source gates are necessary but insufficient.

## Nonclaims

The current audit does not establish dictionary authority, semantic document or
owner identity, typed snapshot decoding, `MemberFactory::open`, store/app
publication, or runtime behavior. Existing Flow native lifecycle laws remain
separately unrun.

## 2026-09-04 staged range-index primitive — isolated source review

The newly staged
[`RetainedDictionaryIndex`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🦀️.rs:23)
is not mounted by `open/history` and is absent from the five-law foundation.
It stores only offset/length scalar ranges; it has no input reader, dictionary
text, request, `StepContext`, semantic decoder, or member-factory call.

For that intentionally narrow role, the source is sound. It caps the dense
index to 128 fixed 1,024-byte pages, accepts an exact-base delta, keeps
`visible` unchanged until `publish_delta`, monotonically charges bytes across
provisional and published entries, disables all lookup after a sticky
rejection, and wipes/relinquishes exactly one page under bounded close
([same index](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🦀️.rs:38-113)). Its two local Rust laws are staged only; no
neutral-corpus binding, mount, or native terminal was observed here.

An empty range is intentionally not rejected by this scalar range index. That
is correct only because first-party replication dictionaries permit general
empty text; the future record owner must validate nonempty identity only when
a tagged-ID actually consumes an entry. The neutral record corpus therefore
needs both an accepted unused-empty dictionary entry and a rejected
referenced-empty identity. Provider has accepted that precise separation. It
must not tighten general dictionary text or make this index a shadow tagged-ID
parser.

## 2026-09-04 staged delta-record cursor — source review

[`RetainedDictionaryDelta`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧾️record/🦀️.rs:14)
is likewise unmounted and has no input reader, range index, tagged-ID cursor,
request authority, semantic decode, or publication call. Its narrow parser is
source-sound: each wire byte consumes one fuel unit, LEB128 is bounded and
canonical, a `Begin` or `Entry` event must be consumed before another byte is
accepted, UTF-8 uses a fixed four-byte scratch buffer with Rust validation,
and `finish` requires both exact EOF and no pending event (lines 33-131).

The parser intentionally permits empty general dictionary text. Nonempty and
control-free requirements belong only to a subsequent tagged-ID consumption;
the cursor must not become a shadow identity grammar. Its partial multibyte
UTF-8 scratch is the important outstanding full-owner accounting boundary:
malformed/cancelled prefixes can retain one to three bytes even though a
successful completed string has zero scratch. A future owner must include that
fixture-pinned dynamic amount in exact retirement, in addition to request and
index pages.

Before its local Rust laws can support a packet claim, add independent neutral
vectors for a pending Begin/Entry followed by another push (sticky `State`
without offset/fuel advance), multi-byte canonical/nonminimal/overflow
base/count/length LEBs, pending-event EOF, all UTF-8 scalar widths with
malformed/truncated variants, and the unused-empty positive versus
referenced-empty-ID denial. The reported `3a6ceb` syntax/source result and the
two staged local native laws are not a native record-owner terminal.

## 2026-09-04 expanded record corpus — current binding RED

The new local record corpus now does include the previously missing hostile
wire states: exact event fencing, UTF-8 scratch lengths zero through four,
truncated/surrogate/out-of-range scalars, canonical/noncanonical/overflow
LEBs, capacity, EOF tails, and cancellation at every four-byte scalar
boundary ([fixture](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧾️record/🧫️fixture/🔣️.json:2-38)). Its schema fixes the 34
rows and grants 1/7/4096, including a fixture-owned `scratchBytes` value
([schema](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧾️record/🧬️schema/🔣️.json:3-18)). The host model uses fatal `TextDecoder` and
`@webassemblyjs/leb128`, and wipes the fixture-pinned UTF-8 scratch one grant
at a time ([host script](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:493-556)). This is a useful, source-only neutral oracle.

However, the staged Rust laws are currently bound to the wrong fixture:
[`record/🦀️.rs:149`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧾️record/🦀️.rs:149)
uses `include_str!("../🧫️fixture/🔣️.json")`, which resolves to the parent
dictionary-owner fixture, rather than this record's
`record/🧫️fixture/🔣️.json`. Its two tests therefore construct only the old
`dictionary`/`secondDictionary` happy paths and do not execute any of the 34
new record vectors. The host gate itself correctly requires the local fixture
spelling and both law names before it may claim a source law
([host script](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:557-560)), so `member-history-record-check -- --oracle-only` must fail on the current bytes rather than silently certify the mismatch.

The record also remains outside the mounted production chain:
`open/📜️history/🦀️.rs` declares no dictionary module, and no
`dictionary/🦀️.rs` exists. The script's later mount assertion points at that
nonexistent parent and is presently unreachable because the command accepts
only `--oracle-only` ([host script](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:493-495,561-564)). Correct the direct local fixture reference first; then, separately, add a real bounded owner/mount and make its native gate selectable. Do not convert this isolated cursor into dictionary authority merely to make the tests run.

## 2026-09-04 local-record binding superseded; event trace still under-specified

The direct-local-fixture finding above is source-closed in the current tree.
Both staged Rust laws now load `record/🧫️fixture/🔣️.json` through their shared
`verify_payload_fixture` helper, selecting all 11 accepted and 23 denied rows
at grants 1, 7, and 4,096. The helper pins one-byte fuel/offset progress,
sticky-error non-progress, final offset, entry count, retained UTF-8 scratch
0–4, and bounded scratch retirement. Coordinator-reported session `99946`
is a source-oracle terminal only; the record module remains unmounted and its
two Rust laws have not been executed in this audit.

One current neutral-corpus gap remains before treating the scalar event
contract as independently specified. Local successful rows carry only a
single `hold` value plus final entry count; they do not encode the ordered
`Begin { base, count }` and `Entry { offset, length }` sequence. The Rust
test's two parent-fixture happy paths check ranges, but they are supplementary
examples rather than the local 34-row neutral corpus. A wrong implementation
could emit a normal `Begin` or `Entry` at the wrong successful byte boundary
and still satisfy those local rows. Add fixture-owned expected event facts (or
an exact event trace), including multi-byte base/count/length cases, and have
both the Rust helper and independent Node model assert them. This is a
source-only coverage RED, not a parser or authority finding.

The bounded parser itself remains narrowly classified: no dictionary input
owner, aggregate index/pin retirement, tagged-ID resolution, semantic
identity, typed member decode, `MemberFactory::open`, publication, mount, or
native runtime evidence is provided by `99946`.

## 2026-09-04 ordered scalar events — source coverage PASS

The event-trace coverage RED above is source-closed. The local fixture now
owns an `events` map for every one of the 34 record rows; it pins each ordered
`Begin(base,count)` and each `Entry(offset,length)`. Repeated entries use a
fixture-owned compact `(offset,length,count,stride)` form, expanded identically
by the Rust helper and independent Node model. The schema fixes the exact 34
keys, and the Node oracle explicitly rejects a fixture whose event-key set
does not equal the case-ID set.

Current Rust source collects every event before conditional acknowledgement
and compares the expanded trace after each grant run
([record cursor](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧾️record/🦀️.rs:158-197)); the Node model independently records and compares the
same facts ([host script](../../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:532-570)). Coordinator-reported source session `38374` covers 11
accepted and 23 denied rows at all three grants. It is not a Rust/native
terminal. The cursor/index/full dictionary-owner mount, aggregate
input/index/scratch retirement, semantic resolution, and publication
nonclaims remain unchanged.

## 2026-09-04 staged full dictionary owner — source review

The earlier “no dictionary Rust owner” observation is superseded: an
**unmounted, unexecuted** owner now exists at
`open/📜️history/🗂️dictionary/🦀️.rs`. The parent
`open/📜️history/🦀️.rs` still does not mount it, so its Rust tests are not
evidence and no `MemberFactory`, typed decode, store, or publication path is
accepted.

For its deliberately private source boundary, the sequencing is sound:

- `begin` retains the sole `VerifiedMemberHistoryInput` before scanner/index
  allocation and returns that exact input in its admission error
  (`dictionary/🦀️.rs:93-106`). `check` revalidates the retained request's
  operation, generation, cancellation, and expiry before every unit and
  before the only handoff (`:115-130`, `:248-253`).
- Every dictionary delta remains private in `RetainedDictionaryIndex` until
  exact record EOF; a rejected owner latches its index and makes lookup
  unavailable. Per-record publication before the final semantic record is
  acceptable only because the index has no external reader and `take_ready`
  is unavailable after any later denial (`:138-146`, `:215-240`; index
  `📇️index/🦀️.rs:50-90`). It is not a durable/public dictionary publication.
- The owner uses the existing tagged-ID cursor and its non-mutating
  `is_complete()` query; it resolves only the exact pending index through its
  private range index, feeds copied bytes one at a time, then closes the
  cursor before proceeding (`dictionary/🦀️.rs:148-180`). It does not
  duplicate tag/LEB/UTF-8 grammar.
- A duplicate document record is rejected, every earlier composition overlay
  is parsed, and the final composition match is selected last-wins. The
  owner requires both a matching document and matching final composition
  before its committed span may hand off (`:188-240`). This preserves the
  existing semantic rule rather than treating a later valid overlay as a
  reason to skip an earlier malformed one.
- Close retains input, index pages, pending wire/dictionary byte, UTF-8
  scratch, and tagged-ID output until their individual bounded terminal steps
  (`:257-302`). The output witness exposes no request/span extraction method;
  it only owns the retained authorities for its successor/close.

### Remaining integration RED — schema source is not authority-bound

`MemberHistoryDictionaryOwner::begin` currently receives `schema: &'static
str` independently of `VerifiedMemberHistoryInput` (`dictionary/🦀️.rs:93-98`).
`MemberOpenRequest` proves the expected full `ArtifactRef` and optional
`OwnerRef`, but carries no schema field (`open/🦀️.rs:42-95`). The semantic
decoder correctly compares record schema to that argument
(`dictionary/🛂️identity/🦀️.rs:63-89`), yet a future caller can still choose
which static schema becomes the comparison authority.

Before mounting a consumer, create one crate-private expected-member identity
from the selected closed `MemberFactory`/declaration: exact expected
`ArtifactRef`, optional exact `OwnerRef`, and that factory's declared schema.
Pass that single construction into the owner. Do not accept a route, UI, or
generic caller string; do not infer schema from a dialect. Add a hostile
factory-bound native law whose expected reference/owner is valid but whose
factory schema differs from the history schema, and require rejection before
typed snapshot decode. This is a source integration RED, not a defect in the
current unmounted parser/owner mechanics.

### Evidence status

The staged tests enumerate 33 history rows, 11 lifecycle rows, and seven
record-scratch traces under grants 1/7/4096
(`dictionary/🧪️tests/🦀️.rs:118-201`; fixture `:12-68`), including one-use
handoff and exact close totals. They have not run in this audit because the
module remains unmounted and Cargo is intentionally reserved for other lanes.
They must be attached to a focused exact-one native gate only after the
factory-bound schema construction exists. This review grants source design
credit only; it grants no native, route, D0, or end-to-end acceptance.

## 2026-09-04 owner-retirement corpus reread — native binding RED

Coordinator-reported source terminal `50966` completed the independent
33-history, 11-lifecycle, and seven record-scratch traces under grants 1/7/
4096, then stopped at the intentionally missing non-mutating
`RetainedHistoryIdV1::is_complete()` requirement. That is useful neutral-model
evidence only. The owner still cannot compile or run, is still unmounted, and
does not gain native credit.

The newly staged `ownerRetirement` fixture/schema adds nine exact
pending-wire, framing, dictionary-lookup, partially fed ID, complete-ID,
ID-retire, and raw-ID traces with fixture-pinned pending/lookup/ID bytes and
terminal totals (`dictionary/🧫️fixture/🔣️.json:56-65`; schema `:37-48`).
The independent host model tracks these three retained scratch buckets,
halts its scan after cancellation, and retires them before index and request
authority (`host/📜️script.ts:778-862`). This correctly extends the *source
model* beyond generic cancellation.

Two current source defects prevent those neutral rows from becoming a truthful
native law:

1. The two raw-ID rows name stage `id-raw`, but the production owner never
   assigns that transition. Raw `RetainedHistoryIdV1::push_wire` work occurs
   through the generic `payload` transition
   (`dictionary/🦀️.rs:199-211`); only `id-begin`, `id-lookup`, `id-copy`,
   `id-feed`, `id-complete`, and `id-retire` exist elsewhere. A native stage
   scanner would never reach `id-raw`. Bind the law to a direct private
   retained-ID length/stage predicate, or introduce an exact production
   transition before `push_wire`; do not preserve a model-only event label.
2. The host model constructs an actual raw document identity payload for
   `raw-document` (`host/📜️script.ts:638-640`), but the Rust production
   `history()` writer helper has no `raw-document` branch
   (`dictionary/🧪️tests/🦀️.rs:18-46`). It therefore emits the normal
   dictionary-backed `docHex` record for both raw-ID fixture rows. Add the
   equivalent first-party writer payload before selecting those rows; a
   native test using the current helper would not exercise raw-ID admission or
   retirement.

These defects have been sent directly to the provider before it adds the
source-required `fixture["ownerRetirement"]` reference. They are confined to
the staged test binding; the private owner’s request/input/index retention and
no-shadow-parser boundary remain source-sound. The already-recorded
factory-bound schema authority requirement remains an independent integration
RED, and no semantic typed decode/factory/open/publication claim is made.

## 2026-09-04 owner-retirement binding repair — source PASS, native pending

The two binding defects above are superseded in the current staged bytes.
`history()` now emits the same first-party raw document record used by the
neutral model—format `1`, raw-ID tag `0`, canonical protocol varint length,
raw bytes, then the schema dictionary reference—before passing it through the
production `SprWriter` (`dictionary/🧪️tests/🦀️.rs:30-34`). The raw-ID fixture
does not invent a transition: its native predicate selects the actual generic
`payload` state only after the retained ID exists, no dictionary lookup is
live, the active record is a document, and its scanner has crossed the raw
ID preamble (`:220-224`). Its two rows cover ASCII and a partial multibyte
UTF-8 ID (`dictionary/🧫️fixture/🔣️.json:66-67`).

The normal trace now pins all fourteen production-owner publication states
and all ten actual dictionary ranges under a one-unit grant; Rust obtains
those directly from the owner/index and compares the fixture, while the
independent host model does the same (`🧪️tests/🦀️.rs:118-152`; host
`📜️script.ts:682-683,811`; fixture `:10-11`). The cancellation rows now
assert the exact scanner offset and fuel are unchanged after cancellation,
repeat the denied step for stickiness, and account for retained ID bytes as a
separate close component (`🧪️tests/🦀️.rs:216-245`). That is a truthful
source-level owner/lifecycle binding rather than a model-only trace.

The owner still calls the intentionally absent non-mutating
`RetainedHistoryIdV1::is_complete()` (`dictionary/🦀️.rs:153-156`), so neither
native compilation nor a focused runtime law is available. The module also
remains unmounted. The script’s prospective native-mode assertion correctly
requires `mod dictionary;`, but its failure message says "remains unmounted"
despite testing the opposite condition (`host/📜️script.ts:877`); fix that
diagnostic before treating a later gate failure as evidence. This is gate
hygiene only. The factory-bound schema-authority integration RED remains
unchanged.

### Identity scope and pin nonclaim

At this retained layer, document identity is checked against the admitted
child `ArtifactRef`, and the last composition overlay checks the exact owner
triple plus child dialect (`dictionary/🛂️identity/🦀️.rs:63-89`). Earlier
overlays are still parsed and bounded but only the final overlay determines
the accepted ownership/dialect, matching the established last-wins protocol.
That is correct for the staged parser.

Checkpoint and child-pin identifiers are deliberately only grammar-checked
and counted here: their fields return `true` in `accept_id` (`:78`), while
the full history hydrator later parses pin artifact URIs and applies
`validate_composition_pins` (`store/🦀️.rs:10899-10915`). Thus this private
witness must not be described as proving pin membership, checkpoint
existence, or cross-document authorization. Likewise neither
`MemberOpenRequest` nor `ArtifactRef` carries a space identity
(`open/🦀️.rs:42-95`); the future closed selected-factory expected-member
identity must be constructed inside an already authorized document/space
scope. Add its native hostile cases for a foreign scope and for a syntactically
valid but non-admissible pin at the later typed-hydration boundary. No such
scope/pin authority is currently claimed or granted by this source-only
packet.

## 2026-09-04 composition-frame criticality — RED before selected-factory rerun

The selected-history owner currently rejects an ordinary, producer-generated
composition history. This is a source-contract mismatch, not a recovery or
authority exception.

`REC_COMPOSITION` is caller-defined extension kind `0x41`, deliberately
non-critical so older readers can skip it
([history codec](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:1210)).
`encode_history` consequently writes the composition frame with
`critical = false`, while it writes document and dictionary frames as critical
([producer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:1486),
[composition](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:1538)).
For codec zero, the flag byte is exactly `2` when critical and `0` when
non-critical: the bit layout is defined by the shared wire contract
([flags](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs:148)).

In contrast, the dictionary owner treats kinds `1` (document), `3`
(dictionary), and `65` (composition) uniformly and requires flag `2`
([owner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:313)).
The handwritten neutral Rust writer repeats that false premise by sending
`critical = true` for every record
([writer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧪️tests/🦀️.rs:86)).
The factory test's `semantic_history()` instead invokes the real producer,
which correctly exposes the failure
([factory witness](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🧪️tests/🦀️.rs:307)).

Required closed repair:

- Require exact `flags == 2` for kinds `1` and `3`, and exact `flags == 0`
  for kind `65`; do not accept both states for composition.
- Make the handcrafted neutral writer select its critical boolean from the
  record kind, so successful neutral rows use the producer's byte contract.
- Add one CRC-valid hostile history which changes only a composition frame's
  flag from `0` to `2` and prove the dictionary owner rejects it. Retain (or
  add) non-critical document/dictionary hostile rows. The test must rewrite
  the affected frame CRC so the rejection is attributed to the selected
  semantic profile rather than framing corruption.
- Rerun the existing selected-factory exact pair only after the real
  `encode_history` positive passes. This audit did not run native tests.

This repair does not loosen generic SPR framing, change unknown-extension
handling, or validate any selected-factory, space, pin-membership, typed
decode, or publication boundary.
