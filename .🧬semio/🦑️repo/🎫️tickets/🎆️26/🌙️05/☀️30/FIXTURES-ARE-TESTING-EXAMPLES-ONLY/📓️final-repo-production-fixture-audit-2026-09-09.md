# Final Repo Production Fixture Audit

## Result

This bounded read-only audit found **four actionable edges** in the workspace root, repo library, and Hub boundary. Two language-neutral JSON catalogs under the repo library's `🧫️fixtures` are synchronously loaded as production normalization authority. Hub has no direct runtime or manifest fixture dependency, but its cold default development startup blocks trusted-catalog publication on a Cargo test whose binary embeds a GIS fixture. A separate Nx edge makes 46 caching fixtures inputs to production cache keys without compiling, copying, or serving their bytes.

The machine-readable findings are in `🗑️generated/framework-final-audit/findings.json`. The bounded literal census is in `🗑️generated/framework-final-audit/literal-fixture-path-candidates.json`; its 61 current literal paths all existed when captured, but presence is not reachability proof.

## Actionable production reads

### Repo normalization authority stored as fixtures

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:869-870` declares two exact fixture paths:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json`, 3,664 bytes, SHA-256 `c67b436abca4b9005efce6023f74bd900de5e88187ee9eb3c4328c2397bd081b`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/💉️ticket-important-exact-mutations/🔣️.json`, 1,733 bytes, SHA-256 `a5012fa365145d27001fdd21d87e072f2b7a0d7992d017b7753390febf24a195`.

These are active authority inputs rather than example coordinates. `ticketImportantExactMutationCases` at `:6338` resolves, stats, reads, parses, and validates the second file; its results authorize exact mutations in projection/planning at `:6427`, `:7592`, and `:7684`, then validate authority at `:8913`. `serializedSentinelCases` at `:7616` resolves, stats, reads, parses, and hashes the first file; planning consumes it at `:7639`, validation consumes it at `:9301`, and apply revalidates it at `:11056`.

The root clean command reaches these consumers through `planTaxonomy` at `📜️script.ts:16263` and `applyTaxonomyPlan` at `:16294`. Their bytes therefore control normal cleanup planning and application. Both catalogs should move byte-identically to the nearest semantic `🖼️assets` owner, keeping the same semantic leaf names, and their constants and dependent identity fields should be updated together.

### Hub cold-start publication depends on fixture bytes

This is an indirect edge whose role matters. Running a test as a production qualification step can be intentional; the defect is that the mandatory decision depends on bytes classified as a test-only fixture.

The default Hub script command is `dev` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:17134`). On a missing current trusted catalog, `DevScript` at `:12539` materializes a candidate at `:12562` and awaits `validateAndPublishTrustedStdioGisCandidate` at `:12565`. That validator calls `proveTrustedGisColdMapComponentV1` before staging or publication at `:10890`. The proof invokes `runExactCargoLaws` with target `{ kind: "test", name: "component_cold_map_patch" }` at `:10208-10223`, while passing the freshly built component and descriptor identity through `SEMIO_GIS_COMPONENT_WASM` and `SEMIO_GIS_DESCRIPTOR_SHA256`.

The selected target is declared in `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:66-68`. Its source, `✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs:27`, embeds `✏️s/🔌️plugins/🌍️gis/🧫️fixtures/🌉️component-cold-map-patch/🔣️.json` with `include_str!`. The fixture's before/after documents drive both the positive cold-load law at `:256` and stale-authority rejection at `:273`. The file is 1,404 bytes with SHA-256 `2138e94143609cdbbd268f2fdfb84a3fe17dd627229062c0fcd1bf3975a28250`.

Failure prevents publication, so this is a real development/production qualification dependency on fixture data. Preserve the qualification boundary, but move the executable proof needed by publication to a production qualification/support owner. If the fixed before/after document remains the published qualification input, classify it as that owner's `🖼️assets`; keep hostile examples used only by canonical tests in `🧫️fixtures`.

## Production cache-key edge

Root `nx.json:13` includes the entire repo library `⚡️caching/**/*` tree in `sharedGlobals`. The dynamic project plugin repeats that broad runner input at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:528`. Its `production` group excludes project and delivery-owner fixtures at `:550`, but does not exclude fixtures inside the shared runner tree. The current runner subtree contains 46 files under `🧫️fixtures`.

Production-oriented component materialization and distribution targets consume `production` and `^production` at `🟨️.mjs:723` and `:792`. This makes the 46 fixture files production cache inputs. It is a cache invalidation dependency only: this audit found no evidence that those bytes are compiled, copied, served, or loaded by the product through this edge. Add a production-only exclusion for the runner caching fixture subtree, while retaining the fixtures in test inputs.

## Cleared boundaries

The Hub runtime boundary is otherwise clean in the inspected graph:

- A scan of non-test Hub Rust sources, excluding the Hub test/check script and fixture trees, found no fixture path literal or filesystem read.
- Hub Cargo manifests, package manifests, Nx projects, TypeScript configs, and inspected config files contain no fixture executable/static-directory route. The existing 457-manifest census in `📓️fixture-manifest-dependencies-2026-09-09.md` independently reports zero selected fixture package or static-asset dependencies.
- `🌎️hub/📦️packages/🦀️rust/📋️project.json:36` builds `os-hub` through the Cargo cache runner rather than the fixture-reading Hub script. Dynamic Cargo build/check targets use `nativeSources`, whose test/fixture boundary is separate from the broad TypeScript production group.
- Hub's `test-support` feature is absent from the default feature list in `🌎️hub/📦️packages/🦀️rust/Cargo.toml:22-35`. The OS `directory` dependency with `features = ["testkit"]` is a dev-dependency at `:78`. No workspace manifest enables `semio-hub/test-support`. The module at `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1323-1324` is feature-gated support and contains no repository fixture read. It remains opt-in test support rather than a default/transitive production fixture edge.
- Hub's 51 exact literal fixture roots in `🌎️hub/📦️packages/🦀️rust/📜️script.ts` belong to explicit `*-check`, oracle, smoke, or process-test functions. Merely declaring those functions at module scope does not read their paths. The one exception with demonstrated `dev` reachability is the indirect GIS cold-map qualification above.

Root `VerifyScript` consumes fixed-operation, shared-action, ownership, Puzzle fill, and similar fixtures only in verification branches. `SchemaScript` treats the schema export entries dump as tracked expected output for schema test/entries commands. These are test/check consumers rather than product loaders.

The 70 fixture-like strings currently visible in `🔣️taxonomy.json` are mostly semantic directory vocabulary, fixed generated-output identities, generator output roots, or four frozen example-coordinate contracts. `frozenCoordinateEvidenceCoordinates` declares coordinates and hashes; the normalizer compares inventory bytes when such an example is encountered. Those declarations are active normalization support for examples, but they do not open fixture files as runtime authority. Likewise, JCO filenames and analyzer paths describe generated/test coordinates; a string alone is not a dependency. JCO coordinates were changing concurrently under the parent's independently owned relocation and were not audited as stable final paths here.

`fixtureItemsOf` at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:128` is an uncalled generic in-memory `{items}` accessor. It performs no import, path resolution, or filesystem read, so its name is not a production fixture finding.

## Method and limits

The audit inspected the current call chains for direct and indirect reads, root/Hub script routing, Cargo targets and features, Nx named inputs and target policies, manifest/static-directory configuration, taxonomy coordinate roles, and the earlier runtime-taxonomy-assets report. It did not rerun the parent's active global layout scan.

This was static and read-only. No production source or manifest was edited, and no build or runtime command was executed. Computed paths assembled outside the inspected call chains and behavior inside third-party tools were not dynamically traced. The shared working tree changed during inspection, notably the independently owned JCO relocation, so paths and line numbers describe the 2026-09-09 snapshot. The generated context files are audit evidence, not retained inputs.
