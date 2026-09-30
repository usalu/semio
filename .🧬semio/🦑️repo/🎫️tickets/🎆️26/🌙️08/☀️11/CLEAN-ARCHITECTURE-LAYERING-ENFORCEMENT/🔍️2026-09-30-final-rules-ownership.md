# Final Rules and Ownership Audit

Read-only source review of the dependency-rules ownership slice on 2026-09-30. No production sources, ticket status, or Git state changed. Aggregate suites were left to the parent. The only runtime check below was a narrow validator reproduction; this report makes no new passing-suite claim.

## Actionable Finding

### P2: Self-consistent incomplete local graphs pass the completeness guard

Location: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🟦️.ts`, lines 24–42.

The completeness guard compares totals against the submitted arrays and requires one applicable framework source. It never requires resolved, followed local sources to appear in the module map. A graph containing only `framework/a.ts → framework/missing.ts`, with totals 1 module/1 dependency and no violation, is accepted with `requireFramework=true`. Removing a whole local module and adjusting totals can therefore remove its outgoing forbidden edges while retaining a passing verdict. This specifically limits the claimed incomplete-graph defense; no actual live dependency-cruiser truncation was observed.

Executed a targeted `bun -e` call against the actual exported validator with one strict framework→implementation rule and this synthetic graph. Console output was `[DEBUG] Missing local source accepted: []` (exit 0). No temporary file was created.

Recommended correction: supply the resolver's followed-local boundary and authoritative expected source inventory to validation, require local graph closure outside explicit exclusions/do-not-follow boundaries, and add neutral fixtures for self-consistent missing local modules and missing framework roots. Vendor, builtin, unresolved-package and excluded targets require explicit handling so legitimate terminal dependencies remain accepted.

## Other Reviewed Contracts

- Strict rules derive implementation areas and workspace aliases from authored taxonomy/manifests. Semantic role rules cover resolved owner paths, aliases/subpaths and React/Three vendor names. The existing fixture oracle independently invokes dependency-cruiser and checks unresolved-package status, including type/dynamic/export/require syntax.
- The validator recomputes every selected rule, checks distinct strict rule names and resolver policy names, rejects conflicting duplicate source dependencies, and compares exact deduplicated reported/recomputed verdicts. These protections are visible in source; they do not resolve the completeness finding above.
- Live execution selects taxonomy framework areas, retains focused taxonomy exclusions, and uses bounded `runOwnedCommand` execution with progress, interrupt handling and process-tree termination. Zero-violation claims are explicitly TypeScript/JavaScript; Cargo and broader schema/typecheck debt remain follow-up scope.
- Nx case discovery rejects symbolic links before recursion and rejects linked feature files; authored reserved/cache/opaque exclusions are consulted before descent. Windows junction coverage is declared in the discovery fixture suite; this audit did not execute native platform tests.
- Layout, Raster and Procedural declare owner canonical targets routed through their renderer-contract command. Puzzle's Rust canonical router invokes relocated renderer coverage. Energy and Hub targeted commands route to implementation-owned authority/freshness suites. Root canonical orchestration discovers contributed targets through Nx.
- Root layering routes to the repository-owned live lint target. Searching root script and repository library for `LayeringBaseline`, `LayeringReference`, `loadLayering`, `writeLayering` and `countLayering` returned no matches. This is a scoped source search, not a repository-wide API compilation claim.

No additional concrete actionable finding was identified in the reviewed registration, discovery, alias, vendor, cancellation or baseline-removal paths.
