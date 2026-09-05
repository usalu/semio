@capability-lowpoly-1-mutate
@oracle-lowpoly-python-independent
@comparison-ordered-json-v1
@mutations-lowpoly-1-any
Feature: Apply every typed lowpoly mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a
  second implementation of the `s.lowpoly.lowpoly` document and all seventeen typed mutations, written
  in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from rules 2,
  3 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the seventeen committed quintets. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. This case used to argue that its two-level addressing — an id-keyed object list whose members
  each carry an INDEX-keyed, anonymous paint stack — "is this subset's own specification, not a fact
  an external mesh or scene library could confirm or refute". `mutate-fem3d-1` and
  `🗺️mutate-gisterrain-1` refuted that in this same wave by taking Python second implementations over
  this same carrier. Two levels of addressing are not an obstacle to a second implementation; they are
  something a second implementation must model. A third-party library was nonetheless declined and the
  reason is concrete: no mesh or scene format models a paint stack addressed by index inside an object
  addressed by id, and none of them reads this carrier.

  Three things only the committed vectors state, and both implementations take them from there. First,
  this subset tags its mutations EXTERNALLY: a payload is `{"MoveObject": {…}}`, a PascalCase variant
  name as the single key, where every sibling subset in this repository tags INTERNALLY with a
  `"mutation"` member. A reference that assumed the house convention would fail on all seventeen rows,
  which is why the reference checks the arm rather than a member. Second, `edit-paint-layer` splices
  base64 RUNS into the layer's pixel buffer at byte offsets, overwriting in place and never resizing
  it — the reference decodes and re-encodes those buffers itself, and inverts an edit by capturing the
  same byte ranges out of the pre-mutation buffer, which is exact precisely because the verb cannot
  resize. Third, `create-mesh` carries a `meshWorkspace` argument the snapshot does not hold at all.

  ✅️ ALL SEVENTEEN KINDS ARE ADJUDICATED AND NONE IS REFUSED: the mesh child handle carries the
  caller's own `childId`, so unlike `mutate-block-3d-1`'s `catalog` or `🟩️mutate-program-1`'s
  `knowledge`/`benchmarks`, nothing here depends on a content-addressing function no specification
  states.

  📌️ TWO CEILINGS ON WHAT THIS COMPARISON ESTABLISHES, stated rather than implied. First, the
  SUBJECT half does not run this subset's codec: `🦀️.rs` beside this file links no plugin
  crate and replays the committed vectors, so today the comparison establishes that an independent
  implementation of the specification computes the committed after-snapshots — a real check of the
  vectors, and the class of check that found `🦅️mutate-jack-1`'s wrong vector — but not yet our codec
  against a second producer. A `lowpoly_mutation_report_json` bridge beside the mutation enum closes
  it; it was not added here for two reasons, and neither is that
  the verb is hard: it is PRODUCTION code in a crate this test-side pass deliberately does not
  touch, and it could not be verified end to end today anyway.
  `parity` was not measured for any case in this pass: the single-case probe
  `parity exhaustive --owner 🗒️note --case mutate-note-1` was killed at the runner's OWN 900 s
  per-case budget while still COMPILING the generated subject host — the runner's message names the
  cause, shared cargo target-dir lock contention from a concurrent session — and then threw
  `spawnSync cargo ETIMEDOUT` out of `runProbe` with no summary line at all.
  `📓️w14-final-audit.md` §5.3 measured the underlying blocker one day earlier (`unresolved import
  component::component_persistent_local` in `semio-framework-plugin`, which sits in every generated
  host's dependency graph); this pass did NOT re-verify whether that is still the state, and says so
  rather than repeating it as fact. This subset's own plugin crate compiles clean at
  `cargo check --lib`. Second, this case reads no real-world
  artifact: all 85 of its fixtures are handcrafted specification vectors.

  The committed specification vectors were KEPT, not replaced, and the reference asserts more against
  them than the subject half can: it applies each verb, requires the committed after-snapshot member
  by member, applies its OWN computed inverse and requires the committed before-snapshot back — the
  full inverse law, where the subject half asserts only the weaker footprint precondition.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: The committed <id> vector declares its own kind and moves the document
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "asset://🧬️schema/🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome": "asset://🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
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
  @mode-differential
  Scenario Outline: The committed <id> vector changes only what its diff declares
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "asset://🧬️schema/🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome": "asset://🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
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
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/➖️remove-paint-layer/🧪️tests/🚪️drops-the-detail-layer-at-index-1/📸️snapshot/⬅️before/🔣️.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
