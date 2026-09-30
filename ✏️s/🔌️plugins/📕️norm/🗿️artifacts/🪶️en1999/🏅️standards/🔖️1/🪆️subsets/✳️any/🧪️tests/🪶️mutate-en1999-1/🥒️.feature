@capability-en1999-1-mutate
@oracle-en1999-1-python-independent
@comparison-ordered-json-v1
@mutations-en1999-1-any
Feature: Apply every typed EN 1999 mutation against an independent Python implementation
  `s.norm.en1999` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 18 vectors cover
  every kind of the current vocabulary (16 `change`, 1 `add`, 1 `remove`) on an aluminium roof purlin with its connections; each vector's after-snapshot and diff
  were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same laws
  in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the document,
  and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list position
  included. `parity` adds that two implementations, in two languages, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio`. The carrier has no published grammar: the committed
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
      | id                            | dir                             | fixture              |
      | change-annex                  | 🌍️change-annex                  | ✏️to-en              |
      | change-materials              | 🧱change-materials               | ✏️sets-materials     |
      | change-sections               | 📐️change-sections               | ✏️sets-sections      |
      | change-members                | 🏗️change-members                | ✏️sets-members       |
      | change-connections            | 🔗change-connections             | ✏️larger-weld-throat |
      | change-fire-scenarios         | 🔥️change-fire-scenarios         | 🔥️hotter-fire        |
      | change-fatigue-details        | 🔄️change-fatigue-details        | ✏️more-stress        |
      | change-cold-formed            | ❄️change-cold-formed            | ✏️thinner-sheet      |
      | change-shells                 | 🫙change-shells                  | ✏️thicker-shell      |
      | add-member                    | ➕add-member                     | ➕️adds-member        |
      | remove-member                 | ➖remove-member                  | ➖️removes-member     |
      | change-member-n-ed            | 🏋️change-member-n-ed            | ✏️to-2500            |
      | change-member-my-ed           | ⤴️change-member-my-ed           | ✏️to-5000            |
      | change-member-buckling-length | 📏️change-member-buckling-length | ✏️to-0               |
      | change-material-designation   | ⚗️change-material-designation   | ✏️to-en-aw           |
      | change-plate-thickness        | 🧱change-plate-thickness         | ✏️to-0-0125          |
      | change-weld-throat            | 🔥️change-weld-throat            | ✏️to-0-006           |
      | change-bolt-count             | 🔩change-bolt-count              | ✏️to-3               |

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
      | id                            | dir                             | fixture              |
      | change-annex                  | 🌍️change-annex                  | ✏️to-en              |
      | change-materials              | 🧱change-materials               | ✏️sets-materials     |
      | change-sections               | 📐️change-sections               | ✏️sets-sections      |
      | change-members                | 🏗️change-members                | ✏️sets-members       |
      | change-connections            | 🔗change-connections             | ✏️larger-weld-throat |
      | change-fire-scenarios         | 🔥️change-fire-scenarios         | 🔥️hotter-fire        |
      | change-fatigue-details        | 🔄️change-fatigue-details        | ✏️more-stress        |
      | change-cold-formed            | ❄️change-cold-formed            | ✏️thinner-sheet      |
      | change-shells                 | 🫙change-shells                  | ✏️thicker-shell      |
      | add-member                    | ➕add-member                     | ➕️adds-member        |
      | remove-member                 | ➖remove-member                  | ➖️removes-member     |
      | change-member-n-ed            | 🏋️change-member-n-ed            | ✏️to-2500            |
      | change-member-my-ed           | ⤴️change-member-my-ed           | ✏️to-5000            |
      | change-member-buckling-length | 📏️change-member-buckling-length | ✏️to-0               |
      | change-material-designation   | ⚗️change-material-designation   | ✏️to-en-aw           |
      | change-plate-thickness        | 🧱change-plate-thickness         | ✏️to-0-0125          |
      | change-weld-throat            | 🔥️change-weld-throat            | ✏️to-0-006           |
      | change-bolt-count             | 🔩change-bolt-count              | ✏️to-3               |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1999 document from the parsed carrier
    Given the real committed text artifact asset://🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio
    And its committed binary twin asset://🏠️aluminium-roof-purlin/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
