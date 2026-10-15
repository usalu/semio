# ALL10 Native Declaration Provenance

ALL10 completed its first project assembly pass, then revalidated after a Source conflict in the installed Nx patch. Round 1 reported a Job Cargo manifest conflict and a missing native dependency at the old UI Budget path. The original deadline expired without a graph pin. This investigation reads current declarations only; it does not reconstruct unretained historical bytes or identify another writer.

The exact missing path remains absent: `🧰️framework/🔨️modules/🖱️ui/📏️budget/📦️packages/🦀️rust/Cargo.toml`. The current canonical budget package is instead under `🖱️ui/🧬️contract/📏️budget/📦️packages/🦀️rust`: package `semio-framework-ui-contract-budget`, declared owner `@semio-tech/ui-contract-budget-rs`, physical project manifest and matching framework workspace member. Its actual Cargo SHA is `6446a7c200e08ef3afb951b9f704ca527e359badb5e02cd650fc5c215936c131`.

The current direct Cargo declarations are coherent. Framework workspace member line 117 names the canonical contract-budget root. The contract crate's path dependency `../../📏️budget/📦️packages/🦀️rust` and contract-text crate's `../../../📏️budget/📦️packages/🦀️rust` independently resolve to that same existing manifest. A bounded framework Cargo reference search found no current reference to the retired direct UI Budget location.

Current Job Cargo is package `semio-framework-job`, owner `@semio-tech/framework-job-rs`, SHA `b2c32d05758fcfc556ae80055298f0075c10e7075f35aa8e4dc7220a6e308c96`. It has no direct UI Budget edge. A targeted independent `@iarna/toml` traversal then resolved normal, build and target-specific path dependencies with actual workspace inheritance, including optional local declarations. It separately included development dependencies only at the selected Job test root, matching the production native closure contract.

Actual bounded results were:

| Root mode | Local packages | Local edges | Missing manifests | Retired UI Budget edges |
| --- | ---: | ---: | ---: | ---: |
| Normal | 34 | 111 | 0 | 0 |
| Root tests | 34 | 112 | 0 | 0 |

All 35 consumed Source files, including workspace manifests, were rehashed and restamped unchanged before the traversal returned. The original Source-only diagnostic had a 60-second control, 256-manifest/2,000-edge bounds and awaited progress. It exited 0; no Cargo command, compiler, FactStore, cache writer, graph producer or Source mutation ran. The broader Job/UI/workspace/registry pre/post byte snapshots also remained unchanged during this investigation.

ALL10's log lifecycle was 03:01:30.871–03:21:31.392 UTC. Current Job Cargo is stamped 03:17:14.571; current framework workspace is stamped 03:20:37.193; canonical budget declarations and registry are stamped around 03:21:03. These are observations of current physical metadata, not a complete history or proof of exactly when semantic changes occurred. The actual current registry differs from the recorded launch registry. The current coherent declarations therefore do not establish what dependency path an earlier observation consumed.

There is no proven stable residual path defect in the targeted current Job closure. The evidence supports retaining strict Source guards and preserving current concurrent authoring, rather than inventing the missing old package or changing dependencies. A future producer must consume a new coherent current Source image under its original controls; this report alone accepts neither a full graph nor a retry.

Retained input is `native-provenance/📜️script.ts`. Generated evidence is `all10-native-pre.json`, `all10-native-post.json`, `all10-job-local-closure.json` and `.log` under ticket generated storage. Those files retain selected identities, edges and hashes, not copied Source files.
