@capability-en1996-1-mutate
@oracle-en1996-1-python-independent
@comparison-ordered-json-v1
@mutations-en1996-1-any
Feature: Apply every typed EN 1996 mutation against an independent Python implementation
  `s.norm.en1996` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1996` distribution, and the nearest real packages (`structuralcodes`,
  `concreteproperties`, `anastruct`) implement design-code FORMULAE and speak no interchange format, so
  none of them could be authoritative over `En1996Mutation`. The second producer a differential comparison
  needs is therefore a second IMPLEMENTATION: `semio_norm_vocabulary`, imported by `🐍️.py` beside this
  file, reads every one of the 58 kinds from the naming mechanic (`new<Field>` sets the field its
  name spells) and the addressing convention (`<entity>Index` positions and `<entity>Id` native keys
  descend, in wire order, to the record the verb acts inside; inverses are computed from the base and
  are empty when the target is missing). It imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a declared `shared://` fixture the Rust producer wrote and the Python engine independently
  reached, so neither side holds a transcription that could drift. The 58 kinds address a masonry
  building at four depths — document scalars (`change-annex`, `change-storeys`), wall fields by position
  (`change-wall-height {index}`), opening and load-case fields inside a wall (`{wallIndex, index}`)
  and concentrated loads inside a load case (`{wallIndex, loadCaseIndex, index}`). That nested
  position addressing is where a second reading written from the addressing convention alone can land
  on the wrong collection, and it is the part of this subset the differential actually tests.

  Each side asserts the same laws in role — the applied document must BE the committed after-snapshot,
  an `applied` vector must move the document and a `rejected` one must leave it bit-identical, and the
  mutation followed by its OWN computed inverse must restore the before-snapshot exactly. `inverse-`
  projects BOTH the mutated and the restored document, because the restored one is always the
  before-snapshot and projecting only it would make the differential vacuous.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed example
  `asset://🧱️loadbearing-wall/🧱️loadbearing-wall/🗣️.dsl.semio`. The carrier has no published grammar (the subset's `📖️.grammar.semio` is the
  repository-wide `payload = OCTET+` placeholder), so the two implementations are compared on the
  envelope preamble, the ordered `key=value` fields, the table rows as written, and the digest and
  length of what each side re-emitted — never on a mapping from carrier tokens to the JSON snapshot's
  enum spellings, which is stated nowhere.

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
      | id                                 | dir                                  | fixture                                              |
      | change-concentrated-bearing-length | ↔️change-concentrated-bearing-length | ↔️applies-change-concentrated-bearing-length         |
      | change-slab-span                   | ↔️change-slab-span                   | ↔️applies-change-slab-span                           |
      | change-wall-length                 | ↔️change-wall-length                 | ↔️applies-change-wall-length                         |
      | change-wall-height                 | ↕️change-wall-height                 | ↕️shortens-first-wall                                |
      | change-wall-thickness              | ↕️change-wall-thickness              | ↕️thickens-first-wall                                |
      | change-eccentricity-bottom         | ↗️change-eccentricity-bottom         | ↗️applies-change-eccentricity-bottom                 |
      | change-eccentricity-top            | ↘️change-eccentricity-top            | ↘️applies-change-eccentricity-top                    |
      | change-phi-infinity                | ♾️change-phi-infinity                | ♾️applies-change-phi-infinity                        |
      | change-qk-snow                     | ❄️change-qk-snow                     | ❄️applies-change-qk-snow                             |
      | insert-concentrated                | ➕️insert-concentrated                | ➕️applies-insert-concentrated                        |
      | insert-load-case                   | ➕️insert-load-case                   | ➕️applies-insert-load-case                           |
      | insert-opening                     | ➕️insert-opening                     | ➕️applies-insert-opening                             |
      | insert-wall                        | ➕️insert-wall                        | ➕️inserts-a-wall                                     |
      | remove-concentrated                | ➖️remove-concentrated                | ➖️applies-remove-concentrated                        |
      | remove-load-case                   | ➖️remove-load-case                   | ➖️applies-remove-load-case                           |
      | remove-opening                     | ➖️remove-opening                     | ➖️applies-remove-opening                             |
      | remove-wall                        | ➖️remove-wall                        | ➖️removes-first-wall                                 |
      | change-annex                       | 🌍️change-annex                       | 🌍️switches-annex-to-en                               |
      | change-qp-wind                     | 🌬️change-qp-wind                     | 🌬️applies-change-qp-wind                             |
      | change-design-situation            | 🎭️change-design-situation            | 🌋️switches-the-design-situation-to-seismic           |
      | change-load-case-situation         | 🎭️change-load-case-situation         | 🎭️applies-change-load-case-situation                 |
      | change-concentrated-force          | 🏋️change-concentrated-force          | 🏋️applies-change-concentrated-force                  |
      | change-gk-slab                     | 🏋️change-gk-slab                     | 🏋️applies-change-gk-slab                             |
      | change-qk-imposed                  | 🏋️change-qk-imposed                  | 🏋️applies-change-qk-imposed                          |
      | change-is-basement                 | 🏗️change-is-basement                 | 🏗️applies-change-is-basement                         |
      | change-storeys                     | 🏢️change-storeys                     | 🏢️applies-change-storeys                             |
      | change-masonry-class               | 🏭️change-masonry-class               | 🏭️applies-change-masonry-class                       |
      | change-imposed-category            | 🏷️change-imposed-category            | 🏷️applies-change-imposed-category                    |
      | change-wall-label-de               | 🏷️change-wall-label-de               | 🏷️applies-change-wall-label-de                       |
      | change-wall-label-en               | 🏷️change-wall-label-en               | 🏷️applies-change-wall-label-en                       |
      | change-exposure                    | 💧️change-exposure                    | 💧️applies-change-exposure                            |
      | change-concentrated-bearing-area   | 📐️change-concentrated-bearing-area   | 📐️applies-change-concentrated-bearing-area           |
      | change-slab-bearing-depth          | 📐️change-slab-bearing-depth          | 📐️applies-change-slab-bearing-depth                  |
      | change-tributary-area              | 📐️change-tributary-area              | 📐️applies-change-tributary-area                      |
      | change-fire-rei                    | 🔥️change-fire-rei                    | 🔥️applies-change-fire-rei                            |
      | change-as-horizontal               | 🔩change-as-horizontal                | 🔩applies-change-as-horizontal                        |
      | change-as-vertical                 | 🔩change-as-vertical                  | 🔩applies-change-as-vertical                          |
      | change-f-yd                        | 🔩change-f-yd                         | 🔩applies-change-f-yd                                 |
      | change-reinforced                  | 🔩change-reinforced                   | 🔩applies-change-reinforced                           |
      | change-bed-joint-thickness         | 🥪️change-bed-joint-thickness         | 🥪️applies-change-bed-joint-thickness                 |
      | change-fm                          | 🧈change-fm                           | 🧈applies-change-fm                                   |
      | change-mortar-class                | 🧈change-mortar-class                 | 🧈upgrades-mortar-to-m20                              |
      | change-mortar-type                 | 🧈change-mortar-type                  | 🧈applies-change-mortar-type                          |
      | change-c-pe                        | 🧮change-c-pe                         | 🧮applies-change-c-pe                                 |
      | change-density                     | 🧱change-density                      | 🧱applies-change-density                              |
      | change-support-sides               | 🧱change-support-sides                | 🧱sets-four-sided-support                             |
      | change-unit-fb                     | 🧱change-unit-fb                      | 🧱raises-unit-strength                                |
      | change-unit-group                  | 🧱change-unit-group                   | 🧱applies-change-unit-group                           |
      | change-unit-height                 | 🧱change-unit-height                  | 🧱applies-change-unit-height                          |
      | change-unit-length                 | 🧱change-unit-length                  | 🧱applies-change-unit-length                          |
      | change-unit-material               | 🧱change-unit-material                | 🧱applies-change-unit-material                        |
      | change-unit-width                  | 🧱change-unit-width                   | 🧱applies-change-unit-width                           |
      | change-wall-type                   | 🧱change-wall-type                    | 🧱applies-change-wall-type                            |
      | change-mu                          | 🧲️change-mu                          | 🧲️raises-the-bed-joint-friction-coefficient-to-0-625 |
      | change-opening-height              | 🪟change-opening-height               | 🪟applies-change-opening-height                       |
      | change-opening-sill                | 🪟change-opening-sill                 | 🪟applies-change-opening-sill                         |
      | change-opening-width               | 🪟change-opening-width                | 🪟applies-change-opening-width                        |
      | change-hk-earth                    | 🪨change-hk-earth                     | 🪨applies-change-hk-earth                             |

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
      | id                                 | dir                                  | fixture                                              |
      | change-concentrated-bearing-length | ↔️change-concentrated-bearing-length | ↔️applies-change-concentrated-bearing-length         |
      | change-slab-span                   | ↔️change-slab-span                   | ↔️applies-change-slab-span                           |
      | change-wall-length                 | ↔️change-wall-length                 | ↔️applies-change-wall-length                         |
      | change-wall-height                 | ↕️change-wall-height                 | ↕️shortens-first-wall                                |
      | change-wall-thickness              | ↕️change-wall-thickness              | ↕️thickens-first-wall                                |
      | change-eccentricity-bottom         | ↗️change-eccentricity-bottom         | ↗️applies-change-eccentricity-bottom                 |
      | change-eccentricity-top            | ↘️change-eccentricity-top            | ↘️applies-change-eccentricity-top                    |
      | change-phi-infinity                | ♾️change-phi-infinity                | ♾️applies-change-phi-infinity                        |
      | change-qk-snow                     | ❄️change-qk-snow                     | ❄️applies-change-qk-snow                             |
      | insert-concentrated                | ➕️insert-concentrated                | ➕️applies-insert-concentrated                        |
      | insert-load-case                   | ➕️insert-load-case                   | ➕️applies-insert-load-case                           |
      | insert-opening                     | ➕️insert-opening                     | ➕️applies-insert-opening                             |
      | insert-wall                        | ➕️insert-wall                        | ➕️inserts-a-wall                                     |
      | remove-concentrated                | ➖️remove-concentrated                | ➖️applies-remove-concentrated                        |
      | remove-load-case                   | ➖️remove-load-case                   | ➖️applies-remove-load-case                           |
      | remove-opening                     | ➖️remove-opening                     | ➖️applies-remove-opening                             |
      | remove-wall                        | ➖️remove-wall                        | ➖️removes-first-wall                                 |
      | change-annex                       | 🌍️change-annex                       | 🌍️switches-annex-to-en                               |
      | change-qp-wind                     | 🌬️change-qp-wind                     | 🌬️applies-change-qp-wind                             |
      | change-design-situation            | 🎭️change-design-situation            | 🌋️switches-the-design-situation-to-seismic           |
      | change-load-case-situation         | 🎭️change-load-case-situation         | 🎭️applies-change-load-case-situation                 |
      | change-concentrated-force          | 🏋️change-concentrated-force          | 🏋️applies-change-concentrated-force                  |
      | change-gk-slab                     | 🏋️change-gk-slab                     | 🏋️applies-change-gk-slab                             |
      | change-qk-imposed                  | 🏋️change-qk-imposed                  | 🏋️applies-change-qk-imposed                          |
      | change-is-basement                 | 🏗️change-is-basement                 | 🏗️applies-change-is-basement                         |
      | change-storeys                     | 🏢️change-storeys                     | 🏢️applies-change-storeys                             |
      | change-masonry-class               | 🏭️change-masonry-class               | 🏭️applies-change-masonry-class                       |
      | change-imposed-category            | 🏷️change-imposed-category            | 🏷️applies-change-imposed-category                    |
      | change-wall-label-de               | 🏷️change-wall-label-de               | 🏷️applies-change-wall-label-de                       |
      | change-wall-label-en               | 🏷️change-wall-label-en               | 🏷️applies-change-wall-label-en                       |
      | change-exposure                    | 💧️change-exposure                    | 💧️applies-change-exposure                            |
      | change-concentrated-bearing-area   | 📐️change-concentrated-bearing-area   | 📐️applies-change-concentrated-bearing-area           |
      | change-slab-bearing-depth          | 📐️change-slab-bearing-depth          | 📐️applies-change-slab-bearing-depth                  |
      | change-tributary-area              | 📐️change-tributary-area              | 📐️applies-change-tributary-area                      |
      | change-fire-rei                    | 🔥️change-fire-rei                    | 🔥️applies-change-fire-rei                            |
      | change-as-horizontal               | 🔩change-as-horizontal                | 🔩applies-change-as-horizontal                        |
      | change-as-vertical                 | 🔩change-as-vertical                  | 🔩applies-change-as-vertical                          |
      | change-f-yd                        | 🔩change-f-yd                         | 🔩applies-change-f-yd                                 |
      | change-reinforced                  | 🔩change-reinforced                   | 🔩applies-change-reinforced                           |
      | change-bed-joint-thickness         | 🥪️change-bed-joint-thickness         | 🥪️applies-change-bed-joint-thickness                 |
      | change-fm                          | 🧈change-fm                           | 🧈applies-change-fm                                   |
      | change-mortar-class                | 🧈change-mortar-class                 | 🧈upgrades-mortar-to-m20                              |
      | change-mortar-type                 | 🧈change-mortar-type                  | 🧈applies-change-mortar-type                          |
      | change-c-pe                        | 🧮change-c-pe                         | 🧮applies-change-c-pe                                 |
      | change-density                     | 🧱change-density                      | 🧱applies-change-density                              |
      | change-support-sides               | 🧱change-support-sides                | 🧱sets-four-sided-support                             |
      | change-unit-fb                     | 🧱change-unit-fb                      | 🧱raises-unit-strength                                |
      | change-unit-group                  | 🧱change-unit-group                   | 🧱applies-change-unit-group                           |
      | change-unit-height                 | 🧱change-unit-height                  | 🧱applies-change-unit-height                          |
      | change-unit-length                 | 🧱change-unit-length                  | 🧱applies-change-unit-length                          |
      | change-unit-material               | 🧱change-unit-material                | 🧱applies-change-unit-material                        |
      | change-unit-width                  | 🧱change-unit-width                   | 🧱applies-change-unit-width                           |
      | change-wall-type                   | 🧱change-wall-type                    | 🧱applies-change-wall-type                            |
      | change-mu                          | 🧲️change-mu                          | 🧲️raises-the-bed-joint-friction-coefficient-to-0-625 |
      | change-opening-height              | 🪟change-opening-height               | 🪟applies-change-opening-height                       |
      | change-opening-sill                | 🪟change-opening-sill                 | 🪟applies-change-opening-sill                         |
      | change-opening-width               | 🪟change-opening-width                | 🪟applies-change-opening-width                        |
      | change-hk-earth                    | 🪨change-hk-earth                     | 🪨applies-change-hk-earth                             |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1996 document from the parsed carrier
    Given the real committed text artifact asset://🧱️loadbearing-wall/🧱️loadbearing-wall/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
