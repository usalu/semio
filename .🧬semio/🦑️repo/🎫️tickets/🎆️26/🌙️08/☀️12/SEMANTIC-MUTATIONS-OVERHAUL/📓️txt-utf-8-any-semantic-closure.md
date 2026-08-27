# TXT UTF-8 Any Semantic / Codec Closure

## Scope

This bounded closure covers only `stdio.txt` / UTF-8 / `✳️any` direct mutations:
`set-trailing-newline`, `set-line-ending`, `insert-line`, `remove-line`, and
`set-line`. No `compose/**` path was read or changed.

## Semantic Result

The five leaf owners now expose direct typed mutation payloads. The former
`Apply(payload) | Restore(TxtDiff)` phase carrier is absent from the active
TXT leaf mutations and inverse plans are ordinary, typed `TxtMutation` steps:

| Forward operation | Inverse plan |
| --- | --- |
| `set-trailing-newline` | set the prior trailing-newline value |
| `set-line-ending` | set the prior line ending |
| `insert-line` | remove at the normalized/clamped insertion index |
| `remove-line` | insert the exact removed line at its original index |
| `set-line` | set the exact prior text at its original index |

Forward mutations validate resulting canonical line shape before an inverse is
emitted. Empty, out-of-range, and non-representable cases are no-op/refusal
plans rather than panics. In particular, `insert-line` uses the same clamped
index in its diff and inverse.

## Codec / Schema Result

The Rust and TypeScript roots use the canonical direct envelope
`{ mutation: <kebab-kind>, payload: <leaf payload> }`; the JSON Schema has a
five-way discriminator `oneOf`, protobuf retains its five typed `oneof`
members, and GraphQL has a typed five-value tag plus one typed leaf payload
field. The GraphQL input convention follows the repository's standard
discriminated-input representation because GraphQL has no input unions.

Root text and binary codecs now provide framing only. Their executable
registries are made from leaf opcode/tag constants and leaf-owned
`try_encode`/`decode_mutation` callbacks; the root no longer has per-variant
match dispatch or whole-enum serde encode/decode. The text hex decoder now
uses a checked lowercase-hex match, so punctuation cannot trigger unsigned
subtraction before rejection.

## Direct Writes

- Five direct leaf Rust mutation components plus their ten leaf text/binary
  codec components.
- TXT mutation root Rust, TypeScript, JSON Schema, GraphQL, protobuf, text
  grammar/ANTLR mirrors, and binary protocol/ABNF/Kaitai/Spicy mirrors.
- TXT mutation root tests, TXT editor consumer, UTF-8 Any oracle mutation
  catalog/logic, and the mounted `mutate-txt-utf-8` subject adapter.
- The TXT diff fallback helper that reconstructed an arbitrary snapshot was
  removed. The shared checkout already contains the removal of the obsolete
  `set-snapshot` leaf tree; this report does not attribute that concurrent
  removal to this lane.

## Verification Record

Red state established by the governing audit: direct TXT leaves used
`Restore(TxtDiff)`, root carriers took a whole-enum serde path, and the
mounted subject inverse called the oracle inverse specification.

Green commands run:

```text
rustfmt --edition 2024 --check <all TXT mutation Rust components + adapter/oracle/editor>
exit 0

bun -e '<Ajv validates five literal direct envelopes; rejects an incomplete set-line payload; checks root leaf registries>'
[DEBUG] TXT isolated Ajv+registry runtime active vectors=5 invalidRejected=true rootLeafRegistries=true

bun -e '<checks all five leaves expose ten callbacks and no active aggregate inverse carrier remains>'
[DEBUG] TXT isolated carrier scan leaves=5 leafCallbacks=10 aggregateInverseCarrier=false
```

The added production Rust tests exercise each inverse's root/direct text and
binary round trips, exact snapshot restoration, direct emitted payload shape,
and malformed text/binary frames (odd hex, punctuation, invalid UTF-8,
unknown opcode/tag, and malformed leaf binary payload).

Deferred by the root-owned Cargo serialization: no Cargo build/test or Nx
target was run here. Consequently Rust name resolution and the actual compiled
production codec/inverse runtime remain to be executed in the authorized
serial lane. The existing `csv` oracle only independently covers a narrow
line-boundary subset and cannot validate blank-line/full-line-ending semantics;
Ajv was the available third-party runtime used here to validate all five JSON
payload schemas. An attempted GraphQL parse was not run because the `graphql`
package is not installed in this workspace.

## Review Focus

Run the authorized TXT production compile/test next, then inspect the five
literal inverse vectors and malformed-frame test in
`🧬️schema/🧬️mutations/🦀️component.rs`. The remaining blocker is verification
rather than an intentionally retained semantic or codec carrier.

## Freeze-Time Read-Only Findings

The root-requested carrier probe found a real semantic hole in the frozen
source. `TxtSnapshot::from_body` treats a standalone `\r` as ordinary line
content and round-trips it exactly. It treats LF and CRLF as separators. The
direct `insert-line` and `set-line` payloads currently accept `text` containing
LF or CRLF, while `non_canonical_shape` only checks line count, final-empty
line, and trailing-terminator state. Therefore these accepted payloads can
lose line identity when their snapshot is rendered then parsed again:

```text
[DEBUG] TXT carrier probe bareCrContent=true embeddedLfProjectionStable=false embeddedCrlfProjectionStable=false lfBody="a\nb\nc" crlfBody="a\nb\r\nc"
```

Initial correction finding: `insert-line` and `set-line` need semantic
validation before constructing a diff. The later style-dependent probe below
supersedes this finding's initial suggestion of an unconditional JSON Schema
ban. Standalone CR remains ordinary content under the present snapshot domain
unless it is immediately followed by a structural LF separator, which creates
a detected CRLF boundary.

There is also strict-schema divergence. Every leaf JSON Schema has
`additionalProperties: false`, and Ajv rejects an extra property, but the five
Rust payload structs derive `Deserialize` without `deny_unknown_fields`.
Serde will therefore accept unknown JSON payload members that the published
schema rejects. The post-gate correction is to add `#[serde(deny_unknown_fields)]`
to all five direct payload structs and cover the rejection with a production
codec test.

```text
[DEBUG] TXT schema unknown payload field rejected=true rustDenyUnknownFields=false (source inspection)
```

Region inventory (read-only): the direct mutation root, its root text/binary
codecs, all five leaf mutation components, and all ten leaf text/binary codec
components have zero region markers. The pre-existing diff, editor, and oracle
components have markers. The direct owner set therefore needs regions and
subregions added during the narrowly authorized correction.

The third-party situation remains limited: Ajv validates the JSON contract;
the existing CSV evidence is only a partial line-boundary comparison and does
not independently validate all five mutation semantics. This is an evidence
gap, not a passing semantic oracle claim.

### Line-Ending Visibility Invariant

`lineEnding: crLf` is representable by the native text carrier only when the
body contains at least one visible CRLF separator: either more than one line,
or a non-empty line collection with `trailingNewline: true`. `from_body`
otherwise has no CRLF evidence and returns `lf`.

```text
[DEBUG] TXT ending visibility one-line-crlf stable=false body="a"
[DEBUG] TXT ending visibility empty-crlf stable=false body=""
[DEBUG] TXT ending visibility one-line-visible-crlf stable=true body="a\r\n"
[DEBUG] TXT ending visibility two-lines-crlf stable=true body="a\r\nb"
```

This makes the current positive root vector
`["a"], trailingNewline=false, lf → set-line-ending(crLf)` incorrect: it must
become a typed refusal/no-inverse vector. The same visibility check must guard
every direct operation whose resulting state can preserve or select `crLf`:
`set-line-ending`, `set-trailing-newline` when removing the only visible
terminator, `insert-line` into an empty CRLF snapshot, and `remove-line` when
it removes the last visible inter-line separator. A `set-line` applied to an
already unrepresentable manually constructed snapshot must refuse rather than
claiming a native-carrier identity. Normal imported snapshots start in the
representable domain, but the mutation boundary must preserve that invariant.

### Style-Dependent Embedded-Control Domain (Supersedes Blanket LF Rejection)

The initial correction note above was deliberately conservative. A further
probe of the real `from_body` separator-detection algorithm shows that a
blanket ban on LF would incorrectly narrow supported CRLF snapshots. The
precise domain is:

- CRLF snapshots may contain bare LF and bare CR in line content, but no line
  content may contain the full CRLF separator.
- LF snapshots may not contain LF in line content. Bare CR is valid only when
  it is in the final line of an unterminated document; a bare CR immediately
  before a structural LF makes the rendered body look like CRLF and changes
  the detected style.

```text
[DEBUG] TXT style-domain crlf-bare-lf-content stable=true body="a\nb\r\nc"
[DEBUG] TXT style-domain crlf-bare-cr-before-separator stable=true body="a\r\r\nb"
[DEBUG] TXT style-domain crlf-embedded-crlf stable=false body="a\r\nb\r\n"
[DEBUG] TXT style-domain lf-bare-cr-final-unterminated stable=true body="a\r"
[DEBUG] TXT style-domain lf-bare-cr-before-delimiter stable=false body="a\r\nb"
[DEBUG] TXT style-domain lf-bare-cr-before-trailing stable=false body="a\r\n"
[DEBUG] TXT style-domain lf-embedded-lf stable=false body="a\nb"
```

Post-gate implementation must use this style-dependent native-roundtrip
predicate for baseline and result validation. JSON Schema can state only the
syntax of a payload independent of its base snapshot; it must not add an
unconditional LF pattern. The `diff` methods perform the context-dependent
semantic refusal.

## Post-Release Correction

The queued correction is now applied without a Cargo invocation:

- `mutation_support` has a shared style-dependent native-carrier predicate for
  shape, CRLF visibility, and the precise LF/CRLF content rules above.
- Every leaf validates its baseline and exact result before producing a diff
  or inverse. `remove-line` computes the resulting final-line shape directly,
  without cloning the line vector.
- All five payload structs now use serde `deny_unknown_fields`, matching the
  JSON Schema's `additionalProperties: false` behavior; the tagged root enum
  is strict too, so extra envelope members are not accepted by Rust alone.
- Neutral vectors now use a visible two-line LF→CRLF transition; refusal
  vectors cover invisible CRLF, loss of the last CRLF separator, LF/CRLF
  content hazards, while positives retain legal CRLF bare-LF content and LF
  final bare-CR content.
- The direct root, five leaves, and ten leaf codecs now have region markers.

Additional post-correction non-Cargo evidence:

```text
rustfmt --edition 2024 --check <TXT mutation root, leaves, leaf codecs, support>
exit 0

[DEBUG] TXT direct owner structure leaves=5 strictSerde=5 leafCallbacks=10 regions=true
[DEBUG] TXT Ajv direct-schema vectors=5 contextSyntaxAccepted=true unknownRejected=true
[DEBUG] TXT independent native-domain probe cases=5 exactPredicate=true
[DEBUG] TXT strict envelope source parity rootDenyUnknownFields=true
[DEBUG] TXT post-correction source shape leaves=5 strict=6 regions=18
```

The Ajv result intentionally accepts syntactically valid embedded control
content: whether it is semantically admissible depends on the current
snapshot's line ending and structural position, and is enforced by the
production `diff` predicate. Cargo/Nx remains unrun by this lane; a fresh
registered production gate after this correction is still required.

## Follow-Up: Leaf Test Ownership and One-Line Removal

The one-line `remove-line` result previously computed `len - 2` before bounds
checking. It now uses `checked_sub(2)` followed by `get`, so removing the only
LF line produces the native empty snapshot and a typed insert inverse without
underflow. Its owning leaf test now verifies forward and inverse
`to_body`/`from_body` equality as well as root text and binary inverse codec
round trips.

Concrete vectors, refusal cases, direct envelopes, strict-field cases, and
concrete malformed payload frames have moved from the aggregate mutation test
module to their direct leaf owners. The aggregate root retains only the
structural five-kind roster assertion. Generic malformed framing tests reside
in the root text/binary codec facets. The shared representability helper has a
432-case product matrix that compares its predicate directly with the real
production `TxtSnapshot::to_body` then `from_body` carrier, covering empty,
single, and two-line vectors across both line endings, both trailing states,
and embedded LF/CR boundary text.

## Scoped Production Runtime Checkpoint

Root ran the retained ticket-only actual-source harness after the final source
fingerprint. It mounted the current TXT snapshot/diff/mutation/codec sources
against the already-built checkpoint kernel and serde dependencies; it did not
copy an implementation. Result: **28/28 production TXT tests passed**,
including the 432-case native carrier matrix, direct root/leaf codecs, and the
one-line removal inverse. The before/after source SHA was identical:

```text
a34d91f9c19f9cd57b33b82ce7e6f81b4444b82c4ed6f0101637182d06683dc4
```

Evidence: `🧪️txt-production-leaf-runtime/🧪️metadata-retry.log` in this ticket.
The first harness attempt had five metadata-stub errors because the invocation
omitted paired rlib+rmeta dependency flags; the retry used Cargo's paired flags
and passed. This is scoped production runtime proof, **not** a registered full
STDIO/Nx acceptance result. No Cargo command was run by this lane.
