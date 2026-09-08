@capability-gis-gisterrain-1-config-mutate
@no-oracle-gis-gisterrain-config-mutation-semantics
@comparison-ordered-json-v1
@mutations-gis-gisterrain-1-config
Feature: Apply the gis3d editor camera mutation to a real config record
  Gis3dConfig owns the session-only viewport camera. The host-owned locale is supplied through
  ViewModel and is outside this mutation vocabulary.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply set-camera to a real gis3d config record
    Given a real gis3d config record with cameraJson <baseCamera>
    When set-camera is applied with value <value>
      """
      {"kind":"set-camera","baseCameraJson":<baseCamera>,"value":<value>}
      """
    Then the resulting record differs from the base record
    Examples:
      | baseCamera               | value                      |
      | "{\"position\":[0,0,0]}" | "{\"position\":[9,9,9]}" |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing set-camera restores the real gis3d config record
    Given a real gis3d config record with cameraJson <baseCamera>
    When set-camera is applied and then its own computed inverse is applied
      """
      {"kind":"set-camera","baseCameraJson":<baseCamera>,"value":<value>}
      """
    Then the record equals the base record again, field for field
    Examples:
      | baseCamera               | value                      |
      | "{\"position\":[0,0,0]}" | "{\"position\":[9,9,9]}" |
