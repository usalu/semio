# Mandated Script Semantic Ownership Review

## Scope And Method

This is a bounded, read-only ownership review of the 138 current mandatory `📜️script.ts` entries. It uses the coordinator's installed-TypeScript AST snapshot, re-read live examples, and direct import/consumer searches. It makes no source, schema, Git, lifecycle, or generated-output change. Supporting buckets and consumer observations are under `🗑️generated/terra-script-ownership`.

The snapshot parses all 138 sources without syntax diagnostics: 43,357 newline segments; 97 sources below 100 lines and 41 at least 100 lines. Size is only a triage signal.

## Small Forms: 89 Retained Routers, Eight Specific Reviews

Native AST data finds 89 small sources with no top-level functions, types, or exports. Their 198 classes all extend `BundleScript`; the two zero-class cases are direct artifact task launchers. These are legitimate task routers, not an extraction backlog merely because the policy's current closed grammar cannot prove every routing shape. They retain the mandated executable leaf, argument parsing, cancellation/budget propagation, and imported-tool invocation.

The remaining eight small sources require individual dispositions:

| Script | Evidence | Disposition |
| --- | --- | --- |
| `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts` | Local paths, isolated typegen preview and Cargo invocation; no export or external consumer. | Retain as generator-task glue; consider a later generator-owner split only with its artifact/receipt contract. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts` | Exports `FONT_ASSET` and `validateFontAsset`; OS dev imports both. | Extract the packed-font validation/source constants to an anonymous infinite-font domain leaf; retain the font-tool and staging router. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts` | Exports the pure `defineLint` policy while also routing build/dev/test. | Move policy construction to the coordinator linting owner; router imports it. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts` | Exports `policyFile` and a pure file-lint policy while routing native CLI build/test/dev. | Move the lint policy to the client CLI linting owner; retain executable routing. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts` | `buildExtension` is local task orchestration over existing Vite build config. | Retain. No reusable API or independent consumer was found. |
| `✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript/📜️script.ts` | Fixture schema validation, production-source oracle, and three hostile-source mutations are all inside the router. | Move the oracle and fixture model to the existing mathematical verification/test owner; router calls that owner. |
| `🌎️hub/📦️packages/🟦️typescript/📜️script.ts` | `buildHubBinary` only gates an existing native build for the package's own integration task. | Retain as task glue. |
| `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts` | `warnWhenHubIsUnreachable` is a local preflight for `dev`, followed by imported Vite/test runners. | Retain as task glue. |

The resident Rust script is the important near-boundary positive: it rejects arguments, invokes imported Cargo test infrastructure, then iterates a literal two-target list through the shared runner. The scene script similarly uses a local closure only to issue two literal native compiler checks. These are task control flow, but they do **not** justify a name-, `BundleScript`-, router-, or import-based exemption. No grammar expansion is recommended from these observations alone. Any future structural admission must model literal target loops/local command closures and add adversarial variants that place a domain calculation, declaration, or effect inside the same shape.

## Larger Forms And Consumer Evidence

All 41 larger entries contain local behavior and need source-level ownership review; 17 expose public API from a mandatory command file. The following active consumers establish priority independently of line count:

| Source currently in a mandatory script | Direct consumer evidence | Required source owner |
| --- | --- | --- |
| Styling Rust: `fetchElementsFonts`, catalog/font parsing and rendering | Styling TypeScript router imports `fetchElementsFonts`. | `🖱️ui/🎨️styling` font/catalog/generation owners; preserve the completed build orchestration. |
| Infinite Rust: `FONT_ASSET`, `validateFontAsset` | OS dev imports both to validate staged guest fonts. | `♾️infinite` packed-font validation owner. |
| Assets TypeScript: SVG normalization, icon identifiers/catalog, output manifest, logo keyframes | Path-statute tests compile and exercise `catalogSvgSources` and `logoKeyframePaths` from the command source. | `🖼️assets` catalog and logo generator owners, with tests importing the moved owner directly. |
| Plugin descriptor TypeScript: source epochs, component production, descriptor emission | Hub imports `produceFreshComponentV1`; numerous plugin command routers import `describePluginComponent`; its native tests import the source functions. | `🔌️plugin/🖨️describe` source-epoch, build/staging, and descriptor domains. |
| OS dev TypeScript: registry/session/distribution/browser-host staging | Hub dynamically obtains `stageTestBrowserHostV1`; registry tests inspect `ensurePluginRegistry`; dev imports registry projections/rendering. | Existing `🧑‍💻dev` staging, session, distribution, and browser-host owners. |
| GIS Rust: controlled component/cold-map proofs | Hub imports `proveGisComponentColdMapPatch`. | GIS verification/oracle owner. |
| Hub Rust: fixture expectation, publication oracle, credential delivery | It imports descriptor, registry, browser-bundle and GIS script APIs, while exporting its own oracle/credential primitives. | Hub verification, publication, and credential-delivery owners, split only after current hub feature work is coherent. |

The other large entries fall into three bounded source slices rather than one 41-file move:

1. **Generator rendering:** graph (454 lines; catalog parsing/rendering/output writer), schema (216; entity catalog and TS/Go/Rust renderers), UI (211; UI axes and renderer emitters), and actor (89; typegen helper). Move pure rendering/planning into each existing domain's generated/source owner; leave preview/check/stale-prune command steps in the router.
2. **Verification and oracle bodies:** flow TypeScript (192), process TypeScript (177), mathematical TypeScript (52), block/draw TypeScript (104/109), puzzle TypeScript/Rust (352/302), norm TypeScript/Rust (113/294), energy Python oracle (171), GIS/VCS/space Rust (509/157/581), OS root TypeScript/Rust (695/2143), renderer React/WGPU (894/645), MCP (659), shell (173), and stdio (1,015). Move each production-source parser, hostile mutation checker, oracle, receipt/manifest validator, or reusable process protocol beside its existing `🧪️tests`, `🔮️oracles`, publication, or runtime owner. Keep the test/build command router in `📜️script.ts`.
3. **Platform composition:** OS dev (5,685) and hub Rust (17,245) require separate Sol lanes because they currently combine public build/session/staging APIs with task routing and active feature code. First extract APIs with existing consumers; only then split local command composition. Do not create a generic utility dumping ground.

## Concrete Sol Execution Slices

1. **Font and asset APIs:** infinite font validator/constants, styling font catalog/download/render functions, and asset catalog/logo helpers. Redirect OS dev, styling router, and statute tests to anonymous domain leaves. Preserve native output/receipt and cancellation tests.
2. **Plugin build/publication APIs:** descriptor fresh-source/build/stage/receipt functions, then registry projection/session APIs where active consumers use them. Update every plugin describe router, Hub, dev, and direct test importer in the same slice; do not leave a facade export in `📜️script.ts`.
3. **Generator renderers:** graph/schema/UI/actor pure reading/rendering/planning functions. Keep only `BundleScript` routing, filesystem write/stale-removal, and preview/check execution in package scripts. Verify language-generated byte identity and launch/project registration after each owner move.
4. **App verification libraries:** begin with mathematical, GIS, VCS and energy because their fixture/oracle contracts are bounded and GIS has a Hub consumer. Move hostile mutation and source parsing into their test/oracle owners, then have task routes call them.
5. **Large active compositions:** defer OS dev and Hub Rust until their current source lanes settle. Split by consumer-tested domains (browser host/session/distribution; Hub publication/credential/fixture) and retain the executable router after each extraction.

## Policy Boundary

The review does not endorse a blanket allowance for all 138 mandated names. The existing seven counterexamples must remain rejected: Python import plus domain function; Vitest configuration plus hidden class; root script plus hidden class; unknown Rust registration computation; C arithmetic macro; known Rust registration plus executable call; and C domain-call macro. The 89 router-only forms can be admitted only by a closed structural grammar proven against those cases and new literal-loop/local-closure hostile vectors. All source-bearing entries above remain implementation or implementation-review candidates until their behavior moves to their neutral domain owner.

## Limits

This is a source/AST/consumer review, not a refactor or rerun of the broad body census. Several selected OS/framework paths are changing concurrently; the proposed slices therefore name stable semantic owners and live callers, rather than pinning a transient full-file line count as a final extraction plan.
