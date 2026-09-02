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
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is
  it: all 32 kinds of this vocabulary, written in Python from the repository's own written
  specification of what a semantic mutation means — `📓️taxonomy.md`'s verb table, naming mechanics
  ("New-value fields are `new_<field>`") and addressing convention ("Inverse always computed from
  `base`", "Missing target ⇒ `inverse` returns `Vec::new()`"), and `📓️derivation-rules.md`'s shape
  rules — plus this subset's committed catalog for the closed list of kinds. It imports nothing from
  the Rust it judges and transliterates none of it: the document field a `new*` argument names is
  resolved by normalised spelling against the document's own keys, which is what the naming mechanic
  states, never from a table copied out of `🧬️mutations/**` — and the paragraph below names the
  spellings in THIS subset where that resolution can genuinely go wrong. The recorded no-oracle
  decision it replaces is gone from
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`, because there is now a
  reference to compare against.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)`
  path below is a declared `asset://` fixture, so neither side holds a transcription that could
  drift. All thirty-two kinds are flat `change-<field>` edits, but the field set is the most
  HETEROGENEOUS in the plugin: one document carries snow (`change-snow-zone`,
  `change-snow-altitude-m`), wind (`change-wind-zone`, `change-en-vbms`, `change-cd`, `change-cs`),
  thermal (`change-delta-tk`), crane (`change-crane-class`, `change-hoist-class`,
  `change-hoisting-speed-ms`), silo (`change-silo-k`, `change-silo-mu`,
  `change-silo-bulk-density-kn-m3`), bridge, accidental-impact and fire families side by side. The
  reading risk here is not spelling but SCOPE — an argument resolved into the wrong action family
  still names a real key. Each side then asserts the same three laws in role — the applied document
  must BE the committed after-snapshot; an `applied` vector must move the document and a `rejected`
  one must leave it bit-identical; and the mutation followed by its OWN computed inverse must
  restore the before-snapshot exactly. What `parity` adds on top is the only thing a single
  implementation can never provide: that two implementations, in two languages, written from one
  written specification, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the
  restored document is the before-document on all thirty-two rows; the mutated projection is the
  only half that tells the snow row from the silo row.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🗣️retail-hydrocarbon-fire.dsl.semio` — a named
  hydrocarbon-fire retail case rather than a generic demo, so the fire family (`change-fire-curve`,
  `change-fire-resistance-min`, `change-fire-member-capacity-c`) is exercised against a document
  that actually motivates it. It is still an authored case, not a submitted design. The carrier has
  no published grammar: the committed `📖️component.grammar.semio` is the repository-wide `payload =
  OCTET+` placeholder, so the two implementations are compared at the envelope preamble, the ordered
  `key=value` fields and the digest and length of what each re-emitted — never at a
  carrier-token-to-enum mapping this repository nowhere states.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id                                  | dir                                  | fixture                                           |
      | change-area-m2                      | 🧪change-area-m2                      | enlarges-loaded-area-to-360-m2                    |
      | change-category                     | 🧭change-category                     | reclassifies-imposed-load-to-category-d           |
      | change-annex                        | 🧫change-annex                        | switches-national-annex-to-en                     |
      | change-self-weight-material         | 🏷️change-self-weight-material        | switches-self-weight-material-to-structural-steel |
      | change-self-weight-thickness-m      | 🧮change-self-weight-thickness-m      | thickens-self-weight-layer-to-0-375-m             |
      | change-assumed-gk-kn-m2             | 🎢change-assumed-gk-kn-m2             | raises-assumed-gk-to-7-5-kn-m2                    |
      | change-fire-curve                   | 🔭change-fire-curve                   | switches-fire-curve-to-hydrocarbon                |
      | change-fire-resistance-min          | 🧱change-fire-resistance-min          | extends-fire-resistance-to-120-min                |
      | change-fire-member-capacity-c       | 🏛️change-fire-member-capacity-c      | raises-fire-member-capacity-to-700-c              |
      | change-snow-zone                    | 💧change-snow-zone                    | moves-site-to-snow-zone-3                         |
      | change-snow-altitude-m              | 🌡️change-snow-altitude-m             | lifts-snow-altitude-to-780-m                      |
      | change-en-sk-kn-m2                  | 🏗️change-en-sk-kn-m2                 | raises-en-characteristic-snow-load-to-1-25-kn-m2  |
      | change-wind-zone                    | 🌞change-wind-zone                    | moves-site-to-wind-zone-4                         |
      | change-en-vbms                      | 📈change-en-vbms                      | raises-en-basic-wind-speed-to-30-m-s              |
      | change-delta-tk                     | ⚡change-delta-tk                     | raises-thermal-delta-tk-to-45-k                   |
      | change-construction-activity        | 🪟change-construction-activity        | switches-construction-activity-to-concreting      |
      | change-accidental-mass-t            | 🎚️change-accidental-mass-t           | lightens-impact-vehicle-to-12-5-t                 |
      | change-accidental-speed-km-h        | 🕹️change-accidental-speed-km-h       | lowers-impact-speed-to-50-km-h                    |
      | change-bridge-lane                  | ❄️change-bridge-lane                 | widens-carriageway-to-3-notional-lanes            |
      | change-bridge-span-m                | 🎯change-bridge-span-m                | lengthens-bridge-span-to-36-m                     |
      | change-bridge-lane-width-m          | 🗺️change-bridge-lane-width-m         | widens-notional-lane-to-3-5-m                     |
      | change-bridge-moment-resistance-knm | 📡change-bridge-moment-resistance-knm | raises-bridge-moment-resistance-to-4500-knm       |
      | change-crane-class                  | 🔀change-crane-class                  | upgrades-crane-to-class-hc3                       |
      | change-hoist-class                  | 🧊change-hoist-class                  | upgrades-hoist-to-class-hc4                       |
      | change-hoisting-speed-ms            | 🌬️change-hoisting-speed-ms           | speeds-hoisting-to-1-25-m-s                       |
      | change-silo-bulk-density-kn-m3      | 🌗change-silo-bulk-density-kn-m3      | raises-silo-bulk-density-to-10-5-kn-m3            |
      | change-silo-height-m                | 🎛️change-silo-height-m               | raises-silo-to-18-m                               |
      | change-silo-hydraulic-radius-m      | 📊change-silo-hydraulic-radius-m      | widens-silo-hydraulic-radius-to-2-25-m            |
      | change-silo-mu                      | 📉change-silo-mu                      | raises-silo-wall-friction-mu-to-0-625             |
      | change-silo-k                       | 🔬change-silo-k                       | raises-silo-lateral-pressure-ratio-k-to-0-625     |
      | change-cs                           | 🔥change-cs                           | raises-size-factor-cs-to-1-125                    |
      | change-cd                           | 🔆change-cd                           | lowers-dynamic-factor-cd-to-0-875                 |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id                                  | dir                                  | fixture                                           |
      | change-area-m2                      | 🧪change-area-m2                      | enlarges-loaded-area-to-360-m2                    |
      | change-category                     | 🧭change-category                     | reclassifies-imposed-load-to-category-d           |
      | change-annex                        | 🧫change-annex                        | switches-national-annex-to-en                     |
      | change-self-weight-material         | 🏷️change-self-weight-material        | switches-self-weight-material-to-structural-steel |
      | change-self-weight-thickness-m      | 🧮change-self-weight-thickness-m      | thickens-self-weight-layer-to-0-375-m             |
      | change-assumed-gk-kn-m2             | 🎢change-assumed-gk-kn-m2             | raises-assumed-gk-to-7-5-kn-m2                    |
      | change-fire-curve                   | 🔭change-fire-curve                   | switches-fire-curve-to-hydrocarbon                |
      | change-fire-resistance-min          | 🧱change-fire-resistance-min          | extends-fire-resistance-to-120-min                |
      | change-fire-member-capacity-c       | 🏛️change-fire-member-capacity-c      | raises-fire-member-capacity-to-700-c              |
      | change-snow-zone                    | 💧change-snow-zone                    | moves-site-to-snow-zone-3                         |
      | change-snow-altitude-m              | 🌡️change-snow-altitude-m             | lifts-snow-altitude-to-780-m                      |
      | change-en-sk-kn-m2                  | 🏗️change-en-sk-kn-m2                 | raises-en-characteristic-snow-load-to-1-25-kn-m2  |
      | change-wind-zone                    | 🌞change-wind-zone                    | moves-site-to-wind-zone-4                         |
      | change-en-vbms                      | 📈change-en-vbms                      | raises-en-basic-wind-speed-to-30-m-s              |
      | change-delta-tk                     | ⚡change-delta-tk                     | raises-thermal-delta-tk-to-45-k                   |
      | change-construction-activity        | 🪟change-construction-activity        | switches-construction-activity-to-concreting      |
      | change-accidental-mass-t            | 🎚️change-accidental-mass-t           | lightens-impact-vehicle-to-12-5-t                 |
      | change-accidental-speed-km-h        | 🕹️change-accidental-speed-km-h       | lowers-impact-speed-to-50-km-h                    |
      | change-bridge-lane                  | ❄️change-bridge-lane                 | widens-carriageway-to-3-notional-lanes            |
      | change-bridge-span-m                | 🎯change-bridge-span-m                | lengthens-bridge-span-to-36-m                     |
      | change-bridge-lane-width-m          | 🗺️change-bridge-lane-width-m         | widens-notional-lane-to-3-5-m                     |
      | change-bridge-moment-resistance-knm | 📡change-bridge-moment-resistance-knm | raises-bridge-moment-resistance-to-4500-knm       |
      | change-crane-class                  | 🔀change-crane-class                  | upgrades-crane-to-class-hc3                       |
      | change-hoist-class                  | 🧊change-hoist-class                  | upgrades-hoist-to-class-hc4                       |
      | change-hoisting-speed-ms            | 🌬️change-hoisting-speed-ms           | speeds-hoisting-to-1-25-m-s                       |
      | change-silo-bulk-density-kn-m3      | 🌗change-silo-bulk-density-kn-m3      | raises-silo-bulk-density-to-10-5-kn-m3            |
      | change-silo-height-m                | 🎛️change-silo-height-m               | raises-silo-to-18-m                               |
      | change-silo-hydraulic-radius-m      | 📊change-silo-hydraulic-radius-m      | widens-silo-hydraulic-radius-to-2-25-m            |
      | change-silo-mu                      | 📉change-silo-mu                      | raises-silo-wall-friction-mu-to-0-625             |
      | change-silo-k                       | 🔬change-silo-k                       | raises-silo-lateral-pressure-ratio-k-to-0-625     |
      | change-cs                           | 🔥change-cs                           | raises-size-factor-cs-to-1-125                    |
      | change-cd                           | 🔆change-cd                           | lowers-dynamic-factor-cd-to-0-875                 |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1991 document from the parsed carrier
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🧪️retail-hydrocarbon-fire/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
