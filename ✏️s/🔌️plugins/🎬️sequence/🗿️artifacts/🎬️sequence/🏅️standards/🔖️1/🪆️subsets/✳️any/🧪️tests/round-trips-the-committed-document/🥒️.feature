@capability-sequence-1-mutate
@no-oracle-sequence-step-graph-mutation-semantics
@comparison-ordered-json-v1
Feature: Decode and re-encode the real committed sequence artifact
  This case carries the whole-document identity law that used to live inside the artifact-level
  `mutate-sequence-1` case alongside the `✳️step` and `✳️dependency` mutation Examples. It has no
  vector and no mutation kind, so unlike its two subset siblings it claims no `@mutations-` catalog —
  `✳️any` owns no mutation catalog of its own now that `steps` and dependency-edges each have their
  smallest owner. It carries the same recorded no-oracle decision
  (`sequence-step-graph-mutation-semantics`, in `../../🧪️oracle/🔣️.json`) because the same debt
  applies here too: this subset's committed snapshot text grammar is the repository-wide placeholder
  `payload = OCTET+`, whose header production declares `"schema" SP "stdio.json"` against an artifact
  whose own first line says otherwise, so a second implementation would be refused by clause exactly
  as `mutate-note-1`'s and `mutate-draw-1`'s siblings are.

  📄️ `asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` is this codec's OWN committed output — a
  semio-native envelope no foreign writer ever produced — so reproducing it exactly is the correct
  answer and any drift between the committed artifact and the printer is the defect this scenario
  exists to catch.

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real committed sequence artifact
    Given the real committed sequence artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When it is parsed with parse_dsl and printed back with print_dsl
    Then the printed bytes are identical to the committed bytes and reparsing preserves the projection
      """
      {"kind": "identity-round-trip", "params": {"carrier": "byte-exact"}}
      """
