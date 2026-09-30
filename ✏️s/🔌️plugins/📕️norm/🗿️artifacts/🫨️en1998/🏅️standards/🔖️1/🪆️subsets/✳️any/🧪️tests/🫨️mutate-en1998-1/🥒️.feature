@capability-en1998-1-mutate
@oracle-en1998-1-python-independent
@comparison-ordered-json-v1
@mutations-en1998-1-any
Feature: Apply every typed EN 1998 mutation against an independent Python implementation
  `s.norm.en1998` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1998` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1998Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this
  subset's kind list, vectors and carrier. It is written from the repository's own specification of what
  a semantic mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention
  and the derivation rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)`
  path below is a declared `shared://` fixture, so neither side holds a transcription that could
  drift. All twenty-nine vectors start from the committed `🏢️seismic-multipart` example, the one
  document that carries every EN 1998 part at once — an office RC frame (part 1) with two lateral
  systems, four storeys and two members, a bridge (part 2), an assessed element (part 3), a silo and a
  tank (part 4), a foundation and a retaining wall (part 5) and a tower (part 6). The vocabulary has
  three addressing shapes and each is exercised: the document-level `change-annex` and the whole-facet
  `update-site`; the positional `insert-<part>` / `remove-<part>` pairs and `change-<part>-<field>
  {index}` of every part list; and the NESTED `{buildingIndex, storeyIndex|systemIndex|memberIndex}`
  addressing of the building's own storeys, lateral systems and members, which is where a resolution
  into the wrong list would still find a field of the right name. Each side asserts the same laws in
  role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot
  exactly, list position included. `parity` adds that two implementations, in two languages, reach the
  same document.

  `inverse-` projects BOTH the mutated and the restored document: for every `insert-<part>` the inverse
  is the matching `remove-<part>` at the inserted position, and for every `remove-<part>` it is the
  `insert-<part>` of the removed record, so the mutated half is what distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `📚️examples/🏢️seismic-rc-frame/🖼️assets/🏢️seismic-rc-frame/🗣️.dsl.semio` — a named
  reinforced-concrete frame. The carrier has no published grammar: the committed
  `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so the two sides
  are compared at the envelope preamble, the ordered `key=value` fields and the digest and length of
  what each re-emitted. The Rust side additionally proves it PARSED the document: the committed binary
  twin must decode to the same document as the text, through a separately written codec.

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
      | id                           | dir                            | fixture             |
      | change-annex                 | 📎️change-annex                 | 🌍️switches-to-en    |
      | update-site                  | 🌚️update-site                  | 🌋️zone-3-soft-soil  |
      | insert-building              | ➕️insert-building              | 🏢️adds-an-annex     |
      | remove-building              | ➖️remove-building              | 🏚️drops-the-office  |
      | change-system-v-rd-n         | 💪️change-system-v-rd-n         | 💪️stronger-y        |
      | change-storey-permanent-gk-n | ⚖️change-storey-permanent-gk-n | ⚖️heavier           |
      | change-storey-stiffness-x    | 📐️change-storey-stiffness-x    | 📐️softer            |
      | change-storey-drift-xm       | 📏️change-storey-drift-xm       | 📏️more-drift        |
      | change-building-plan-regular | 🧭️change-building-plan-regular | 🧭️twist             |
      | change-elevation-regular     | 📏️change-elevation-regular     | 🏙️irregular         |
      | change-member-detailing      | ✅️change-member-detailing      | ✅️beam-unfit        |
      | change-masonry-wall-ratio    | 🧱️change-masonry-wall-ratio    | 🧱️four-pct          |
      | insert-bridge                | 🌉insert-bridge                 | 🌉️adds-a-viaduct    |
      | change-bridge-v-rd-n         | 🛑️change-bridge-v-rd-n         | 🛑️stronger-pier     |
      | insert-assessment            | 🔧insert-assessment             | 🔧️adds-a-kl3-check  |
      | change-assessment-rkn        | 🏋️change-assessment-rkn        | 🏋️stronger          |
      | insert-silo                  | 🫙insert-silo                   | 🫙️adds-a-grain-silo |
      | insert-tank                  | 🛢insert-tank                   | 🛢️adds-a-water-tank |
      | insert-foundation            | 🪨insert-foundation             | 🪨️adds-a-pad        |
      | insert-retaining-wall        | 🧱️insert-retaining-wall        | 🧱️adds-a-wall       |
      | insert-tower                 | 🗼insert-tower                  | 🏭️adds-a-chimney    |
      | change-tower-m-rd-nm         | ↪️change-tower-m-rd-nm         | ↪️stronger-base     |
      | remove-bridge                | ➖️remove-bridge                | 🌉️drops-the-viaduct |
      | remove-assessment            | ➖️remove-assessment            | 🔧️drops-the-check   |
      | remove-silo                  | ➖️remove-silo                  | 🫙️drops-the-silo    |
      | remove-tank                  | ➖️remove-tank                  | 🛢️drops-the-tank    |
      | remove-foundation            | ➖️remove-foundation            | 🪨️drops-the-pad     |
      | remove-retaining-wall        | ➖️remove-retaining-wall        | 🧱️drops-it          |
      | remove-tower                 | ➖️remove-tower                 | 🗼️drops-the-tower   |

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
      | id                           | dir                            | fixture             |
      | change-annex                 | 📎️change-annex                 | 🌍️switches-to-en    |
      | update-site                  | 🌚️update-site                  | 🌋️zone-3-soft-soil  |
      | insert-building              | ➕️insert-building              | 🏢️adds-an-annex     |
      | remove-building              | ➖️remove-building              | 🏚️drops-the-office  |
      | change-system-v-rd-n         | 💪️change-system-v-rd-n         | 💪️stronger-y        |
      | change-storey-permanent-gk-n | ⚖️change-storey-permanent-gk-n | ⚖️heavier           |
      | change-storey-stiffness-x    | 📐️change-storey-stiffness-x    | 📐️softer            |
      | change-storey-drift-xm       | 📏️change-storey-drift-xm       | 📏️more-drift        |
      | change-building-plan-regular | 🧭️change-building-plan-regular | 🧭️twist             |
      | change-elevation-regular     | 📏️change-elevation-regular     | 🏙️irregular         |
      | change-member-detailing      | ✅️change-member-detailing      | ✅️beam-unfit        |
      | change-masonry-wall-ratio    | 🧱️change-masonry-wall-ratio    | 🧱️four-pct          |
      | insert-bridge                | 🌉insert-bridge                 | 🌉️adds-a-viaduct    |
      | change-bridge-v-rd-n         | 🛑️change-bridge-v-rd-n         | 🛑️stronger-pier     |
      | insert-assessment            | 🔧insert-assessment             | 🔧️adds-a-kl3-check  |
      | change-assessment-rkn        | 🏋️change-assessment-rkn        | 🏋️stronger          |
      | insert-silo                  | 🫙insert-silo                   | 🫙️adds-a-grain-silo |
      | insert-tank                  | 🛢insert-tank                   | 🛢️adds-a-water-tank |
      | insert-foundation            | 🪨insert-foundation             | 🪨️adds-a-pad        |
      | insert-retaining-wall        | 🧱️insert-retaining-wall        | 🧱️adds-a-wall       |
      | insert-tower                 | 🗼insert-tower                  | 🏭️adds-a-chimney    |
      | change-tower-m-rd-nm         | ↪️change-tower-m-rd-nm         | ↪️stronger-base     |
      | remove-bridge                | ➖️remove-bridge                | 🌉️drops-the-viaduct |
      | remove-assessment            | ➖️remove-assessment            | 🔧️drops-the-check   |
      | remove-silo                  | ➖️remove-silo                  | 🫙️drops-the-silo    |
      | remove-tank                  | ➖️remove-tank                  | 🛢️drops-the-tank    |
      | remove-foundation            | ➖️remove-foundation            | 🪨️drops-the-pad     |
      | remove-retaining-wall        | ➖️remove-retaining-wall        | 🧱️drops-it          |
      | remove-tower                 | ➖️remove-tower                 | 🗼️drops-the-tower   |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1998 document from the parsed carrier
    Given the real committed text artifact asset://🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio
    And its committed binary twin asset://🏢️seismic-rc-frame/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
