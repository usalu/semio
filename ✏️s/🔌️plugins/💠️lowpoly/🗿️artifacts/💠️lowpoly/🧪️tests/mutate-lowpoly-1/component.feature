@capability-lowpoly-1-mutate
@no-oracle-lowpoly-mutation-semantics
@comparison-ordered-json-v1
@mutations-lowpoly-1-any
Feature: Replay every typed Lowpoly 1 mutation against its committed specification vector
  `s.lowpoly.lowpoly@1/*` is a semio-NATIVE document, carried as `.dsl.semio`/`.pack.semio`. No third party
  reads those, and none is authoritative over `LowpolyMutation`, so this case rests on the recorded
  `lowpoly-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) and its two named substitutes: the
  committed specification vectors, and the metamorphic laws below.

  What distinguishes this subset is how SHALLOW its document is and how deep its addressing has to be. The
  snapshot has exactly two top-level fields — `schema` and `objects` — so every one of the seventeen kinds
  lands in the SAME field and the footprint law can say almost nothing here; the observability law carries
  the weight instead. The real structure is one level down: each object owns an INDEX-keyed, anonymous
  `paintLayers` sub-collection, so `insert-paint-layer` and `remove-paint-layer` address by POSITION while
  every object-level kind addresses by id. A layer removal that shifted the wrong way is invisible to an
  id-keyed reading, which is why the committed vectors name the index they touch
  (`drops-the-detail-layer-at-index-1`) rather than a layer id.

  This subset also serializes its mutations EXTERNALLY tagged — `{"CreateObject": {…}}`, the Rust variant
  name as the sole object key — where the `🧩️puzzle`, `🧱️block` and `📐️cad` vocabularies use an internal
  `"mutation"` discriminant in camelCase. The adapter checks the encoding this subset actually uses, so a
  vector filed under the wrong leaf fails instead of passing.

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
      | id                            | vector                                                                       |
      | create-object                 | 🌱️create-object/🧪️tests/inserts-obj-mast-between-hull-and-fin                |
      | delete-object                 | 💀️delete-object/🧪️tests/removes-obj-fin-without-touching-the-order           |
      | reorder-objects               | 🔀️reorder-objects/🧪️tests/moves-obj-fin-in-front-of-obj-hull                 |
      | rename-object                 | 🏷️rename-object/🧪️tests/retitles-obj-hull                                    |
      | change-object-smooth-shading  | 🔘️change-object-smooth-shading/🧪️tests/turns-on-smooth-shading-for-obj-hull  |
      | move-object                   | ↗️move-object/🧪️tests/translates-obj-hull-along-x-and-z                      |
      | rotate-object                 | 🔄️rotate-object/🧪️tests/yaws-obj-hull-about-the-y-axis                       |
      | scale-object                  | 📐️scale-object/🧪️tests/halves-obj-hull-uniformly                             |
      | create-mesh                   | 🕸️create-mesh/🧪️tests/attaches-a-mesh-child-handle-to-obj-fin                |
      | delete-mesh                   | 🧨delete-mesh/🧪️tests/detaches-the-mesh-child-handle-from-obj-hull            |
      | insert-paint-layer            | ➕️insert-paint-layer/🧪️tests/stacks-a-detail-layer-above-the-base-layer      |
      | remove-paint-layer            | ➖️remove-paint-layer/🧪️tests/drops-the-detail-layer-at-index-1               |
      | rename-paint-layer            | 🔖️rename-paint-layer/🧪️tests/retitles-the-base-layer-to-undercoat            |
      | change-paint-layer-visible    | 👁️change-paint-layer-visible/🧪️tests/hides-the-base-layer                    |
      | change-paint-layer-opacity    | 🌫️change-paint-layer-opacity/🧪️tests/fades-the-base-layer-to-half            |
      | change-paint-layer-blend-mode | 🎛️change-paint-layer-blend-mode/🧪️tests/switches-the-base-layer-to-multiply  |
      | edit-paint-layer              | 🎨️edit-paint-layer/🧪️tests/paints-red-over-the-second-half-of-the-base-layer |

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
      | id                            | vector                                                                       |
      | create-object                 | 🌱️create-object/🧪️tests/inserts-obj-mast-between-hull-and-fin                |
      | delete-object                 | 💀️delete-object/🧪️tests/removes-obj-fin-without-touching-the-order           |
      | reorder-objects               | 🔀️reorder-objects/🧪️tests/moves-obj-fin-in-front-of-obj-hull                 |
      | rename-object                 | 🏷️rename-object/🧪️tests/retitles-obj-hull                                    |
      | change-object-smooth-shading  | 🔘️change-object-smooth-shading/🧪️tests/turns-on-smooth-shading-for-obj-hull  |
      | move-object                   | ↗️move-object/🧪️tests/translates-obj-hull-along-x-and-z                      |
      | rotate-object                 | 🔄️rotate-object/🧪️tests/yaws-obj-hull-about-the-y-axis                       |
      | scale-object                  | 📐️scale-object/🧪️tests/halves-obj-hull-uniformly                             |
      | create-mesh                   | 🕸️create-mesh/🧪️tests/attaches-a-mesh-child-handle-to-obj-fin                |
      | delete-mesh                   | 🧨delete-mesh/🧪️tests/detaches-the-mesh-child-handle-from-obj-hull            |
      | insert-paint-layer            | ➕️insert-paint-layer/🧪️tests/stacks-a-detail-layer-above-the-base-layer      |
      | remove-paint-layer            | ➖️remove-paint-layer/🧪️tests/drops-the-detail-layer-at-index-1               |
      | rename-paint-layer            | 🔖️rename-paint-layer/🧪️tests/retitles-the-base-layer-to-undercoat            |
      | change-paint-layer-visible    | 👁️change-paint-layer-visible/🧪️tests/hides-the-base-layer                    |
      | change-paint-layer-opacity    | 🌫️change-paint-layer-opacity/🧪️tests/fades-the-base-layer-to-half            |
      | change-paint-layer-blend-mode | 🎛️change-paint-layer-blend-mode/🧪️tests/switches-the-base-layer-to-multiply  |
      | edit-paint-layer              | 🎨️edit-paint-layer/🧪️tests/paints-red-over-the-second-half-of-the-base-layer |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the two-object lowpoly document that carries stacked paint layers
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-paint-layer/🧪️tests/drops-the-detail-layer-at-index-1/📸️snapshot/⬅️before/🔣️component.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
