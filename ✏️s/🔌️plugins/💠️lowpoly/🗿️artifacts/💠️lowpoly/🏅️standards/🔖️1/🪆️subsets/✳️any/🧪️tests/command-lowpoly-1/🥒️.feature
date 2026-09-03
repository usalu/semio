@capability-lowpoly-1-commands
@no-oracle-lowpoly-command-catalog-shape
@comparison-ordered-json-v1
Feature: One representative command per group constructs and reports its manifest id, and patchObject dispatches
  This case exercises the lowpoly editor's declared command CATALOG — 47 commands across 13 groups
  (`✏️patch-object`, `➕️add-primitive`, `🌞️sun`, `🎥️camera`, `👁️chrome`, `💬️engagement`, `📄️fixture`,
  `🔷️mesh-edit`, `🖌️paint`, `🗂️selection`, `🧰️utility`, `🧲️transform`, `🧵️uv`; this ticket's own
  research report's headline count of 48 is off by one against the macro's own row count and the
  crate's own `command_ids_are_unique` test, which asserts 47 — corrected here) — one representative
  command per group, constructed with the same example payload the crate's own `every_command()` test
  helper uses (`🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`'s `#[cfg(test)] mod tests`).

  The generated Rust subject host links only `semio-repo-test-host` and the lowpoly SUT crate. Lowpoly
  therefore re-exports `ArtifactView`, `ConfigView`, `Emit`, `Fault`, and `HistoryView` from
  `semio_s_plugin_lowpoly::editor::lowpoly`, letting generated hosts construct command inputs and
  inspect command emissions without a direct `semio_framework_plugin` dependency. This case retains
  catalog-shape coverage for every group and additionally dispatches `patchObject` through that public
  shim, asserting its one `RenameObject` document mutation and zero config mutations.

  @id-command
  @level-long
  @mode-conformance
  Scenario Outline: The <group> group's representative command reports the <commandId> manifest id
    Given the representative <group> command
      """
      {
        "group": "<group>",
        "commandId": "<commandId>"
      }
      """
    Then constructing the command with its documented example payload succeeds
    And its command_id() equals the declared manifest id
    And LowpolyCommand::TOOL_JOB_IDS contains that id exactly once
    Examples:
      | id            | group         | commandId        |
      | patch-object  | patch-object  | patchObject      |
      | add-primitive | add-primitive | addPrimitive     |
      | sun           | sun           | setSunAzimuth    |
      | camera        | camera        | setCamera        |
      | chrome        | chrome        | toggleShowEdges  |
      | engagement    | engagement    | engagementInput  |
      | fixture       | fixture       | setFixtureJson   |
      | mesh-edit     | mesh-edit     | toggleSmooth     |
      | paint         | paint         | addPaintLayer    |
      | selection     | selection     | setActiveObject  |
      | utility       | utility       | setUtilityParam  |
      | transform     | transform     | transformEnd     |
      | uv            | uv            | unwrapActive     |

  @id-patch-object-dispatch
  @level-long
  @mode-conformance
  Scenario: The patchObject handler emits its rename mutation through the public lowpoly editor shim
    Given a lowpoly snapshot containing object "obj-1" named "Original"
    When patchObject receives a request to rename it to "Renamed"
    Then the generated Rust subject host constructs ArtifactView and ConfigView from the lowpoly crate
    And patchObject emits exactly one RenameObject mutation for "obj-1" to "Renamed"
    And patchObject emits zero config mutations

  @id-catalog-size
  @level-long
  @mode-conformance
  Scenario: The declared command catalog holds exactly the 47 commands the macro's own rows declare
    Given nothing beyond the linked `semio-s-plugin-lowpoly` crate
    Then LowpolyCommand::TOOL_JOB_IDS has exactly 47 entries
    And every entry is unique
