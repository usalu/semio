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
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 29 vectors cover
  every kind of the current vocabulary (8 `introduce`, 8 `retire`, 5 `change`, 4 `rename`, 2 `remove`, 1 `replace`, 1 `add`) on a building-services product catalogue; each vector's after-snapshot and diff
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
      | id                            | dir                             | fixture             |
      | change-exchange-process       | 🔄️change-exchange-process       | ✏️sets              |
      | change-script-limits          | 🚦️change-script-limits          | ✏️to-20000          |
      | replace-part-number-rule      | 🧮️replace-part-number-rule      | ✏️sets              |
      | change-part-number-input      | 🎛️change-part-number-input      | ✏️sets              |
      | remove-part-number-input      | 🔌️remove-part-number-input      | ➖️removes           |
      | change-selection-class        | 🎯️change-selection-class        | ✏️to-class          |
      | change-selection-series       | 🧵️change-selection-series       | ✏️to-series         |
      | add-selection-constraint      | 🔒️add-selection-constraint      | ➕️adds              |
      | remove-selection-constraint   | 🔓️remove-selection-constraint   | ✏️sets              |
      | rename-catalogue              | 📇️rename-catalogue              | ✏️to-fixture        |
      | rename-manufacturer           | 🏭️rename-manufacturer           | 🏭️adds-the-ag       |
      | introduce-product-group       | 🧺️introduce-product-group       | ✏️sets              |
      | retire-product-group          | 🧹️retire-product-group          | ➖️retires           |
      | rename-product-group          | 🗂️rename-product-group          | ✏️to-panel          |
      | introduce-product             | 📦️introduce-product             | ➕️introduces        |
      | retire-product                | 🚫️retire-product                | 🚫️removes-the-pr600 |
      | rename-product                | 🏷️rename-product                | ✏️renames-pr600     |
      | introduce-property-definition | 📐️introduce-property-definition | ✏️new               |
      | retire-property-definition    | 🧽️retire-property-definition    | ✏️sets              |
      | introduce-subject             | 🌳️introduce-subject             | 🌳️appends-towel     |
      | retire-subject                | ✂️retire-subject                | ➖️retires-subject   |
      | introduce-product-class       | 🏷️introduce-product-class       | ✏️sets              |
      | retire-product-class          | 🗑️retire-product-class          | ➖️retires           |
      | introduce-product-series      | 📚introduce-product-series       | 📚appends-a-pr       |
      | retire-product-series         | 🗑️retire-product-series         | ➖️retires           |
      | introduce-product-index       | 🔎introduce-product-index        | ➕️introduces        |
      | retire-product-index          | 🗑️retire-product-index          | ➖️retires           |
      | introduce-geometry-object     | 📐introduce-geometry-object      | ➕️introduces        |
      | retire-geometry-object        | 🗑️retire-geometry-object        | ➖️retires           |

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
      | id                            | dir                             | fixture             |
      | change-exchange-process       | 🔄️change-exchange-process       | ✏️sets              |
      | change-script-limits          | 🚦️change-script-limits          | ✏️to-20000          |
      | replace-part-number-rule      | 🧮️replace-part-number-rule      | ✏️sets              |
      | change-part-number-input      | 🎛️change-part-number-input      | ✏️sets              |
      | remove-part-number-input      | 🔌️remove-part-number-input      | ➖️removes           |
      | change-selection-class        | 🎯️change-selection-class        | ✏️to-class          |
      | change-selection-series       | 🧵️change-selection-series       | ✏️to-series         |
      | add-selection-constraint      | 🔒️add-selection-constraint      | ➕️adds              |
      | remove-selection-constraint   | 🔓️remove-selection-constraint   | ✏️sets              |
      | rename-catalogue              | 📇️rename-catalogue              | ✏️to-fixture        |
      | rename-manufacturer           | 🏭️rename-manufacturer           | 🏭️adds-the-ag       |
      | introduce-product-group       | 🧺️introduce-product-group       | ✏️sets              |
      | retire-product-group          | 🧹️retire-product-group          | ➖️retires           |
      | rename-product-group          | 🗂️rename-product-group          | ✏️to-panel          |
      | introduce-product             | 📦️introduce-product             | ➕️introduces        |
      | retire-product                | 🚫️retire-product                | 🚫️removes-the-pr600 |
      | rename-product                | 🏷️rename-product                | ✏️renames-pr600     |
      | introduce-property-definition | 📐️introduce-property-definition | ✏️new               |
      | retire-property-definition    | 🧽️retire-property-definition    | ✏️sets              |
      | introduce-subject             | 🌳️introduce-subject             | 🌳️appends-towel     |
      | retire-subject                | ✂️retire-subject                | ➖️retires-subject   |
      | introduce-product-class       | 🏷️introduce-product-class       | ✏️sets              |
      | retire-product-class          | 🗑️retire-product-class          | ➖️retires           |
      | introduce-product-series      | 📚introduce-product-series       | 📚appends-a-pr       |
      | retire-product-series         | 🗑️retire-product-series         | ➖️retires           |
      | introduce-product-index       | 🔎introduce-product-index        | ➕️introduces        |
      | retire-product-index          | 🗑️retire-product-index          | ➖️retires           |
      | introduce-geometry-object     | 📐introduce-geometry-object      | ➕️introduces        |
      | retire-geometry-object        | 🗑️retire-geometry-object        | ➖️retires           |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed ISO 16757 document from the parsed carrier
    Given the real committed text artifact asset://🎬️demo/🗣️.dsl.semio
    And its committed binary twin asset://🎬️demo/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
