@capability-presentation-1-mutate
@oracle-presentation-1-python-independent
@comparison-ordered-json-v1
@mutations-presentation-1-any
Feature: Apply every typed animate PRESENTATION mutation to the real committed figure deck and against an independent Python implementation
  `s.animate.presentation` is a semio-NATIVE artifact — the `animate.presentation.dsl` envelope is
  defined by this repository alone and no package in any ecosystem reads it. The second producer a
  differential comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside
  this file is it: written in Python from this subset's own committed
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  `resize`/`replace`/`create`/`delete`/`rename`/`reorder` verb entries and
  `📓️derivation-rules.md`'s per-ordered-collection recipe. It imports nothing from the Rust it judges
  and transliterates none of it. The no-oracle decision this replaces
  (`presentation-figure-deck-mutation-semantics`) is narrowed to an empty `capabilities` list rather
  than deleted, because its own investigation remains the honest record of what was checked.

  ⚠️ Honest boundary. `source` is not decodable from any committed fixture this reference can read —
  only the real `.dsl.semio` example's own parser resolves it — so `resize-source-frame` and
  `replace-source` are modelled with `source` as an OPAQUE marker (touched vs not) rather than real
  content; the seven `tiles`-scoped kinds are verified for real against this case's own committed
  `local://🔣️.json` base graph, already declared as this case's local fixture.

  📄️ The base document is real and committed. `asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` is
  parsed by production's own `parse_dsl`, and the figure SOURCE every scenario starts from is whatever
  that artifact decodes to — never a literal in this case. What the committed artifact cannot supply is
  tiles: `PresentationSnapshot` keeps its `(source, tiles)` in a composed `s.stdio.semio.presentation` CHILD
  and the `.presentation` DSL persists the child HANDLE, not the child, so a case that only parsed the file
  would find an empty tile list and six of the nine kinds would address nothing. The three tiles are
  therefore committed once in `local://🔣️.json`, derived from this vocabulary's OWN committed
  per-kind leaf fixtures under `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/`
  — that file records exactly which id and which crop came from which committed payload, and which two
  crops are this case's own derivation.

  🧬️ The vocabulary is `PresentationMutation`'s nine variants in declaration order, and it is genuinely this
  subset's own: a singleton `source` facet gets `resize-source-frame`/`replace-source`, while `tiles` — an
  id-keyed ORDERED collection — gets the per-collection recipe `create`/`delete`/`delete-tiles`/`rename`/
  `resize-tile-crop`/`reorder`/`replace-tiles`. There is no `no-mutation` and no `set-snapshot`: whole-
  document replacement is not expressible as an in-history mutation in this generation of the taxonomy and
  goes through `ArtifactStore::reset` instead. Every `params` cell below is the mutation's own
  externally-tagged JSON (`PresentationMutation` declares no `#[serde(tag)]`, unlike its `flow`/`shooting`
  siblings) and is chosen to MOVE the projection against that base — an inverse that trivially holds
  because nothing happened is the failure this wave exists to stop.

  ⚖️ The projection is `(schema, source, tiles)` read back through `presentation_working_scene`. The two child
  handles are deliberately NOT projected: `presentation_child_handle` content-addresses exactly this
  `(source, tiles)` pair with `std`'s deliberately unspecified `DefaultHasher`, so projecting the handle
  would both compare the same content twice and pin a value the standard library does not promise.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real committed deck and observe it move
    Given the real committed deck artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    And its composed presentation child seeded from local://🔣️.json
    When the <id> mutation is applied through apply_presentation_mutation, and separately by the Python reference
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the resulting projection differs from the base projection in both implementations
    Examples:
      | id                  | params                                                                                                                                                           |
      | resize-source-frame | {"ResizeSourceFrame":{"newFrame":{"x":0.0,"y":0.0,"width":1.0,"height":1.0}}}                                                                                    |
      | replace-source      | {"ReplaceSource":{"newSource":{"src":"/fixture-deck.png","kind":"figure","frame":{"x":0.0,"y":0.0,"width":1.0,"height":1.0},"sourceAspect":1.5,"pdfPage":null}}} |
      | create-tile         | {"CreateTile":{"index":1,"tile":{"id":"t-macro","name":"Macro","crop":{"x":0.4,"y":0.4,"width":0.2,"height":0.2}}}}                                              |
      | delete-tile         | {"DeleteTile":{"id":"t-hero"}}                                                                                                                                   |
      | delete-tiles        | {"DeleteTiles":{"ids":["t-alpha","t-omega"]}}                                                                                                                    |
      | rename-tile         | {"RenameTile":{"id":"t-hero","newName":"Lead"}}                                                                                                                  |
      | resize-tile-crop    | {"ResizeTileCrop":{"id":"t-hero","newCrop":{"x":0.3,"y":0.3,"width":0.4,"height":0.4}}}                                                                          |
      | reorder-tiles       | {"ReorderTiles":{"id":"t-hero","toIndex":2}}                                                                                                                     |
      | replace-tiles       | {"ReplaceTiles":{"newTiles":[]}}                                                                                                                                 |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real committed deck exactly
    Given the real committed deck artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    And its composed presentation child seeded from local://🔣️.json
    When the <id> mutation is applied through apply_presentation_mutation, and separately by the Python reference
      """
      {"kind": "<id>", "params": <params>}
      """
    And every step of its own computed inverse is applied through apply_presentation_mutation, and separately by the Python reference
    Then the projection equals the base projection again in both implementations
    Examples:
      | id                  | params                                                                                                                                                           |
      | resize-source-frame | {"ResizeSourceFrame":{"newFrame":{"x":0.0,"y":0.0,"width":1.0,"height":1.0}}}                                                                                    |
      | replace-source      | {"ReplaceSource":{"newSource":{"src":"/fixture-deck.png","kind":"figure","frame":{"x":0.0,"y":0.0,"width":1.0,"height":1.0},"sourceAspect":1.5,"pdfPage":null}}} |
      | create-tile         | {"CreateTile":{"index":1,"tile":{"id":"t-macro","name":"Macro","crop":{"x":0.4,"y":0.4,"width":0.2,"height":0.2}}}}                                              |
      | delete-tile         | {"DeleteTile":{"id":"t-hero"}}                                                                                                                                   |
      | delete-tiles        | {"DeleteTiles":{"ids":["t-alpha","t-omega"]}}                                                                                                                    |
      | rename-tile         | {"RenameTile":{"id":"t-hero","newName":"Lead"}}                                                                                                                  |
      | resize-tile-crop    | {"ResizeTileCrop":{"id":"t-hero","newCrop":{"x":0.3,"y":0.3,"width":0.4,"height":0.4}}}                                                                          |
      | reorder-tiles       | {"ReorderTiles":{"id":"t-hero","toIndex":2}}                                                                                                                     |
      | replace-tiles       | {"ReplaceTiles":{"newTiles":[]}}                                                                                                                                 |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real committed deck artifact
    Given the real committed deck artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When it is parsed with parse_dsl and printed back with print_dsl
    Then the printed bytes are identical to the committed bytes and reparsing preserves the projection
      """
      {"kind": "identity-round-trip", "params": {"carrier": "byte-exact"}}
      """
