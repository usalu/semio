# 🧪️ Runtime Spawns Resolve Through The Tool Registry

## Cause

Tests and one production file spawned bare `bun`, `node` and `cargo` by name, so they depended on the host PATH. On this host `bun` is only the repo-managed runtime, so 3 cancel tests failed (28 pass, 3 fail).

## Fix

- Added `registeredToolExecutableV1(toolId, root?)` to `🧰️framework/🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts`. It resolves through the tool registry (`resolveRegisteredToolV1`) and throws `unknown-tool` or `tool-missing` like `runToolV1`.
- Rewrote 114 `.ts` files under `🧰️framework/🛍️products/🦑️repo` and `🧰️framework/🔨️modules/🏃️process` (182 call sites). Each spawn call with a literal `"bun"`, `"node"` or `"cargo"` now calls the helper, and each file imports it. One production file was included: `📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts` (cargo).
- Loop and ternary literals in `🧱️command-composition-source` and `📐️test-layout` now use the helper too.

## Skipped (other agent owns them)

Files still spawning bare names, under `🕸️graph` or `📥️inference`:

- `📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🧹️supersede/🟦️.ts`
- `⚡️caching/🕸️graph/🏁️startup/🚦️service/🧪️tests/🟦️.ts`
- `⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts`
- `⚡️caching/📥️inference/🧪️tests/🟦️.ts`
- `🏗️build/🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts`
- `🏗️build/🕸️graph/📥️inference/🧪️tests/🧮️covered-inputs/🟦️.ts`
- `🏗️build/🕸️graph/📥️inference/🧪️tests/🔗️workspace-closure/🟦️.ts`
- `🏗️build/🕸️graph/📥️inference/🧪️tests/📐️declared-inputs/🟦️.ts`
- `🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests/🟦️.ts`
- `🏗️build/🕸️graph/📥️inference/🧪️tests/🪆️owner-child-closure/🟦️.ts`

Left as is: `🚷️discovery-boundaries` line ~102 (`for (const runtime of ["node", ...])` compares the literal, so it needs a deliberate edit) and fixture data such as `command: ["bun", ...]` that is not spawned.

## Verification (managed bun, `~/.cache/semio/runtimes/bun/c678…/bun test`)

- `🛑️cancel` (resident): 31 pass, 0 fail.
- `🧪️test` `command-composition-source`: 11 pass. `discovery-boundaries`: 5 pass. `dashboard` `⌨️controls`: 10 pass.
- `normalization` `taxonomy-input`: 18 pass (needs `SEMIO_TEST_ARTIFACT_DIR`).
- `rust-physical-reference-context`: 31 pass, 2 fail. The same 2 fail on HEAD, so they are pre-existing.
- `🧰️tools/🧪️tests/🏃️run`: 15 pass (from repo root). `direct-tool-spawn` law tests: 9 pass.
- `test-layout`: 84 pass, 2 fail. The failures need the script-invocation parent (`repo` run), an environment gate.

## Follow-ups

- The `direct-tool-spawn` test-code ledger (`⚖️laws/🧰️direct-tool-spawn/🧾️ledger/🔣️.json`) still lists the rewritten files. The law test passes, so it is not enforced, but the ledger should be regenerated.
