@capability-wires-1-mutate
@no-oracle-wires-1-argument-board-mutation-semantics
@comparison-ordered-json-v1
@mutations-wires-1-any
Feature: Apply every typed wires mutation to its committed vector and to a real-effect payload
  `s.reasoning.wires` is a semio-NATIVE argument board. Its five-line `.wires.dsl.semio` body is
  hex-encoded `DslValue`, nothing third-party reads it, and the thing being mutated is an UNTYPED
  value tree — `wiresFixture` holding `identities`, `relationships` and a nested `board` — edited one
  scalar key at a time through this facet's own `set_node_field`. So no oracle is registered
  (recorded as the `wires-1-argument-board-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`).

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

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion lives inside the subject handler. A handler that merely ran the mutation and returned
  would report a pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed vector and then for real
    Given the committed specification vector for the <id> kind and the committed <base> board
    When <id> is replayed against its vector and then applied for real
      """
      {"kind": "<id>", "base": "<base>", "code": "<code>", "level": "<level>", "params": <params>}
      """
    Then the vector produces exactly <code> at <level> and leaves the committed after-snapshot, and the real payload moves the board
    Examples:
      | id                | base             | code                    | level   | params                                                                                                                                                                                                              |
      | create-node       | create-node      | mutation.duplicate-id   | Fatal   | {"mutation": "createNode", "node": {"id": "node-gamma", "nodeKind": "topic", "shape": "rectangle", "x": 80.0, "y": 40.0, "radius": 24.0, "text": "Gamma", "handles": []}}                                            |
      | delete-node       | delete-node      | mutation.target-missing | Error   | {"mutation": "deleteNode", "nodeId": "node-anchor"}                                                                                                                                                                 |
      | move-node         | move-node        | mutation.no-op          | Warning | {"mutation": "moveNode", "nodeId": "node-drifter", "newX": 48.0, "newY": 36.0}                                                                                                                                       |
      | resize-node       | resize-node      | mutation.no-op          | Warning | {"mutation": "resizeNode", "nodeId": "node-nucleus", "newRadius": 40.0}                                                                                                                                              |
      | change-node-kind  | change-node-kind | mutation.no-op          | Warning | {"mutation": "changeNodeKind", "nodeId": "node-metabolism", "newNodeKind": "identity"}                                                                                                                               |
      | change-node-shape | change-node-shape| mutation.no-op          | Warning | {"mutation": "changeNodeShape", "nodeId": "node-orbit", "newShape": "rectangle"}                                                                                                                                     |
      | edit-node-text    | edit-node-text   | mutation.no-op          | Warning | {"mutation": "editNodeText", "nodeId": "node-thesis", "newText": "Antithesis"}                                                                                                                                       |
      | set-node-root     | set-node-root    | mutation.no-op          | Warning | {"mutation": "setNodeRoot", "nodeId": "node-leaf", "newRoot": true}                                                                                                                                                  |
      | connect-nodes     | disconnect-nodes | mutation.target-missing | Error   | {"mutation": "connectNodes", "edge": {"id": "edge-mentions", "edgeKind": "wires.owns", "source": "node-source", "target": "node-sink"}, "relationship": {"relationshipId": 2.0, "kind": "owns", "sourceIdentityId": 1.0, "targetIdentityId": 2.0, "edgeId": "edge-mentions"}} |
      | disconnect-nodes  | disconnect-nodes | mutation.target-missing | Error   | {"mutation": "disconnectNodes", "edgeId": "edge-owns"}                                                                                                                                                               |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed <base> board
    Given the committed <base> board
    When the real <id> payload is applied to it and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "base": "<base>", "code": "<code>", "level": "<level>", "params": <params>}
      """
    Then the document equals the committed before-snapshot again, member for member
    Examples:
      | id                | base             | code                    | level   | params                                                                                                                                                                                                              |
      | create-node       | create-node      | mutation.duplicate-id   | Fatal   | {"mutation": "createNode", "node": {"id": "node-gamma", "nodeKind": "topic", "shape": "rectangle", "x": 80.0, "y": 40.0, "radius": 24.0, "text": "Gamma", "handles": []}}                                            |
      | delete-node       | delete-node      | mutation.target-missing | Error   | {"mutation": "deleteNode", "nodeId": "node-anchor"}                                                                                                                                                                 |
      | move-node         | move-node        | mutation.no-op          | Warning | {"mutation": "moveNode", "nodeId": "node-drifter", "newX": 48.0, "newY": 36.0}                                                                                                                                       |
      | resize-node       | resize-node      | mutation.no-op          | Warning | {"mutation": "resizeNode", "nodeId": "node-nucleus", "newRadius": 40.0}                                                                                                                                              |
      | change-node-kind  | change-node-kind | mutation.no-op          | Warning | {"mutation": "changeNodeKind", "nodeId": "node-metabolism", "newNodeKind": "identity"}                                                                                                                               |
      | change-node-shape | change-node-shape| mutation.no-op          | Warning | {"mutation": "changeNodeShape", "nodeId": "node-orbit", "newShape": "rectangle"}                                                                                                                                     |
      | edit-node-text    | edit-node-text   | mutation.no-op          | Warning | {"mutation": "editNodeText", "nodeId": "node-thesis", "newText": "Antithesis"}                                                                                                                                       |
      | set-node-root     | set-node-root    | mutation.no-op          | Warning | {"mutation": "setNodeRoot", "nodeId": "node-leaf", "newRoot": true}                                                                                                                                                  |
      | connect-nodes     | disconnect-nodes | mutation.target-missing | Error   | {"mutation": "connectNodes", "edge": {"id": "edge-mentions", "edgeKind": "wires.owns", "source": "node-source", "target": "node-sink"}, "relationship": {"relationshipId": 2.0, "kind": "owns", "sourceIdentityId": 1.0, "targetIdentityId": 2.0, "edgeId": "edge-mentions"}} |
      | disconnect-nodes  | disconnect-nodes | mutation.target-missing | Error   | {"mutation": "disconnectNodes", "edgeId": "edge-owns"}                                                                                                                                                               |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed board through its own DSL carrier and print it back
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the artifact is parsed, printed back to `.wires.dsl.semio` and parsed again
    Then every decoding agrees on the same one-node board carrying `node-1`, and the printed text reproduces the committed file byte for byte
