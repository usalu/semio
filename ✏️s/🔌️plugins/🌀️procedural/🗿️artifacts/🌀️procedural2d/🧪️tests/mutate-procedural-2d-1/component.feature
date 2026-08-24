@capability-procedural-2d-1-mutate
@no-oracle-procedural-2d-mutation-semantics
@comparison-ordered-json-v1
@mutations-procedural-2d-1-any
Feature: Replay every typed Procedural2d 1 mutation against its committed specification vector
  `s.procedural.procedural2d@1/*` is a semio-NATIVE document, carried as `.dsl.semio`/`.pack.semio`. No
  third party reads those, and none is authoritative over `Procedural2dMutation`, so this case rests on the
  recorded `procedural-2d-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) and its two named substitutes: the
  committed specification vectors, and the metamorphic laws below.

  This subset shares a snapshot SHAPE with `🧊️procedural3d` — a `fixture` graph and a `generation` history
  — and deliberately does NOT share its vocabulary. Editing here is by whole-value REPLACEMENT:
  `replace-widget` and `replace-synapse` swap a record outright where the 3d subset patches fields with
  `update-widget`/`update-synapse`, and `clear-widget-layout` drops the layout map for the whole document
  where the 3d subset removes exactly one widget's position with `delete-widget-position`. Three of the
  fourteen kinds differ, and this case measures this catalog, not that one.

  One naming artefact is worth stating so the vectors read honestly: `update-camera`'s triad leaf is still
  named `🎛set-camera` from the generic slot it was repurposed from, while its `SemanticDescriptor` — and
  therefore its catalog kind — is `update-camera`. The row below names the leaf path in full, so the
  mismatch is visible rather than something a reader has to discover.

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
      | id                      | vector                                                                                      |
      | create-widget           | 🌱create-widget/🧪️tests/inserts-note-c-at-index-2                                            |
      | replace-widget          | 🔁replace-widget/🧪️tests/rewrites-the-note-b-body-in-place                                   |
      | delete-widget           | 🗑️delete-widget/🧪️tests/removes-note-a-and-flags-the-dangling-synapse                       |
      | connect-synapse         | 🔗connect-synapse/🧪️tests/joins-note-b-to-note-c-at-index-1                                  |
      | replace-synapse         | 🔄replace-synapse/🧪️tests/repoints-link-ab-onto-the-alt-port                                 |
      | disconnect-synapse      | ✂️disconnect-synapse/🧪️tests/severs-link-ab-leaving-both-notes                              |
      | move-widget             | 📍move-widget/🧪️tests/repositions-note-a-on-the-canvas                                       |
      | clear-widget-layout     | 🧹clear-widget-layout/🧪️tests/drops-the-note-a-layout-entry                                  |
      | update-camera           | 🎛set-camera/🧪️tests/pans-and-zooms-the-graph-camera                                         |
      | change-schema           | 🔤change-schema/🧪️tests/restamps-the-fixture-schema                                          |
      | create-generation       | ➕create-generation/🧪️tests/appends-generation-2-and-selects-it                              |
      | delete-generation       | ➖delete-generation/🧪️tests/removes-the-selected-generation-2-and-falls-back-to-generation-1 |
      | rename-generation       | 🏷️rename-generation/🧪️tests/retitles-generation-1                                           |
      | change-generation-value | 🔢change-generation-value/🧪️tests/raises-the-height-answer-in-generation-1                   |

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
      | id                      | vector                                                                                      |
      | create-widget           | 🌱create-widget/🧪️tests/inserts-note-c-at-index-2                                            |
      | replace-widget          | 🔁replace-widget/🧪️tests/rewrites-the-note-b-body-in-place                                   |
      | delete-widget           | 🗑️delete-widget/🧪️tests/removes-note-a-and-flags-the-dangling-synapse                       |
      | connect-synapse         | 🔗connect-synapse/🧪️tests/joins-note-b-to-note-c-at-index-1                                  |
      | replace-synapse         | 🔄replace-synapse/🧪️tests/repoints-link-ab-onto-the-alt-port                                 |
      | disconnect-synapse      | ✂️disconnect-synapse/🧪️tests/severs-link-ab-leaving-both-notes                              |
      | move-widget             | 📍move-widget/🧪️tests/repositions-note-a-on-the-canvas                                       |
      | clear-widget-layout     | 🧹clear-widget-layout/🧪️tests/drops-the-note-a-layout-entry                                  |
      | update-camera           | 🎛set-camera/🧪️tests/pans-and-zooms-the-graph-camera                                         |
      | change-schema           | 🔤change-schema/🧪️tests/restamps-the-fixture-schema                                          |
      | create-generation       | ➕create-generation/🧪️tests/appends-generation-2-and-selects-it                              |
      | delete-generation       | ➖delete-generation/🧪️tests/removes-the-selected-generation-2-and-falls-back-to-generation-1 |
      | rename-generation       | 🏷️rename-generation/🧪️tests/retitles-generation-1                                           |
      | change-generation-value | 🔢change-generation-value/🧪️tests/raises-the-height-answer-in-generation-1                   |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the two-widget graph with its two-generation history
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-generation/🧪️tests/removes-the-selected-generation-2-and-falls-back-to-generation-1/📸️snapshot/⬅️before/🔣️component.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
