@capability-en1993-1-mutate
@oracle-en1993-1-python-independent
@comparison-ordered-json-v1
@mutations-en1993-1-any
Feature: Apply every typed EN 1993 mutation against an independent Python implementation
  `s.norm.en1993` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 49 vectors cover
  every kind of the current vocabulary (16 `update`, 16 `insert`, 16 `remove`, 1 `change`) on a high-strength bolted steel connection; each vector's after-snapshot and diff
  were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same laws
  in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the document,
  and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list position
  included. `parity` adds that two implementations, in two languages, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://🔩️high-strength-connection/🔩️high-strength-connection/🗣️.dsl.semio`. The carrier has no published grammar: the committed
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
      | id                              | dir                               | fixture                 |
      | change-annex                    | 🌍️change-annex                    | 🌐️switches-the-national |
      | update-member-properties        | 📊️update-member-properties        | 🏋️re-grades             |
      | update-fire-inputs              | 🔥️update-fire-inputs              | 🧯️raises-the-fire       |
      | update-cold-formed-inputs       | 🥶️update-cold-formed-inputs       | ✏️sets                  |
      | update-stainless-inputs         | ✨️update-stainless-inputs         | ✏️sets-inputs           |
      | update-plated-inputs            | 🧱️update-plated-inputs            | 📈️makes-plate           |
      | update-silo-shell-inputs        | 🛢️update-silo-shell-inputs        | ✏️sets                  |
      | update-bolt-inputs              | 🔩️update-bolt-inputs              | ✏️sets-inputs           |
      | update-weld-inputs              | 🧲️update-weld-inputs              | ✏️sets-inputs           |
      | update-fatigue-inputs           | 🔁️update-fatigue-inputs           | 🔁️drops-detail          |
      | update-through-thickness-inputs | ↕️update-through-thickness-inputs | ✏️sets                  |
      | update-tension-component-inputs | 🪢️update-tension-component-inputs | ✏️new                   |
      | update-hss-inputs               | ⬜️update-hss-inputs               | ✏️sets-inputs           |
      | update-bridge-inputs            | 🌉️update-bridge-inputs            | 🌉️raises-bridge         |
      | update-tower-inputs             | 🗼️update-tower-inputs             | ✏️sets-inputs           |
      | update-pile-inputs              | 🪵️update-pile-inputs              | ✏️sets-inputs           |
      | update-crane-inputs             | 🏗️update-crane-inputs             | 🏋️widens-crane          |
      | insert-material                 | ➕️insert-material                 | ➕️inserts-material      |
      | remove-material                 | ➖️remove-material                 | ➖️removes-material      |
      | insert-section                  | ➕️insert-section                  | ➕️inserts-section       |
      | remove-section                  | ➖️remove-section                  | ➖️removes-section       |
      | insert-member                   | ➕️insert-member                   | ➕️inserts-member        |
      | remove-member                   | ➖️remove-member                   | ➖️removes-member        |
      | insert-load-case                | ➕️insert-load-case                | ➕️inserts-case          |
      | remove-load-case                | ➖️remove-load-case                | ➖️removes-case          |
      | insert-member-action            | ➕️insert-member-action            | ➕️inserts-action        |
      | remove-member-action            | ➖️remove-member-action            | ➖️removes-action        |
      | insert-joint                    | ➕️insert-joint                    | ➕️inserts-joint         |
      | remove-joint                    | ➖️remove-joint                    | ➖️removes-joint         |
      | insert-fatigue-detail           | ➕️insert-fatigue-detail           | ➕️inserts-detail        |
      | remove-fatigue-detail           | ➖️remove-fatigue-detail           | ➖️removes-detail        |
      | insert-fire-exposure            | ➕️insert-fire-exposure            | ➕️inserts               |
      | remove-fire-exposure            | ➖️remove-fire-exposure            | ➖️removes               |
      | insert-cold-formed-member       | ➕️insert-cold-formed-member       | ➕️inserts               |
      | remove-cold-formed-member       | ➖️remove-cold-formed-member       | ➖️removes               |
      | insert-plated-panel             | ➕️insert-plated-panel             | ➕️inserts-panel         |
      | remove-plated-panel             | ➖️remove-plated-panel             | ➖️removes-panel         |
      | insert-silo-shell               | ➕️insert-silo-shell               | ➕️inserts-shell         |
      | remove-silo-shell               | ➖️remove-silo-shell               | ➖️removes-shell         |
      | insert-tension-component        | ➕️insert-tension-component        | ➕️inserts               |
      | remove-tension-component        | ➖️remove-tension-component        | ➖️removes               |
      | insert-bridge-fatigue           | ➕️insert-bridge-fatigue           | ➕️inserts               |
      | remove-bridge-fatigue           | ➖️remove-bridge-fatigue           | ➖️removes               |
      | insert-tower-leg                | ➕️insert-tower-leg                | ➕️inserts-leg           |
      | remove-tower-leg                | ➖️remove-tower-leg                | ➖️removes-leg           |
      | insert-pile                     | ➕️insert-pile                     | ➕️inserts-pile          |
      | remove-pile                     | ➖️remove-pile                     | ➖️removes-pile          |
      | insert-crane-runway             | ➕️insert-crane-runway             | ➕️inserts-runway        |
      | remove-crane-runway             | ➖️remove-crane-runway             | ➖️removes-runway        |

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
      | id                              | dir                               | fixture                 |
      | change-annex                    | 🌍️change-annex                    | 🌐️switches-the-national |
      | update-member-properties        | 📊️update-member-properties        | 🏋️re-grades             |
      | update-fire-inputs              | 🔥️update-fire-inputs              | 🧯️raises-the-fire       |
      | update-cold-formed-inputs       | 🥶️update-cold-formed-inputs       | ✏️sets                  |
      | update-stainless-inputs         | ✨️update-stainless-inputs         | ✏️sets-inputs           |
      | update-plated-inputs            | 🧱️update-plated-inputs            | 📈️makes-plate           |
      | update-silo-shell-inputs        | 🛢️update-silo-shell-inputs        | ✏️sets                  |
      | update-bolt-inputs              | 🔩️update-bolt-inputs              | ✏️sets-inputs           |
      | update-weld-inputs              | 🧲️update-weld-inputs              | ✏️sets-inputs           |
      | update-fatigue-inputs           | 🔁️update-fatigue-inputs           | 🔁️drops-detail          |
      | update-through-thickness-inputs | ↕️update-through-thickness-inputs | ✏️sets                  |
      | update-tension-component-inputs | 🪢️update-tension-component-inputs | ✏️new                   |
      | update-hss-inputs               | ⬜️update-hss-inputs               | ✏️sets-inputs           |
      | update-bridge-inputs            | 🌉️update-bridge-inputs            | 🌉️raises-bridge         |
      | update-tower-inputs             | 🗼️update-tower-inputs             | ✏️sets-inputs           |
      | update-pile-inputs              | 🪵️update-pile-inputs              | ✏️sets-inputs           |
      | update-crane-inputs             | 🏗️update-crane-inputs             | 🏋️widens-crane          |
      | insert-material                 | ➕️insert-material                 | ➕️inserts-material      |
      | remove-material                 | ➖️remove-material                 | ➖️removes-material      |
      | insert-section                  | ➕️insert-section                  | ➕️inserts-section       |
      | remove-section                  | ➖️remove-section                  | ➖️removes-section       |
      | insert-member                   | ➕️insert-member                   | ➕️inserts-member        |
      | remove-member                   | ➖️remove-member                   | ➖️removes-member        |
      | insert-load-case                | ➕️insert-load-case                | ➕️inserts-case          |
      | remove-load-case                | ➖️remove-load-case                | ➖️removes-case          |
      | insert-member-action            | ➕️insert-member-action            | ➕️inserts-action        |
      | remove-member-action            | ➖️remove-member-action            | ➖️removes-action        |
      | insert-joint                    | ➕️insert-joint                    | ➕️inserts-joint         |
      | remove-joint                    | ➖️remove-joint                    | ➖️removes-joint         |
      | insert-fatigue-detail           | ➕️insert-fatigue-detail           | ➕️inserts-detail        |
      | remove-fatigue-detail           | ➖️remove-fatigue-detail           | ➖️removes-detail        |
      | insert-fire-exposure            | ➕️insert-fire-exposure            | ➕️inserts               |
      | remove-fire-exposure            | ➖️remove-fire-exposure            | ➖️removes               |
      | insert-cold-formed-member       | ➕️insert-cold-formed-member       | ➕️inserts               |
      | remove-cold-formed-member       | ➖️remove-cold-formed-member       | ➖️removes               |
      | insert-plated-panel             | ➕️insert-plated-panel             | ➕️inserts-panel         |
      | remove-plated-panel             | ➖️remove-plated-panel             | ➖️removes-panel         |
      | insert-silo-shell               | ➕️insert-silo-shell               | ➕️inserts-shell         |
      | remove-silo-shell               | ➖️remove-silo-shell               | ➖️removes-shell         |
      | insert-tension-component        | ➕️insert-tension-component        | ➕️inserts               |
      | remove-tension-component        | ➖️remove-tension-component        | ➖️removes               |
      | insert-bridge-fatigue           | ➕️insert-bridge-fatigue           | ➕️inserts               |
      | remove-bridge-fatigue           | ➖️remove-bridge-fatigue           | ➖️removes               |
      | insert-tower-leg                | ➕️insert-tower-leg                | ➕️inserts-leg           |
      | remove-tower-leg                | ➖️remove-tower-leg                | ➖️removes-leg           |
      | insert-pile                     | ➕️insert-pile                     | ➕️inserts-pile          |
      | remove-pile                     | ➖️remove-pile                     | ➖️removes-pile          |
      | insert-crane-runway             | ➕️insert-crane-runway             | ➕️inserts-runway        |
      | remove-crane-runway             | ➖️remove-crane-runway             | ➖️removes-runway        |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1993 document from the parsed carrier
    Given the real committed text artifact asset://🔩️high-strength-connection/🔩️high-strength-connection/🗣️.dsl.semio
    And its committed binary twin asset://🔩️high-strength-connection/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
