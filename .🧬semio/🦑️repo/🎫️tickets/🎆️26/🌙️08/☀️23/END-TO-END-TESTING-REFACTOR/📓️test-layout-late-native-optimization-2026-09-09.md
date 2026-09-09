# Native Optimization Canonical Case

The concurrently added native component optimization test and neutral fixture moved from the plugin TypeScript delivery package to the plugin semantic owner. Only its production import path changed in the body; the existing cache-contract dispatcher now imports the canonical case. Its independent JavaScript Binaryen byte and WebAssembly runtime assertions are preserved.

Public Bun/Nx ran the actual relocated case successfully. Native optimization selected only the component-owned core modules, preserved unoptimized development output and foreign core bytes, matched the independent JavaScript Binaryen output exactly, and executed the expected WebAssembly export result.

```json
[
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🧪️tests/🕸️native-optimization/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🕸️native-optimization/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🧪️tests/🕸️native-optimization/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🕸️native-optimization/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻native-optimization-runtime/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-late-native-optimization-2026-09-09.md"
]
```

The current shared cache-contract dispatcher was also re-read after the late moves: all 18 literal relative imports resolve, including the newly relocated CI baseline and native optimization cases.
