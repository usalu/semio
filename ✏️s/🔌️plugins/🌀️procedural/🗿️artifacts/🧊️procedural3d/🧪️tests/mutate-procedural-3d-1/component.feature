@capability-procedural-3d-1-mutate
@no-oracle-procedural-3d-mutation-semantics
@comparison-ordered-json-v1
@mutations-procedural-3d-1-any
Feature: Replay every typed Procedural3d 1 mutation against its committed specification vector
  `s.procedural.procedural3d@1/*` is a semio-NATIVE document, carried as `.dsl.semio`/`.pack.semio`. No
  third party reads those, and none is authoritative over `Procedural3dMutation`, so this case rests on the
  recorded `procedural-3d-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) and its two named substitutes: the
  committed specification vectors, and the metamorphic laws below.

  What distinguishes this subset is that its snapshot has only two top-level fields and they belong to
  different worlds: `fixture` is the widget/synapse graph with its camera and per-widget layout, and
  `generation` is a parameter-set history with a selected id. Every one of the fourteen kinds therefore
  lands in one of exactly two footprints, which is why the observability law does the real work here and
  the footprint law only distinguishes graph edits from history edits. The layout is a per-widget MAP, so
  `move-widget` and `delete-widget-position` address a widget that may not be there and must leave the
  widget collection itself untouched.

  Its 2d sibling is NOT the same vocabulary and this case is not a copy of that one: where this subset has
  `update-widget`/`update-synapse` and a per-widget `delete-widget-position`, `🌀️procedural2d` has
  whole-value `replace-widget`/`replace-synapse` and a document-wide `clear-widget-layout`. The two catalogs
  differ in three of fourteen kinds and each case measures its own.

  Every scenario replays one committed `(before, mutation, diff, outcome, after)` quintet — the same
  bytes the production crate's own fixture tests beside each leaf assert against — end to end through
  the test platform. The vector each row names is written out in full in the row itself, so the
  provenance of every input is readable here and pinned by digest at plan time.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: The committed <id> vector declares its own kind and moves the document
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json",
        "mutation": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json",
        "diff": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🔺️diff/🔣️component.json",
        "outcome": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️component.json",
        "after": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️component.json"
      }
      """
    Then the committed mutation payload declares the <id> kind
    And the after-snapshot differs from the before-snapshot, or the committed outcome declares the vector a no-op
    Examples:
      | id                      | vector                                                                      |
      | create-widget           | 🌱create-widget/🧪️tests/inserts-node-c-at-index-2                            |
      | update-widget           | 🩹update-widget/🧪️tests/retunes-the-knob-slider-value                        |
      | delete-widget           | ❌delete-widget/🧪️tests/removes-node-a-and-leaves-wire-ab-dangling           |
      | connect-synapse         | 🔗connect-synapse/🧪️tests/wires-node-b-to-node-c-at-index-1                  |
      | update-synapse          | 🔄update-synapse/🧪️tests/repoints-wire-ab-onto-the-cap-port                  |
      | disconnect-synapse      | ✂️disconnect-synapse/🧪️tests/cuts-wire-ab-leaving-both-nodes                |
      | move-widget             | 📍move-widget/🧪️tests/repositions-node-a-in-the-graph                        |
      | delete-widget-position  | 🧹delete-widget-position/🧪️tests/unpins-the-node-a-position                  |
      | update-camera           | 📷update-camera/🧪️tests/frames-the-graph-at-double-zoom                      |
      | change-schema           | 🔤change-schema/🧪️tests/restamps-the-fixture-schema-id                       |
      | create-generation       | ➕create-generation/🧪️tests/appends-generation-2-and-moves-the-selection     |
      | delete-generation       | 🗑delete-generation/🧪️tests/removes-the-selected-generation-2-and-falls-back |
      | rename-generation       | 🏷rename-generation/🧪️tests/retitles-generation-1-via-new-name               |
      | change-generation-value | 🔧change-generation-value/🧪️tests/raises-the-storeys-answer-in-generation-1  |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: The committed <id> vector changes only what its diff declares
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json",
        "mutation": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json",
        "diff": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🔺️diff/🔣️component.json",
        "outcome": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️component.json",
        "after": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️component.json"
      }
      """
    Then every field where the after-snapshot differs from the before-snapshot is declared by the committed diff
    And every field the committed diff declares actually differs
    Examples:
      | id                      | vector                                                                      |
      | create-widget           | 🌱create-widget/🧪️tests/inserts-node-c-at-index-2                            |
      | update-widget           | 🩹update-widget/🧪️tests/retunes-the-knob-slider-value                        |
      | delete-widget           | ❌delete-widget/🧪️tests/removes-node-a-and-leaves-wire-ab-dangling           |
      | connect-synapse         | 🔗connect-synapse/🧪️tests/wires-node-b-to-node-c-at-index-1                  |
      | update-synapse          | 🔄update-synapse/🧪️tests/repoints-wire-ab-onto-the-cap-port                  |
      | disconnect-synapse      | ✂️disconnect-synapse/🧪️tests/cuts-wire-ab-leaving-both-nodes                |
      | move-widget             | 📍move-widget/🧪️tests/repositions-node-a-in-the-graph                        |
      | delete-widget-position  | 🧹delete-widget-position/🧪️tests/unpins-the-node-a-position                  |
      | update-camera           | 📷update-camera/🧪️tests/frames-the-graph-at-double-zoom                      |
      | change-schema           | 🔤change-schema/🧪️tests/restamps-the-fixture-schema-id                       |
      | create-generation       | ➕create-generation/🧪️tests/appends-generation-2-and-moves-the-selection     |
      | delete-generation       | 🗑delete-generation/🧪️tests/removes-the-selected-generation-2-and-falls-back |
      | rename-generation       | 🏷rename-generation/🧪️tests/retitles-generation-1-via-new-name               |
      | change-generation-value | 🔧change-generation-value/🧪️tests/raises-the-storeys-answer-in-generation-1  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the two-widget graph with its two-generation history
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-generation/🧪️tests/removes-the-selected-generation-2-and-falls-back/📸️snapshot/⬅️before/🔣️component.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
