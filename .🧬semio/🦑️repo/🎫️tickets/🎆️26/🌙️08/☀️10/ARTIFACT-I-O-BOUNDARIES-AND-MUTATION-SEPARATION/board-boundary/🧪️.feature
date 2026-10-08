Feature: Board Owned Semantics and Explicit Palette IO
  Scenario: Hidden takes precedence over visible without interpreting serialization
    Given the shared visibility vectors
    Then pure visibility agrees with independent object semantics
  Scenario: Closed typed palette overlays
    Given all 48 RGBA fields and the default base
    When a typed overlay is applied
    Then unspecified fields equal the base and specified fields equal exact u8 tuples
  Scenario: Explicit physical JSON admission
    Given malformed, duplicate, unknown and invalid component vectors
    Then admission refuses them before publication
    And caller cancellation and allocation limits are observed
  Scenario: Owned userdata and snapshot records
    Given nested first-party values in all four element families
    Then public descriptors preserve them without serialized maps
