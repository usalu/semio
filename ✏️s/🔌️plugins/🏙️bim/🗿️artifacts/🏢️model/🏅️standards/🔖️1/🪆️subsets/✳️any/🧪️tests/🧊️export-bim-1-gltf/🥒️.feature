@capability-bim-1-export-gltf
@oracle-bim-1-three-gltf
@comparison-floating-point-v1
Feature: Load the binary glTF export of the BIM house with three.js and measure its scene graph
  The subject writes the committed house model (one site, one building, four storeys and every element family) as a binary glTF 2.0 file: one node per element under the
  site, building and storey nodes, one mesh per element with a primitive per material, PBR materials with translucent glazing, vertices converted from the model's Z-up
  frame to glTF's Y-up frame. The three.js 0.182.0 oracle never sees the subject's writer: its GLTFLoader parses the committed file, builds the scene graph and the oracle
  counts the nodes, meshes, primitives, triangles read from the index buffers and materials, the element nodes per kind and per storey, and the world bounds of every
  vertex through the whole chain of node transforms. The subject reports the same table from its typed document, working from the vertices it wrote as 32-bit floats.

  @id-export-gltf-house
  @level-quick
  @mode-differential
  Scenario: Node, mesh, triangle and material counts and the world bounds of the exported house equal the subject's report
    Given the committed house export shared://🧊️gltf/🏠️house/🏠️house.glb
    And the authored house snapshot it is exported from shared://🏗️ifc/🏠️house/📸️snapshot/🔣️.json
    When the file is parsed by GLTFLoader and every node, primitive and vertex is counted and measured
    Then the counts, the element nodes per kind and storey and the world bounds equal the subject's within 1e-9
