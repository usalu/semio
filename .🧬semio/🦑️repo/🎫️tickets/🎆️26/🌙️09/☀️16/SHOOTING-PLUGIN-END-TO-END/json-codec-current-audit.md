# I-JSON Codec Current Audit

Independent read-only review of seven json-prerequisite.md files and existing tagged-value/pack decoder owners. No source edits, tests, builds or extra queue. Native owns73126 JSON lib validation. Review precedes actual result and is not a compiler pass.

## Production Scope

The local JsonIJsonMutation owner now explicitly mounts its own text/binary IO modules, satisfying orphan ownership without reusing unrestricted JSON mutation types or dropping the editor. All eight distinct variants and existing typed workers remain: set top level, upsert/remove member, rename, safe number, string, insert/remove array element. Existing package dependencies supply owned protocol/value/pack JSON/diagnostic APIs; no external runtime library or dependency change was introduced.

Text framing uses exact json-i-json-mutation prefix then owned JSON parsing with duplicate-member rejection and typed FromValue. The protocol declares version1 and eight stable tags1–8. Binary delegates to the existing codebase tagged_value_binary with mutation discriminator, which checks format1, parses tag varint, maps tag through the authored protocol and reconstructs the typed value. Its payload decoder uses decode_value_record_body_exact with existing default pack decode limits; exact decoding is the source mechanism for rejecting trailing body bytes. This is not a new unbounded asynchronous operation or alternate codec stack; the synchronous OpText/OpBinary signatures remain the established owned interface.

No concrete production codec defect was confirmed by this bounded review. Invalid typed fields ultimately depend on current FromValue contracts and lower pack decoder checks; full behavior must be exercised, not inferred solely from the wrapper. No semantic I-JSON acceptance rule was weakened: decode produces the typed mutation, and existing mutation application retains its specific clauses. New source paths are reported by native as at most179UTF-16 units and use relative include/module paths.

## Concrete Verification Limits Sent Native

The added neutral fixture covers eight verbs and the new test compares printed text semantic JSON through serde_json, which is a real independent text/value oracle. It also checks text roundtrip, malformed text, wrong prefix and duplicate JSON keys. Binary checks only exact two-byte format/tag header, then roundtrips production encoding/decoding through the same helper. Its body has no independently authored full expected bytes or independent encoder comparison in the current fixture. Shared encode/decode bugs can therefore escape. This does not yet satisfy independent full binary wire agreement.

The current test truncates the final byte and checks unknown tag, but has no explicit trailing-input, invalid-format, malformed-field, extra-field or all-truncation-cut cases. Existing exact pack decoding supports some rejection by construction; that source fact is not execution evidence for this new vocabulary. Native/root were sent these concrete test gaps, with no speculative production rewrite requested. Focused runtime results and stronger independent wire evidence remain required before claiming those properties proven.

## Other Acceptance Boundaries

A successful owning JSON lib run would validate this prerequisite only; the selected shooting native tests, icon-host tests, shell check, equality/control, route publication/activation and browser acceptance remain separate. Route manifest review is awaiting actual PATCH-DONE and is not conflated with codec acceptance.

## Strengthened Fixture Follow-Up

Current eight neutral cases now carry complete binaryHex vectors authored through a separate limited Node wire reference, per native owner report, without production encoder calls. The regression compares actual complete encode bytes and decodes those fixture bytes directly; it checks every truncation boundary, appended trailing byte and invalid format for every case. All eight leaf FromValue declarations now explicitly deny unknown fields, matching their existing closed schemas. Text unknown-field rejection was added. This materially closes the previous unconstrained binary-body oracle gap; actual test execution remains pending73126.

Source crosscheck confirms the vector format: current dynamic symbol collection ignores map keys, so keys are inline while short string values are sorted/interned. This corrects an earlier abbreviated symbolic description sent native, not the fixture vectors, which already used inline keys. Retained independent reference provenance was requested for source inspection without execution.

One remaining test-precision limitation was sent native: the standalone extra-record-field negative vector also has an empty removeMember map missing required path/key. Its rejection cannot by itself distinguish unknown-record rejection from missing-payload-field rejection. A valid baseline payload with a deliberate extra field would isolate that boundary. This is a verification limitation, not a confirmed production bug, and does not undo the strengthened full-byte expectations.

## Final Prepared Oracle Source Review

Read retained json-wire-oracle.cjs without executing it. It imports only system fs/assert, independently emits limited fixture shapes from literal wire tags, UTF-8 lengths, unsigned varints, sorted short-string value symbols and occurrence-ordered maps. It does not import/call the production encoder or decoder. Nine current cases cover the eight verbs plus explicit zero versus omitted optional upsert index. Current expected complete bytes and Rust fixture decode checks therefore have reviewable independent provenance. This is a limited reference for these authored shapes, not a general Pack implementation or proof of arbitrary values.

The rejectedBinary fixture was corrected to start from a valid removeMember frame, change its declared record count from1 to2, and append field2 dynamic-null. Required path/key remain present, so the rejection now isolates the extra-record boundary rather than missing required payload fields. The regression reads this neutral rejected vector. No new defect was found in the final prepared source. Actual owning73126 test result remains the runtime authority; none is claimed by this read-only review.
