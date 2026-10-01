@capability-en1998-1-mutate
@oracle-en1998-1-python-independent
@comparison-ordered-json-v1
@mutations-en1998-1-any
Feature: Apply every typed EN 1998 mutation against an independent Python implementation
  `s.norm.en1998` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 29 `✅apply` vectors
  cover every kind of the current vocabulary (12 `change`, 8 `insert`, 8 `remove`, 1 `update`) on a seismic reinforced-concrete frame, bridges, silos, tanks, foundations, walls and towers; each vector's after-snapshot and
  diff were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same
  laws in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list
  position included. `parity` adds that two implementations, in two languages, reach the same document.

  The 9 refusal rows (`⛔dupe`, `🟰noop`) re-apply a kind's applied mutation to the
  after-snapshot it produced: re-inserting an id the collection now holds must be refused `mutation.duplicate-id`
  (Fatal), re-removing a member that is gone `mutation.target-missing` (Error) and re-setting a value the document
  already has must report `mutation.no-op` (Warning). Both sides must refuse under the committed code and leave the
  document bit-identical; a refusal has nothing to undo, so these rows are `mutate-` only.

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio`. The carrier has no published grammar: the committed
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
      | id                           | dir                            | fixture |
      | change-annex                 | 📎️change-annex                 | ✅apply  |
      | update-site                  | 🌚️update-site                  | ✅apply  |
      | update-site-noop             | 🌚️update-site                  | 🟰noop   |
      | insert-building              | ➕️insert-building              | ✅apply  |
      | insert-building-dupe         | ➕️insert-building              | ⛔dupe   |
      | remove-building              | ➖️remove-building              | ✅apply  |
      | change-system-v-rd-n         | 💪️change-system-v-rd-n         | ✅apply  |
      | change-storey-permanent-gk-n | ⚖️change-storey-permanent-gk-n | ✅apply  |
      | change-storey-stiffness-x    | 📐️change-storey-stiffness-x    | ✅apply  |
      | change-storey-drift-xm       | 📏️change-storey-drift-xm       | ✅apply  |
      | change-building-plan-regular | 🧭️change-building-plan-regular | ✅apply  |
      | change-elevation-regular     | 📏️change-elevation-regular     | ✅apply  |
      | change-member-detailing      | ✅️change-member-detailing      | ✅apply  |
      | change-masonry-wall-ratio    | 🧱️change-masonry-wall-ratio    | ✅apply  |
      | insert-bridge                | 🌉insert-bridge                 | ✅apply  |
      | insert-bridge-dupe           | 🌉insert-bridge                 | ⛔dupe   |
      | change-bridge-v-rd-n         | 🛑️change-bridge-v-rd-n         | ✅apply  |
      | insert-assessment            | 🔧insert-assessment             | ✅apply  |
      | insert-assessment-dupe       | 🔧insert-assessment             | ⛔dupe   |
      | change-assessment-rkn        | 🏋️change-assessment-rkn        | ✅apply  |
      | insert-silo                  | 🫙insert-silo                   | ✅apply  |
      | insert-silo-dupe             | 🫙insert-silo                   | ⛔dupe   |
      | insert-tank                  | 🛢insert-tank                   | ✅apply  |
      | insert-tank-dupe             | 🛢insert-tank                   | ⛔dupe   |
      | insert-foundation            | 🪨insert-foundation             | ✅apply  |
      | insert-foundation-dupe       | 🪨insert-foundation             | ⛔dupe   |
      | insert-retaining-wall        | 🧱️insert-retaining-wall        | ✅apply  |
      | insert-retaining-wall-dupe   | 🧱️insert-retaining-wall        | ⛔dupe   |
      | insert-tower                 | 🗼insert-tower                  | ✅apply  |
      | insert-tower-dupe            | 🗼insert-tower                  | ⛔dupe   |
      | change-tower-m-rd-nm         | ↪️change-tower-m-rd-nm         | ✅apply  |
      | remove-bridge                | ➖️remove-bridge                | ✅apply  |
      | remove-assessment            | ➖️remove-assessment            | ✅apply  |
      | remove-silo                  | ➖️remove-silo                  | ✅apply  |
      | remove-tank                  | ➖️remove-tank                  | ✅apply  |
      | remove-foundation            | ➖️remove-foundation            | ✅apply  |
      | remove-retaining-wall        | ➖️remove-retaining-wall        | ✅apply  |
      | remove-tower                 | ➖️remove-tower                 | ✅apply  |

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
      | id                           | dir                            | fixture |
      | change-annex                 | 📎️change-annex                 | ✅apply  |
      | update-site                  | 🌚️update-site                  | ✅apply  |
      | insert-building              | ➕️insert-building              | ✅apply  |
      | remove-building              | ➖️remove-building              | ✅apply  |
      | change-system-v-rd-n         | 💪️change-system-v-rd-n         | ✅apply  |
      | change-storey-permanent-gk-n | ⚖️change-storey-permanent-gk-n | ✅apply  |
      | change-storey-stiffness-x    | 📐️change-storey-stiffness-x    | ✅apply  |
      | change-storey-drift-xm       | 📏️change-storey-drift-xm       | ✅apply  |
      | change-building-plan-regular | 🧭️change-building-plan-regular | ✅apply  |
      | change-elevation-regular     | 📏️change-elevation-regular     | ✅apply  |
      | change-member-detailing      | ✅️change-member-detailing      | ✅apply  |
      | change-masonry-wall-ratio    | 🧱️change-masonry-wall-ratio    | ✅apply  |
      | insert-bridge                | 🌉insert-bridge                 | ✅apply  |
      | change-bridge-v-rd-n         | 🛑️change-bridge-v-rd-n         | ✅apply  |
      | insert-assessment            | 🔧insert-assessment             | ✅apply  |
      | change-assessment-rkn        | 🏋️change-assessment-rkn        | ✅apply  |
      | insert-silo                  | 🫙insert-silo                   | ✅apply  |
      | insert-tank                  | 🛢insert-tank                   | ✅apply  |
      | insert-foundation            | 🪨insert-foundation             | ✅apply  |
      | insert-retaining-wall        | 🧱️insert-retaining-wall        | ✅apply  |
      | insert-tower                 | 🗼insert-tower                  | ✅apply  |
      | change-tower-m-rd-nm         | ↪️change-tower-m-rd-nm         | ✅apply  |
      | remove-bridge                | ➖️remove-bridge                | ✅apply  |
      | remove-assessment            | ➖️remove-assessment            | ✅apply  |
      | remove-silo                  | ➖️remove-silo                  | ✅apply  |
      | remove-tank                  | ➖️remove-tank                  | ✅apply  |
      | remove-foundation            | ➖️remove-foundation            | ✅apply  |
      | remove-retaining-wall        | ➖️remove-retaining-wall        | ✅apply  |
      | remove-tower                 | ➖️remove-tower                 | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1998 document from the parsed carrier
    Given the real committed text artifact asset://🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio
    And its committed binary twin asset://🏢️seismic-rc-frame/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
