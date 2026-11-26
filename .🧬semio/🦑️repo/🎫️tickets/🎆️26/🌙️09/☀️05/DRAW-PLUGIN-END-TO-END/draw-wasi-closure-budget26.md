# Draw WASI Closure Memory Audit

The most recent retained prototype Draw component plan is `.cache/b/r/m/components/runs/wL66blt/l/unit-graph-prototype-00.json`. Its single root is the `semio-hub-draw-component` cdylib on `wasm32-wasip2`. The root reaches all 923 units, including 698 target units and 31 Draw-owned target library or cdylib owners. The attempt recorded 868 compiler rows, with 30 successful Draw target libraries. Its final link never ran. The last admitted completed final was in `wP6yy5m` on November 25 at 21:08, with a 1,290.62 MiB final linker observation. Both plans predate the member wire and Hub role carves.

The 1,536 MiB library ceiling equals 1,610,612,736 bytes. The `wL66blt/r/units.json` successful local target observations show exactly four oversize libraries in that attempt.

| Owner | Peak bytes | MiB | Observation |
| --- | ---: | ---: | --- |
| `semio-s-artifact-stdio-txt-tests` | 1,897,136,128 | 1,809.25 | Shared STDIO, 02:26 |
| `semio-s-artifact-stdio-binary-io-registry` | 1,789,937,968 | 1,707.02 | Shared STDIO, 02:21 |
| `semio-hub-draw` | 1,694,845,328 | 1,616.33 | Draw composition before its 02:55 role carve |
| `semio-s-artifact-draw-drawing-editor-member-wire` | 1,691,175,168 | 1,612.83 | Draw editor member wire before its decoder carve |

The history ledger `.cache/b/r/m/components/build-telemetry/units.jsonl` preserves older peaks of 2,690.32 MiB for Draw editor store preparation and 1,905.78 MiB for the Draw model. Their latest managed target measurements in the retained attempt were 454.77 MiB and 1,303.62 MiB. The next-largest Draw library observations were the editor at 1,072.62 MiB, editor tool at 996.30 MiB and viewer at 973.11 MiB. The separate measured post-carve WASI decoder peak is 1,527.64 MiB and the thin wire peak is 424.23 MiB. Neither exceeds the limit. The decoder’s measured headroom is 8.36 MiB.

The current manifests route Draw through both new Hub role owners and route member wire through the new decode crate. No retained target graph, compiler report or local target observations for the new Hub role libraries or carved composition had landed at the time of this audit. The available evidence identifies no additional Draw-owned oversize library. A complete fresh current-closure result still requires that build and its actual unit measurements.

## Shared STDIO Owners

The retained dependency path to the text library is Draw component → Draw composition → STDIO JSON → STDIO TXT → STDIO TXT tests. The binary path adds STDIO Binary below the text artifact. Their original recent compiler dep-info recorded 11 source hashes for TXT and eight for Binary. I compared all 19 live files to those BLAKE3 checks and every one matched. This ties the observed owner sources to current file bytes. It does not attest that the full current dependency closure or future compiler peak is identical.

The TXT crate owns the production `TxtApps` closed dispatch, mounts and exposes the live editor and viewer, and provides artifact definition, assembly, declaration, contribution, native codec and IO registry reexports to the public TXT facade. Formula and fixture tests are independently gated by `cfg(test)` and dev dependencies. Its package name describes its original physical carve. Removing the entire dependency would remove production behavior. Binary owns the corresponding `BinaryApps` dispatch, assembly and contribution.

The actual retained Wasm archives contain 27,817 named functions for TXT and 37,402 for Binary. The explicit viewer roles account for 4,937 and 4,915 functions and about 2.78 and 2.79 MB of code bodies. The editor roles account for 5,022 and 4,245 functions and about 2.79 and 2.59 MB. These symbol families overlap with shared history and store machinery. The IO-related text appears in only 41 and 56 symbols. The largest bodies are retained history folding, history decode and lifted store control, not the registration table.

Binary already has an independent owner of the actual editor implementation, `semio-s-artifact-stdio-binary-mutations`, which measured 638.66 MiB in the retained attempt. The two-source carve moves the closed editor dispatch to that owner. Its parent registry preserves the raw typed editor conversion, viewer variant, surface constructors and IO declarations. No package, lock or registry changes were required. The neutral role fixture and independent SQLite role-table comparison reproduced the unsplit editor then passed after the carve. Both resulting library peaks still need a fresh managed target measurement.

TXT mounts the actual editor and viewer in its current oversized library. Its editor also requires the standard's artifact kind, examples and edit rules. There is no independently registered concrete editor or viewer mount to receive that closed dispatch. The nearby mutation package owns mutations and has no standalone view role. Moving the mounted viewer there would obscure ownership. A real viewer-role package would share the currently mounted source, define the single closed runtime viewer dispatch and reexport through the existing facade. The current TXT/Binary owners are present in 113 checked-in Cargo lockfiles, so a package split must include every affected resolved lock closure.

## Snapshot-only Draw Dependency Paths

The retained Draw graph has 23 incoming target edges to the full TXT artifact. The owner families are JSON, XML and DXF. The actual compiler dep-info under the successful units confirms where their two converter sources were compiled.

| Family | Full TXT dependencies | Actual converter compiler owners |
| --- | ---: | --- |
| JSON | 7 | `semio-s-artifact-stdio-json-schema` (GeoJSON schema) mounts both shared Base converters |
| XML | 9 | `semio-s-artifact-stdio-xml-schema` (Base schema) mounts import, `semio-s-artifact-stdio-xml-mutations` (Base mutations) mounts export |
| DXF | 7 | `semio-s-artifact-stdio-dxf-inferences` (R12 header inferences) mounts both converters |

Those six converter files import only `TxtSnapshot`. The TXT facade reexports `schema::snapshot::TxtSnapshot` and obtains the snapshot module from the actual TXT schema crate. The schema crate also exposes it at its root. Replacing these four dependency consumers with the schema crate keeps the identical Rust type. The other 19 owners have no source reference to the full TXT crate, including their separately mounted test source. The JSON changes have an independently delegated owner.

The projected graph retains XML through SVG and its genuine XML model, snapshot, codec and SQLite uses. JSON-only dependency cleanup preserves all 923 original units because Draw still reaches TXT independently through SVG → XML and through DXF. A projection that narrows all three families removes the full TXT facade and production app owner. This projection predicts reachability but does not serve as a new compiler graph.

Draw's export library has a direct normal dependency on the full DEFLATE artifact. I inspected all 11 source files recorded by its admitted compiler and found no reference to DEFLATE, zlib or flate2. The edge persists in four lockfiles. Dropping it would still leave Draw export → SEMIO drawing export → PDF conformance support → DEFLATE. The full DEFLATE artifact declares seven normal Binary dependencies. Its `semio-s-artifact-stdio-deflate-tests` owner compiles both real converters and requires only `BinarySnapshot` and `STDIO_BINARY_DOCUMENT_SCHEMA`. Both are the unchanged public types and data exposed directly by `semio-s-artifact-stdio-binary-snapshot`. The six other DEFLATE owners mount neither converter. Narrowing DEFLATE would still leave Binary reachable through paths outside DEFLATE, including TXT schema.

The XML and DXF manifests now declare the snapshot dependency only in the three real converter owners. Their four converter imports select the same snapshot type directly. The 13 redundant XML and DXF dependency edges and the one dead Draw export dependency have been removed. The neutral graph law checks actual Rust mounts against their owning Cargo manifests and independently compares dependency edges in SQLite. It passed alongside the Binary role law. The shared lockfile synchronization includes these changes and the separately owned JSON edits. No Rust build ran during this audit.
