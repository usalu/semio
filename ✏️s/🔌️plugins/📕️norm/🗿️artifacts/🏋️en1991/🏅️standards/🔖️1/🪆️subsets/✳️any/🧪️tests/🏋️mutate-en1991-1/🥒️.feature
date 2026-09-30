@capability-en1991-1-mutate
@oracle-en1991-1-python-independent
@comparison-ordered-json-v1
@mutations-en1991-1-any
Feature: Apply every typed EN 1991 mutation against an independent Python implementation
  `s.norm.en1991` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1991` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1991Mutation` vocabulary. The second producer a differential
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
  before-snapshot is the committed `⚠️multi-fail-noncompliant` example — the one EN 1991 example that
  carries a member in every collection, the vehicle impact case included — and its after-snapshot,
  diff and outcome are production dispatch's answer, held by the crate's own vector law and judged
  here by the second implementation. All eighty kinds APPLY and move the document.

  The field set is the most HETEROGENEOUS in the plugin: site, snow, wind, building geometry, thermal,
  fire, execution-stage, bridge-traffic, crane and silo scalars sit side by side at the document root,
  five index-addressed setters write one field of a member record, and five ordered collections take
  `insert-`/`remove-`. The reading risk is the index-addressed half: `change-self-weight-assumed-gk`
  names a collection no spelling of its noun reaches (`selfWeightElements`), and
  `change-accidental-assumed-force` writes a field one record below the member (the case's impact
  record). The engine resolves both by the generic rule — the one collection whose members carry the
  named field, and the shallowest field of that name inside the member — never by a per-kind mapping.
  Each side then asserts the same three laws in role — the applied document must BE the committed
  after-snapshot; an `applied` vector must move the document; and the mutation followed by its OWN
  computed inverse must restore the before-snapshot exactly. What `parity` adds on top is the only
  thing a single implementation can never provide: that two implementations, in two languages,
  written from one written specification, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document, so every row projects a value only
  its own kind produces.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `🖼️assets/🏢de-office-compliant/🏢de-office-compliant/🗣️.dsl.semio` — the compliant office example,
  whose binary twin the Rust side also decodes. The carrier has no published grammar: the committed
  `📖️.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so the two
  implementations are compared at the envelope preamble, the ordered `key=value` fields and the
  digest and length of what each re-emitted — never at a carrier-token-to-enum mapping this
  repository nowhere states.

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
      | id                                            | dir                                            | fixture      |
      | change-annex                                  | 🌍change-annex                                  | 🌍en          |
      | change-snow-zone                              | 🗺️change-snow-zone                             | ⛄zone-3      |
      | change-altitude                               | ❄change-altitude                               | 🗻480-m       |
      | change-en-sk                                  | ❄️change-en-sk                                 | ⛄1250-pa     |
      | change-exceptional-snow-north-german-lowlands | 🏔change-exceptional-snow-north-german-lowlands | ⛄on          |
      | change-wind-zone                              | 🪁change-wind-zone                              | 🪁zone-3      |
      | change-en-vb                                  | 🌬change-en-vb                                  | 💨27-5-m-s    |
      | change-terrain-category                       | 🏞️change-terrain-category                      | 🌳class-3     |
      | change-mixed-terrain-upwind                   | 🧭change-mixed-terrain-upwind                   | 🧭class-1     |
      | change-mixed-terrain-distance                 | 📏change-mixed-terrain-distance                 | 📏1500-m      |
      | change-orography-factor                       | 📐change-orography-factor                       | 📐1-15        |
      | change-coast-or-island                        | 🏝️change-coast-or-island                       | 🌊coast       |
      | change-air-density                            | 🏢change-air-density                            | 💨1-225       |
      | change-height                                 | 🧱change-height                                 | 🧱24-m        |
      | change-width                                  | 🏠change-width                                  | 🏠18-5-m      |
      | change-depth                                  | 💨change-depth                                  | 🏢30-m        |
      | change-assumed-delta-t                        | 🌡change-assumed-delta-t                        | 📈15-k        |
      | change-construction-activity                  | 🔥change-construction-activity                  | 🧰formwork    |
      | change-assumed-construction-qk                | ⚙change-assumed-construction-qk                | 👷2-kpa       |
      | change-structure-kind                         | 🌉change-structure-kind                         | 🌉bridge      |
      | change-bridge-lane                            | 🌉change-bridge-lane                            | 🚦3-lanes     |
      | change-bridge-span                            | 🏗change-bridge-span                            | 🌉36-m        |
      | change-bridge-lane-width                      | ↔️change-bridge-lane-width                     | 📏3-5-m       |
      | change-assumed-bridge-tandem                  | 🌾change-assumed-bridge-tandem                  | 🚚600-kn      |
      | change-assumed-bridge-udl                     | 🛣change-assumed-bridge-udl                     | 🚦9-kpa       |
      | change-assumed-bridge-lm2                     | 🚛change-assumed-bridge-lm2                     | 🚛400-kn      |
      | change-assumed-bridge-footway                 | 🚶change-assumed-bridge-footway                 | 🚶5-kpa       |
      | change-storey-count                           | 🏙change-storey-count                           | 🏢5-storeys   |
      | change-t-max                                  | 🌡change-t-max                                  | 🌞39-c        |
      | change-t-min                                  | 🧊change-t-min                                  | 🧊minus-28-c  |
      | change-initial-temperature                    | 🕰change-initial-temperature                    | ⏰15-c        |
      | change-thermal-element-type                   | 🏗change-thermal-element-type                   | 🌉bridge2     |
      | change-thermal-bridge-type                    | 🌉change-thermal-bridge-type                    | 🌁type-2      |
      | change-linear-temperature-gradient            | 📏change-linear-temperature-gradient            | 📈5-k         |
      | change-fire-mode                              | 🔥change-fire-mode                              | 🔥parametric  |
      | change-fire-curve                             | 📉change-fire-curve                             | 📉hydrocarbon |
      | change-fire-duration                          | ⏱change-fire-duration                          | ⌛90-min      |
      | change-assumed-gas-temperature                | ♨change-assumed-gas-temperature                | 🔥1300-k      |
      | change-assumed-h-net                          | 🔆change-assumed-h-net                          | 🔆35-kw-m2    |
      | change-fire-compartment-area                  | 🗺change-fire-compartment-area                  | 📐150-m2      |
      | change-fire-compartment-height                | 📐change-fire-compartment-height                | 📐3-5-m       |
      | change-fire-opening-factor                    | 🪟change-fire-opening-factor                    | 🪟0-06        |
      | change-fire-thermal-inertia                   | 🧱change-fire-thermal-inertia                   | 🧱1500        |
      | change-fire-occupancy                         | 🏢change-fire-occupancy                         | 🏬shopping    |
      | change-fire-load-density-qf                   | ⛽change-fire-load-density-qf                   | ⛽600-mj      |
      | change-assumed-qf-d                           | 🔋change-assumed-qf-d                           | 🔋511-mj      |
      | change-assumed-bridge-lm3                     | 🚛change-assumed-bridge-lm3                     | 🚛600-kn      |
      | change-assumed-bridge-lm4                     | 👥change-assumed-bridge-lm4                     | 👥5-kpa       |
      | change-bridge-load-group                      | 📦change-bridge-load-group                      | 📦gr1b        |
      | change-crane-claimed                          | 🏗️change-crane-claimed                         | 🚫withdrawn   |
      | change-crane-class                            | 💥change-crane-class                            | 💥hc3         |
      | change-hoist-class                            | ➕change-hoist-class                            | 🪝hc4         |
      | change-hoisting-speed                         | ⏫change-hoisting-speed                         | ⏫1-25-m-s    |
      | change-assumed-crane-wheel                    | ➖change-assumed-crane-wheel                    | 🛞75-kn       |
      | change-assumed-crane-horizontal               | ↔️change-assumed-crane-horizontal              | 🧲7-5-kn      |
      | change-silo-claimed                           | 🏭change-silo-claimed                           | 🚫withdrawn   |
      | change-silo-kind                              | ⚖change-silo-kind                              | 💧tank        |
      | change-silo-bulk-density                      | 🌾change-silo-bulk-density                      | 🌾9-kn-m3     |
      | change-silo-height                            | 🏷change-silo-height                            | 📏18-m        |
      | change-silo-hydraulic-radius                  | ⭕change-silo-hydraulic-radius                  | ⭕2-25-m      |
      | change-silo-mu                                | 🔎change-silo-mu                                | 🔎0-5         |
      | change-silo-k                                 | ⚙️change-silo-k                                | 🔩0-55        |
      | change-assumed-silo-pressure                  | 🌀change-assumed-silo-pressure                  | 🌀8-kpa       |
      | change-assumed-silo-patch                     | 📦change-assumed-silo-patch                     | 📦1-5-kpa     |
      | change-assumed-silo-wall-friction             | 🧱change-assumed-silo-wall-friction             | 🧱2-kpa       |
      | change-floor-assumed-qk                       | 🏢change-floor-assumed-qk                       | 🏢3-kpa       |
      | change-self-weight-assumed-gk                 | 🚧change-self-weight-assumed-gk                 | 🚧5-kpa       |
      | change-roof-assumed-sk                        | 🌨️change-roof-assumed-sk                       | ⛄900-pa      |
      | change-wind-face-assumed-wp                   | 🛡change-wind-face-assumed-wp                   | 🪟750-pa      |
      | change-accidental-assumed-force               | 🚗change-accidental-assumed-force               | 🚗150-kn      |
      | insert-floors                                 | ➕️insert-floors                                | ➕archive     |
      | remove-floors                                 | ➖️remove-floors                                | ➖office      |
      | insert-self-weight-elements                   | ➕️insert-self-weight-elements                  | ➕screed      |
      | remove-self-weight-elements                   | ➖️remove-self-weight-elements                  | ➖slab        |
      | insert-roofs                                  | ➕️insert-roofs                                 | ➕annex-roof  |
      | remove-roofs                                  | ➖️remove-roofs                                 | ➖main-roof   |
      | insert-wind-faces                             | ➕️insert-wind-faces                            | ➕leeward     |
      | remove-wind-faces                             | ➖️remove-wind-faces                            | ➖windward    |
      | insert-accidental-cases                       | ➕️insert-accidental-cases                      | ➕explosion   |
      | remove-accidental-cases                       | ➖️remove-accidental-cases                      | ➖impact      |

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
      | id                                            | dir                                            | fixture      |
      | change-annex                                  | 🌍change-annex                                  | 🌍en          |
      | change-snow-zone                              | 🗺️change-snow-zone                             | ⛄zone-3      |
      | change-altitude                               | ❄change-altitude                               | 🗻480-m       |
      | change-en-sk                                  | ❄️change-en-sk                                 | ⛄1250-pa     |
      | change-exceptional-snow-north-german-lowlands | 🏔change-exceptional-snow-north-german-lowlands | ⛄on          |
      | change-wind-zone                              | 🪁change-wind-zone                              | 🪁zone-3      |
      | change-en-vb                                  | 🌬change-en-vb                                  | 💨27-5-m-s    |
      | change-terrain-category                       | 🏞️change-terrain-category                      | 🌳class-3     |
      | change-mixed-terrain-upwind                   | 🧭change-mixed-terrain-upwind                   | 🧭class-1     |
      | change-mixed-terrain-distance                 | 📏change-mixed-terrain-distance                 | 📏1500-m      |
      | change-orography-factor                       | 📐change-orography-factor                       | 📐1-15        |
      | change-coast-or-island                        | 🏝️change-coast-or-island                       | 🌊coast       |
      | change-air-density                            | 🏢change-air-density                            | 💨1-225       |
      | change-height                                 | 🧱change-height                                 | 🧱24-m        |
      | change-width                                  | 🏠change-width                                  | 🏠18-5-m      |
      | change-depth                                  | 💨change-depth                                  | 🏢30-m        |
      | change-assumed-delta-t                        | 🌡change-assumed-delta-t                        | 📈15-k        |
      | change-construction-activity                  | 🔥change-construction-activity                  | 🧰formwork    |
      | change-assumed-construction-qk                | ⚙change-assumed-construction-qk                | 👷2-kpa       |
      | change-structure-kind                         | 🌉change-structure-kind                         | 🌉bridge      |
      | change-bridge-lane                            | 🌉change-bridge-lane                            | 🚦3-lanes     |
      | change-bridge-span                            | 🏗change-bridge-span                            | 🌉36-m        |
      | change-bridge-lane-width                      | ↔️change-bridge-lane-width                     | 📏3-5-m       |
      | change-assumed-bridge-tandem                  | 🌾change-assumed-bridge-tandem                  | 🚚600-kn      |
      | change-assumed-bridge-udl                     | 🛣change-assumed-bridge-udl                     | 🚦9-kpa       |
      | change-assumed-bridge-lm2                     | 🚛change-assumed-bridge-lm2                     | 🚛400-kn      |
      | change-assumed-bridge-footway                 | 🚶change-assumed-bridge-footway                 | 🚶5-kpa       |
      | change-storey-count                           | 🏙change-storey-count                           | 🏢5-storeys   |
      | change-t-max                                  | 🌡change-t-max                                  | 🌞39-c        |
      | change-t-min                                  | 🧊change-t-min                                  | 🧊minus-28-c  |
      | change-initial-temperature                    | 🕰change-initial-temperature                    | ⏰15-c        |
      | change-thermal-element-type                   | 🏗change-thermal-element-type                   | 🌉bridge2     |
      | change-thermal-bridge-type                    | 🌉change-thermal-bridge-type                    | 🌁type-2      |
      | change-linear-temperature-gradient            | 📏change-linear-temperature-gradient            | 📈5-k         |
      | change-fire-mode                              | 🔥change-fire-mode                              | 🔥parametric  |
      | change-fire-curve                             | 📉change-fire-curve                             | 📉hydrocarbon |
      | change-fire-duration                          | ⏱change-fire-duration                          | ⌛90-min      |
      | change-assumed-gas-temperature                | ♨change-assumed-gas-temperature                | 🔥1300-k      |
      | change-assumed-h-net                          | 🔆change-assumed-h-net                          | 🔆35-kw-m2    |
      | change-fire-compartment-area                  | 🗺change-fire-compartment-area                  | 📐150-m2      |
      | change-fire-compartment-height                | 📐change-fire-compartment-height                | 📐3-5-m       |
      | change-fire-opening-factor                    | 🪟change-fire-opening-factor                    | 🪟0-06        |
      | change-fire-thermal-inertia                   | 🧱change-fire-thermal-inertia                   | 🧱1500        |
      | change-fire-occupancy                         | 🏢change-fire-occupancy                         | 🏬shopping    |
      | change-fire-load-density-qf                   | ⛽change-fire-load-density-qf                   | ⛽600-mj      |
      | change-assumed-qf-d                           | 🔋change-assumed-qf-d                           | 🔋511-mj      |
      | change-assumed-bridge-lm3                     | 🚛change-assumed-bridge-lm3                     | 🚛600-kn      |
      | change-assumed-bridge-lm4                     | 👥change-assumed-bridge-lm4                     | 👥5-kpa       |
      | change-bridge-load-group                      | 📦change-bridge-load-group                      | 📦gr1b        |
      | change-crane-claimed                          | 🏗️change-crane-claimed                         | 🚫withdrawn   |
      | change-crane-class                            | 💥change-crane-class                            | 💥hc3         |
      | change-hoist-class                            | ➕change-hoist-class                            | 🪝hc4         |
      | change-hoisting-speed                         | ⏫change-hoisting-speed                         | ⏫1-25-m-s    |
      | change-assumed-crane-wheel                    | ➖change-assumed-crane-wheel                    | 🛞75-kn       |
      | change-assumed-crane-horizontal               | ↔️change-assumed-crane-horizontal              | 🧲7-5-kn      |
      | change-silo-claimed                           | 🏭change-silo-claimed                           | 🚫withdrawn   |
      | change-silo-kind                              | ⚖change-silo-kind                              | 💧tank        |
      | change-silo-bulk-density                      | 🌾change-silo-bulk-density                      | 🌾9-kn-m3     |
      | change-silo-height                            | 🏷change-silo-height                            | 📏18-m        |
      | change-silo-hydraulic-radius                  | ⭕change-silo-hydraulic-radius                  | ⭕2-25-m      |
      | change-silo-mu                                | 🔎change-silo-mu                                | 🔎0-5         |
      | change-silo-k                                 | ⚙️change-silo-k                                | 🔩0-55        |
      | change-assumed-silo-pressure                  | 🌀change-assumed-silo-pressure                  | 🌀8-kpa       |
      | change-assumed-silo-patch                     | 📦change-assumed-silo-patch                     | 📦1-5-kpa     |
      | change-assumed-silo-wall-friction             | 🧱change-assumed-silo-wall-friction             | 🧱2-kpa       |
      | change-floor-assumed-qk                       | 🏢change-floor-assumed-qk                       | 🏢3-kpa       |
      | change-self-weight-assumed-gk                 | 🚧change-self-weight-assumed-gk                 | 🚧5-kpa       |
      | change-roof-assumed-sk                        | 🌨️change-roof-assumed-sk                       | ⛄900-pa      |
      | change-wind-face-assumed-wp                   | 🛡change-wind-face-assumed-wp                   | 🪟750-pa      |
      | change-accidental-assumed-force               | 🚗change-accidental-assumed-force               | 🚗150-kn      |
      | insert-floors                                 | ➕️insert-floors                                | ➕archive     |
      | remove-floors                                 | ➖️remove-floors                                | ➖office      |
      | insert-self-weight-elements                   | ➕️insert-self-weight-elements                  | ➕screed      |
      | remove-self-weight-elements                   | ➖️remove-self-weight-elements                  | ➖slab        |
      | insert-roofs                                  | ➕️insert-roofs                                 | ➕annex-roof  |
      | remove-roofs                                  | ➖️remove-roofs                                 | ➖main-roof   |
      | insert-wind-faces                             | ➕️insert-wind-faces                            | ➕leeward     |
      | remove-wind-faces                             | ➖️remove-wind-faces                            | ➖windward    |
      | insert-accidental-cases                       | ➕️insert-accidental-cases                      | ➕explosion   |
      | remove-accidental-cases                       | ➖️remove-accidental-cases                      | ➖impact      |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1991 document from the parsed carrier
    Given the real committed text artifact asset://🏢de-office-compliant/🏢de-office-compliant/🗣️.dsl.semio
    And its committed binary twin asset://🏢de-office-compliant/🏢de-office-compliant/📦️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
