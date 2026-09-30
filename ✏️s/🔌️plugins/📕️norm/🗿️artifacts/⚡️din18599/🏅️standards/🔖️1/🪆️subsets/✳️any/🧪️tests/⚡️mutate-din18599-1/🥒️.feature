@capability-din18599-1-mutate
@oracle-din18599-1-python-independent
@comparison-ordered-json-v1
@mutations-din18599-1-any
Feature: Apply every typed DIN V 18599 mutation against an independent Python implementation
  `s.norm.din18599` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `din18599` distribution, and the nearest real packages (`structuralcodes`,
  `concreteproperties`, `anastruct`) implement design-code FORMULAE and speak no interchange format, so
  none of them could be authoritative over `Din18599Mutation`. The second producer a differential comparison
  needs is therefore a second IMPLEMENTATION: `semio_norm_vocabulary`, imported by `🐍️.py` beside this
  file, reads every one of the 19 kinds from the naming mechanic (`new<Field>` sets the field its
  name spells) and the addressing convention (`<entity>Index` positions and `<entity>Id` native keys
  descend, in wire order, to the record the verb acts inside; inverses are computed from the base and
  are empty when the target is missing). It imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a declared `shared://` fixture the Rust producer wrote and the Python engine independently
  reached, so neither side holds a transcription that could drift. Nine kinds change a document scalar,
  six `specify-`/`update-` kinds replace one whole system facet, two `replace-` kinds replace the zone
  and element lists, `change-element-u` addresses one element by its native id, and `update-climate`
  addresses the composed climate CHILD, whose content-addressed handle no document here specifies: its
  row is the committed invariant refusal, which both sides must reject bit-identically; its applied wire
  form is witnessed payload-only under `🧾️wire-witness` and held by the crate's payload law.

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
      | id                       | dir                        | fixture                                  |
      | change-building-category | 🏠️change-building-category | 🎯️applies-building-category              |
      | change-attachment        | 🧱change-attachment         | 🎯️applies-attachment                     |
      | change-use-class         | 🏷️change-use-class         | 🏢️reclassifies-the-building-as-an-office |
      | change-method            | 🧮change-method             | 🎯️applies-method                         |
      | change-net-floor-area-m2 | 📐️change-net-floor-area-m2 | 📏️extends-net-floor-area-to-160-m2       |
      | change-heated-volume-m3  | 📦change-heated-volume-m3   | 🎯️applies-heated-volume-m3               |
      | change-geg-qp-factor     | ⚖️change-geg-qp-factor     | 🎯️applies-geg-qp-factor                  |
      | change-delta-u-wb        | 🌉change-delta-u-wb         | 🎯️applies-delta-u-wb                     |
      | change-automation-class  | 🎛️change-automation-class  | 🎯️applies-automation-class               |
      | specify-heating-system   | 🔥specify-heating-system    | 🎯️applies-specify-heating-system         |
      | specify-dhw-system       | 🚿specify-dhw-system        | 🎯️applies-specify-dhw-system             |
      | update-ventilation       | 🌬️update-ventilation       | 🎯️applies-update-ventilation             |
      | update-cooling           | ❄️update-cooling           | 🎯️applies-update-cooling                 |
      | update-lighting          | 💡update-lighting           | 🎯️applies-update-lighting                |
      | update-renewables        | ☀️update-renewables        | 🎯️applies-update-renewables              |
      | replace-zones            | 🗺️replace-zones            | 🎯️applies-replace-zones                  |
      | replace-elements         | 🧩replace-elements          | 🎯️applies-replace-elements               |
      | change-element-u         | 🌡️change-element-u         | 🎯️applies-change-element-u               |
      | update-climate           | 🌦️update-climate           | 🌧️refuses-a-negative-january-irradiance  |

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
      | id                       | dir                        | fixture                                  |
      | change-building-category | 🏠️change-building-category | 🎯️applies-building-category              |
      | change-attachment        | 🧱change-attachment         | 🎯️applies-attachment                     |
      | change-use-class         | 🏷️change-use-class         | 🏢️reclassifies-the-building-as-an-office |
      | change-method            | 🧮change-method             | 🎯️applies-method                         |
      | change-net-floor-area-m2 | 📐️change-net-floor-area-m2 | 📏️extends-net-floor-area-to-160-m2       |
      | change-heated-volume-m3  | 📦change-heated-volume-m3   | 🎯️applies-heated-volume-m3               |
      | change-geg-qp-factor     | ⚖️change-geg-qp-factor     | 🎯️applies-geg-qp-factor                  |
      | change-delta-u-wb        | 🌉change-delta-u-wb         | 🎯️applies-delta-u-wb                     |
      | change-automation-class  | 🎛️change-automation-class  | 🎯️applies-automation-class               |
      | specify-heating-system   | 🔥specify-heating-system    | 🎯️applies-specify-heating-system         |
      | specify-dhw-system       | 🚿specify-dhw-system        | 🎯️applies-specify-dhw-system             |
      | update-ventilation       | 🌬️update-ventilation       | 🎯️applies-update-ventilation             |
      | update-cooling           | ❄️update-cooling           | 🎯️applies-update-cooling                 |
      | update-lighting          | 💡update-lighting           | 🎯️applies-update-lighting                |
      | update-renewables        | ☀️update-renewables        | 🎯️applies-update-renewables              |
      | replace-zones            | 🗺️replace-zones            | 🎯️applies-replace-zones                  |
      | replace-elements         | 🧩replace-elements          | 🎯️applies-replace-elements               |
      | change-element-u         | 🌡️change-element-u         | 🎯️applies-change-element-u               |
      | update-climate           | 🌦️update-climate           | 🌧️refuses-a-negative-january-irradiance  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed DIN V 18599 document from the parsed carrier
    Given the real committed text artifact asset://🎬️demo/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
