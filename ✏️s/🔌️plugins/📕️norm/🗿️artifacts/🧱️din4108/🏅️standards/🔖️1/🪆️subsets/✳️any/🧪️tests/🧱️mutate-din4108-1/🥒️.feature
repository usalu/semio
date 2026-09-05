@capability-din4108-1-mutate
@oracle-din4108-1-python-independent
@comparison-ordered-json-v1
@mutations-din4108-1-any
Feature: Apply every typed DIN 4108 mutation against an independent Python implementation
  `s.norm.din4108` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `din4108` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `Din4108Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is
  it: all 22 kinds of this vocabulary, written in Python from the repository's own written
  specification of what a semantic mutation means — `📓️taxonomy.md`'s verb table, naming mechanics
  ("New-value fields are `new_<field>`") and addressing convention ("Inverse always computed from
  `base`", "Missing target ⇒ `inverse` returns `Vec::new()`"), and `📓️derivation-rules.md`'s shape
  rules — plus this subset's committed catalog for the closed list of kinds. It imports nothing from
  the Rust it judges and transliterates none of it: the document field a `new*` argument names is
  resolved by normalised spelling against the document's own keys, which is what the naming mechanic
  states, never from a table copied out of `🧬️mutations/**` — and the paragraph below names the
  spellings in THIS subset where that resolution can genuinely go wrong. The recorded no-oracle
  decision it replaces is gone from
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`, because there is now a
  reference to compare against.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)`
  path below is a declared `asset://` fixture, so neither side holds a transcription that could
  drift. What the second reading has to get right HERE is that nineteen of the twenty-two kinds are
  flat scalar edits on the envelope record — `airtightness-n50`, `rh-int`, `psi-times-l-sum`,
  `solar-absorptance` — while `insert-layer`, `remove-layer` and `reorder-layers` address the
  `layers` build-up BY POSITION. A layer list that shifted the wrong way is invisible to any scalar
  reading, so those three rows are the only ones in this subset where the two implementations can
  disagree about structure rather than about a number. Each side then asserts the same three laws in
  role — the applied document must BE the committed after-snapshot; an `applied` vector must move
  the document and a `rejected` one must leave it bit-identical; and the mutation followed by its
  OWN computed inverse must restore the before-snapshot exactly. What `parity` adds on top is the
  only thing a single implementation can never provide: that two implementations, in two languages,
  written from one written specification, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document, because for nineteen scalar kinds
  the restored document is the before-document and projecting only it would make every row of the
  table report the same value. On `insert-layer`/`remove-layer`/`reorder-layers` the mutated
  projection is the only place the layer ORDER is observable at all.

  ⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads
  `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, a 455-byte hand-authored single-build-up demo:
  one `category=residential` envelope over a two-entry `layers [thickness-m:QTY lambda-w-mk:NUM]`
  build-up. It is the smallest committed document in this plugin and it is NOT a real DIN 4108
  verification — no measured n50 report, no real material catalogue. It exercises the grammar, not
  the domain, and that is the ceiling on what this case's identity evidence proves. The carrier
  itself has no published grammar either: this subset's committed
  `🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` is the repository-wide `payload = OCTET+`
  placeholder, so the two implementations are compared on the envelope preamble, the ordered
  `key=value` fields and the `layers` block as written, plus the digest and length of what each side
  re-emitted — never on a mapping from carrier tokens to the JSON snapshot's enum spellings, which
  is stated nowhere.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id                                | dir                                | fixture                                              |
      | change-category                   | 🗂️change-category                   | retypes-the-assembly-as-office                       |
      | change-climate                    | 🌦️change-climate                   | moves-the-building-to-climate-zone-4                 |
      | change-airtightness-n50           | 💨️change-airtightness-n50           | tightens-n50-to-1-point-5-per-hour                   |
      | change-psi-times-l-sum            | 🌉️change-psi-times-l-sum            | raises-the-thermal-bridge-sum-to-0-point-05          |
      | change-rh-int                     | 💧️change-rh-int                     | raises-indoor-relative-humidity-to-0-point-65        |
      | change-catalog-id                 | 📇️change-catalog-id                 | repoints-the-catalogue-entry-to-aw-07                |
      | change-material-id                | 🧽️change-material-id                | swaps-the-insulation-material-to-eps                 |
      | change-airtightness-class         | 🔒️change-airtightness-class         | upgrades-the-airtightness-class-to-class1            |
      | change-t-int-c                    | 🌡️change-t-int-c                    | raises-the-indoor-design-temperature-to-22-point-5-c |
      | change-solar-absorptance          | ☀️change-solar-absorptance          | lightens-the-facade-to-absorptance-0-point-25        |
      | change-irradiance-wm2             | 🔆️change-irradiance-wm2             | raises-design-irradiance-to-750-w-per-m2             |
      | change-moisture-mu-exterior       | 🌧️change-moisture-mu-exterior       | raises-the-exterior-mu-value-to-20                   |
      | change-moisture-mu-interior       | 💦️change-moisture-mu-interior       | raises-the-interior-mu-value-to-2-point-5            |
      | change-envelope-area-m2           | 📐️change-envelope-area-m2           | grows-the-envelope-to-150-m2                         |
      | change-bb2-details-conform        | ✅️change-bb2-details-conform        | declares-the-beiblatt-2-details-non-conforming       |
      | change-application-type           | 🧩️change-application-type           | reclassifies-the-application-type-as-wab             |
      | change-declared-application-class | 🏷️change-declared-application-class | declares-application-class-kh                        |
      | insert-layer                      | ➕️insert-layer                      | inserts-an-interior-plaster-layer-at-index-1         |
      | remove-layer                      | ➖️remove-layer                     | removes-the-load-bearing-masonry-layer               |
      | reorder-layers                    | 🔀️reorder-layers                    | moves-the-insulation-in-front-of-the-masonry         |
      | change-layer-thickness            | 📏️change-layer-thickness            | thickens-the-insulation-layer-to-0-point-2-m         |
      | change-layer-lambda               | 🧊️change-layer-lambda               | degrades-the-masonry-lambda-to-0-point-5             |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id                                | dir                                | fixture                                              |
      | change-category                   | 🗂️change-category                   | retypes-the-assembly-as-office                       |
      | change-climate                    | 🌦️change-climate                   | moves-the-building-to-climate-zone-4                 |
      | change-airtightness-n50           | 💨️change-airtightness-n50           | tightens-n50-to-1-point-5-per-hour                   |
      | change-psi-times-l-sum            | 🌉️change-psi-times-l-sum            | raises-the-thermal-bridge-sum-to-0-point-05          |
      | change-rh-int                     | 💧️change-rh-int                     | raises-indoor-relative-humidity-to-0-point-65        |
      | change-catalog-id                 | 📇️change-catalog-id                 | repoints-the-catalogue-entry-to-aw-07                |
      | change-material-id                | 🧽️change-material-id                | swaps-the-insulation-material-to-eps                 |
      | change-airtightness-class         | 🔒️change-airtightness-class         | upgrades-the-airtightness-class-to-class1            |
      | change-t-int-c                    | 🌡️change-t-int-c                    | raises-the-indoor-design-temperature-to-22-point-5-c |
      | change-solar-absorptance          | ☀️change-solar-absorptance          | lightens-the-facade-to-absorptance-0-point-25        |
      | change-irradiance-wm2             | 🔆️change-irradiance-wm2             | raises-design-irradiance-to-750-w-per-m2             |
      | change-moisture-mu-exterior       | 🌧️change-moisture-mu-exterior       | raises-the-exterior-mu-value-to-20                   |
      | change-moisture-mu-interior       | 💦️change-moisture-mu-interior       | raises-the-interior-mu-value-to-2-point-5            |
      | change-envelope-area-m2           | 📐️change-envelope-area-m2           | grows-the-envelope-to-150-m2                         |
      | change-bb2-details-conform        | ✅️change-bb2-details-conform        | declares-the-beiblatt-2-details-non-conforming       |
      | change-application-type           | 🧩️change-application-type           | reclassifies-the-application-type-as-wab             |
      | change-declared-application-class | 🏷️change-declared-application-class | declares-application-class-kh                        |
      | insert-layer                      | ➕️insert-layer                      | inserts-an-interior-plaster-layer-at-index-1         |
      | remove-layer                      | ➖️remove-layer                     | removes-the-load-bearing-masonry-layer               |
      | reorder-layers                    | 🔀️reorder-layers                    | moves-the-insulation-in-front-of-the-masonry         |
      | change-layer-thickness            | 📏️change-layer-thickness            | thickens-the-insulation-layer-to-0-point-2-m         |
      | change-layer-lambda               | 🧊️change-layer-lambda               | degrades-the-masonry-lambda-to-0-point-5             |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed DIN 4108 document from the parsed carrier
    Given the real committed text artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
