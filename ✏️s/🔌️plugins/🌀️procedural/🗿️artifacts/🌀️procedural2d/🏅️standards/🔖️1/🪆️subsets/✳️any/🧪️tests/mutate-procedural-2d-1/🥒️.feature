@capability-procedural-2d-1-mutate
@oracle-procedural-2d-python-independent
@comparison-ordered-json-v1
@mutations-procedural-2d-1-any
Feature: Apply every typed procedural2d mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.procedural.procedural2d` document and all fourteen typed mutations, written in
  Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from rules 1, 2, 3
  and 4 of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`,
  and from the fourteen committed quintets. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. This case used to argue that whether this subset takes whole-value `replace-widget`/`replace-synapse` or field-wise
  `update-*`, and a document-wide `clear-widget-layout` or a per-widget `delete-widget-position`, "IS
  the specification, not a fact an external library could confirm or refute". `mutate-fem2d-1`, `mutate-fem3d-1` and
  `mutate-gismap-1` refuted that in this same wave by taking Python second implementations over this
  same carrier. Which shape a vocabulary takes is not an obstacle to a second implementation; it is
  what a second implementation is written from. A third-party library was nonetheless declined and the
  reason is concrete: no node-graph format models a graph whose layout is a SPARSE side table keyed by
  node id and whose document's second half is an unrelated parameter history, and none of them reads
  this carrier.

  Four things only the committed vectors state, and both implementations take them from there. This
  subset tags its mutations EXTERNALLY — a payload is `{"CreateWidget": {…}}`, a PascalCase variant
  name as the single key. `delete-widget` does NOT cascade: it removes the widget and DELIBERATELY
  leaves both the synapse that named it and its layout entry standing, which is why the layout map has
  its own `clear-widget-layout` verb and why the reference's own document validator refuses to require live
  endpoints. `create-generation` appends AND selects. `delete-generation` falls back to the first
  remaining generation when the one it removed was selected.

  ✅️ ALL FOURTEEN KINDS ARE ADJUDICATED AND NONE IS REFUSED: this document holds no composed child,
  so nothing here depends on a content-addressing function no specification states — the blocker
  `mutate-block-3d-1` and `mutate-program-1` both report.

  📌️ A SIBLING NOTE, because the count of second implementations must not be overstated. This
  reference and `mutate-procedural-3d-1`'s are ONE implementation instantiated twice: the two
  documents are the same shape and the two vocabularies differ only in three kind names and four
  argument names — which is precisely what both no-oracle decisions called the specification. Writing
  them side by side surfaced two real divergences that neither case could see alone: `delete-widget`
  raises `mutation.cascade` at level `info` in `mutate-procedural-2d-1` and raises NOTHING in
  `mutate-procedural-3d-1`, for an effect that is byte-for-byte identical in both committed vectors;
  and `s.procedural.procedural2d` spells one argument `question_id` in snake_case — the only
  snake_case identifier in either document model — where its sibling spells it `questionId`.

  📌️ TWO CEILINGS ON WHAT THIS COMPARISON ESTABLISHES, stated rather than implied. First, the
  SUBJECT half does not run this subset's codec: `🦀️component.rs` beside this file links no plugin
  crate and replays the committed vectors, so today the comparison establishes that an independent
  implementation of the specification computes the committed after-snapshots — a real check of the
  vectors, and the class of check that found `mutate-jack-1`'s wrong vector — but not yet our codec
  against a second producer. A `procedural2d_mutation_report_json` bridge beside the mutation enum closes it; it was not added here for two reasons, and neither is that
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
  `cargo check --lib`. Second, this case reads no real-world artifact: all 70 of its
  fixtures are handcrafted specification vectors.

  The committed specification vectors were KEPT, not replaced, and the reference asserts more against
  them than the subject half can: it applies each verb, requires the committed after-snapshot half by
  half, requires that the verb moved exactly ONE of the two halves, applies its OWN computed inverse
  and requires the committed before-snapshot back — the full inverse law, where the subject half
  asserts only the weaker footprint precondition.

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
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/➖delete-generation/🧪️tests/removes-the-selected-generation-2-and-falls-back-to-generation-1/📸️snapshot/⬅️before/🔣️.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
