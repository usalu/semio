@capability-lowpoly-1-commands
@no-oracle-lowpoly-command-catalog-shape
@comparison-ordered-json-v1
Feature: One representative command per group constructs and reports its manifest id correctly
  This case exercises the lowpoly editor's declared command CATALOG — 47 commands across 13 groups
  (`✏️patch-object`, `➕️add-primitive`, `🌞️sun`, `🎥️camera`, `👁️chrome`, `💬️engagement`, `📄️fixture`,
  `🔷️mesh-edit`, `🖌️paint`, `🗂️selection`, `🧰️utility`, `🧲️transform`, `🧵️uv`; this ticket's own
  research report's headline count of 48 is off by one against the macro's own row count and the
  crate's own `command_ids_are_unique` test, which asserts 47 — corrected here) — one representative
  command per group, constructed with the same example payload the crate's own `every_command()` test
  helper uses (`🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`'s `#[cfg(test)] mod tests`).

  ⚠️ REDUCED SCOPE, STATED HONESTLY — READ BEFORE EXTENDING THIS CASE. The original intent (this
  ticket's own brief) was: dispatch each representative command against a known starting snapshot and
  assert the MUTATION it produces. That is NOT achievable from an externally generated Rust test host
  today, for EVERY plugin in this repository, not only lowpoly — confirmed by
  `grep -rl "ArtifactView\|ConfigView" --include="🦀️.rs" ✏️s | grep 🧪️tests/` returning NOTHING.
  `semio_framework_plugin::app_commands!` generates `LowpolyCommand::dispatch(&self, doc: &ArtifactView<..>,
  cfg: &ConfigView<..>, ctx: &mut LowpolyScratch) -> Result<Emit<..>, Fault>`, and every per-command
  `handle()` carries the identical `ArtifactView`/`ConfigView` parameter pair — both types live in
  `semio_framework_plugin`, which no generated Rust test host links: `materializeRustHost`
  (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts`) wires exactly THREE crates —
  `semio-repo-test-host` (dependency-free by design), this case owner's OWN `sut` crate found by
  walking UP from the case's owner directory, and `contributedOraclePackages` resolved from an
  `oracleHostPackages` array declared in an ANCESTOR-scoped `🧪️oracle/🔣️.json` — and lowpoly declares
  none. `Mutation::diff`/`apply` (needed to verify a mutation's EFFECT) are similarly gated behind the
  `protocol` crate's `Mutation` trait, also unlinked. Registering `semio-framework-plugin` (and
  `protocol`) as an `oracleHostPackages` entry would fix this for every future lowpoly test case, but
  needs a NEW file at an ancestor path of `🗿️artifacts/💠️lowpoly` (this artifact's own root, or the
  plugin root) — outside the file ownership this pass was granted (`✳️any/🧪️oracle/🔣️.json` is a
  DESCENDANT of this case's owner, the wrong direction for `oracleHostPackagesFor`'s prefix match) —
  recorded as a handoff item rather than added unilaterally.

  What IS reachable without any additional crate: `app_commands!` also emits `LowpolyCommand`'s
  `TOOL_JOB_IDS` constant and `command_id()` as plain INHERENT items (no trait import needed), and
  every payload struct is a public, directly constructible type. This case therefore asserts the
  narrower, still-real claim that survives: the representative payload for each group constructs with
  the documented example shape, and the command it produces reports the exact manifest id
  `📝️editor-commands.md` (this ticket's own inventory) and the crate's own `every_command()` test both
  name — a real trip-wire against a payload field rename, a dropped variant, or the row moving to a
  different `$id`, even though it does not exercise `handle()` itself.

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

  @id-catalog-size
  @level-long
  @mode-conformance
  Scenario: The declared command catalog holds exactly the 47 commands the macro's own rows declare
    Given nothing beyond the linked `semio-s-plugin-lowpoly` crate
    Then LowpolyCommand::TOOL_JOB_IDS has exactly 47 entries
    And every entry is unique
