@capability-themes
@oracle-python-cie-wcag-machado
Feature: Themes are user-customisable bundles that stay accessible
  A theme is a plain file in the theme schema; built-ins ship in light, dark and high contrast.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Built-in themes: Given the built-in themes When they are validated and checked for accessibility Then they have bilingual names, valid ramp ids and no contrast findings
  - Round trip: Given a theme with a custom background and role colour When it is encoded and decoded through the value codec Then it is unchanged
  - Invalid bundles: Given a theme with a bad id, wrong ramp kinds, out-of-range scales and duplicate roles When it is validated Then each violated path is reported
  - Custom background: Given a light theme with a pale yellow background When accessibility findings are computed Then the utilisation ramp is reported as below the contrast floor
  - Twins: Given the Rust theme declarations Then the proto, TypeScript and GraphQL twins declare every type
  - Colour-blind safe theme ramps: Given the utilisation ramp of each built-in theme When protan, deutan and tritan vision are simulated Then the ends of the ramp differ by at least 15 in CIEDE2000
