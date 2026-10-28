# Layout Verification Audit

Inspected 2026-10-21 by the read-only verification agent. No source or build changes. Paths below are relative to the workspace unless stated. Artifact root: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any.

## Executed Checks

| Invocation | Result | Evidence |
| --- | --- | --- |
| repo --help | Exit 1 | repo is absent from this PowerShell PATH. |
| & './.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo.exe' --help | Exit 0 | Legacy CLI supports test, MCP and entity commands; no run command. |
| Same binary: run @semio-tech/layout-js:test --output=stream | Exit 1 | unknown flag: --output=stream |
| Same binary: run @semio-tech/layout-js:test | Exit 1 | unknown command run for repo |
| Filesystem-only JSON integrity scan of layout tree | 412 parsed, 0 malformed | JSON.parse over each .json file; this checks syntax only, not schemas or semantics. |

No authoritative runtime layout test passed in this audit. The legacy test command was already reported by the coordinator to return silently; it must not be counted as a pass. I did not bypass the repo-only shell rule through bun, cargo, node child processes, or a renamed executable. No source changes to repair the separately owned build system were made.

## Entrypoint and Focused Test Routes

The root package.json routes its repo script to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts. That current source exposes runBuildCliV1 and RepoScript with run target handling, unlike the cached executable. Native recovery exists at its 🚀️bootstrap/⌨️cli/🟦️.ts and only accepts fixed authored frontend build/install, maintenance and cache-reset targets. A working installed binary was not located in the workspace cache, user .cargo/bin, user .local/bin, WinGet Links or node_modules/.bin. The existing cached repo.exe is the MCP-only entrypoint identified by coordination. A modern repo frontend must be supplied by its build-system owners before focused runtime verification can be authoritative.

The layout TypeScript owner has @semio-tech/layout-js:test in ✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/📋️project.json (repo:script). Its script routes through ../../🧪️tests/🔬️typescript-package-commands/🟦️.ts. That executor currently lists only the two demo/example tests. It omits blueprint config tests, export adoption tests and any document mutation fixture runner. It calls runCmd without await and prints layout ts ok; confirm the execution interface before treating that message as completion evidence.

Native focused crate: semio-s-artifact-layout-layout-mutations, with sibling content, frames, diff, model and descriptor crates. Editor commands, panels, engine and export have separate crate owners. The language-neutral feature 🧪️tests/📐️mutate-layout-1/🥒️.feature declares differential Rust/Python ordered-JSON comparison, explicit before/mutation/after inputs, observability and inverse laws. Existing Rust KINDS includes 31 entries; Python VECTORS contains 29 entries; prose still says 25. This coverage does not establish the existing 48-tag TypeScript union, nor upcoming structural additions. Expand the actual registered tables and scenario rows, rather than changing only descriptions.

## TypeScript Missing Semantics

The canonical 🧬️schema/🔺️diff/🟦️.ts implements strict parsers and data interfaces. It has no document diff applier, inverse algebra, concatenation or snapshot comparison. 🧬️schema/🧬️mutations/🟦️.ts defines the 48-tag externally tagged vocabulary and only three selection-transform parsers; it has no document execution/inverse dispatcher. A full layout TypeScript source scan found only window-config and window-transient apply functions, which do not mutate the document. Therefore schema/transport presence cannot substantiate TypeScript document editing or event replay parity.

The mutation file also independently redeclares Page/Frame/Story/etc with raw number values, while the canonical snapshot/diff uses branded IEEE754 word types. The next implementation must establish explicit transport-to-domain conversion and preserve float word semantics rather than coercing them through ordinary numeric arithmetic.

## Owned Next Phase

1. Assign one implementation agent exclusive conceptual ownership of TypeScript document semantics in the existing layout schema domain; coordinate edits with layout_core because its native structural vocabulary is being extended. Add language-neutral before/mutation/diff/after/inverse cases first.
2. Implement one immutable checked diff applier and inverse algebra in the TS diff domain. Validate keyed positional collections, duplicate IDs, stale indices, references, optional presence, nested layer/frame memberships and no-op semantics against Rust. All mutation executors must emit diffs and call this applier, never mutate snapshots directly.
3. Implement the closed semantic dispatcher and base-dependent inverse capture for every current and newly added variant. Reuse canonical snapshot types or explicit wire adapters. Cover Unicode text offsets, finite bounds, positive scaling and stable IDs.
4. Extend existing TypeScript owner test selection to the owned fixture runner and meaningful editor/export tests. Run via modern repo run @semio-tech/layout-js:test once its frontend is available; native crate targets should use the same admitted repo frontend. Do not claim runtime validation until real case counts and terminal status are returned.
5. Run the Rust/Python differential feature with all variants, plus inverse replay and event replay. Fix coverage tables/schema/catalog together. Python is an independent implementation, not a third-party library oracle.

## Independent Libraries Already Available

The root declares Ajv 8.20.0, fast-json-patch 3.1.1, Immer 10.1.1, pdfjs-dist, jsdom and Testing Library as existing test tooling. The existing export 🧪️tests/🟦️.ts already compares the in-house schema validator against Ajv, but this validates source-adoption contracts, not PDF rendering or document mutation semantics. Use Ajv for schema acceptance/rejection parity; use fast-json-patch compare/apply as a test-only independent resulting-snapshot oracle (translate its output behind an internal testing interface), without confusing RFC6902 operations with LayoutDiff. Immer can independently derive immutable after snapshots for fixture comparisons. PDF.js can independently parse/render exported PDF pages and inspect text; it is not an IDML or STEP validator. Existing Rust dev dependencies include serde_json, which checks JSON representation rather than native mutation correctness. No newly installed or runtime dependencies are needed for these checks.
