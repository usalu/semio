@capability-process3d-1-mutate
@no-oracle-process3d-mutation-semantics
@comparison-ordered-json-v1
@mutations-process3d-1-any
Feature: Apply every typed process.process3d mutation to its committed specification vector

  `process.process3d` is a semio-NATIVE artifact and nothing outside this repository reads
  `.dsl.semio`. That is recorded as the `process3d-mutation-semantics` no-oracle decision in
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, which also records why G-code parsers
  and STEP/BREP kernels were surveyed and DECLINED.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. Declining a third-party
  LIBRARY is a different judgement from declining a SECOND IMPLEMENTATION, and only the first was ever
  made here. `mutate-assembly-1` and `mutate-cad-1` took Python second
  implementations over this same `.dsl.semio` carrier in this wave, so the same is writable for this
  subset from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`, the rules of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` and the
  committed vectors this feature already replays. What blocks it TODAY is stated in the decision and
  is one edit: this case's vectors are not declared as `asset://` fixtures — the `Examples` table
  carries the payloads inline and the adapter reads the committed files through `include_str!` — so
  the plan pins none of their digests and a Python reference cannot read them at all. Separately, `identity-round-trip` would still be refused: this subset's committed
  snapshot text grammar is the repository-wide placeholder `payload = OCTET+`, whose header production
  declares `"schema" SP "stdio.json"` against an artifact whose own first line says otherwise.
  Until that is done, every assertion below still lives in the SUBJECT role, and the ceiling is the
  one this decision has always had: no second producer runs beside it, so a mistake shared by the
  handcrafted vector and the production code passes unseen.

  What distinguishes this subset is that one document carries three different collection shapes at
  once, and the vocabulary is derived from that difference rather than applied uniformly. `steps` is
  an id-keyed, ORDER-MEANINGFUL timeline, so it gets `create`, `delete`, `rename`, two field-scoped
  changes (`change-step-enabled`, `change-step-origin`), a large-payload `replace-step-measure`, and
  a `reorder-steps`. `workshop.machines` is an id-keyed but UNORDERED set, so it gets `create`,
  `delete`, `rename`, `change-machine-icon` and `replace-machine-capabilities` — and no reorder,
  because position carries no meaning there. `stock` is a single facet split three ways by the size
  and kind of what changes: `move-stock` for the pose, `change-stock-label` for the identity string,
  and `replace-stock-solid` for the large structured BREP child handle. `change-cursor` is the one
  document-level scalar, the replay position the viewer resolves up to.

  A gap, named kind by kind rather than averaged away. Seven of the sixteen kinds — `create-step`,
  `delete-step`, `rename-step`, `change-step-enabled`, `change-step-origin`, `replace-step-measure`
  and `reorder-steps`, that is every step-scoped verb — are implemented TODAY as documented no-ops.
  Their diff builders take `_payload` and `_base` by underscore and return
  `MutationOutcome::empty().warn("mutation.no-op", ...)`, because the timeline is a composed
  `s.stdio.semio.flow` child and no link resolver exists yet. Their committed vectors record exactly
  that: a fully formed step payload, a before-snapshot equal to the after-snapshot, an empty diff and
  one `mutation.no-op` message. This case asserts that declared behaviour precisely — the empty diff,
  the warning and the untouched document — which is a real check on the documented degradation, and
  it does NOT assert observability for those seven, which is why they are listed by name in the
  adapter's own `mutation_is_observable` call. Their forward semantics are uncovered until the flow
  link resolver lands, and no fixture was invented to hide that.

  The other nine are genuine. `replace-machine-capabilities` trades a blade cut for a GATED pocket
  cut, so a capability list that dropped the gate rule fails. `move-stock` lifts AND tilts, so a pose
  codec that carried position and forgot the axis-angle fails. `replace-stock-solid` reissues the
  stock BREP child handle, which is the one place a content-addressed digest has to move.

  The identity round trip reads the artifact's own committed demo example, a 12 KB document whose
  workshop is hex-encoded JSON carrying a bench saw, a drill, an attacher, a circular saw with a
  max-cut-depth rule and a table — the largest committed document in this plugin and the only one
  that exercises the capability-rule vocabulary at all.

  Where the assertions live. This case records a no-oracle decision, so the runner dispatches NO
  oracle role at all: `oracleDecision` resolves an oracle implementation from an `@oracle-` tag, this
  feature has none, and the comparison profile therefore never receives two sides to compare. Every
  law below is asserted INSIDE the adapter's handler, through the shared law module
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` that the stdio subsets use — `divergence` for
  a path-named first difference, `mutation_is_observable` for the forward law, `inverse_restores` for
  the inverse law, `round_trip_preserves` and `carrier_is_exact` for the identity law. A handler that
  applied the mutation and returned would report a pass having checked nothing, which is exactly the
  failure this platform exists to prevent.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied to that vector's before-snapshot through process3d_mutation_report_json
    Then the applied snapshot, the produced diff and the outcome's diagnostics are exactly what the vector commits, and a kind the vector declares observable really moved the projection
    Examples:
      | id                           |
      | create-step                  |
      | delete-step                  |
      | rename-step                  |
      | change-step-enabled          |
      | change-step-origin           |
      | replace-step-measure         |
      | reorder-steps                |
      | create-machine               |
      | delete-machine               |
      | rename-machine               |
      | change-machine-icon          |
      | replace-machine-capabilities |
      | move-stock                   |
      | change-stock-label           |
      | replace-stock-solid          |
      | change-cursor                |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through process3d_mutation_report_json
    Then the snapshot's projection is the before-snapshot's projection again, and any divergence is reported by JSON path
    Examples:
      | id                           |
      | create-step                  |
      | delete-step                  |
      | rename-step                  |
      | change-step-enabled          |
      | change-step-origin           |
      | replace-step-measure         |
      | reorder-steps                |
      | create-machine               |
      | delete-machine               |
      | rename-machine               |
      | change-machine-icon          |
      | replace-machine-capabilities |
      | move-stock                   |
      | change-stock-label           |
      | replace-stock-solid          |
      | change-cursor                |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed fabrication document, print it back and cross it against its binary encoding
    Given the real committed document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
