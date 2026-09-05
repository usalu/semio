@capability-vdi3805-1-mutate
@oracle-vdi3805-1-python-independent
@comparison-ordered-json-v1
@mutations-vdi3805-1-any
Feature: Apply every typed VDI 3805 mutation against an independent Python implementation
  `s.norm.vdi3805` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `vdi3805` distribution, and none for `eurocode`, `iso16757` or `din18599`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `Vdi3805Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is
  it: all 19 kinds of this vocabulary, written in Python from the repository's own written
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
  drift. This vocabulary is split three ways and no sibling subset is:
  `create`/`delete`/`rename-product` and `replace-product-configuration` work an id-keyed catalogue;
  `create`/`delete-geometry`, `resize-geometry`, `replace-geometry-parameters` and
  `add`/`remove-geometry-connection` work a per-product geometry graph; `create`/`delete-curve` and
  `replace-curve-points` work ordered point lists. Only `change-strict-mode`,
  `change-correction-as-of`, `change-edition-profile` and `update-limits` are flat scalars. A second
  implementation therefore has to reproduce three different addressing conventions here, not one.
  Each side then asserts the same three laws in role — the applied document must BE the committed
  after-snapshot; an `applied` vector must move the document and a `rejected` one must leave it
  bit-identical; and the mutation followed by its OWN computed inverse must restore the
  before-snapshot exactly. What `parity` adds on top is the only thing a single implementation can
  never provide: that two implementations, in two languages, written from one written specification,
  reach the same document.

  `inverse-` projects BOTH the mutated and the restored document. For the geometry and curve kinds
  the restored document is the before-document on every row, so the mutated projection is the only
  place a connection re-attached to the wrong endpoint or a point list restored in the wrong order
  can be seen.

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

  ⚠️ The `index` view is DERIVED. Every entry of it is a projection of one `catalog.products` record —
  the article number, the sheet, the title texts and the nominal diameter — so every product-level
  mutation has to rebuild it. That derivation is stated in no document; the Python side discovers it
  by finding the projection that reproduces the COMMITTED index from the COMMITTED products, refuses
  if no such projection exists, and only then applies it to the mutated records. Self-checking
  inference, not a transcription — but inference, and said so.

  ⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads
  `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, 1,961 bytes of hand-authored demo
  (`manufacturer=DEMO record-count=3 building-system-number=system-code="420" …`). Three records is
  enough to exercise the nesting and the derived index, and it is not a real VDI 3805 manufacturer
  file, so nothing here evidences conformity to the published data-exchange format. The carrier has
  no published grammar either: the committed `📖️component.grammar.semio` is the repository-wide
  `payload = OCTET+` placeholder, so the two sides are compared at the envelope preamble, the
  ordered `key=value` fields and the nested blocks as written plus the digest and length of what
  each re-emitted — which is where the recorded divergence above is observed.

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
      | id                            | dir                            | fixture                                                  |
      | update-manufacturer-file      | 🏕️update-manufacturer-file     | renames-the-header-manufacturer-to-acme                  |
      | change-correction-as-of       | 🏜️change-correction-as-of      | advances-the-correction-cut-off-to-2025-03               |
      | change-strict-mode            | 🦋change-strict-mode            | turns-strict-mode-on                                     |
      | update-limits                 | 🦈update-limits                 | tightens-every-untrusted-input-limit                     |
      | change-edition-profile        | 🐝change-edition-profile        | switches-sheet-8-from-legacy-to-current                  |
      | remove-edition-profile        | ⛰️remove-edition-profile       | clears-the-sheet-8-legacy-override                       |
      | create-product                | 🪵create-product                | appends-vlv-80-002-and-its-index-entry                   |
      | delete-product                | 🐳delete-product                | removes-vlv-50-001-and-its-index-entry                   |
      | rename-product                | 🏖️rename-product               | retitles-vlv-50-001-and-resyncs-its-index-tags           |
      | replace-product-configuration | 🗻replace-product-configuration | reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn |
      | create-geometry               | 🦭create-geometry               | adds-the-geom-valve-80-definition                        |
      | delete-geometry               | 🐬delete-geometry               | removes-the-geom-valve-50-definition                     |
      | resize-geometry               | 🏟️resize-geometry              | doubles-the-geom-valve-50-bounding-box                   |
      | add-geometry-connection       | 🐞add-geometry-connection       | attaches-the-drain-connection-to-geom-valve-50           |
      | remove-geometry-connection    | 🏔️remove-geometry-connection   | detaches-the-out-connection-from-geom-valve-50           |
      | replace-geometry-parameters   | 🐌replace-geometry-parameters   | rescales-geom-valve-50-to-half-and-adds-clearance        |
      | create-curve                  | 🏝️create-curve                 | adds-the-curve-dp-pressure-drop-curve                    |
      | delete-curve                  | 🐢delete-curve                  | removes-the-curve-kvs-flow-curve                         |
      | replace-curve-points          | 🏞️replace-curve-points         | resamples-curve-kvs-onto-three-points                    |

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
      | id                            | dir                            | fixture                                                  |
      | update-manufacturer-file      | 🏕️update-manufacturer-file     | renames-the-header-manufacturer-to-acme                  |
      | change-correction-as-of       | 🏜️change-correction-as-of      | advances-the-correction-cut-off-to-2025-03               |
      | change-strict-mode            | 🦋change-strict-mode            | turns-strict-mode-on                                     |
      | update-limits                 | 🦈update-limits                 | tightens-every-untrusted-input-limit                     |
      | change-edition-profile        | 🐝change-edition-profile        | switches-sheet-8-from-legacy-to-current                  |
      | remove-edition-profile        | ⛰️remove-edition-profile       | clears-the-sheet-8-legacy-override                       |
      | create-product                | 🪵create-product                | appends-vlv-80-002-and-its-index-entry                   |
      | delete-product                | 🐳delete-product                | removes-vlv-50-001-and-its-index-entry                   |
      | rename-product                | 🏖️rename-product               | retitles-vlv-50-001-and-resyncs-its-index-tags           |
      | replace-product-configuration | 🗻replace-product-configuration | reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn |
      | create-geometry               | 🦭create-geometry               | adds-the-geom-valve-80-definition                        |
      | delete-geometry               | 🐬delete-geometry               | removes-the-geom-valve-50-definition                     |
      | resize-geometry               | 🏟️resize-geometry              | doubles-the-geom-valve-50-bounding-box                   |
      | add-geometry-connection       | 🐞add-geometry-connection       | attaches-the-drain-connection-to-geom-valve-50           |
      | remove-geometry-connection    | 🏔️remove-geometry-connection   | detaches-the-out-connection-from-geom-valve-50           |
      | replace-geometry-parameters   | 🐌replace-geometry-parameters   | rescales-geom-valve-50-to-half-and-adds-clearance        |
      | create-curve                  | 🏝️create-curve                 | adds-the-curve-dp-pressure-drop-curve                    |
      | delete-curve                  | 🐢delete-curve                  | removes-the-curve-kvs-flow-curve                         |
      | replace-curve-points          | 🏞️replace-curve-points         | resamples-curve-kvs-onto-three-points                    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed VDI 3805 document from the parsed carrier
    Given the real committed text artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then the Rust side reproduces the committed file byte for byte and the Python side refuses, because this carrier's notation is specified nowhere
