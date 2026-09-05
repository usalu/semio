@capability-equation-1-mutate
@no-oracle-equation-mutation-semantics
@comparison-ordered-json-v1
Feature: Parse the real committed equation document, print it back and cross it against its binary encoding
  This case carries the whole-document identity law that used to live inside the artifact-level
  `mutate-equation-1` case alongside the `✳️graph`, `✳️geometry` and `✳️equation` mutation
  Examples. It has no vector and no mutation kind, so unlike its three mutation siblings it claims no
  `@mutations-` catalog — `✳️any` owns no mutation catalog of its own now that the graph, the point
  cloud and the equation each have their smallest owner. It carries the same recorded no-oracle
  decision (`equation-mutation-semantics`, in `../../🔮️oracle/🔣️.json`) because the same debt
  applies here too: this subset's committed snapshot text grammar is the repository-wide placeholder
  `payload = OCTET+`, whose header production declares `"schema" SP "stdio.json"` against an artifact
  whose own first line says otherwise.

  📄️ The real committed document is `asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`, whose
  `equation` field is the default single-term integer expression `0` at label 0 with `nextLabel` 1,
  and whose three composed child handles all carry the same real content key
  `equation-scene-ed395b82221de2b2` — the one committed document where the composition is
  resolved rather than placeheld.

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed equation document, print it back and cross it against its binary encoding
    Given the real committed document asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
