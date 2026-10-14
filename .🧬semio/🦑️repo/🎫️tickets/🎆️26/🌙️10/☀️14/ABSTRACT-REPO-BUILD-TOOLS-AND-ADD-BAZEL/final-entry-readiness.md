# Final Public Entry Readiness

## Exact pin harness review

The current command-audit harness requires exactly an absolute immutable pin path and lowercase SHA-256 as literal arguments, matches them against the environment, requires the exact ticket artifact directory and reuse flag, independently hashes the bytes, applies the strict neutral reader and requires whole-workspace admission. The import-only `string-argv` oracle resolves from its declaring Repository library package and loads by dynamic ESM import; no `require` fallback or install was introduced.

The current planner and Stdio harnesses also require the literal absolute path and digest, exact matching pin environment and strict whole-workspace admission. Planning independently hashes before strict parsing, resolves Ajv from the actual Nx SDK and validates all base/configured plans without task execution. Its local five-minute planning bound begins after graph admission and retains 250-plan progress/cancellation checks. Stdio's current shared pin reader independently checks bytes and strict schema; the actual comparison uses Cargo metadata and the neutral project/dependency graph. It releases the initial admission parse before the later comparison. Unlike command-audit, those two retained harnesses do not independently assert `SEMIO_BUILD_REUSE_GRAPH=true` or the artifact directory's exact ticket identity; the final authorized invocation must supply those original values through the outer public entry.

Final commands remain public neutral `bun repo --tool=nx exec --projects=workspace -- bun <absolute ticket harness> <absolute ALL pin> <SHA-256>`, with the same mandatory pin/digest/reuse/root/artifact environment. Root must release the actual ALL pin and serialize the large readers before these checks run. This review loaded no graph and makes no final runtime acceptance claim.

## Stdio ownership oracle correction

Actual bounded command ownership exposed the stale bare `bun repo` package-string assertion after the package wrapper root-routing correction. The retained red is `🗑️generated/stdio-ownership-routing-red.txt`. Only the existing Stdio test oracle and its language-neutral fixture changed. The oracle now parses through declaring-owned `string-argv`, validates the package-relative cwd against the actual workspace root with native Node path semantics and checks the exact neutral suffix arguments. POSIX and Windows fixtures include spaces and wrong-cwd/target refusals. Runtime package scripts and original ownership/input assertions are unchanged.

The existing bounded ownership suite passed: **1 test, 9 owners, 2 routers, 36 artifacts, 5 commands, 1.236 seconds**, exit 0; `stdio-ownership-routing-green.txt`. This actual run exercises dynamic ESM loading of the import-only parser. It reads no build graph, runs no Cargo metadata and opens no runtime store.

## Scoped public entry scan

The read-only scan used the retained source inventory plus explicit `.github`, `.vscode` and `.agents` surfaces, excluding caches, generated outputs, ticket history, tests and private adapter oracles. It inspected 252 manifest, project, launch, workflow and instruction/documentation files and found no direct old `bun nx`, `bunx nx`, `npx nx`, `pnpm nx` or public `nx run/exec/serve/build/test/reset/graph/show` command forms. Exact inputs and matches are retained in `public-entry-scan.json`.

A separate scoped Source scan checked the retained dashboard registry/inventory Rust files and Stdio composition readers. It found no Nx public invocation; the only match was the harmless `.nx` directory exclusion in source walking. Evidence is `public-entry-dashboard-scan.json`. This is a bounded static entry audit, not a claim that every file in the repository or every backend runtime was executed. No additional production Source change was needed.

## Final production counter agreement

Read-only review confirms the planner visits every base task and every explicit configuration, expands full dependency closures through the shared planner, validates strict plan schema and reports cycle/configuration failures. The generic graph schema intentionally admits an empty ALL fixture; the tiny strict-reader diagnostic in `planner-vacuity-diagnostic.txt` is unit evidence only, not a defect or production qualification. No harness or graph-schema change was made.

Root requires final production assertions after the actual run: digest must equal the released Core ALL receipt, project/base/configuration counts must equal that same ALL graph's enumeration and be positive, expanded task count must be positive and failures must be zero. The exact reuse and ticket output environment is set by the original outer invocation and recorded in its receipt. No zero-task synthetic ALL fixture substitutes for that production evidence. Final actual planner and Stdio execution remain blocked until Root releases the genuine ALL pin and serialized reader slot.
