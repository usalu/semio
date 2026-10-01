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
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 20 `✅apply` vectors
  cover every kind of the current vocabulary (14 `change`, 3 `insert`, 3 `remove`) on a spread foundation, piles, a retaining wall and a slope on layered soil; each vector's after-snapshot and
  diff were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same
  laws in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list
  position included. `parity` adds that two implementations, in two languages, reach the same document.

  The 3 refusal rows (`⛔dupe`) re-apply a kind's applied mutation to the
  after-snapshot it produced: re-inserting an id the collection now holds must be refused `mutation.duplicate-id`
  (Fatal), re-removing a member that is gone `mutation.target-missing` (Error) and re-setting a value the document
  already has must report `mutation.no-op` (Warning). Both sides must refuse under the committed code and leave the
  document bit-identical; a refusal has nothing to undo, so these rows are `mutate-` only.

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
      | id                              | dir                               | fixture |
      | change-annex                    | 🌍️change-annex                    | ✅apply  |
      | change-geotechnical-category    | 🗂️change-geotechnical-category    | ✅apply  |
      | change-design-situation         | 📅️change-design-situation         | ✅apply  |
      | change-design-approach          | 🧭️change-design-approach          | ✅apply  |
      | change-groundwater-level        | 💧change-groundwater-level         | ✅apply  |
      | change-investigation-depth      | 🔎️change-investigation-depth      | ✅apply  |
      | change-footing-width            | ↔️change-footing-width            | ✅apply  |
      | change-footing-embedment        | ⬇️change-footing-embedment        | ✅apply  |
      | change-pile-length              | 📏️change-pile-length              | ✅apply  |
      | change-pile-count               | 🔢change-pile-count                | ✅apply  |
      | change-wall-base-width          | 🧱change-wall-base-width           | ✅apply  |
      | change-slope-angle              | ⛰️change-slope-angle              | ✅apply  |
      | change-layer-phi-prime          | 📐️change-layer-phi-prime          | ✅apply  |
      | change-layer-oedometric-modulus | 🌀️change-layer-oedometric-modulus | ✅apply  |
      | insert-layer                    | ➕️insert-layer                    | ✅apply  |
      | insert-layer-dupe               | ➕️insert-layer                    | ⛔dupe   |
      | remove-layer                    | ➖️remove-layer                    | ✅apply  |
      | insert-footing                  | ➕insert-footing                   | ✅apply  |
      | insert-footing-dupe             | ➕insert-footing                   | ⛔dupe   |
      | remove-footing                  | ➖remove-footing                   | ✅apply  |
      | insert-pile                     | 📥insert-pile                      | ✅apply  |
      | insert-pile-dupe                | 📥insert-pile                      | ⛔dupe   |
      | remove-pile                     | 📤remove-pile                      | ✅apply  |

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
      | id                              | dir                               | fixture |
      | change-annex                    | 🌍️change-annex                    | ✅apply  |
      | change-geotechnical-category    | 🗂️change-geotechnical-category    | ✅apply  |
      | change-design-situation         | 📅️change-design-situation         | ✅apply  |
      | change-design-approach          | 🧭️change-design-approach          | ✅apply  |
      | change-groundwater-level        | 💧change-groundwater-level         | ✅apply  |
      | change-investigation-depth      | 🔎️change-investigation-depth      | ✅apply  |
      | change-footing-width            | ↔️change-footing-width            | ✅apply  |
      | change-footing-embedment        | ⬇️change-footing-embedment        | ✅apply  |
      | change-pile-length              | 📏️change-pile-length              | ✅apply  |
      | change-pile-count               | 🔢change-pile-count                | ✅apply  |
      | change-wall-base-width          | 🧱change-wall-base-width           | ✅apply  |
      | change-slope-angle              | ⛰️change-slope-angle              | ✅apply  |
      | change-layer-phi-prime          | 📐️change-layer-phi-prime          | ✅apply  |
      | change-layer-oedometric-modulus | 🌀️change-layer-oedometric-modulus | ✅apply  |
      | insert-layer                    | ➕️insert-layer                    | ✅apply  |
      | remove-layer                    | ➖️remove-layer                    | ✅apply  |
      | insert-footing                  | ➕insert-footing                   | ✅apply  |
      | remove-footing                  | ➖remove-footing                   | ✅apply  |
      | insert-pile                     | 📥insert-pile                      | ✅apply  |
      | remove-pile                     | 📤remove-pile                      | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1997 document from the parsed carrier
    Given the real committed text artifact asset://🎬️demo/🗣️.dsl.semio
    And its committed binary twin asset://🎬️demo/📦️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
