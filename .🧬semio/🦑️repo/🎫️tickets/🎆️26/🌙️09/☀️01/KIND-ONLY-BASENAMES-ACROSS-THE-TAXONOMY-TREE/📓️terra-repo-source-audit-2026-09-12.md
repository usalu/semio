# Repository Source Ownership Independent Audit

Date: 2026-09-12

## Scope and method

This bounded read-only review covered the portable 22-source repository ownership map, Next wrappers, retained package glue, CLI/test-host native manifests, the .NET include, VS Code builder anchors, coordinator adapter anchor, and live taxonomy chains. It did not regenerate launch data or outputs, reset caches, invoke broad services, run a coordinator runtime, mutate source/configuration/Git/AGENTS, or touch lifecycle state. The registered check used a private Nx data directory below `🗑️generated/terra-repo-source` with `NX_DAEMON=false` and `NX_ISOLATE_PLUGINS=false`.

## Accepted evidence

- `bunx nx run @semio-tech/repo-lib:test-repo-source-ownership --skip-nx-cache` passed: **6 tests, 212 expectations, 0 failures** (684 ms test time; 916 ms Nx elapsed).
- Independent live traversal of all **22** fixture entries confirmed owner existence, required anonymous kind leaf, anchor presence, and legacy disposition. It parsed every TypeScript owner with the installed TypeScript parser. The **15** retained wrappers consist only of export declarations and resolve to their stipulated owner.
- The **12** Next wrappers have zero parse diagnostics, no non-export statements, exactly one relative target each, and the required export sets: authentication `GET, POST`; breach `GET`; diff `POST`; event `GET, POST`; repository `POST`; scope `GET`; ticket detail `GET`; ticket `GET, POST`; warning `GET`; GitHub webhook `POST`; layout `default, metadata`; page `default`.
- The shared package-body classifier classifies all three retained TypeScript package entries (VS Code, repository library, test) as `declaration`. This is classifier evidence, not a line-count inference.
- `cargo metadata --no-deps --format-version 1` returned zero for both exact manifests. The CLI package has one matching manifest package and existing library `⌨️cli/🦀️.rs` plus binary `⌨️cli/🚪️entrypoint/🦀️.rs`; the test-host has one matching package and existing library `🧪️test/🦀️.rs`.
- The .NET project retains `EnableDefaultCompileItems=false` and explicitly includes/links `../../🔷️.cs`, which exists. The completed executor report's zero-warning .NET build is retained as its separate build evidence; this audit inspected the include rather than repeating a build.
- The VS Code package `📜️script.ts` imports `../../🏗️builder/🟦️.ts`; its Nx `sourceRoot` is the semantic VS Code root. The builder owns the `import.meta.vitest` transform anchor. The coordinator Nx project likewise has its semantic root as `sourceRoot`.
- The coordinator port adapter anchors `createRequire` with `new URL("../📦️packages/🟦️typescript/package.json", import.meta.url)`, and that package manifest exists. The active server-library consumer directly imports `../../../🎛️coordinator/🔌️ports/🟦️.ts`.

## Actionable finding: library-side directory taxonomy gap

The registered fixture and all native/wrapper checks pass, but the live `semanticDirectoryKindId()` resolver cannot type the full chain of two owned library-side paths:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`: `📚️library` has no resolution under `modules`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟦️.d.mts`: `📚️library`, then `🕸️dependencies`, then `🧩️runtime` have no resolvable chain.

Exact pre-correction trace contexts were `[capabilities, products, repo, modules]` followed by unresolved `📚️library` for the first owner, and that same resolved prefix followed by unresolved `[📚️library, 🕸️dependencies, 🧩️runtime]` for the declaration owner. Directly, `semanticDirectoryKindId("📚️library", taxonomy, { parentKindId: "modules" })` returned `null`. The missing library kind prevents a meaningful resolved parent context for its descendants. This is a current failed acceptance, not a pending-repair pass.

The taxonomy member registry contains the thirteen coordinator/VS Code additions named by the executor (`🚪️entrypoint`, `⚠️warnings`, `🔐️authentication`, `🎫️ticket`, `🎯️scope`, `📊️dashboard`, `🖼️shell`, `🧾️diff`, `🪝️webhook`, `🦑️repo-source-ownership`, `🚨️breaches`, `🔎️detail`, and `🐙️github`), and the direct full-owner traversal has no other unresolved row. The gap is specifically the library branch, which also contains the ownership fixture/schema/test contract.

Repair the taxonomy chain rather than exempting these owners: register `📚️library` under `modules`, `🕸️dependencies` below that library member, and `🧩️runtime` below dependencies; then add a focused resolver/inventory vector for both paths. This gives the new owner and its declaration leaf a complete domain-driven directory taxonomy.

## Current coordinator alias limit and provenance

Current coordinator `tsconfig.json` maps `@/lib` to `../lib/index.ts`; that target is absent in the live tree, while route owners import `@/lib`. Per the completed executor's comparison, this is the only missing-referent finding with **both HEAD and current** evidence. Bun's entry bundles externalize aliases and bare imports, so their success does not prove this alias resolves at runtime.

The omitted server library that owns the required API and its alias repair are already assigned in the separate manifestless-source closure lane. This audit does not assign a preimage to current VS Code TypeScript diagnostics or CLI formatter diagnostics; their completed reports establish only current observations. It does not create a compatibility module.

## Limits retained

- No broad coordinator/VS Code TypeScript run, service startup, or external runtime probe was run. The missing alias makes a coordinator runtime proof unavailable in any event.
- The executor's CLI 24-test pass, test-host compiler completion, .NET zero-warning build, VS Code Vite bundle, and native Next bundles remain recorded evidence; no broad suite was repeated here.
- The six separately discovered manifestless sources, active discovery policy correction, root-framework source moves, and the concurrent WGPU browser-profile source addition are outside this 22-row audit. No intermediate WGPU artifact state was treated as a repository-source defect.

## Probe retention

The private Nx log, AST/metadata probe records, and isolated Nx state were written below `🗑️generated/terra-repo-source` during the audit and then removed. This report retains the exact observations.
