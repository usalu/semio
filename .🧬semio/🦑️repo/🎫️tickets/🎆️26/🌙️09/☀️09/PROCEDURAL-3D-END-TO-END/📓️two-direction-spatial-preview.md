# Spatial previews for inputs and outputs

Edit, Generate and View now publish geometry from both evaluated directions for preview-enabled graph nodes. An operator that returns only a scalar can still show its geometric input. A paired input and output get distinct stable world-instance identifiers, including when their authored channel names match. Input labels carry an incoming arrow. Instances referring to the same retained source share a mesh buffer.

Tessellation demand, mesh residency and preview work status follow the same directional walk. The export-only route preserves its named source-channel scope. Component editing remains available on output geometry. Input instance picks use the graph handle address, and a hovered incoming wire still highlights its visible geometric producer.

## Verification

- Two native tests passed against the actual Edit and View payload builders, directional traversal and mesh demand functions. They used test records at the Flow session and mesh-buffer boundaries and checked matching complete payloads, coincident buffer reuse, input and output addresses, exact-instance marking and component edit admission.
- Eight SQLite-backed tests passed for direction, export authority and prior geometry paths. Twelve focused hover regression tests passed.
- Rust syntax passed for the six affected source and test files. The final focused native rerun, expanded to cover incremental viewer rebuild and an export-only source, hit an evicted cached Rust dependency. The independent export scope check passed.
- The official repository Cargo runner completed admission for 882 source units and started compiling with one admitted core. Its registered preview-eval test is still in progress. Canonical mounted app verification remains open.

## Hover verification

The [directional graph port report](📓️directional-graph-port-hover.md) records the subsequent graph-address changes and current verification. Real coincident geometry still needs a mounted pick check.
