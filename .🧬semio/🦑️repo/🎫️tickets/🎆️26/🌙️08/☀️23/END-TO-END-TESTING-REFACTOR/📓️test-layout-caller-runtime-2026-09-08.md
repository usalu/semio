# Relocated Package Caller Runtime

The final runner audit found four obsolete flat-suite references in three package scripts. They now invoke their owner’s canonical `🧪️tests/🧩️suite/🟦️.ts`; a styling Rust documentation path was corrected as well.

The first public Bun/Nx fixture run actually collected 45 tests: 39 passed and 6 failed. Five failures exposed a surface source-root anchor and four styling fixture anchors that had not moved with the suite; one expensive mesh-catalog check exceeded Bun’s default five-second test timeout. After those anchors were corrected, the follow-up collected the same 45 tests with 43 pass and 2 failures. Both remaining failures exposed expected HTML/JavaScript strings that had been incorrectly rewritten from `./` to `../`; those strings describe emitted assets and must retain the original relative identity. The surface original was verified against pre-goal Git revision `6152f9ca6a0fbb55aa61992077837a230996b51d`.

Those two expected literals were restored. The full surface compiler-companion check and full 93-item mesh-catalog check now each declare a bounded 30-second test timeout; all byte, path, catalog-count, and schema assertions remain in force. The final run uses the normal Bun default for every other case.

## Exact Changed Files

```json
[
  "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts",
  "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🔨️modules/🗺️surface/🧪️tests/🧩️suite/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-caller-runtime-2026-09-08.md"
]
```

## Final Runtime Result

The final public Bun/Nx invocation passed all 45 tests across the three canonical suites, with 1,443 assertions and no failures (40.24-second test process). Console evidence confirms exact favicon identity and payloads, a real Vite build/preview of the declared HTML output, all seven build-adapter write modes, every current mesh asset, and graph output/registry contracts. This run used no global timeout override; only the two explicitly expensive cases use their local 30-second bounds.

## Final Wire-Retirement Dispatcher Repair

The runner inventory found the action-bus wire-retirement case had no current caller. The existing framework Rust `test-wire-retirement-source` command was already registered in its project and VS Code launch configuration, but still imported the deleted test-only script. Its import now points directly to the canonical implementation.

Public `bun nx exec --projects=layout-probe -- bun <framework Rust package>/📜️script.ts test-wire-retirement-source` executed the actual dispatcher through the private minimal Nx graph and returned exit 0. Runtime output verified five ownership cases, five hostile fixtures, and four short-close frontiers. No new public command was introduced.

```json
["🧰️framework/📦️packages/🦀️rust/📜️script.ts"]
```
