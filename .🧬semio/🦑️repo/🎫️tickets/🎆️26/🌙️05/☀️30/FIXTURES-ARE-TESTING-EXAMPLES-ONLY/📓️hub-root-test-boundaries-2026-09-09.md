# Hub and Root Test Boundaries

The Hub admin build router used two example JSON graphs to execute test assertions during production builds. Those assertions now live in the semantic admin owner’s build-graph test implementation, invoked only by its test route; production build invokes Vite directly. The root Rust warning scope vectors and Cargo oracle now live in a canonical test implementation, awaited by the existing verification route. Existing schema contracts remain at their owners. Public Bun/Nx execution passed both relocated Hub graph checks and the Rust target-scope oracle: 4 entry laws, 5 stylesheet laws, 3 target vectors, 2 rejection vectors and Cargo metadata agreement for 160 shipping component crates.

## Preserved Moves

```json
[
  {
    "old": "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🚪️entry-graph.json",
    "new": "🌎️hub/🔨️modules/🛡️admin/🧫️fixtures/🕸️build-graph/🚪️entry-graph.json",
    "sha256": "d84dea08fd0cfd2c2fd84cc39ed86ed61d64685566e0be9a6f98fa2e382123f1"
  },
  {
    "old": "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🎨️stylesheet-graph.json",
    "new": "🌎️hub/🔨️modules/🛡️admin/🧫️fixtures/🕸️build-graph/🎨️stylesheet-graph.json",
    "sha256": "4e0d80fb67f0053afae16ee8a70612c2f10d6798e645f97e33f0845a91a26cd5"
  },
  {
    "old": "🧪️tests/🦀️rust-warnings/🔣️.json",
    "new": "🧫️fixtures/🦀️rust-warnings/🔣️.json",
    "sha256": "1fa4ee9a73e56673a0bb8505fd9f79b8ddc2574a407025d91f3e9bdd139a7aa9"
  }
]
```

## Authored Paths

```json
[
  "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts",
  "🌎️hub/🔨️modules/🛡️admin/🧪️tests/🕸️build-graph/🟦️.ts",
  "📜️script.ts",
  "🧪️tests/🦀️rust-warnings/🟦️.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️hub-root-test-boundaries-2026-09-09.md",
  "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🚪️entry-graph.json",
  "🌎️hub/🔨️modules/🛡️admin/🧫️fixtures/🕸️build-graph/🚪️entry-graph.json",
  "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🎨️stylesheet-graph.json",
  "🌎️hub/🔨️modules/🛡️admin/🧫️fixtures/🕸️build-graph/🎨️stylesheet-graph.json",
  "🧪️tests/🦀️rust-warnings/🔣️.json",
  "🧫️fixtures/🦀️rust-warnings/🔣️.json"
]
```

## Verification Input

`🧑‍💻coordination/🧪️hub-root/📜️script.ts` invokes the actual extracted exports against the live workspace. The production build route is structurally separated from these fixture checks; a complete Vite production build has not yet been rerun.
