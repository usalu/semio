@capability-procedural-3d-1-io
@oracle-procedural-3d-io-python-independent
@comparison-semantic-mesh-v1
Feature: Encode one committed cube in every text mesh grammar generation3d exports, read it back in another language, and require the same geometry
  This case is a CROSS-LANGUAGE DIFFERENTIAL over `s.procedural.generation3d`'s IO surface. The
  reference is `🐍️.py` in this directory: an independent Python implementation of the three text mesh
  grammars this artifact writes and reads — ASCII STL, Wavefront OBJ and ASCII PLY — written from the
  format specifications and from `🧫️fixtures/🚪️io/🧊️unit-cube/🔣️.json`. It imports nothing from this
  repository's Rust.

  Why this case exists at all, stated plainly, because the defect it guards against was invisible for
  a long time. Until ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, seven of this artifact's nine format
  leaves were placeholders that COULD NOT FAIL. Export returned
  `print_dsl(snapshot).into_bytes()` — the artifact's own DSL text — under the target format's name,
  for `las`, `png`, `stl`, `dwg`, `obj`, `gltf` and `ply` alike, byte-identical bodies in all seven.
  Import did `let _ = bytes; Ok(Generation3dSnapshot::default())`: it discarded the file and answered
  with an EMPTY document. Neither direction ever raised an error, so every one of them read as
  success in the UI, and no test in the artifact contradicted them. The single assertion that
  separates a working codec from that state is a reader that did not come from the same source as
  the writer, which is what this case is.

  What both halves are given, and what each may use. ONE geometry: the committed unit cube — 8
  shared vertices, 12 triangles, bounds [0,0,0]..[1,1,1], enclosed volume exactly 1.0. It is a unit
  cube on purpose: every quantity a mesh format can lose is independently checkable on it, and 1.0 is
  exact in binary floating point, so no format's `f32` storage introduces a tolerance of its own. The
  ORACLE half writes and reads the three grammars itself. The SUBJECT half has no grammar: a
  generated test host may not link a plugin crate, so it replays the committed vector, computes the
  projection that vector implies with its own arithmetic, and checks only the file-level laws that
  need no parser — the container's opening token and the count of the record each grammar repeats
  once per triangle. A file that carried the right magic and the wrong geometry therefore passes the
  subject half and FAILS the comparison, which is precisely the division of labour intended.

  📌️ ONE CEILING ON WHAT THIS COMPARISON ESTABLISHES, stated rather than implied, and it is the same
  one this subset's sibling `🧊️mutate-procedural-3d-1` records: the SUBJECT half does not run this
  artifact's own codec, because it cannot link it. What this case establishes today is that an
  independent implementation of the three grammars, in another language, computes the committed
  geometry from the committed encodings. Our codec is exercised against those same bytes elsewhere
  and deliberately: `[[test]] io-round-trip` (`🚪️io/🧪️tests/🔁️round-trip/🦀️.rs`) drives the same cube
  out through every export leaf and back in through every import leaf, and additionally feeds THIS
  case's Python-written committed files through the real import leaves — so the two implementations
  do meet, in that lane, on both sides of the boundary. A third-party library is exercised there too:
  `parry3d` recomputes the enclosed volume and bounds of every recovered mesh, so the expected 1.0 is
  never only this repository's own arithmetic.

  📌️ A THIRD-PARTY LIBRARY WAS DECLINED FOR THIS LANE, not merely absent, and the reason is
  concrete. The question this case asks is not "does a mesh library agree about a cube" — that is
  what the `parry3d` check in the Rust lane already answers. It is "does a reader written from the
  SPECIFICATION, by someone who did not read our writer, recover our geometry from our bytes", and a
  library that shares an ecosystem with the subject cannot answer it in a second language. The three
  grammars involved are small, fully specified plain text; a second implementation of them is a
  complete reference rather than an approximation of one.

  📌️ WHAT THIS CASE DOES NOT COVER, so the count of formats is not overstated: `gltf` and `dwg` are
  binary/JSON container formats whose encoders are not re-implementable in a page of Python without
  becoming a worse copy of the artifact's own codec, and `las` is export-only for this artifact by
  design (a point cloud has no surface for any BRep import operator to consume). All three are
  covered by the in-crate round-trip lane with the `parry3d` third-party check; only the three text
  grammars are adjudicated cross-language here.

  @id-read
  @level-quick
  @mode-differential
  Scenario Outline: The committed <format> encoding of the unit cube is read back as the unit cube
    Given the committed second-implementation encoding
      """
      {
        "format": "<format>",
        "encoding": "shared://🚪️io/🧊️unit-cube/<file>",
        "cube": "shared://🚪️io/🧊️unit-cube/🔣️.json"
      }
      """
    Then the encoding opens with its own format's container token
    And it exposes one record per triangle of the committed cube
    And an independent reader recovers the committed cube's triangle count, bounds and enclosed volume
    Examples:
      | format | file                          |
      | stl    | 🔺️second-implementation.stl  |
      | obj    | 🗿️second-implementation.obj  |
      | ply    | 🧱️second-implementation.ply  |

  @id-round-trip
  @level-quick
  @mode-round-trip
  Scenario Outline: Writing and re-reading the unit cube in <format> does not move the geometry the format can carry
    Given the committed unit cube
      """
      {
        "format": "<format>",
        "cube": "shared://🚪️io/🧊️unit-cube/🔣️.json"
      }
      """
    Then the committed cube encloses unit volume with outward winding
    And writing it in the format and reading it back recovers the same triangle count, bounds and enclosed volume
    And the recovered vertex count is the one the format's own model can carry, not the one another format's can
    Examples:
      | format |
      | stl    |
      | obj    |
      | ply    |
