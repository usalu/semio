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
  `📖️.grammar.semio` is the repository-wide `payload = OCTET+` rule, so the two
  implementations are compared at the envelope preamble, the ordered `key=value` fields and the
  digest and length of what each re-emitted — never at a carrier-token-to-enum mapping this
  repository nowhere states.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.before.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🔣️.mutation.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.after.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🔣️.outcome.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id                                 | dir                                 | fixture |
      | change-annex                       | 🌍change-annex                       | ✅apply  |
      | change-snow-zone                   | 🗺️change-snow-zone                  | ✅apply  |
      | change-altitude                    | ❄️change-altitude                    | ✅apply  |
      | change-en-sk                       | ❄️change-en-sk                      | ✅apply  |
      | change-north-german-lowland-snow   | 🏔️change-north-german-lowland-snow   | ✅apply  |
      | change-wind-zone                   | 🪁change-wind-zone                   | ✅apply  |
      | change-en-vb                       | 🌬️change-en-vb                       | ✅apply  |
      | change-terrain-category            | 🏞️change-terrain-category           | ✅apply  |
      | change-mixed-terrain-upwind        | 🧭change-mixed-terrain-upwind        | ✅apply  |
      | change-mixed-terrain-distance      | 📏change-mixed-terrain-distance      | ✅apply  |
      | change-orography-factor            | 📐change-orography-factor            | ✅apply  |
      | change-coast-or-island             | 🏝️change-coast-or-island            | ✅apply  |
      | change-air-density                 | 🏢change-air-density                 | ✅apply  |
      | change-height                      | 🧱change-height                      | ✅apply  |
      | change-width                       | 🏠change-width                       | ✅apply  |
      | change-depth                       | 💨change-depth                       | ✅apply  |
      | change-assumed-delta-t             | 🌡️change-assumed-delta-t             | ✅apply  |
      | change-construction-activity       | 🔥change-construction-activity       | ✅apply  |
      | change-assumed-construction-qk     | ⚙️change-assumed-construction-qk     | ✅apply  |
      | change-structure-kind              | 🌉change-structure-kind              | ✅apply  |
      | change-bridge-lane                 | 🌉change-bridge-lane                 | ✅apply  |
      | change-bridge-span                 | 🏗️change-bridge-span                 | ✅apply  |
      | change-bridge-tandem-x                 | 📍change-bridge-tandem-x                 | ✅apply  |
      | change-bridge-curve-radius                 | 🌀change-bridge-curve-radius                 | ✅apply  |
      | change-rail-alpha                 | 🚆change-rail-alpha                 | ✅apply  |
      | change-assumed-rail-lm71                 | 🚆change-assumed-rail-lm71                 | ✅apply  |
      | change-assumed-rail-lm71-udl                 | 🚆change-assumed-rail-lm71-udl                 | ✅apply  |
      | change-assumed-rail-sw0                 | 🚆change-assumed-rail-sw0                 | ✅apply  |
      | change-assumed-rail-sw2                 | 🚆change-assumed-rail-sw2                 | ✅apply  |
      | change-crane-x                 | 🏗️change-crane-x                 | ✅apply  |
      | change-crane-runway-length                 | 📏change-crane-runway-length                 | ✅apply  |
      | change-crane-wheel-base                 | 🛞change-crane-wheel-base                 | ✅apply  |
      | change-silo-diameter                 | ⭕change-silo-diameter                 | ✅apply  |
      | change-silo-eccentricity                 | 🎯change-silo-eccentricity                 | ✅apply  |
      | change-silo-hopper-angle-deg                 | 🔻change-silo-hopper-angle-deg                 | ✅apply  |
      | change-bridge-lane-width           | ↔️change-bridge-lane-width          | ✅apply  |
      | change-assumed-bridge-tandem       | 🌾change-assumed-bridge-tandem       | ✅apply  |
      | change-assumed-bridge-udl          | 🛣️change-assumed-bridge-udl          | ✅apply  |
      | change-assumed-bridge-lm2          | 🚛change-assumed-bridge-lm2          | ✅apply  |
      | change-assumed-bridge-footway      | 🚶change-assumed-bridge-footway      | ✅apply  |
      | change-storey-count                | 🏙️change-storey-count                | ✅apply  |
      | change-t-max                       | 🌡️change-t-max                       | ✅apply  |
      | change-t-min                       | 🧊change-t-min                       | ✅apply  |
      | change-initial-temperature         | 🕰️change-initial-temperature         | ✅apply  |
      | change-thermal-element-type        | 🏗️change-thermal-element-type        | ✅apply  |
      | change-thermal-bridge-type         | 🌉change-thermal-bridge-type         | ✅apply  |
      | change-linear-temperature-gradient | 📏change-linear-temperature-gradient | ✅apply  |
      | change-fire-mode                   | 🔥change-fire-mode                   | ✅apply  |
      | change-fire-load-basis                   | 🧯change-fire-load-basis                   | ✅apply  |
      | change-fire-curve                  | 📉change-fire-curve                  | ✅apply  |
      | change-fire-duration               | ⏱️change-fire-duration               | ✅apply  |
      | change-assumed-gas-temperature     | ♨️change-assumed-gas-temperature     | ✅apply  |
      | change-assumed-h-net               | 🔆change-assumed-h-net               | ✅apply  |
      | change-fire-compartment-area       | 🗺️change-fire-compartment-area       | ✅apply  |
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
      | change-silo-kind                   | ⚖️change-silo-kind                   | ✅apply  |
      | change-silo-bulk-density           | 🌾change-silo-bulk-density           | ✅apply  |
      | change-silo-height                 | 🏷️change-silo-height                 | ✅apply  |
      | change-silo-hydraulic-radius       | ⭕change-silo-hydraulic-radius       | ✅apply  |
      | change-silo-mu                     | 🔎change-silo-mu                     | ✅apply  |
      | change-silo-k                      | ⚙️change-silo-k                     | ✅apply  |
      | change-assumed-silo-pressure       | 🌀change-assumed-silo-pressure       | ✅apply  |
      | change-assumed-silo-patch          | 📦change-assumed-silo-patch          | ✅apply  |
      | change-assumed-silo-wall-friction  | 🧱change-assumed-silo-wall-friction  | ✅apply  |
      | change-floor-assumed-qk            | 🏢change-floor-assumed-qk            | ✅apply  |
      | change-self-weight-assumed-gk      | 🚧change-self-weight-assumed-gk      | ✅apply  |
      | change-roof-assumed-sk             | 🌨️change-roof-assumed-sk            | ✅apply  |
      | change-wind-face-assumed-wp        | 🛡️change-wind-face-assumed-wp        | ✅apply  |
      | change-accidental-assumed-force    | 🚗change-accidental-assumed-force    | ✅apply  |
      | insert-floor                      | ➕️insert-floor                     | ✅apply  |
      | remove-floor                      | ➖️remove-floor                     | ✅apply  |
      | insert-self-weight-element        | ➕️insert-self-weight-element       | ✅apply  |
      | remove-self-weight-element        | ➖️remove-self-weight-element       | ✅apply  |
      | insert-roof                       | ➕️insert-roof                      | ✅apply  |
      | remove-roof                       | ➖️remove-roof                      | ✅apply  |
      | insert-wind-face                  | ➕️insert-wind-face                 | ✅apply  |
      | remove-wind-face                  | ➖️remove-wind-face                 | ✅apply  |
      | insert-accidental-case            | ➕️insert-accidental-case           | ✅apply  |
      | remove-accidental-case            | ➖️remove-accidental-case           | ✅apply  |
      | change-roof-pitch-deg            | 📐change-roof-pitch-deg           | ✅apply  |
      | change-roof-parapet-height            | 📏change-roof-parapet-height           | ✅apply  |
      | change-roof-drift-obstruction-height            | 🧱change-roof-drift-obstruction-height           | ✅apply  |
      | change-floor-area            | 🗺️change-floor-area           | ✅apply  |
      | change-footbridge-loaded-length | 🚶️change-footbridge-loaded-length | ✅apply  |
      | change-footbridge-deck-width | 🚶️change-footbridge-deck-width | ✅apply  |
      | change-footbridge-natural-frequency | 🚶️change-footbridge-natural-frequency | ✅apply  |
      | change-footbridge-assumed-service-axle | 🚶️change-footbridge-assumed-service-axle | ✅apply  |
      | insert-footbridge | 🚶️insert-footbridge | ✅apply  |
      | remove-footbridge | 🚶️remove-footbridge | ✅apply  |
      | change-rail-dynamic-determinant-length | 🛤️change-rail-dynamic-determinant-length | ✅apply  |
      | change-rail-dynamic-maintenance | 🛤️change-rail-dynamic-maintenance | ✅apply  |
      | change-fatigue-model-model | 🛤️change-fatigue-model-model | ✅apply  |
      | change-support-collision-location | 🛤️change-support-collision-location | ✅apply  |
      | change-snow-guard-roof-id | 🛤️change-snow-guard-roof-id | ✅apply  |
      | change-rail-derailment-situation | 🛤️change-rail-derailment-situation | ✅apply  |
      | change-rail-load-group-load-group | 🛤️change-rail-load-group-load-group | ✅apply  |
      | insert-rail-dynamic | 🛤️insert-rail-dynamic | ✅apply  |
      | remove-rail-dynamic | 🛤️remove-rail-dynamic | ✅apply  |
      | change-load-dispersal-wheel-length | 📏️change-load-dispersal-wheel-length | ✅apply  |
      | change-load-dispersal-wheel-width | 📏️change-load-dispersal-wheel-width | ✅apply  |
      | change-load-dispersal-paving-thickness | 📏️change-load-dispersal-paving-thickness | ✅apply  |
      | change-load-dispersal-slab-thickness | 📏️change-load-dispersal-slab-thickness | ✅apply  |
      | insert-load-dispersal | 📏️insert-load-dispersal | ✅apply  |
      | remove-load-dispersal | 📏️remove-load-dispersal | ✅apply  |
      | change-bulk-solid-internal-friction | 🌾️change-bulk-solid-internal-friction | ✅apply  |
      | insert-bulk-solid | 🌾️insert-bulk-solid | ✅apply  |
      | remove-bulk-solid | 🌾️remove-bulk-solid | ✅apply  |
      | change-rotating-machine-mass | ⚙️change-rotating-machine-mass | ✅apply  |
      | change-rotating-machine-eccentricity | ⚙️change-rotating-machine-eccentricity | ✅apply  |
      | change-rotating-machine-frequency | ⚙️change-rotating-machine-frequency | ✅apply  |
      | change-fatigue-model-assumed-udl | ⚙️change-fatigue-model-assumed-udl | ✅apply  |
      | change-fatigue-model-assumed-axle-load | ⚙️change-fatigue-model-assumed-axle-load | ✅apply  |
      | change-support-collision-position-x | ⚙️change-support-collision-position-x | ✅apply  |
      | change-support-collision-height | ⚙️change-support-collision-height | ✅apply  |
      | change-support-collision-assumed-force-y | ⚙️change-support-collision-assumed-force-y | ✅apply  |
      | change-support-collision-assumed-force-x | ⚙️change-support-collision-assumed-force-x | ✅apply  |
      | change-local-fire-assumed-flux | ⚙️change-local-fire-assumed-flux | ✅apply  |
      | change-local-fire-horizontal-distance | ⚙️change-local-fire-horizontal-distance | ✅apply  |
      | change-local-fire-ceiling-height | ⚙️change-local-fire-ceiling-height | ✅apply  |
      | change-local-fire-heat-release-rate | ⚙️change-local-fire-heat-release-rate | ✅apply  |
      | change-local-fire-fire-diameter | ⚙️change-local-fire-fire-diameter | ✅apply  |
      | change-snow-guard-assumed-force | ⚙️change-snow-guard-assumed-force | ✅apply  |
      | change-snow-guard-slope-length | ⚙️change-snow-guard-slope-length | ✅apply  |
      | change-rail-derailment-offset | ⚙️change-rail-derailment-offset | ✅apply  |
      | change-rail-derailment-loaded-length | ⚙️change-rail-derailment-loaded-length | ✅apply  |
      | change-rail-derailment-assumed-line-load | ⚙️change-rail-derailment-assumed-line-load | ✅apply  |
      | change-rail-load-group-assumed-lateral | ⚙️change-rail-load-group-assumed-lateral | ✅apply  |
      | change-rail-load-group-assumed-longitudinal | ⚙️change-rail-load-group-assumed-longitudinal | ✅apply  |
      | change-rail-load-group-assumed-udl | ⚙️change-rail-load-group-assumed-udl | ✅apply  |
      | change-rail-load-group-assumed-axle-load | ⚙️change-rail-load-group-assumed-axle-load | ✅apply  |
      | change-rail-load-group-factor-lateral | ⚙️change-rail-load-group-factor-lateral | ✅apply  |
      | change-rail-load-group-factor-longitudinal | ⚙️change-rail-load-group-factor-longitudinal | ✅apply  |
      | change-rail-load-group-factor-vertical | ⚙️change-rail-load-group-factor-vertical | ✅apply  |
      | change-dust-explosion-design-pressure | ⚙️change-dust-explosion-design-pressure | ✅apply  |
      | change-dust-explosion-static-opening-pressure | ⚙️change-dust-explosion-static-opening-pressure | ✅apply  |
      | change-dust-explosion-vent-area | ⚙️change-dust-explosion-vent-area | ✅apply  |
      | change-dust-explosion-volume | ⚙️change-dust-explosion-volume | ✅apply  |
      | change-dust-explosion-p-max | ⚙️change-dust-explosion-p-max | ✅apply  |
      | change-dust-explosion-kst | ⚙️change-dust-explosion-kst | ✅apply  |
      | insert-rotating-machine | ⚙️insert-rotating-machine | ✅apply  |
      | remove-rotating-machine | ⚙️remove-rotating-machine | ✅apply  |
      | insert-dust-explosion | ⚙️insert-dust-explosion | ✅apply  |
      | remove-dust-explosion | ⚙️remove-dust-explosion | ✅apply  |
      | insert-rail-load-group | ⚙️insert-rail-load-group | ✅apply  |
      | remove-rail-load-group | ⚙️remove-rail-load-group | ✅apply  |
      | insert-rail-derailment | ⚙️insert-rail-derailment | ✅apply  |
      | remove-rail-derailment | ⚙️remove-rail-derailment | ✅apply  |
      | insert-fatigue-model | ⚙️insert-fatigue-model | ✅apply  |
      | remove-fatigue-model | ⚙️remove-fatigue-model | ✅apply  |
      | insert-support-collision | ⚙️insert-support-collision | ✅apply  |
      | remove-support-collision | ⚙️remove-support-collision | ✅apply  |
      | insert-local-fire | ⚙️insert-local-fire | ✅apply  |
      | remove-local-fire | ⚙️remove-local-fire | ✅apply  |
      | insert-snow-guard | ⚙️insert-snow-guard | ✅apply  |
      | remove-snow-guard | ⚙️remove-snow-guard | ✅apply  |
      | change-crane-hoist-rated-load | 🏗️change-crane-hoist-rated-load | ✅apply  |
      | change-crane-hoist-release-fraction | 🏗️change-crane-hoist-release-fraction | ✅apply  |
      | change-crane-hoist-release-device | 🏗️change-crane-hoist-release-device | ✅apply  |
      | insert-crane-hoist | 🏗️insert-crane-hoist | ✅apply  |
      | remove-crane-hoist | 🏗️remove-crane-hoist | ✅apply  |
      | change-helicopter-mass | 🚁️change-helicopter-mass | ✅apply  |
      | change-helicopter-x | 🚁️change-helicopter-x | ✅apply  |
      | change-helicopter-y | 🚁️change-helicopter-y | ✅apply  |
      | change-helicopter-height | 🚁️change-helicopter-height | ✅apply  |
      | change-helicopter-assumed-force | 🚁️change-helicopter-assumed-force | ✅apply  |
      | insert-helicopter | 🚁️insert-helicopter | ✅apply  |
      | remove-helicopter | 🚁️remove-helicopter | ✅apply  |
      | change-tie-run-kind | 🔗️change-tie-run-kind | ✅apply  |
      | change-tie-run-spacing | 🔗️change-tie-run-spacing | ✅apply  |
      | change-tie-run-span | 🔗️change-tie-run-span | ✅apply  |
      | change-tie-run-dead-load | 🔗️change-tie-run-dead-load | ✅apply  |
      | change-tie-run-imposed-load | 🔗️change-tie-run-imposed-load | ✅apply  |
      | change-tie-run-psi | 🔗️change-tie-run-psi | ✅apply  |
      | change-tie-run-x | 🔗️change-tie-run-x | ✅apply  |
      | change-tie-run-y | 🔗️change-tie-run-y | ✅apply  |
      | change-tie-run-provided-force | 🔗️change-tie-run-provided-force | ✅apply  |
      | insert-tie-run | 🔗️insert-tie-run | ✅apply  |
      | remove-tie-run | 🔗️remove-tie-run | ✅apply  |
      | change-construction-load-stage | 🏗️change-construction-load-stage | ✅apply  |
      | change-construction-load-kind | 🏗️change-construction-load-kind | ✅apply  |
      | change-fatigue-model-lane | 🏗️change-fatigue-model-lane | ✅apply  |
      | change-construction-load-magnitude | 🏗️change-construction-load-magnitude | ✅apply  |
      | insert-construction-load | 🏗️insert-construction-load | ✅apply  |
      | remove-construction-load | 🏗️remove-construction-load | ✅apply  |
      | change-falling-mass | ⬇️change-falling-mass | ✅apply  |
      | change-falling-drop-height | ⬇️change-falling-drop-height | ✅apply  |
      | change-falling-stopping-distance | ⬇️change-falling-stopping-distance | ✅apply  |
      | change-falling-x | ⬇️change-falling-x | ✅apply  |
      | change-falling-y | ⬇️change-falling-y | ✅apply  |
      | change-falling-assumed-force | ⬇️change-falling-assumed-force | ✅apply  |
      | insert-falling | ⬇️insert-falling | ✅apply  |
      | remove-falling | ⬇️remove-falling | ✅apply  |
      | change-imperfection-height | 📐️change-imperfection-height | ✅apply  |
      | change-imperfection-tolerance | 📐️change-imperfection-tolerance | ✅apply  |
      | change-imperfection-vertical-load | 📐️change-imperfection-vertical-load | ✅apply  |
      | change-imperfection-x | 📐️change-imperfection-x | ✅apply  |
      | change-imperfection-y | 📐️change-imperfection-y | ✅apply  |
      | change-imperfection-assumed-force | 📐️change-imperfection-assumed-force | ✅apply  |
      | insert-imperfection | 📐️insert-imperfection | ✅apply  |
      | remove-imperfection | 📐️remove-imperfection | ✅apply  |
      | change-derailment-distance | 🚆️change-derailment-distance | ✅apply  |
      | change-derailment-design-force-x | 🚆️change-derailment-design-force-x | ✅apply  |
      | change-derailment-design-force-y | 🚆️change-derailment-design-force-y | ✅apply  |
      | change-derailment-x | 🚆️change-derailment-x | ✅apply  |
      | change-derailment-y | 🚆️change-derailment-y | ✅apply  |
      | change-derailment-assumed-force | 🚆️change-derailment-assumed-force | ✅apply  |
      | insert-derailment | 🚆️insert-derailment | ✅apply  |
      | remove-derailment | 🚆️remove-derailment | ✅apply  |
      | change-floor-category | 🏢️change-floor-category | ✅apply  |
      | change-floor-assumed-qk-conc | 🏢️change-floor-assumed-qk-conc | ✅apply  |
      | change-floor-assumed-partitions | 🏢️change-floor-assumed-partitions | ✅apply  |
      | change-self-weight-material | 🧱️change-self-weight-material | ✅apply  |
      | change-roof-roof-type | 🏠️change-roof-roof-type | ✅apply  |
      | change-roof-ce | 🏠️change-roof-ce | ✅apply  |
      | change-roof-ct | 🏠️change-roof-ct | ✅apply  |
      | change-roof-has-parapet | 🏠️change-roof-has-parapet | ✅apply  |
      | change-roof-multi-span | 🏠️change-roof-multi-span | ✅apply  |
      | change-wind-face-zone | 🌬️change-wind-face-zone | ✅apply  |
      | change-wind-face-c-pe10 | 🌬️change-wind-face-c-pe10 | ✅apply  |
      | change-wind-face-c-pe1 | 🌬️change-wind-face-c-pe1 | ✅apply  |
      | change-wind-face-c-pi | 🌬️change-wind-face-c-pi | ✅apply  |
      | change-wind-face-cs | 🌬️change-wind-face-cs | ✅apply  |
      | change-wind-face-cd | 🌬️change-wind-face-cd | ✅apply  |
      | change-special-vehicle-designation | 🚛️change-special-vehicle-designation | ✅apply  |
      | change-special-vehicle-x | 🚛️change-special-vehicle-x | ✅apply  |
      | change-special-vehicle-y | 🚛️change-special-vehicle-y | ✅apply  |
      | insert-special-vehicle | 🚛️insert-special-vehicle | ✅apply  |
      | remove-special-vehicle | 🚛️remove-special-vehicle | ✅apply  |
      | change-wind-bridge-deck-width | 🌉️change-wind-bridge-deck-width | ✅apply  |
      | change-wind-bridge-deck-depth | 🌉️change-wind-bridge-deck-depth | ✅apply  |
      | change-wind-bridge-parapet-kind | 🌉️change-wind-bridge-parapet-kind | ✅apply  |
      | change-wind-bridge-parapet-height | 🌉️change-wind-bridge-parapet-height | ✅apply  |
      | change-wind-bridge-traffic-kind | 🌉️change-wind-bridge-traffic-kind | ✅apply  |
      | change-wind-bridge-ground-height | 🌉️change-wind-bridge-ground-height | ✅apply  |
      | change-wind-bridge-span | 🌉️change-wind-bridge-span | ✅apply  |
      | change-wind-bridge-assumed-force | 🌉️change-wind-bridge-assumed-force | ✅apply  |
      | insert-wind-bridge | 🌉️insert-wind-bridge | ✅apply  |
      | remove-wind-bridge | 🌉️remove-wind-bridge | ✅apply  |
      | change-dynamic-element-height | 🌀️change-dynamic-element-height | ✅apply  |
      | change-dynamic-element-width | 🌀️change-dynamic-element-width | ✅apply  |
      | change-dynamic-first-frequency | 🌀️change-dynamic-first-frequency | ✅apply  |
      | change-dynamic-log-decrement | 🌀️change-dynamic-log-decrement | ✅apply  |
      | change-dynamic-mass-per-length | 🌀️change-dynamic-mass-per-length | ✅apply  |
      | change-dynamic-element-strouhal | 🌀️change-dynamic-element-strouhal | ✅apply  |
      | change-dynamic-assumed-cscd | 🌀️change-dynamic-assumed-cscd | ✅apply  |
      | insert-dynamic-element | 🌀️insert-dynamic-element | ✅apply  |
      | remove-dynamic-element | 🌀️remove-dynamic-element | ✅apply  |
      | change-impact-action-kind | 💥️change-impact-action-kind | ✅apply  |
      | change-impact-action-mass | 💥️change-impact-action-mass | ✅apply  |
      | change-impact-action-speed | 💥️change-impact-action-speed | ✅apply  |
      | change-impact-action-stiffness | 💥️change-impact-action-stiffness | ✅apply  |
      | change-impact-action-height | 💥️change-impact-action-height | ✅apply  |
      | change-impact-action-x | 💥️change-impact-action-x | ✅apply  |
      | change-impact-action-y | 💥️change-impact-action-y | ✅apply  |
      | change-impact-action-assumed-force | 💥️change-impact-action-assumed-force | ✅apply  |
      | insert-impact-action | 💥️insert-impact-action | ✅apply  |
      | remove-impact-action | 💥️remove-impact-action | ✅apply  |
      | change-internal-explosion-volume | 🧨️change-internal-explosion-volume | ✅apply  |
      | change-explosion-vent-area | 🧨️change-explosion-vent-area | ✅apply  |
      | change-explosion-vent-pressure | 🧨️change-explosion-vent-pressure | ✅apply  |
      | change-explosion-assumed-pressure | 🧨️change-explosion-assumed-pressure | ✅apply  |
      | insert-internal-explosion | 🧨️insert-internal-explosion | ✅apply  |
      | remove-internal-explosion | 🧨️remove-internal-explosion | ✅apply  |
      | change-thermal-section-deck-type | 🌡️change-thermal-section-deck-type | ✅apply  |
      | change-thermal-section-surfacing | 🌡️change-thermal-section-surfacing | ✅apply  |
      | change-thermal-section-length | 🌡️change-thermal-section-length | ✅apply  |
      | change-thermal-section-depth | 🌡️change-thermal-section-depth | ✅apply  |
      | change-thermal-section-alpha-micro | 🌡️change-thermal-section-alpha-micro | ✅apply  |
      | change-thermal-assumed-range | 🌡️change-thermal-assumed-range | ✅apply  |
      | insert-thermal-section | 🌡️insert-thermal-section | ✅apply  |
      | remove-thermal-section | 🌡️remove-thermal-section | ✅apply  |
      | change-accidental-vehicle-mass | 🚗️change-accidental-vehicle-mass | ✅apply  |
      | change-accidental-vehicle-speed | 🚗️change-accidental-vehicle-speed | ✅apply  |
      | change-accidental-explosion-mass | 🚗️change-accidental-explosion-mass | ✅apply  |
      | change-accidental-standoff | 🚗️change-accidental-standoff | ✅apply  |
      | change-accidental-assumed-pressure | 🚗️change-accidental-assumed-pressure | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | insert-placement | 📍️insert-placement | ✅apply  |
      | remove-placement | 📍️remove-placement | ✅apply  |
      | insert-case | ➕️insert-case | ✅apply  |
      | remove-case | ➖️remove-case | ✅apply  |
      | change-case-number | 🔢️change-case-number | ✅apply  |
      | change-case-text | 🔤️change-case-text | ✅apply  |
      | change-case-label-en | 🏷️change-case-label-en | ✅apply  |
      | change-case-label-de | 🏷️change-case-label-de | ✅apply  |
      | change-case-entry-id | 📇️change-case-entry-id | ✅apply  |
      | change-lane-width | 🛣️change-lane-width | ✅apply  |
      | change-lane-offset | 🛣️change-lane-offset | ✅apply  |
      | change-lane-lm1-lane | 🛣️change-lane-lm1-lane | ✅apply  |
      | insert-bridge-lane | 🛣️insert-bridge-lane | ✅apply  |
      | remove-bridge-lane | 🛣️remove-bridge-lane | ✅apply  |
      | change-stage-activity | 🏗️change-stage-activity | ✅apply  |
      | change-stage-duration-days | 🏗️change-stage-duration-days | ✅apply  |
      | change-stage-return-years | 🏗️change-stage-return-years | ✅apply  |
      | change-stage-assumed-qk | 🏗️change-stage-assumed-qk | ✅apply  |
      | insert-construction-stage | 🏗️insert-construction-stage | ✅apply  |
      | remove-construction-stage | 🏗️remove-construction-stage | ✅apply  |
      | change-roof-surface-face | 🏠️change-roof-surface-face | ✅apply  |
      | change-roof-surface-shape | 🏠️change-roof-surface-shape | ✅apply  |
      | change-roof-surface-pitch-deg | 🏠️change-roof-surface-pitch-deg | ✅apply  |
      | change-roof-surface-extent | 🏠️change-roof-surface-extent | ✅apply  |
      | insert-roof-surface | 🏠️insert-roof-surface | ✅apply  |
      | remove-roof-surface | 🏠️remove-roof-surface | ✅apply  |
      | change-self-weight-thickness            | 🥞change-self-weight-thickness           | ✅apply  |
      | change-wind-face-z            | ↕️change-wind-face-z           | ✅apply  |
      | change-wind-face-loaded-area            | 🔲change-wind-face-loaded-area           | ✅apply  |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.before.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🔣️.mutation.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.after.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🔣️.outcome.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id                                 | dir                                 | fixture |
      | change-annex                       | 🌍change-annex                       | ✅apply  |
      | change-snow-zone                   | 🗺️change-snow-zone                  | ✅apply  |
      | change-altitude                    | ❄️change-altitude                    | ✅apply  |
      | change-en-sk                       | ❄️change-en-sk                      | ✅apply  |
      | change-north-german-lowland-snow   | 🏔️change-north-german-lowland-snow   | ✅apply  |
      | change-wind-zone                   | 🪁change-wind-zone                   | ✅apply  |
      | change-en-vb                       | 🌬️change-en-vb                       | ✅apply  |
      | change-terrain-category            | 🏞️change-terrain-category           | ✅apply  |
      | change-mixed-terrain-upwind        | 🧭change-mixed-terrain-upwind        | ✅apply  |
      | change-mixed-terrain-distance      | 📏change-mixed-terrain-distance      | ✅apply  |
      | change-orography-factor            | 📐change-orography-factor            | ✅apply  |
      | change-coast-or-island             | 🏝️change-coast-or-island            | ✅apply  |
      | change-air-density                 | 🏢change-air-density                 | ✅apply  |
      | change-height                      | 🧱change-height                      | ✅apply  |
      | change-width                       | 🏠change-width                       | ✅apply  |
      | change-depth                       | 💨change-depth                       | ✅apply  |
      | change-assumed-delta-t             | 🌡️change-assumed-delta-t             | ✅apply  |
      | change-construction-activity       | 🔥change-construction-activity       | ✅apply  |
      | change-assumed-construction-qk     | ⚙️change-assumed-construction-qk     | ✅apply  |
      | change-structure-kind              | 🌉change-structure-kind              | ✅apply  |
      | change-bridge-lane                 | 🌉change-bridge-lane                 | ✅apply  |
      | change-bridge-span                 | 🏗️change-bridge-span                 | ✅apply  |
      | change-bridge-tandem-x                 | 📍change-bridge-tandem-x                 | ✅apply  |
      | change-bridge-curve-radius                 | 🌀change-bridge-curve-radius                 | ✅apply  |
      | change-rail-alpha                 | 🚆change-rail-alpha                 | ✅apply  |
      | change-assumed-rail-lm71                 | 🚆change-assumed-rail-lm71                 | ✅apply  |
      | change-assumed-rail-lm71-udl                 | 🚆change-assumed-rail-lm71-udl                 | ✅apply  |
      | change-assumed-rail-sw0                 | 🚆change-assumed-rail-sw0                 | ✅apply  |
      | change-assumed-rail-sw2                 | 🚆change-assumed-rail-sw2                 | ✅apply  |
      | change-crane-x                 | 🏗️change-crane-x                 | ✅apply  |
      | change-crane-runway-length                 | 📏change-crane-runway-length                 | ✅apply  |
      | change-crane-wheel-base                 | 🛞change-crane-wheel-base                 | ✅apply  |
      | change-silo-diameter                 | ⭕change-silo-diameter                 | ✅apply  |
      | change-silo-eccentricity                 | 🎯change-silo-eccentricity                 | ✅apply  |
      | change-silo-hopper-angle-deg                 | 🔻change-silo-hopper-angle-deg                 | ✅apply  |
      | change-bridge-lane-width           | ↔️change-bridge-lane-width          | ✅apply  |
      | change-assumed-bridge-tandem       | 🌾change-assumed-bridge-tandem       | ✅apply  |
      | change-assumed-bridge-udl          | 🛣️change-assumed-bridge-udl          | ✅apply  |
      | change-assumed-bridge-lm2          | 🚛change-assumed-bridge-lm2          | ✅apply  |
      | change-assumed-bridge-footway      | 🚶change-assumed-bridge-footway      | ✅apply  |
      | change-storey-count                | 🏙️change-storey-count                | ✅apply  |
      | change-t-max                       | 🌡️change-t-max                       | ✅apply  |
      | change-t-min                       | 🧊change-t-min                       | ✅apply  |
      | change-initial-temperature         | 🕰️change-initial-temperature         | ✅apply  |
      | change-thermal-element-type        | 🏗️change-thermal-element-type        | ✅apply  |
      | change-thermal-bridge-type         | 🌉change-thermal-bridge-type         | ✅apply  |
      | change-linear-temperature-gradient | 📏change-linear-temperature-gradient | ✅apply  |
      | change-fire-mode                   | 🔥change-fire-mode                   | ✅apply  |
      | change-fire-load-basis                   | 🧯change-fire-load-basis                   | ✅apply  |
      | change-fire-curve                  | 📉change-fire-curve                  | ✅apply  |
      | change-fire-duration               | ⏱️change-fire-duration               | ✅apply  |
      | change-assumed-gas-temperature     | ♨️change-assumed-gas-temperature     | ✅apply  |
      | change-assumed-h-net               | 🔆change-assumed-h-net               | ✅apply  |
      | change-fire-compartment-area       | 🗺️change-fire-compartment-area       | ✅apply  |
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
      | change-silo-kind                   | ⚖️change-silo-kind                   | ✅apply  |
      | change-silo-bulk-density           | 🌾change-silo-bulk-density           | ✅apply  |
      | change-silo-height                 | 🏷️change-silo-height                 | ✅apply  |
      | change-silo-hydraulic-radius       | ⭕change-silo-hydraulic-radius       | ✅apply  |
      | change-silo-mu                     | 🔎change-silo-mu                     | ✅apply  |
      | change-silo-k                      | ⚙️change-silo-k                     | ✅apply  |
      | change-assumed-silo-pressure       | 🌀change-assumed-silo-pressure       | ✅apply  |
      | change-assumed-silo-patch          | 📦change-assumed-silo-patch          | ✅apply  |
      | change-assumed-silo-wall-friction  | 🧱change-assumed-silo-wall-friction  | ✅apply  |
      | change-floor-assumed-qk            | 🏢change-floor-assumed-qk            | ✅apply  |
      | change-self-weight-assumed-gk      | 🚧change-self-weight-assumed-gk      | ✅apply  |
      | change-roof-assumed-sk             | 🌨️change-roof-assumed-sk            | ✅apply  |
      | change-wind-face-assumed-wp        | 🛡️change-wind-face-assumed-wp        | ✅apply  |
      | change-accidental-assumed-force    | 🚗change-accidental-assumed-force    | ✅apply  |
      | insert-floor                      | ➕️insert-floor                     | ✅apply  |
      | remove-floor                      | ➖️remove-floor                     | ✅apply  |
      | insert-self-weight-element        | ➕️insert-self-weight-element       | ✅apply  |
      | remove-self-weight-element        | ➖️remove-self-weight-element       | ✅apply  |
      | insert-roof                       | ➕️insert-roof                      | ✅apply  |
      | remove-roof                       | ➖️remove-roof                      | ✅apply  |
      | insert-wind-face                  | ➕️insert-wind-face                 | ✅apply  |
      | remove-wind-face                  | ➖️remove-wind-face                 | ✅apply  |
      | insert-accidental-case            | ➕️insert-accidental-case           | ✅apply  |
      | remove-accidental-case            | ➖️remove-accidental-case           | ✅apply  |
      | change-roof-pitch-deg            | 📐change-roof-pitch-deg           | ✅apply  |
      | change-roof-parapet-height            | 📏change-roof-parapet-height           | ✅apply  |
      | change-roof-drift-obstruction-height            | 🧱change-roof-drift-obstruction-height           | ✅apply  |
      | change-floor-area            | 🗺️change-floor-area           | ✅apply  |
      | change-footbridge-loaded-length | 🚶️change-footbridge-loaded-length | ✅apply  |
      | change-footbridge-deck-width | 🚶️change-footbridge-deck-width | ✅apply  |
      | change-footbridge-natural-frequency | 🚶️change-footbridge-natural-frequency | ✅apply  |
      | change-footbridge-assumed-service-axle | 🚶️change-footbridge-assumed-service-axle | ✅apply  |
      | insert-footbridge | 🚶️insert-footbridge | ✅apply  |
      | remove-footbridge | 🚶️remove-footbridge | ✅apply  |
      | change-rail-dynamic-determinant-length | 🛤️change-rail-dynamic-determinant-length | ✅apply  |
      | change-rail-dynamic-maintenance | 🛤️change-rail-dynamic-maintenance | ✅apply  |
      | change-fatigue-model-model | 🛤️change-fatigue-model-model | ✅apply  |
      | change-support-collision-location | 🛤️change-support-collision-location | ✅apply  |
      | change-snow-guard-roof-id | 🛤️change-snow-guard-roof-id | ✅apply  |
      | change-rail-derailment-situation | 🛤️change-rail-derailment-situation | ✅apply  |
      | change-rail-load-group-load-group | 🛤️change-rail-load-group-load-group | ✅apply  |
      | insert-rail-dynamic | 🛤️insert-rail-dynamic | ✅apply  |
      | remove-rail-dynamic | 🛤️remove-rail-dynamic | ✅apply  |
      | change-load-dispersal-wheel-length | 📏️change-load-dispersal-wheel-length | ✅apply  |
      | change-load-dispersal-wheel-width | 📏️change-load-dispersal-wheel-width | ✅apply  |
      | change-load-dispersal-paving-thickness | 📏️change-load-dispersal-paving-thickness | ✅apply  |
      | change-load-dispersal-slab-thickness | 📏️change-load-dispersal-slab-thickness | ✅apply  |
      | insert-load-dispersal | 📏️insert-load-dispersal | ✅apply  |
      | remove-load-dispersal | 📏️remove-load-dispersal | ✅apply  |
      | change-bulk-solid-internal-friction | 🌾️change-bulk-solid-internal-friction | ✅apply  |
      | insert-bulk-solid | 🌾️insert-bulk-solid | ✅apply  |
      | remove-bulk-solid | 🌾️remove-bulk-solid | ✅apply  |
      | change-rotating-machine-mass | ⚙️change-rotating-machine-mass | ✅apply  |
      | change-rotating-machine-eccentricity | ⚙️change-rotating-machine-eccentricity | ✅apply  |
      | change-rotating-machine-frequency | ⚙️change-rotating-machine-frequency | ✅apply  |
      | change-fatigue-model-assumed-udl | ⚙️change-fatigue-model-assumed-udl | ✅apply  |
      | change-fatigue-model-assumed-axle-load | ⚙️change-fatigue-model-assumed-axle-load | ✅apply  |
      | change-support-collision-position-x | ⚙️change-support-collision-position-x | ✅apply  |
      | change-support-collision-height | ⚙️change-support-collision-height | ✅apply  |
      | change-support-collision-assumed-force-y | ⚙️change-support-collision-assumed-force-y | ✅apply  |
      | change-support-collision-assumed-force-x | ⚙️change-support-collision-assumed-force-x | ✅apply  |
      | change-local-fire-assumed-flux | ⚙️change-local-fire-assumed-flux | ✅apply  |
      | change-local-fire-horizontal-distance | ⚙️change-local-fire-horizontal-distance | ✅apply  |
      | change-local-fire-ceiling-height | ⚙️change-local-fire-ceiling-height | ✅apply  |
      | change-local-fire-heat-release-rate | ⚙️change-local-fire-heat-release-rate | ✅apply  |
      | change-local-fire-fire-diameter | ⚙️change-local-fire-fire-diameter | ✅apply  |
      | change-snow-guard-assumed-force | ⚙️change-snow-guard-assumed-force | ✅apply  |
      | change-snow-guard-slope-length | ⚙️change-snow-guard-slope-length | ✅apply  |
      | change-rail-derailment-offset | ⚙️change-rail-derailment-offset | ✅apply  |
      | change-rail-derailment-loaded-length | ⚙️change-rail-derailment-loaded-length | ✅apply  |
      | change-rail-derailment-assumed-line-load | ⚙️change-rail-derailment-assumed-line-load | ✅apply  |
      | change-rail-load-group-assumed-lateral | ⚙️change-rail-load-group-assumed-lateral | ✅apply  |
      | change-rail-load-group-assumed-longitudinal | ⚙️change-rail-load-group-assumed-longitudinal | ✅apply  |
      | change-rail-load-group-assumed-udl | ⚙️change-rail-load-group-assumed-udl | ✅apply  |
      | change-rail-load-group-assumed-axle-load | ⚙️change-rail-load-group-assumed-axle-load | ✅apply  |
      | change-rail-load-group-factor-lateral | ⚙️change-rail-load-group-factor-lateral | ✅apply  |
      | change-rail-load-group-factor-longitudinal | ⚙️change-rail-load-group-factor-longitudinal | ✅apply  |
      | change-rail-load-group-factor-vertical | ⚙️change-rail-load-group-factor-vertical | ✅apply  |
      | change-dust-explosion-design-pressure | ⚙️change-dust-explosion-design-pressure | ✅apply  |
      | change-dust-explosion-static-opening-pressure | ⚙️change-dust-explosion-static-opening-pressure | ✅apply  |
      | change-dust-explosion-vent-area | ⚙️change-dust-explosion-vent-area | ✅apply  |
      | change-dust-explosion-volume | ⚙️change-dust-explosion-volume | ✅apply  |
      | change-dust-explosion-p-max | ⚙️change-dust-explosion-p-max | ✅apply  |
      | change-dust-explosion-kst | ⚙️change-dust-explosion-kst | ✅apply  |
      | insert-rotating-machine | ⚙️insert-rotating-machine | ✅apply  |
      | remove-rotating-machine | ⚙️remove-rotating-machine | ✅apply  |
      | insert-dust-explosion | ⚙️insert-dust-explosion | ✅apply  |
      | remove-dust-explosion | ⚙️remove-dust-explosion | ✅apply  |
      | insert-rail-load-group | ⚙️insert-rail-load-group | ✅apply  |
      | remove-rail-load-group | ⚙️remove-rail-load-group | ✅apply  |
      | insert-rail-derailment | ⚙️insert-rail-derailment | ✅apply  |
      | remove-rail-derailment | ⚙️remove-rail-derailment | ✅apply  |
      | insert-fatigue-model | ⚙️insert-fatigue-model | ✅apply  |
      | remove-fatigue-model | ⚙️remove-fatigue-model | ✅apply  |
      | insert-support-collision | ⚙️insert-support-collision | ✅apply  |
      | remove-support-collision | ⚙️remove-support-collision | ✅apply  |
      | insert-local-fire | ⚙️insert-local-fire | ✅apply  |
      | remove-local-fire | ⚙️remove-local-fire | ✅apply  |
      | insert-snow-guard | ⚙️insert-snow-guard | ✅apply  |
      | remove-snow-guard | ⚙️remove-snow-guard | ✅apply  |
      | change-crane-hoist-rated-load | 🏗️change-crane-hoist-rated-load | ✅apply  |
      | change-crane-hoist-release-fraction | 🏗️change-crane-hoist-release-fraction | ✅apply  |
      | change-crane-hoist-release-device | 🏗️change-crane-hoist-release-device | ✅apply  |
      | insert-crane-hoist | 🏗️insert-crane-hoist | ✅apply  |
      | remove-crane-hoist | 🏗️remove-crane-hoist | ✅apply  |
      | change-helicopter-mass | 🚁️change-helicopter-mass | ✅apply  |
      | change-helicopter-x | 🚁️change-helicopter-x | ✅apply  |
      | change-helicopter-y | 🚁️change-helicopter-y | ✅apply  |
      | change-helicopter-height | 🚁️change-helicopter-height | ✅apply  |
      | change-helicopter-assumed-force | 🚁️change-helicopter-assumed-force | ✅apply  |
      | insert-helicopter | 🚁️insert-helicopter | ✅apply  |
      | remove-helicopter | 🚁️remove-helicopter | ✅apply  |
      | change-tie-run-kind | 🔗️change-tie-run-kind | ✅apply  |
      | change-tie-run-spacing | 🔗️change-tie-run-spacing | ✅apply  |
      | change-tie-run-span | 🔗️change-tie-run-span | ✅apply  |
      | change-tie-run-dead-load | 🔗️change-tie-run-dead-load | ✅apply  |
      | change-tie-run-imposed-load | 🔗️change-tie-run-imposed-load | ✅apply  |
      | change-tie-run-psi | 🔗️change-tie-run-psi | ✅apply  |
      | change-tie-run-x | 🔗️change-tie-run-x | ✅apply  |
      | change-tie-run-y | 🔗️change-tie-run-y | ✅apply  |
      | change-tie-run-provided-force | 🔗️change-tie-run-provided-force | ✅apply  |
      | insert-tie-run | 🔗️insert-tie-run | ✅apply  |
      | remove-tie-run | 🔗️remove-tie-run | ✅apply  |
      | change-construction-load-stage | 🏗️change-construction-load-stage | ✅apply  |
      | change-construction-load-kind | 🏗️change-construction-load-kind | ✅apply  |
      | change-fatigue-model-lane | 🏗️change-fatigue-model-lane | ✅apply  |
      | change-construction-load-magnitude | 🏗️change-construction-load-magnitude | ✅apply  |
      | insert-construction-load | 🏗️insert-construction-load | ✅apply  |
      | remove-construction-load | 🏗️remove-construction-load | ✅apply  |
      | change-falling-mass | ⬇️change-falling-mass | ✅apply  |
      | change-falling-drop-height | ⬇️change-falling-drop-height | ✅apply  |
      | change-falling-stopping-distance | ⬇️change-falling-stopping-distance | ✅apply  |
      | change-falling-x | ⬇️change-falling-x | ✅apply  |
      | change-falling-y | ⬇️change-falling-y | ✅apply  |
      | change-falling-assumed-force | ⬇️change-falling-assumed-force | ✅apply  |
      | insert-falling | ⬇️insert-falling | ✅apply  |
      | remove-falling | ⬇️remove-falling | ✅apply  |
      | change-imperfection-height | 📐️change-imperfection-height | ✅apply  |
      | change-imperfection-tolerance | 📐️change-imperfection-tolerance | ✅apply  |
      | change-imperfection-vertical-load | 📐️change-imperfection-vertical-load | ✅apply  |
      | change-imperfection-x | 📐️change-imperfection-x | ✅apply  |
      | change-imperfection-y | 📐️change-imperfection-y | ✅apply  |
      | change-imperfection-assumed-force | 📐️change-imperfection-assumed-force | ✅apply  |
      | insert-imperfection | 📐️insert-imperfection | ✅apply  |
      | remove-imperfection | 📐️remove-imperfection | ✅apply  |
      | change-derailment-distance | 🚆️change-derailment-distance | ✅apply  |
      | change-derailment-design-force-x | 🚆️change-derailment-design-force-x | ✅apply  |
      | change-derailment-design-force-y | 🚆️change-derailment-design-force-y | ✅apply  |
      | change-derailment-x | 🚆️change-derailment-x | ✅apply  |
      | change-derailment-y | 🚆️change-derailment-y | ✅apply  |
      | change-derailment-assumed-force | 🚆️change-derailment-assumed-force | ✅apply  |
      | insert-derailment | 🚆️insert-derailment | ✅apply  |
      | remove-derailment | 🚆️remove-derailment | ✅apply  |
      | change-floor-category | 🏢️change-floor-category | ✅apply  |
      | change-floor-assumed-qk-conc | 🏢️change-floor-assumed-qk-conc | ✅apply  |
      | change-floor-assumed-partitions | 🏢️change-floor-assumed-partitions | ✅apply  |
      | change-self-weight-material | 🧱️change-self-weight-material | ✅apply  |
      | change-roof-roof-type | 🏠️change-roof-roof-type | ✅apply  |
      | change-roof-ce | 🏠️change-roof-ce | ✅apply  |
      | change-roof-ct | 🏠️change-roof-ct | ✅apply  |
      | change-roof-has-parapet | 🏠️change-roof-has-parapet | ✅apply  |
      | change-roof-multi-span | 🏠️change-roof-multi-span | ✅apply  |
      | change-wind-face-zone | 🌬️change-wind-face-zone | ✅apply  |
      | change-wind-face-c-pe10 | 🌬️change-wind-face-c-pe10 | ✅apply  |
      | change-wind-face-c-pe1 | 🌬️change-wind-face-c-pe1 | ✅apply  |
      | change-wind-face-c-pi | 🌬️change-wind-face-c-pi | ✅apply  |
      | change-wind-face-cs | 🌬️change-wind-face-cs | ✅apply  |
      | change-wind-face-cd | 🌬️change-wind-face-cd | ✅apply  |
      | change-special-vehicle-designation | 🚛️change-special-vehicle-designation | ✅apply  |
      | change-special-vehicle-x | 🚛️change-special-vehicle-x | ✅apply  |
      | change-special-vehicle-y | 🚛️change-special-vehicle-y | ✅apply  |
      | insert-special-vehicle | 🚛️insert-special-vehicle | ✅apply  |
      | remove-special-vehicle | 🚛️remove-special-vehicle | ✅apply  |
      | change-wind-bridge-deck-width | 🌉️change-wind-bridge-deck-width | ✅apply  |
      | change-wind-bridge-deck-depth | 🌉️change-wind-bridge-deck-depth | ✅apply  |
      | change-wind-bridge-parapet-kind | 🌉️change-wind-bridge-parapet-kind | ✅apply  |
      | change-wind-bridge-parapet-height | 🌉️change-wind-bridge-parapet-height | ✅apply  |
      | change-wind-bridge-traffic-kind | 🌉️change-wind-bridge-traffic-kind | ✅apply  |
      | change-wind-bridge-ground-height | 🌉️change-wind-bridge-ground-height | ✅apply  |
      | change-wind-bridge-span | 🌉️change-wind-bridge-span | ✅apply  |
      | change-wind-bridge-assumed-force | 🌉️change-wind-bridge-assumed-force | ✅apply  |
      | insert-wind-bridge | 🌉️insert-wind-bridge | ✅apply  |
      | remove-wind-bridge | 🌉️remove-wind-bridge | ✅apply  |
      | change-dynamic-element-height | 🌀️change-dynamic-element-height | ✅apply  |
      | change-dynamic-element-width | 🌀️change-dynamic-element-width | ✅apply  |
      | change-dynamic-first-frequency | 🌀️change-dynamic-first-frequency | ✅apply  |
      | change-dynamic-log-decrement | 🌀️change-dynamic-log-decrement | ✅apply  |
      | change-dynamic-mass-per-length | 🌀️change-dynamic-mass-per-length | ✅apply  |
      | change-dynamic-element-strouhal | 🌀️change-dynamic-element-strouhal | ✅apply  |
      | change-dynamic-assumed-cscd | 🌀️change-dynamic-assumed-cscd | ✅apply  |
      | insert-dynamic-element | 🌀️insert-dynamic-element | ✅apply  |
      | remove-dynamic-element | 🌀️remove-dynamic-element | ✅apply  |
      | change-impact-action-kind | 💥️change-impact-action-kind | ✅apply  |
      | change-impact-action-mass | 💥️change-impact-action-mass | ✅apply  |
      | change-impact-action-speed | 💥️change-impact-action-speed | ✅apply  |
      | change-impact-action-stiffness | 💥️change-impact-action-stiffness | ✅apply  |
      | change-impact-action-height | 💥️change-impact-action-height | ✅apply  |
      | change-impact-action-x | 💥️change-impact-action-x | ✅apply  |
      | change-impact-action-y | 💥️change-impact-action-y | ✅apply  |
      | change-impact-action-assumed-force | 💥️change-impact-action-assumed-force | ✅apply  |
      | insert-impact-action | 💥️insert-impact-action | ✅apply  |
      | remove-impact-action | 💥️remove-impact-action | ✅apply  |
      | change-internal-explosion-volume | 🧨️change-internal-explosion-volume | ✅apply  |
      | change-explosion-vent-area | 🧨️change-explosion-vent-area | ✅apply  |
      | change-explosion-vent-pressure | 🧨️change-explosion-vent-pressure | ✅apply  |
      | change-explosion-assumed-pressure | 🧨️change-explosion-assumed-pressure | ✅apply  |
      | insert-internal-explosion | 🧨️insert-internal-explosion | ✅apply  |
      | remove-internal-explosion | 🧨️remove-internal-explosion | ✅apply  |
      | change-thermal-section-deck-type | 🌡️change-thermal-section-deck-type | ✅apply  |
      | change-thermal-section-surfacing | 🌡️change-thermal-section-surfacing | ✅apply  |
      | change-thermal-section-length | 🌡️change-thermal-section-length | ✅apply  |
      | change-thermal-section-depth | 🌡️change-thermal-section-depth | ✅apply  |
      | change-thermal-section-alpha-micro | 🌡️change-thermal-section-alpha-micro | ✅apply  |
      | change-thermal-assumed-range | 🌡️change-thermal-assumed-range | ✅apply  |
      | insert-thermal-section | 🌡️insert-thermal-section | ✅apply  |
      | remove-thermal-section | 🌡️remove-thermal-section | ✅apply  |
      | change-accidental-vehicle-mass | 🚗️change-accidental-vehicle-mass | ✅apply  |
      | change-accidental-vehicle-speed | 🚗️change-accidental-vehicle-speed | ✅apply  |
      | change-accidental-explosion-mass | 🚗️change-accidental-explosion-mass | ✅apply  |
      | change-accidental-standoff | 🚗️change-accidental-standoff | ✅apply  |
      | change-accidental-assumed-pressure | 🚗️change-accidental-assumed-pressure | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | insert-placement | 📍️insert-placement | ✅apply  |
      | remove-placement | 📍️remove-placement | ✅apply  |
      | insert-case | ➕️insert-case | ✅apply  |
      | remove-case | ➖️remove-case | ✅apply  |
      | change-case-number | 🔢️change-case-number | ✅apply  |
      | change-case-text | 🔤️change-case-text | ✅apply  |
      | change-case-label-en | 🏷️change-case-label-en | ✅apply  |
      | change-case-label-de | 🏷️change-case-label-de | ✅apply  |
      | change-case-entry-id | 📇️change-case-entry-id | ✅apply  |
      | change-lane-width | 🛣️change-lane-width | ✅apply  |
      | change-lane-offset | 🛣️change-lane-offset | ✅apply  |
      | change-lane-lm1-lane | 🛣️change-lane-lm1-lane | ✅apply  |
      | insert-bridge-lane | 🛣️insert-bridge-lane | ✅apply  |
      | remove-bridge-lane | 🛣️remove-bridge-lane | ✅apply  |
      | change-stage-activity | 🏗️change-stage-activity | ✅apply  |
      | change-stage-duration-days | 🏗️change-stage-duration-days | ✅apply  |
      | change-stage-return-years | 🏗️change-stage-return-years | ✅apply  |
      | change-stage-assumed-qk | 🏗️change-stage-assumed-qk | ✅apply  |
      | insert-construction-stage | 🏗️insert-construction-stage | ✅apply  |
      | remove-construction-stage | 🏗️remove-construction-stage | ✅apply  |
      | change-roof-surface-face | 🏠️change-roof-surface-face | ✅apply  |
      | change-roof-surface-shape | 🏠️change-roof-surface-shape | ✅apply  |
      | change-roof-surface-pitch-deg | 🏠️change-roof-surface-pitch-deg | ✅apply  |
      | change-roof-surface-extent | 🏠️change-roof-surface-extent | ✅apply  |
      | insert-roof-surface | 🏠️insert-roof-surface | ✅apply  |
      | remove-roof-surface | 🏠️remove-roof-surface | ✅apply  |
      | change-self-weight-thickness            | 🥞change-self-weight-thickness           | ✅apply  |
      | change-wind-face-z            | ↕️change-wind-face-z           | ✅apply  |
      | change-wind-face-loaded-area            | 🔲change-wind-face-loaded-area           | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1991 document from the parsed carrier
    Given the real committed text artifact asset://🏢de-office-compliant/🏢de-office-compliant/🗣️.dsl.semio
    And its committed binary twin asset://🏢de-office-compliant/🏢de-office-compliant/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
