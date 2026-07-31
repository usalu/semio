# Work summary

- Added `BoardKindCatalogBundle` (handles, wires, nodes, edges) serialized to WASM via `setBoardKindCatalogsJson`; handle entries support `label`/`name`, `color`, `defaultWireKind`.
- Replaced handle-only compat list with `BoardKindCompatEntry` (`bidirectional`, `important`, `specificity`: general | node | edge | handle | wire). Resolution: if any matching rule is `important`, only important matches count; else only rules at the highest matching specificity rank count. Empty list = unrestricted.
- Link drag / commit uses combined rules: wire rows compare resolved default wire kind from source handle catalog to target handle kind; edge rows use default edge kinds implied from each side’s wire catalog; node/general/handle as documented in Rust.
- Scene descriptor + fixtures carry optional `nodeKind`, `edgeKind`, `wireKind`; `Node`/`Edge`/`Wire` retain corresponding fields; default wire kind fallback `board.wire.link`.
- `BoardCanvas` props: `kindCatalogs`, `kindCompatibility`; defaults `BOARD_DEFAULT_KIND_CATALOG_BUNDLE`.
- Rust tests for wire-only allow list and important override.
