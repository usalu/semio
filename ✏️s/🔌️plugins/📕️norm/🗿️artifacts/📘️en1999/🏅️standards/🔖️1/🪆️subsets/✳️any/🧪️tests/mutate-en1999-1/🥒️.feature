@capability-en1999-1-mutate
@oracle-en1999-1-python-independent
@comparison-ordered-json-v1
@mutations-en1999-1-any
Feature: Apply every typed EN 1999 mutation against an independent Python implementation
  `s.norm.en1999` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1999` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1999Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is
  it: all 26 kinds of this vocabulary, written in Python from the repository's own written
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
  drift. All twenty-six kinds are flat `change-<field>` edits, and this subset is the one where the
  SAME quantity appears three times under different qualifiers: `change-m-ed-knm` (member) beside
  `change-sheet-m-ed-knm` (sheeting), `change-w-el-mm3` beside `change-sheet-w-el-mm3`,
  `change-delta-sigma-ed` beside `change-sigma-ed-shell-mpa`, plus a weld group
  (`change-weld-throat-mm`, `change-weld-length-mm`, `change-v-weld-ed-kn`) and a shell group
  (`change-shell-r-mm`, `change-shell-t-mm`). Aluminium's `change-alloy` is the one enum and it is
  what makes the rest of the record mean anything. Each side then asserts the same three laws in
  role — the applied document must BE the committed after-snapshot; an `applied` vector must move
  the document and a `rejected` one must leave it bit-identical; and the mutation followed by its
  OWN computed inverse must restore the before-snapshot exactly. What `parity` adds on top is the
  only thing a single implementation can never provide: that two implementations, in two languages,
  written from one written specification, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the
  restored document repeats the before-document on all twenty-six rows; the mutated projection is
  the only half that tells the member `m-ed-knm` from the sheeting one.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `📚️examples/📕️aluminium-roof-purlin/🖼️assets/🗣️aluminium-roof-purlin.dsl.semio` — a named
  aluminium roof purlin, so `change-alloy`, the section keys and the sheeting group are populated by
  a document that motivates them; the shell and weld groups ride at their committed defaults, which
  is a real limit on what this file's identity evidence covers. The carrier has no published
  grammar: the committed `📖️component.grammar.semio` is the repository-wide `payload = OCTET+`
  placeholder, so identity is compared at the envelope preamble, the ordered `key=value` fields and
  the digest and length of the re-emitted bytes, never at an inferred token-to-enum mapping.

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
      | id                        | dir                        | fixture                                     |
      | change-n-ed-kn            | 🦎change-n-ed-kn            | raises-axial-force-to-180-kn                |
      | change-m-ed-knm           | 🐍change-m-ed-knm           | raises-design-moment-to-9-5-knm             |
      | change-a-mm2              | 🦂change-a-mm2              | enlarges-section-area-to-2250-mm2           |
      | change-w-el-mm3           | 🦟change-w-el-mm3           | raises-section-modulus-to-40000-mm3         |
      | change-alloy              | 🦗change-alloy              | switches-alloy-to-aw7020t6                  |
      | change-chi                | 🕷️change-chi               | lowers-buckling-chi-to-0-5                  |
      | change-it-mm4             | 🐜change-it-mm4             | raises-torsion-constant-to-10240-mm4        |
      | change-l-cr-mm            | 🦔change-l-cr-mm            | lengthens-buckling-length-to-4000-mm        |
      | change-theta-c            | 🦇change-theta-c            | raises-fatigue-detail-theta-c-to-225-mpa    |
      | change-delta-sigma-ed     | 🦉change-delta-sigma-ed     | raises-fatigue-stress-range-to-62-5-mpa     |
      | change-delta-sigma-c      | 🐴change-delta-sigma-c      | upgrades-detail-category-to-90-mpa          |
      | change-fatigue-m          | 🐎change-fatigue-m          | flattens-sn-slope-to-m-5                    |
      | change-n-cycles           | 🦄change-n-cycles           | doubles-fatigue-cycles-to-2000000           |
      | change-v-weld-ed-kn       | 🐑change-v-weld-ed-kn       | raises-weld-shear-to-48-kn                  |
      | change-weld-throat-mm     | 🐐change-weld-throat-mm     | thickens-weld-throat-to-6-5-mm              |
      | change-weld-length-mm     | 🐮change-weld-length-mm     | lengthens-weld-to-200-mm                    |
      | change-beta-w             | 🐷change-beta-w             | raises-weld-correlation-beta-w-to-0-75      |
      | change-sheet-b-mm         | 🐗change-sheet-b-mm         | widens-sheet-to-320-mm                      |
      | change-sheet-t-mm         | 🦌change-sheet-t-mm         | thickens-sheet-to-3-5-mm                    |
      | change-sheet-k-sigma      | 🐘change-sheet-k-sigma      | raises-sheet-plate-buckling-k-sigma-to-6-25 |
      | change-sheet-w-el-mm3     | 🦏change-sheet-w-el-mm3     | raises-sheet-section-modulus-to-12800-mm3   |
      | change-sheet-m-ed-knm     | 🦛change-sheet-m-ed-knm     | raises-sheet-design-moment-to-1-25-knm      |
      | change-shell-t-mm         | 🐪change-shell-t-mm         | thickens-shell-to-6-25-mm                   |
      | change-shell-r-mm         | 🐫change-shell-r-mm         | widens-shell-radius-to-750-mm               |
      | change-sigma-ed-shell-mpa | 🦒change-sigma-ed-shell-mpa | raises-shell-design-stress-to-165-mpa       |
      | change-annex              | 🦘change-annex              | switches-national-annex-to-en               |

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
      | id                        | dir                        | fixture                                     |
      | change-n-ed-kn            | 🦎change-n-ed-kn            | raises-axial-force-to-180-kn                |
      | change-m-ed-knm           | 🐍change-m-ed-knm           | raises-design-moment-to-9-5-knm             |
      | change-a-mm2              | 🦂change-a-mm2              | enlarges-section-area-to-2250-mm2           |
      | change-w-el-mm3           | 🦟change-w-el-mm3           | raises-section-modulus-to-40000-mm3         |
      | change-alloy              | 🦗change-alloy              | switches-alloy-to-aw7020t6                  |
      | change-chi                | 🕷️change-chi               | lowers-buckling-chi-to-0-5                  |
      | change-it-mm4             | 🐜change-it-mm4             | raises-torsion-constant-to-10240-mm4        |
      | change-l-cr-mm            | 🦔change-l-cr-mm            | lengthens-buckling-length-to-4000-mm        |
      | change-theta-c            | 🦇change-theta-c            | raises-fatigue-detail-theta-c-to-225-mpa    |
      | change-delta-sigma-ed     | 🦉change-delta-sigma-ed     | raises-fatigue-stress-range-to-62-5-mpa     |
      | change-delta-sigma-c      | 🐴change-delta-sigma-c      | upgrades-detail-category-to-90-mpa          |
      | change-fatigue-m          | 🐎change-fatigue-m          | flattens-sn-slope-to-m-5                    |
      | change-n-cycles           | 🦄change-n-cycles           | doubles-fatigue-cycles-to-2000000           |
      | change-v-weld-ed-kn       | 🐑change-v-weld-ed-kn       | raises-weld-shear-to-48-kn                  |
      | change-weld-throat-mm     | 🐐change-weld-throat-mm     | thickens-weld-throat-to-6-5-mm              |
      | change-weld-length-mm     | 🐮change-weld-length-mm     | lengthens-weld-to-200-mm                    |
      | change-beta-w             | 🐷change-beta-w             | raises-weld-correlation-beta-w-to-0-75      |
      | change-sheet-b-mm         | 🐗change-sheet-b-mm         | widens-sheet-to-320-mm                      |
      | change-sheet-t-mm         | 🦌change-sheet-t-mm         | thickens-sheet-to-3-5-mm                    |
      | change-sheet-k-sigma      | 🐘change-sheet-k-sigma      | raises-sheet-plate-buckling-k-sigma-to-6-25 |
      | change-sheet-w-el-mm3     | 🦏change-sheet-w-el-mm3     | raises-sheet-section-modulus-to-12800-mm3   |
      | change-sheet-m-ed-knm     | 🦛change-sheet-m-ed-knm     | raises-sheet-design-moment-to-1-25-knm      |
      | change-shell-t-mm         | 🐪change-shell-t-mm         | thickens-shell-to-6-25-mm                   |
      | change-shell-r-mm         | 🐫change-shell-r-mm         | widens-shell-radius-to-750-mm               |
      | change-sigma-ed-shell-mpa | 🦒change-sigma-ed-shell-mpa | raises-shell-design-stress-to-165-mpa       |
      | change-annex              | 🦘change-annex              | switches-national-annex-to-en               |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1999 document from the parsed carrier
    Given the real committed text artifact asset://📚️examples/📕️aluminium-roof-purlin/🖼️assets/🧪️aluminium-roof-purlin/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
