@capability-en1997-1-mutate
@oracle-en1997-1-python-independent
@comparison-ordered-json-v1
@mutations-en1997-1-any
Feature: Apply every typed EN 1997 mutation against an independent Python implementation
  `s.norm.en1997` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 20 vectors cover
  every kind of the current vocabulary (14 `change`, 3 `insert`, 3 `remove`) on a spread foundation, piles, a retaining wall and a slope on layered soil; each vector's after-snapshot and diff
  were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same laws
  in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the document,
  and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list position
  included. `parity` adds that two implementations, in two languages, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://🎬️demo/🗣️.dsl.semio`. The carrier has no published grammar: the committed
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
      | id                              | dir                               | fixture           |
      | change-annex                    | 🌍️change-annex                    | ✏️to-en           |
      | change-geotechnical-category    | 🗂️change-geotechnical-category    | ✏️to-3            |
      | change-design-situation         | 📅️change-design-situation         | ✏️to-bs-t         |
      | change-design-approach          | 🧭️change-design-approach          | ✏️to-da3          |
      | change-groundwater-level        | 💧change-groundwater-level         | ✏️to-2-5          |
      | change-investigation-depth      | 🔎️change-investigation-depth      | ✏️to-25           |
      | change-footing-width            | ↔️change-footing-width            | ✏️to-3            |
      | change-footing-embedment        | ⬇️change-footing-embedment        | ✏️to-1-8          |
      | change-pile-length              | 📏️change-pile-length              | ✏️to-16           |
      | change-pile-count               | 🔢change-pile-count                | ✏️to-3            |
      | change-wall-base-width          | 🧱change-wall-base-width           | ✏️to-2-9          |
      | change-slope-angle              | ⛰️change-slope-angle              | ✏️to-30           |
      | change-layer-phi-prime          | 📐️change-layer-phi-prime          | ✏️to-32-5         |
      | change-layer-oedometric-modulus | 🌀️change-layer-oedometric-modulus | ✏️new             |
      | insert-layer                    | ➕️insert-layer                    | ➕️inserts-layer   |
      | remove-layer                    | ➖️remove-layer                    | ➖️removes-layer   |
      | insert-footing                  | ➕insert-footing                   | ➕️inserts-footing |
      | remove-footing                  | ➖remove-footing                   | ➖️removes-footing |
      | insert-pile                     | 📥insert-pile                      | ➕️inserts-pile    |
      | remove-pile                     | 📤remove-pile                      | ➖️removes-pile    |

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
      | id                              | dir                               | fixture           |
      | change-annex                    | 🌍️change-annex                    | ✏️to-en           |
      | change-geotechnical-category    | 🗂️change-geotechnical-category    | ✏️to-3            |
      | change-design-situation         | 📅️change-design-situation         | ✏️to-bs-t         |
      | change-design-approach          | 🧭️change-design-approach          | ✏️to-da3          |
      | change-groundwater-level        | 💧change-groundwater-level         | ✏️to-2-5          |
      | change-investigation-depth      | 🔎️change-investigation-depth      | ✏️to-25           |
      | change-footing-width            | ↔️change-footing-width            | ✏️to-3            |
      | change-footing-embedment        | ⬇️change-footing-embedment        | ✏️to-1-8          |
      | change-pile-length              | 📏️change-pile-length              | ✏️to-16           |
      | change-pile-count               | 🔢change-pile-count                | ✏️to-3            |
      | change-wall-base-width          | 🧱change-wall-base-width           | ✏️to-2-9          |
      | change-slope-angle              | ⛰️change-slope-angle              | ✏️to-30           |
      | change-layer-phi-prime          | 📐️change-layer-phi-prime          | ✏️to-32-5         |
      | change-layer-oedometric-modulus | 🌀️change-layer-oedometric-modulus | ✏️new             |
      | insert-layer                    | ➕️insert-layer                    | ➕️inserts-layer   |
      | remove-layer                    | ➖️remove-layer                    | ➖️removes-layer   |
      | insert-footing                  | ➕insert-footing                   | ➕️inserts-footing |
      | remove-footing                  | ➖remove-footing                   | ➖️removes-footing |
      | insert-pile                     | 📥insert-pile                      | ➕️inserts-pile    |
      | remove-pile                     | 📤remove-pile                      | ➖️removes-pile    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1997 document from the parsed carrier
    Given the real committed text artifact asset://🎬️demo/🗣️.dsl.semio
    And its committed binary twin asset://🎬️demo/📦️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
