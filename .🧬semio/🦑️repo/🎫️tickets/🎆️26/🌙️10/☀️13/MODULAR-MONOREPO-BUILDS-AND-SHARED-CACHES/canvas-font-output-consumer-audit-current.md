# Canvas and Font Output Consumer Audit

Source inspection only on 2026-10-15. No builds, runtime checks, cache inspection, or Source edits. Paths below are repository-relative. Read `product-artifact-caller-relocation-current.md` first.

## Canvas Producer and Consumers

`🧰️framework/🔨️modules/🖼️canvas/🔌️runtime-assets/🚚️staging/🟦️.ts:16` fixes `RUNTIME_ASSET_STAGE_ROOT` to Canvas Rust `dist`. Its `runtimeAssetStageTrees()` produces six port/guest roots; `canvasRuntimeAssetSources(workspace)` prepends workspace. Supply the actual product root as an argument, without repository cache authority in this domain-neutral framework module.

`🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust/📜️script.ts:20-24` directly stages each tree through the old publication API. Its `📋️project.json` stage-runtime-assets outputs still name `{projectRoot}/dist/port-assets` and `dist/guest-assets`. Inputs cover owning script, runtime channel contract, staging module, compiler fonts, MapLabelSans, and dependent generated shortcode JSON; the compiler font list comes through imported `📚️compiler/🌍️world/🟦️.ts`, which is not explicitly among these inputs. Include the path/ownership binding and transitive channel-definition module when invalidating relocated output.

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🟦️.ts:49` is the production source resolver using `canvasRuntimeAssetSources(workspace)`. It already has `plan.profile`; pass the same explicit mode-specific products root as the producer. `🧰️framework/🔨️modules/🖼️canvas/🔌️runtime-assets/🧪️tests/📦️bundle-staging/🟦️.ts:28,33,41,83,85` must change fixture roots and function arguments together.

Canvas browser URLs remain declared by `🔌️runtime-assets/🟦️.ts`: `/renderer-modules/compiler-fonts`, `/renderer-modules/canvas-fonts`, `/renderer-modules/icon-pack`, plus guest package destinations `/renderer-modules/package-assets/<repository source directory>`. `runtimeAssetDeliveries()` intentionally serves original source directories for dev; it does not consume old staged dist roots.

## Infinite Font Producer and Consumers

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts:17-29` resolves the font executable at Canvas fonts Rust `dist/build`, uses Infinite `dist/.fonts-*` scratch, and publishes `dist/fonts`. All three must move: exact published Rust artifact reader for the tool, managed diagnostics scratch for tool output, products publication for validated font bytes. Retain Rust backing plus exact executable artifact shared ownership until `runRepositoryCommand` settles; a returned path alone cannot preserve this lifetime.

The Infinite `📋️project.json` fonts target declares old `dist/fonts`. Its only direct source inputs are its script and Canvas font validator; dependency output wildcard is `**/*`, with `semio-framework-canvas-font-assets:build` prerequisite. Canvas font tool `📋️project.json` itself still declares `dist/build`. Update both output metadata consistently with managed native/product semantics; include mode and product binding/reader implementation in the relevant invalidation surface.

All old font consumers found in actual OS Source:

- `🧑‍💻dev/🚚️distribution/🔌️components/🟦️.ts:48`: production owner `infinite:fonts`, destination plugin route plus vendor directory; use `plan.profile` to choose products.
- `🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts:64`: directly reads and validates the old font file for readiness.
- `🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts:75`: support digest map directly references old font file; retain font publication while hashing and all later effects that read its bytes.
- `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:57,243`: `fontsDir` is old dist and a static directory mount under `${MODULE_PLUGIN_ROUTE}/${MODULE_VENDOR_DIRECTORY}`. A dev server's file reads continue after configuration returns; the managed font owner must stay retained through serve lifetime, or copy into an independently retained serve owner before mounting.
- `🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts:94-108`: `guestSlimTypstFontSeed(repoRoot)` returns old path; `ensureGuestSlimTypstFontsAt` synchronously checks and copies it without ownership, and short-circuits on any existing nonempty output. Bind explicit mode and actual owner; retain through copy, and validate freshness rather than trusting nonempty previous bytes.
- `🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:47`: caller of that seed-copy function must propagate original invocation and mode instead of only workspace.

Paths in the preceding list are under `🧰️framework/🛍️products/💻️os/🔨️modules/`.

## Reader Boundary and Editor Routes

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/🟦️.ts:18` acquires products storage and exact `artifact:<path>` shared lease, validates marker/inventory/hashes and provides a live callback-scoped recheck. Consumers must perform file reads/copies inside that callback; passing resolved roots outside it loses protection. Production source descriptors are data only and need the actual copier to retain ownership during copying.

`.vscode/launch.json:46541-46556` exposes canvas-font-assets build through `workspace:build-canvas-font-assets` and Canvas runtime asset staging through `semio-framework-canvas:stage-runtime-assets`. They dispatch Nx targets rather than raw dist paths; preserve these routes while correcting target metadata. No new executable route is required merely to relocate outputs.

Third-party KaTeX `node_modules/katex/dist/fonts` distribution-layout entries are unrelated and must remain as package-owned inputs. Native/SDK/general publication and full-graph audit are outside this census.
