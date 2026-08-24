@capability-playground-1-mutate
@no-oracle-playground-mutation-semantics
@comparison-ordered-json-v1
@mutations-playground-1-any
Feature: Apply the playground artifact's whole one-kind mutation vocabulary
  `s.demonstrator.playground` is a semio-NATIVE artifact and no third party reads or
  writes `.dsl.semio`/`.pack.semio`, so there is no reference implementation to register as an oracle.
  That is recorded as the `playground-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`.

  This vocabulary has exactly ONE kind, and that is a property of the artifact rather than a gap in
  the test: `PlaygroundSnapshot` carries a single persistent field, so `change-schema` retagging it is
  the whole of what this artifact can do to itself. What genuinely distinguishes the subset is its
  WIRE SHAPE. `PlaygroundMutation` carries no `#[serde(tag = ..)]` and its payload struct carries no
  `rename_all`, so alone among the artifacts in this repository it encodes EXTERNALLY tagged with a
  snake_case field — `{"ChangeSchema": {"new_schema": …}}` — where every sibling encodes internally
  tagged and camelCase. The committed vector under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️change-schema/🧪️tests/retags-the-playground-document-schema/`
  is the pin on exactly that, and this case re-reads those same bytes end-to-end through
  `apply_playground_mutation_json`/`undo_playground_mutation_json`, so a serde attribute added to that
  enum in passing breaks a scenario rather than silently changing the wire format.

  The kind's diff oracle is root-scoped — there is no target that could be missing, only an equality
  guard that downgrades an unchanged retag to a `mutation.no-op` warning — so `change-schema` has no
  missing-target error path to exercise, and the committed vector moves the tag from
  `playground.playground` to `playground.experiment` precisely so the guard does not fire.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion below lives in the subject handler, which compares against the committed after-document
  through the shared `⚖️law` module and fails with the first divergence named by JSON path.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Applying <id> reaches its committed after-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> is applied through apply_playground_mutation_json
    Then the resulting document is the committed after-document, and the mutation moved it
    Examples:
      | id            |
      | change-schema |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> and then every step of its own computed inverse are applied through undo_playground_mutation_json
    Then the document is the committed before-document again, member positions included
    Examples:
      | id            |
      | change-schema |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse and reprint the real committed example without passing bytes through
    Given the real committed example asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When it is parsed, printed back to DSL and parsed again through round_trip_playground_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
