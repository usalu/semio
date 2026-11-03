@capability-unit-vocabulary
@oracle-python-exact-rational-unit-algebra
@comparison-float-relative-1e-12
Feature: Units convert, parse and format like the independent oracle
  Executed by `🔬️unit/🦀️.rs`, which spawns `🔮️oracles/🐍️.py`. The oracle derives every factor from SI atoms and prefixes with exact rationals and never reads the Rust unit table.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Every unit equals its oracle derivation: Given every unit symbol of the vocabulary When the oracle derives factor, offset and SI dimension from the symbol Then factor and offset equal the table within 1e-12 relative And all units of one family share one oracle dimension
  - Conversion between compatible units: Given a value 123.456 in every compatible unit pair When the value is converted in the table and by the oracle Then both results agree within 1e-12 relative And converting across families is refused with an incompatibility error
  - Parse "<text>" with fallback unit metre: When the text is parsed with decimal comma or point, grouping spaces and optional unit Then the value and the unit factor equal the oracle's reading Examples (6 rows: 12-5-kn, 1-250-5mm, 1-250-mm, 3, 2e3-n, 7-c)
  - Format <value> with <precision> decimals for <locale>: When the number is formatted Then it equals the oracle text with locale decimal mark and thin-space grouping from five integer digits Examples (3 rows: 12345-678-2-de, 987654-321-3-en, 0-001-0-en)
  - English and German unit names resolve: Given the names "kilonewton" and "Kilonewton" When they are looked up Then both resolve to the unit with symbol "kN"
