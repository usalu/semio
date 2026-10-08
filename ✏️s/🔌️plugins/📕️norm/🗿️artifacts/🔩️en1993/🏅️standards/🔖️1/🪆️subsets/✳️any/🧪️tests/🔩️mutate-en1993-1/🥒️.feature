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
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 49 `✅apply` vectors
  cover every kind of the current vocabulary (16 `update`, 16 `insert`, 16 `remove`, 1 `change`) on a high-strength bolted steel connection; each vector's after-snapshot and
  diff were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same
  laws in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list
  position included. `parity` adds that two implementations, in two languages, reach the same document.

  The 16 refusal rows (`⛔dupe`) re-apply a kind's applied mutation to the
  after-snapshot it produced: re-inserting an id the collection now holds must be refused `mutation.duplicate-id`
  (Fatal), re-removing a member that is gone `mutation.target-missing` (Error) and re-setting a value the document
  already has must report `mutation.no-op` (Warning). Both sides must refuse under the committed code and leave the
  document bit-identical; a refusal has nothing to undo, so these rows are `mutate-` only.

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
      | id                              | dir                               | fixture |
      | change-annex                    | 🌍️change-annex                    | ✅apply  |
      | update-member-properties        | 📊️update-member-properties        | ✅apply  |
      | update-fire-inputs              | 🔥️update-fire-inputs              | ✅apply  |
      | update-cold-formed-inputs       | 🥶️update-cold-formed-inputs       | ✅apply  |
      | update-stainless-inputs         | ✨️update-stainless-inputs         | ✅apply  |
      | update-plated-inputs            | 🧱️update-plated-inputs            | ✅apply  |
      | update-silo-shell-inputs        | 🛢️update-silo-shell-inputs        | ✅apply  |
      | update-bolt-inputs              | 🔩️update-bolt-inputs              | ✅apply  |
      | update-weld-inputs              | 🧲️update-weld-inputs              | ✅apply  |
      | update-fatigue-inputs           | 🔁️update-fatigue-inputs           | ✅apply  |
      | update-through-thickness-inputs | ↕️update-through-thickness-inputs | ✅apply  |
      | update-tension-component-inputs | 🪢️update-tension-component-inputs | ✅apply  |
      | update-hss-inputs               | ⬜️update-hss-inputs               | ✅apply  |
      | update-bridge-inputs            | 🌉️update-bridge-inputs            | ✅apply  |
      | update-tower-inputs             | 🗼️update-tower-inputs             | ✅apply  |
      | update-pile-inputs              | 🪵️update-pile-inputs              | ✅apply  |
      | update-crane-inputs             | 🏗️update-crane-inputs             | ✅apply  |
      | insert-material                 | ➕️insert-material                 | ✅apply  |
      | insert-material-dupe            | ➕️insert-material                 | ⛔dupe   |
      | remove-material                 | ➖️remove-material                 | ✅apply  |
      | remove-material-middle-row      | ➖️remove-material                 | 🔬️middle-row |
      | insert-section                  | ➕️insert-section                  | ✅apply  |
      | insert-section-dupe             | ➕️insert-section                  | ⛔dupe   |
      | remove-section                  | ➖️remove-section                  | ✅apply  |
      | remove-section-middle-row       | ➖️remove-section                  | 🔬️middle-row |
      | insert-member                   | ➕️insert-member                   | ✅apply  |
      | insert-member-dupe              | ➕️insert-member                   | ⛔dupe   |
      | remove-member                   | ➖️remove-member                   | ✅apply  |
      | remove-member-middle-row        | ➖️remove-member                   | 🔬️middle-row |
      | insert-load-case                | ➕️insert-load-case                | ✅apply  |
      | insert-load-case-dupe           | ➕️insert-load-case                | ⛔dupe   |
      | remove-load-case                | ➖️remove-load-case                | ✅apply  |
      | remove-load-case-middle-row     | ➖️remove-load-case                | 🔬️middle-row |
      | insert-member-action            | ➕️insert-member-action            | ✅apply  |
      | insert-member-action-dupe       | ➕️insert-member-action            | ⛔dupe   |
      | remove-member-action            | ➖️remove-member-action            | ✅apply  |
      | remove-member-action-middle-row | ➖️remove-member-action            | 🔬️middle-row |
      | insert-joint                    | ➕️insert-joint                    | ✅apply  |
      | insert-joint-dupe               | ➕️insert-joint                    | ⛔dupe   |
      | remove-joint                    | ➖️remove-joint                    | ✅apply  |
      | remove-joint-middle-row         | ➖️remove-joint                    | 🔬️middle-row |
      | insert-fatigue-detail           | ➕️insert-fatigue-detail           | ✅apply  |
      | insert-fatigue-detail-dupe      | ➕️insert-fatigue-detail           | ⛔dupe   |
      | remove-fatigue-detail           | ➖️remove-fatigue-detail           | ✅apply  |
      | remove-fatigue-detail-middle-row | ➖️remove-fatigue-detail           | 🔬️middle-row |
      | insert-fire-exposure            | ➕️insert-fire-exposure            | ✅apply  |
      | insert-fire-exposure-dupe       | ➕️insert-fire-exposure            | ⛔dupe   |
      | remove-fire-exposure            | ➖️remove-fire-exposure            | ✅apply  |
      | remove-fire-exposure-middle-row | ➖️remove-fire-exposure            | 🔬️middle-row |
      | insert-cold-formed-member       | ➕️insert-cold-formed-member       | ✅apply  |
      | insert-cold-formed-member-dupe  | ➕️insert-cold-formed-member       | ⛔dupe   |
      | remove-cold-formed-member       | ➖️remove-cold-formed-member       | ✅apply  |
      | remove-cold-formed-member-middle-row | ➖️remove-cold-formed-member       | 🔬️middle-row |
      | insert-plated-panel             | ➕️insert-plated-panel             | ✅apply  |
      | insert-plated-panel-dupe        | ➕️insert-plated-panel             | ⛔dupe   |
      | remove-plated-panel             | ➖️remove-plated-panel             | ✅apply  |
      | remove-plated-panel-middle-row  | ➖️remove-plated-panel             | 🔬️middle-row |
      | insert-silo-shell               | ➕️insert-silo-shell               | ✅apply  |
      | insert-silo-shell-dupe          | ➕️insert-silo-shell               | ⛔dupe   |
      | remove-silo-shell               | ➖️remove-silo-shell               | ✅apply  |
      | remove-silo-shell-middle-row    | ➖️remove-silo-shell               | 🔬️middle-row |
      | insert-tension-component        | ➕️insert-tension-component        | ✅apply  |
      | insert-tension-component-dupe   | ➕️insert-tension-component        | ⛔dupe   |
      | remove-tension-component        | ➖️remove-tension-component        | ✅apply  |
      | remove-tension-component-middle-row | ➖️remove-tension-component        | 🔬️middle-row |
      | insert-bridge-fatigue           | ➕️insert-bridge-fatigue           | ✅apply  |
      | insert-bridge-fatigue-dupe      | ➕️insert-bridge-fatigue           | ⛔dupe   |
      | remove-bridge-fatigue           | ➖️remove-bridge-fatigue           | ✅apply  |
      | remove-bridge-fatigue-middle-row | ➖️remove-bridge-fatigue           | 🔬️middle-row |
      | insert-tower-leg                | ➕️insert-tower-leg                | ✅apply  |
      | insert-tower-leg-dupe           | ➕️insert-tower-leg                | ⛔dupe   |
      | remove-tower-leg                | ➖️remove-tower-leg                | ✅apply  |
      | remove-tower-leg-middle-row     | ➖️remove-tower-leg                | 🔬️middle-row |
      | insert-pile                     | ➕️insert-pile                     | ✅apply  |
      | insert-pile-dupe                | ➕️insert-pile                     | ⛔dupe   |
      | remove-pile                     | ➖️remove-pile                     | ✅apply  |
      | remove-pile-middle-row          | ➖️remove-pile                     | 🔬️middle-row |
      | insert-crane-runway             | ➕️insert-crane-runway             | ✅apply  |
      | insert-crane-runway-dupe        | ➕️insert-crane-runway             | ⛔dupe   |
      | remove-crane-runway             | ➖️remove-crane-runway             | ✅apply  |
      | remove-crane-runway-middle-row  | ➖️remove-crane-runway             | 🔬️middle-row |

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
      | update-member-properties        | 📊️update-member-properties        | ✅apply  |
      | update-fire-inputs              | 🔥️update-fire-inputs              | ✅apply  |
      | update-cold-formed-inputs       | 🥶️update-cold-formed-inputs       | ✅apply  |
      | update-stainless-inputs         | ✨️update-stainless-inputs         | ✅apply  |
      | update-plated-inputs            | 🧱️update-plated-inputs            | ✅apply  |
      | update-silo-shell-inputs        | 🛢️update-silo-shell-inputs        | ✅apply  |
      | update-bolt-inputs              | 🔩️update-bolt-inputs              | ✅apply  |
      | update-weld-inputs              | 🧲️update-weld-inputs              | ✅apply  |
      | update-fatigue-inputs           | 🔁️update-fatigue-inputs           | ✅apply  |
      | update-through-thickness-inputs | ↕️update-through-thickness-inputs | ✅apply  |
      | update-tension-component-inputs | 🪢️update-tension-component-inputs | ✅apply  |
      | update-hss-inputs               | ⬜️update-hss-inputs               | ✅apply  |
      | update-bridge-inputs            | 🌉️update-bridge-inputs            | ✅apply  |
      | update-tower-inputs             | 🗼️update-tower-inputs             | ✅apply  |
      | update-pile-inputs              | 🪵️update-pile-inputs              | ✅apply  |
      | update-crane-inputs             | 🏗️update-crane-inputs             | ✅apply  |
      | insert-material                 | ➕️insert-material                 | ✅apply  |
      | remove-material                 | ➖️remove-material                 | ✅apply  |
      | insert-section                  | ➕️insert-section                  | ✅apply  |
      | remove-section                  | ➖️remove-section                  | ✅apply  |
      | insert-member                   | ➕️insert-member                   | ✅apply  |
      | remove-member                   | ➖️remove-member                   | ✅apply  |
      | insert-load-case                | ➕️insert-load-case                | ✅apply  |
      | remove-load-case                | ➖️remove-load-case                | ✅apply  |
      | insert-member-action            | ➕️insert-member-action            | ✅apply  |
      | remove-member-action            | ➖️remove-member-action            | ✅apply  |
      | insert-joint                    | ➕️insert-joint                    | ✅apply  |
      | remove-joint                    | ➖️remove-joint                    | ✅apply  |
      | insert-fatigue-detail           | ➕️insert-fatigue-detail           | ✅apply  |
      | remove-fatigue-detail           | ➖️remove-fatigue-detail           | ✅apply  |
      | insert-fire-exposure            | ➕️insert-fire-exposure            | ✅apply  |
      | remove-fire-exposure            | ➖️remove-fire-exposure            | ✅apply  |
      | insert-cold-formed-member       | ➕️insert-cold-formed-member       | ✅apply  |
      | remove-cold-formed-member       | ➖️remove-cold-formed-member       | ✅apply  |
      | insert-plated-panel             | ➕️insert-plated-panel             | ✅apply  |
      | remove-plated-panel             | ➖️remove-plated-panel             | ✅apply  |
      | insert-silo-shell               | ➕️insert-silo-shell               | ✅apply  |
      | remove-silo-shell               | ➖️remove-silo-shell               | ✅apply  |
      | insert-tension-component        | ➕️insert-tension-component        | ✅apply  |
      | remove-tension-component        | ➖️remove-tension-component        | ✅apply  |
      | insert-bridge-fatigue           | ➕️insert-bridge-fatigue           | ✅apply  |
      | remove-bridge-fatigue           | ➖️remove-bridge-fatigue           | ✅apply  |
      | insert-tower-leg                | ➕️insert-tower-leg                | ✅apply  |
      | remove-tower-leg                | ➖️remove-tower-leg                | ✅apply  |
      | insert-pile                     | ➕️insert-pile                     | ✅apply  |
      | remove-pile                     | ➖️remove-pile                     | ✅apply  |
      | insert-crane-runway             | ➕️insert-crane-runway             | ✅apply  |
      | remove-crane-runway             | ➖️remove-crane-runway             | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1993 document from the parsed carrier
    Given the real committed text artifact asset://🔩️high-strength-connection/🔩️high-strength-connection/🗣️.dsl.semio
    And its committed binary twin asset://🔩️high-strength-connection/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
