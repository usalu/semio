@capability-bim-1-infer
@oracle-bim-1-jsonschema-properties
@comparison-floating-point-v1
Feature: Infer the properties every element really has from its own values, its type and the property set templates and audit it with jsonschema
  `s.bim.model@1` stores property set templates (a name, the element and type kinds they apply to and per property a value kind, a unit, an enumeration, a range,
  a required flag and a default) and typed property values on elements and on their types. `🏷️effective-properties` derives what a holder really has: the value it
  states itself wins, else the value its type states or gets by default, else the default of a template that applies to its kind; an own value that breaks its
  definition (kind, range, enumeration) and a required property without any value are findings, and an inherited value is reported once, at its type. The oracle
  is `🐍️.py` in this directory. It derives one JSON Schema per property definition, lets `jsonschema` (Draft 2020-12) decide every own value, restates the
  inheritance from the authored records alone and audits it. The committed expectation is written by that file, never by hand.

  @id-psets-walls-columns-spaces
  @level-quick
  @mode-differential
  Scenario: Walls, a wall type, a column with its type and spaces under four templates resolve to their effective values and findings
    Given the committed property model shared://💡️inferences/🏷️effective-properties/🏗️psets/📸️snapshot/🔣️.json
    When 🏷️effective-properties is inferred for it
    Then every holder's effective values and findings equal the table shared://💡️inferences/🏷️effective-properties/🏗️psets/💡️inference/🏷️effective-properties/🔣️.json
