@capability-norm-typed-inspector
@oracle-norm-inspector-python-exact-arithmetic
Feature: The norm inspector edits typed, unit-aware fields with live utilisation feedback
  Every norm app edits its document through one typed inspector and one entity outliner. The document stays in SI;
  conversion happens at the control boundary. `🔮️oracles/🐍️.py` recomputes the unit text with exact rational arithmetic
  from the standard library and shares `🔮️oracles/🔣️.json` with the Rust unit test.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - A length is shown in the preset's natural unit and committed in SI: Given a member with "length" = 0.0015 m When the inspector renders with the engineering preset Then the field is labelled "Length (mm)" in English and "Länge (mm)" in German And the control displays 1.5 and scales a typed value back by 1/1000 before "setField"
  - Typed text is parsed per locale: Given the German locale When the user types "1,5 mm" into a length shown in millimetres Then the committed SI value is 0.0015 And "1.234,5" parses as 1234.5 while "1.234" parses as 1234
  - A failing check is visible at the field it concerns: Given a failing check "b1.bending" with utilisation 1.12 whose subject path is "members[id=b1].length" When the inspector renders Then the length row carries a utilisation bar reading "✗ 112 %" And a remedy chip "22.0 mm" dispatching "applyRemedy" for check "b1.bending" remedy 0
  - Values outside the norm's validity range are refused: Given a length with the hard range 1 mm to 500 mm When 2 m is typed Then the refusal reads "Out of range, allowed: 1 mm … 500 mm" and nothing is dispatched
  - The outliner lists entities and edits the collection: Given collections "sections" and "members" Then each member is a row "id — name" with its worst utilisation and a remove action And the collection offers "insertItem" at its end And typing "beam" in the filter leaves only matching rows
  - No JSON is shown: When any document renders Then no visible text is a serialized object or array
  - The field specification drives the control: Given a length with the soft range 1 mm to 100 mm, the step 1 mm, the help "Span between the supports" and a clause When the inspector renders in German with the engineering system Then the row offers a slider over the soft range, a number input with the hard range and the German help text And a clause row reads "EN 1993-1-1 6 Abschn. 6.2.5" And an angle stored in radians is shown in degrees and typed "90" is committed as pi/2
  - Annex parameters show their symbol and reset to the annex value: Given a field with the national-annex parameter "γ_M0" whose annex value differs from the document value When the inspector renders Then the row label carries "[NDP γ_M0]" and a reset action dispatches "setField" with the annex value
  - Predicates and disclosure tiers hide fields: Given the field "columnWidth" visible only when "kind" equals "column" and an advanced field When the member is a beam shown at the basic tier Then neither field is listed and the document still contains both
