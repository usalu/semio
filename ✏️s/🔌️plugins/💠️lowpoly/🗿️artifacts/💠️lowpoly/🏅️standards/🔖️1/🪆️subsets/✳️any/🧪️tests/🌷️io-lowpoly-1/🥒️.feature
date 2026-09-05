@capability-lowpoly-1-io
@no-oracle-lowpoly-io-native-round-trip
@comparison-ordered-json-v1
Feature: Round-trip the lowpoly document through every non-PNG stdio format the Rust IO layer declares
  This case is a ROUND-TRIP over `LowpolyMutation`'s carrier subset's own IO bridge:
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`'s `import_stdio_kinds()`/
  `export_stdio_kinds()` both declare nine `stdio.*` formats — `dwg`, `gltf`, `json`, `las`, `obj`,
  `ply`, `png`, `stl`, `txt`. `png` is covered by the Pillow-backed sibling `🟩️io-lowpoly-png-1` case;
  every scenario here takes the SAME committed `LowpolySnapshot` fixture
  (`local://lowpoly-snapshot.json`, two objects: one with a mesh child handle and a paint layer, one
  bare) and exports it through this subset's own `serialize_bytes` for the named format. For the four
  formats whose serializer is real (`json`, `obj`, `ply`, `txt`) the produced bytes are imported back
  through the matching `deserialize_bytes` and the re-imported document must equal the committed
  original member for member. For the four formats whose serializer is a committed HONEST stub
  (`dwg`, `gltf`, `las`, `stl`) `serialize_bytes` must instead return its documented explicit error —
  there is nothing to import back.

  📌️ WHY NO ORACLE (`@no-oracle-lowpoly-io-native-round-trip`, recorded in this subset's own
  `🧪️oracle/🔣️.json`). `LowpolyObject.mesh` is a content-addressed HANDLE
  (`store::ArtifactChild<SemioMeshSnapshot>`), never embedded geometry — see that field's own doc
  comment in `🗿️artifacts/💠️lowpoly/🦀️.rs`. Because of that, four of these eight exporters
  (`dwg`, `gltf`, `las`, `stl`) are committed, HONEST stubs that unconditionally return an error —
  their own doc comments name the exact reason, and the production crate's own
  `unimplemented_geometry_formats_error_honestly_instead_of_lying` unit test
  (`../../🚪️io/🦀️.rs`) already asserts `serialize_bytes` returns `Err` for all four. A genuinely
  independent THIRD-PARTY reader (`tobj`/`ply-rs`/`stl_io`, already vendored behind the sibling
  `🗄️stdio` plugin's `🧪️oracle/📦️packages/🦀️rust` crate) could still validate that the four working
  exporters (`obj`, `ply`, `json`, `txt`) emit well-formed bytes in their target grammar, but reaching
  that crate from here needs an `oracleHostPackages` contribution registered at an ANCESTOR path of
  this artifact (this subset's own `🧪️oracle/🔣️.json` is a DESCENDANT of this case's owner, the wrong
  direction for `oracleHostPackagesFor`'s prefix match) — outside the file ownership this pass was
  granted, so it is recorded as a handoff item rather than added unilaterally. `metamorphic-laws` is
  therefore the substitute this decision actually rests on: export-then-import against our own codec
  is the achievable, honest evidence at this subset's current architecture, and asserting the honest
  stub's own error for the other four is the achievable, honest evidence there.

  📌️ FOUR OF EIGHT SCENARIOS ASSERT AN ERROR, NOT A ROUND TRIP, ON PURPOSE: `dwg`, `gltf`, `las`,
  `stl` are honest stubs, out of THIS ticket's owned files (`🚪️io/**` is off limits to this pass) and
  the architecture work their own doc comments name is a separate, larger undertaking. `txt` was a
  fifth stub when this case was first drafted — a concurrent agent finished it mid-pass, and its row
  now exercises the real round trip. Each of the remaining four rows asserts the SAME committed,
  documented error `serialize_bytes` already returns today (content-addressed mesh handle unreachable
  at this layer) rather than a round trip that architecture cannot yet support — a scenario that
  demanded the round trip today would only ever be able to fail, which is not evidence of anything;
  asserting today's real, honest error is. Should any of these four formats gain a real implementation,
  its row must be promoted to the round-trip law by hand — never left silently green against a
  superseded error string.

  @id-roundtrip
  @level-long
  @mode-round-trip
  Scenario Outline: Export the committed document through <format> and import it back unchanged
    Given the committed lowpoly document
      """
      {
        "format": "<format>",
        "document": "local://lowpoly-snapshot.json"
      }
      """
    When it is exported through this subset's own `serialize_bytes` for <format> and the produced bytes are imported back through the matching `deserialize_bytes`
    Then the re-imported document equals the committed original member for member
    Examples:
      | id   | format |
      | json | json   |
      | obj  | obj    |
      | ply  | ply    |
      | txt  | txt    |

  @id-roundtrip
  @level-long
  @mode-round-trip
  Scenario Outline: Export the committed document through <format> and require the documented honest error
    Given the committed lowpoly document
      """
      {
        "format": "<format>",
        "document": "local://lowpoly-snapshot.json"
      }
      """
    When it is exported through this subset's own `serialize_bytes` for <format>
    Then `serialize_bytes` returns the documented explicit error naming the unreachable content-addressed mesh handle, never a silent pack-envelope lie and never a successful round trip
    Examples:
      | id   | format |
      | dwg  | dwg    |
      | gltf | gltf   |
      | las  | las    |
      | stl  | stl    |
