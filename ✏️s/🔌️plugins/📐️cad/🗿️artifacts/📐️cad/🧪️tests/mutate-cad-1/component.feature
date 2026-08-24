@capability-cad-1-mutate
@no-oracle-cad-mutation-semantics
@comparison-ordered-json-v1
@mutations-cad-1-any
Feature: Replay every typed CAD 1 mutation against its committed specification vector
  `s.cad.cad@1/*` is a semio-NATIVE composition document, carried as `.dsl.semio`/`.pack.semio`/
  `.op.semio`/`.spr.semio`. No third party reads those, and none is authoritative over `CadMutation`, so
  this case rests on the recorded `cad-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) and its two named substitutes: the
  committed specification vectors, and the metamorphic laws below.

  What distinguishes this subset is that a CAD document owns almost no geometry of its own. It is a
  COMPOSITION: four FIXED child slots (`shapeModel`, `buildingModel`, `energyModel`,
  `structureClassicModel`), each either empty or holding one `s.stdio.semio.model` child by reference, plus
  a `drawings` COLLECTION of the same shape, a node tree, and reference planes filed per model definition
  in `referencesByModelDefinitionId`. The fourteen per-element verbs this enum used to carry were retired
  when that data moved into the child documents, so what is left is exactly slot lifecycle, node lifecycle
  and reference editing — three different collection disciplines in one vocabulary, which is why the
  fixed-slot create kinds are vectored against an ALREADY OCCUPIED slot
  (`rehandles-the-occupied-shape-slot`), the `drawings` create appends (`appends-drawing-2`), and the
  deletes each remove a NAMED member rather than the last one.

  One wire defect is named here rather than excused. `CadDiff::shape_model` and its three sibling slots are
  `Option<Option<CadModelChild>>`, so a VACATED slot renders as `null` on the JSON wire — indistinguishable
  from an untouched one, which `delete-shape-model`'s own fixture test
  (`…/🧨delete-shape-model/🧪️tests/vacates-the-shape-slot/🦀️component.rs`,
  `committed_diff_applies_to_after`) records explicitly. The footprint law below accepts an undeclared
  change on exactly those four fields and only when the new value IS `null`; a slot that changed to
  anything else still fails.

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
      | id                             | vector                                                                                 |
      | create-shape-model             | 🧱create-shape-model/🧪️tests/rehandles-the-occupied-shape-slot                          |
      | delete-shape-model             | 🧨delete-shape-model/🧪️tests/vacates-the-shape-slot                                     |
      | create-building-model          | 🏢create-building-model/🧪️tests/rehandles-the-occupied-building-slot                    |
      | delete-building-model          | 💥delete-building-model/🧪️tests/vacates-the-building-slot                               |
      | create-energy-model            | ⚡create-energy-model/🧪️tests/rehandles-the-occupied-energy-slot                        |
      | delete-energy-model            | 🔌delete-energy-model/🧪️tests/vacates-the-energy-slot                                   |
      | create-structure-classic-model | 🏛create-structure-classic-model/🧪️tests/rehandles-the-occupied-structure-classic-slot  |
      | delete-structure-classic-model | 💣delete-structure-classic-model/🧪️tests/vacates-the-structure-classic-slot             |
      | create-drawing                 | 📐️create-drawing/🧪️tests/appends-drawing-2                                             |
      | delete-drawing                 | 🧹delete-drawing/🧪️tests/removes-drawing-1                                              |
      | create-node                    | ➕create-node/🧪️tests/appends-node-3                                                    |
      | delete-node                    | 🗑delete-node/🧪️tests/removes-node-2                                                    |
      | rename-node                    | 🏷rename-node/🧪️tests/relabels-the-root-node                                            |
      | change-reference-hidden        | 👁change-reference-hidden/🧪️tests/hides-the-shape-reference                             |
      | change-reference-locked        | 🔒change-reference-locked/🧪️tests/unlocks-the-shape-reference                           |
      | change-reference-width         | 📏change-reference-width/🧪️tests/widens-the-shape-reference-plane                       |
      | move-reference                 | 📍move-reference/🧪️tests/moves-the-shape-reference-off-origin                           |
      | replace-reference-media        | 🖇replace-reference-media/🧪️tests/reattaches-the-shape-reference-to-a-new-plan          |
      | replace-references             | 📎replace-references/🧪️tests/swaps-the-shape-reference-list                             |
      | change-active-model-definition | 🎯change-active-model-definition/🧪️tests/switches-the-active-pane-to-the-building-model |

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
      | id                             | vector                                                                                 |
      | create-shape-model             | 🧱create-shape-model/🧪️tests/rehandles-the-occupied-shape-slot                          |
      | delete-shape-model             | 🧨delete-shape-model/🧪️tests/vacates-the-shape-slot                                     |
      | create-building-model          | 🏢create-building-model/🧪️tests/rehandles-the-occupied-building-slot                    |
      | delete-building-model          | 💥delete-building-model/🧪️tests/vacates-the-building-slot                               |
      | create-energy-model            | ⚡create-energy-model/🧪️tests/rehandles-the-occupied-energy-slot                        |
      | delete-energy-model            | 🔌delete-energy-model/🧪️tests/vacates-the-energy-slot                                   |
      | create-structure-classic-model | 🏛create-structure-classic-model/🧪️tests/rehandles-the-occupied-structure-classic-slot  |
      | delete-structure-classic-model | 💣delete-structure-classic-model/🧪️tests/vacates-the-structure-classic-slot             |
      | create-drawing                 | 📐️create-drawing/🧪️tests/appends-drawing-2                                             |
      | delete-drawing                 | 🧹delete-drawing/🧪️tests/removes-drawing-1                                              |
      | create-node                    | ➕create-node/🧪️tests/appends-node-3                                                    |
      | delete-node                    | 🗑delete-node/🧪️tests/removes-node-2                                                    |
      | rename-node                    | 🏷rename-node/🧪️tests/relabels-the-root-node                                            |
      | change-reference-hidden        | 👁change-reference-hidden/🧪️tests/hides-the-shape-reference                             |
      | change-reference-locked        | 🔒change-reference-locked/🧪️tests/unlocks-the-shape-reference                           |
      | change-reference-width         | 📏change-reference-width/🧪️tests/widens-the-shape-reference-plane                       |
      | move-reference                 | 📍move-reference/🧪️tests/moves-the-shape-reference-off-origin                           |
      | replace-reference-media        | 🖇replace-reference-media/🧪️tests/reattaches-the-shape-reference-to-a-new-plan          |
      | replace-references             | 📎replace-references/🧪️tests/swaps-the-shape-reference-list                             |
      | change-active-model-definition | 🎯change-active-model-definition/🧪️tests/switches-the-active-pane-to-the-building-model |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the reference-bearing CAD composition
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎replace-references/🧪️tests/swaps-the-shape-reference-list/📸️snapshot/⬅️before/🔣️component.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
