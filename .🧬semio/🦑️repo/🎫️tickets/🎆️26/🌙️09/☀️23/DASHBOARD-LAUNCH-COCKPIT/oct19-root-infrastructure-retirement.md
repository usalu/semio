# Root Infrastructure Retirement

The final independent audit found active setup generation of removed IDE instruction aliases, root test readers for removed IDE MCP files, and two Rust boundary test target inputs that still depended on launch configuration. These active dependencies are now retired.

## Changes

- The setup command's `AGENT_INSTRUCTION_ALIASES` contains only `CLAUDE.md`. Codex reads canonical `AGENTS.md`; Claude retains its existing link with the existing Windows hard-link fallback. Removed Gemini and Copilot alias generation and its misleading client documentation. No AGENTS file changed.
- Root README documents only current Codex/Claude headless instruction behavior.
- Root checked-in asset tests no longer read `.cursor/mcp.json` or `.vscode/mcp.json`. Existing Codex, generic headless MCP, devcontainer, native bootstrap, Postgres, ignore and hook assertions remain. The existing MCP handshake uses the retained Codex profile.
- UI and OS Rust `test-crate-boundaries` targets now declare canonical `📋️build-owners.json` and their own project manifest inputs instead of `.vscode/launch.json`.
- Expanded the language-neutral dashboard-control schema, JSON vectors and Gherkin with the current instruction alias list, root test reader coverage and neutral Rust target inputs. Third-party TypeScript parsing inspects the actual setup alias declaration; the repository schema compiler and Ajv independently validate the shared vectors. The root source is also checked for indirect retired asset path literals.

## Verification

Before implementation, the new focused regression produced two failures: the Rust input still contained launch configuration and setup aliases still included Gemini/Copilot. The independent auditor separately executed exactly the two old Cursor/Copilot root asset tests; both failed with ENOENT, with nine other tests filtered out.

After implementation:

- Complete dashboard-control suite: **7 passed, 242 assertions, 0 failures**.
- Root MCP bootstrap asset checks: **5 passed, 20 assertions, 0 failures**; four unrelated tests filtered out.
- Read-only scans of active framework project manifests and root tests found no remaining removed MCP/launch input references. Setup source and README contain none of the retired client aliases checked.
- Read-only `git diff --check` passed for this slice.
- An additional strict TypeScript check of dashboard-control was attempted and failed on existing transitive native storage signatures, unresolved workspace package aliases and library discovery/normalization diagnostics. This is not claimed as a successful static check. The executed test suite includes TypeScript syntax diagnostics for the root reader source.

No `setup git`, Git hook test or MCP initialize process was executed in this slice: the first two would mutate Git configuration or create a test Git repository, and the latter depends on the separately blocked native installation. Actual alias filesystem regeneration and the native handshake therefore remain unverified here. Source inspection confirms only the retained alias is visited by the existing loop. No Git mutation, worktree operation, storage recovery, IDE asset deletion or cleanup bypass occurred. The parent retains ownership of conserving and removing the concurrently recreated launch asset.

## Files Updated

- `README.md`
- `🧪️tests/🧱️checked-in-asset-contracts/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🥒️.feature`

Concurrent owner conservation changes in `🎮️owners.json` were preserved and were not authored by this slice. This report is retained ticket evidence.
