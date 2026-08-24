@capability-present-1-mutate
@no-oracle-present-figure-deck-mutation-semantics
@comparison-ordered-json-v1
@mutations-present-1-any
Feature: Apply every typed animate PRESENT mutation to the real committed figure deck
  `s.animate.present` is a semio-NATIVE artifact — the `animate.present.dsl` envelope is defined by this
  repository alone and no package in any ecosystem reads it — so this case carries a recorded no-oracle
  decision (`present-figure-deck-mutation-semantics`, in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) rather than a registered reference
  library. That decision names two substitutes and this feature spends both. ⚠️ Consequence, stated
  plainly: the runner dispatches NO oracle role for a recorded no-oracle case, so every scenario below
  carries its evidence in the SUBJECT role or carries none at all. A handler that merely applied the
  mutation and returned would report a pass having checked nothing, which is why each one asserts its law
  through the shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` module before it returns.

  📄️ The base document is real and committed. `asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` is
  parsed by production's own `parse_dsl`, and the figure SOURCE every scenario starts from is whatever
  that artifact decodes to — never a literal in this case. What the committed artifact cannot supply is
  tiles: `PresentSnapshot` keeps its `(source, tiles)` in a composed `s.stdio.semio.presentation` CHILD
  and the `.present` DSL persists the child HANDLE, not the child, so a case that only parsed the file
  would find an empty tile list and six of the nine kinds would address nothing. The three tiles are
  therefore committed once in `local://🎞️base-tiles.json`, derived from this vocabulary's OWN committed
  per-kind leaf fixtures under `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/`
  — that file records exactly which id and which crop came from which committed payload, and which two
  crops are this case's own derivation.

  🧬️ The vocabulary is `PresentMutation`'s nine variants in declaration order, and it is genuinely this
  subset's own: a singleton `source` facet gets `resize-source-frame`/`replace-source`, while `tiles` — an
  id-keyed ORDERED collection — gets the per-collection recipe `create`/`delete`/`delete-tiles`/`rename`/
  `resize-tile-crop`/`reorder`/`replace-tiles`. There is no `no-mutation` and no `set-snapshot`: whole-
  document replacement is not expressible as an in-history mutation in this generation of the taxonomy and
  goes through `ArtifactStore::reset` instead. Every `params` cell below is the mutation's own
  externally-tagged JSON (`PresentMutation` declares no `#[serde(tag)]`, unlike its `flow`/`shooting`
  siblings) and is chosen to MOVE the projection against that base — an inverse that trivially holds
  because nothing happened is the failure this wave exists to stop.

  ⚖️ The projection is `(schema, source, tiles)` read back through `present_working_scene`. The two child
  handles are deliberately NOT projected: `presentation_child_handle` content-addresses exactly this
  `(source, tiles)` pair with `std`'s deliberately unspecified `DefaultHasher`, so projecting the handle
  would both compare the same content twice and pin a value the standard library does not promise.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the real committed deck and observe it move
    Given the real committed deck artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    And its composed presentation child seeded from local://🎞️base-tiles.json
    When the <id> mutation is applied through apply_present_mutation
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the resulting projection differs from the base projection
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
  @mode-property
  Scenario Outline: Undoing <id> restores the real committed deck exactly
    Given the real committed deck artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    And its composed presentation child seeded from local://🎞️base-tiles.json
    When the <id> mutation is applied through apply_present_mutation
      """
      {"kind": "<id>", "params": <params>}
      """
    And every step of its own computed inverse is applied through apply_present_mutation
    Then the projection equals the base projection again
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
    Given the real committed deck artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When it is parsed with parse_dsl and printed back with print_dsl
    Then the printed bytes are identical to the committed bytes and reparsing preserves the projection
      """
      {"kind": "identity-round-trip", "params": {"carrier": "byte-exact"}}
      """
