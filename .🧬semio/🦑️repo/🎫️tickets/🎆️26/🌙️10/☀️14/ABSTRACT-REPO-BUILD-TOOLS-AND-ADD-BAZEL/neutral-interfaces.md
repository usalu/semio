# Codec and MCP Neutral Entry Corrections

The two codec package wrappers now call the neutral repository entry. The existing `../../../../../..` cwd was independently resolved from the actual package directory to the actual repository root; it is six parents. Only the two entry words changed from `nx` to `repo`, preserving targets and all other manifest bytes.

MCP native staging now has one route: `bun repo run @semio-tech/framework-os-mcp-rs:build --output=stream`, with repository cwd, existing inherited environment and owner PID marker, `shell:false`, captured progress and exit-status propagation. The direct package attempt and subsequent Nx fallback were removed. Current target authoring resolves to the real Rust package script, whose build branch calls the owned Cargo artifact publisher. The repository build router has no `ensureMcpBinary` call, and no bootstrap recursion requiring the bypass was found. This is Source inspection, not a compiled build claim. The four repair commands and three stale Nx docstrings were neutralized as well.

## Test Evidence

The package routing law previously skipped parsed direct-Nx wrappers; it now refuses those candidates. The existing language-neutral command-source fixture now includes the codec manifest and MCP driver. A new assertion in the same test file uses actual TypeScript AST, actual declared project identity and independent `string-argv` to require one exact neutral staging command, correct cwd, shell behavior and inherited owner environment.

- Actual RED: three failing cases / 2,948 assertions, including the codec escape, MCP direct-Nx source and two staging subprocess calls. Retained `🗑️generated/neutral-interface-red.txt`.
- Focused GREEN: three passing cases / 4,135 assertions / 809 ms. The package oracle covered 678 tracked wrappers using independent argv and actual Node path resolution. Retained `🗑️generated/neutral-interface-green.txt`.
- Existing independent Ajv fixture/schema law: one passing case / nine assertions / 201 ms. Retained `🗑️generated/neutral-interface-schema-green.txt`.
- Existing native MCP fixture, registered without modification through ticket `mcp-stage/📜️script.ts`: 19 passing cases / 36 assertions / 198 ms. Actual injected staging ran once when absent, skipped unchanged sources, restaged after a dependency mutation, rejected absent/malformed source receipts, preserved explicit overrides and checked process-owner lifetime. POSIX/Windows artifact vectors and dep-info/freshness vectors passed. All fixture files lived under ticket generated and were retired by the fixture; the longest constructed path was 240 characters. Retained `🗑️generated/mcp-staging-fixture-green.txt`.

No real MCP compiler, target build, public product test or full graph producer ran. The initial Bun invocation without a leading `./` was a file-selection refusal, not a semantic RED; the retained RED above is the subsequent actual test execution.

## Owned Files and Final Identities

- `🧰️framework/🔨️modules/🚪️io/📦️codec/📦️packages/🦀️rust/package.json` — SHA `52944046f5b4eee27cc1100e7ec46892255ec73dff800035e85bb9552beab1c3`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` — SHA `cd879da6c3d74073e2bba2974756a22e56f2cd28a933444606e89d1dd6a94c42`, including final docstrings.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔧️build-surfaces/🟦️.ts`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🔧️build-surfaces/🔣️.json`.
- Ticket `mcp-stage/📜️script.ts` and this report are retained inputs/evidence. Source and harness paths were added if absent to `📥️files.json`; existing entries were preserved.

Independent read-only review confirmed the neutral staging route, cwd/environment inheritance and lack of a build-path staging callback. Runtime compilation remains unclaimed.
