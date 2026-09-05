@capability-en1997-1-mutate
@oracle-en1997-1-python-independent
@comparison-ordered-json-v1
@mutations-en1997-1-any
Feature: Apply every typed EN 1997 mutation against an independent Python implementation
  `s.norm.en1997` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1997` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1997Mutation` vocabulary. The second producer a differential
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
  drift. All twenty-two kinds are flat `change-<field>` edits over one geotechnical record, and the
  reading hazard is the PILE family: `change-pile-base-area-m2`, `change-pile-dm`, `change-pile-lm`
  and `change-pile-n-profiles` sit beside the spread-footing keys (`change-footing-area-m2`,
  `change-b-m`, `change-dfm`) they mirror. `change-design-approach` is the one enum, and it selects
  which of the two families a verification even reads. Each side then asserts the same three laws in
  role — the applied document must BE the committed after-snapshot; an `applied` vector must move
  the document and a `rejected` one must leave it bit-identical; and the mutation followed by its
  OWN computed inverse must restore the before-snapshot exactly. What `parity` adds on top is the
  only thing a single implementation can never provide: that two implementations, in two languages,
  written from one written specification, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the
  restored document repeats the before-document on all twenty-two rows; the mutated projection is
  what separates the pile row from the footing row that mirrors it.

  ⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads
  `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, 330 bytes — the smallest committed document in
  the plugin, a single line of demo values (`v-ed-kn=500 h-ed-kn=80 footing-area-m2=2 phi-deg=30
  …`). It touches every field once and nothing more: no ground-investigation record, no real design
  approach comparison, no second soil layer. Unlike its nine Eurocode siblings this subset has NO
  named example case, and that is a real gap in its evidence rather than a stylistic difference. The
  carrier has no published grammar either: the committed `📖️component.grammar.semio` is the
  repository-wide `payload = OCTET+` placeholder, so the two sides are compared at the envelope
  preamble, the ordered `key=value` fields and the digest and length of what each re-emitted.

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
      | id                         | dir                         | fixture                                                 |
      | change-v-ed-kn             | 🪒change-v-ed-kn             | raises-the-design-vertical-load-to-750-kn               |
      | change-h-ed-kn             | 🪥change-h-ed-kn             | raises-the-design-horizontal-load-to-120-kn             |
      | change-footing-area-m2     | 🧴change-footing-area-m2     | enlarges-the-footing-area-to-6-25-m2                    |
      | change-phi-deg             | 🧼change-phi-deg             | raises-the-friction-angle-to-35-degrees                 |
      | change-c-kpa               | 🧽change-c-kpa               | gives-the-drained-sand-12-5-kpa-of-effective-cohesion   |
      | change-gamma-kn-m3         | 🪠change-gamma-kn-m3         | raises-the-soil-unit-weight-to-20-kn-m3                 |
      | change-bm                  | 🧹change-bm                  | widens-the-footing-to-2-5-m                             |
      | change-dfm                 | 🧺change-dfm                 | deepens-the-founding-level-to-2-m                       |
      | change-es-mpa              | 🪑change-es-mpa              | stiffens-the-soil-modulus-to-45-mpa                     |
      | change-nu                  | 🪞change-nu                  | raises-poissons-ratio-to-0-375                          |
      | change-design-approach     | 🛋️change-design-approach    | switches-from-design-approach-1-to-design-approach-2    |
      | change-annex               | 🛏️change-annex              | switches-from-the-german-na-to-the-recommended-en-annex |
      | change-settlement-limit-mm | 🚿change-settlement-limit-mm | relaxes-the-settlement-limit-to-40-mm                   |
      | change-n-pile-ed-kn        | 🛁change-n-pile-ed-kn        | raises-the-design-pile-axial-load-to-1200-kn            |
      | change-alpha-s             | 🌿change-alpha-s             | lowers-the-shaft-resistance-factor-to-0-5               |
      | change-pile-dm             | 🍀change-pile-dm             | enlarges-the-pile-diameter-to-0-75-m                    |
      | change-qs-kpa              | 🌾change-qs-kpa              | raises-the-unit-shaft-resistance-to-120-kpa             |
      | change-pile-lm             | 🌵change-pile-lm             | lengthens-the-pile-to-15-m                              |
      | change-qb-kpa              | 🌴change-qb-kpa              | raises-the-unit-base-resistance-to-3200-kpa             |
      | change-pile-base-area-m2   | 🌳change-pile-base-area-m2   | doubles-the-pile-base-area-to-0-5-m2                    |
      | change-pile-n-profiles     | 🌲change-pile-n-profiles     | adds-a-third-investigated-ground-profile                |
      | change-z-investigated-m    | 🍁change-z-investigated-m    | deepens-the-investigated-depth-to-12-m                  |

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
      | id                         | dir                         | fixture                                                 |
      | change-v-ed-kn             | 🪒change-v-ed-kn             | raises-the-design-vertical-load-to-750-kn               |
      | change-h-ed-kn             | 🪥change-h-ed-kn             | raises-the-design-horizontal-load-to-120-kn             |
      | change-footing-area-m2     | 🧴change-footing-area-m2     | enlarges-the-footing-area-to-6-25-m2                    |
      | change-phi-deg             | 🧼change-phi-deg             | raises-the-friction-angle-to-35-degrees                 |
      | change-c-kpa               | 🧽change-c-kpa               | gives-the-drained-sand-12-5-kpa-of-effective-cohesion   |
      | change-gamma-kn-m3         | 🪠change-gamma-kn-m3         | raises-the-soil-unit-weight-to-20-kn-m3                 |
      | change-bm                  | 🧹change-bm                  | widens-the-footing-to-2-5-m                             |
      | change-dfm                 | 🧺change-dfm                 | deepens-the-founding-level-to-2-m                       |
      | change-es-mpa              | 🪑change-es-mpa              | stiffens-the-soil-modulus-to-45-mpa                     |
      | change-nu                  | 🪞change-nu                  | raises-poissons-ratio-to-0-375                          |
      | change-design-approach     | 🛋️change-design-approach    | switches-from-design-approach-1-to-design-approach-2    |
      | change-annex               | 🛏️change-annex              | switches-from-the-german-na-to-the-recommended-en-annex |
      | change-settlement-limit-mm | 🚿change-settlement-limit-mm | relaxes-the-settlement-limit-to-40-mm                   |
      | change-n-pile-ed-kn        | 🛁change-n-pile-ed-kn        | raises-the-design-pile-axial-load-to-1200-kn            |
      | change-alpha-s             | 🌿change-alpha-s             | lowers-the-shaft-resistance-factor-to-0-5               |
      | change-pile-dm             | 🍀change-pile-dm             | enlarges-the-pile-diameter-to-0-75-m                    |
      | change-qs-kpa              | 🌾change-qs-kpa              | raises-the-unit-shaft-resistance-to-120-kpa             |
      | change-pile-lm             | 🌵change-pile-lm             | lengthens-the-pile-to-15-m                              |
      | change-qb-kpa              | 🌴change-qb-kpa              | raises-the-unit-base-resistance-to-3200-kpa             |
      | change-pile-base-area-m2   | 🌳change-pile-base-area-m2   | doubles-the-pile-base-area-to-0-5-m2                    |
      | change-pile-n-profiles     | 🌲change-pile-n-profiles     | adds-a-third-investigated-ground-profile                |
      | change-z-investigated-m    | 🍁change-z-investigated-m    | deepens-the-investigated-depth-to-12-m                  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1997 document from the parsed carrier
    Given the real committed text artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
