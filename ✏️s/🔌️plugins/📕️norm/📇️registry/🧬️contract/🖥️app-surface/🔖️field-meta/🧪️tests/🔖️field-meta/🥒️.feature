@capability-field-spec
@oracle-python-exact-rational-unit-algebra
@comparison-float-relative-1e-12
Feature: Field specifications resolve by path template and convert at the display boundary
  Executed by `🔬️unit/🦀️.rs`; unit conversion factors are validated against the Python unit oracle of the compliance core.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Path-template lookup is unchanged: Given a table with "beams", "beams[].span" and "beams[].angle" When "beams[id=B1].span", "beams[2].unknown" and "columns[0].span" are resolved Then the exact template, the longest prefix and no match result respectively
  - Edits convert only at the field boundary: Given a length field stored in metres and shown in millimetres When the user enters "2500" or "2,5 m" Then the stored value is 2.5 And a value in newton is refused
  - Angles are stored in radians: Given an angle field When the user enters "90 °" Then the stored value is pi/2 and the display unit is the degree
  - Every artifact table keeps both languages: Given the ten Eurocode field tables When every entry label is read Then English and German labels are non-empty
