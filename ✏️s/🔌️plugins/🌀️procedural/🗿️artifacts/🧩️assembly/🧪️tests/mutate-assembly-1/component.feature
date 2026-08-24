@capability-assembly-1-mutate
@no-oracle-assembly-mutation-semantics
@comparison-ordered-json-v1
@mutations-assembly-1-any
Feature: Replay every typed Assembly 1 mutation against its committed specification vector
  `s.procedural.assembly@1/*` is a semio-NATIVE document, carried as `.dsl.semio`/`.pack.semio`. No third
  party reads those, and none is authoritative over `AssemblyMutation`, so this case rests on the recorded
  `assembly-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) and its two named substitutes: the
  committed specification vectors, and the metamorphic laws below.

  This is a wave-function-collapse assembly: a `slots` lattice joined by `edges`, a `modules` list of kit
  children, a `rules` adjacency table and a `weights` table, all under one `seed`. Nine kinds, and they
  reach across each other — `delete-slot` must also drop every edge that named the slot, which is why its
  committed vector moves BOTH `slots` and `edges` and why the footprint law is worth stating here at all.

  It is also the only subset in this scope whose diff does not mirror its snapshot. Every collection is
  split in two on the wire — `slotsRemoved` carries ids, `slotsUpserted` carries `(index, record)` pairs —
  so the footprint law has to resolve a changed `slots` field to EITHER half before it can decide whether
  the change was declared. The adapter's alias table spells that out field by field rather than guessing at
  a naming convention.

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
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-slots/🧪️tests/joins-slot-b-to-slot-c-at-index-1/📸️snapshot/⬅️before/🔣️component.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
