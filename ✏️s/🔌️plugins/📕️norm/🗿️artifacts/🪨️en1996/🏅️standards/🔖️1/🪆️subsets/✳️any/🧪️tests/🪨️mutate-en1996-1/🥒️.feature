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
  an `applied` vector must move the document and a `no-op` or `rejected` one must leave it bit-identical
  (a rejected one under its committed outcome code), and the mutation followed by its OWN computed
  inverse must restore the before-snapshot exactly. `inverse-` projects BOTH the mutated and the
  restored document, because the restored one is always the before-snapshot and projecting only it
  would make the differential vacuous.

  The four `<kind>-noop` rows re-apply a document-scalar change to its own after-snapshot: the field
  already has the value, so production answers with a `mutation.no-op` warning and an empty diff, and
  both sides must leave the document bit-identical. They have no inverse row, because nothing moved.

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
      | id                                 | dir                                 | fixture |
      | change-concentrated-bearing-length | 📏change-concentrated-bearing-length | ✅apply  |
      | change-slab-span                   | ↔️change-slab-span                  | ✅apply  |
      | change-wall-length                 | ↔️change-wall-length                | ✅apply  |
      | change-wall-height                 | ↕️change-wall-height                | ✅apply  |
      | change-wall-thickness              | ↕️change-wall-thickness             | ✅apply  |
      | change-eccentricity-bottom         | ↗️change-eccentricity-bottom        | ✅apply  |
      | change-eccentricity-top            | ↘️change-eccentricity-top           | ✅apply  |
      | change-phi-infinity                | ♾️change-phi-infinity               | ✅apply  |
      | change-qk-snow                     | ❄️change-qk-snow                    | ✅apply  |
      | insert-concentrated                | ➕️insert-concentrated               | ✅apply  |
      | insert-load-case                   | ➕️insert-load-case                  | ✅apply  |
      | insert-opening                     | ➕️insert-opening                    | ✅apply  |
      | insert-wall                        | ➕️insert-wall                       | ✅apply  |
      | remove-concentrated                | ➖️remove-concentrated               | ✅apply  |
      | remove-load-case                   | ➖️remove-load-case                  | ✅apply  |
      | remove-opening                     | ➖️remove-opening                    | ✅apply  |
      | remove-wall                        | ➖️remove-wall                       | ✅apply  |
      | change-annex                       | 🌍️change-annex                      | ✅apply  |
      | change-annex-noop                  | 🌍️change-annex                      | 🟰noop   |
      | change-qp-wind                     | 🌬️change-qp-wind                    | ✅apply  |
      | change-design-situation            | 🎭️change-design-situation           | ✅apply  |
      | change-design-situation-noop       | 🎭️change-design-situation           | 🟰noop   |
      | change-load-case-situation         | 🎭️change-load-case-situation        | ✅apply  |
      | change-concentrated-force          | 🏋️change-concentrated-force         | ✅apply  |
      | change-gk-slab                     | 🏋️change-gk-slab                    | ✅apply  |
      | change-qk-imposed                  | 🏋️change-qk-imposed                 | ✅apply  |
      | change-is-basement                 | 🏗️change-is-basement                | ✅apply  |
      | change-storeys                     | 🏢️change-storeys                    | ✅apply  |
      | change-storeys-noop                | 🏢️change-storeys                    | 🟰noop   |
      | change-masonry-class               | 🏭️change-masonry-class              | ✅apply  |
      | change-masonry-class-noop          | 🏭️change-masonry-class              | 🟰noop   |
      | change-imposed-category            | 🏷️change-imposed-category           | ✅apply  |
      | change-wall-label-de               | 🏷️change-wall-label-de              | ✅apply  |
      | change-wall-label-en               | 🏷️change-wall-label-en              | ✅apply  |
      | change-exposure                    | 💧️change-exposure                   | ✅apply  |
      | change-concentrated-bearing-area   | 📐️change-concentrated-bearing-area  | ✅apply  |
      | change-slab-bearing-depth          | 📐️change-slab-bearing-depth         | ✅apply  |
      | change-tributary-area              | 📐️change-tributary-area             | ✅apply  |
      | change-fire-rei                    | 🔥️change-fire-rei                   | ✅apply  |
      | change-as-horizontal               | 🔩change-as-horizontal               | ✅apply  |
      | change-as-vertical                 | 🔩change-as-vertical                 | ✅apply  |
      | change-f-yd                        | 🔩change-f-yd                        | ✅apply  |
      | change-reinforced                  | 🔩change-reinforced                  | ✅apply  |
      | change-bed-joint-thickness         | 🥪️change-bed-joint-thickness        | ✅apply  |
      | change-fm                          | 🧈change-fm                          | ✅apply  |
      | change-mortar-class                | 🧈change-mortar-class                | ✅apply  |
      | change-mortar-type                 | 🧈change-mortar-type                 | ✅apply  |
      | change-c-pe                        | 🧮change-c-pe                        | ✅apply  |
      | change-density                     | 🧱change-density                     | ✅apply  |
      | change-support-sides               | 🧱change-support-sides               | ✅apply  |
      | change-unit-fb                     | 🧱change-unit-fb                     | ✅apply  |
      | change-unit-group                  | 🧱change-unit-group                  | ✅apply  |
      | change-unit-height                 | 🧱change-unit-height                 | ✅apply  |
      | change-unit-length                 | 🧱change-unit-length                 | ✅apply  |
      | change-unit-material               | 🧱change-unit-material               | ✅apply  |
      | change-unit-width                  | 🧱change-unit-width                  | ✅apply  |
      | change-wall-type                   | 🧱change-wall-type                   | ✅apply  |
      | change-mu                          | 🧲️change-mu                         | ✅apply  |
      | change-opening-height              | 🪟change-opening-height              | ✅apply  |
      | change-opening-sill                | 🪟change-opening-sill                | ✅apply  |
      | change-opening-width               | 🪟change-opening-width               | ✅apply  |
      | change-hk-earth                    | 🪨change-hk-earth                    | ✅apply  |

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
      | id                                 | dir                                 | fixture |
      | change-concentrated-bearing-length | 📏change-concentrated-bearing-length | ✅apply  |
      | change-slab-span                   | ↔️change-slab-span                  | ✅apply  |
      | change-wall-length                 | ↔️change-wall-length                | ✅apply  |
      | change-wall-height                 | ↕️change-wall-height                | ✅apply  |
      | change-wall-thickness              | ↕️change-wall-thickness             | ✅apply  |
      | change-eccentricity-bottom         | ↗️change-eccentricity-bottom        | ✅apply  |
      | change-eccentricity-top            | ↘️change-eccentricity-top           | ✅apply  |
      | change-phi-infinity                | ♾️change-phi-infinity               | ✅apply  |
      | change-qk-snow                     | ❄️change-qk-snow                    | ✅apply  |
      | insert-concentrated                | ➕️insert-concentrated               | ✅apply  |
      | insert-load-case                   | ➕️insert-load-case                  | ✅apply  |
      | insert-opening                     | ➕️insert-opening                    | ✅apply  |
      | insert-wall                        | ➕️insert-wall                       | ✅apply  |
      | remove-concentrated                | ➖️remove-concentrated               | ✅apply  |
      | remove-load-case                   | ➖️remove-load-case                  | ✅apply  |
      | remove-opening                     | ➖️remove-opening                    | ✅apply  |
      | remove-wall                        | ➖️remove-wall                       | ✅apply  |
      | change-annex                       | 🌍️change-annex                      | ✅apply  |
      | change-qp-wind                     | 🌬️change-qp-wind                    | ✅apply  |
      | change-design-situation            | 🎭️change-design-situation           | ✅apply  |
      | change-load-case-situation         | 🎭️change-load-case-situation        | ✅apply  |
      | change-concentrated-force          | 🏋️change-concentrated-force         | ✅apply  |
      | change-gk-slab                     | 🏋️change-gk-slab                    | ✅apply  |
      | change-qk-imposed                  | 🏋️change-qk-imposed                 | ✅apply  |
      | change-is-basement                 | 🏗️change-is-basement                | ✅apply  |
      | change-storeys                     | 🏢️change-storeys                    | ✅apply  |
      | change-masonry-class               | 🏭️change-masonry-class              | ✅apply  |
      | change-imposed-category            | 🏷️change-imposed-category           | ✅apply  |
      | change-wall-label-de               | 🏷️change-wall-label-de              | ✅apply  |
      | change-wall-label-en               | 🏷️change-wall-label-en              | ✅apply  |
      | change-exposure                    | 💧️change-exposure                   | ✅apply  |
      | change-concentrated-bearing-area   | 📐️change-concentrated-bearing-area  | ✅apply  |
      | change-slab-bearing-depth          | 📐️change-slab-bearing-depth         | ✅apply  |
      | change-tributary-area              | 📐️change-tributary-area             | ✅apply  |
      | change-fire-rei                    | 🔥️change-fire-rei                   | ✅apply  |
      | change-as-horizontal               | 🔩change-as-horizontal               | ✅apply  |
      | change-as-vertical                 | 🔩change-as-vertical                 | ✅apply  |
      | change-f-yd                        | 🔩change-f-yd                        | ✅apply  |
      | change-reinforced                  | 🔩change-reinforced                  | ✅apply  |
      | change-bed-joint-thickness         | 🥪️change-bed-joint-thickness        | ✅apply  |
      | change-fm                          | 🧈change-fm                          | ✅apply  |
      | change-mortar-class                | 🧈change-mortar-class                | ✅apply  |
      | change-mortar-type                 | 🧈change-mortar-type                 | ✅apply  |
      | change-c-pe                        | 🧮change-c-pe                        | ✅apply  |
      | change-density                     | 🧱change-density                     | ✅apply  |
      | change-support-sides               | 🧱change-support-sides               | ✅apply  |
      | change-unit-fb                     | 🧱change-unit-fb                     | ✅apply  |
      | change-unit-group                  | 🧱change-unit-group                  | ✅apply  |
      | change-unit-height                 | 🧱change-unit-height                 | ✅apply  |
      | change-unit-length                 | 🧱change-unit-length                 | ✅apply  |
      | change-unit-material               | 🧱change-unit-material               | ✅apply  |
      | change-unit-width                  | 🧱change-unit-width                  | ✅apply  |
      | change-wall-type                   | 🧱change-wall-type                   | ✅apply  |
      | change-mu                          | 🧲️change-mu                         | ✅apply  |
      | change-opening-height              | 🪟change-opening-height              | ✅apply  |
      | change-opening-sill                | 🪟change-opening-sill                | ✅apply  |
      | change-opening-width               | 🪟change-opening-width               | ✅apply  |
      | change-hk-earth                    | 🪨change-hk-earth                    | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1996 document from the parsed carrier
    Given the real committed text artifact asset://🧱️loadbearing-wall/🧱️loadbearing-wall/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
