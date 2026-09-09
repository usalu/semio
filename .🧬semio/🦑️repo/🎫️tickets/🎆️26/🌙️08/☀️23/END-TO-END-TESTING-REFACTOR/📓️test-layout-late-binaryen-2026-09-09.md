# Binaryen Toolchain Test Layout

The last two scan findings refer to one newly nested TypeScript test. Its executable and language-neutral fixture now live under the Binaryen toolchain owner at `🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain`. The existing cache-contracts dispatcher imports the new case; production imports and fixture paths were rebased without changing assertions or fixture bytes.

Public Bun/Nx ran the actual relocated export with an explicit ticket output directory and exited 0. The platform/archive contract matched the language-neutral fixture and Ajv schema; the case also verified target wiring, pre-abort cancellation without fetch, a mocked archive checksum rejection, and removal of rejected output. No archive was accepted or installed by the rejection probe.

```json
[
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/🛠️toolchain/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/🧫️toolchain.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-late-binaryen-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻late-binaryen-runtime/📜️script.ts"
]
```
