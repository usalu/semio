# Current Worker Native Provenance

ALL11 reported a Source conflict in `🧰️framework/🔨️modules/🧵️job/👷️worker/📦️packages/🦀️rust/Cargo.toml` and later reached its original deadline without a pin. Root requested a bounded current-declaration check only; it did not authorize a repair, another producer or an additional profile.

The existing retained pure TOML traversal was reused with a fixed `--worker` owner selector. Actual current results, terminal exit 0, are:

| Mode | Local packages | Local path edges | Missing manifests |
| --- | ---: | ---: | ---: |
| Normal | 35 | 115 | 0 |
| Development dependencies at the Worker test root | 37 | 122 | 0 |

Every one of the 38 consumed Source files and the registry bytes remained unchanged across the traversal. The real package is `semio-framework-job-worker`; its canonical registry owner is `@semio-tech/framework-job-worker-rs`. Its physical root is an explicit member of the actual framework Cargo workspace. Optional path declarations, target tables, normal/build dependencies and root-only development dependencies are included using actual `@iarna/toml` parsing and Node path resolution. The original 60-second control and finite 256-manifest/2,000-edge bounds are unchanged.

Worker Cargo is 930 bytes, SHA `18a4f6f26459f71dfea79667ba4bae5956fd9141129d4d9e1baae0caa304ce2c`, with current mtime/ctime `2026-10-15T03:28:01.648Z`. Current stable registry SHA is `794997754841130b559d858687689680e32dbc5cbb612a9df454327cd4864e9b`, different from ALL11's launch registry. These current facts do not reconstruct prior consumed bytes or identify who changed them. No missing declaration or residual retired UI Budget edge was found in this targeted current closure.

No production Source, Cargo declaration, graph API, cache, task or policy was changed. The retained diagnostic input is `native-provenance/📜️script.ts`; generated results are `🗑️generated/all11-worker-local-closure.json` and `.log`. This proof is read-only current local closure/membership/registry evidence, not compilation, graph acceptance or permission to bypass the original Source guard.
