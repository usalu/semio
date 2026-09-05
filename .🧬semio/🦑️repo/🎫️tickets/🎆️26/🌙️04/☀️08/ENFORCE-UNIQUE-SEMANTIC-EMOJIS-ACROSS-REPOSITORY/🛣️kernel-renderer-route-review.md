# Kernel and Renderer Module Route Review

## Coordinated scope

The parent approved the current public route identities `/🔌️plugin-modules` and `/🧩️extension-modules` so copied distribution directory names remain meaningful and sibling-distinct. Public plugin and extension IDs do not change. The OS owner is implementing one strict deployment route authority and rejection tests; this review owns only the framework kernel, framework TypeScript tests, renderer consumers, and WGPU exact compiler inputs/current output refresh.

The parent selected explicit watch-URL injection: OS route authority stays OS-owned, and the neutral framework kernel does not import product configuration. Both factory signatures now require a `watchUrl: string`; their `EventSource` instances use that exact argument. The former literal watcher constants were removed, with no default, alias, or compatibility branch.

## Exact current consumers

Within `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`, the real consumers are `PLUGIN_SOURCE_WATCH_PATH` (`/plugin-modules/watch`) and `EXTENSION_SOURCE_WATCH_PATH` (`/extensions/watch`), each used by its `EventSource`. Six accompanying current namespace documentation lines also require exact correction.

The language-neutral kernel descriptor fixture at `🧫️fixtures/📇️descriptor-load/🔣️.json` owns one module URL and its expected descriptor URL. These are test data, not historical provenance.

`🧰️framework/📦️packages/🟦️typescript/🟦️.ts` contains the current mock plugin/extension catalog URL builders, boot fixtures, and PluginSource URL assertions. Mock public IDs and mock payload basenames remain unchanged; route values must match the single authority. URL pathname checks must account for normal browser percent-encoding of Unicode rather than changing the intended route.

Within renderer `🧑️‍🎨️engine`, `🧱️elements/🏛️ShellHost/🟦️.tsx` owns two extension install POSTs and one uninstall DELETE, currently under `/extensions/install`, plus current namespace documentation. `📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts` owns two mock descriptor-load URL strings. No other authored `/extension-modules` consumer was found in these scopes. Actor shard-worker routes and generated module mirrors are explicitly owned by the parent/OS agent, not this review.

The WGPU current package profile and central compiler input list now both include the exact `🔌️plugin/📇️registry/📦️deployment/🛣️routes.json` authority path in byte-sorted order. `loadTaxonomy` passed. The frozen historical package witness remains untouched. Generated workers will be rebuilt through their normal producer only after the OS owner confirms source, authority, and external consumer paths are stable.

Read-only inventory evidence is retained under `🗑️generated/metabolism-glb/kernel-renderer-route-inventory.txt`; follow-up inspection also included the older `/extensions` namespace. No bulk substitution, alias, compatibility route, or endpoint fallback is authorized.

## Implemented exact boundary changes

ShellHost imports the OS deployment route constants directly. Its two factory calls supply `${MODULE_PLUGIN_ROUTE}/watch` and `${MODULE_EXTENSION_ROUTE}/watch`; its two install POSTs and one uninstall DELETE use `${MODULE_EXTENSION_ROUTE}/install`. No neutral module imports these constants or their JSON source. Its stale namespace documentation was corrected in place.

The two kernel descriptor-fixture URLs, two renderer descriptor-test URLs, and individually inspected framework mock/expectation URLs now use the new public namespaces. Browser `URL.pathname` assertions decode the normal Unicode percent-encoding before comparing the expected logical route; cache-busting and public identity assertions remain intact.

A read-only scan of authored kernel, framework-package, and renderer files (excluding normal generated workers and catalog output) finds no former `/plugin-modules` or `/extensions` path or removed watcher-constant reference.

## Test-driven evidence

New language-neutral input `🎠️kernel/🧫️fixtures/📡️source-watch.json` declares distinct relative and absolute adapter watch URLs; its hand-authored sibling `📐️source-watch.schema.json` defines the record shape. These names are purpose-specific and distinct from both existing sibling directories (📇 descriptor loading and 🚪 turn ownership). No new directory or taxonomy exception was needed.

The permanent framework test validates this input independently with Ajv, observes each actual factory's `EventSource` constructor argument, and verifies unsubscribe closes that exact stream. Before implementation it failed with `/plugin-modules/watch` instead of the injected `/services/catalog-feed`; the other 88 tests passed. After implementation all 89 framework tests pass. Evidence: `kernel-watch-red.log` and `kernel-watch-green.log` under the ticket-generated directory.

Fresh follow-up: all 49 kernel quick tests pass. The renderer's explicit descriptor-focused long target passes 16 tests; 521 unrelated tests are filtered/skipped, so this is not a full-suite claim. Those commands both exited successfully through Bun/Nx. Evidence: `kernel-route-quick.log` and `renderer-route-focused.log`. The four immediate fixture siblings have zero path-emoji findings.

After both OS and parent mirror all-clears, all four normal producer/freshness targets completed successfully: `@semio-tech/framework-os:generate-wgpu` (six exact artifacts, one changed), `@semio-tech/framework-os:check-wgpu` (six exact artifacts, zero changed), `@semio-tech/framework-renderer-wgpu:generate-frame-worker`, and `@semio-tech/framework-renderer-wgpu:check-browser-worker`. No generated worker was hand-edited and no frozen package witness was rewritten. Evidence: `wgpu-route-generate-retry.log`, `wgpu-route-check-final.log`, `wgpu-route-frame-worker-final.log`, and `wgpu-route-browser-check-final.log`.

The later parent-approved Trunk follow-up sets its plugin copy target to `🔌️plugin-modules` and asset copy target to `🖼️assets`. The asset source now directly resolves to `🧰️framework/🔨️modules/🖼️assets`, exactly matching the actual `resolveSemioAssetRoot` used by the Vite copier. The stale dependency on `dev/dist/asset` is removed; no Dev distribution directory was moved. Both source directories exist. The normal WGPU generate and check targets passed again with six exact artifacts and zero changes (`wgpu-trunk-source-generate.log`, `wgpu-trunk-source-check.log`). No Trunk copy/build was run during this source-boundary validation.
