# Native Trinity rewrite regressions

- The native inferences suite reported 22 passes and three failures. The structured merge case reached Jack's property validator with a typed object for Nakagin's `Piece.position`. Jack's string-based checks rejected that object because the declared `ValueType::Any` reports `value` as its diagnostic identifier. Jack now accepts every property value for an explicit `Any` declaration.
- The bundled Nakagin DSL used erased JSON field names and inline graph content from an earlier schema. It now uses the typed record fields and the existing Nakagin graph child address. The Nakagin round-trip and Pack equivalence test passed against a local copy of the previous test binary containing the updated demo text.
- The DSL reader parsed a record prefix. It now requires the complete record. The unknown-keyword test appends an unknown token to a valid document and checks that the diagnostic identifies that token.
