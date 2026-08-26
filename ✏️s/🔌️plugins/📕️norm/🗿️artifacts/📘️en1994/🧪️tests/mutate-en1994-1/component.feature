@capability-en1994-1-mutate
@oracle-en1994-1-python-independent
@comparison-ordered-json-v1
@mutations-en1994-1-any
Feature: Apply every typed EN 1994 mutation against an independent Python implementation
  `s.norm.en1994` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1994` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1994Mutation` vocabulary. The second producer a differential
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
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, because there is now a
  reference to compare against.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)`
  path below is a declared `asset://` fixture, so neither side holds a transcription that could
  drift. All twenty-two kinds are flat `change-<field>` edits over a composite steel-concrete
  section, and the specific reading hazard here is the STUD family: `change-delta-tau-stud-mpa`,
  `change-n-cycles-stud` and `change-v-ed-per-stud-kn` sit beside the member-level
  `change-delta-sigma-mpa`, `change-v-ed-kn` and `change-m-ed-knm` they resemble. Resolving
  `new_v_ed_kn` must land on the member key, not the per-stud one. Each side then asserts the same
  three laws in role — the applied document must BE the committed after-snapshot; an `applied`
  vector must move the document and a `rejected` one must leave it bit-identical; and the mutation
  followed by its OWN computed inverse must restore the before-snapshot exactly. What `parity` adds
  on top is the only thing a single implementation can never provide: that two implementations, in
  two languages, written from one written specification, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the
  restored document repeats the before-document on all twenty-two rows and only the mutated
  projection separates the member-level row from the per-stud row that shadows it.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `📚️examples/📕️composite-bridge-girder/🖼️assets/🗣️composite-bridge-girder.dsl.semio` — a named
  composite bridge-girder case, which is why the fatigue keys (`change-fatigue-detail`,
  `change-n-cycles-stud`, `change-delta-sigma-mpa`) carry real values instead of defaults. It is an
  authored case, not a designed bridge. The carrier has no published grammar: the committed
  `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so identity is
  compared at the envelope preamble, the ordered `key=value` fields and the digest and length of the
  re-emitted bytes, never at an inferred token-to-enum mapping.

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
      | id                             | dir                              | fixture                                        |
      | change-annex                   | 🏞️change-annex                   | switches-national-annex-to-en                  |
      | change-m-ed-knm                | 🗻change-m-ed-knm                 | raises-design-moment-to-320-knm                |
      | change-v-ed-kn                 | 🍄change-v-ed-kn                  | raises-design-shear-to-225-kn                  |
      | change-m-pla                   | 🏝️change-m-pla                   | raises-steel-plastic-moment-to-128-knm         |
      | change-m-pl-rd                 | 🐚change-m-pl-rd                  | raises-plastic-moment-resistance-to-375-knm    |
      | change-eta                     | 🏕️change-eta                     | raises-shear-connection-degree-to-0-875        |
      | change-vl-rd                   | 🐞change-vl-rd                    | raises-longitudinal-shear-resistance-to-240-kn |
      | change-insulation-thickness-mm | 🏖️change-insulation-thickness-mm | thickens-fire-insulation-to-40-mm              |
      | change-fire-rating             | 🏟️change-fire-rating             | upgrades-fire-rating-to-r90                    |
      | change-deck-type               | 🐝change-deck-type                | switches-deck-to-re-entrant                    |
      | change-delta-sigma-mpa         | 🌏️change-delta-sigma-mpa         | raises-steel-stress-range-to-96-mpa            |
      | change-fatigue-detail          | ⛰️change-fatigue-detail          | switches-fatigue-detail-to-flange-butt-weld    |
      | change-d-mm                    | 🌰change-d-mm                     | thickens-stud-shank-to-22-mm                   |
      | change-h-sc-mm                 | 🌐change-h-sc-mm                  | lengthens-stud-to-125-mm                       |
      | change-f-ck-mpa                | 🪵change-f-ck-mpa                 | upgrades-concrete-cylinder-strength-to-40-mpa  |
      | change-fu-mpa                  | 🪨change-fu-mpa                   | upgrades-stud-ultimate-strength-to-500-mpa     |
      | change-e-cm-mpa                | 🌍️change-e-cm-mpa                | raises-concrete-modulus-to-35000-mpa           |
      | change-v-ed-per-stud-kn        | 🏜️change-v-ed-per-stud-kn        | raises-per-stud-shear-to-62-5-kn               |
      | change-span-m                  | 🌊change-span-m                   | lengthens-span-to-12-m                         |
      | change-fy-mpa                  | 🌼change-fy-mpa                   | upgrades-steel-yield-to-460-mpa                |
      | change-n-cycles-stud           | 🏔️change-n-cycles-stud           | raises-stud-cycle-count-to-5000000             |
      | change-delta-tau-stud-mpa      | 🌎️change-delta-tau-stud-mpa      | raises-stud-shear-stress-range-to-110-mpa      |

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
      | id                             | dir                              | fixture                                        |
      | change-annex                   | 🏞️change-annex                   | switches-national-annex-to-en                  |
      | change-m-ed-knm                | 🗻change-m-ed-knm                 | raises-design-moment-to-320-knm                |
      | change-v-ed-kn                 | 🍄change-v-ed-kn                  | raises-design-shear-to-225-kn                  |
      | change-m-pla                   | 🏝️change-m-pla                   | raises-steel-plastic-moment-to-128-knm         |
      | change-m-pl-rd                 | 🐚change-m-pl-rd                  | raises-plastic-moment-resistance-to-375-knm    |
      | change-eta                     | 🏕️change-eta                     | raises-shear-connection-degree-to-0-875        |
      | change-vl-rd                   | 🐞change-vl-rd                    | raises-longitudinal-shear-resistance-to-240-kn |
      | change-insulation-thickness-mm | 🏖️change-insulation-thickness-mm | thickens-fire-insulation-to-40-mm              |
      | change-fire-rating             | 🏟️change-fire-rating             | upgrades-fire-rating-to-r90                    |
      | change-deck-type               | 🐝change-deck-type                | switches-deck-to-re-entrant                    |
      | change-delta-sigma-mpa         | 🌏️change-delta-sigma-mpa         | raises-steel-stress-range-to-96-mpa            |
      | change-fatigue-detail          | ⛰️change-fatigue-detail          | switches-fatigue-detail-to-flange-butt-weld    |
      | change-d-mm                    | 🌰change-d-mm                     | thickens-stud-shank-to-22-mm                   |
      | change-h-sc-mm                 | 🌐change-h-sc-mm                  | lengthens-stud-to-125-mm                       |
      | change-f-ck-mpa                | 🪵change-f-ck-mpa                 | upgrades-concrete-cylinder-strength-to-40-mpa  |
      | change-fu-mpa                  | 🪨change-fu-mpa                   | upgrades-stud-ultimate-strength-to-500-mpa     |
      | change-e-cm-mpa                | 🌍️change-e-cm-mpa                | raises-concrete-modulus-to-35000-mpa           |
      | change-v-ed-per-stud-kn        | 🏜️change-v-ed-per-stud-kn        | raises-per-stud-shear-to-62-5-kn               |
      | change-span-m                  | 🌊change-span-m                   | lengthens-span-to-12-m                         |
      | change-fy-mpa                  | 🌼change-fy-mpa                   | upgrades-steel-yield-to-460-mpa                |
      | change-n-cycles-stud           | 🏔️change-n-cycles-stud           | raises-stud-cycle-count-to-5000000             |
      | change-delta-tau-stud-mpa      | 🌎️change-delta-tau-stud-mpa      | raises-stud-shear-stress-range-to-110-mpa      |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1994 document from the parsed carrier
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️composite-bridge-girder/🖼️assets/🗣️composite-bridge-girder.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
