# Layout Direct Dependency Source Audit

Canonical native source closure and Rust tokens checked against ordinary dependency aliases for 34 library manifests. This static audit may include source alias false positives; it is separate from compiler validation.

```json
[
  {
    "package": "semio-s-artifact-layout-layout-model",
    "dependency": "semio_framework_value_persistent",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml"
  },
  {
    "package": "semio-s-artifact-layout-layout-model",
    "dependency": "semio_format",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml"
  },
  {
    "package": "semio-s-artifact-layout-layout-editor-commands",
    "dependency": "semio_format",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📦️packages/🦀️rust/Cargo.toml"
  }
]
```

Confirmed persistent-list identifier is a direct production crate reference; added its precise internal path dependency to model Cargo.toml. The two semio_format rows are nested store::semio_format paths, not external crate roots. Model testkit additionally requires optional serde_json for its handwritten serde implementations; moved that requirement into the explicit testkit feature without enabling it in default runtime dependencies.
