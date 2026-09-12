# Browser Actor Import Dispatch Taxonomy — 2026-09-12

The assertion implementation remains a direct case leaf. Nx project metadata and executable dispatch belong to the browser-bundle semantic owner.

## baseline

```json
[
  {
    "code": "test-data-in-case",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/📋️project.json",
    "line": null,
    "detail": "Test folders contain executable implementations and feature contracts; example inputs and expected outputs belong in owner fixtures."
  },
  {
    "code": "test-implementation-filename",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/📜️script.ts",
    "line": null,
    "detail": "The implementation filename must be one of 🐍️.py, 🐹️.go, 🔷️.cs, 🟦️.cts, 🟦️.d.cts, 🟦️.d.mts, 🟦️.mts, 🟦️.ts, 🟦️.tsx, 🟨️.cjs, 🟨️.js, 🟨️.mjs, 🦀️.rs."
  }
]
```

## verify

```json
[]
```

The actual layout guard reports zero case findings. The TypeScript parser accepted the owner dispatcher, JSONC parsing succeeded for both launch files, and both launch commands resolve to the retained project name and targets. No full JCO actor runtime success is inferred from these structural checks.

Exact edits: removed `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/📜️script.ts`, moved `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/📋️project.json` to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📋️project.json`, extended `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts`.

## Exact Dispatcher File Ledger

```json
{
  "updated": [
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts"
  ],
  "created": [
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📋️project.json"
  ],
  "removed": [
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/📜️script.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/📋️project.json"
  ]
}
```
