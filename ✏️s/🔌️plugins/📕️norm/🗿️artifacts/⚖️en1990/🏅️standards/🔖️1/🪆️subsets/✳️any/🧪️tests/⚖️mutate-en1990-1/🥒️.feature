@capability-en1990-1-mutate
@oracle-en1990-1-python-independent
@comparison-ordered-json-v1
@mutations-en1990-1-any
Feature: Apply every typed EN 1990 mutation against an independent Python implementation
  `s.norm.en1990` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1990` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1990Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION: the norm plugin's one independent Python
  engine, which `🐍️.py` beside this file imports and feeds with this subset's own committed
  catalog. The engine is written from the repository's own written specification of what a semantic
  mutation means — `📓️taxonomy.md`'s verb table, naming mechanics ("New-value fields are
  `new_<field>`") and addressing convention ("Inverse always computed from `base`", "Missing target
  ⇒ `inverse` returns `Vec::new()`"), and `📓️derivation-rules.md`'s shape rules. It imports nothing
  from the Rust it judges: the document field a `new*` argument names is resolved by normalised
  spelling against the document's own keys, never from a table copied out of `🧬️mutations/**`.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a declared `shared://` fixture, one vector per kind, and each vector is that leaf's
  committed wire witness. Its `🦠️mutation` payload is written by hand against the leaf schema; its
  before-snapshot is the committed `🏢️accidental-seismic-compliant` example — the one EN 1990 example
  that carries a member in every action collection, the accidental and the seismic one included — and
  its after-snapshot, diff and outcome are production dispatch's answer, held by the crate's own
  vector law and judged here by the second implementation. All thirty kinds APPLY and move the
  document.

  The vocabulary has three shapes. Eleven root scalars (annex, project id, site altitude, consequence
  and reliability class, design working life, reference period, supervision and inspection level,
  computed β) take `change-`; seven whole collections (permanent, variable, accidental and seismic
  actions, members, bridge serviceability records, member effects) take a whole-list `change-`; and
  six ordered collections take `insert-`/`remove-` by position. Each side then asserts the same three
  laws in role — the applied document must BE the committed after-snapshot; an `applied` vector must
  move the document; and the mutation followed by its OWN computed inverse must restore the
  before-snapshot exactly. The two sides may undo differently — production undoes an `insert-` with a
  whole-list `change-`, the reference with the paired `remove-` — and both must still land on the same
  before-snapshot. What `parity` adds on top is the only thing a single implementation can never
  provide: that two implementations, in two languages, written from one written specification, reach
  the same document.

  `inverse-` projects BOTH the mutated and the restored document, so every row projects a value only
  its own kind produces.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `🖼️assets/🏢️high-consequence-office/🏢️high-consequence-office/🗣️.dsl.semio` — a named CC3 office
  case, not a generic demo — whose binary twin the Rust side also decodes. The carrier has no published
  grammar: the committed `📖️.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so
  identity is compared at the envelope preamble, the ordered `key=value` fields and the digest and
  length of the re-emitted bytes, never at an inferred mapping from carrier tokens onto the JSON
  snapshot's enum spellings.

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
      | id                                  | dir                                  | fixture         |
      | change-annex                        | 🌍️change-annex                       | 🌍en             |
      | change-project-id                   | 🏷️change-project-id                  | 📛office-tower-b |
      | change-altitude-m                   | ⛰️change-altitude-m                  | 🗻950-m          |
      | change-consequence-class            | ⚠️change-consequence-class           | 🚨cc3            |
      | change-reliability-class            | 🎯change-reliability-class            | 🎯rc3            |
      | change-design-working-life-category | 📅change-design-working-life-category | ⏳cat-5          |
      | change-design-working-life-years    | 📆change-design-working-life-years    | 📆100-y          |
      | change-reference-period-years       | ⏱️change-reference-period-years      | ⌛1-y            |
      | change-supervision-level            | 👁️change-supervision-level           | 👀dsl3           |
      | change-inspection-level             | 🔍change-inspection-level             | 🔍il3            |
      | change-beta-computed                | 📐change-beta-computed                | 📐4-3            |
      | change-permanents                   | ⚓️change-permanents                  | ⚓95-kn          |
      | change-variables                    | 🏋️change-variables                   | ⛄adds-snow      |
      | change-accidentals                  | 💥change-accidentals                  | 💥75-kn          |
      | change-seismics                     | 🌋️change-seismics                    | 🌋class-iii      |
      | change-members                      | 🏗️change-members                     | 💪300-kn         |
      | change-bridge-sls                   | 🌉change-bridge-sls                   | 🌉deck-check     |
      | change-effects                      | 🔗change-effects                      | 🔗half-wind      |
      | remove-effect                       | ✂️remove-effect                      | 🔌e-1            |
      | remove-member                       | 🪚remove-member                       | 🪚beam-b1        |
      | remove-seismic                      | 🕳️remove-seismic                     | ❌e-1            |
      | remove-accidental                   | 🧯remove-accidental                   | 🧯impact         |
      | remove-variable                     | 📤remove-variable                     | 📤wind           |
      | remove-permanent                    | ➖remove-permanent                    | ➖g-inf          |
      | insert-effect                       | 📎insert-effect                       | 📎office-half    |
      | insert-member                       | 🔩insert-member                       | 🔩beam-b2        |
      | insert-seismic                      | 🌋insert-seismic                      | 🌋class-iv       |
      | insert-accidental                   | 💣insert-accidental                   | 💣explosion      |
      | insert-variable                     | 📥insert-variable                     | 📥snow           |
      | insert-permanent                    | ➕insert-permanent                    | ➕finishes       |

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
      | id                                  | dir                                  | fixture         |
      | change-annex                        | 🌍️change-annex                       | 🌍en             |
      | change-project-id                   | 🏷️change-project-id                  | 📛office-tower-b |
      | change-altitude-m                   | ⛰️change-altitude-m                  | 🗻950-m          |
      | change-consequence-class            | ⚠️change-consequence-class           | 🚨cc3            |
      | change-reliability-class            | 🎯change-reliability-class            | 🎯rc3            |
      | change-design-working-life-category | 📅change-design-working-life-category | ⏳cat-5          |
      | change-design-working-life-years    | 📆change-design-working-life-years    | 📆100-y          |
      | change-reference-period-years       | ⏱️change-reference-period-years      | ⌛1-y            |
      | change-supervision-level            | 👁️change-supervision-level           | 👀dsl3           |
      | change-inspection-level             | 🔍change-inspection-level             | 🔍il3            |
      | change-beta-computed                | 📐change-beta-computed                | 📐4-3            |
      | change-permanents                   | ⚓️change-permanents                  | ⚓95-kn          |
      | change-variables                    | 🏋️change-variables                   | ⛄adds-snow      |
      | change-accidentals                  | 💥change-accidentals                  | 💥75-kn          |
      | change-seismics                     | 🌋️change-seismics                    | 🌋class-iii      |
      | change-members                      | 🏗️change-members                     | 💪300-kn         |
      | change-bridge-sls                   | 🌉change-bridge-sls                   | 🌉deck-check     |
      | change-effects                      | 🔗change-effects                      | 🔗half-wind      |
      | remove-effect                       | ✂️remove-effect                      | 🔌e-1            |
      | remove-member                       | 🪚remove-member                       | 🪚beam-b1        |
      | remove-seismic                      | 🕳️remove-seismic                     | ❌e-1            |
      | remove-accidental                   | 🧯remove-accidental                   | 🧯impact         |
      | remove-variable                     | 📤remove-variable                     | 📤wind           |
      | remove-permanent                    | ➖remove-permanent                    | ➖g-inf          |
      | insert-effect                       | 📎insert-effect                       | 📎office-half    |
      | insert-member                       | 🔩insert-member                       | 🔩beam-b2        |
      | insert-seismic                      | 🌋insert-seismic                      | 🌋class-iv       |
      | insert-accidental                   | 💣insert-accidental                   | 💣explosion      |
      | insert-variable                     | 📥insert-variable                     | 📥snow           |
      | insert-permanent                    | ➕insert-permanent                    | ➕finishes       |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1990 document from the parsed carrier
    Given the real committed text artifact asset://🏢️high-consequence-office/🏢️high-consequence-office/🗣️.dsl.semio
    And its committed binary twin asset://🏢️high-consequence-office/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
