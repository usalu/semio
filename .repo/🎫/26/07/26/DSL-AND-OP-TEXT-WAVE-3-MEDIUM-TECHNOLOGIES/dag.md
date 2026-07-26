# infinite/board/port/directed/dag — DSL + OpText notes

## Design
- Reused `mathematical_graph_dsl::wire` (`WireNode`/`WireEdge`, `wire_literal_from_dag`,
  `dag_from_wire_literal`) UNCHANGED. `DagNodeSpec`/`DagFixtureEdge` carry far more than
  id/kind/port/properties, so every extra field (layout, kind-specific payload, the node's own
  free-form `properties`) is folded into the wire node/edge's `PropertyValue` property bag — the
  extension point the wire module already exposes.
- Wire lexer has ZERO escaping (`'...'` strings just scan to the next literal `'` byte; no `\n`
  handling). Added `wire_safe_string`/`wire_unsafe_string` (percent-encoding of `'`, `%`, `\n`, `\r`)
  applied to every string leaf so arbitrary text (multi-line note bodies, data URIs with embedded
  quotes like the demo screen's SVG `src`) survives.
- OpText's one-line law is met by a uniform outer envelope: `opName key=value key=value ...` with
  plain whitespace splitting; any structured/free-text value (a whole embedded wire node/edge line, a
  whole document body, a note's text) is percent-token-encoded (`token_safe`/`token_unsafe`, escaping
  space/tab/newline/`%` too) into ONE token first. So the outer grammar never needs to understand
  nesting, and no raw `\n` can ever reach the printed line.
- `DagNodePatch.kind: Option<DagNodeKind>` (a full kind replacement) is embedded via a throwaway
  `_`-id synthetic `WireNode` carrying just the kind-specific property bag (`kind_to_token`/
  `kind_from_token`), token_safe'd like everything else structured.
- Known caveat (documented in `json_to_property`'s docstring): `serde_json::Number` distinguishes
  integer vs float tags; round-tripping an untyped JSON leaf (`IoPortSpec.default`/`.value`,
  `DagPreviewContent::Tree.json`) through `PropertyValue::Number(f64)` re-tags an integer literal as
  a float. No current fixture/test stores raw JSON literals in those leaves, so this is inert today.

## Files touched
- `infinite/board/port/directed/dag/rs/lib.rs`: added `//#region 🔖Dsl` (DocumentDsl for DagDocument)
  and `//#region 🔖OpText` (OpText for DagOperation), both inside the existing
  `// #region 🔖DocumentVcs` region, right before `//#region 🔖WasmBridge`. Extended
  `#[cfg(test)] mod dag_vcs_tests` (inside the same DocumentVcs region) with a `//#region 🔖DslTests`
  subregion. Retargeted `impl Default for DagFixture` to `DagDocument::parse_dsl(include_str!(
  "../example/demo.dag"))` instead of `serde_json::from_str(include_str!("../example/demo.dag.json"))`.
- `infinite/board/port/directed/dag/example/demo.dag`: new handcrafted text fixture, replacing
  `demo.dag.json` (deleted).

## Verification
See final report for exact test counts / wasm check result.
