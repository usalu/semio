@capability-cad-1-mutate
@oracle-cad-python-independent
@comparison-ordered-json-v1
@mutations-cad-1-any
Feature: Apply every typed cad composition mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.cad.cad` document and all twenty typed mutations, written in Python
  from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from rules 1, 2 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the twenty committed quintets. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was only
  half right. This case used to argue two things: that the child-slot lifecycle "IS this subset's
  specification, not a fact an external library could confirm or refute", and that registering a
  general-purpose CAD or JSON-patch crate "would put a library that has never seen this document model
  in the position of the reference, which is worse than recording no oracle at all". The second half
  is correct and still stands. The first half was refuted in this same wave by `mutate-gismap-1`,
  which took a Python second implementation over this same carrier: a second implementation written
  from this subset's own schemas is neither a third-party library nor no reference at all.

  What the document is, and why it is unusual: it holds no geometry. Its whole content is HANDLES to
  other documents — four fixed child slots (`shapeModel`, `buildingModel`, `energyModel`,
  `structureClassicModel`), one child collection (`drawings`) — plus a flat node tree and reference
  lists filed per model-definition id. The reference therefore has to implement the child-target wire
  spelling in both directions: the mutation payloads carry a target as the single string
  `"<artifactId>!<artifactKind>@<standard>/<subset>"` while the snapshot carries it EXPANDED into a
  record, so a reader that never split it could not reproduce a single `create-` vector.

  ✅️ ALL TWENTY KINDS ARE ADJUDICATED AND NONE IS REFUSED. Every child id in this vocabulary is
  supplied by the caller: `create-shape-model` names `shape-model-2` outright and REHANDLES the
  occupied slot. That is the difference between this case and `🧩️mutate-block-3d-1` or
  `🟩️mutate-program-1`, where the corresponding verbs rewrite a child id that is a CONTENT ADDRESS of
  the child document and no specification states the addressing function. Here nothing is
  content-addressed, so nothing has to be guessed — and the inverse of a rehandle is another rehandle
  carrying the displaced handle, which is only expressible for the same reason.

  📌️ TWO CEILINGS ON WHAT THIS COMPARISON ESTABLISHES, stated rather than implied. First, the
  SUBJECT half does not run this subset's codec: `🦀️component.rs` beside this file links no plugin
  crate and replays the committed vectors, so today the comparison establishes that an independent
  implementation of the specification computes the committed after-snapshots — a real check of the
  vectors, and the class of check that found `🦅️mutate-jack-1`'s wrong vector — but not yet our codec
  against a second producer. A `cad_mutation_report_json` bridge beside the mutation enum closes it;
  it was not added here for two reasons, and neither is that
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
  artifact: all 100 of its fixtures are handcrafted specification vectors.

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
      | id                             | vector                                                                                 |
      | create-shape-model             | 🧱create-shape-model/🧪️tests/rehandles-the-occupied-shape-slot                          |
      | delete-shape-model             | 🧨delete-shape-model/🧪️tests/vacates-the-shape-slot                                     |
      | create-building-model          | 🏢create-building-model/🧪️tests/rehandles-the-occupied-building-slot                    |
      | delete-building-model          | 💥delete-building-model/🧪️tests/vacates-the-building-slot                               |
      | create-energy-model            | ⚡create-energy-model/🧪️tests/rehandles-the-occupied-energy-slot                        |
      | delete-energy-model            | 🔌delete-energy-model/🧪️tests/vacates-the-energy-slot                                   |
      | create-structure-classic-model | 🏛️create-structure-classic-model/🧪️tests/rehandles-the-occupied-structure-classic-slot  |
      | delete-structure-classic-model | 💣delete-structure-classic-model/🧪️tests/vacates-the-structure-classic-slot             |
      | create-drawing                 | 📐️create-drawing/🧪️tests/appends-drawing-2                                             |
      | delete-drawing                 | 🧹delete-drawing/🧪️tests/removes-drawing-1                                              |
      | create-node                    | ➕create-node/🧪️tests/appends-node-3                                                    |
      | delete-node                    | 🗑️delete-node/🧪️tests/removes-node-2                                                    |
      | rename-node                    | 🏷️rename-node/🧪️tests/relabels-the-root-node                                            |
      | change-reference-hidden        | 👁️change-reference-hidden/🧪️tests/hides-the-shape-reference                             |
      | change-reference-locked        | 🔒change-reference-locked/🧪️tests/unlocks-the-shape-reference                           |
      | change-reference-width         | 📏change-reference-width/🧪️tests/widens-the-shape-reference-plane                       |
      | move-reference                 | 📍move-reference/🧪️tests/moves-the-shape-reference-off-origin                           |
      | replace-reference-media        | 🖇️replace-reference-media/🧪️tests/reattaches-the-shape-reference-to-a-new-plan          |
      | replace-references             | 📎replace-references/🧪️tests/swaps-the-shape-reference-list                             |
      | change-active-model-definition | 🎯change-active-model-definition/🧪️tests/switches-the-active-pane-to-the-building-model |

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
      | id                             | vector                                                                                 |
      | create-shape-model             | 🧱create-shape-model/🧪️tests/rehandles-the-occupied-shape-slot                          |
      | delete-shape-model             | 🧨delete-shape-model/🧪️tests/vacates-the-shape-slot                                     |
      | create-building-model          | 🏢create-building-model/🧪️tests/rehandles-the-occupied-building-slot                    |
      | delete-building-model          | 💥delete-building-model/🧪️tests/vacates-the-building-slot                               |
      | create-energy-model            | ⚡create-energy-model/🧪️tests/rehandles-the-occupied-energy-slot                        |
      | delete-energy-model            | 🔌delete-energy-model/🧪️tests/vacates-the-energy-slot                                   |
      | create-structure-classic-model | 🏛️create-structure-classic-model/🧪️tests/rehandles-the-occupied-structure-classic-slot  |
      | delete-structure-classic-model | 💣delete-structure-classic-model/🧪️tests/vacates-the-structure-classic-slot             |
      | create-drawing                 | 📐️create-drawing/🧪️tests/appends-drawing-2                                             |
      | delete-drawing                 | 🧹delete-drawing/🧪️tests/removes-drawing-1                                              |
      | create-node                    | ➕create-node/🧪️tests/appends-node-3                                                    |
      | delete-node                    | 🗑️delete-node/🧪️tests/removes-node-2                                                    |
      | rename-node                    | 🏷️rename-node/🧪️tests/relabels-the-root-node                                            |
      | change-reference-hidden        | 👁️change-reference-hidden/🧪️tests/hides-the-shape-reference                             |
      | change-reference-locked        | 🔒change-reference-locked/🧪️tests/unlocks-the-shape-reference                           |
      | change-reference-width         | 📏change-reference-width/🧪️tests/widens-the-shape-reference-plane                       |
      | move-reference                 | 📍move-reference/🧪️tests/moves-the-shape-reference-off-origin                           |
      | replace-reference-media        | 🖇️replace-reference-media/🧪️tests/reattaches-the-shape-reference-to-a-new-plan          |
      | replace-references             | 📎replace-references/🧪️tests/swaps-the-shape-reference-list                             |
      | change-active-model-definition | 🎯change-active-model-definition/🧪️tests/switches-the-active-pane-to-the-building-model |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the reference-bearing CAD composition
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/📎replace-references/🧪️tests/🧭️swaps-the-shape-reference-list/📸️snapshot/⬅️before/🔣️.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
