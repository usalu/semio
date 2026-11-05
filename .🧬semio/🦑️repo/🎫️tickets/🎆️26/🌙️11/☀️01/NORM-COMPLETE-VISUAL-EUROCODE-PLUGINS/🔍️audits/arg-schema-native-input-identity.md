# Arg Schema Authored Native Input Identity

This is a bounded read-only trace of the current authored metadata and the actual inference implementation. No fresh graph or native acceptance is claimed.

The exact package root is `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📦️packages/🦀️rust`. Its existing authored project identity is `@semio-tech/manifest-arg-schema-rs`; Cargo's package name is semio-framework-manifest-arg-schema.

The actual project-node inference reads authored json.name and the metadata dirname at project-nodes/🟨️.mjs182–195. It builds declaredRoots from those exact roots at198, and skips Cargo fallback when the root is already declared at205. Consequently this package cannot silently receive a parallel Cargo-named inferred owner. The root-to-owner map is then constructed from the actual projects at222.

inferProjectInputs receives the actual project json and root (inputs/🟨️.mjs263–267), reads declarations from json.namedInputs at312, and merges declarations.nativeSources into both effective nativeSources and nativeTestSources at322. The native test target selects nativeTestSources at project-assembly/🟨️.mjs69–70. Native dependency owner selection also uses that exact project root map, rather than requiring Cargo package-name equality, at native-manifests/🟨️.mjs282–288.

The explicit runtime corpus lies outside arg-schema's own test directory; the native production-group test exclusions cannot accidentally remove that sibling fixture path. The status/reference fixture files are outside the test-directory exclusion too. The actual nativeTestSources group has no production test-directory exclusion.

Current declared, physically read input assets:

| Input | SHA256 | Bytes |
| --- | --- | ---: |
| 🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🧫️mutation-inputs/🔣️.json | 4770240765abd3611ec91a3ea2c1f8a123ef7b8c01294ee7b3a9326140ffa420 | 99809 |
| 🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/status-counter-cases.json | bfa3d896478a38a6cf5047d024b40f036460d55d602a388a300909d7750211d4 | 2774 |
| 🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/value-reference-cases.json | f6846b98c3cda909264839a7d19c49c7dd78063a2d253dac9d809493052e6e41 | 7805 |
| 🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/value-reference.schema.json | 52dc8b1aa9df87c05720eadef649d9bd1b22a64729cd301f239c1e7da25c6e75 | 3670 |
| 🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/singleton-cases.json | 70370beb7d9557fb632c1f65f621b5f71198d4614193bffb4792e7f5f10a8f5f | 1910 |
| 🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/seed.schema.json | e5af4e09b42f971f279e06b4b0c5cdab33e97431178918d63a967d74b814fafa | 919 |

Runtime repo_text! avoids embedding this corpus into the Rust source closure. Its data remains an explicit hashed native source of the authored owner. This trace establishes the real ownership/normalization path and asset bytes; it does not claim that an older stored graph already contains the latest reference declarations.
