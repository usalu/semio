@capability-en1993-1-mutate
@oracle-en1993-1-python-independent
@comparison-ordered-json-v1
@mutations-en1993-1-any
Feature: Apply every typed EN 1993 mutation against an independent Python implementation
  `s.norm.en1993` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1993` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1993Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is
  it: all 17 kinds of this vocabulary, written in Python from the repository's own written
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
  drift. This subset has a SHAPE no other norm subset has: sixteen of its seventeen kinds are
  `update-<group>-inputs` — `update-bolt-inputs`, `update-weld-inputs`, `update-plated-inputs`,
  `update-silo-shell-inputs`, `update-tower-inputs` and eleven more — each replacing a whole nested
  input RECORD, not a scalar. Only `change-annex` is a flat field. A second implementation therefore
  has to agree on record-level replacement semantics (which sub-fields the payload overwrites and
  which it leaves standing), a question the twelve scalar-only subsets never ask. Each side then
  asserts the same three laws in role — the applied document must BE the committed after-snapshot;
  an `applied` vector must move the document and a `rejected` one must leave it bit-identical; and
  the mutation followed by its OWN computed inverse must restore the before-snapshot exactly. What
  `parity` adds on top is the only thing a single implementation can never provide: that two
  implementations, in two languages, written from one written specification, reach the same
  document.

  `inverse-` projects BOTH the mutated and the restored document. For a record-replacing `update-`
  kind the restored document is the before-document, so only the mutated projection shows whether
  the replacement overwrote the whole record or merged into it — the single most likely place these
  two implementations could differ.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `📚️examples/🔩️high-strength-connection/🖼️assets/🔩️high-strength-connection/🗣️.dsl.semio` — a named
  high-strength bolted-connection case, so the bolt, weld and through-thickness input records are
  populated rather than defaulted. It is an authored case, not a fabricator's submission. The
  carrier has no published grammar: the committed `📖️component.grammar.semio` is the repository-wide
  `payload = OCTET+` placeholder, so the two sides are compared at the envelope preamble, the
  ordered `key=value` fields and the nested input blocks as written, plus the digest and length of
  what each re-emitted — and for this subset the nested blocks are the whole point.

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
      | id                              | dir                               | fixture                                                          |
      | change-annex                    | 🌍️change-annex                    | 🌐️switches-the-national-annex-from-de-to-en                        |
      | update-member-properties        | 📊️update-member-properties        | 🏋️re-grades-the-base-member-to-s460-under-a-heavier-load           |
      | update-fire-inputs              | 🔥️update-fire-inputs              | 🧯️raises-the-fire-protection-to-r90                                |
      | update-cold-formed-inputs       | 🥶️update-cold-formed-inputs       | ↪️thickens-the-cold-formed-flange-and-reverses-its-stress-gradient |
      | update-stainless-inputs         | ✨️update-stainless-inputs         | ✨️upsizes-the-stainless-section-to-a-duplex-grade                  |
      | update-plated-inputs            | 🧱️update-plated-inputs            | 📈️makes-the-plate-panel-more-slender-and-more-stressed             |
      | update-silo-shell-inputs        | 🛢️update-silo-shell-inputs        | 🛢️deepens-the-silo-and-thickens-its-shell                          |
      | update-bolt-inputs              | 🔩️update-bolt-inputs              | 🔩️moves-the-connection-to-four-m24-grade-10-9-bolts                |
      | update-weld-inputs              | 🧲️update-weld-inputs              | 🧲️lengthens-the-fillet-weld-and-re-grades-it-to-s460               |
      | update-fatigue-inputs           | 🔁️update-fatigue-inputs           | 🔁️drops-to-detail-category-56-under-a-safe-life-assessment         |
      | update-through-thickness-inputs | ↕️update-through-thickness-inputs | 🥶️upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c     |
      | update-tension-component-inputs | 🪢️update-tension-component-inputs | 📉️derates-the-tension-rod-to-a-400-kn-characteristic-strength      |
      | update-hss-inputs               | ⬜️update-hss-inputs               | ⬜️reclassifies-the-hollow-section-to-class-3-in-s355               |
      | update-bridge-inputs            | 🌉️update-bridge-inputs            | 🌉️raises-the-bridge-damage-equivalence-and-dynamic-factors         |
      | update-tower-inputs             | 🗼️update-tower-inputs              | 🌬️raises-the-tower-wind-factor-and-leg-force                       |
      | update-pile-inputs              | 🪵️update-pile-inputs              | 🔨️derates-the-driven-pile-for-hard-driving                         |
      | update-crane-inputs             | 🏗️update-crane-inputs             | 🏋️widens-the-crane-wheel-contact-patch-under-a-heavier-wheel       |

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
      | id                              | dir                               | fixture                                                          |
      | change-annex                    | 🌍️change-annex                    | 🌐️switches-the-national-annex-from-de-to-en                        |
      | update-member-properties        | 📊️update-member-properties        | 🏋️re-grades-the-base-member-to-s460-under-a-heavier-load           |
      | update-fire-inputs              | 🔥️update-fire-inputs              | 🧯️raises-the-fire-protection-to-r90                                |
      | update-cold-formed-inputs       | 🥶️update-cold-formed-inputs       | ↪️thickens-the-cold-formed-flange-and-reverses-its-stress-gradient |
      | update-stainless-inputs         | ✨️update-stainless-inputs         | ✨️upsizes-the-stainless-section-to-a-duplex-grade                  |
      | update-plated-inputs            | 🧱️update-plated-inputs            | 📈️makes-the-plate-panel-more-slender-and-more-stressed             |
      | update-silo-shell-inputs        | 🛢️update-silo-shell-inputs        | 🛢️deepens-the-silo-and-thickens-its-shell                          |
      | update-bolt-inputs              | 🔩️update-bolt-inputs              | 🔩️moves-the-connection-to-four-m24-grade-10-9-bolts                |
      | update-weld-inputs              | 🧲️update-weld-inputs              | 🧲️lengthens-the-fillet-weld-and-re-grades-it-to-s460               |
      | update-fatigue-inputs           | 🔁️update-fatigue-inputs           | 🔁️drops-to-detail-category-56-under-a-safe-life-assessment         |
      | update-through-thickness-inputs | ↕️update-through-thickness-inputs | 🥶️upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c     |
      | update-tension-component-inputs | 🪢️update-tension-component-inputs | 📉️derates-the-tension-rod-to-a-400-kn-characteristic-strength      |
      | update-hss-inputs               | ⬜️update-hss-inputs               | ⬜️reclassifies-the-hollow-section-to-class-3-in-s355               |
      | update-bridge-inputs            | 🌉️update-bridge-inputs            | 🌉️raises-the-bridge-damage-equivalence-and-dynamic-factors         |
      | update-tower-inputs             | 🗼️update-tower-inputs              | 🌬️raises-the-tower-wind-factor-and-leg-force                       |
      | update-pile-inputs              | 🪵️update-pile-inputs              | 🔨️derates-the-driven-pile-for-hard-driving                         |
      | update-crane-inputs             | 🏗️update-crane-inputs             | 🏋️widens-the-crane-wheel-contact-patch-under-a-heavier-wheel       |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1993 document from the parsed carrier
    Given the real committed text artifact asset://📚️examples/🔩️high-strength-connection/🖼️assets/🔩️high-strength-connection/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
