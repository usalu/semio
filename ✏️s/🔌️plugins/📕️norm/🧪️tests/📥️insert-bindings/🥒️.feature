Feature: Schema-owned native insert payload bindings
  Scenario: Every cohort binds insertion payload members declared by its native leaf schema
    Given the committed census of 236 insert rules across fifteen norm cohorts
    When each literal index, item and selector binding is compared with its native payload schema
    Then every emitted member exists and every required payload member is supplied
    And an independent strict Ajv membership oracle agrees
    And root mutation discriminators remain the aggregate decoder's responsibility

  Scenario: Rust field names do not substitute for camelCase wire names
    Given the EN1998 tie beam and seismic slope insertion payloads
    Then their item bindings are tieBeam and seismicSlope
    And unknown snake_case payload members are refused by the schema oracle

  Scenario: An insert into an empty collection starts from the schema-derived default row
    Given a norm document whose collection has no row to copy
    When the insert tool places the first row of that collection
    Then the row is the default of the item schema of the collection's insert mutation: its required fields, schema defaults and first enum options
    And the row carries the fresh id and the snapped position like every copied row
    And the row resolves through the family's edit rules and is admitted by the document
    And an independent TypeScript derivation from the 236 payload schemas alone yields a row each strict item schema accepts
    And a collection without an insert mutation schema is refused with a named error
