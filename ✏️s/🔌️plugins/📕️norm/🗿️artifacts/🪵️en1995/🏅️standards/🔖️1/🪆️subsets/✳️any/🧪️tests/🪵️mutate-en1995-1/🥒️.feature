@capability-en1995-1-mutate
@oracle-en1995-1-python-independent
@comparison-ordered-json-v1
@mutations-en1995-1-any
Feature: Apply every typed EN 1995 mutation against an independent Python implementation
  `s.norm.en1995` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 66 `✅apply` vectors
  cover every kind of the current vocabulary (58 `change`, 4 `insert`, 4 `remove`) on a glued-laminated timber floor beam; each vector's after-snapshot and
  diff were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same
  laws in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list
  position included. `parity` adds that two implementations, in two languages, reach the same document.

  The 2 refusal rows (`⛔dupe`) re-apply a kind's applied mutation to the
  after-snapshot it produced: re-inserting an id the collection now holds must be refused `mutation.duplicate-id`
  (Fatal), re-removing a member that is gone `mutation.target-missing` (Error) and re-setting a value the document
  already has must report `mutation.no-op` (Warning). Both sides must refuse under the committed code and leave the
  document bit-identical; a refusal has nothing to undo, so these rows are `mutate-` only.

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://🏠️glulam-floor-beam/🏠️glulam-floor-beam/🗣️.dsl.semio`. The carrier has no published grammar: the committed
  `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so the two sides are compared
  at the envelope preamble, the ordered lines and the digest and length of what each re-emitted. The Rust side additionally proves it PARSED the document: the committed binary
  twin must decode to the same document as the text, through a separately written codec.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id                               | dir                                | fixture |
      | change-annex                     | 🌍️change-annex                     | ✅apply  |
      | insert-member                    | ➕️insert-member                    | ✅apply  |
      | insert-member-dupe               | ➕️insert-member                    | ⛔dupe   |
      | remove-member                    | ➖️remove-member                    | ✅apply  |
      | change-member-label-en           | 🏷️change-member-label-en           | ✅apply  |
      | change-member-label-de           | 🏷️change-member-label-de           | ✅apply  |
      | change-member-role               | 🎯️change-member-role               | ✅apply  |
      | change-member-strength-class     | 🛡️change-member-strength-class     | ✅apply  |
      | change-member-service-class      | 🌧️change-member-service-class      | ✅apply  |
      | change-member-support            | 📍️change-member-support            | ✅apply  |
      | change-member-b                  | ↔️change-member-b                  | ✅apply  |
      | change-member-h                  | ↕️change-member-h                  | ✅apply  |
      | change-member-span               | ↔️change-member-span               | ✅apply  |
      | change-member-support-length     | ↔️change-member-support-length     | ✅apply  |
      | change-member-bearing-length     | ↔️change-member-bearing-length     | ✅apply  |
      | change-member-buckling-length-y         | ↔️change-member-buckling-length-y         | ✅apply  |
      | change-member-buckling-length-z         | ↔️change-member-buckling-length-z         | ✅apply  |
      | change-member-restraint-spacing  | ↔️change-member-restraint-spacing  | ✅apply  |
      | change-member-notch-depth        | ↔️change-member-notch-depth        | ✅apply  |
      | change-member-notch-distance     | ↔️change-member-notch-distance     | ✅apply  |
      | change-member-m-crit             | ⚠️change-member-m-crit             | ✅apply  |
      | change-member-mass-kg-per-m         | ⚖️change-member-mass-kg-per-m         | ✅apply  |
      | change-member-mass-kg-per-m2        | ⚖️change-member-mass-kg-per-m2        | ✅apply  |
      | change-member-damping-xi            | 🌊️change-member-damping-xi            | ✅apply  |
      | change-member-fire-duration      | 🔥️change-member-fire-duration      | ✅apply  |
      | change-member-bridge-n-obs       | 🌉️change-member-bridge-n-obs       | ✅apply  |
      | change-member-bridge-tl-years    | 🌉️change-member-bridge-tl-years    | ✅apply  |
      | change-member-bridge-beta        | 🌉️change-member-bridge-beta        | ✅apply  |
      | change-member-bridge-a           | 🌉️change-member-bridge-a           | ✅apply  |
      | change-member-bridge-b           | 🌉️change-member-bridge-b           | ✅apply  |
      | change-member-bridge-crowd       | 🚶️change-member-bridge-crowd       | ✅apply  |
      | insert-member-action             | ➕️insert-member-action             | ✅apply  |
      | remove-member-action             | ➖️remove-member-action             | ✅apply  |
      | change-member-action-kind        | ⚖️change-member-action-kind        | ✅apply  |
      | change-member-action-category    | 🏢️change-member-action-category    | ✅apply  |
      | change-member-load-duration      | ⏳️change-member-load-duration      | ✅apply  |
      | change-member-action-q-line      | ⬇️change-member-action-q-line      | ✅apply  |
      | change-member-action-f-point     | ⬇️change-member-action-f-point     | ✅apply  |
      | change-member-action-mk          | ⤴️change-member-action-mk          | ✅apply  |
      | change-member-action-vk          | ↕️change-member-action-vk          | ✅apply  |
      | change-member-action-nk          | 🏋️change-member-action-nk          | ✅apply  |
      | change-member-action-ntk         | 🏋️change-member-action-ntk         | ✅apply  |
      | change-member-action-fc90-k      | 🏋️change-member-action-fc90-k      | ✅apply  |
      | insert-connection                | ➕️insert-connection                | ✅apply  |
      | insert-connection-dupe           | ➕️insert-connection                | ⛔dupe   |
      | remove-connection                | ➖️remove-connection                | ✅apply  |
      | change-connection-label-en       | 🏷️change-connection-label-en       | ✅apply  |
      | change-connection-label-de       | 🏷️change-connection-label-de       | ✅apply  |
      | change-connection-fastener-type  | 🔩️change-connection-fastener-type  | ✅apply  |
      | change-connection-strength-class | 🛡️change-connection-strength-class | ✅apply  |
      | change-connection-service-class  | 🌧️change-connection-service-class  | ✅apply  |
      | change-connection-diameter       | ↔️change-connection-diameter       | ✅apply  |
      | change-connection-number         | 🔢️change-connection-number         | ✅apply  |
      | change-connection-rows           | 🔢️change-connection-rows           | ✅apply  |
      | change-connection-spacing        | ↔️change-connection-spacing        | ✅apply  |
      | change-connection-edge-distance  | ↔️change-connection-edge-distance  | ✅apply  |
      | change-connection-end-distance   | ↔️change-connection-end-distance   | ✅apply  |
      | change-connection-t1             | ↔️change-connection-t1             | ✅apply  |
      | change-connection-t2             | ↔️change-connection-t2             | ✅apply  |
      | change-connection-steel-plate    | 🔩️change-connection-steel-plate    | ✅apply  |
      | change-connection-plate-thickness        | ↔️change-connection-plate-thickness        | ✅apply  |
      | change-connection-shear-planes   | 🔢️change-connection-shear-planes   | ✅apply  |
      | change-connection-fuk            | 🛡️change-connection-fuk            | ✅apply  |
      | insert-connection-action         | ➕️insert-connection-action         | ✅apply  |
      | remove-connection-action         | ➖️remove-connection-action         | ✅apply  |
      | change-connection-action-kind    | ⚖️change-connection-action-kind    | ✅apply  |
      | change-connection-load-duration  | ⏳️change-connection-load-duration  | ✅apply  |
      | change-connection-action-fk      | 🔩️change-connection-action-fk      | ✅apply  |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id                               | dir                                | fixture |
      | change-annex                     | 🌍️change-annex                     | ✅apply  |
      | insert-member                    | ➕️insert-member                    | ✅apply  |
      | remove-member                    | ➖️remove-member                    | ✅apply  |
      | change-member-label-en           | 🏷️change-member-label-en           | ✅apply  |
      | change-member-label-de           | 🏷️change-member-label-de           | ✅apply  |
      | change-member-role               | 🎯️change-member-role               | ✅apply  |
      | change-member-strength-class     | 🛡️change-member-strength-class     | ✅apply  |
      | change-member-service-class      | 🌧️change-member-service-class      | ✅apply  |
      | change-member-support            | 📍️change-member-support            | ✅apply  |
      | change-member-b                  | ↔️change-member-b                  | ✅apply  |
      | change-member-h                  | ↕️change-member-h                  | ✅apply  |
      | change-member-span               | ↔️change-member-span               | ✅apply  |
      | change-member-support-length     | ↔️change-member-support-length     | ✅apply  |
      | change-member-bearing-length     | ↔️change-member-bearing-length     | ✅apply  |
      | change-member-buckling-length-y         | ↔️change-member-buckling-length-y         | ✅apply  |
      | change-member-buckling-length-z         | ↔️change-member-buckling-length-z         | ✅apply  |
      | change-member-restraint-spacing  | ↔️change-member-restraint-spacing  | ✅apply  |
      | change-member-notch-depth        | ↔️change-member-notch-depth        | ✅apply  |
      | change-member-notch-distance     | ↔️change-member-notch-distance     | ✅apply  |
      | change-member-m-crit             | ⚠️change-member-m-crit             | ✅apply  |
      | change-member-mass-kg-per-m         | ⚖️change-member-mass-kg-per-m         | ✅apply  |
      | change-member-mass-kg-per-m2        | ⚖️change-member-mass-kg-per-m2        | ✅apply  |
      | change-member-damping-xi            | 🌊️change-member-damping-xi            | ✅apply  |
      | change-member-fire-duration      | 🔥️change-member-fire-duration      | ✅apply  |
      | change-member-bridge-n-obs       | 🌉️change-member-bridge-n-obs       | ✅apply  |
      | change-member-bridge-tl-years    | 🌉️change-member-bridge-tl-years    | ✅apply  |
      | change-member-bridge-beta        | 🌉️change-member-bridge-beta        | ✅apply  |
      | change-member-bridge-a           | 🌉️change-member-bridge-a           | ✅apply  |
      | change-member-bridge-b           | 🌉️change-member-bridge-b           | ✅apply  |
      | change-member-bridge-crowd       | 🚶️change-member-bridge-crowd       | ✅apply  |
      | insert-member-action             | ➕️insert-member-action             | ✅apply  |
      | remove-member-action             | ➖️remove-member-action             | ✅apply  |
      | change-member-action-kind        | ⚖️change-member-action-kind        | ✅apply  |
      | change-member-action-category    | 🏢️change-member-action-category    | ✅apply  |
      | change-member-load-duration      | ⏳️change-member-load-duration      | ✅apply  |
      | change-member-action-q-line      | ⬇️change-member-action-q-line      | ✅apply  |
      | change-member-action-f-point     | ⬇️change-member-action-f-point     | ✅apply  |
      | change-member-action-mk          | ⤴️change-member-action-mk          | ✅apply  |
      | change-member-action-vk          | ↕️change-member-action-vk          | ✅apply  |
      | change-member-action-nk          | 🏋️change-member-action-nk          | ✅apply  |
      | change-member-action-ntk         | 🏋️change-member-action-ntk         | ✅apply  |
      | change-member-action-fc90-k      | 🏋️change-member-action-fc90-k      | ✅apply  |
      | insert-connection                | ➕️insert-connection                | ✅apply  |
      | remove-connection                | ➖️remove-connection                | ✅apply  |
      | change-connection-label-en       | 🏷️change-connection-label-en       | ✅apply  |
      | change-connection-label-de       | 🏷️change-connection-label-de       | ✅apply  |
      | change-connection-fastener-type  | 🔩️change-connection-fastener-type  | ✅apply  |
      | change-connection-strength-class | 🛡️change-connection-strength-class | ✅apply  |
      | change-connection-service-class  | 🌧️change-connection-service-class  | ✅apply  |
      | change-connection-diameter       | ↔️change-connection-diameter       | ✅apply  |
      | change-connection-number         | 🔢️change-connection-number         | ✅apply  |
      | change-connection-rows           | 🔢️change-connection-rows           | ✅apply  |
      | change-connection-spacing        | ↔️change-connection-spacing        | ✅apply  |
      | change-connection-edge-distance  | ↔️change-connection-edge-distance  | ✅apply  |
      | change-connection-end-distance   | ↔️change-connection-end-distance   | ✅apply  |
      | change-connection-t1             | ↔️change-connection-t1             | ✅apply  |
      | change-connection-t2             | ↔️change-connection-t2             | ✅apply  |
      | change-connection-steel-plate    | 🔩️change-connection-steel-plate    | ✅apply  |
      | change-connection-plate-thickness        | ↔️change-connection-plate-thickness        | ✅apply  |
      | change-connection-shear-planes   | 🔢️change-connection-shear-planes   | ✅apply  |
      | change-connection-fuk            | 🛡️change-connection-fuk            | ✅apply  |
      | insert-connection-action         | ➕️insert-connection-action         | ✅apply  |
      | remove-connection-action         | ➖️remove-connection-action         | ✅apply  |
      | change-connection-action-kind    | ⚖️change-connection-action-kind    | ✅apply  |
      | change-connection-load-duration  | ⏳️change-connection-load-duration  | ✅apply  |
      | change-connection-action-fk      | 🔩️change-connection-action-fk      | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1995 document from the parsed carrier
    Given the real committed text artifact asset://🏠️glulam-floor-beam/🏠️glulam-floor-beam/🗣️.dsl.semio
    And its committed binary twin asset://🏠️glulam-floor-beam/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
