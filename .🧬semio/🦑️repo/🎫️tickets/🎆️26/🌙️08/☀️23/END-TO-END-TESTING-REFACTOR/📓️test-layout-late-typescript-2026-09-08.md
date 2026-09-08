# Late TypeScript Test Layout

The 58-finding full scan caught two newly added flat TypeScript tests. Each executable and its unchanged language-neutral fixture now live in a named case under the existing semantic owner. The cache-contracts runner imports both new paths. Only module specifiers and the source-directory calculation changed; assertions and fixtures remain unchanged.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🔣️.json` → `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🔒️playground-preferences/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🔣️.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🔒️playground-preferences/🔣️.json`

Runtime verification passed through public `bun nx exec --projects=layout-probe -- bun <retained verification input>` using the private minimal graph and the actual moved exports. Preference forwarding matched the language-neutral schema fixture and lodash oracle. The production case copied five declared browser artifact owners through Vite, loaded the relocated imports, respected `write:false`, and verified native Nx warm-cache reuse, deleted-output restoration, byte equality, and dependency-change rebuilding. The command exited 0.

The first private harness attempt used an output directory named `runtime`, which tripped the existing assertion forbidding `runtime` anywhere in each absolute source path. Repeating with the neutral `fixtures` output directory passed; no test assertion or production source was changed. The retained verification input requires explicit repository/output paths. Both fixtures moved byte-for-byte.

## Exact Authored Paths

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-late-typescript-2026-09-08.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻late-typescript-verification/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🔒️playground-preferences/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🔒️playground-preferences/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🟦️.ts"
]
```
