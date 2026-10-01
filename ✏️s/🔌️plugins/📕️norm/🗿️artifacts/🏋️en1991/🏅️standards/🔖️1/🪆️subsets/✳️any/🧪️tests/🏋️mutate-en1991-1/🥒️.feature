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
      | id                                 | dir                                 | fixture |
      | change-annex                       | 🌍change-annex                       | ✅apply  |
      | change-snow-zone                   | 🗺️change-snow-zone                  | ✅apply  |
      | change-altitude                    | ❄change-altitude                    | ✅apply  |
      | change-en-sk                       | ❄️change-en-sk                      | ✅apply  |
      | change-north-german-lowland-snow   | 🏔change-north-german-lowland-snow   | ✅apply  |
      | change-wind-zone                   | 🪁change-wind-zone                   | ✅apply  |
      | change-en-vb                       | 🌬change-en-vb                       | ✅apply  |
      | change-terrain-category            | 🏞️change-terrain-category           | ✅apply  |
      | change-mixed-terrain-upwind        | 🧭change-mixed-terrain-upwind        | ✅apply  |
      | change-mixed-terrain-distance      | 📏change-mixed-terrain-distance      | ✅apply  |
      | change-orography-factor            | 📐change-orography-factor            | ✅apply  |
      | change-coast-or-island             | 🏝️change-coast-or-island            | ✅apply  |
      | change-air-density                 | 🏢change-air-density                 | ✅apply  |
      | change-height                      | 🧱change-height                      | ✅apply  |
      | change-width                       | 🏠change-width                       | ✅apply  |
      | change-depth                       | 💨change-depth                       | ✅apply  |
      | change-assumed-delta-t             | 🌡change-assumed-delta-t             | ✅apply  |
      | change-construction-activity       | 🔥change-construction-activity       | ✅apply  |
      | change-assumed-construction-qk     | ⚙change-assumed-construction-qk     | ✅apply  |
      | change-structure-kind              | 🌉change-structure-kind              | ✅apply  |
      | change-bridge-lane                 | 🌉change-bridge-lane                 | ✅apply  |
      | change-bridge-span                 | 🏗change-bridge-span                 | ✅apply  |
      | change-bridge-lane-width           | ↔️change-bridge-lane-width          | ✅apply  |
      | change-assumed-bridge-tandem       | 🌾change-assumed-bridge-tandem       | ✅apply  |
      | change-assumed-bridge-udl          | 🛣change-assumed-bridge-udl          | ✅apply  |
      | change-assumed-bridge-lm2          | 🚛change-assumed-bridge-lm2          | ✅apply  |
      | change-assumed-bridge-footway      | 🚶change-assumed-bridge-footway      | ✅apply  |
      | change-storey-count                | 🏙change-storey-count                | ✅apply  |
      | change-t-max                       | 🌡change-t-max                       | ✅apply  |
      | change-t-min                       | 🧊change-t-min                       | ✅apply  |
      | change-initial-temperature         | 🕰change-initial-temperature         | ✅apply  |
      | change-thermal-element-type        | 🏗change-thermal-element-type        | ✅apply  |
      | change-thermal-bridge-type         | 🌉change-thermal-bridge-type         | ✅apply  |
      | change-linear-temperature-gradient | 📏change-linear-temperature-gradient | ✅apply  |
      | change-fire-mode                   | 🔥change-fire-mode                   | ✅apply  |
      | change-fire-curve                  | 📉change-fire-curve                  | ✅apply  |
      | change-fire-duration               | ⏱change-fire-duration               | ✅apply  |
      | change-assumed-gas-temperature     | ♨change-assumed-gas-temperature     | ✅apply  |
      | change-assumed-h-net               | 🔆change-assumed-h-net               | ✅apply  |
      | change-fire-compartment-area       | 🗺change-fire-compartment-area       | ✅apply  |
      | change-fire-compartment-height     | 📐change-fire-compartment-height     | ✅apply  |
      | change-fire-opening-factor         | 🪟change-fire-opening-factor         | ✅apply  |
      | change-fire-thermal-inertia        | 🧱change-fire-thermal-inertia        | ✅apply  |
      | change-fire-occupancy              | 🏢change-fire-occupancy              | ✅apply  |
      | change-fire-load-density-qf        | ⛽change-fire-load-density-qf        | ✅apply  |
      | change-assumed-qf-d                | 🔋change-assumed-qf-d                | ✅apply  |
      | change-assumed-bridge-lm3          | 🚛change-assumed-bridge-lm3          | ✅apply  |
      | change-assumed-bridge-lm4          | 👥change-assumed-bridge-lm4          | ✅apply  |
      | change-bridge-load-group           | 📦change-bridge-load-group           | ✅apply  |
      | change-crane-claimed               | 🏗️change-crane-claimed              | ✅apply  |
      | change-crane-class                 | 💥change-crane-class                 | ✅apply  |
      | change-hoist-class                 | ➕change-hoist-class                 | ✅apply  |
      | change-hoisting-speed              | ⏫change-hoisting-speed              | ✅apply  |
      | change-assumed-crane-wheel         | ➖change-assumed-crane-wheel         | ✅apply  |
      | change-assumed-crane-horizontal    | ↔️change-assumed-crane-horizontal   | ✅apply  |
      | change-silo-claimed                | 🏭change-silo-claimed                | ✅apply  |
      | change-silo-kind                   | ⚖change-silo-kind                   | ✅apply  |
      | change-silo-bulk-density           | 🌾change-silo-bulk-density           | ✅apply  |
      | change-silo-height                 | 🏷change-silo-height                 | ✅apply  |
      | change-silo-hydraulic-radius       | ⭕change-silo-hydraulic-radius       | ✅apply  |
      | change-silo-mu                     | 🔎change-silo-mu                     | ✅apply  |
      | change-silo-k                      | ⚙️change-silo-k                     | ✅apply  |
      | change-assumed-silo-pressure       | 🌀change-assumed-silo-pressure       | ✅apply  |
      | change-assumed-silo-patch          | 📦change-assumed-silo-patch          | ✅apply  |
      | change-assumed-silo-wall-friction  | 🧱change-assumed-silo-wall-friction  | ✅apply  |
      | change-floor-assumed-qk            | 🏢change-floor-assumed-qk            | ✅apply  |
      | change-self-weight-assumed-gk      | 🚧change-self-weight-assumed-gk      | ✅apply  |
      | change-roof-assumed-sk             | 🌨️change-roof-assumed-sk            | ✅apply  |
      | change-wind-face-assumed-wp        | 🛡change-wind-face-assumed-wp        | ✅apply  |
      | change-accidental-assumed-force    | 🚗change-accidental-assumed-force    | ✅apply  |
      | insert-floors                      | ➕️insert-floors                     | ✅apply  |
      | remove-floors                      | ➖️remove-floors                     | ✅apply  |
      | insert-self-weight-elements        | ➕️insert-self-weight-elements       | ✅apply  |
      | remove-self-weight-elements        | ➖️remove-self-weight-elements       | ✅apply  |
      | insert-roofs                       | ➕️insert-roofs                      | ✅apply  |
      | remove-roofs                       | ➖️remove-roofs                      | ✅apply  |
      | insert-wind-faces                  | ➕️insert-wind-faces                 | ✅apply  |
      | remove-wind-faces                  | ➖️remove-wind-faces                 | ✅apply  |
      | insert-accidental-cases            | ➕️insert-accidental-cases           | ✅apply  |
      | remove-accidental-cases            | ➖️remove-accidental-cases           | ✅apply  |

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
      | change-annex                       | 🌍change-annex                       | ✅apply  |
      | change-snow-zone                   | 🗺️change-snow-zone                  | ✅apply  |
      | change-altitude                    | ❄change-altitude                    | ✅apply  |
      | change-en-sk                       | ❄️change-en-sk                      | ✅apply  |
      | change-north-german-lowland-snow   | 🏔change-north-german-lowland-snow   | ✅apply  |
      | change-wind-zone                   | 🪁change-wind-zone                   | ✅apply  |
      | change-en-vb                       | 🌬change-en-vb                       | ✅apply  |
      | change-terrain-category            | 🏞️change-terrain-category           | ✅apply  |
      | change-mixed-terrain-upwind        | 🧭change-mixed-terrain-upwind        | ✅apply  |
      | change-mixed-terrain-distance      | 📏change-mixed-terrain-distance      | ✅apply  |
      | change-orography-factor            | 📐change-orography-factor            | ✅apply  |
      | change-coast-or-island             | 🏝️change-coast-or-island            | ✅apply  |
      | change-air-density                 | 🏢change-air-density                 | ✅apply  |
      | change-height                      | 🧱change-height                      | ✅apply  |
      | change-width                       | 🏠change-width                       | ✅apply  |
      | change-depth                       | 💨change-depth                       | ✅apply  |
      | change-assumed-delta-t             | 🌡change-assumed-delta-t             | ✅apply  |
      | change-construction-activity       | 🔥change-construction-activity       | ✅apply  |
      | change-assumed-construction-qk     | ⚙change-assumed-construction-qk     | ✅apply  |
      | change-structure-kind              | 🌉change-structure-kind              | ✅apply  |
      | change-bridge-lane                 | 🌉change-bridge-lane                 | ✅apply  |
      | change-bridge-span                 | 🏗change-bridge-span                 | ✅apply  |
      | change-bridge-lane-width           | ↔️change-bridge-lane-width          | ✅apply  |
      | change-assumed-bridge-tandem       | 🌾change-assumed-bridge-tandem       | ✅apply  |
      | change-assumed-bridge-udl          | 🛣change-assumed-bridge-udl          | ✅apply  |
      | change-assumed-bridge-lm2          | 🚛change-assumed-bridge-lm2          | ✅apply  |
      | change-assumed-bridge-footway      | 🚶change-assumed-bridge-footway      | ✅apply  |
      | change-storey-count                | 🏙change-storey-count                | ✅apply  |
      | change-t-max                       | 🌡change-t-max                       | ✅apply  |
      | change-t-min                       | 🧊change-t-min                       | ✅apply  |
      | change-initial-temperature         | 🕰change-initial-temperature         | ✅apply  |
      | change-thermal-element-type        | 🏗change-thermal-element-type        | ✅apply  |
      | change-thermal-bridge-type         | 🌉change-thermal-bridge-type         | ✅apply  |
      | change-linear-temperature-gradient | 📏change-linear-temperature-gradient | ✅apply  |
      | change-fire-mode                   | 🔥change-fire-mode                   | ✅apply  |
      | change-fire-curve                  | 📉change-fire-curve                  | ✅apply  |
      | change-fire-duration               | ⏱change-fire-duration               | ✅apply  |
      | change-assumed-gas-temperature     | ♨change-assumed-gas-temperature     | ✅apply  |
      | change-assumed-h-net               | 🔆change-assumed-h-net               | ✅apply  |
      | change-fire-compartment-area       | 🗺change-fire-compartment-area       | ✅apply  |
      | change-fire-compartment-height     | 📐change-fire-compartment-height     | ✅apply  |
      | change-fire-opening-factor         | 🪟change-fire-opening-factor         | ✅apply  |
      | change-fire-thermal-inertia        | 🧱change-fire-thermal-inertia        | ✅apply  |
      | change-fire-occupancy              | 🏢change-fire-occupancy              | ✅apply  |
      | change-fire-load-density-qf        | ⛽change-fire-load-density-qf        | ✅apply  |
      | change-assumed-qf-d                | 🔋change-assumed-qf-d                | ✅apply  |
      | change-assumed-bridge-lm3          | 🚛change-assumed-bridge-lm3          | ✅apply  |
      | change-assumed-bridge-lm4          | 👥change-assumed-bridge-lm4          | ✅apply  |
      | change-bridge-load-group           | 📦change-bridge-load-group           | ✅apply  |
      | change-crane-claimed               | 🏗️change-crane-claimed              | ✅apply  |
      | change-crane-class                 | 💥change-crane-class                 | ✅apply  |
      | change-hoist-class                 | ➕change-hoist-class                 | ✅apply  |
      | change-hoisting-speed              | ⏫change-hoisting-speed              | ✅apply  |
      | change-assumed-crane-wheel         | ➖change-assumed-crane-wheel         | ✅apply  |
      | change-assumed-crane-horizontal    | ↔️change-assumed-crane-horizontal   | ✅apply  |
      | change-silo-claimed                | 🏭change-silo-claimed                | ✅apply  |
      | change-silo-kind                   | ⚖change-silo-kind                   | ✅apply  |
      | change-silo-bulk-density           | 🌾change-silo-bulk-density           | ✅apply  |
      | change-silo-height                 | 🏷change-silo-height                 | ✅apply  |
      | change-silo-hydraulic-radius       | ⭕change-silo-hydraulic-radius       | ✅apply  |
      | change-silo-mu                     | 🔎change-silo-mu                     | ✅apply  |
      | change-silo-k                      | ⚙️change-silo-k                     | ✅apply  |
      | change-assumed-silo-pressure       | 🌀change-assumed-silo-pressure       | ✅apply  |
      | change-assumed-silo-patch          | 📦change-assumed-silo-patch          | ✅apply  |
      | change-assumed-silo-wall-friction  | 🧱change-assumed-silo-wall-friction  | ✅apply  |
      | change-floor-assumed-qk            | 🏢change-floor-assumed-qk            | ✅apply  |
      | change-self-weight-assumed-gk      | 🚧change-self-weight-assumed-gk      | ✅apply  |
      | change-roof-assumed-sk             | 🌨️change-roof-assumed-sk            | ✅apply  |
      | change-wind-face-assumed-wp        | 🛡change-wind-face-assumed-wp        | ✅apply  |
      | change-accidental-assumed-force    | 🚗change-accidental-assumed-force    | ✅apply  |
      | insert-floors                      | ➕️insert-floors                     | ✅apply  |
      | remove-floors                      | ➖️remove-floors                     | ✅apply  |
      | insert-self-weight-elements        | ➕️insert-self-weight-elements       | ✅apply  |
      | remove-self-weight-elements        | ➖️remove-self-weight-elements       | ✅apply  |
      | insert-roofs                       | ➕️insert-roofs                      | ✅apply  |
      | remove-roofs                       | ➖️remove-roofs                      | ✅apply  |
      | insert-wind-faces                  | ➕️insert-wind-faces                 | ✅apply  |
      | remove-wind-faces                  | ➖️remove-wind-faces                 | ✅apply  |
      | insert-accidental-cases            | ➕️insert-accidental-cases           | ✅apply  |
      | remove-accidental-cases            | ➖️remove-accidental-cases           | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1991 document from the parsed carrier
    Given the real committed text artifact asset://🏢de-office-compliant/🏢de-office-compliant/🗣️.dsl.semio
    And its committed binary twin asset://🏢de-office-compliant/🏢de-office-compliant/📦️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
