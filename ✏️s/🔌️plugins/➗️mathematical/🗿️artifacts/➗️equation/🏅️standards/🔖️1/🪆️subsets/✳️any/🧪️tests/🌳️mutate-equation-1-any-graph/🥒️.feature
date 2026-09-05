@capability-equation-1-graph-mutate
@no-oracle-equation-mutation-semantics
@comparison-ordered-json-v1
@mutations-equation-1-any-graph
Feature: Apply every typed s.mathematical.equation graph mutation to its committed specification vector
  🧩️ Duplicated verbatim (only relative paths adjusted) from `../../../🕸️graph/🧪️tests/🌳️mutate-equation-1-graph/` by shard F4 (this ticket) to close `unregistered-mutation-vocabulary` at the `✳️any/🧬️schema/🧬️mutations` + `✳️any/🚪️io/🧬️mutations` owner — same mechanism E3 already proved on `sequence`: reuse the already-manifested `equation-1-graph-mutate` capability, no new v2 manifest entry or runtime-inventory coordinate.


  `s.mathematical.equation` is a semio-NATIVE artifact and no third party reads `.dsl.semio`.
  That is recorded as the `equation-mutation-semantics` no-oracle decision in
  `../../🔮️oracle/🔣️.json`, which also records why `petgraph` and the external CAS
  candidates were surveyed and DECLINED.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. What blocks a second
  implementation TODAY is stated in the decision: this case's vectors are not declared as `asset://`
  fixtures — the adapter reads the committed files through `include_str!` — so the plan pins none of
  their digests and a Python reference cannot read them at all.

  What distinguishes this subset is a mismatch between what it declares and what it persists, and
  this case exists partly to make that mismatch visible. The snapshot no longer holds the `graph`
  collection this vocabulary addresses: ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM replaced it with a
  composed `notation` child no fixture can resolve. The consequence is that NONE of this subset's ten
  declared kinds addresses a collection a committed fixture can resolve. Seven of them —
  `create-node`, `delete-node`, `delete-nodes`, `change-node-label`, `move-node`, `connect-nodes`,
  `disconnect-nodes` — carry REJECTION vectors whose declared outcome is `mutation.target-missing` or
  `mutation.duplicate-id` with the offending address in `path`. Three more —
  `change-graph-directed`, `update-graph-algorithm`, `replace-graph` — carry `applied`-but-
  `mutation.no-op` vectors that restate a value the document already holds. Those ten vectors are
  real, handcrafted and worth asserting: a rejection vector pins the exact fault code AND that the
  document was left untouched, which is where the frozen outcome contract's law 2 lives, and this
  case asserts the code and the path verbatim. What they are not is evidence of forward semantics, so
  all ten are listed by name in the adapter's own `mutation_is_observable` call rather than being
  allowed to pass as observed.

  Where the assertions live. This case records a no-oracle decision, so the runner dispatches NO
  oracle role at all. Every law below is asserted INSIDE the adapter's handler, through the shared
  law module `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` that the stdio subsets use —
  `divergence` for a path-named first difference, `mutation_is_observable` for the forward law,
  `inverse_restores` for the inverse law.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> specification vector under 🧬️schema/🧬️mutations
    When <id> is applied to that vector's before-snapshot through equation_mutation_report_json
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

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> specification vector under 🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through equation_mutation_report_json
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
