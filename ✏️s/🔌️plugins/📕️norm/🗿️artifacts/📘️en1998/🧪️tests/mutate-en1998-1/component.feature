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
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is
  it: all 49 kinds of this vocabulary, written in Python from the repository's own written
  specification of what a semantic mutation means — `📓️taxonomy.md`'s verb table, naming mechanics
  ("New-value fields are `new_<field>`") and addressing convention ("Inverse always computed from
  `base`", "Missing target ⇒ `inverse` returns `Vec::new()`"), and `📓️derivation-rules.md`'s shape
  rules — plus this subset's committed catalog for the closed list of kinds. It imports nothing from
  the Rust it judges and transliterates none of it: the document field a `new*` argument names is
  resolved by normalised spelling against the document's own keys, which is what the naming mechanic
  states, never from a table copied out of `🧬️mutations/**` — and the paragraph below names the
  spellings in THIS subset where that resolution can genuinely go wrong. The recorded no-oracle
  decision it replaces is gone from
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, because there is now a
  reference to compare against.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)`
  path below is a declared `asset://` fixture, so neither side holds a transcription that could
  drift. Forty-nine flat `change-<field>` kinds make this the widest Eurocode vocabulary here, and
  they are grouped by STRUCTURE TYPE rather than by quantity: `change-silo-*` (six), `change-tank-*`
  (four), `change-tower-*` (six), `change-wall-*` (five), `change-bearing-*` (two),
  `change-foundation-*` (four) and `change-retrofit-*` (five) all coexist with the bare frame keys.
  Six of those groups carry a `v-rd-kn` or `m-rd-knm` resistance of their own, so an argument
  resolved into the wrong group still names a key of the right shape — the sharpest version of the
  scope hazard in this plugin. Each side then asserts the same three laws in role — the applied
  document must BE the committed after-snapshot; an `applied` vector must move the document and a
  `rejected` one must leave it bit-identical; and the mutation followed by its OWN computed inverse
  must restore the before-snapshot exactly. What `parity` adds on top is the only thing a single
  implementation can never provide: that two implementations, in two languages, written from one
  written specification, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the
  restored document repeats the before-document on all forty-nine rows; the mutated projection is
  the only half that distinguishes `change-silo-v-rd-kn` from `change-bridge-v-rd-kn` from the bare
  `change-v-rd-kn`.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `📚️examples/📕️seismic-rc-frame/🖼️assets/🗣️seismic-rc-frame.dsl.semio` — a named
  reinforced-concrete frame. Be precise about what that means for the forty-nine kinds above: the
  frame case populates the spectrum, ground-type, mass and drift keys, while the silo, tank, tower
  and retrofit groups are carried at their committed defaults, so identity evidence for those groups
  is thinner than the mutate/inverse evidence is. The carrier has no published grammar: the
  committed `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so
  the two sides are compared at the envelope preamble, the ordered `key=value` fields and the digest
  and length of what each re-emitted.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json
    And the committed after-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️component.json
    And the committed outcome asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️component.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id                                | dir                                | fixture                                        |
      | change-seismic-zone               | 🌼change-seismic-zone               | raises-seismic-zone-to-4                       |
      | change-ground-type                | 🍄change-ground-type                | switches-ground-type-to-c                      |
      | change-importance-class           | 🌰change-importance-class           | switches-importance-class-to-cc3               |
      | change-structural-system          | 🌊change-structural-system          | switches-structural-system-to-wall-dcm         |
      | change-t1-s                       | 🐚change-t1-s                       | raises-t1-s-to-0-75                            |
      | change-mass-t                     | 🪨change-mass-t                     | raises-mass-t-to-812-5                         |
      | change-v-rd-kn                    | 🌍️change-v-rd-kn                   | raises-v-rd-kn-to-925-0                        |
      | change-drift-mm                   | 🌎️change-drift-mm                  | raises-drift-mm-to-33-5                        |
      | change-height-m                   | 🌏️change-height-m                  | raises-height-m-to-18-75                       |
      | change-multiple-resisting-systems | 🌐change-multiple-resisting-systems | turns-multiple-resisting-systems-off           |
      | change-annex                      | 🗻change-annex                      | switches-annex-to-en                           |
      | change-en-a-gr                    | 🏔️change-en-a-gr                   | raises-en-a-gr-to-0-25                         |
      | change-en-ground-type             | ⛰️change-en-ground-type            | switches-en-ground-type-to-e                   |
      | change-en-spectrum-type           | 🏕️change-en-spectrum-type          | switches-en-spectrum-type-to-type2             |
      | change-period-ratio               | 🏖️change-period-ratio              | raises-period-ratio-to-3-5                     |
      | change-bridge-v-rd-kn             | 🏜️change-bridge-v-rd-kn            | raises-bridge-v-rd-kn-to-725-0                 |
      | change-bearing-d-ed-mm            | 🏝️change-bearing-d-ed-mm           | raises-bearing-d-ed-mm-to-165-5                |
      | change-bearing-d-rd-mm            | 🏞️change-bearing-d-rd-mm           | raises-bearing-d-rd-mm-to-312-5                |
      | change-retrofit-knowledge-level   | 🏟️change-retrofit-knowledge-level  | switches-retrofit-knowledge-level-to-kl3       |
      | change-retrofit-limit-state       | 🪵change-retrofit-limit-state       | switches-retrofit-limit-state-to-near-collapse |
      | change-retrofit-ed-kn             | 🐝change-retrofit-ed-kn             | raises-retrofit-e-d-kn-to-337-5                |
      | change-retrofit-rk-kn             | 🐞change-retrofit-rk-kn             | raises-retrofit-r-k-kn-to-512-5                |
      | change-retrofit-gamma-el          | 🦋change-retrofit-gamma-el          | raises-retrofit-gamma-el-to-1-25               |
      | change-silo-height-m              | 🐌change-silo-height-m              | raises-silo-height-m-to-14-5                   |
      | change-silo-radius-m              | 🐢change-silo-radius-m              | raises-silo-radius-m-to-6-25                   |
      | change-silo-n-rd-kn               | 🐬change-silo-n-rd-kn               | raises-silo-n-rd-kn-to-640-0                   |
      | change-silo-v-ed-kn               | 🐳change-silo-v-ed-kn               | raises-silo-v-ed-kn-to-225-5                   |
      | change-silo-v-rd-kn               | 🦈change-silo-v-rd-kn               | raises-silo-v-rd-kn-to-412-5                   |
      | change-silo-q-nominal             | 🦭change-silo-q-nominal             | raises-silo-q-nominal-to-2-75                  |
      | change-tank-height-m              | 🐊change-tank-height-m              | raises-tank-height-m-to-11-5                   |
      | change-tank-radius-m              | 🦎change-tank-radius-m              | raises-tank-radius-m-to-5-75                   |
      | change-tank-mass-t                | 🐍change-tank-mass-t                | raises-tank-mass-t-to-425-0                    |
      | change-tank-v-rd-kn               | 🦂change-tank-v-rd-kn               | raises-tank-v-rd-kn-to-537-5                   |
      | change-tower-m-ed-knm             | 🦟change-tower-m-ed-knm             | raises-tower-m-ed-knm-to-1562-5                |
      | change-tower-m-rd-knm             | 🦗change-tower-m-rd-knm             | raises-tower-m-rd-knm-to-2812-5                |
      | change-tower-is-chimney           | 🕷️change-tower-is-chimney          | turns-tower-is-chimney-off                     |
      | change-tower-q-nominal            | 🐜change-tower-q-nominal            | raises-tower-q-nominal-to-3-25                 |
      | change-tower-mass-t               | 🦔change-tower-mass-t               | raises-tower-mass-t-to-112-5                   |
      | change-foundation-area-m2         | 🦇change-foundation-area-m2         | raises-foundation-area-m2-to-144-0             |
      | change-foundation-p-rd-kpa        | 🦉change-foundation-p-rd-kpa        | raises-foundation-p-rd-kpa-to-625-0            |
      | change-foundation-h-ed-kn         | 🐴change-foundation-h-ed-kn         | raises-foundation-h-ed-kn-to-212-5             |
      | change-foundation-h-rd-kn         | 🐎change-foundation-h-rd-kn         | raises-foundation-h-rd-kn-to-475-0             |
      | change-k-foundation               | 🦄change-k-foundation               | raises-k-foundation-to-640000-0                |
      | change-k-soil                     | 🐑change-k-soil                     | raises-k-soil-to-262500-0                      |
      | change-wall-height-m              | 🐐change-wall-height-m              | raises-wall-height-m-to-5-5                    |
      | change-wall-phi-deg               | 🐮change-wall-phi-deg               | raises-wall-phi-deg-to-37-5                    |
      | change-wall-soil-gamma-kn-m3      | 🐷change-wall-soil-gamma-kn-m3      | raises-wall-soil-gamma-kn-m3-to-20-5           |
      | change-wall-r                     | 🐗change-wall-r                     | raises-wall-r-to-2-25                          |
      | change-wall-h-rd-kn               | 🦌change-wall-h-rd-kn               | raises-wall-h-rd-kn-to-187-5                   |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json
    And the committed after-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️component.json
    And the committed outcome asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️component.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id                                | dir                                | fixture                                        |
      | change-seismic-zone               | 🌼change-seismic-zone               | raises-seismic-zone-to-4                       |
      | change-ground-type                | 🍄change-ground-type                | switches-ground-type-to-c                      |
      | change-importance-class           | 🌰change-importance-class           | switches-importance-class-to-cc3               |
      | change-structural-system          | 🌊change-structural-system          | switches-structural-system-to-wall-dcm         |
      | change-t1-s                       | 🐚change-t1-s                       | raises-t1-s-to-0-75                            |
      | change-mass-t                     | 🪨change-mass-t                     | raises-mass-t-to-812-5                         |
      | change-v-rd-kn                    | 🌍️change-v-rd-kn                   | raises-v-rd-kn-to-925-0                        |
      | change-drift-mm                   | 🌎️change-drift-mm                  | raises-drift-mm-to-33-5                        |
      | change-height-m                   | 🌏️change-height-m                  | raises-height-m-to-18-75                       |
      | change-multiple-resisting-systems | 🌐change-multiple-resisting-systems | turns-multiple-resisting-systems-off           |
      | change-annex                      | 🗻change-annex                      | switches-annex-to-en                           |
      | change-en-a-gr                    | 🏔️change-en-a-gr                   | raises-en-a-gr-to-0-25                         |
      | change-en-ground-type             | ⛰️change-en-ground-type            | switches-en-ground-type-to-e                   |
      | change-en-spectrum-type           | 🏕️change-en-spectrum-type          | switches-en-spectrum-type-to-type2             |
      | change-period-ratio               | 🏖️change-period-ratio              | raises-period-ratio-to-3-5                     |
      | change-bridge-v-rd-kn             | 🏜️change-bridge-v-rd-kn            | raises-bridge-v-rd-kn-to-725-0                 |
      | change-bearing-d-ed-mm            | 🏝️change-bearing-d-ed-mm           | raises-bearing-d-ed-mm-to-165-5                |
      | change-bearing-d-rd-mm            | 🏞️change-bearing-d-rd-mm           | raises-bearing-d-rd-mm-to-312-5                |
      | change-retrofit-knowledge-level   | 🏟️change-retrofit-knowledge-level  | switches-retrofit-knowledge-level-to-kl3       |
      | change-retrofit-limit-state       | 🪵change-retrofit-limit-state       | switches-retrofit-limit-state-to-near-collapse |
      | change-retrofit-ed-kn             | 🐝change-retrofit-ed-kn             | raises-retrofit-e-d-kn-to-337-5                |
      | change-retrofit-rk-kn             | 🐞change-retrofit-rk-kn             | raises-retrofit-r-k-kn-to-512-5                |
      | change-retrofit-gamma-el          | 🦋change-retrofit-gamma-el          | raises-retrofit-gamma-el-to-1-25               |
      | change-silo-height-m              | 🐌change-silo-height-m              | raises-silo-height-m-to-14-5                   |
      | change-silo-radius-m              | 🐢change-silo-radius-m              | raises-silo-radius-m-to-6-25                   |
      | change-silo-n-rd-kn               | 🐬change-silo-n-rd-kn               | raises-silo-n-rd-kn-to-640-0                   |
      | change-silo-v-ed-kn               | 🐳change-silo-v-ed-kn               | raises-silo-v-ed-kn-to-225-5                   |
      | change-silo-v-rd-kn               | 🦈change-silo-v-rd-kn               | raises-silo-v-rd-kn-to-412-5                   |
      | change-silo-q-nominal             | 🦭change-silo-q-nominal             | raises-silo-q-nominal-to-2-75                  |
      | change-tank-height-m              | 🐊change-tank-height-m              | raises-tank-height-m-to-11-5                   |
      | change-tank-radius-m              | 🦎change-tank-radius-m              | raises-tank-radius-m-to-5-75                   |
      | change-tank-mass-t                | 🐍change-tank-mass-t                | raises-tank-mass-t-to-425-0                    |
      | change-tank-v-rd-kn               | 🦂change-tank-v-rd-kn               | raises-tank-v-rd-kn-to-537-5                   |
      | change-tower-m-ed-knm             | 🦟change-tower-m-ed-knm             | raises-tower-m-ed-knm-to-1562-5                |
      | change-tower-m-rd-knm             | 🦗change-tower-m-rd-knm             | raises-tower-m-rd-knm-to-2812-5                |
      | change-tower-is-chimney           | 🕷️change-tower-is-chimney          | turns-tower-is-chimney-off                     |
      | change-tower-q-nominal            | 🐜change-tower-q-nominal            | raises-tower-q-nominal-to-3-25                 |
      | change-tower-mass-t               | 🦔change-tower-mass-t               | raises-tower-mass-t-to-112-5                   |
      | change-foundation-area-m2         | 🦇change-foundation-area-m2         | raises-foundation-area-m2-to-144-0             |
      | change-foundation-p-rd-kpa        | 🦉change-foundation-p-rd-kpa        | raises-foundation-p-rd-kpa-to-625-0            |
      | change-foundation-h-ed-kn         | 🐴change-foundation-h-ed-kn         | raises-foundation-h-ed-kn-to-212-5             |
      | change-foundation-h-rd-kn         | 🐎change-foundation-h-rd-kn         | raises-foundation-h-rd-kn-to-475-0             |
      | change-k-foundation               | 🦄change-k-foundation               | raises-k-foundation-to-640000-0                |
      | change-k-soil                     | 🐑change-k-soil                     | raises-k-soil-to-262500-0                      |
      | change-wall-height-m              | 🐐change-wall-height-m              | raises-wall-height-m-to-5-5                    |
      | change-wall-phi-deg               | 🐮change-wall-phi-deg               | raises-wall-phi-deg-to-37-5                    |
      | change-wall-soil-gamma-kn-m3      | 🐷change-wall-soil-gamma-kn-m3      | raises-wall-soil-gamma-kn-m3-to-20-5           |
      | change-wall-r                     | 🐗change-wall-r                     | raises-wall-r-to-2-25                          |
      | change-wall-h-rd-kn               | 🦌change-wall-h-rd-kn               | raises-wall-h-rd-kn-to-187-5                   |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1998 document from the parsed carrier
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️seismic-rc-frame/🖼️assets/🗣️seismic-rc-frame.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
