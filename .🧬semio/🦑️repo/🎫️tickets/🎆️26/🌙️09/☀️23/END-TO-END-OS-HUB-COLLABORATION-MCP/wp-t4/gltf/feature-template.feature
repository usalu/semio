@capability-gltf-2-0-mutate
@oracle-json-rust-gltf-2-0-mutate
@comparison-semantic-gltf-v1
@mutations-gltf-2-0-@SHORT@
Feature: Apply every registered glTF 2.0 @SHORT@ mutation to a real-world document
  The `gltf-2-0-@SHORT@` catalog (`../../🔮️oracles/🔣️.json`) declares the @COUNT@ kinds that own
  @SURFACE@. Each kind runs against its own committed `⬅️before.gltf`, written by the three.js
  fixture generator (`../../../♾️any/🏭️generator/📜️script.ts`), with the parameters that recipe
  names.

  The oracle is `json-rust-gltf-2-0-mutate`: a second reading of the glTF 2.0 specification over
  a domain-blind JSON tree, including every reference a top-level family change must move — scene
  roots, children, skin joints and skeleton, animation targets, node meshes, primitive attributes,
  indices and morph targets, inverse-bind matrices, sampler input and output, accessor and image
  buffer views, and each buffer view's buffer. It is a cross-semio implementation, a supplement to
  a third-party reader rather than a substitute for one. The subject parses the same document into
  `GltfSnapshot`, applies the leaf's own typed `apply()` and serializes from the model alone.

  Every kind is also undone. A kind whose payload can carry the undo is inverted by the kinds the
  inverse column names; a kind whose payload cannot (every delete, a node weight override, a new
  morph target) restores the listed top-level members from the original document.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://<input>
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>, "fixture": "shared://<input>"}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | input | params |
@ROWS_MUTATE@

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document shared://<input>
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>, "fixture": "shared://<input>", "inverse": <inverse>, "restore": <restore>}
      """
    And the mutation's own inverse is applied to the result
    Then the document matches its pre-mutation semantic projection
    Examples:
      | id | input | params | inverse | restore |
@ROWS_INVERSE@
