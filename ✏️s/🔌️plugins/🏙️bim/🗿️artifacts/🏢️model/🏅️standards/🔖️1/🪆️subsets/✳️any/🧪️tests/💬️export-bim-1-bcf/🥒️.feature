@capability-bim-1-export-bcf
@oracle-bim-1-bcf-xml
@comparison-floating-point-v1
Feature: Exchange coordination issues with BCF 2.1

  @id-export-bcf-frame
  @level-quick
  @mode-differential
  Scenario: Preserve topics, comments, selection and viewpoint geometry
    Given the annotated frame snapshot shared://🚪️bcf/🏢️frame/model.json
    And its exported BCF 2.1 archive shared://🚪️bcf/🏢️frame/issues.bcf
    When the archive is opened and its topic and viewpoint references are followed
    Then the topic metadata, comments, selected IFC products, camera and clipping box agree with the authored issues within 1e-9
