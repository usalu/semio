Feature: Pure Package Admission and Typed Flow Cache
  Artifact semantic roots admit typed package declarations.
  Registry cache ingestion accepts typed dictionaries.
  Physical JSON exists only in explicit IO modules.

  Scenario: Neutral package declarations agree with independent JSON schema admission
    Given canonical space and collection package declarations and invalid identity, version, dependency, and unknown-field cases
    When typed admission and explicit IO decode execute
    Then independent Ajv and serde_json agree with the first-party package output and refusal

  Scenario: Host output decoding preserves dictionaries before typed cache seeding
    Given nested dictionaries, decimals, a large integer, and invalid physical responses
    When explicit host response IO decodes and the registry seeds a typed cache
    Then the independent JSON reader sees the same values and refused responses do not seed the cache
