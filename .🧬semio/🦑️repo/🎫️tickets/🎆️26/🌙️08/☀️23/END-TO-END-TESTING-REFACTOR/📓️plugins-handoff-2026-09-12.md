# Plugins Testing Taxonomy Handoff

Date: 2026-09-12  
Lane: plugins

## Root-Owned Changes

- No plugin-lane change is outstanding in root `Cargo.toml` or root `📜️script.ts`. A focused scan found no `testkit`, `test-support`, singular oracle path, or renamed hub feature reference in either file.
- Keep `✏️s/🔌️plugins/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js` as a semantic test-case implementation. The audit explicitly classifies `🔮️protocol-oracle` as a case name rather than an oracle collection alias; it must not be folded into a `🔮️oracles` collection.
- The root-owned document-contract substitution remains required in `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/📌️retained-actions/🟨️.js`: its direct `Ajv` strict compiler rejects the shared schema keyword `x-semio-child-kind`. The full Sequence script stops there after its first two JavaScript checks; the separately invoked protocol-oracle case passes.

## Framework Coordination Evidence

- Plugin consumers now use `semio_framework_plugin::artifact_app_laws`; static verification resolved 720 direct references across 78 owning Cargo manifests, with every manifest enabling the test-only `artifact-app-testing` feature.
- SPR consumers now use `protocol_laws`; static verification resolved 282 direct references across 36 owning Cargo manifests, with every manifest enabling the test-only `protocol-laws` feature.
- The attempted five-package Cargo check is currently blocked before any selected plugin package compiles by concurrent framework failures:
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2232`: missing `LoadDocumentArchive` and `ReadDocumentArchive` command match arms.
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2600`: missing `DocumentArchive` frame match arm.
  - `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs:23`: mounted `../../📤️output/🦀️.rs` is absent.
- Rust parser coverage reaches 365 of 366 authored/current destination files. The remaining parse is blocked while rustfmt recursively resolves `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:36`, where a concurrent production struct edit currently ends with `expected identifier`.
