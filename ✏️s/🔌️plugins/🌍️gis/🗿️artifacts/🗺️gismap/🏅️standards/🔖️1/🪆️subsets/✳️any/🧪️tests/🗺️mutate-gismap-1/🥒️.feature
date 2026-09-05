@capability-gismap-1-mutate
@oracle-gismap-python-independent
@comparison-ordered-json-v1
@mutations-gismap-1-any
Feature: Apply every typed gis.gismap mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory:
  a second implementation of the `s.gis.gismap` document and all twelve typed mutations, written in
  Python from the committed specification — `🧬️schema/📸️snapshot/🔣️.json` for the document,
  `🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` for the twelve verbs and their argument
  lists, and the committed `(before, mutation, after, diff, outcome)` vectors for the wire form of
  each verb. It imports nothing from this repository's Rust and transliterates none of it.

  Why a second implementation rather than a third-party library. A `GisMapFeature` is a stable `id`
  and a `data` object the committed schema declares OPAQUE (`additionalProperties: true`, never
  inspected by the artifact), so `replace-<noun>-data` swaps an untyped value rather than editing a
  typed geometry. `geojson`, `geo` and `gdal` were surveyed by an earlier wave and declined: none of
  them reads `.dsl.semio`, and none is authoritative over a payload the format itself never looks
  inside. What a reference genuinely CAN adjudicate is the collection algebra — insert-at-index,
  delete-by-id, replace-payload-by-id, move-to-index, and the inverse of each — which is what the
  twelve kinds are, four verbs over three parallel id-keyed collections.

  The artifact is real. `identity-round-trip` reads the artifact's own committed demo document — a
  Liège fragment carrying the Institut de Botanique and Lycée Block 3000 with their true WGS84
  coordinates and two named routes with real polylines — and requires both languages to read the
  same document out of the same bytes, with the Python additionally reproducing the file byte for
  byte in role. It is a genuine real-world document but a SMALL one: two positions, two routes, no
  regions. That is the richest `gis.gismap` document committed anywhere in this repository, and
  `asset://` cannot reach outside this artifact's own root, so it is stated here rather than
  papered over.

  Because the committed example carries no regions at all, the twenty-four real-document scenarios
  read local://🗺️liege-with-derived-regions.dsl.semio, DERIVED ONCE from that same committed file by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-gismap-regions.py`:
  nothing is removed or edited, and three regions are added whose every coordinate is the
  axis-aligned envelope of geometry already in the file — route one's polyline, route two's
  polyline, and the two positions together. Without them `delete-region`, `replace-region-data` and
  `reorder-regions` could exercise only the rejection path and their inverse law would be vacuous.

  The committed specification vectors were KEPT, not replaced. `spec-vector-<kind>` replays each
  handcrafted `(before, mutation, after)` triple through both implementations, so the evidence this
  case rested on before the conversion is still here and the real-document scenarios are added on
  top of it.

  WHAT THIS COMPARISON FOUND, and why the case is red. `create-<noun>` declares an insertion `index`
  — the grammar writes `create-position SP number`, the mutation payload carries it and the Rust
  `CreatePosition` struct holds it — but the sparse delta it builds
  (`🧬️schema/🧬️mutations/🆕create-position/🔺️diff/🦀️component.rs`) records only `added: [item]`, and
  `apply_features_delta` (`🧬️schema/🔺️diff/📝️text/🦀️component.rs`) applies additions by `push`. So
  every `create-<noun>` APPENDS and the declared index is silently dropped. The knock-on is the
  inverse law: `delete-<noun>`'s inverse is `create-<noun>` at the captured index, so undoing the
  deletion of any non-trailing feature puts it back in the wrong place.
  All three committed specification vectors miss this because each creates at index 1 in a
  ONE-element collection, where append and insert-at-1 coincide; the real document has two, and the
  divergence is immediate. The failing scenarios are `mutate-create-position`, `mutate-create-route`,
  `mutate-create-region`, their three `inverse-` rows and `inverse-delete-position`. They are left
  RED on purpose: no parameter was softened, no fixture was swapped and no assertion was relaxed to
  make them green. The fix belongs in the diff, which needs to be able to express an insertion
  position at all — a design change to `GisMapFeaturesDelta` that also has to keep the diff-absorb
  law, and therefore not a test's business.

  What the cross-language projection carries. The three `x-semio-state: artifact` collections the
  committed JSON Schema declares, and nothing else. `drawing` and `value` are composed children
  whose `childId` is a `std::hash::DefaultHasher` digest, and the standard library documents that
  hasher's output as unspecified, so no implementation in another language can reproduce it. Those
  two handles are still asserted exactly, in role, by the Rust subject against the committed
  after-snapshot together with the committed `🔺️diff` and `🎯️outcome` — unchanged from before this
  conversion. No comparison profile was touched and no `ignoreKeys` was added.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real Liège document
    Given the real Liège document local://🗺️liege-with-derived-regions.dsl.semio
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same positions, routes and regions
    Examples:
      | id                    | mutation                                                                                                                                                       |
      | create-position       | {"CreatePosition":{"index":1,"item":{"id":"p_val_benoit_campus","data":{"id":"p_val_benoit_campus","label":"Val Benoît Campus","lat":50.6231,"lon":5.5674}}}}    |
      | delete-position       | {"DeletePosition":{"id":"p_institut_de_botanique_ulg_liege"}}                                                                                                   |
      | replace-position-data | {"ReplacePositionData":{"id":"p_lycee_block_3000","newData":{"id":"p_lycee_block_3000","label":"Lycée Block 3000 (surveyed)","lat":50.61025,"lon":5.59015}}}     |
      | reorder-positions     | {"ReorderPositions":{"id":"p_institut_de_botanique_ulg_liege","toIndex":1}}                                                                                     |
      | create-route          | {"CreateRoute":{"index":0,"item":{"id":"bg_link_botanique_lycee:0","data":{"id":"bg_link_botanique_lycee:0","label":"Campus Link","points":[[5.5818,50.603],[5.5901,50.6102]]}}}} |
      | delete-route          | {"DeleteRoute":{"id":"bg_stahl_mehrere_lycee_profiles_canopy:bw_lycee_block_3000:0"}}                                                                           |
      | replace-route-data    | {"ReplaceRouteData":{"id":"bg_holz_fassade_botanique:bw_institut_botanique_ulg:0","newData":{"id":"bg_holz_fassade_botanique:bw_institut_botanique_ulg:0","label":"Holz Fassade (resurveyed)","points":[[5.5818,50.603],[5.5821,50.6032],[5.5825,50.6035]]}}} |
      | reorder-routes        | {"ReorderRoutes":{"id":"bg_stahl_mehrere_lycee_profiles_canopy:bw_lycee_block_3000:0","toIndex":0}}                                                             |
      | create-region         | {"CreateRegion":{"index":1,"item":{"id":"region-liege-quarter","data":{"id":"region-liege-quarter","label":"Liège Quarter","points":[[5.5674,50.603],[5.591,50.603],[5.591,50.6231],[5.5674,50.6231],[5.5674,50.603]]}}}} |
      | delete-region         | {"DeleteRegion":{"id":"region-campus-envelope"}}                                                                                                               |
      | replace-region-data   | {"ReplaceRegionData":{"id":"region-holz-fassade-envelope","newData":{"id":"region-holz-fassade-envelope","label":"Holz Fassade Envelope (buffered)","points":[[5.5817,50.6029],[5.5826,50.6029],[5.5826,50.6036],[5.5817,50.6036],[5.5817,50.6029]]}}} |
      | reorder-regions       | {"ReorderRegions":{"id":"region-campus-envelope","toIndex":1}}                                                                                                 |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real Liège document and land back on it
    Given the real Liège document local://🗺️liege-with-derived-regions.dsl.semio
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      <mutation>
      """
    Then both implementations agree on the mutated document AND on the restored one
    Examples:
      | id                    | mutation                                                                                                                                                       |
      | create-position       | {"CreatePosition":{"index":1,"item":{"id":"p_val_benoit_campus","data":{"id":"p_val_benoit_campus","label":"Val Benoît Campus","lat":50.6231,"lon":5.5674}}}}    |
      | delete-position       | {"DeletePosition":{"id":"p_institut_de_botanique_ulg_liege"}}                                                                                                   |
      | replace-position-data | {"ReplacePositionData":{"id":"p_lycee_block_3000","newData":{"id":"p_lycee_block_3000","label":"Lycée Block 3000 (surveyed)","lat":50.61025,"lon":5.59015}}}     |
      | reorder-positions     | {"ReorderPositions":{"id":"p_institut_de_botanique_ulg_liege","toIndex":1}}                                                                                     |
      | create-route          | {"CreateRoute":{"index":0,"item":{"id":"bg_link_botanique_lycee:0","data":{"id":"bg_link_botanique_lycee:0","label":"Campus Link","points":[[5.5818,50.603],[5.5901,50.6102]]}}}} |
      | delete-route          | {"DeleteRoute":{"id":"bg_stahl_mehrere_lycee_profiles_canopy:bw_lycee_block_3000:0"}}                                                                           |
      | replace-route-data    | {"ReplaceRouteData":{"id":"bg_holz_fassade_botanique:bw_institut_botanique_ulg:0","newData":{"id":"bg_holz_fassade_botanique:bw_institut_botanique_ulg:0","label":"Holz Fassade (resurveyed)","points":[[5.5818,50.603],[5.5821,50.6032],[5.5825,50.6035]]}}} |
      | reorder-routes        | {"ReorderRoutes":{"id":"bg_stahl_mehrere_lycee_profiles_canopy:bw_lycee_block_3000:0","toIndex":0}}                                                             |
      | create-region         | {"CreateRegion":{"index":1,"item":{"id":"region-liege-quarter","data":{"id":"region-liege-quarter","label":"Liège Quarter","points":[[5.5674,50.603],[5.591,50.603],[5.591,50.6231],[5.5674,50.6231],[5.5674,50.603]]}}}} |
      | delete-region         | {"DeleteRegion":{"id":"region-campus-envelope"}}                                                                                                               |
      | replace-region-data   | {"ReplaceRegionData":{"id":"region-holz-fassade-envelope","newData":{"id":"region-holz-fassade-envelope","label":"Holz Fassade Envelope (buffered)","points":[[5.5817,50.6029],[5.5826,50.6029],[5.5826,50.6036],[5.5817,50.6036],[5.5817,50.6029]]}}} |
      | reorder-regions       | {"ReorderRegions":{"id":"region-campus-envelope","toIndex":1}}                                                                                                 |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the committed mutation is applied to the committed before-snapshot
    Then each implementation lands on the committed after-snapshot in role, and the two agree
    Examples:
      | id                    | dir                     | fixture                                    |
      | create-position       | 🆕create-position       | 💡️adds-lighthouse-position-after-harbor      |
      | delete-position       | 🗑️delete-position       | 🚫️removes-lighthouse-position                |
      | replace-position-data | 🔁replace-position-data | ⚓️rewrites-harbor-position-payload           |
      | reorder-positions     | 🔀reorder-positions     | ⚓️moves-harbor-position-to-end               |
      | create-route          | 🛣️create-route         | 🚋️adds-tram-route-after-ferry                |
      | delete-route          | ✂️delete-route          | 🚫️removes-tram-route                         |
      | replace-route-data    | ♻️replace-route-data    | ⛴️rewrites-ferry-route-payload               |
      | reorder-routes        | 🧭reorder-routes        | 🚌️moves-bus-route-to-front                   |
      | create-region         | 🌐create-region         | 🏘️adds-old-town-region-after-harbor-district |
      | delete-region         | 🧹delete-region         | 🚫️removes-old-town-region                    |
      | replace-region-data   | 🔄replace-region-data   | 🏘️rewrites-harbor-district-region-payload    |
      | reorder-regions       | 🔃reorder-regions       | 🌳️moves-park-region-between-2-districts      |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the artifact's own committed Liège document in both languages and agree on it
    Given the real committed document asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation parses it, prints it back through its own carrier and parses it again
    Then both languages read the same positions, routes and regions out of the same real bytes, the Python reproduces the file byte for byte, and the Rust holds its own canonical printing to ArtifactDsl's fixpoint law and cross-checks the pack codec
