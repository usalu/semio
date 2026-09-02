@capability-playground-1-mutate
@oracle-playground-1-python-independent
@comparison-ordered-json-v1
@mutations-playground-1-any
Feature: Apply the playground artifact's whole one-kind mutation vocabulary against an independent Python implementation
  `s.demonstrator.playground` is a semio-NATIVE artifact and no third party reads or writes
  `.dsl.semio`/`.pack.semio`, so no reference LIBRARY exists — the second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is it:
  the one kind of `PlaygroundMutation`, written in Python from this subset's own committed
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and mutation payload schema, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  `change` verb entry. It imports nothing from the Rust it judges and transliterates none of it. The
  no-oracle decision this replaces (`playground-mutation-semantics`) is narrowed to an empty
  `capabilities` list rather than deleted, because its own investigation remains the honest record of
  what was checked.

  Both implementations now read the SAME committed bytes: `(before, mutation, after, diff, outcome)`
  under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️change-schema/🧪️tests/retags-the-playground-document-schema/`
  is a declared `asset://` fixture rather than an `include_str!`-only literal, so the plan pins its
  digest and a Python reference can resolve it.

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

  Where the assertions live. `mutate-change-schema` and `inverse-change-schema` now dispatch BOTH an
  oracle role (the Python implementation, reached through this plugin's `oracleHostPackages` entry)
  and a subject role (this repository's own `apply_playground_mutation_json`/
  `undo_playground_mutation_json`), each independently asserting the forward/inverse laws in role
  before the two are compared byte for byte through the shared `⚖️law` module, which fails with the
  first divergence named by JSON path.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id            | dir             | fixture                                    |
      | change-schema | ✒️change-schema | retags-the-playground-document-schema      |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id            | dir             | fixture                                    |
      | change-schema | ✒️change-schema | retags-the-playground-document-schema      |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse and reprint the real committed example without passing bytes through
    Given the real committed example asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When it is parsed, printed back to DSL and parsed again through round_trip_playground_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
