Feature: Schema-owned native insert payload bindings
  Scenario: Every cohort binds insertion payload members declared by its native leaf schema
    Given the committed census of 234 insert rules across fifteen norm cohorts
    When each literal index, item and selector binding is compared with its native payload schema
    Then every emitted member exists and every required payload member is supplied
    And an independent strict Ajv membership oracle agrees
    And root mutation discriminators remain the aggregate decoder's responsibility

  Scenario: Rust field names do not substitute for camelCase wire names
    Given the EN1998 tie beam and seismic slope insertion payloads
    Then their item bindings are tieBeam and seismicSlope
    And unknown snake_case payload members are refused by the schema oracle
