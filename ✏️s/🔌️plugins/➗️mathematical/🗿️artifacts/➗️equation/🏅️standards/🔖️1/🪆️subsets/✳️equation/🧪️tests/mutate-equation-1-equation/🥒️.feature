@capability-equation-1-equation-mutate
@no-oracle-equation-mutation-semantics
@comparison-ordered-json-v1
@mutations-equation-1-equation
Feature: Apply the s.mathematical.equation equation mutation to its committed specification vector

  `s.mathematical.equation` is a semio-NATIVE artifact and no third party reads `.dsl.semio`.
  That is recorded as the `equation-mutation-semantics` no-oracle decision in
  `../../../✳️any/🧪️oracle/🔣️.json`, which also records why `petgraph` and the external CAS
  candidates were surveyed and DECLINED.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. What blocks a second
  implementation TODAY is stated in the decision: this case's vector is not declared as an `asset://`
  fixture — the adapter reads the committed files through `include_str!` — so the plan pins none of
  its digests and a Python reference cannot read it at all.

  `equation` is the one plain field this artifact still persists inline — a labelled expression tree
  whose `EquationNodeLabel` allocator only ever increases so that an address survives edits a
  positional path would not. `change-coefficient` is this subset's only kind and it carries a real,
  observable vector: it raises the leading coefficient of the persisted polynomial to three halves,
  addressing term label 2 through the never-reused allocator and writing a `Rational` node with
  decimal `numer`/`denom` lexemes rather than an `f64`, so a coefficient edit that went through a
  float loses precision and fails.

  Where the assertions live. This case records a no-oracle decision, so the runner dispatches NO
  oracle role at all. Every law below is asserted INSIDE the adapter's handler, through the shared
  law module `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` that the stdio subsets use.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> specification vector under 🧬️schema/🧬️mutations
    When <id> is applied to that vector's before-snapshot through equation_mutation_report_json
    Then the applied snapshot, the produced diff and the outcome's diagnostics are exactly what the vector commits, and the kind really moved the projection
    Examples:
      | id                 |
      | change-coefficient |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> specification vector under 🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through equation_mutation_report_json
    Then the snapshot's projection is the before-snapshot's projection again, and any divergence is reported by JSON path
    Examples:
      | id                 |
      | change-coefficient |
