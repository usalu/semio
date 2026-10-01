@capability-en1994-1-mutate
@oracle-en1994-1-python-independent
@comparison-ordered-json-v1
@mutations-en1994-1-any
Feature: Apply every typed EN 1994 mutation against an independent Python implementation
  `s.norm.en1994` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 25 `✅apply` vectors
  cover every kind of the current vocabulary (19 `change`, 3 `insert`, 3 `remove`) on a steel-concrete composite bridge girder; each vector's after-snapshot and
  diff were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same
  laws in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list
  position included. `parity` adds that two implementations, in two languages, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://🌉️composite-bridge-girder/🌉️composite-bridge-girder/🗣️.dsl.semio`. The carrier has no published grammar: the committed
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
      | id                            | dir                             | fixture |
      | change-annex                  | 🌍️change-annex                  | ✅apply  |
      | change-structure-kind         | 🏗️change-structure-kind         | ✅apply  |
      | change-steel-fy-pa            | 🏋️change-steel-fy-pa            | ✅apply  |
      | change-fire-rating            | 🔥️change-fire-rating            | ✅apply  |
      | change-insulation-thickness-m | 🧯️change-insulation-thickness-m | ✅apply  |
      | change-fatigue-detail         | 🔁️change-fatigue-detail         | ✅apply  |
      | insert-beam                   | ➕️insert-beam                   | ✅apply  |
      | remove-beam                   | ➖️remove-beam                   | ✅apply  |
      | change-beam-action-q-area-pa  | 🌀️change-beam-action-q-area-pa  | ✅apply  |
      | change-beam-stud-spacing-m    | ✂️change-beam-stud-spacing-m    | ✅apply  |
      | change-beam-span-m            | 📏️change-beam-span-m            | ✅apply  |
      | change-beam-slab-thickness-m  | 🧱change-beam-slab-thickness-m   | ✅apply  |
      | change-beam-stud-diameter-m   | ⭕️change-beam-stud-diameter-m   | ✅apply  |
      | change-beam-stud-count        | #️⃣change-beam-stud-count       | ✅apply  |
      | change-beam-stud-fu-pa        | 💪️change-beam-stud-fu-pa        | ✅apply  |
      | change-beam-transverse-as     | ↔️change-beam-transverse-as     | ✅apply  |
      | change-beam-construction      | 🛠️change-beam-construction      | ✅apply  |
      | insert-column                 | ➗️insert-column                 | ✅apply  |
      | remove-column                 | ⛔️remove-column                 | ✅apply  |
      | change-column-action-force-n  | ⬇️change-column-action-force-n  | ✅apply  |
      | change-column-kind            | ↪️change-column-kind            | ✅apply  |
      | insert-slab                   | ➕insert-slab                    | ✅apply  |
      | remove-slab                   | ➖remove-slab                    | ✅apply  |
      | change-slab-action-q-area-pa  | 📐️change-slab-action-q-area-pa  | ✅apply  |
      | change-slab-thickness-m       | 📏change-slab-thickness-m        | ✅apply  |

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
      | id                            | dir                             | fixture |
      | change-annex                  | 🌍️change-annex                  | ✅apply  |
      | change-structure-kind         | 🏗️change-structure-kind         | ✅apply  |
      | change-steel-fy-pa            | 🏋️change-steel-fy-pa            | ✅apply  |
      | change-fire-rating            | 🔥️change-fire-rating            | ✅apply  |
      | change-insulation-thickness-m | 🧯️change-insulation-thickness-m | ✅apply  |
      | change-fatigue-detail         | 🔁️change-fatigue-detail         | ✅apply  |
      | insert-beam                   | ➕️insert-beam                   | ✅apply  |
      | remove-beam                   | ➖️remove-beam                   | ✅apply  |
      | change-beam-action-q-area-pa  | 🌀️change-beam-action-q-area-pa  | ✅apply  |
      | change-beam-stud-spacing-m    | ✂️change-beam-stud-spacing-m    | ✅apply  |
      | change-beam-span-m            | 📏️change-beam-span-m            | ✅apply  |
      | change-beam-slab-thickness-m  | 🧱change-beam-slab-thickness-m   | ✅apply  |
      | change-beam-stud-diameter-m   | ⭕️change-beam-stud-diameter-m   | ✅apply  |
      | change-beam-stud-count        | #️⃣change-beam-stud-count       | ✅apply  |
      | change-beam-stud-fu-pa        | 💪️change-beam-stud-fu-pa        | ✅apply  |
      | change-beam-transverse-as     | ↔️change-beam-transverse-as     | ✅apply  |
      | change-beam-construction      | 🛠️change-beam-construction      | ✅apply  |
      | insert-column                 | ➗️insert-column                 | ✅apply  |
      | remove-column                 | ⛔️remove-column                 | ✅apply  |
      | change-column-action-force-n  | ⬇️change-column-action-force-n  | ✅apply  |
      | change-column-kind            | ↪️change-column-kind            | ✅apply  |
      | insert-slab                   | ➕insert-slab                    | ✅apply  |
      | remove-slab                   | ➖remove-slab                    | ✅apply  |
      | change-slab-action-q-area-pa  | 📐️change-slab-action-q-area-pa  | ✅apply  |
      | change-slab-thickness-m       | 📏change-slab-thickness-m        | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1994 document from the parsed carrier
    Given the real committed text artifact asset://🌉️composite-bridge-girder/🌉️composite-bridge-girder/🗣️.dsl.semio
    And its committed binary twin asset://🌉️composite-bridge-girder/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
