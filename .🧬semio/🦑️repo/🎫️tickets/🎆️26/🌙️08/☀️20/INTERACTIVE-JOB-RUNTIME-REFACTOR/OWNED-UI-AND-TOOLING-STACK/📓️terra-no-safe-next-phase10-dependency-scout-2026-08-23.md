# Terra Next Phase 10 Direct Dependency Scout — No Safe Candidate — 2026-08-23

## Decision

**NO SAFE CANDIDATE.** The accepted live boundary remains **130 = 67 JavaScript + 63 Rust**. There is no smallest truthful direct Phase 10 retirement packet after the zero-input root MDX configuration chain was exhausted.

The only smaller-looking unbound tooling rows are manifest-only and expressly fail this scout's direct-use requirement. The nearby active leaves all alter live build, lint, test, Storybook, Nx, extension, or materialization behavior and have no owned replacement with a parity differential. A speculative removal would be a behavior regression rather than a dependency retirement.

## Current Boundary And Candidate Review

`bun ./📜️script.ts verify dependencies list js --format json` reports 67 JavaScript identities. The narrow direct-tooling candidates closest to a one-identity wave were independently reviewed:

| Candidate                                                              | Live owner / binding                                                                                                                            | Decision                                                                                |
| ---------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `eslint-plugin-react-hooks`                                            | Root manifest tuple and one `7.1.1` lock resolution only; no source/config import, registration, executable, or runtime use outside inventories | Reject: manifest-only, so no truthful direct-use proof                                  |
| `@storybook/addon-vitest`                                              | Root manifest/resolution and `.storybook/main.ts` `addons` entry                                                                                | Reject: active Storybook test-addon behavior                                            |
| `@storybook/addon-docs`                                                | Root manifest/resolution and the same active `addons` entry; produces the 61 Autodocs boundary                                                  | Reject: active generated-docs behavior                                                  |
| `@storybook/react-vite` / `storybook`                                  | Root framework and build path                                                                                                                   | Reject: active builder/framework infrastructure                                         |
| `@tailwindcss/postcss` / `postcss`                                     | UI React manifest and exported `🎨️postcss.config.ts` which registers `@tailwindcss/postcss`                                                     | Reject: active CSS transform configuration                                              |
| `@vitest/coverage-v8`                                                  | Root/UI manifests, shared resolution, and UI Vitest coverage configuration                                                                      | Reject: active coverage-provider behavior                                               |
| `@nx/js` / `@nxlv/python`                                              | Root `nx.json` plugin registrations; Python executor is also named by excluded Compose project targets                                          | Reject: active workspace configuration; Compose boundary prevents a root-only deletion  |
| `@bytecodealliance/jco` / `binaryen`                                   | Plugin web materializer invokes JCO `transpile` and Binaryen `wasm-opt`                                                                         | Reject: active WebAssembly materialization, requiring a Phase 9 ownership/parity packet |
| `eslint-plugin-storybook`, `@eslint/js`, `typescript-eslint`, `eslint` | Root lint configuration imports and applies them                                                                                                | Reject: active lint policy                                                              |
| `@playwright/test`, VS Code test/package tools                         | Root/extension or Compose test/package behavior; rows without a live binding are manifest-only                                                  | Reject: no bounded owned replacement/differential, or no direct-use proof               |

`eslint-plugin-react-hooks` is the closest one-owner root tooling row lacking any active non-manifest binding. It is disqualified rather than accepted: the source census finds only its root manifest and inventory entry. Treating that absence as sufficient would contradict the required active-binding standard. Its lock resolution currently retains Babel, Hermes parser, Zod, and validation-error transitives, so its removal would also expand the unexplained lock delta beyond a clean source-led packet.

## Held Boundaries

- `dagre` remains held for the real Rust/Wasm/OffscreenCanvas directed-layout lane.
- Compose is excluded. Its active Vite, PostCSS, Python-executor, Playwright, and MDX-related behavior cannot be used to justify or test a root/UI wave.
- P3/P8 Rust paths and all Cargo actions are excluded.
- No Storybook/addon-docs, Vite/Tailwind/PostCSS, Nx, test provider, extension tool, or JCO/Binaryen retirement is bundled into this scout.

## Expected Delta And Executor Packet

Expected delta: **none**. The accepted boundary stays **130 = 67 JavaScript + 63 Rust**.

No executor packet is authorized. A valid next packet must first establish all of the following for exactly one direct identity:

1. An active source or configuration binding within the non-Compose Phase 10 scope.
2. A behavior proof showing that binding is unreachable or an owned replacement plus a pre/post parity differential.
3. Exact direct-manifest owners and lock reachability, with no unexplained transitive sweep.
4. No P3/P8 overlap, Cargo action, Dagre impact, or Storybook/tooling bundle.

## Independent Gates For A Future Accepted Leaf

Any newly proven candidate must retain the dependency ratchet/list/parity and frozen install gates, then add the domain-specific differential before deletion. For a UI tooling leaf this includes the uncached UI Storybook build, exact `231 = 170 stories + 61 docs`, 61 TS/TSX inputs, 61 Autodocs, zero MDX, the frozen index hash where reliable pre/post evidence exists, UI quick/lint/typecheck, source/manifest/lock absence and retention scans, root script syntax, relevant Prettier baseline, and scoped plus whole working/staged/HEAD diff checks.

For a runtime or materialization leaf, Storybook equivalence is not a substitute: an owned behavior parity packet is mandatory. No direct production, manifest, lockfile, Git, ticket-lifecycle, or Cargo input was changed by this read-only scout.
