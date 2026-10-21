# Print Cargo Direct Owner Audit

Observed 2026-10-21T06:57:07.375Z. Data-only independent TOML parsing; no Source function execution or native run. Read both Cargo manifests under print, root workspace dependency table and each directly resolved path Cargo manifest. Canonical owner roster 1630743 B / 23f82404e8b112ae63b04727993207aad90b1df749c870f403eddbd457ca49a6.

26 direct dependency occurrences resolve to 17 physical crate roots. Every requested dependency name equals the target Cargo package name. Sole missing exact canonical root is semio-framework-value-derive-resources, already under Cargo repair. No additional missing direct print/companion owners found; this is bounded direct closure, not a full transitive graph proof.

| Package | Canonical owner |
|---|---|
| semio-framework-artifact-reference | @semio-tech/framework-artifact-reference-rs |
| semio-framework-os-kernel | @semio-tech/framework-os-kernel |
| semio-framework-plugin | @semio-tech/framework-plugin |
| semio-framework-value-derive | @semio-tech/value-derive-rs |
| semio-framework-value-derive-resources | MISSING: 🧰️framework/🔨️modules/🌱️value/✨️derive/🚪️io/📝️text/📸️snapshot/📦️packages/🦀️rust |
| semio-framework-value | @semio-tech/value-rs |
| semio-framework-value-native-owned | @semio-tech/value-native-owned-rs |
| semio-framework-schema-registry | @semio-tech/schema-registry-rs |
| semio-framework-pack | @semio-tech/framework-pack-rs |
| semio-framework-pack-json | @semio-tech/framework-pack-json-rs |
| semio-framework-schema-validator | @semio-tech/schema-validator-rs |
| semio-framework-dsl-record | @semio-tech/framework-dsl-record-rs |
| semio-framework-dsl-record-derive | @semio-tech/framework-dsl-record-derive-rs |
| semio-framework-diagnostic | @semio-tech/framework-diagnostic-rs |
| semio-framework-print | @semio-tech/print-rs |
| semio-framework-test-fixtures | @semio-tech/test-fixtures-rs |
| semio-framework-async-macros | @semio-tech/async-macros-rs |

Print Cargo1563/aa15ff49692c8ef46120586e559a11bdc6eaeef2b9cb48f4f6bd4474c9595904. Missing resources Cargo837/d349da85618dd842c4c4d8386dabb2c20ecca77e0a29b026f45c507a298abc3c. Existing authored owner rows have exact matching roots; no fallback owner name was invented.
