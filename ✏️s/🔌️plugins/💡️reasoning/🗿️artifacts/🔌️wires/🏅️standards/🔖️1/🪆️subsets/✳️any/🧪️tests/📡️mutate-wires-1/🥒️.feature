@capability-wires-1-mutate
@oracle-wires-1-python-independent
@comparison-ordered-json-v1
@mutations-wires-1-any
Feature: Apply every typed wires mutation to its committed vector, to a real-effect payload, and against an independent Python implementation
  `s.reasoning.wires` is a semio-NATIVE argument board. Its five-line `.wires.dsl.semio` body is
  hex-encoded `DslValue`, nothing third-party reads it, and the thing being mutated is an UNTYPED
  value tree — `wiresFixture` holding `identities`, `relationships` and a nested `board` — edited one
  scalar key at a time through this facet's own `set_node_field`. No reference LIBRARY exists. The
  second producer a differential comparison needs is therefore a second IMPLEMENTATION, and
  `🐍️component.py` beside this file is it: all ten kinds of this vocabulary, written in Python from
  this subset's own committed `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and each
  mutation's own payload schema, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  `create`/`delete`/`move`/`resize`/`change`/`edit`/`set`/`connect`/`disconnect` verb entries. It
  imports nothing from the Rust it judges and transliterates none of it. The no-oracle decision this
  replaces (`wires-1-argument-board-mutation-semantics`) is narrowed to an empty `capabilities` list
  rather than deleted, because its own investigation remains the honest record of what was checked.

  Both implementations now read the SAME committed bytes: every `(before, mutation, after, outcome)`
  path — and, for the six no-op kinds, `🔺️diff` — is a declared `asset://` fixture rather than an
  `include_str!`-only literal, so the plan pins its digest and a Python reference can resolve it.

  What genuinely distinguishes this vocabulary is the shape of its committed evidence. Ten kinds,
  ten handcrafted specification vectors under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<slug>/🧪️tests/` — and SIX of them are
  NO-OP vectors, not refusals. Every single-field node verb here degrades to an `applied` outcome
  carrying a `Warning`-level `mutation.no-op` when the field already holds the requested value:
  retyping "Thesis" over "Thesis", setting a `topic` node to `topic`, resizing a radius-24 node to
  24, moving a node with no `y` key at all to `y = 0`, reshaping a circle to a circle, clearing a
  root flag that was never set. Only four kinds refuse outright, and they refuse for two different
  reasons — a missing node or edge is the Error-level `mutation.target-missing`, while an id the
  board already holds is the Fatal `mutation.duplicate-id`. The `code` and `level` columns below
  carry that distinction per kind, and the subject handler requires the exact pair; a refusal
  reported as a no-op, or a no-op reported at Error level, fails.

  A no-op vector proves the degenerate branch and, by construction, moves nothing. So each
  `mutate-<kind>` scenario also applies the real-effect payload in the `params` column to that same
  committed base — a different value on the same address — and requires the projection to MOVE. Only
  `connect-nodes` reads a different base (`disconnect-nodes`'s two-node board with `edge-owns`),
  because its own vector's base deliberately holds only the TARGET node so that the missing SOURCE
  is what gets reported; the `base` column names it.

  `mutate-<kind>`/`inverse-<kind>` now dispatch BOTH an oracle role (the Python implementation,
  reached through this plugin's `oracleHostPackages` entry, comparing the OUTCOME each kind's own
  committed vector commits to) and a subject role (this repository's own real dispatch, unaffected by
  this change — the same `Given`/`When`/`Then` and the same docstring payload as before). A handler
  that merely ran the mutation and returned would report a pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed vector and then for real
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    And the committed specification vector for the <id> kind and the committed <base> board
    When <id> is replayed against its vector and then applied for real
      """
      {"kind": "<id>", "base": "<base>", "code": "<code>", "level": "<level>", "params": <params>}
      """
    Then the vector produces exactly <code> at <level> and leaves the committed after-snapshot, the real payload moves the board, and the two implementations agree on the committed vector's outcome
    Examples:
      | id                | dir               | fixture                                                     | base             | code                    | level   | params                                                                                                                                                                                                              |
      | create-node       | 🌱create-node      | rejects-a-node-id-the-board-already-holds                   | create-node      | mutation.duplicate-id   | Fatal   | {"mutation": "createNode", "node": {"id": "node-gamma", "nodeKind": "topic", "shape": "rectangle", "x": 80.0, "y": 40.0, "radius": 24.0, "text": "Gamma", "handles": []}}                                            |
      | delete-node       | 🗑️delete-node      | rejects-deleting-a-node-the-board-never-held                | delete-node      | mutation.target-missing | Error   | {"mutation": "deleteNode", "nodeId": "node-anchor"}                                                                                                                                                                 |
      | move-node         | 🧭move-node        | reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero       | move-node        | mutation.no-op          | Warning | {"mutation": "moveNode", "nodeId": "node-drifter", "newX": 48.0, "newY": 36.0}                                                                                                                                       |
      | resize-node       | 📐resize-node      | reports-a-no-op-when-the-radius-already-matches             | resize-node      | mutation.no-op          | Warning | {"mutation": "resizeNode", "nodeId": "node-nucleus", "newRadius": 40.0}                                                                                                                                              |
      | change-node-kind  | 🏷️change-node-kind | reports-a-no-op-when-the-kind-already-reads-topic           | change-node-kind | mutation.no-op          | Warning | {"mutation": "changeNodeKind", "nodeId": "node-metabolism", "newNodeKind": "identity"}                                                                                                                               |
      | change-node-shape | 🔷change-node-shape| reports-a-no-op-when-the-shape-already-reads-circle         | change-node-shape| mutation.no-op          | Warning | {"mutation": "changeNodeShape", "nodeId": "node-orbit", "newShape": "rectangle"}                                                                                                                                     |
      | edit-node-text    | ✏️edit-node-text   | reports-a-no-op-when-the-label-is-retyped-verbatim          | edit-node-text   | mutation.no-op          | Warning | {"mutation": "editNodeText", "nodeId": "node-thesis", "newText": "Antithesis"}                                                                                                                                       |
      | set-node-root     | 🚩set-node-root    | reports-a-no-op-when-an-unflagged-node-is-set-to-not-root   | set-node-root    | mutation.no-op          | Warning | {"mutation": "setNodeRoot", "nodeId": "node-leaf", "newRoot": true}                                                                                                                                                  |
      | connect-nodes     | 🤝️connect-nodes    | rejects-an-edge-whose-source-node-is-absent                 | disconnect-nodes | mutation.target-missing | Error   | {"mutation": "connectNodes", "edge": {"id": "edge-mentions", "edgeKind": "wires.owns", "source": "node-source", "target": "node-sink"}, "relationship": {"relationshipId": 2.0, "kind": "owns", "sourceIdentityId": 1.0, "targetIdentityId": 2.0, "edgeId": "edge-mentions"}} |
      | disconnect-nodes  | ✂️disconnect-nodes | rejects-cutting-an-edge-the-board-never-carried             | disconnect-nodes | mutation.target-missing | Error   | {"mutation": "disconnectNodes", "edgeId": "edge-owns"}                                                                                                                                                               |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed <base> board
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    And the committed <base> board
    When the real <id> payload is applied to it and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "base": "<base>", "code": "<code>", "level": "<level>", "params": <params>}
      """
    Then the document equals the committed before-snapshot again, member for member, and both implementations agree
    Examples:
      | id                | dir               | fixture                                                     | base             | code                    | level   | params                                                                                                                                                                                                              |
      | create-node       | 🌱create-node      | rejects-a-node-id-the-board-already-holds                   | create-node      | mutation.duplicate-id   | Fatal   | {"mutation": "createNode", "node": {"id": "node-gamma", "nodeKind": "topic", "shape": "rectangle", "x": 80.0, "y": 40.0, "radius": 24.0, "text": "Gamma", "handles": []}}                                            |
      | delete-node       | 🗑️delete-node      | rejects-deleting-a-node-the-board-never-held                | delete-node      | mutation.target-missing | Error   | {"mutation": "deleteNode", "nodeId": "node-anchor"}                                                                                                                                                                 |
      | move-node         | 🧭move-node        | reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero       | move-node        | mutation.no-op          | Warning | {"mutation": "moveNode", "nodeId": "node-drifter", "newX": 48.0, "newY": 36.0}                                                                                                                                       |
      | resize-node       | 📐resize-node      | reports-a-no-op-when-the-radius-already-matches             | resize-node      | mutation.no-op          | Warning | {"mutation": "resizeNode", "nodeId": "node-nucleus", "newRadius": 40.0}                                                                                                                                              |
      | change-node-kind  | 🏷️change-node-kind | reports-a-no-op-when-the-kind-already-reads-topic           | change-node-kind | mutation.no-op          | Warning | {"mutation": "changeNodeKind", "nodeId": "node-metabolism", "newNodeKind": "identity"}                                                                                                                               |
      | change-node-shape | 🔷change-node-shape| reports-a-no-op-when-the-shape-already-reads-circle         | change-node-shape| mutation.no-op          | Warning | {"mutation": "changeNodeShape", "nodeId": "node-orbit", "newShape": "rectangle"}                                                                                                                                     |
      | edit-node-text    | ✏️edit-node-text   | reports-a-no-op-when-the-label-is-retyped-verbatim          | edit-node-text   | mutation.no-op          | Warning | {"mutation": "editNodeText", "nodeId": "node-thesis", "newText": "Antithesis"}                                                                                                                                       |
      | set-node-root     | 🚩set-node-root    | reports-a-no-op-when-an-unflagged-node-is-set-to-not-root   | set-node-root    | mutation.no-op          | Warning | {"mutation": "setNodeRoot", "nodeId": "node-leaf", "newRoot": true}                                                                                                                                                  |
      | connect-nodes     | 🤝️connect-nodes    | rejects-an-edge-whose-source-node-is-absent                 | disconnect-nodes | mutation.target-missing | Error   | {"mutation": "connectNodes", "edge": {"id": "edge-mentions", "edgeKind": "wires.owns", "source": "node-source", "target": "node-sink"}, "relationship": {"relationshipId": 2.0, "kind": "owns", "sourceIdentityId": 1.0, "targetIdentityId": 2.0, "edgeId": "edge-mentions"}} |
      | disconnect-nodes  | ✂️disconnect-nodes | rejects-cutting-an-edge-the-board-never-carried             | disconnect-nodes | mutation.target-missing | Error   | {"mutation": "disconnectNodes", "edgeId": "edge-owns"}                                                                                                                                                               |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed board through its own DSL carrier and print it back
    Given the real committed text artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the artifact is parsed, printed back to `.wires.dsl.semio` and parsed again
    Then every decoding agrees on the same one-node board carrying `node-1`, and the printed text reproduces the committed file byte for byte
