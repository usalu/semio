@capability-en1996-1-mutate
@oracle-en1996-1-python-independent
@comparison-ordered-json-v1
@mutations-en1996-1-any
Feature: Apply every typed EN 1996 mutation against an independent Python implementation
  `s.norm.en1996` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1996` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1996Mutation` vocabulary. The second producer a differential
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
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`, because there is now a
  reference to compare against.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)`
  path below is a declared `asset://` fixture, so neither side holds a transcription that could
  drift. All twenty-two kinds are flat `change-<field>` edits, and half of this subset's fields are
  ENUM-valued rather than numeric — `change-unit`, `change-mortar`, `change-masonry-class`,
  `change-exposure`, `change-design-situation`, `change-annex`. That matters because an independent
  implementation resolves the FIELD by normalised spelling but must reproduce the VALUE's spelling
  exactly, and an enum is the one place where a plausible-looking near-miss survives a numeric
  comparison. Each side then asserts the same three laws in role — the applied document must BE the
  committed after-snapshot; an `applied` vector must move the document and a `rejected` one must
  leave it bit-identical; and the mutation followed by its OWN computed inverse must restore the
  before-snapshot exactly. What `parity` adds on top is the only thing a single implementation can
  never provide: that two implementations, in two languages, written from one written specification,
  reach the same document.

  `inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the
  restored document repeats the before-document on all twenty-two rows; for the six enum-valued
  kinds the mutated projection is the only place the written spelling of the new value is
  observable.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `📚️examples/📕️loadbearing-wall/🖼️assets/🗣️loadbearing-wall.dsl.semio` — a named load-bearing
  masonry wall, so the enum fields above carry real spellings rather than defaults. It is an
  authored case, not a surveyed wall. The carrier has no published grammar: the committed
  `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so identity is
  compared at the envelope preamble, the ordered `key=value` fields and the digest and length of the
  re-emitted bytes — and for an enum-heavy subset that byte-level comparison is doing more work than
  it does elsewhere in this plugin.

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
      | id                            | dir                            | fixture                                                   |
      | change-m-ed-knm               | 📐change-m-ed-knm               | raises-the-design-bending-moment-to-12-5-knm              |
      | change-n-ed-kn                | 🔽change-n-ed-kn                | raises-the-design-axial-force-to-320-kn                   |
      | change-v-ed-kn                | 🔼change-v-ed-kn                | raises-the-design-shear-force-to-48-kn                    |
      | change-h-ed-kn                | ↔️change-h-ed-kn               | raises-the-design-sliding-force-to-26-kn                  |
      | change-z-mm3                  | ➡️change-z-mm3                 | raises-the-section-modulus-to-9500000-mm3                 |
      | change-area-mm2               | ⬅️change-area-mm2              | enlarges-the-gross-area-to-640000-mm2                     |
      | change-shear-area-mm2         | 📏change-shear-area-mm2         | enlarges-the-shear-area-to-384000-mm2                     |
      | change-fk-mpa                 | 🟩change-fk-mpa                 | raises-the-characteristic-compressive-strength-to-7-5-mpa |
      | change-f-vk-mpa               | ✂️change-f-vk-mpa              | raises-the-characteristic-shear-strength-to-0-375-mpa     |
      | change-annex                  | 🔨change-annex                  | switches-from-the-german-na-to-the-recommended-en-annex   |
      | change-masonry-class          | 🗺️change-masonry-class         | downgrades-manufacturing-control-to-class-4               |
      | change-design-situation       | 🧱change-design-situation       | switches-the-design-situation-to-seismic                  |
      | change-mu                     | 🏗️change-mu                    | raises-the-bed-joint-friction-coefficient-to-0-625        |
      | change-wall-thickness-mm      | 🎢change-wall-thickness-mm      | thickens-the-wall-to-300-mm                               |
      | change-fire-resistance-min    | 🧊change-fire-resistance-min    | raises-the-fire-resistance-requirement-from-r60-to-r90    |
      | change-unit                   | 🌡️change-unit                  | switches-the-masonry-unit-from-clay-to-calcium-silicate   |
      | change-exposure               | 💧change-exposure               | moves-the-wall-to-exposure-class-mx3                      |
      | change-mortar                 | 🌬️change-mortar                | upgrades-the-general-purpose-mortar-to-m10                |
      | change-bed-joint-thickness-mm | 🔥change-bed-joint-thickness-mm | thickens-the-bed-joint-to-the-15-mm-upper-limit           |
      | change-storeys                | ❄️change-storeys               | adds-a-third-storey-at-the-simplified-method-limit        |
      | change-h-ef-mm                | ⚡change-h-ef-mm                | lengthens-the-effective-height-to-2750-mm                 |
      | change-t-ef-mm                | 🔆change-t-ef-mm                | raises-the-effective-thickness-to-300-mm                  |

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
      | id                            | dir                            | fixture                                                   |
      | change-m-ed-knm               | 📐change-m-ed-knm               | raises-the-design-bending-moment-to-12-5-knm              |
      | change-n-ed-kn                | 🔽change-n-ed-kn                | raises-the-design-axial-force-to-320-kn                   |
      | change-v-ed-kn                | 🔼change-v-ed-kn                | raises-the-design-shear-force-to-48-kn                    |
      | change-h-ed-kn                | ↔️change-h-ed-kn               | raises-the-design-sliding-force-to-26-kn                  |
      | change-z-mm3                  | ➡️change-z-mm3                 | raises-the-section-modulus-to-9500000-mm3                 |
      | change-area-mm2               | ⬅️change-area-mm2              | enlarges-the-gross-area-to-640000-mm2                     |
      | change-shear-area-mm2         | 📏change-shear-area-mm2         | enlarges-the-shear-area-to-384000-mm2                     |
      | change-fk-mpa                 | 🟩change-fk-mpa                 | raises-the-characteristic-compressive-strength-to-7-5-mpa |
      | change-f-vk-mpa               | ✂️change-f-vk-mpa              | raises-the-characteristic-shear-strength-to-0-375-mpa     |
      | change-annex                  | 🔨change-annex                  | switches-from-the-german-na-to-the-recommended-en-annex   |
      | change-masonry-class          | 🗺️change-masonry-class         | downgrades-manufacturing-control-to-class-4               |
      | change-design-situation       | 🧱change-design-situation       | switches-the-design-situation-to-seismic                  |
      | change-mu                     | 🏗️change-mu                    | raises-the-bed-joint-friction-coefficient-to-0-625        |
      | change-wall-thickness-mm      | 🎢change-wall-thickness-mm      | thickens-the-wall-to-300-mm                               |
      | change-fire-resistance-min    | 🧊change-fire-resistance-min    | raises-the-fire-resistance-requirement-from-r60-to-r90    |
      | change-unit                   | 🌡️change-unit                  | switches-the-masonry-unit-from-clay-to-calcium-silicate   |
      | change-exposure               | 💧change-exposure               | moves-the-wall-to-exposure-class-mx3                      |
      | change-mortar                 | 🌬️change-mortar                | upgrades-the-general-purpose-mortar-to-m10                |
      | change-bed-joint-thickness-mm | 🔥change-bed-joint-thickness-mm | thickens-the-bed-joint-to-the-15-mm-upper-limit           |
      | change-storeys                | ❄️change-storeys               | adds-a-third-storey-at-the-simplified-method-limit        |
      | change-h-ef-mm                | ⚡change-h-ef-mm                | lengthens-the-effective-height-to-2750-mm                 |
      | change-t-ef-mm                | 🔆change-t-ef-mm                | raises-the-effective-thickness-to-300-mm                  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1996 document from the parsed carrier
    Given the real committed text artifact asset://📚️examples/📕️loadbearing-wall/🖼️assets/🧪️loadbearing-wall/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
