# Terra Final Boundary Audit

## Scope

Read-only audit on 2026-09-09 after the native local-router repair settled. I read the acceptance contract, results, previous final-boundary audit, consumption and native-input audits, and all four current ownership ledgers. I inspected current manifests, source roots, Cargo metadata, normalizer and launch seed/generated launch wiring. I did not edit production sources, AGENTS.md, Git state, tickets/goals other than this audit, compile native code, or run a broad Nx task.

## Current Boundary Result

No current packaging-owner, implementation-location, parent-plugin-assembly backedge, exact-law-route, launch-persistence, or local-router-input defect was found in the inspected source snapshot.

`cargo metadata --offline --format-version 1` completed successfully and is retained as `🗑️generated/final-terra-cargo-metadata.json`. The independent classifier accounts for framework-prefixed artifact names and excludes the two non-product Draw support crates. It reports:

| Check | Current result |
| --- | --- |
| Production Rust artifact owners | 99 |
| Shared contracts | 2: stdio and Norm |
| Product-or-contract owners checked | 101 |
| Cargo library paths that remain inside a package declaration directory | 0 |
| Non-generated Rust/TypeScript implementation files in artifact package declaration directories | 0 |
| TypeScript artifact package manifests | 40 |
| Direct or transitive dependency paths to a root parent-plugin assembly package | 0 |
| Artifact-root parent mounts/imports (`crate::artifacts`, `crate::registry`, or a parent plugin symbol) | 0 |

The supporting source/metadata receipt is `🗑️generated/final-terra-static-boundary.json`: inventory and source-location results are at lines 2-19, TypeScript count at line 19, parent-boundary results at lines 63-64, and the default/optional component boundary results at lines 65-69. The two Draw FSM crates are classified as support crates at lines 4-7 rather than product artifact owners. This makes the 99+2 acceptance inventory unambiguous even though seven framework packages use the `semio-framework-artifact-*` name family.

All checked owner manifests have `default = []`. Puzzle's six optional component dependencies are explicitly enabled by `component-app-assembly` in `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml` lines 18-19 and 22-45. FEM's two optional UI dependencies are likewise bound by that feature in `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml` lines 18-19 and 36-37. The static receipt records no nonempty artifact defaults and no missing Puzzle/FEM component bindings.

The parent-plugin test deliberately recognizes only root plugin assembly manifests. It does not misclassify a nested artifact-local support crate, such as Draw FSM, or extension leaf packages as a parent assembly. This is why it checks the intended boundary rather than producing false failures for Sequence's independently owned imperative-extension integration.

## Source And Route Review

The normalizer resolves literal command imports recursively and rejects workspace escapes in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` lines 321-335. The settled native-target helper parses the selected command from its declared working directory and unions that local router/import closure with the existing Cargo/toolchain inputs at lines 353-365. Native build, check and test normalization consumes those inputs at lines 627-635.

The independent schema-validated/esbuild fixture now reports three exact executable files and excludes its sibling (`🗑️generated/native-local-router-input-green.txt`, line 1). The ordinary project proofs report the same result for Gismap with its Terrain sibling excluded (`🗑️generated/gismap-native-router-input-proof.txt`, line 1) and PDF with its JPG sibling excluded (`🗑️generated/pdf-native-router-input-proof.txt`, line 1). The older omission reported in `📓️native-local-router-input-audit.md` lines 3-9 is therefore resolved; its lines 11-17 preserve the correction history and current ordinary-proof status. The broader PDF/JPG production-root closure refresh belongs to the coordinator and is not inferred from these focused router proofs.

Flow’s twelve leaf laws select `semio-s-artifact-flow-flow` at `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts` lines 35-45, 60-62, and 175-178. The sole parent-composition law stays on `semio-s-plugin-flow` at lines 46-48. GIS selects the Gismap owner for the map and durable assembly leaf laws at `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts` lines 439-475; its two codec/controlled-proposal laws intentionally target the parent component test at lines 487-499. Norm routes the mutation contract to `semio-s-artifact-norm-contract` at `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts` lines 166-170 and the parent-owned surface integration to `semio-s-plugin-norm` at lines 216-220. These are owner-correct routes, not the retired parent-package routing that the prior audit recorded.

The persisted launcher is present in both `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`: the cross-package contract at lines 774-781 and the parameterized package target at lines 785-788. Each file also retains the paired project/target inputs at seed lines 14345-14346 and generated-launch lines 28270-28271.

## Evidence Status

Observed source/evidence results should not be promoted to runtime acceptance:

| Status | Evidence |
| --- | --- |
| Observed | Current Cargo/manifest/source ownership, declaration-only package directories, parent assembly direction, Puzzle/FEM component boundaries, law routing, launch seed persistence, local-router closure fixture, and ordinary Gismap/PDF target-input proofs. |
| Runtime observed | Norm’s repaired Unicode-chunk law passed through ordinary Nx: 1 passed and 19 skipped; `📓️results.md` line 28. |
| Pending | Fresh full production-root PDF/JPG closure capture and native PDF cache restoration remain pending; `📓️results.md` lines 21 and 32. |
| Pending | GIS default/component runtime acceptance, Block focused parent retry, Norm surface retry, Flow exact native-law execution, and remaining multi-artifact/parent gates remain pending; `📓️results.md` lines 25-26 and 40-41. |

The STEP global-preflight backlog is outside this audit's packaging boundary. No owner/path-resolution defect was observed here, so it is not counted as a packaging finding.

## Raw Evidence

- `🗑️generated/final-terra-cargo-metadata.json`
- `🗑️generated/final-terra-static-boundary.json`
- `🗑️generated/native-local-router-input-green.txt`
- `🗑️generated/gismap-project-native-router-current.json`
- `🗑️generated/gismap-native-router-input-proof.txt`
- `🗑️generated/pdf-project-native-router-current.json`
- `🗑️generated/pdf-native-router-input-proof.txt`

## Coordinator Follow-Up After Audit

The coordinator refreshed the full production-root oracle against the ordinary post-router graph. The conservative closure covers 31 projects, 34 target invocations and 2,322 unique positive input patterns, with zero unresolved groups or targets. Independent minimatch accepts the actual PDF production root and rejects the actual JPG production root; both the local PDF router and shared Rust artifact router are included. All candidate paths were checked to exist. Graph SHA-256: `816ac778bc18db7d6f116c7026ebb23408c00365ff989060e03b2302d4e4c372`. Receipts: `pdf-native-production-isolation-post-router.json` and `pdf-native-production-isolation-post-router.txt`, exact exit 0. This remains structural evidence; cache-hit restoration is pending.
