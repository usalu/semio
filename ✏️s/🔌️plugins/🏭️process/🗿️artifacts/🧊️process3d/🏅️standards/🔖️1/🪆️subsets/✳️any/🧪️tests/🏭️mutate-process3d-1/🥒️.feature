@capability-process3d-1-mutate
@oracle-process3d-1-python-independent
@comparison-ordered-json-v1
@mutations-process3d-1-any
Feature: Apply every typed process.process3d mutation to its committed specification vector and against an independent Python implementation

  `process.process3d` is a semio-NATIVE artifact and nothing outside this repository reads
  `.dsl.semio` — G-code parsers and STEP/BREP kernels were surveyed and DECLINED. The second producer
  a differential comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside
  this file is it: all sixteen kinds of this vocabulary, written in Python from this subset's own
  committed `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and each mutation's own
  payload schema, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  `create`/`delete`/`rename`/`change`/`replace`/`move`/`reorder` verb entries. It imports nothing from
  the Rust it judges and transliterates none of it. The no-oracle decision this replaces
  (`process3d-mutation-semantics`) is narrowed to an empty `capabilities` list rather than deleted,
  because its own investigation remains the honest record of what was checked.

  ⚠️ Honest boundary. `steps` is a content-addressed CHILD HANDLE minting a NEW `childId` (and each
  `toolSolids[]` entry its own) whenever `stepPayloads` changes, through a digest algorithm no schema
  in this repository publishes. The seven STEP-scoped kinds below therefore have the Python side
  verify `stepPayloads` itself — the real, computed content — without claiming to reproduce that
  hash; the other nine kinds touch no content-addressed field (`replace-stock-solid`'s new handle is
  supplied VERBATIM by the payload, never computed) and are verified as a full snapshot equality.
  Both implementations now read the SAME committed bytes: every `(before, mutation, after, outcome)`
  path is a declared `asset://` fixture rather than an `include_str!`-only literal.

  What distinguishes this subset is that one document carries three different collection shapes at
  once, and the vocabulary is derived from that difference rather than applied uniformly. `steps` is
  an id-keyed, ORDER-MEANINGFUL timeline, so it gets `create`, `delete`, `rename`, two field-scoped
  changes (`change-step-enabled`, `change-step-origin`), a large-payload `replace-step-measure`, and
  a `reorder-steps`. `workshop.machines` is an id-keyed but UNORDERED set, so it gets `create`,
  `delete`, `rename`, `change-machine-icon` and `replace-machine-capabilities` — and no reorder,
  because position carries no meaning there. `stock` is a single facet split three ways by the size
  and kind of what changes: `move-stock` for the pose, `change-stock-label` for the identity string,
  and `replace-stock-solid` for the large structured BREP child handle. `change-cursor` is the one
  document-level scalar, the replay position the viewer resolves up to.

  All sixteen kinds are genuine, ticket `26/09/01/PROCESS-END-TO-END`. The seven step-scoped verbs —
  `create-step`, `delete-step`, `rename-step`, `change-step-enabled`, `change-step-origin`,
  `replace-step-measure` and `reorder-steps` — used to be documented no-ops (the timeline read
  through an unresolved composed `s.stdio.semio.flow` child, with no resolver reaching it), but
  `step_payloads` is the durable, inline timeline record since `26/08/12/UNIFIED-COMPOSABLE-
  ARTIFACT-SYSTEM` wave 4, so every one of them now mutates it directly and re-mints `steps`/
  `tool_solids` to match (`process3d_step_timeline_diff`, reusing `process_working_scene_to_
  snapshot`'s own minting). Their committed vectors record the real observed effect, and the
  adapter's `mutation_is_observable` call no longer lists any kind as exempt.

  `replace-machine-capabilities` trades a blade cut for a GATED pocket cut, so a capability list
  that dropped the gate rule fails. `move-stock` lifts AND tilts, so a pose codec that carried
  position and forgot the axis-angle fails. `replace-stock-solid` reissues the stock BREP child
  handle, which is the one place a content-addressed digest has to move.

  The identity round trip reads the artifact's own committed demo example, a 12 KB document whose
  workshop is hex-encoded JSON carrying a bench saw, a drill, an attacher, a circular saw with a
  max-cut-depth rule and a table — the largest committed document in this plugin and the only one
  that exercises the capability-rule vocabulary at all.

  `mutate-<kind>`/`inverse-<kind>` now dispatch BOTH an oracle role (the Python implementation,
  reached through this plugin's `oracleHostPackages` entry) and a subject role (this repository's own
  `process3d_mutation_report_json`, unaffected by this change), each independently asserting the
  forward/inverse laws in role through the shared law module
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` that the stdio subsets use, before the two are
  compared — on `stepPayloads` for the seven step-scoped kinds, on the whole snapshot for the other
  nine. A handler that applied the mutation and returned would report a pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    And the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied to that vector's before-snapshot through process3d_mutation_report_json
    Then the applied snapshot, the produced diff and the outcome's diagnostics are exactly what the vector commits, a kind the vector declares observable really moved the projection, and the two implementations agree
    Examples:
      | id                            | dir                           | fixture                                                       |
      | create-step                   | 🌱create-step                  | 🪚️accepts-a-rip-cut-step-and-inserts-it                      |
      | delete-step                   | 🗑️delete-step                 | 🚫️accepts-a-step-id-and-removes-it                           |
      | rename-step                   | 🏷️rename-step                 | 🔤️accepts-a-new-label-and-applies-it                         |
      | change-step-enabled           | 🔘change-step-enabled          | ⏸️accepts-a-disable-flag-and-applies-it                      |
      | change-step-origin            | 🧷change-step-origin           | 🏭️accepts-a-machine-provenance-and-applies-it                |
      | replace-step-measure          | 📐replace-step-measure         | 🕳️accepts-a-bore-measure-and-replaces-it                     |
      | reorder-steps                 | 🔀reorder-steps                | 🔀️accepts-a-target-index-and-reorders-them                   |
      | create-machine                | 🏭create-machine               | 🪛️adds-a-drill-press-to-the-workshop                         |
      | delete-machine                | ❌delete-machine               | ➖️empties-the-workshop-of-the-saw                            |
      | rename-machine                | 🔖rename-machine               | 🏷️retitles-the-saw                                           |
      | change-machine-icon           | 🎨change-machine-icon          | 🪚️swaps-the-saw-icon                                         |
      | replace-machine-capabilities  | 🔁replace-machine-capabilities | 🕳️trades-the-blade-cut-for-a-gated-pocket-cut                |
      | move-stock                    | 📍move-stock                   | 🎈️lifts-and-tilts-the-stock                                  |
      | change-stock-label            | 🔤change-stock-label           | 🔤️relabels-the-oak-beam-as-planed                            |
      | replace-stock-solid           | 🧊replace-stock-solid          | 🧊️reissues-the-stock-brep-child-handle                       |
      | change-cursor                 | ⏱️change-cursor               | ⏯️pins-the-replay-cursor-to-two-steps                        |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    And the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through process3d_mutation_report_json
    Then the snapshot's projection is the before-snapshot's projection again, any divergence is reported by JSON path, and both implementations agree
    Examples:
      | id                            | dir                           | fixture                                                       |
      | create-step                   | 🌱create-step                  | 🪚️accepts-a-rip-cut-step-and-inserts-it                      |
      | delete-step                   | 🗑️delete-step                 | 🚫️accepts-a-step-id-and-removes-it                           |
      | rename-step                   | 🏷️rename-step                 | 🔤️accepts-a-new-label-and-applies-it                         |
      | change-step-enabled           | 🔘change-step-enabled          | ⏸️accepts-a-disable-flag-and-applies-it                      |
      | change-step-origin            | 🧷change-step-origin           | 🏭️accepts-a-machine-provenance-and-applies-it                |
      | replace-step-measure          | 📐replace-step-measure         | 🕳️accepts-a-bore-measure-and-replaces-it                     |
      | reorder-steps                 | 🔀reorder-steps                | 🔀️accepts-a-target-index-and-reorders-them                   |
      | create-machine                | 🏭create-machine               | 🪛️adds-a-drill-press-to-the-workshop                         |
      | delete-machine                | ❌delete-machine               | ➖️empties-the-workshop-of-the-saw                            |
      | rename-machine                | 🔖rename-machine               | 🏷️retitles-the-saw                                           |
      | change-machine-icon           | 🎨change-machine-icon          | 🪚️swaps-the-saw-icon                                         |
      | replace-machine-capabilities  | 🔁replace-machine-capabilities | 🕳️trades-the-blade-cut-for-a-gated-pocket-cut                |
      | move-stock                    | 📍move-stock                   | 🎈️lifts-and-tilts-the-stock                                  |
      | change-stock-label            | 🔤change-stock-label           | 🔤️relabels-the-oak-beam-as-planed                            |
      | replace-stock-solid           | 🧊replace-stock-solid          | 🧊️reissues-the-stock-brep-child-handle                       |
      | change-cursor                 | ⏱️change-cursor               | ⏯️pins-the-replay-cursor-to-two-steps                        |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed fabrication document, print it back and cross it against its binary encoding
    Given the real committed document asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
