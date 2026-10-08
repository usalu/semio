@capability-equation-1-geometry-mutate
@no-oracle-equation-mutation-semantics
@comparison-ordered-json-v1
@mutations-equation-1-geometry
Feature: Apply every typed s.mathematical.equation geometry mutation to its committed specification vector

  `s.mathematical.equation` is a semio-NATIVE artifact and no third party reads `.dsl.semio`.
  That is recorded as the `equation-mutation-semantics` no-oracle decision in
  `../../../✳️any/🔮️oracles/🔣️.json`, which also records why `petgraph` and the external CAS
  candidates were surveyed and DECLINED.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. What blocks a second
  implementation TODAY is stated in the decision: this case's vectors are not declared as `asset://`
  fixtures — the adapter reads the committed files through `include_str!` — so the plan pins none of
  their digests and a Python reference cannot read them at all.

  The snapshot holds the point cloud this vocabulary addresses INLINE (model (a), design §20.15), so the
  vectors are forward evidence wherever their scenario applies: `insert-point` seeds the empty cloud with
  its first point (its inverse, `remove-point` at the same index, is exercised end to end), `move-points`
  drags two points by an offset from their base positions, and `set-point-positions` puts them back — each
  the other's exact undo. `remove-point` carries a REJECTION vector (declared outcome
  `mutation.target-missing`) and none an `applied`-but-`mutation.no-op` vector that restates the
  empty cloud the document already holds; those two are listed by name in the adapter's own
  `mutation_is_observable` call.

  Where the assertions live. This case records a no-oracle decision, so the runner dispatches NO
  oracle role at all. Every law below is asserted INSIDE the adapter's handler, through the shared
  law module `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs` that the stdio subsets use.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> specification vector under 🧬️schema/🧬️mutations
    When <id> is applied to that vector's before-snapshot through equation_mutation_report_json
    Then the applied snapshot, the produced diff and the outcome's diagnostics are exactly what the vector commits, and a kind the vector declares observable really moved the projection
    Examples:
      | id                  |
      | insert-point        |
      | remove-point        |
      | move-points         |
      | set-point-positions |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> specification vector under 🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through equation_mutation_report_json
    Then the snapshot's projection is the before-snapshot's projection again, and any divergence is reported by JSON path
    Examples:
      | id                  |
      | insert-point        |
      | remove-point        |
      | move-points         |
      | set-point-positions |
