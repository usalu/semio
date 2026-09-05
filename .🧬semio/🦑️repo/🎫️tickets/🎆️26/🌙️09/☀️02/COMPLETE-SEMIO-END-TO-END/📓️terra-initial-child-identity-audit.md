# Initial Child Coordinate Identity Audit

## Scope and evidence boundary

Read-only current-tree review of the staged neutral coordinate contract at `store/🧩️composition/🌱️initial/🪪️identity`. No Rust helper is mounted, no native test was run, and the active source target reported by the coordinator has not been treated as runtime evidence.

The identity fixture is included by the existing member-dialect source oracle through `🧩️composition/🪪️member-dialect/📜️script.ts:6-9`. That is source-oracle coverage only.

## Verdict

**SOURCE-PASS for a pure, stable coordinate-to-target derivation.** The staged contract has an unambiguous domain, fixed field order, byte-length framing, full BLAKE3-256 output, and correct text bounds. It explicitly makes no creation-authority claim.

**RED for authoring admission and for native parity.** The current generic coordinate schema accepts ordinal `1` for any slot and has no relation to a parent-declared slot, selected member factory, or immutable initial content commitment. It must not be exposed as a viewer or generic caller capability.

## Current source evidence

- `🌱️initial/🪪️identity/🧪️fixtures/🔣️.json:2-17` declares domain `semio.initial-child.v1`, prefix `initial-child-`, BLAKE3-256, eight ordered coordinate fields, `maximumFieldBytes: 256`, `maximumChildren: 64`, and `authority: none`.
- The field order is the full parent `ArtifactRef` coordinate, `slot`, then full child dialect: `parent.artifactId`, kind, standard, subset, slot, child kind, standard, subset (`fixtures:6-15`; asserted again at `📜️script.ts:14-18`). That correctly prevents an identity collision between otherwise-equal parent kinds, standards, subsets, slots, or child dialects.
- `📜️script.ts:21-34` encodes `UTF-8(domain) || 0x00 ||` eight `u32le byte_length || UTF-8(field)` frames `|| u32le ordinal`. The single literal domain terminator and length-prefixed fields make the byte stream unambiguous; no variable-width number is involved.
- `📜️script.ts:36-54` independently reconstructs the wire bytes through `Buffer`/`writeUInt32LE`, checks every fixture byte length and full expected target, and detects duplicate fixture targets. Seven fixed full-digest vectors include Unicode, changed parent standard, child subset, slot, and ordinal (`fixtures:19-130`).
- The JSON schema requires exactly eight nonempty strings and an integer ordinal in `0..63` (`🧬️schema/🔣️.json:5-26`). The runtime admission additionally requires well-formed JavaScript strings and UTF-8 byte length at most 256 (`📜️script.ts:20`); the fixture exercises empty, byte overflow, NUL/C0/DEL/C1 controls, an unpaired surrogate, and a 256-byte emoji boundary (`📜️script.ts:58-71`, `fixtures:132-165`).
- Surface id, initial pack, current graph, and checkpoint are expressly excluded and rejected as extra schema fields (`fixtures:167-173`, `📜️script.ts:68-70`). That is correct: mutable content and UI identity cannot be inputs to a stable child target.

## Material boundaries to retain

### 1. Coordinate derivation is not creation authority

The fixture's `authority: none` is correct. The Rust helper should accept typed inputs rather than `&[String; 8]`—an exact parent `ArtifactRef`, a declared slot, an exact child `ArtifactDialect`, and ordinal—so field transposition cannot become a production caller responsibility. Keep the helper crate-private to the author-side initial-composition receipt/request path.

Before deriving, the author path must authenticate the document and validate all of the following against the loaded parent declaration and selected closed member binding:

- the full parent reference;
- the exact declared slot name;
- `ChildSlotSpec.kind == child_dialect.artifact_kind`;
- the slot's cardinality and currently reserved ordinal; and
- the selected child dialect/factory.

`ChildSlotSpec` carries `name`, `kind`, and `many` only (`🧬️schema/🧩️composition/🦀️.rs:11-15`); its kind deliberately admits a dialect family rather than a precise standard/subset (`🔌️plugin/🦀️.rs:1167-1171`). The receipt path, not this hash helper, must perform that additional closed-binding validation.

Read-only `OpenExistingComposition` must take an already-persisted target/relation and never call an authoring API that creates a durable initial child. A surface id must remain outside both identity and authority.

### 2. Stable target needs a separate immutable-content commitment

Do not put an initial snapshot, pack, checkpoint, graph, or mutable content digest into this target identity. The child target must remain stable as its checkpoint changes.

Instead, the authenticated author-only creation receipt/event must atomically bind:

- this exact derived target;
- full parent reference, slot, child dialect, and ordinal;
- the canonical initial child pack or immutable initial checkpoint commitment; and
- the accepted member/factory identity.

A hostile receipt law must deny an otherwise-identical coordinate/target paired with substituted initial-pack or checkpoint bytes. The current pure identity fixture cannot prove this and should not be overloaded to pretend it does.

### 3. Ordinal is a receipt/cardinality input, not a universal slot property

The staged `ordinal-one` vector proves encoding distinction, but accepts ordinal `1` on `content` without a slot declaration (`fixtures:116-130`). That is fine for a pure coordinate fixture but unsafe as an admission corpus.

The schema composition model distinguishes singular and collection fields: `ChildSlotSpec.many` is the source-of-truth cardinality (`🧬️schema/🧩️composition/🦀️.rs:11-15`), and the plugin contract specifically allows multiple instances for the same `many` slot/kind (`🔌️plugin/🦀️.rs:1003-1005`). The creation packet must therefore add neutral rows proving:

- a singular slot accepts ordinal 0 and rejects ordinal 1;
- a declared collection slot accepts distinct ordinals 0 and 1;
- duplicate `(parent, slot, ordinal)` is denied before publication;
- an undeclared slot or wrong child kind is denied;
- the author-side seed cap rejects ordinal 64; and
- a collection's cardinality remains constrained by the explicit receipt/request cap, not silently by this generic identity schema.

`maximumChildren: 64` is currently a fixture constant (`fixtures:16-17`), not a schema-derived rule. It becomes honest only when the author receipt enforces it as a bounded initial-composition policy.

## Required native and neutral proof before helper admission

1. Add one exact Rust law over the same JSON fixture that constructs the byte frames using a first-party typed encoder and asserts raw bytes, `wireBytes`, and all expected full target strings. It must call the existing Rust `blake3` crate rather than the TypeScript port.
2. Keep the current AJV plus `DataView`/`Buffer` byte-oracle. It is independently encoded at the byte layer, but its digest currently comes from the repository's self-contained TypeScript BLAKE3 port (`📜️script.ts:5`, `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:452-458,657-662`), so it is not the native implementation parity proof.
3. Add a receipt-level neutral corpus separately from the pure hash fixture for author authorization, declared-slot/kind/cardinality validation, duplicate reservation, substituted initial-pack/checkpoint denial, atomic publication/no-publication-on-rejection, and viewer-open-without-genesis denial.
4. Register the source fixture through the existing member-dialect source target as it is now, then register the Rust law by an exact-one selector in the same bounded native gate. A source-target green result alone cannot establish helper, receipt, or runtime correctness.

## Acceptance nonclaims

This review does not accept a mounted Rust identity helper, authenticated initial-child creation, receipt persistence, graph publication, viewer restore, any document codec, or runtime launch. The staged artifact is a sound source-only coordinate contract contingent on the above authoring and native-parity packet.

## 2026-09-04 current-byte reread

Current source preserves the stated pure-contract PASS. The Rust helper's
`str::len()` makes its 256 limit a UTF-8 byte limit, matching the JavaScript
`Buffer.byteLength` guard rather than AJV's character-count constraint; both
also reject C0 and C1 controls and the browser oracle rejects malformed
surrogates. `semio_framework_hash::Hasher` uses the fixed domain and the test
uses direct `blake3::hash` as its independent native oracle. The parent input
is complete for the current `ArtifactRef` shape—artifact id plus the three
dialect fields—and the child dialect is likewise complete.

Reported source target `92798` is not treated as native parity: this helper is
still unmounted and no Rust terminal was run by this audit. The only retained
implementation requirement is type safety at its future author-only caller:
pass typed parent `ArtifactRef`, declared slot, child `ArtifactDialect`, and
receipt-reserved ordinal rather than letting callers assemble eight strings.
That requirement preserves the existing explicit nonauthority rather than
changing the digest contract.

## 2026-09-04 D0 multi-space identity correction

**RED — the eight-field derivation is not a D0-safe initial-child identity.**
It must be changed before its first native mount to hash an authenticated
`spaceId` as a ninth, separately length-prefixed field. The earlier
source-PASS remains true only for a scope-unspecified local coordinate
function; it is superseded for a hub document where equal document ids in two
spaces are permitted.

### Why the current eight fields do not structurally scope the target

- `ArtifactRef` contains only `artifact_id` and `dialect`
  (`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:170-193`). It has neither a
  `spaceId` nor a D0 scope type. `OwnerRef` similarly contains an
  `ArtifactRef`, slot, and child id only
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2935-2939`).
- The generic `CompositionGraph` keys ownership and links by bare strings
  (`🏪️store/🦀️.rs:18795-18798`); `owner_of`, cycle detection, and
  `admit_owns` all compare raw parent/child ids
  (`:18823-18826`, `:18835-18853`, `:18865-18875`). It therefore cannot
  separate two parents with equal artifact ids merely because their callers
  came from different spaces.
- The live plugin child admission constructs that same unscoped `OwnerRef`
  from `self.store.envelope().id` and commits the raw ids to that graph
  (`🔌️plugin/🦀️.rs:19949-19986`). No type or equality check there requires a
  hub `DocumentScope`.
- The hub *does* protect its flat database/fanout registry with
  `document_scope_key_v1 = v1:<space-byte-length>:<document-byte-length>:…`
  and uses it as `db_artifact_id` (`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:394-407`);
  its test asserts different keys for `space-a/shared` and `space-b/shared`
  (`:7938-7942`). Route and socket paths authenticate a `DocumentScope`
  before operating (`:1987-2005`, `:2129-2141`, `:2201-2206`). That is a
  correct hub boundary, but it is a convention at the DB/fanout caller. The
  staged generic hash accepts any `ArtifactRef.artifact_id`, including an
  unscoped raw document id, and has no evidence that a caller supplied the
  encoded DB key. It is not a substitute for scope in the initial identity.

### Required source-first correction

Define the identity wire as:

```
"semio.initial-child.v1\\0"
+ u32le(utf8Bytes(authenticatedScope.spaceId)) + authenticatedScope.spaceId
+ the existing eight u32le-length-prefixed fields, in their present order
+ u32le(ordinal)
```

The ninth field must be named `spaceId` in fixture/schema/typed API, be
nonempty, at most 256 UTF-8 bytes, and reject the same C0/C1 controls as the
other coordinate fields. `ArtifactRef` must remain the complete parent
artifact coordinate; no caller may derive the space from it. This is a
schema-only breaking correction because the module remains unmounted.

The future author-only `CreateInitialComposition` request obtains `spaceId`
from the already authenticated `DocumentScope` and binds it into the durable
request/receipt. It must not accept a UI/client-provided string as authority.
`OpenExistingComposition` receives an already persisted scoped relation and
never derives an id. The hub's encoded `db_artifact_id(scope)` remains useful
as a defence-in-depth storage key, but must not replace the explicit
authenticated field at the generic composition boundary.

### Exact neutral/native acceptance additions

1. Update the fixture/schema and Node `Buffer`/`DataView` independent framing
   oracle plus the existing first-party TypeScript `blake3Hex` reference to
   require nine fields and nine `u32le` byte frames. Add two rows
   that are identical in the old eight values and ordinal but use `space-a`
   and `space-b`; their full digests must differ. Mutate each of all nine
   fields independently and retain the exact full-digest checks.
2. Add empty, control-character, and over-256-byte `spaceId` denial rows.
   Keep the existing Unicode byte-boundary rows; character counts are not a
   valid substitute for UTF-8 byte lengths.
3. The first mounted Rust law must reconstruct the nine-frame byte stream
   with the first-party typed encoder and direct `blake3`, then prove two
   same-document-id scopes produce different initial targets *before* graph
   or store insertion. A separate receipt law must prove the authenticated
   `DocumentScope` and its hashed `spaceId` cannot disagree.
4. Only after that, an author-side integration law may create two equal raw
   document ids in separate spaces and verify each `OwnerRef`/graph resides
   in its own document-scoped factory. This is currently absent; no D0,
   graph, authoring, or viewer acceptance follows from the fixture.

## 2026-09-04 scope correction reread — source PASS, native pending

The required unmounted source correction is now present and closes the
multi-space collision in the pure identity contract. The helper accepts nine
fields (`identity/🦀️.rs:7-20`), and the fixture makes
`scope.spaceId` the first literal field before the full parent coordinate,
slot, and child dialect (`🧪️fixtures/🔣️.json:5-16`). The domain remains
unchanged, so every field is length-framed in its declared schema order:

```
UTF-8("semio.initial-child.v1") || 0x00
|| u32le(bytes(scope.spaceId)) || scope.spaceId
|| eight existing u32le(bytes(field)) || field frames
|| u32le(ordinal)
```

The pair `document-one` and `different-space-same-document` holds the same
raw document id, full parent dialect, slot, child dialect, and ordinal while
changing only `scope.spaceId`; their two expected full BLAKE3 targets differ
(`fixtures:21-52`). The Rust staged law explicitly checks this equality/delta
pair, performs all nine one-field substitutions, and runs all nine positions
through empty/control/over-byte-limit rejection (`identity/🦀️.rs:43-85`).
The JSON schema fixes exactly nine values (`🧬️schema/🔣️.json:11-26`).

The Node source oracle has also been correctly characterized: `DataView`
and `Buffer.writeUInt32LE` separately reconstruct the frame bytes
(`📜️script.ts:27-58`), while the digest uses the repository's existing
first-party `blake3Hex` implementation (`:5,25,58`) with an `abc` known-answer
check. **WebCrypto is not and cannot be claimed as a BLAKE3 oracle.** The
planned Rust `blake3::hash` law is the independent native hash parity proof.

Coordinator-reported source terminal `73019` completed eight vectors, 71
denials, and nine UTF-8 boundary frames after intentional missing-scope RED
`27574`. This audit did not run it. The helper and test remain unmounted, and
no native `blake3`, author receipt, authenticated scope-to-field binding,
factory/graph, viewer, D0, or runtime acceptance is implied. In particular,
the ninth value must still be populated only from authenticated
`DocumentScope.spaceId` at future author-only creation; the pure array helper
does not itself establish that authority.
