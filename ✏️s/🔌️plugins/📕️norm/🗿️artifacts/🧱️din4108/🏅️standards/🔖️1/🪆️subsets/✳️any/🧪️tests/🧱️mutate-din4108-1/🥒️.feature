@capability-din4108-1-mutate
@oracle-din4108-1-python-independent
@comparison-ordered-json-v1
@mutations-din4108-1-any
Feature: Apply every typed DIN 4108 mutation against an independent Python implementation
  `s.norm.din4108` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `din4108` distribution, and the nearest real packages (`structuralcodes`,
  `concreteproperties`, `anastruct`) implement design-code FORMULAE and speak no interchange format, so
  none of them could be authoritative over `Din4108Mutation`. The second producer a differential comparison
  needs is therefore a second IMPLEMENTATION: `semio_norm_vocabulary`, imported by `🐍️.py` beside this
  file, reads every one of the 43 kinds from the naming mechanic (`new<Field>` sets the field its
  name spells) and the addressing convention (`<entity>Index` positions and `<entity>Id` native keys
  descend, in wire order, to the record the verb acts inside; inverses are computed from the base and
  are empty when the target is missing). It imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a declared `shared://` fixture the Rust producer wrote and the Python engine independently
  reached, so neither side holds a transcription that could drift. The 43 kinds edit a thermal-
  protection building at three depths — document scalars, zone/element/bridge fields addressed by
  native id, and window and layer fields addressed inside their zone or element (`{zoneId, windowId}`,
  `{elementId, index}`) — plus the insert, remove and reorder kinds of every collection, whose
  inverses must restore the build-up exactly, position included.

  Each side asserts the same laws in role — the applied document must BE the committed after-snapshot,
  an `applied` vector must move the document and a `rejected` one must leave it bit-identical, and the
  mutation followed by its OWN computed inverse must restore the before-snapshot exactly. `inverse-`
  projects BOTH the mutated and the restored document, because the restored one is always the
  before-snapshot and projecting only it would make the differential vacuous.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed example
  `asset://🎬️demo/🗣️.dsl.semio`. The carrier has no published grammar (the subset's `📖️.grammar.semio` is the
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
      | id                                 | dir                                 | fixture                           |
      | change-climate-zone                | 🌦️change-climate-zone               | 🗺️moves-to-zone-3                 |
      | change-usage                       | 🗂️change-usage                      | 🏢️sets-nonresidential             |
      | change-t-int-c                     | 🌡️change-t-int-c                    | 🌡️sets-t-int-to-21-point-5        |
      | change-rh-int                      | 💧️change-rh-int                     | 💧️raises-rh-to-0-point-55         |
      | change-airtightness-n50            | 💨️change-airtightness-n50           | 💨️tightens-n50-to-1-point-0       |
      | change-has-mechanical-ventilation  | 🌬️change-has-mechanical-ventilation | 🌬️disables-mechanical-ventilation |
      | change-bb2-details-conform         | ✅️change-bb2-details-conform        | ❌️declares-bb2-non-conforming     |
      | insert-zone                        | ➕️insert-zone                       | ➕️appends-extra-zone              |
      | remove-zone                        | ➖️remove-zone                       | 🚫️removes-first-zone              |
      | change-zone-floor-area             | 📐️change-zone-floor-area            | 📐️sets-floor-area-to-90           |
      | change-zone-heaviness              | 🧱change-zone-heaviness              | 🧱sets-heaviness-light             |
      | change-zone-night-ventilation      | 🌙change-zone-night-ventilation      | 🌙sets-night-ventilation-high      |
      | insert-zone-window                 | 🪟insert-zone-window                 | 🪟appends-extra-window             |
      | remove-zone-window                 | 🚫️remove-zone-window                | 🚫️removes-east-window             |
      | change-zone-window-area            | 📏change-zone-window-area            | 📏grows-south-window               |
      | change-zone-window-g-value         | ☀️change-zone-window-g-value        | ☀️sets-g-value-0-point-6          |
      | change-zone-window-shading-fc      | ⛱️change-zone-window-shading-fc     | ⛱️tightens-shading-fc             |
      | insert-element                     | 🏠️insert-element                    | 🏠️appends-extra-wall              |
      | remove-element                     | 🚫️remove-element                    | 🚫️removes-first-element           |
      | change-element-area                | 📐️change-element-area               | 📐️grows-wall-area                 |
      | change-element-adjacent            | ↔️change-element-adjacent           | ↔️sets-adjacent-unheated          |
      | change-element-kind                | 🏷️change-element-kind               | 🏷️retags-as-opaque-frame          |
      | insert-layer                       | ➕️insert-layer                      | ➕️inserts-layer-into-wall         |
      | remove-layer                       | ➖️remove-layer                      | ➖️removes-eps-layer               |
      | reorder-layers                     | 🔀️reorder-layers                    | 🧭️swaps-first-two-layers          |
      | change-layer-thickness             | 📏️change-layer-thickness            | 📏️thickens-eps-to-0-point-2       |
      | change-layer-lambda                | 🌡change-layer-lambda                | 🌡️sets-eps-lambda                 |
      | change-layer-mu                    | 💧change-layer-mu                    | 💧raises-eps-mu                    |
      | change-layer-material-id           | 🧽️change-layer-material-id          | 🧽️retags-eps-material             |
      | insert-thermal-bridge              | 🌉️insert-thermal-bridge             | 🌉️appends-extra-bridge            |
      | remove-thermal-bridge              | 🧊remove-thermal-bridge              | 🧊removes-first-bridge             |
      | change-thermal-bridge-psi          | 🔘change-thermal-bridge-psi          | 🔘lowers-psi                       |
      | change-thermal-bridge-length       | ↔️change-thermal-bridge-length      | ↔️shortens-bridge                 |
      | change-element-orientation-deg     | 🧭change-element-orientation-deg     | 🧭turns-north-wall-south           |
      | change-element-inclination-deg     | 📐change-element-inclination-deg     | 📐tilts-north-wall-to-45-degrees   |
      | change-element-delta-ug            | 📈️change-element-delta-ug           | 📈️raises-glazing-delta-ug         |
      | change-element-delta-uf            | 📈️change-element-delta-uf           | 📈️raises-frame-delta-uf           |
      | change-element-delta-ur            | 📈️change-element-delta-ur           | 📈️raises-roof-delta-ur            |
      | change-thermal-bridge-bb2-type     | 🏷change-thermal-bridge-bb2-type     | 🏷️reclassifies-reveal-bridge      |
      | change-zone-window-orientation     | 🧭change-zone-window-orientation     | 🧭turns-south-window-west          |
      | change-zone-window-inclination-deg | 📐change-zone-window-inclination-deg | 📐tilts-south-window-to-60-degrees |
      | change-layer-application-type      | 🏷️change-layer-application-type     | 🏷️reclassifies-eps-as-wab         |
      | change-layer-compressive-class     | 🏷️change-layer-compressive-class    | 🏷️raises-eps-compressive-class    |

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
      | id                                 | dir                                 | fixture                           |
      | change-climate-zone                | 🌦️change-climate-zone               | 🗺️moves-to-zone-3                 |
      | change-usage                       | 🗂️change-usage                      | 🏢️sets-nonresidential             |
      | change-t-int-c                     | 🌡️change-t-int-c                    | 🌡️sets-t-int-to-21-point-5        |
      | change-rh-int                      | 💧️change-rh-int                     | 💧️raises-rh-to-0-point-55         |
      | change-airtightness-n50            | 💨️change-airtightness-n50           | 💨️tightens-n50-to-1-point-0       |
      | change-has-mechanical-ventilation  | 🌬️change-has-mechanical-ventilation | 🌬️disables-mechanical-ventilation |
      | change-bb2-details-conform         | ✅️change-bb2-details-conform        | ❌️declares-bb2-non-conforming     |
      | insert-zone                        | ➕️insert-zone                       | ➕️appends-extra-zone              |
      | remove-zone                        | ➖️remove-zone                       | 🚫️removes-first-zone              |
      | change-zone-floor-area             | 📐️change-zone-floor-area            | 📐️sets-floor-area-to-90           |
      | change-zone-heaviness              | 🧱change-zone-heaviness              | 🧱sets-heaviness-light             |
      | change-zone-night-ventilation      | 🌙change-zone-night-ventilation      | 🌙sets-night-ventilation-high      |
      | insert-zone-window                 | 🪟insert-zone-window                 | 🪟appends-extra-window             |
      | remove-zone-window                 | 🚫️remove-zone-window                | 🚫️removes-east-window             |
      | change-zone-window-area            | 📏change-zone-window-area            | 📏grows-south-window               |
      | change-zone-window-g-value         | ☀️change-zone-window-g-value        | ☀️sets-g-value-0-point-6          |
      | change-zone-window-shading-fc      | ⛱️change-zone-window-shading-fc     | ⛱️tightens-shading-fc             |
      | insert-element                     | 🏠️insert-element                    | 🏠️appends-extra-wall              |
      | remove-element                     | 🚫️remove-element                    | 🚫️removes-first-element           |
      | change-element-area                | 📐️change-element-area               | 📐️grows-wall-area                 |
      | change-element-adjacent            | ↔️change-element-adjacent           | ↔️sets-adjacent-unheated          |
      | change-element-kind                | 🏷️change-element-kind               | 🏷️retags-as-opaque-frame          |
      | insert-layer                       | ➕️insert-layer                      | ➕️inserts-layer-into-wall         |
      | remove-layer                       | ➖️remove-layer                      | ➖️removes-eps-layer               |
      | reorder-layers                     | 🔀️reorder-layers                    | 🧭️swaps-first-two-layers          |
      | change-layer-thickness             | 📏️change-layer-thickness            | 📏️thickens-eps-to-0-point-2       |
      | change-layer-lambda                | 🌡change-layer-lambda                | 🌡️sets-eps-lambda                 |
      | change-layer-mu                    | 💧change-layer-mu                    | 💧raises-eps-mu                    |
      | change-layer-material-id           | 🧽️change-layer-material-id          | 🧽️retags-eps-material             |
      | insert-thermal-bridge              | 🌉️insert-thermal-bridge             | 🌉️appends-extra-bridge            |
      | remove-thermal-bridge              | 🧊remove-thermal-bridge              | 🧊removes-first-bridge             |
      | change-thermal-bridge-psi          | 🔘change-thermal-bridge-psi          | 🔘lowers-psi                       |
      | change-thermal-bridge-length       | ↔️change-thermal-bridge-length      | ↔️shortens-bridge                 |
      | change-element-orientation-deg     | 🧭change-element-orientation-deg     | 🧭turns-north-wall-south           |
      | change-element-inclination-deg     | 📐change-element-inclination-deg     | 📐tilts-north-wall-to-45-degrees   |
      | change-element-delta-ug            | 📈️change-element-delta-ug           | 📈️raises-glazing-delta-ug         |
      | change-element-delta-uf            | 📈️change-element-delta-uf           | 📈️raises-frame-delta-uf           |
      | change-element-delta-ur            | 📈️change-element-delta-ur           | 📈️raises-roof-delta-ur            |
      | change-thermal-bridge-bb2-type     | 🏷change-thermal-bridge-bb2-type     | 🏷️reclassifies-reveal-bridge      |
      | change-zone-window-orientation     | 🧭change-zone-window-orientation     | 🧭turns-south-window-west          |
      | change-zone-window-inclination-deg | 📐change-zone-window-inclination-deg | 📐tilts-south-window-to-60-degrees |
      | change-layer-application-type      | 🏷️change-layer-application-type     | 🏷️reclassifies-eps-as-wab         |
      | change-layer-compressive-class     | 🏷️change-layer-compressive-class    | 🏷️raises-eps-compressive-class    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed DIN 4108 document from the parsed carrier
    Given the real committed text artifact asset://🎬️demo/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
