@capability-en1990-1-mutate
@oracle-en1990-1-python-independent
@comparison-ordered-json-v1
@mutations-en1990-1-any
Feature: Apply every typed EN 1990 mutation against an independent Python implementation
  `s.norm.en1990` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1990` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1990Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is
  it: all 10 kinds of this vocabulary, written in Python from the repository's own written
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
  drift. This is the SMALLEST vocabulary in the plugin — ten kinds — and the only one whose
  collection half is entirely REFUSALS: `change-annex`, `change-permanent-action`,
  `change-resistance`, `change-consequence-class` and `change-seismic-action` apply, while
  `remove`/`change-category`/`change-value`/`reorder-variable-actions` are committed as `rejected`
  against an unseeded child slot and `insert-variable-action` is the recorded failure above. Four of
  the ten rows are therefore evidence that both sides REFUSE the same thing, which is weaker than
  four rows moving a document, and it is stated here rather than counted as if it were the same.
  Each side then asserts the same three laws in role — the applied document must BE the committed
  after-snapshot; an `applied` vector must move the document and a `rejected` one must leave it
  bit-identical; and the mutation followed by its OWN computed inverse must restore the
  before-snapshot exactly. What `parity` adds on top is the only thing a single implementation can
  never provide: that two implementations, in two languages, written from one written specification,
  reach the same document.

  `inverse-` projects BOTH the mutated and the restored document. Four rows are `rejected`, so their
  mutated and restored projections are identical by construction; projecting only the restored
  document would collapse the other six rows onto the same value too and leave the table saying
  nothing at all.

  ⚠️ `insert-variable-action` FAILS, and the failure is the finding. `q_k` is not an inline list but a
  composed `s.stdio.semio.table` CHILD slot: the document carries only a handle, and the handle's
  `childId` is a Rust `DefaultHasher` value over the child's JSON — a hash Rust's own documentation
  declares unspecified and unstable across releases — while the entries themselves live in a
  process-local scratch cache that is not part of the persisted document at all. No second
  implementation, in any language, can mint that identity or read those entries, so this repository's
  answer to `insert-variable-action` is unreproducible by construction. The other four collection
  kinds agree, because their committed outcome is `rejected` and both sides refuse them. The
  divergence is kept rather than hidden: it is a defect in the codec, not in the reference.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `📚️examples/🏢️high-consequence-office/🖼️assets/🏢️high-consequence-office/🗣️.dsl.semio` — a named CC3
  office case, not a generic demo, which is a stronger input than the six norm subsets that read
  `🎬️demo/🗣️example.dsl.semio`. What it still cannot carry is the `q_k` child: the carrier holds the
  handle, so a byte-exact re-emission says nothing about the variable actions themselves. The
  carrier has no published grammar either — the committed `📖️component.grammar.semio` is the
  repository-wide `payload = OCTET+` placeholder — so identity is compared at the envelope preamble,
  the ordered `key=value` fields and the digest and length of the re-emitted bytes, never at an
  inferred mapping from carrier tokens onto the JSON snapshot's enum spellings.

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
      | id                              | dir                              | fixture                                                |
      | change-annex                    | 🌍️change-annex                   | 🌐️switches-the-national-annex-from-de-to-en            |
      | change-permanent-action         | ⚓️change-permanent-action        | ⚓️raises-the-permanent-action-to-62-5-kn               |
      | change-resistance               | 🛡️change-resistance              | 🛡️raises-the-design-resistance-to-320-kn               |
      | change-consequence-class        | ⚠️change-consequence-class       | 🏗️escalates-the-building-from-cc2-to-cc3               |
      | change-seismic-action           | 🌋️change-seismic-action          | 🌋️enables-the-seismic-situation-with-an-85-kn-a-ed     |
      | insert-variable-action          | ➕️insert-variable-action          | ❄️seeds-the-first-variable-action-q-snow-at-12-5-kn      |
      | remove-variable-action          | 🗑️remove-variable-action         | 🚫️refuses-to-remove-action-0-from-an-unseeded-child-slot |
      | change-variable-action-category | 🏷️change-variable-action-category | 🚫️refuses-to-recategorise-a-missing-action-0             |
      | change-variable-action-value    | 🏋️change-variable-action-value    | ⛔️refuses-to-revalue-a-missing-action-0                  |
      | reorder-variable-actions        | 🔀️reorder-variable-actions        | ⛔️refuses-to-move-action-0-to-slot-1-in-an-empty-list    |

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
      | id                              | dir                              | fixture                                                |
      | change-annex                    | 🌍️change-annex                   | 🌐️switches-the-national-annex-from-de-to-en            |
      | change-permanent-action         | ⚓️change-permanent-action        | ⚓️raises-the-permanent-action-to-62-5-kn               |
      | change-resistance               | 🛡️change-resistance              | 🛡️raises-the-design-resistance-to-320-kn               |
      | change-consequence-class        | ⚠️change-consequence-class       | 🏗️escalates-the-building-from-cc2-to-cc3               |
      | change-seismic-action           | 🌋️change-seismic-action          | 🌋️enables-the-seismic-situation-with-an-85-kn-a-ed     |
      | insert-variable-action          | ➕️insert-variable-action          | ❄️seeds-the-first-variable-action-q-snow-at-12-5-kn      |
      | remove-variable-action          | 🗑️remove-variable-action         | 🚫️refuses-to-remove-action-0-from-an-unseeded-child-slot |
      | change-variable-action-category | 🏷️change-variable-action-category | 🚫️refuses-to-recategorise-a-missing-action-0             |
      | change-variable-action-value    | 🏋️change-variable-action-value    | ⛔️refuses-to-revalue-a-missing-action-0                  |
      | reorder-variable-actions        | 🔀️reorder-variable-actions        | ⛔️refuses-to-move-action-0-to-slot-1-in-an-empty-list    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1990 document from the parsed carrier
    Given the real committed text artifact asset://📚️examples/🏢️high-consequence-office/🖼️assets/🏢️high-consequence-office/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
