# DWG Snapshot Compiler Cache Misses

The six retained wrapper lookups for semio-s-artifact-stdio-dwg-snapshot-objects between 17:56 and 19:15 PST were distinct cache keys. All six agree on the toolchain, the DWG source digest, the input-root digest, the platform and every tracked environment field. Every transition changes at least one compiler extern input. The kernel changes in every transition. The wrapper normalizes output/search paths and substitutes the digest of actual compiler dependency bytes for the absolute extern path. Its metadata/codegen/environment components remain independently checked.

| Run | DWG cache key prefix | Framework schema extern | OS kernel extern | Cargo artifact suffix |
| --- | --- | --- | --- | --- |
| FEM wFz8qpa | b94baea1aa05 | 0228c710f65b1495 | fa47dd51e580c647 | 632ec61d599e60be |
| Demonstrator wKqQfVt | e77189ea6af4 | 911d0223e61f336b | 52455ae32a7b7867 | 9448b71e4825e244 |

The FEM values are the actual rmeta SHA-256 prefixes recorded in its sealed compiler provenance. The corresponding current Demonstrator files still exist in p73ac0a. Independent physical SHA-256 hashing produced exactly the two contrasting prefixes above. Cargo selected the same no-feature DWG library under release wasm32-wasip2 in both runs. Its compiler metadata argument differs as well. The successful FEM broker record measured 134.060 seconds and 4,160,913,408 bytes peak resident. The current Demonstrator exact-key cache entry points to readable CAS objects for its dep-info, rmeta, rlib and rmeta dependency sidecar and retains a 110.745-second compiler witness at 4,138,696,704 bytes peak resident.

The older FEM lane and key entry are absent at the audit. The local eviction journal preserves aggregate entry counts, so it cannot identify a specific key retirement. A different real extern digest already precluded reusing the FEM key even if its outputs had all remained intact. No same-key miss or unsafe path-dependent hash is shown by these records.

## Release feature selection

The actual selected wKqQfVt and w2wlSTL graphs enable protocol-laws and record-codec-laws on semio-os-kernel. The captured wFz8qpa FEM graph has neither. Scanning every incoming kernel dependency's manifest in the two affected graphs finds one non-dev request for protocol-laws. The glTF mutation fixture corpus package semio-s-artifact-stdio-gltf-mutations-mut-any-4 requests it to call the original mutation inverse sum law. The kernel enables record-codec-laws transitively through protocol-laws.

The glTF mutations root puts that fixture corpus and six split test packages in ordinary dependencies. Their only public root reference is the corpus re-export used by mutation unit tests. There are no production references to that re-export or to the split test packages. The narrow candidate moves all seven dependency declarations to the same root manifest's dev-dependencies and gates the corpus re-export with cfg(test). The fixture corpus and the six shard packages keep their own authored test code and independent workspace test targets. No compiler cache key or policy needs a change.

## Candidate verification before shared apply

The live pinned locked Demonstrator graph reproduces the leaked features. The live glTF workspace native test graph independently requires an already stale lock update before any candidate edit. The pinned Cargo has no alternate lockfile option, so a private scratch manifest projection reproduced all 50 authored member declarations, using newly authored empty stub targets for local metadata and real unchanged external inputs. The baseline and candidate each use a copy of the existing lock as input and refresh only their own scratch lock.

| Manifest selection | Release units | Native test units | Release fixture packages | Test fixture packages |
| --- | --- | --- | --- | --- |
| Scratch baseline | 735 | 739 | 7 | 7 |
| Scratch candidate | 728 | 739 | 0 | 7 |

The candidate release loses both kernel test-law features. The candidate native test graph and baseline graph retain identical kernel features. Both scratch lock refreshes are byte-identical, with SHA-256 052dbb492c5ea466236b431338b3015d546702800ccfe5c13fd55969149f1dcf and 24 added lines relative to the captured live lock. The new lines reflect existing framework dependency edits and the plugin window-config contract package. There are no registry version upgrades, removals or candidate-caused lock changes within the standalone glTF workspace. The dependency classification has a separate minimal native unit law and SQLite reachability oracle, both green. Evidence is generated gltf-graph/projection.log, gltf-graph/neutral-oracle.log and gltf-graph/tiny-native.log.

At the candidate snapshot, ten lockfiles named the corpus package. The two product files awaiting application were the root mutations package `Cargo.toml` and sibling Rust entry in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust`.

## Shared change and locked validation

During the next compiler boundary, the glTF mutations manifest moved all seven fixture dependencies to `[dev-dependencies]`. The library entry made the sole corpus re-export conditional on `#[cfg(test)]`. Pinned nightly Cargo reproduced the candidate results from the live standalone workspace with `--locked --offline --unit-graph`.

| Live selection | Units | Test-only packages | Kernel law features |
| --- | ---: | ---: | --- |
| glTF WASI release | 728 | 0 | None |
| glTF native library test | 739 | 7 | `protocol-laws`, `record-codec-laws` |
| Demonstrator WASI release | 1,469 | 0 | None |

An independent SQLite reachability query over the neutral fixture agreed with both Cargo selections. The retained minimal native law passed, and the live product verification used graph-only Cargo invocations. The pinned offline runs use a private ticket Cargo home seeded with registry metadata and archives from the recovery mount.

All nine external lockfiles dropped all seven test packages through pinned `cargo update --workspace --offline`. The standalone glTF lock retains the packages for its declared workspace members and test selection. The following refreshes also resolved existing path dependency drift for their current manifests. The fixture-sweep resolution removed an unreachable `json` entry, and no refresh upgraded any registry dependency.

| Workspace lock | Package additions | Package removals |
| --- | ---: | ---: |
| Hub | 0 | 7 |
| glTF | 1 | 0 |
| stdio semio | 0 | 7 |
| GIS Bridge | 123 | 10 |
| GIS Terrain | 0 | 7 |
| Procedural Bridge | 126 | 23 |
| Generation 3D | 0 | 7 |
| Process 3D | 0 | 7 |
| Dev Composition | 14 | 8 |
| Fixture Sweep | 571 | 17 |
