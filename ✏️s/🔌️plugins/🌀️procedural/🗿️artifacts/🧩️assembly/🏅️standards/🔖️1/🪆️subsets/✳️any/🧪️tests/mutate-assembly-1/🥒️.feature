@capability-assembly-1-mutate
@oracle-assembly-python-independent
@comparison-ordered-json-v1
@mutations-assembly-1-any
Feature: Apply every typed assembly mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.procedural.assembly` document and all nine typed mutations, written
  in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from rules 2,
  4 and 8 of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`,
  and from the nine committed quintets. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. This case used to argue that joining a slot lattice, an adjacency rule set and a per-module
  weight table under one `seed` — with a diff that SPLITS every collection into a
  `<name>Removed`/`<name>Upserted` pair — "is this subset's own specification, not a fact an external
  library could confirm or refute". `mutate-fem3d-1` and `mutate-gisterrain-1` refuted that in this
  same wave by taking Python second implementations over this same carrier. A third-party library was
  nonetheless declined and the reason is concrete: this document is the INPUT to a wave-function-collapse
  solve, not its output; a WFC library computes a collapse, none of them carries the problem statement
  as a document, and none of them reads this carrier.

  Three things only the committed vectors state, and both implementations take them from there. This
  subset tags its mutations EXTERNALLY — a payload is `{"CreateSlot": {…}}`, a PascalCase variant name
  as the single key. `delete-slot` DOES cascade into the edges naming it, and says so with an
  `info`-level `mutation.cascade`. And `change-weight` UPSERTS: it writes an existing module's weight
  in place and APPENDS an entry when the table holds none, which is why the reference refuses to claim
  an inverse for a `remove-weight` on a non-trailing entry — `change-weight` can only append a missing
  one, so the closed vocabulary cannot restore its position.

  📌️ A CROSS-CASE DIVERGENCE THE REFERENCE SURFACED, which neither case could see alone.
  `mutation.cascade` at level `info` means OPPOSITE things inside this one plugin: here it announces
  that `delete-slot` really did remove the edges naming the slot, while in the sibling
  `s.procedural.generation2d` the same code at the same level announces that `delete-widget` LEFT a
  dangling synapse standing. One diagnostic code, two contradictory readings. The reference asserts
  the cascade claim by READING the committed outcome — a vector that declares the code must have moved
  a second member, one that does not must have moved exactly one — rather than from a list of its own,
  which is what makes the divergence visible instead of absorbed.

  ✅️ ALL NINE KINDS ARE ADJUDICATED AND NONE IS REFUSED: the module child handles are only ever read
  here, never re-addressed, so nothing depends on a content-addressing function no specification
  states — the blocker `mutate-block-3d-1` and `mutate-program-1` both report.

  📌️ TWO CEILINGS ON WHAT THIS COMPARISON ESTABLISHES, stated rather than implied. First, the
  SUBJECT half does not run this subset's codec: `🦀️component.rs` beside this file links no plugin
  crate and replays the committed vectors, so today the comparison establishes that an independent
  implementation of the specification computes the committed after-snapshots — a real check of the
  vectors, and the class of check that found `mutate-jack-1`'s wrong vector — but not yet our codec
  against a second producer. An `assembly_mutation_report_json` bridge beside the mutation enum closes
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
  artifact: all 45 of its fixtures are handcrafted specification vectors.

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
      | id               | vector                                                        |
      | create-slot      | 🌱create-slot/🧪️tests/appends-slot-c-at-index-2                |
      | delete-slot      | 🗑️delete-slot/🧪️tests/removes-slot-a-and-cascades-edge-ab     |
      | create-rule      | 🌱create-rule/🧪️tests/appends-a-rule-forbidding-roof-over-wall |
      | delete-rule      | 🗑️delete-rule/🧪️tests/removes-the-wall-roof-rule              |
      | change-weight    | 🔢change-weight/🧪️tests/raises-the-wall-module-selection-bias  |
      | remove-weight    | 🗑️remove-weight/🧪️tests/drops-the-wall-module-weight-override |
      | connect-slots    | 🔗connect-slots/🧪️tests/joins-slot-b-to-slot-c-at-index-1      |
      | disconnect-slots | ✂️disconnect-slots/🧪️tests/severs-edge-ab-leaving-both-slots  |
      | change-seed      | 🎲change-seed/🧪️tests/reseeds-the-solve-from-7-to-99           |

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
      | id               | vector                                                        |
      | create-slot      | 🌱create-slot/🧪️tests/appends-slot-c-at-index-2                |
      | delete-slot      | 🗑️delete-slot/🧪️tests/removes-slot-a-and-cascades-edge-ab     |
      | create-rule      | 🌱create-rule/🧪️tests/appends-a-rule-forbidding-roof-over-wall |
      | delete-rule      | 🗑️delete-rule/🧪️tests/removes-the-wall-roof-rule              |
      | change-weight    | 🔢change-weight/🧪️tests/raises-the-wall-module-selection-bias  |
      | remove-weight    | 🗑️remove-weight/🧪️tests/drops-the-wall-module-weight-override |
      | connect-slots    | 🔗connect-slots/🧪️tests/joins-slot-b-to-slot-c-at-index-1      |
      | disconnect-slots | ✂️disconnect-slots/🧪️tests/severs-edge-ab-leaving-both-slots  |
      | change-seed      | 🎲change-seed/🧪️tests/reseeds-the-solve-from-7-to-99           |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the three-slot wave-function-collapse assembly
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/🔗connect-slots/🧪️tests/joins-slot-b-to-slot-c-at-index-1/📸️snapshot/⬅️before/🔣️.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
