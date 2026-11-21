# DSL Record Model Source Boundary

The current production closure of `semio-framework-dsl-record-model` fits the manifest boundary on both the native Apple Silicon host and `wasm32-wasip2`.

| Target | Source files / 32 | Source bytes / 262,144 | Active lines / 4,000 |
| --- | ---: | ---: | ---: |
| Native host | 26 | 261,849 | 2,332 |
| `wasm32-wasip2` | 26 | 261,849 | 2,332 |

The ticket's earlier scan measured 266,109 bytes for each target. The shared worktree now loads the receiving projection tests from a sibling module marked `#[cfg(test)]`. The parent file fell from 10,224 to 5,964 bytes. The sibling holds the three existing laws for the neutral wire fixture, nested allocation and receipt, and refusal of incomplete allocation requests. The production parent SHA-256 at inspection was `896637a492056856aa9ca01db10699444b1e75fe73c6f62ff17f84ff8f2eadf2`. There is 295 bytes of production headroom at this snapshot.

The retained `dsl-model-measure.ts` probe replayed all 28 retained `roster-graph-*.json` documents against live files using the production `readRustSourceClosureV1` scanner and the pinned compiler's target cfg. It found two distinct production closures and zero read refusals. The limits above come from the current package manifest. The source change was already present when verification began.

The separate native Rust law command reached repository source admission and stopped on `Unknown entry exceeds exact Native creator inventory` (reference `6e13c06d`) before Cargo could execute the tests. A direct run of the adjacent TypeScript admission and storage-recheck suites passed eight cases, including the tighter-limit and physical-byte-refusal cases. Two cases failed outside the measured DSL closure. One direct launch lacked the parent script invocation, and the recheck fixture expected 16 redundant reads for three identical compiler units. The broader source scanner suite could not load `ajv` in the direct launch.
