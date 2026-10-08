@capability-din16798-1-mutate
@oracle-din16798-1-python-independent
@comparison-ordered-json-v1
@mutations-din16798-1-any
Feature: Apply every typed DIN EN 16798 mutation against an independent Python implementation
  `s.norm.din16798` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `din16798` distribution, and the nearest real packages (`structuralcodes`,
  `concreteproperties`, `anastruct`) implement design-code FORMULAE and speak no interchange format, so
  none of them could be authoritative over `Din16798Mutation`. The second producer a differential comparison
  needs is therefore a second IMPLEMENTATION: `semio_norm_vocabulary`, imported by `🐍️.py` beside this
  file, reads every one of the 41 kinds from the naming mechanic (`new<Field>` sets the field its
  name spells) and the addressing convention (`<entity>Index` positions and `<entity>Id` native keys
  descend, in wire order, to the record the verb acts inside; inverses are computed from the base and
  are empty when the target is missing). It imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a declared `shared://` fixture the Rust producer wrote and the Python engine independently
  reached, so neither side holds a transcription that could drift. The 41 kinds edit an indoor-
  environment building: document scalars (`change-theta-rm`, `change-envelope-n50`), zone fields
  addressed by the zone's native id (`{zoneId}`), ventilation-system fields addressed by `{ventId}`
  — whose collection is `ventSystems`, so the id itself, not a spelling, locates it — and the insert
  and remove pairs of both collections, whose inverses must restore the removed record at its position.

  Each side asserts the same laws in role — the applied document must BE the committed after-snapshot,
  an `applied` vector must move the document and a `no-op` or `rejected` one must leave it bit-identical
  (a rejected one under its committed outcome code), and the mutation followed by its OWN computed
  inverse must restore the before-snapshot exactly. `inverse-` projects BOTH the mutated and the
  restored document, because the restored one is always the before-snapshot and projecting only it
  would make the differential vacuous.

  Each insert and remove has further rows. `<kind>-dupe` re-applies an insert to its own after-snapshot,
  whose id is already held — a `mutation.duplicate-id` refusal; `<kind>-gone` re-applies a remove to
  the document it already left — a `mutation.target-missing` refusal; both sides must leave the document
  bit-identical, so neither has an inverse row. `<kind>-clamp` asks an insert for a position past its
  list's end: both sides insert last, where the canonical append landed, and production reports it as a
  `mutation.clamped` warning.

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
      | id                           | dir                            | fixture |
      | change-annex                 | 🌍️change-annex                 | ✅apply  |
      | change-theta-rm              | 🔄️change-theta-rm              | ✅apply  |
      | change-outdoor-co2           | 🌫️change-outdoor-co2           | ✅apply  |
      | change-envelope-n50          | 🏠️change-envelope-n50          | ✅apply  |
      | change-envelope-volume       | 📦️change-envelope-volume       | ✅apply  |
      | change-cellar-area           | 🏚️change-cellar-area           | ✅apply  |
      | change-cellar-ventilation    | 🌀change-cellar-ventilation     | ✅apply  |
      | change-night-setback         | 🌙️change-night-setback         | ✅apply  |
      | insert-zone                  | ➕️insert-zone                  | ✅apply  |
      | insert-zone-dupe             | ➕️insert-zone                  | ⛔dupe   |
      | insert-zone-clamp            | ➕️insert-zone                  | 📏clamp  |
      | remove-zone                  | ➖️remove-zone                  | ✅apply  |
      | remove-zone-middle-row       | ➖️remove-zone                  | 🔬️middle-row |
      | remove-zone-gone             | ➖️remove-zone                  | ❓gone   |
      | change-zone-usage-type       | 🏢️change-zone-usage-type       | ✅apply  |
      | change-zone-floor-area       | 📐️change-zone-floor-area       | ✅apply  |
      | change-zone-occupants        | 👥️change-zone-occupants        | ✅apply  |
      | change-zone-comfort-category | 🛋️change-zone-comfort-category | ✅apply  |
      | change-zone-pollution-class  | 🏭️change-zone-pollution-class  | ✅apply  |
      | change-zone-comfort-model    | 🧭️change-zone-comfort-model    | ✅apply  |
      | change-zone-t-op-winter      | ❄️change-zone-t-op-winter      | ✅apply  |
      | change-zone-t-op-summer      | ☀️change-zone-t-op-summer      | ✅apply  |
      | change-zone-air-speed        | 💨change-zone-air-speed         | ✅apply  |
      | change-zone-clothing         | 👔change-zone-clothing          | ✅apply  |
      | change-zone-metabolic-rate   | 🏃️change-zone-metabolic-rate   | ✅apply  |
      | change-zone-rh               | 💧️change-zone-rh               | ✅apply  |
      | change-zone-outdoor-air      | 💨️change-zone-outdoor-air      | ✅apply  |
      | change-zone-co2              | 🫧change-zone-co2               | ✅apply  |
      | change-zone-illuminance      | 💡change-zone-illuminance       | ✅apply  |
      | change-zone-noise            | 🔊️change-zone-noise            | ✅apply  |
      | change-zone-vent-system-id   | 🔗change-zone-vent-system-id    | ✅apply  |
      | change-zone-turbulence       | 💨change-zone-turbulence        | ✅apply  |
      | change-zone-vent-method      | 📐️change-zone-vent-method      | ✅apply  |
      | insert-vent-system           | 🆕️insert-vent-system           | ✅apply  |
      | insert-vent-system-dupe      | 🆕️insert-vent-system           | ⛔dupe   |
      | insert-vent-system-clamp     | 🆕️insert-vent-system           | 📏clamp  |
      | remove-vent-system           | 🗑️remove-vent-system           | ✅apply  |
      | remove-vent-system-middle-row | 🗑️remove-vent-system           | 🔬️middle-row |
      | remove-vent-system-gone      | 🗑️remove-vent-system           | ❓gone   |
      | change-vent-system-type      | ⚙️change-vent-system-type      | ✅apply  |
      | change-vent-sfp              | 🌀️change-vent-sfp              | ✅apply  |
      | change-vent-sfp-class        | 🎓️change-vent-sfp-class        | ✅apply  |
      | change-vent-heat-recovery    | ♻️change-vent-heat-recovery    | ✅apply  |
      | change-vent-oda-class        | 🏞️change-vent-oda-class        | ✅apply  |
      | change-vent-filter-sup       | 🧽change-vent-filter-sup        | ✅apply  |
      | change-vent-inspection       | 📅️change-vent-inspection       | ✅apply  |
      | change-vent-duct-class       | 🧱change-vent-duct-class        | ✅apply  |
      | change-vent-duct-leakage     | 🕳️change-vent-duct-leakage     | ✅apply  |
      | change-vent-design-airflow   | 🌬️change-vent-design-airflow   | ✅apply  |

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
      | id                           | dir                            | fixture |
      | change-annex                 | 🌍️change-annex                 | ✅apply  |
      | change-theta-rm              | 🔄️change-theta-rm              | ✅apply  |
      | change-outdoor-co2           | 🌫️change-outdoor-co2           | ✅apply  |
      | change-envelope-n50          | 🏠️change-envelope-n50          | ✅apply  |
      | change-envelope-volume       | 📦️change-envelope-volume       | ✅apply  |
      | change-cellar-area           | 🏚️change-cellar-area           | ✅apply  |
      | change-cellar-ventilation    | 🌀change-cellar-ventilation     | ✅apply  |
      | change-night-setback         | 🌙️change-night-setback         | ✅apply  |
      | insert-zone                  | ➕️insert-zone                  | ✅apply  |
      | remove-zone                  | ➖️remove-zone                  | ✅apply  |
      | change-zone-usage-type       | 🏢️change-zone-usage-type       | ✅apply  |
      | change-zone-floor-area       | 📐️change-zone-floor-area       | ✅apply  |
      | change-zone-occupants        | 👥️change-zone-occupants        | ✅apply  |
      | change-zone-comfort-category | 🛋️change-zone-comfort-category | ✅apply  |
      | change-zone-pollution-class  | 🏭️change-zone-pollution-class  | ✅apply  |
      | change-zone-comfort-model    | 🧭️change-zone-comfort-model    | ✅apply  |
      | change-zone-t-op-winter      | ❄️change-zone-t-op-winter      | ✅apply  |
      | change-zone-t-op-summer      | ☀️change-zone-t-op-summer      | ✅apply  |
      | change-zone-air-speed        | 💨change-zone-air-speed         | ✅apply  |
      | change-zone-clothing         | 👔change-zone-clothing          | ✅apply  |
      | change-zone-metabolic-rate   | 🏃️change-zone-metabolic-rate   | ✅apply  |
      | change-zone-rh               | 💧️change-zone-rh               | ✅apply  |
      | change-zone-outdoor-air      | 💨️change-zone-outdoor-air      | ✅apply  |
      | change-zone-co2              | 🫧change-zone-co2               | ✅apply  |
      | change-zone-illuminance      | 💡change-zone-illuminance       | ✅apply  |
      | change-zone-noise            | 🔊️change-zone-noise            | ✅apply  |
      | change-zone-vent-system-id   | 🔗change-zone-vent-system-id    | ✅apply  |
      | change-zone-turbulence       | 💨change-zone-turbulence        | ✅apply  |
      | change-zone-vent-method      | 📐️change-zone-vent-method      | ✅apply  |
      | insert-vent-system           | 🆕️insert-vent-system           | ✅apply  |
      | remove-vent-system           | 🗑️remove-vent-system           | ✅apply  |
      | change-vent-system-type      | ⚙️change-vent-system-type      | ✅apply  |
      | change-vent-sfp              | 🌀️change-vent-sfp              | ✅apply  |
      | change-vent-sfp-class        | 🎓️change-vent-sfp-class        | ✅apply  |
      | change-vent-heat-recovery    | ♻️change-vent-heat-recovery    | ✅apply  |
      | change-vent-oda-class        | 🏞️change-vent-oda-class        | ✅apply  |
      | change-vent-filter-sup       | 🧽change-vent-filter-sup        | ✅apply  |
      | change-vent-inspection       | 📅️change-vent-inspection       | ✅apply  |
      | change-vent-duct-class       | 🧱change-vent-duct-class        | ✅apply  |
      | change-vent-duct-leakage     | 🕳️change-vent-duct-leakage     | ✅apply  |
      | change-vent-design-airflow   | 🌬️change-vent-design-airflow   | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed DIN EN 16798 document from the parsed carrier
    Given the real committed text artifact asset://🎬️demo/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
