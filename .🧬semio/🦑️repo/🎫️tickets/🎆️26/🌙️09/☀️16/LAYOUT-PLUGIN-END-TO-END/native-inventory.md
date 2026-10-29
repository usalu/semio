# Native Layout Normal Source Inventory

2026-10-28. Canonical readRustSourceClosureV1; macOS ARM64/unix cfg without test/testkit; bounded layout artifact tree only. No compiler, graph or test launched. Normal entry means each Cargo lib target with cfg(test) disabled; test-only companions may consequently have an empty closure. Global bounds:32inputs,262144bytes,4000active lines.

Inspected 31 library manifests. Failing owners: 3.

```json
[
  {
    "name": "semio-s-artifact-layout-layout-io",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust/Cargo.toml",
    "files": 37,
    "bytes": 253873,
    "lines": 1732
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml",
    "files": 46,
    "bytes": 413833,
    "lines": 1737
  },
  {
    "name": "semio-s-artifact-layout-layout-diff",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml",
    "files": 33,
    "bytes": 281518,
    "lines": 2653
  }
]
```

Input script retained as native-inventory.ts. This measures package source ownership only, not transitive external package dependency readiness or compiler correctness.
