# Framework Module Sibling Repair

Five choices were made after reading the module implementations. This is an explicit manual rename list, not an automatic naming rule.

| Previous | Handpicked | Inspected responsibility |
| --- | --- | --- |
| `📐️intrinsic-size` | `📏️intrinsic-size` | Reads intrinsic image dimensions without decoding pixels; distinct from geometry. |
| `🔢️hash` | `🔏️hash` | SHA-256, BLAKE3, and Merkle content-integrity algorithms; distinct from numbers. |
| `🔺️mesh-engine` | `🏗️mesh-engine` | Constructs mesh primitives and implements mesh interchange codecs; distinct from mesh topology. |
| `🖼️pixels` | `🔲️pixels` | Pixel buffers and raster codecs; distinct from asset ownership. |
| `🧮️action-argument-resolution` | `🧩️action-argument-resolution` | Assembles staged, seeded, and default action arguments; distinct from mathematics. |

Status: five root moves and exact incoming Cargo/TypeScript/doc references complete. Other hash modules, including UI retained-state hashing, are outside these five moves. Concurrent Cargo checks were notified before the path changes. No code behavior, semantic IDs, Git state, or historical purity evidence was restored or overwritten.

The five mesh specimens now have separate meaningful identities: `🔢️external-buffer.bin` (numeric buffer), `🔗️external-buffer.gltf` (external-link specimen), `✅️expected-single-triangle.json` (expected result), `🔺️single-triangle-embedded.gltf` (triangle document), and `🧊️single-triangle-embedded.glb` (binary scene container). Source includes were changed exactly, and the external-buffer document references its renamed binary. Binary SHA-256 digests are unchanged: `da6a558d6b6af65846b717b81d06857d17a419ab4ba29b78e34dd7326cf0f1d6` and `f8fb1a9687cdd0ec0550c568553e4e4d9724d566dc00875c6a69142183518fb9`.

The complete owned-patch wrapper was separately reviewed and renamed from `🪄️whole` to `📦️whole`, matching its intact patch ownership and handback semantics. Its two Rust module references, two fixture-reader paths, neutral role fixture, and diagnostic fixture were updated. Page/page-list vocabulary now matches existing `📃️page`/`📃️pages` source owners instead of generic `📄️` registry values.

The framework TypeScript quick suite passed all 88 tests after the root moves. All four renamed Rust package targets exist. Dedicated core-module native tests passed all 66 tests across four binaries (zero skipped), including the existing first-party/third-party codec and hash oracles. Execution used `@semio-tech/framework-rs:test-core-modules`, registered in the existing command router and launch configuration, with isolated ticket-local build and nextest artifacts.
