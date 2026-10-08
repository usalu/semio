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
  an `applied` vector must move the document and a `no-op` or `rejected` one must leave it bit-identical
  (a rejected one under its committed outcome code), and the mutation followed by its OWN computed
  inverse must restore the before-snapshot exactly. `inverse-` projects BOTH the mutated and the
  restored document, because the restored one is always the before-snapshot and projecting only it
  would make the differential vacuous.

  Further rows witness every outcome class a leaf declares. `<kind>-dupe` re-applies a zone, element or
  bridge insert to its own after-snapshot, whose id is already held — a `mutation.duplicate-id` refusal;
  `<kind>-clamp` asks the same insert for a position past its list's end: both sides insert last, where
  the canonical append landed, and production reports it as a `mutation.clamped` warning;
  `<kind>-noop` re-applies a document-scalar change to its own after-snapshot — a `mutation.no-op`
  warning with an empty diff; `<kind>-rule` asks the indoor-climate changes for a value outside the bound
  their leaf payload schema states (θ_i above absolute zero, φ_i within [0, 1], n₅₀ ≥ 0) — a
  `mutation.invariant` refusal. Refused and no-op rows leave the document bit-identical, so none of them
  has an inverse row.

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
      | id                                     | dir                                 | fixture |
      | change-climate-zone                    | 🌦️change-climate-zone               | ✅apply  |
      | change-climate-zone-noop               | 🌦️change-climate-zone               | 🟰noop   |
      | change-usage                           | 🗂️change-usage                      | ✅apply  |
      | change-usage-noop                      | 🗂️change-usage                      | 🟰noop   |
      | change-t-int-c                         | 🌡️change-t-int-c                    | ✅apply  |
      | change-t-int-c-rule                    | 🌡️change-t-int-c                    | 🚫rule   |
      | change-t-int-c-noop                    | 🌡️change-t-int-c                    | 🟰noop   |
      | change-rh-int                          | 💧️change-rh-int                     | ✅apply  |
      | change-rh-int-rule                     | 💧️change-rh-int                     | 🚫rule   |
      | change-rh-int-noop                     | 💧️change-rh-int                     | 🟰noop   |
      | change-airtightness-n50                | 💨️change-airtightness-n50           | ✅apply  |
      | change-airtightness-n50-rule           | 💨️change-airtightness-n50           | 🚫rule   |
      | change-airtightness-n50-noop           | 💨️change-airtightness-n50           | 🟰noop   |
      | change-has-mechanical-ventilation      | 💨change-has-mechanical-ventilation  | ✅apply  |
      | change-has-mechanical-ventilation-noop | 💨change-has-mechanical-ventilation  | 🟰noop   |
      | change-bb2-details-conform             | ✅️change-bb2-details-conform        | ✅apply  |
      | change-bb2-details-conform-noop        | ✅️change-bb2-details-conform        | 🟰noop   |
      | insert-zone                            | ➕️insert-zone                       | ✅apply  |
      | insert-zone-dupe                       | ➕️insert-zone                       | ⛔dupe   |
      | insert-zone-clamp                      | ➕️insert-zone                       | 📏clamp  |
      | remove-zone                            | ➖️remove-zone                       | ✅apply  |
      | remove-zone-middle-row                 | ➖️remove-zone                       | 🔬️middle-row |
      | change-zone-floor-area                 | 📐️change-zone-floor-area            | ✅apply  |
      | change-zone-heaviness                  | 🧱change-zone-heaviness              | ✅apply  |
      | change-zone-night-ventilation          | 🌙change-zone-night-ventilation      | ✅apply  |
      | insert-zone-window                     | 🪟insert-zone-window                 | ✅apply  |
      | remove-zone-window                     | 🚫️remove-zone-window                | ✅apply  |
      | remove-zone-window-middle-row          | 🚫️remove-zone-window                | 🔬️middle-row |
      | change-zone-window-area                | 📏change-zone-window-area            | ✅apply  |
      | change-zone-window-g-value             | ☀️change-zone-window-g-value        | ✅apply  |
      | change-zone-window-shading-fc          | ⛱️change-zone-window-shading-fc     | ✅apply  |
      | insert-element                         | 🏠️insert-element                    | ✅apply  |
      | insert-element-dupe                    | 🏠️insert-element                    | ⛔dupe   |
      | insert-element-clamp                   | 🏠️insert-element                    | 📏clamp  |
      | remove-element                         | 🚫️remove-element                    | ✅apply  |
      | remove-element-middle-row              | 🚫️remove-element                    | 🔬️middle-row |
      | change-element-area                    | 📐️change-element-area               | ✅apply  |
      | change-element-adjacent                | ↔️change-element-adjacent           | ✅apply  |
      | change-element-kind                    | 🏷️change-element-kind               | ✅apply  |
      | insert-layer                           | ➕️insert-layer                      | ✅apply  |
      | remove-layer                           | ➖️remove-layer                      | ✅apply  |
      | remove-layer-middle-row                | ➖️remove-layer                      | 🔬️middle-row |
      | reorder-layers                         | 🔀️reorder-layers                    | ✅apply  |
      | reorder-layers-middle-row              | 🔀️reorder-layers                    | 🔬️middle-row |
      | change-layer-thickness                 | 📏️change-layer-thickness            | ✅apply  |
      | change-layer-lambda                    | 🌡️change-layer-lambda                | ✅apply  |
      | change-layer-mu                        | 💧change-layer-mu                    | ✅apply  |
      | change-layer-material-id               | 🧽️change-layer-material-id          | ✅apply  |
      | insert-thermal-bridge                  | 🌉️insert-thermal-bridge             | ✅apply  |
      | insert-thermal-bridge-dupe             | 🌉️insert-thermal-bridge             | ⛔dupe   |
      | insert-thermal-bridge-clamp            | 🌉️insert-thermal-bridge             | 📏clamp  |
      | remove-thermal-bridge                  | 🧊remove-thermal-bridge              | ✅apply  |
      | remove-thermal-bridge-middle-row       | 🧊remove-thermal-bridge              | 🔬️middle-row |
      | change-thermal-bridge-psi              | 🔘change-thermal-bridge-psi          | ✅apply  |
      | change-thermal-bridge-length           | ↔️change-thermal-bridge-length      | ✅apply  |
      | change-element-orientation-deg         | 🧭change-element-orientation-deg     | ✅apply  |
      | change-element-inclination-deg         | 📐change-element-inclination-deg     | ✅apply  |
      | change-element-delta-ug                | 📈️change-element-delta-ug           | ✅apply  |
      | change-element-delta-uf                | 📈️change-element-delta-uf           | ✅apply  |
      | change-element-delta-ur                | 📈️change-element-delta-ur           | ✅apply  |
      | change-thermal-bridge-bb2-type         | 🏷️change-thermal-bridge-bb2-type     | ✅apply  |
      | change-zone-window-orientation         | 🧭change-zone-window-orientation     | ✅apply  |
      | change-zone-window-inclination-deg     | 📐change-zone-window-inclination-deg | ✅apply  |
      | change-layer-application-type          | 🏷️change-layer-application-type     | ✅apply  |
      | change-layer-compressive-class         | 🏷️change-layer-compressive-class    | ✅apply  |

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
      | change-climate-zone                | 🌦️change-climate-zone               | ✅apply  |
      | change-usage                       | 🗂️change-usage                      | ✅apply  |
      | change-t-int-c                     | 🌡️change-t-int-c                    | ✅apply  |
      | change-rh-int                      | 💧️change-rh-int                     | ✅apply  |
      | change-airtightness-n50            | 💨️change-airtightness-n50           | ✅apply  |
      | change-has-mechanical-ventilation  | 💨change-has-mechanical-ventilation  | ✅apply  |
      | change-bb2-details-conform         | ✅️change-bb2-details-conform        | ✅apply  |
      | insert-zone                        | ➕️insert-zone                       | ✅apply  |
      | remove-zone                        | ➖️remove-zone                       | ✅apply  |
      | change-zone-floor-area             | 📐️change-zone-floor-area            | ✅apply  |
      | change-zone-heaviness              | 🧱change-zone-heaviness              | ✅apply  |
      | change-zone-night-ventilation      | 🌙change-zone-night-ventilation      | ✅apply  |
      | insert-zone-window                 | 🪟insert-zone-window                 | ✅apply  |
      | remove-zone-window                 | 🚫️remove-zone-window                | ✅apply  |
      | change-zone-window-area            | 📏change-zone-window-area            | ✅apply  |
      | change-zone-window-g-value         | ☀️change-zone-window-g-value        | ✅apply  |
      | change-zone-window-shading-fc      | ⛱️change-zone-window-shading-fc     | ✅apply  |
      | insert-element                     | 🏠️insert-element                    | ✅apply  |
      | remove-element                     | 🚫️remove-element                    | ✅apply  |
      | change-element-area                | 📐️change-element-area               | ✅apply  |
      | change-element-adjacent            | ↔️change-element-adjacent           | ✅apply  |
      | change-element-kind                | 🏷️change-element-kind               | ✅apply  |
      | insert-layer                       | ➕️insert-layer                      | ✅apply  |
      | remove-layer                       | ➖️remove-layer                      | ✅apply  |
      | reorder-layers                     | 🔀️reorder-layers                    | ✅apply  |
      | change-layer-thickness             | 📏️change-layer-thickness            | ✅apply  |
      | change-layer-lambda                | 🌡️change-layer-lambda                | ✅apply  |
      | change-layer-mu                    | 💧change-layer-mu                    | ✅apply  |
      | change-layer-material-id           | 🧽️change-layer-material-id          | ✅apply  |
      | insert-thermal-bridge              | 🌉️insert-thermal-bridge             | ✅apply  |
      | remove-thermal-bridge              | 🧊remove-thermal-bridge              | ✅apply  |
      | change-thermal-bridge-psi          | 🔘change-thermal-bridge-psi          | ✅apply  |
      | change-thermal-bridge-length       | ↔️change-thermal-bridge-length      | ✅apply  |
      | change-element-orientation-deg     | 🧭change-element-orientation-deg     | ✅apply  |
      | change-element-inclination-deg     | 📐change-element-inclination-deg     | ✅apply  |
      | change-element-delta-ug            | 📈️change-element-delta-ug           | ✅apply  |
      | change-element-delta-uf            | 📈️change-element-delta-uf           | ✅apply  |
      | change-element-delta-ur            | 📈️change-element-delta-ur           | ✅apply  |
      | change-thermal-bridge-bb2-type     | 🏷️change-thermal-bridge-bb2-type     | ✅apply  |
      | change-zone-window-orientation     | 🧭change-zone-window-orientation     | ✅apply  |
      | change-zone-window-inclination-deg | 📐change-zone-window-inclination-deg | ✅apply  |
      | change-layer-application-type      | 🏷️change-layer-application-type     | ✅apply  |
      | change-layer-compressive-class     | 🏷️change-layer-compressive-class    | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed DIN 4108 document from the parsed carrier
    Given the real committed text artifact asset://🎬️demo/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
