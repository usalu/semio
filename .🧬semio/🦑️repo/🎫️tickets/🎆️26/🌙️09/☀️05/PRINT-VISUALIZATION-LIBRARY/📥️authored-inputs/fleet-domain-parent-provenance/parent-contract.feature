Feature: Native inferred chart grammar
  Scenario: All canonical marks consume authored scales and row channels
    Given the language-neutral native chart grammar vectors
    When native chart inference emits its TikZ and pinned Tectonic compiles the source
    Then every declared mark emits nonempty geometry
    And authored positions match independent D3 scales
    And ordinal fill, row opacity, stroke width and rotation reach painted geometry
  Scenario: Detail and order define independent paths
    Given two interleaved detail groups with descending source order
    When the native plot is compiled
    Then each ordered group emits its own path
  Scenario: Continuous color scales interpolate named colors
    Given a continuous named-color scale
    When a midpoint is encoded as fill
    Then the painted color equals the independent D3 RGB midpoint
  Scenario: Every coordinate system consumes the same authored encodings
    Given the neutral coordinate frame, scale ranges, options and row
    When all seven coordinate systems are compiled with pinned Tectonic
    Then their geometry matches independent D3 scales, radial shapes and geographic projections

  Scenario: Declared neural architectures preserve directed incidence
    Given sixteen distinct architecture layer tables and explicit directed link records
    When canonical mutations replay their authored mode and link choices and native TikZ is compiled
    Then complete layer inventories, labels and directed endpoints match independent D3 placement
    And recurrence, adversarial branches and graph message incidence remain observable
    And explicit empty links draw no edges
    And invalid link arity or unknown visible units are refused before painting

  Scenario: Exploded assembly defaults distinguish omission and authored zero
    Given symmetric parts with omitted or explicit zero and positive explosion distances
    When bilingual themed inference output is compiled
    Then native bodies and leaders match independent D3 centroids and unit displacements
    And protected empty or known references are consumed
    And unknown engineering or molecule references are refused

  Scenario: Graph storage consumes scalar identities and rejects invalid references
    Given protected strings with preserved interior spaces and numerical or boolean node identities
    When canonical graph inference is compiled through the shared native graph store
    Then the complete declared node and edge inventories survive
    And empty, null, undefined or duplicate identities are refused
    And unknown directed edge endpoints are refused before layout

  Scenario: Actual images and whole text marks obey authored transforms and alpha
    Given an asymmetric transparent raster and neutral rotation area aspect and opacity vectors
    When canonical mutation replay and inference output is compiled in both languages and themes
    Then actual image matrices alpha and intrinsic dimensions match D3 affine and Pillow metadata
    And whole text glyph origins advances and alpha match the same affine controls
    And all eight PDF text rendering modes respect separate fill and stroke alpha

  Scenario: Authored plane vector paint controls reach actual PDF ink
    Given neutral plane vector render, language, opacity, color and physical width vectors
    When each stroke mutation is replayed, inverted and inferred through the canonical chart pipeline
    Then actual native PDF vector color, width and opacity match independent D3 values in both themes

  Scenario: Plane vector fields reject geographic map controls
    Given a language-neutral schema for applicable plane lattice and paint controls
    When geographic projection or map frame fields are added through canonical mutation and replay
    Then inference rejects publication consistently with independent JSON Schema validation

  Scenario: Flow maps accept meaningful flow selectors and reject route-only controls
    Given neutral band, arrow, desire and migration flow selectors
    When a route interpolation mode, samples or stations is authored on a flow map
    Then canonical mutation and inference reject that inapplicable control consistently with JSON Schema
  Scenario: Directed neural tips and recurrent bypasses remain visible
    Given language-neutral unit and block architectures with directed straight, bypass and recurrent incidence
    When canonical inference is compiled in both languages and themes
    Then independent D3 target-boundary intersections match actual clipped shaft endpoints
    And actual directed arrowhead polygons remain outside opaque target fills
    And authored cubic routes preserve their visible directed curve prefixes
    And stock caption glyph advances fit their independent layer slots without overlapping neighbors
    And authored implicit full and adjacent connections share the directed target-boundary guarantees

  Scenario: Authored hive ordering and radii reach native spoke bodies
    Given a schema-backed graph with unequal within-group degrees
    When canonical mutation, replay and inference publish index, name and degree orders
    Then actual PDF node centres match independent D3 sort and uniform-fit scales in both themes

  Scenario: Authored genomic windows reach every accepted branch body
    Given schema-backed genomic tuples and coverage bins
    When canonical mutation and inference publish omitted, paired, single-axis and reversed windows
    Then actual PDF features, synteny, coverage and pileup widths match independent D3 scales in both themes

  Scenario: Authored music windows reach note and envelope bodies
    Given schema-backed MIDI note tuples and sampled curves
    When canonical mutation and inference publish omitted or authored ranges
    Then actual PDF note corners, MIDI droplines and envelope vertices match independent D3 scales

  Scenario: Meteogram panels consume parent height and shared authored windows
    Given schema-backed temperature and rain series
    When canonical mutation and inference publish two heights and omitted, paired, single or reversed windows
    Then actual PDF panel vertices match independent D3 scales and a shared height budget

  Scenario: Canonical domain scalar identities preserve their authored types
    Given thirteen schema-backed literal strings and non-string ID or parent rows
    When canonical mutation, guarded replay, inverse and inference publish the actual scalar carriers
    Then literal scalar names and escaped carrier-looking strings match independent D3 hierarchy depths and sums
    And actual null, missing, boolean and numerical identities or non-string parents produce native Domain diagnostics

  Scenario: Numerical parents cannot alias literal string identities
    Given a root with string ID 7 and a child with numerical parent 7
    When canonical mutation, replay, inverse and inference preserve the two scalar types
    Then independent AJV refuses the non-string parent
    And the actual Domain compiler emits a hierarchy diagnostic before resolving ancestry
