@capability-en1995-1-mutate
@oracle-en1995-1-python-independent
@comparison-ordered-json-v1
@mutations-en1995-1-any
Feature: Apply every typed EN 1995 mutation against an independent Python implementation
  `s.norm.en1995` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1995` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1995Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is
  it: all 20 kinds of this vocabulary, written in Python from the repository's own written
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
  drift. All twenty kinds are flat `change-<field>` edits, and this subset is the one whose fields
  are mostly SHORT, unpunctuated symbols out of the timber code — `fmk`, `fvk`, `fc0-k`, `chi`-free
  but `w-mm3`, `a-mm2`, `a-ef-mm2`, `h-mm`, `b-mm`. Normalised spelling has less to work with here
  than anywhere else in the plugin (`a-mm2` versus `a-ef-mm2` differ by two characters), which is
  exactly the case a second reading written from the naming mechanic alone has to survive. Each side
  then asserts the same three laws in role — the applied document must BE the committed
  after-snapshot; an `applied` vector must move the document and a `rejected` one must leave it
  bit-identical; and the mutation followed by its OWN computed inverse must restore the
  before-snapshot exactly. What `parity` adds on top is the only thing a single implementation can
  never provide: that two implementations, in two languages, written from one written specification,
  reach the same document.

  `inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the
  restored document repeats the before-document on all twenty rows; the mutated projection is what
  tells `a-mm2` from `a-ef-mm2`.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `📚️examples/📕️glulam-footbridge/🖼️assets/🗣️glulam-footbridge.dsl.semio` — a named glulam
  footbridge, so the vibration and fatigue keys (`change-a-vert-ms2`, `change-n-cycles-bridge`) and
  the `change-service-class`/`change-load-duration` pair are populated by a document that motivates
  them. It is an authored case, not a built bridge. The carrier has no published grammar: the
  committed `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so
  the two sides are compared at the envelope preamble, the ordered `key=value` fields and the digest
  and length of what each re-emitted, never at an inferred token-to-enum mapping.

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
      | id                       | dir                       | fixture                                                 |
      | change-annex             | 📐change-annex             | switches-from-the-german-na-to-the-recommended-en-annex |
      | change-m-ed-knm          | 🧴change-m-ed-knm          | raises-the-design-bending-moment-to-32-knm              |
      | change-n-ed-kn           | 🧽change-n-ed-kn           | raises-the-design-axial-force-to-75-kn                  |
      | change-v-ed-kn           | 🧺change-v-ed-kn           | raises-the-design-shear-force-to-22-5-kn                |
      | change-w-mm3             | 🪑change-w-mm3             | raises-the-section-modulus-to-4000000-mm3               |
      | change-a-mm2             | 🪣change-a-mm2             | enlarges-the-gross-area-to-72000-mm2                    |
      | change-b-mm              | 🧵change-b-mm              | widens-the-beam-to-240-mm                               |
      | change-h-mm              | 🪤change-h-mm              | deepens-the-beam-to-360-mm                              |
      | change-fmk               | 🧷change-fmk               | upgrades-the-bending-strength-class-to-28-mpa           |
      | change-fc0-k             | 🪡change-fc0-k             | raises-the-parallel-compressive-strength-to-26-5-mpa    |
      | change-service-class     | 🧹change-service-class     | moves-the-beam-from-service-class-1-to-service-class-2  |
      | change-load-duration     | 🪒change-load-duration     | shortens-the-load-duration-class-from-medium-to-short   |
      | change-m-crit-knm        | 🪥change-m-crit-knm        | raises-the-critical-buckling-moment-to-96-knm           |
      | change-f-ed-kn           | 🧶change-f-ed-kn           | raises-the-design-fastener-force-to-24-kn               |
      | change-a-ef-mm2          | 🪝change-a-ef-mm2          | enlarges-the-effective-connection-area-to-16000-mm2     |
      | change-fvk               | 🧲change-fvk               | lowers-the-characteristic-shear-strength-to-3-5-mpa     |
      | change-fire-duration-min | 🪢change-fire-duration-min | raises-the-fire-exposure-from-r30-to-r60                |
      | change-section-depth-mm  | 🪠change-section-depth-mm  | raises-the-size-effect-depth-to-360-mm                  |
      | change-a-vert-ms2        | 🧰change-a-vert-ms2        | doubles-the-vertical-footfall-acceleration-to-0-5-m-s2  |
      | change-n-cycles-bridge   | 🧼change-n-cycles-bridge   | quadruples-the-bridge-fatigue-cycles-to-2000000         |

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
      | id                       | dir                       | fixture                                                 |
      | change-annex             | 📐change-annex             | switches-from-the-german-na-to-the-recommended-en-annex |
      | change-m-ed-knm          | 🧴change-m-ed-knm          | raises-the-design-bending-moment-to-32-knm              |
      | change-n-ed-kn           | 🧽change-n-ed-kn           | raises-the-design-axial-force-to-75-kn                  |
      | change-v-ed-kn           | 🧺change-v-ed-kn           | raises-the-design-shear-force-to-22-5-kn                |
      | change-w-mm3             | 🪑change-w-mm3             | raises-the-section-modulus-to-4000000-mm3               |
      | change-a-mm2             | 🪣change-a-mm2             | enlarges-the-gross-area-to-72000-mm2                    |
      | change-b-mm              | 🧵change-b-mm              | widens-the-beam-to-240-mm                               |
      | change-h-mm              | 🪤change-h-mm              | deepens-the-beam-to-360-mm                              |
      | change-fmk               | 🧷change-fmk               | upgrades-the-bending-strength-class-to-28-mpa           |
      | change-fc0-k             | 🪡change-fc0-k             | raises-the-parallel-compressive-strength-to-26-5-mpa    |
      | change-service-class     | 🧹change-service-class     | moves-the-beam-from-service-class-1-to-service-class-2  |
      | change-load-duration     | 🪒change-load-duration     | shortens-the-load-duration-class-from-medium-to-short   |
      | change-m-crit-knm        | 🪥change-m-crit-knm        | raises-the-critical-buckling-moment-to-96-knm           |
      | change-f-ed-kn           | 🧶change-f-ed-kn           | raises-the-design-fastener-force-to-24-kn               |
      | change-a-ef-mm2          | 🪝change-a-ef-mm2          | enlarges-the-effective-connection-area-to-16000-mm2     |
      | change-fvk               | 🧲change-fvk               | lowers-the-characteristic-shear-strength-to-3-5-mpa     |
      | change-fire-duration-min | 🪢change-fire-duration-min | raises-the-fire-exposure-from-r30-to-r60                |
      | change-section-depth-mm  | 🪠change-section-depth-mm  | raises-the-size-effect-depth-to-360-mm                  |
      | change-a-vert-ms2        | 🧰change-a-vert-ms2        | doubles-the-vertical-footfall-acceleration-to-0-5-m-s2  |
      | change-n-cycles-bridge   | 🧼change-n-cycles-bridge   | quadruples-the-bridge-fatigue-cycles-to-2000000         |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1995 document from the parsed carrier
    Given the real committed text artifact asset://📚️examples/📕️glulam-footbridge/🖼️assets/🧪️glulam-footbridge/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
