@capability-din18599-1-mutate
@oracle-din18599-1-python-independent
@comparison-ordered-json-v1
@mutations-din18599-1-any
Feature: Apply every typed DIN V 18599 mutation against an independent Python implementation
  `s.norm.din18599` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `din18599` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `Din18599Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is
  it: all 13 kinds of this vocabulary, written in Python from the repository's own written
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
  drift. Twelve of the thirteen kinds are scalar edits on the primary-energy balance (`change-ht`,
  `change-hv`, `change-system-losses-kwh`, `change-annual-limit-kwh`); the thirteenth,
  `update-climate`, is the only kind in the subset that does not address a value in the document at
  all — it addresses a HANDLE. Each side then asserts the same three laws in role — the applied
  document must BE the committed after-snapshot; an `applied` vector must move the document and a
  `rejected` one must leave it bit-identical; and the mutation followed by its OWN computed inverse
  must restore the before-snapshot exactly. What `parity` adds on top is the only thing a single
  implementation can never provide: that two implementations, in two languages, written from one
  written specification, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document: for the twelve scalar kinds the
  restored document is the before-document, so projecting only it would make all twelve rows report
  the same value. For `update-climate` the mutated projection is where the replacement handle
  becomes visible.

  ⚠️ `climate` is a composed `s.stdio.semio.table` CHILD slot — the document carries a handle whose
  `childId` is a Rust `DefaultHasher` value over the child's JSON, an identity no second
  implementation can mint. `update-climate`'s committed outcome is `rejected` and both sides refuse
  it, so the two agree here; `⚖️en1990`'s `insert-variable-action` takes the opposite decision on the
  same kind of slot and is red for it. That the two subsets disagree with each other about whether a
  composed slot may be written is worth recording.

  ⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads
  `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, a 430-byte hand-authored residential demo. Note
  what is actually IN those bytes: `climate=[64696e...,64696e...]` is a pair of hex-encoded handles,
  not climate data — the child's own rows never appear in the carrier, so a byte-exact re-emission
  proves the handle was preserved and proves nothing whatever about the referenced table. That is
  this case's real evidence ceiling and it is narrower than the other fourteen norm subsets'. The
  carrier also has no published grammar: the committed `📖️component.grammar.semio` is the
  repository-wide `payload = OCTET+` placeholder, so the two sides are compared on the envelope
  preamble, the ordered `key=value` fields and the digest and length of what each re-emitted, never
  on an inferred token-to-enum mapping.

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
      | id                        | dir                        | fixture                                                  |
      | change-use-class          | 🏷️change-use-class          | 🏢️reclassifies-the-building-as-an-office                   |
      | change-heated-area-m2     | 📐️change-heated-area-m2     | 📏️extends-the-heated-area-to-160-m2                        |
      | change-occupants          | 👥️change-occupants          | 👥️raises-the-occupancy-to-six-people                       |
      | change-ht                 | 🧱️change-ht                 | 🧱️raises-the-transmission-loss-coefficient-to-118-w-per-k  |
      | change-hv                 | 🌬️change-hv                 | 🌬️raises-the-ventilation-loss-coefficient-to-52-25-w-per-k |
      | change-internal-gains-wm2 | 🔥️change-internal-gains-wm2 | 🌡️raises-the-internal-gains-to-5-w-per-m2                  |
      | change-solar-gains-kwh    | ☀️change-solar-gains-kwh    | 🌞️raises-the-annual-solar-gains-to-132-kwh                 |
      | change-system-losses-kwh  | 📉️change-system-losses-kwh  | 🛠️cuts-the-system-losses-to-450-kwh                        |
      | change-renewable-kwh      | ♻️change-renewable-kwh      | 🔆️raises-the-on-site-renewable-yield-to-2250-kwh           |
      | change-annual-limit-kwh   | 🚦️change-annual-limit-kwh   | 🎯️tightens-the-annual-primary-energy-limit-to-6000-kwh     |
      | change-energy-carrier     | 🔋️change-energy-carrier     | ⚡️switches-the-energy-carrier-to-an-electric-heat-pump     |
      | change-reference-qp-kwh   | 🏢️change-reference-qp-kwh   | 📉️lowers-the-reference-building-primary-energy-to-8750-kwh |
      | update-climate            | 🌦️update-climate            | 🌧️refuses-a-negative-january-irradiance                    |

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
      | id                        | dir                        | fixture                                                  |
      | change-use-class          | 🏷️change-use-class          | 🏢️reclassifies-the-building-as-an-office                   |
      | change-heated-area-m2     | 📐️change-heated-area-m2     | 📏️extends-the-heated-area-to-160-m2                        |
      | change-occupants          | 👥️change-occupants          | 👥️raises-the-occupancy-to-six-people                       |
      | change-ht                 | 🧱️change-ht                 | 🧱️raises-the-transmission-loss-coefficient-to-118-w-per-k  |
      | change-hv                 | 🌬️change-hv                 | 🌬️raises-the-ventilation-loss-coefficient-to-52-25-w-per-k |
      | change-internal-gains-wm2 | 🔥️change-internal-gains-wm2 | 🌡️raises-the-internal-gains-to-5-w-per-m2                  |
      | change-solar-gains-kwh    | ☀️change-solar-gains-kwh    | 🌞️raises-the-annual-solar-gains-to-132-kwh                 |
      | change-system-losses-kwh  | 📉️change-system-losses-kwh  | 🛠️cuts-the-system-losses-to-450-kwh                        |
      | change-renewable-kwh      | ♻️change-renewable-kwh      | 🔆️raises-the-on-site-renewable-yield-to-2250-kwh           |
      | change-annual-limit-kwh   | 🚦️change-annual-limit-kwh   | 🎯️tightens-the-annual-primary-energy-limit-to-6000-kwh     |
      | change-energy-carrier     | 🔋️change-energy-carrier     | ⚡️switches-the-energy-carrier-to-an-electric-heat-pump     |
      | change-reference-qp-kwh   | 🏢️change-reference-qp-kwh   | 📉️lowers-the-reference-building-primary-energy-to-8750-kwh |
      | update-climate            | 🌦️update-climate            | 🌧️refuses-a-negative-january-irradiance                    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed DIN V 18599 document from the parsed carrier
    Given the real committed text artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
