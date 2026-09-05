@capability-gis-gisterrain-1-config-mutate
@no-oracle-gis-gisterrain-config-mutation-semantics
@comparison-ordered-json-v1
@mutations-gis-gisterrain-1-config
Feature: Apply every gis3d editor-config mutation to a real config record
  `Gis3dConfig` (`../../✏️editor/🎚️config/🦀️.rs`) is gis3d's own session-only editor view-state — a free/live
  viewport camera (`cameraJson`) and a UI locale tag, explicitly never part of the document's undo
  history. The `gis-gisterrain-1-config` catalog (`../../✏️editor/🎚️config/🔮️oracle/🔣️.json`) declares the 2 kinds
  this facet owns: `set-camera`, `set-locale`. No third party implements this repository's own
  ephemeral editor state and none could adjudicate it (recorded as the
  `gis-gisterrain-config-mutation-semantics` no-oracle decision, same shape as gisterrain's own
  sibling `os-config-opening-preferences-mutation-semantics` precedent,
  `../../../../../../../../🧰️framework/🛍️products/💻️os/🎚️config/🧪️oracle/🔣️.json`), so this
  case runs the subject role only.

  `gis3d_config_mutation_report_json` (`../../✏️editor/🎚️config/🦀️.rs`) is the whole surface this case needs: every
  field of `Gis3dConfig` is a plain `String`, so the bridge never needs
  `serde_json::from_str::<Gis3dConfig>` (unreachable from a `sut`-feature adapter crate — see that
  function's own doc comment) and reaches the REAL, unconditional `Mutation<Gis3dConfig>`/
  `MutationDiff<Gis3dConfig>` trait chain `#[derive(dsl::Mutations)]`/`#[derive(dsl::MutationLeaf)]`
  already generate, the identical chain this facet's own `#[cfg(test)] mod tests`
  (`../../✏️editor/🎚️config/🦀️.rs`) already exercises directly. Both `set-camera` and `set-locale` are a WHOLE-FIELD
  replace that leaves the sibling field untouched — `sparse_camera_inverse_and_codecs_preserve_locale`/
  `sparse_locale_inverse_and_codecs_preserve_camera` in that same file already assert this at the
  leaf level; this case re-asserts it end to end through the report bridge.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to a real gis3d config record
    Given a real gis3d config record with cameraJson <baseCamera> and locale <baseLocale>
    When <id> is applied with value <value>
      """
      {"kind": "<id>", "baseCameraJson": <baseCamera>, "baseLocale": <baseLocale>, "value": <value>}
      """
    Then the resulting record differs from the base record and the sibling field is unchanged
    Examples:
      | id         | baseCamera                      | baseLocale | value                            |
      | set-camera | "{\"position\":[0,0,0]}"        | "en-US"    | "{\"position\":[9,9,9]}"         |
      | set-locale | "{\"position\":[0,0,0]}"        | "en-US"    | "de-DE"                          |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real gis3d config record
    Given a real gis3d config record with cameraJson <baseCamera> and locale <baseLocale>
    When <id> is applied and then its own computed inverse is applied
      """
      {"kind": "<id>", "baseCameraJson": <baseCamera>, "baseLocale": <baseLocale>, "value": <value>}
      """
    Then the record equals the base record again, field for field
    Examples:
      | id         | baseCamera                      | baseLocale | value                            |
      | set-camera | "{\"position\":[0,0,0]}"        | "en-US"    | "{\"position\":[9,9,9]}"         |
      | set-locale | "{\"position\":[0,0,0]}"        | "en-US"    | "de-DE"                          |
