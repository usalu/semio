@capability-iso16757-1-mutate
@oracle-iso16757-1-python-independent
@comparison-ordered-json-v1
@mutations-iso16757-1-any
Feature: Apply every typed ISO 16757 mutation against an independent Python implementation
  `s.norm.iso16757` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 29 `✅apply` vectors
  cover every kind of the current vocabulary (8 `introduce`, 8 `retire`, 5 `change`, 4 `rename`, 2 `remove`, 1 `replace`, 1 `add`) on a building-services product catalogue; each vector's after-snapshot and
  diff were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same
  laws in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list
  position included. `parity` adds that two implementations, in two languages, reach the same document.

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
      | id                            | dir                             | fixture |
      | change-exchange-process       | 🔄️change-exchange-process       | ✅apply  |
      | change-script-limits          | 🚦️change-script-limits          | ✅apply  |
      | replace-part-number-rule      | 🧮️replace-part-number-rule      | ✅apply  |
      | change-part-number-input      | 🎛️change-part-number-input      | ✅apply  |
      | remove-part-number-input      | 🔌️remove-part-number-input      | ✅apply  |
      | change-selection-class        | 🎯️change-selection-class        | ✅apply  |
      | change-selection-series       | 🧵️change-selection-series       | ✅apply  |
      | add-selection-constraint      | 🔒️add-selection-constraint      | ✅apply  |
      | remove-selection-constraint   | 🔓️remove-selection-constraint   | ✅apply  |
      | rename-catalogue              | 📇️rename-catalogue              | ✅apply  |
      | rename-manufacturer           | 🏭️rename-manufacturer           | ✅apply  |
      | introduce-product-group       | 🧺️introduce-product-group       | ✅apply  |
      | retire-product-group          | 🧹️retire-product-group          | ✅apply  |
      | rename-product-group          | 🗂️rename-product-group          | ✅apply  |
      | introduce-product             | 📦️introduce-product             | ✅apply  |
      | retire-product                | 🚫️retire-product                | ✅apply  |
      | rename-product                | 🏷️rename-product                | ✅apply  |
      | introduce-property-definition | 📐️introduce-property-definition | ✅apply  |
      | retire-property-definition    | 🧽️retire-property-definition    | ✅apply  |
      | introduce-subject             | 🌳️introduce-subject             | ✅apply  |
      | retire-subject                | ✂️retire-subject                | ✅apply  |
      | introduce-product-class       | 🏷️introduce-product-class       | ✅apply  |
      | retire-product-class          | 🗑️retire-product-class          | ✅apply  |
      | introduce-product-series      | 📚introduce-product-series       | ✅apply  |
      | retire-product-series         | 🗑️retire-product-series         | ✅apply  |
      | introduce-product-index       | 🔎introduce-product-index        | ✅apply  |
      | retire-product-index          | 🗑️retire-product-index          | ✅apply  |
      | introduce-geometry-object     | 📐introduce-geometry-object      | ✅apply  |
      | retire-geometry-object        | 🗑️retire-geometry-object        | ✅apply  |

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
      | change-exchange-process       | 🔄️change-exchange-process       | ✅apply  |
      | change-script-limits          | 🚦️change-script-limits          | ✅apply  |
      | replace-part-number-rule      | 🧮️replace-part-number-rule      | ✅apply  |
      | change-part-number-input      | 🎛️change-part-number-input      | ✅apply  |
      | remove-part-number-input      | 🔌️remove-part-number-input      | ✅apply  |
      | change-selection-class        | 🎯️change-selection-class        | ✅apply  |
      | change-selection-series       | 🧵️change-selection-series       | ✅apply  |
      | add-selection-constraint      | 🔒️add-selection-constraint      | ✅apply  |
      | remove-selection-constraint   | 🔓️remove-selection-constraint   | ✅apply  |
      | rename-catalogue              | 📇️rename-catalogue              | ✅apply  |
      | rename-manufacturer           | 🏭️rename-manufacturer           | ✅apply  |
      | introduce-product-group       | 🧺️introduce-product-group       | ✅apply  |
      | retire-product-group          | 🧹️retire-product-group          | ✅apply  |
      | rename-product-group          | 🗂️rename-product-group          | ✅apply  |
      | introduce-product             | 📦️introduce-product             | ✅apply  |
      | retire-product                | 🚫️retire-product                | ✅apply  |
      | rename-product                | 🏷️rename-product                | ✅apply  |
      | introduce-property-definition | 📐️introduce-property-definition | ✅apply  |
      | retire-property-definition    | 🧽️retire-property-definition    | ✅apply  |
      | introduce-subject             | 🌳️introduce-subject             | ✅apply  |
      | retire-subject                | ✂️retire-subject                | ✅apply  |
      | introduce-product-class       | 🏷️introduce-product-class       | ✅apply  |
      | retire-product-class          | 🗑️retire-product-class          | ✅apply  |
      | introduce-product-series      | 📚introduce-product-series       | ✅apply  |
      | retire-product-series         | 🗑️retire-product-series         | ✅apply  |
      | introduce-product-index       | 🔎introduce-product-index        | ✅apply  |
      | retire-product-index          | 🗑️retire-product-index          | ✅apply  |
      | introduce-geometry-object     | 📐introduce-geometry-object      | ✅apply  |
      | retire-geometry-object        | 🗑️retire-geometry-object        | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed ISO 16757 document from the parsed carrier
    Given the real committed text artifact asset://🎬️demo/🗣️.dsl.semio
    And its committed binary twin asset://🎬️demo/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
