@capability-mathematical-1-mutate
@no-oracle-mathematical-mutation-semantics
@comparison-ordered-json-v1
@mutations-mathematical-1-any
Feature: Apply every typed s.mathematical.mathematical mutation to its committed specification vector

  `s.mathematical.mathematical` is a semio-NATIVE artifact and no third party reads `.dsl.semio`.
  That is recorded as the `mathematical-mutation-semantics` no-oracle decision in
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`, which also records why `petgraph` and
  the external CAS candidates were surveyed and DECLINED.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. Declining a third-party
  LIBRARY is a different judgement from declining a SECOND IMPLEMENTATION, and only the first was ever
  made here. `mutate-puzzle-2d-1` and `mutate-puzzle-3d-1` took Python second
  implementations over this same `.dsl.semio` carrier in this wave, so the same is writable for this
  subset from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, the rules of
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

  What distinguishes this subset is a mismatch between what it declares and what it persists, and
  this case exists partly to make that mismatch visible. The snapshot carries three composed child
  handles — `notation` at `s.stdio.semio.text`, `results` at `s.stdio.semio.table`, `computed` at
  `s.stdio.semio.value` — plus ONE plain field, `equation`, a labelled expression tree whose
  `EquationNodeLabel` allocator only ever increases so that an address survives edits a positional
  path would not. The inline `graph` and `geometry` fields this vocabulary was derived from are gone;
  ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM replaced them with the composed children above.

  The consequence is that thirteen of the fifteen declared kinds address collections that no fixture
  can resolve. Nine of them — `create-node`, `delete-node`, `delete-nodes`, `change-node-label`,
  `move-node`, `connect-nodes`, `disconnect-nodes`, `remove-point` and `move-point` — carry REJECTION
  vectors whose declared outcome is `mutation.target-missing` or `mutation.duplicate-id` with the
  offending address in `path`. Four more — `change-graph-directed`, `update-graph-algorithm`,
  `replace-graph` and `replace-points` — carry `applied`-but-`mutation.no-op` vectors that restate a
  value the document already holds. Those thirteen vectors are real, handcrafted and worth asserting:
  a rejection vector pins the exact fault code AND that the document was left untouched, which is
  where the frozen outcome contract's law 2 lives, and this case asserts the code and the path
  verbatim. What they are not is evidence of forward semantics, so all thirteen are listed by name in
  the adapter's own `mutation_is_observable` call rather than being allowed to pass as observed.

  Two kinds carry forward, observable vectors and are the real content of this case today.
  `change-coefficient` raises the leading coefficient of the persisted polynomial to three halves,
  addressing term label 2 through the never-reused label allocator and writing a `Rational` node with
  decimal `numer`/`denom` lexemes rather than an `f64` — so a coefficient edit that went through a
  float loses precision and fails. `insert-point` seeds the empty cloud with its first point, which
  is the one index-addressed verb whose inverse (`remove-point` at the same index) is exercised
  end to end here.

  The identity round trip reads the artifact's own committed demo example, whose `equation` field is
  the default single-term integer expression `0` at label 0 with `nextLabel` 1, and whose three child
  handles all carry the same real content key `mathematical-scene-ed395b82221de2b2` — the one
  committed document where the composition is resolved rather than placeheld.

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
    When <id> is applied to that vector's before-snapshot through mathematical_mutation_report_json
    Then the applied snapshot, the produced diff and the outcome's diagnostics are exactly what the vector commits, and a kind the vector declares observable really moved the projection
    Examples:
      | id                     |
      | change-graph-directed  |
      | update-graph-algorithm |
      | replace-graph          |
      | create-node            |
      | delete-node            |
      | delete-nodes           |
      | change-node-label      |
      | move-node              |
      | connect-nodes          |
      | disconnect-nodes       |
      | replace-points         |
      | insert-point           |
      | remove-point           |
      | move-point             |
      | change-coefficient     |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through mathematical_mutation_report_json
    Then the snapshot's projection is the before-snapshot's projection again, and any divergence is reported by JSON path
    Examples:
      | id                     |
      | change-graph-directed  |
      | update-graph-algorithm |
      | replace-graph          |
      | create-node            |
      | delete-node            |
      | delete-nodes           |
      | change-node-label      |
      | move-node              |
      | connect-nodes          |
      | disconnect-nodes       |
      | replace-points         |
      | insert-point           |
      | remove-point           |
      | move-point             |
      | change-coefficient     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed mathematical document, print it back and cross it against its binary encoding
    Given the real committed document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
