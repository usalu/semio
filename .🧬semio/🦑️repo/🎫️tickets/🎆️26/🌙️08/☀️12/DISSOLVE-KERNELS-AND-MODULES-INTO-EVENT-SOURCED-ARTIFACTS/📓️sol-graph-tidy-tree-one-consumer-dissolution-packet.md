# Graph Tidy-Tree One-Consumer Dissolution Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`
- Framework graph drawing SHA-256: `39ebcf6e71018dc28386886b31c3d1eba9d3dee4b9d454ef28712e40b9286b1b`
- OS DAG component SHA-256: `955e7f921feb9091e09060ef021ef1b99d8b791889135197a25736ae87c41bff`
- Both paths are clean.

## Consumer Evidence

`graph::drawing::tidy_tree::buchheim_positions` has exactly one production consumer: the OS directed DAG board component. Its only other references are its own definition and same-region test. The test does not qualify as an independent production consumer.

The adjacent graph-drawing responsibilities are separate and remain untouched: force layout has independent OS undirected-board and Trinity command consumers, while routing remains mounted by the OS board facade.

## Lease

Move the complete `tidy_tree` implementation and its useful contract test into a private subregion of the OS DAG component's existing Layout region. Remove the public framework import and the now-empty TidyTree region from graph drawing. Do not change force, routing, package glue, manifests, Cargo files, generated files, or any other path.

Writable paths:

- `🧰️framework/🔨️modules/🕸️graph/🖊️drawing/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`

Validation:

```text
bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache
bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache
```

Acceptance additionally requires zero active references to `drawing::tidy_tree` or a public `tidy_tree` module, a retained DAG layout test, current final hashes, ordinary and cached diff checks, and preservation of external index state.
