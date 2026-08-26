@capability-iso16757-1-mutate
@oracle-iso16757-1-python-independent
@comparison-ordered-json-v1
@mutations-iso16757-1-any
Feature: Apply every typed ISO 16757 mutation against an independent Python implementation
  `s.norm.iso16757` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `iso16757` distribution, and none for `eurocode`, `vdi3805` or `din18599`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `Iso16757Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is
  it: all 21 kinds of this vocabulary, written in Python from the repository's own written
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
  drift. Unlike the twelve Eurocode subsets, this vocabulary is almost entirely COLLECTION work:
  `create`/`delete`/`rename` triples over `products`, `product-groups`, `property-definitions` and
  `subjects`, plus `add`/`remove-selection-constraint` and `replace-part-number-rule`. Nine of the
  twenty-one kinds mint or destroy an id-keyed entity, so a second implementation has to agree on
  identity and on membership order, not merely on a number — which is why this subset, and not its
  scalar siblings, is where the carrier divergence below actually shows up. Each side then asserts
  the same three laws in role — the applied document must BE the committed after-snapshot; an
  `applied` vector must move the document and a `rejected` one must leave it bit-identical; and the
  mutation followed by its OWN computed inverse must restore the before-snapshot exactly. What
  `parity` adds on top is the only thing a single implementation can never provide: that two
  implementations, in two languages, written from one written specification, reach the same
  document.

  `inverse-` projects BOTH the mutated and the restored document. For the `create`/`delete` pairs
  the mutated projection is where a wrongly-placed re-insertion becomes visible; the restored
  projection alone would report the before-document for every row and say nothing about WHERE the
  entity came back.

  ⚠️ `identity-round-trip` FAILS, and the failure is the finding. This subset's carrier nests records
  and tables and flattens nested records into `key=key=value` runs with no delimiter, and this
  repository publishes no grammar for any of it: the committed
  `🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` is the repository-wide `payload = OCTET+`
  placeholder, and the framework's own DSL notation module carries no grammar document either.
  Reconstructing those bytes would mean inferring a grammar from a single committed example rather
  than reading a specification, so the Python side refuses with that explanation instead of handing
  back the bytes it was given and calling it a round trip. The mutation vocabulary is unaffected — it
  IS specified, and the two implementations agree across every kind of it.

  ⚠️ Honest grading of the evidence. The other twelve norm subsets are pure `change-<field>`/`update`
  vocabularies, and the two documented naming mechanics reproduced every committed vector of them on
  the first run, before any of those vectors had been looked at. This subset is a rule-2 id-keyed
  collection vocabulary, so its containers are located by reading the SNAPSHOT SHAPE — which is what
  `📓️derivation-rules.md` directs an implementer to do — and that resolution was refined against the
  committed vectors over several runs rather than landing first time. It is a weaker kind of evidence
  than the twelve, and it is recorded here rather than levelled up by silence.

  ⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads
  `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, 4,128 bytes of hand-authored demo catalogue —
  the largest committed document in this plugin, and the only one that exercises NESTED record
  blocks (`alternatives [locale:TEXT text:TEXT] { … }` inside `catalogue`, `manufacturer` and each
  product). It is a demo, not a shipped manufacturer catalogue, so it evidences the grammar and the
  nesting, not ISO 16757 conformity. The carrier has no published grammar: the committed
  `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so identity is
  compared at the envelope preamble, the ordered `key=value` fields and the nested blocks as
  written, plus the digest and length of the re-emitted bytes — and the nesting is precisely what
  the recorded divergence above is about.

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
      | id                          | dir                           | fixture                                                    |
      | change-exchange-process     | 🍃change-exchange-process      | advances-the-exchange-stage-to-determine-product           |
      | update-script-limits        | 🌷update-script-limits         | doubles-the-step-budget-and-quintuples-the-timeout         |
      | replace-part-number-rule    | 🍂replace-part-number-rule     | swaps-the-literal-rule-for-a-height-driven-script          |
      | change-part-number-input    | 🌱change-part-number-input     | raises-the-height-part-number-input-to-750                 |
      | remove-part-number-input    | 🌿remove-part-number-input     | drops-the-length-part-number-input                         |
      | change-selection-class      | 🌴change-selection-class       | retargets-the-selection-at-the-towel-radiator-class        |
      | change-selection-series     | 🌼change-selection-series      | narrows-the-selection-to-the-pr-plus-series                |
      | add-selection-constraint    | 🛁add-selection-constraint     | appends-a-width-under-800-constraint                       |
      | remove-selection-constraint | 🛋️remove-selection-constraint | drops-the-trailing-length-constraint                       |
      | rename-catalogue            | 🌲rename-catalogue             | restamps-the-catalogue-as-the-2026-edition                 |
      | rename-manufacturer         | 🌳rename-manufacturer          | adds-the-ag-suffix-to-the-manufacturer                     |
      | create-product-group        | 🍀create-product-group         | appends-a-towel-radiators-group                            |
      | delete-product-group        | 🌹delete-product-group         | removes-the-radiators-group-and-strands-its-class          |
      | rename-product-group        | 🚿rename-product-group         | renames-the-radiators-group-to-panel-radiators             |
      | create-product              | 🍁create-product               | appends-a-pr900-product-to-the-existing-series             |
      | delete-product              | 🌸delete-product               | removes-the-pr600-product-from-the-catalogue               |
      | rename-product              | 🛏️rename-product              | renames-pr600-to-the-compact-variant-name                  |
      | create-property-definition  | 🌾create-property-definition   | appends-a-selection-scoped-length-property                 |
      | delete-property-definition  | 🌺delete-property-definition   | removes-the-height-property-definition                     |
      | create-subject              | 🌵create-subject               | appends-a-towel-radiator-subject-under-the-radiator-parent |
      | delete-subject              | 🌻delete-subject               | removes-the-radiator-subject-from-the-dictionary           |

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
      | id                          | dir                           | fixture                                                    |
      | change-exchange-process     | 🍃change-exchange-process      | advances-the-exchange-stage-to-determine-product           |
      | update-script-limits        | 🌷update-script-limits         | doubles-the-step-budget-and-quintuples-the-timeout         |
      | replace-part-number-rule    | 🍂replace-part-number-rule     | swaps-the-literal-rule-for-a-height-driven-script          |
      | change-part-number-input    | 🌱change-part-number-input     | raises-the-height-part-number-input-to-750                 |
      | remove-part-number-input    | 🌿remove-part-number-input     | drops-the-length-part-number-input                         |
      | change-selection-class      | 🌴change-selection-class       | retargets-the-selection-at-the-towel-radiator-class        |
      | change-selection-series     | 🌼change-selection-series      | narrows-the-selection-to-the-pr-plus-series                |
      | add-selection-constraint    | 🛁add-selection-constraint     | appends-a-width-under-800-constraint                       |
      | remove-selection-constraint | 🛋️remove-selection-constraint | drops-the-trailing-length-constraint                       |
      | rename-catalogue            | 🌲rename-catalogue             | restamps-the-catalogue-as-the-2026-edition                 |
      | rename-manufacturer         | 🌳rename-manufacturer          | adds-the-ag-suffix-to-the-manufacturer                     |
      | create-product-group        | 🍀create-product-group         | appends-a-towel-radiators-group                            |
      | delete-product-group        | 🌹delete-product-group         | removes-the-radiators-group-and-strands-its-class          |
      | rename-product-group        | 🚿rename-product-group         | renames-the-radiators-group-to-panel-radiators             |
      | create-product              | 🍁create-product               | appends-a-pr900-product-to-the-existing-series             |
      | delete-product              | 🌸delete-product               | removes-the-pr600-product-from-the-catalogue               |
      | rename-product              | 🛏️rename-product              | renames-pr600-to-the-compact-variant-name                  |
      | create-property-definition  | 🌾create-property-definition   | appends-a-selection-scoped-length-property                 |
      | delete-property-definition  | 🌺delete-property-definition   | removes-the-height-property-definition                     |
      | create-subject              | 🌵create-subject               | appends-a-towel-radiator-subject-under-the-radiator-parent |
      | delete-subject              | 🌻delete-subject               | removes-the-radiator-subject-from-the-dictionary           |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed ISO 16757 document from the parsed carrier
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then the Rust side reproduces the committed file byte for byte and the Python side refuses, because this carrier's notation is specified nowhere
