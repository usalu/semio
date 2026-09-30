@capability-vdi3805-1-mutate
@oracle-vdi3805-1-python-independent
@comparison-ordered-json-v1
@mutations-vdi3805-1-any
Feature: Apply every typed VDI 3805 mutation against an independent Python implementation
  `s.norm.vdi3805` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 19 vectors cover
  every kind of the current vocabulary (8 `change`, 5 `remove`, 4 `add`, 1 `rename`, 1 `resize`) on a VDI 3805 manufacturer product data file; each vector's after-snapshot and diff
  were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same laws
  in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the document,
  and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list position
  included. `parity` adds that two implementations, in two languages, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://🎬️demo/🗣️.dsl.semio`. The carrier has no published grammar: the committed
  `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so the two sides are compared
  at the envelope preamble, the ordered lines and the digest and length of what each re-emitted.

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
      | id                           | dir                            | fixture               |
      | change-manufacturer-file     | 🏭️change-manufacturer-file     | ✏️sets-file           |
      | change-limits                | 🚧️change-limits                | 🛡️tightens-every      |
      | change-correction-as-of      | 📅️change-correction-as-of      | ✏️sets-of             |
      | change-strict-mode           | 🔒️change-strict-mode           | 🔒️turns-strict        |
      | change-edition-profile       | 🔖️change-edition-profile       | ✏️to-current          |
      | remove-edition-profile       | 🧹️remove-edition-profile       | ➖️removes             |
      | add-product                  | 📦️add-product                  | 📦️appends-vlv-80-002  |
      | remove-product               | 🗑️remove-product               | 🚫️removes-vlv-50-001  |
      | rename-product               | 🏷️rename-product               | 🏷️retitles-vlv-50     |
      | change-product-configuration | 🎛️change-product-configuration | ✏️sets                |
      | add-geometry                 | 🧊️add-geometry                 | 🧊️adds-the-geom-valve |
      | remove-geometry              | 🚮️remove-geometry              | 🚫️removes-the-geom    |
      | resize-geometry              | 📐️resize-geometry              | 📐️doubles-the-geom    |
      | add-geometry-connection      | 🔌️add-geometry-connection      | ➕️adds                |
      | remove-geometry-connection   | ✂️remove-geometry-connection   | ➖️removes             |
      | change-geometry-parameters   | 🧮️change-geometry-parameters   | ✏️sets                |
      | add-curve                    | 📈️add-curve                    | 📈️adds-the-curve-dp   |
      | remove-curve                 | 📉️remove-curve                 | 🚫️removes-the-curve   |
      | change-curve-points          | 📍️change-curve-points          | ✏️sets-points         |

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
      | id                           | dir                            | fixture               |
      | change-manufacturer-file     | 🏭️change-manufacturer-file     | ✏️sets-file           |
      | change-limits                | 🚧️change-limits                | 🛡️tightens-every      |
      | change-correction-as-of      | 📅️change-correction-as-of      | ✏️sets-of             |
      | change-strict-mode           | 🔒️change-strict-mode           | 🔒️turns-strict        |
      | change-edition-profile       | 🔖️change-edition-profile       | ✏️to-current          |
      | remove-edition-profile       | 🧹️remove-edition-profile       | ➖️removes             |
      | add-product                  | 📦️add-product                  | 📦️appends-vlv-80-002  |
      | remove-product               | 🗑️remove-product               | 🚫️removes-vlv-50-001  |
      | rename-product               | 🏷️rename-product               | 🏷️retitles-vlv-50     |
      | change-product-configuration | 🎛️change-product-configuration | ✏️sets                |
      | add-geometry                 | 🧊️add-geometry                 | 🧊️adds-the-geom-valve |
      | remove-geometry              | 🚮️remove-geometry              | 🚫️removes-the-geom    |
      | resize-geometry              | 📐️resize-geometry              | 📐️doubles-the-geom    |
      | add-geometry-connection      | 🔌️add-geometry-connection      | ➕️adds                |
      | remove-geometry-connection   | ✂️remove-geometry-connection   | ➖️removes             |
      | change-geometry-parameters   | 🧮️change-geometry-parameters   | ✏️sets                |
      | add-curve                    | 📈️add-curve                    | 📈️adds-the-curve-dp   |
      | remove-curve                 | 📉️remove-curve                 | 🚫️removes-the-curve   |
      | change-curve-points          | 📍️change-curve-points          | ✏️sets-points         |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed VDI 3805 document from the parsed carrier
    Given the real committed text artifact asset://🎬️demo/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
