# Repo Root Boundary Follow-up

The current scan identified two VS Code build cases below a TypeScript delivery package, and one transaction-process example left under the repository tests tree. Both VS Code cases now belong to the semantic extension owner; imports, fixture reads, package roots, and the caching test route were rebased. The process vector now belongs to the repository owner fixture scope beside its actual canonical case. Runtime checks passed: both VS Code exports and all eight process-observer tests (342 assertions).

## Moves

```json
[
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/📦️package/🟦️.ts",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/📦️package/🟦️.ts",
    "inputSha256": "697b3b76d98b019f8988cdfe5b6724031424d443ccb9011309343b7cab292c00",
    "outputSha256": "4ec85516d8b6480edbdfb089edab8d38f1650d8decb9dac0d4feb81a1b699888"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/🧩️host-build/🟦️.ts",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️host-build/🟦️.ts",
    "inputSha256": "be604109ccb6c005cd68895b20ee8e84128191e3edee59230ce4784f192ffc2f",
    "outputSha256": "fb282209716bdcdbcd405cce24f76cf25a12a0810ddf3c1dc74b59e098d2841c"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🧪️tests/🧪️transaction-process-ownership/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🧫️fixtures/⚙️transaction-process-ownership/🔣️.json",
    "sha256": "7d094633bfc6d4043d109ffc4da68927a2e9a654576d8285947da37748c4be18"
  }
]
```

## Authored Paths

```json
[
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🧪️tests/⚙️transaction-process-ownership/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/📦️package/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/📦️package/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/🧩️host-build/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️host-build/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🧪️tests/🧪️transaction-process-ownership/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🧫️fixtures/⚙️transaction-process-ownership/🔣️.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️repo-root-boundary-followup-2026-09-09.md"
]
```

## Runtime Evidence

The existing VS Code host-build case executed Vite, esbuild, and native Node successfully from its new semantic owner. The package case executed VSCE twice and independently decoded the archives with yauzl; differing source timestamps preserved the expected file allowlist, SHA-256 bytes, and ZIP metadata. Both exports passed. The relocated process-observer fixture passed eight tests with 342 assertions, including Ajv/jsonc-parser validation, two compilers, SmartBuffer binary decoding, Lodash identity comparison, and strict TypeScript declaration checking. No native process probe was run.
