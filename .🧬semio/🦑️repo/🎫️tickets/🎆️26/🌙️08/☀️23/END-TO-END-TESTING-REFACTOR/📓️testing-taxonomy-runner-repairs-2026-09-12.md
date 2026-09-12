# Final Runner Coordinate Repairs

Corrected the scale project sourceRoot to its fixture source owner, retargeted the two existing VS Code build commands to the declared build-wasm target, and repaired the host test command's segmented source-reader path. No compatibility target was added.

The actual JSON/TypeScript parsers accepted all four files; every scale launch target resolves to a declared target, both source roots and dispatch script exist, and the host source reader resolves to the fixture. This is configuration/path verification; the separately recorded current WIT ABI failure still limits the WASM build.

The two artifact constants are consumed only by explicit host testing and dev benchmarking commands. They are test-execution orchestration, not production runtime imports of fixture source.

```json
{
  "updated": [
    "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📋️project.json",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts",
    ".vscode/launch.json",
    ".vscode/🧩️launch.seed.jsonc"
  ],
  "verifiedLaunchCommands": 4
}
```
