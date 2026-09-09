# Framework Computed Reader Fixes — 2026-09-09

## Result

Eight computed TypeScript test readers now resolve the existing canonical fixture or mutation-leaf owner. The changes remove stale test-local fixture assumptions, correct two owner-depth calculations, make the mutation-leaf base URL directory-relative, and restore collection of the relocated OS Dev test. No compatibility aliases were added.

## Repairs

1. Mesh suite resolves `../../🧫️fixtures/🔣️.json` from its test directory.
2. OS Dev config resolves `../../🧫️fixtures/⚙️config-graph.json` from the owning test directory, imports its executable-source helper from the package's canonical implementation, and is collected through that package's Vitest config.
3. AgentBridge climbs six owner levels before resolving `🌉️mcp/🧵️bridge/🧫️fixtures/📨️frames.json`.
4. Store native-codec-send resolves the Store owner's `./🧫️fixtures/🔣️.json`.
5. Store backbone-detach resolves the Store owner's `./🧫️fixtures/🔣️.json`.
6. Directory runtime-identity resolves the Directory owner's `./🧫️fixtures/🔣️.json`.
7. Plugin codec-caller-source resolves the Plugin owner's `./🧫️fixtures/🔣️.json`.
8. Replication local-interaction uses a trailing slash on the `set-state/` mutation-leaf base URL, so its descriptor, schema, and fixture resolve beneath that directory.

## Verification

All eight checks were launched through the ticket's private no-plugin Nx harness with Bun, isolated Nx workspace data/cache/temp directories, and `SEMIO_FIXTURE_REPO_ROOT` set to the repository root.

- Mesh's exact Bun test passed: 2 tests and 27 expectations.
- Store native-codec-send passed its registered source check.
- Store backbone-detach passed its registered source check.
- Directory runtime-identity passed its registered source check.
- OS Dev's registered Vitest route collected and ran 7 tests: 4 passed, while 3 later config-graph assertions reported the current 56-module graph, its denied discovery import, and an unrelated Bun/esbuild source-name normalization mismatch.
- AgentBridge loaded the owner implementation and canonical frame fixture and ran 31 tests: 30 passed, while the later inference-state parity assertion reported unrelated Shell fixture drift.
- Plugin codec-caller-source loaded the relocated owner fixture, then its current-source oracle reported unrelated Rust source/fixture drift in three function bodies and the protocol-trait boundary.
- Replication local-interaction loaded and validated the repaired mutation-leaf descriptor, schema, and fixture and emitted its interaction-mutation debug proof, then a later retirement section attempted to add the already registered owner schema again.
- The coordinated static recheck confirmed that all eight repaired coordinates resolve existing canonical targets and that the four scoped `./🧪️tests/🔣️.json` references are gone.

Generated Nx outputs remain under `🗑️generated/plugin-followup/framework-readers` for coordinated final cleanup.

## Authored Paths

```json
[
  "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧪️tests/🧩️suite/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/📦️native-codec-send/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔗️backbone-detach/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🪪️runtime-identity/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔣️codec-caller-source/🟦️.ts",
  "🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️tests/🧪️source-contract/🟦️.ts"
]
```
