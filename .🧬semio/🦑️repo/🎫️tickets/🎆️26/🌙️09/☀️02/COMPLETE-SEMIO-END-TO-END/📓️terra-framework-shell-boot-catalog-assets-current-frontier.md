# Framework OS Shell Catalog And Asset Boot Frontier

## Result

`FrameworkOsShell` on the current `s` session has three independent boot blockers. The reported five extension descriptor failures are real, but they are a subset of a 59-entry host catalog frontier:

| Current condition | Entries | Consequence |
| --- | ---: | --- |
| Full runtime module (bridge, host shim, component JS, extracted core WASM, descriptor JSON) | 37 / 59 | Could pass the file-presence admission layer. |
| Descriptor JSON absent | 21 / 59 | `fetchDescriptorManifest` receives the shell's HTML fallback for a missing JSON asset. |
| Module incomplete even before descriptor admission | 3 / 59 | `draw` and `layout` have no output directory; `energy` has a JSON/shim but no bridge/component JS/core WASM. |
| Present descriptor whose runtime `manifest.dependencies` disagrees with the generated Cargo dependency graph | 38 / 38 present descriptors with non-empty generated dependencies | `AppRouter.build` cannot authorize cross-plugin app surfaces from descriptor bytes. |

The exact current catalog contains 33 plugins and 26 extensions. Since `s` is the sole host configuration, its session is intentionally the all-59 host fleet, not a five-extension program. The host behavior is source-backed by [`🧩️plugins.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins.ts:41) and [`resolvePlaygroundBoot`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2914).

No build was run and no product source was changed for this audit.

## Authorities And The Actual Route

There are four distinct owners; copying JSON to a random served directory is not a valid correction.

1. The hand-authored deployment catalog maps a public plugin ID to its only legal physical directory: [`catalog.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json:1), parsed and enforced by [`deployment/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts:35). The catalog must not be used as an availability claim.
2. A package owner's freshly described `🔣️.json` and `🛂️.descriptor.semio` are the descriptor authority. The materializer writes the descriptor at the crate owner root after probing the just-built component ([`script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:332)).
3. The runtime asset authority is the sibling directory `🧑‍💻dev/🔌️plugin-modules/<declared-directory>/`. `stagePluginDescriptor` deliberately copies the owner descriptor there, and deliberately removes an old output descriptor if its owner has none ([`script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:201)). It is the correct synchronization seam, not an invitation to manufacture output-only metadata.
4. An installed extension has a second runtime root, `🧑‍💻dev/🧩️extension-modules/<declared-directory>/`, atomically mirrored by `publishBuiltExtension` ([`script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:244)). Its canonical URL is `/🧩️extension-modules/<directory>/🌉️bridge.js`; a built extension's staging directory is not its final runtime registration.

The renderer derives `🔣️.json` from the chosen bridge URL and rejects HTML explicitly ([`fetchDescriptorManifest`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:108)). Vite mounts the declared module tree at `/🔌️plugin-modules` ([`vite.config.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts:154)). Its generic static middleware calls `next()` when a requested child does not exist ([`styling/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts:1418)), which reaches the SPA handler and produces the observed HTML response. The client is fail-closed, but this masks a missing asset as an HTML document.

## Captured `s` Session Is Stale And Routes Extensions To The Wrong Tree

The captured generated session at [`🤖️generated/🟦️session.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🟦️session.ts:1) was last modified at `14:17:35+0200`; its generator was modified later, at `14:21:28+0200`. The session currently has both defects:

- Every extension is emitted with a `/🔌️plugin-modules/...` URL (for example, the reported robotic, wood, beams, slabs, and windows entries), rather than `/🧩️extension-modules/...`.
- Its `PlaygroundSessionPlugin` shape and every row omit `dependencies` altogether.

The *current* generator already contains the intended role-aware URL and Cargo-derived dependency projection in [`buildPlaygroundSession`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:893), and its validation explicitly checks both fields ([`script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1867)). The current output is simply older than that generator.

This matters because `resolvePlaygroundBoot` returns a same-variant generated session without rebuilding from `PluginCatalog` ([`kernel/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2914)). Thus the stale session, not the current role-aware catalog constructor, determines the live extension URL. Regenerate the session through the existing registry owner before restarting the shell; do not hand-edit generated output.

## Why Demonstrator Fails `AppRouter.build`

The current generated registry accurately records demonstrator's direct Cargo fleet:

```text
demonstrator -> cad, gis, procedural, process, puzzle, sourcing, stdio
```

See [`🧩️plugins.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins.ts:51). The descriptor served from both the demonstrator owner and runtime directory has *no* `manifest.dependencies` field. This is not a catalog generation error: the demonstrator builder calls no `.depends_on` at all ([`manifest/🦀️.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs:47)), although the builder exposes that exact API and writes it into `PluginManifest` ([`builder/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:305), [`builder/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:707)).

`ShellHost` feeds only loaded descriptor manifests into `AppRouter.build` ([`ShellHost/🟦️.tsx`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5087)); it does not substitute registry dependencies. `AppRouter` therefore sees a demonstrator contribution for `s.cad.cad@1/*#editor`, an already-owned CAD artifact kind, and correctly rejects it as unauthorized ([`kernel/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:604)).

The same split exists in all 38 currently published descriptor JSON files whose generated target has a nonempty `dependsOn`: each descriptor has an empty dependency list, while all 38 generated targets name at least one direct dependency. This is a descriptor-authoring debt, not a reason to inject dependencies in the browser or amend generated registry rows.

## Complete Current Asset Frontier

`plugin-modules` is the build/staging root. The matching installed-extension mirror currently lacks the same 15 extension descriptors, so merely regenerating the session will correctly change their route but will not make those fifteen loadable.

### Descriptor Missing (21)

| ID | Runtime directory | Package-owner descriptor that must be produced |
| --- | --- | --- |
| block | `🧱️block` | `✏️s/🔌️plugins/🧱️block/🔣️.json` |
| draw | `🖍️draw` | `✏️s/🔌️plugins/🖍️draw/🔣️.json` |
| flow-extension-bim | `🏘️flow-extension-bim` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🔣️.json` |
| flow-extension-draw | `🎨️flow-extension-draw` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🔣️.json` |
| imperative-extension-control | `🎮️imperative-extension-control` | `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/🔣️.json` |
| imperative-extension-effect | `📣️imperative-extension-effect` | `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/🔣️.json` |
| imperative-extension-logic | `⚖️imperative-extension-logic` | `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/🔣️.json` |
| imperative-extension-math | `➕️imperative-extension-math` | `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/🔣️.json` |
| imperative-extension-text | `🔡️imperative-extension-text` | `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/🔣️.json` |
| layout | `📏️layout` | `✏️s/🔌️plugins/📏️layout/🔣️.json` |
| playbook | `📖️playbook` | `✏️s/🔌️plugins/📖️playbook/🔣️.json` |
| playbook-module-procedural | `⚙️playbook-module-procedural` | `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🔣️.json` |
| process-extension-concrete | `🏙️process-extension-concrete` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/🔣️.json` |
| process-extension-metal | `🔩️process-extension-metal` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/🔣️.json` |
| process-extension-robotic | `🤖️process-extension-robotic` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/🔣️.json` |
| process-extension-wood | `🪓️process-extension-wood` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/🔣️.json` |
| sourcing-module-beams | `🪜️sourcing-module-beams` | `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/🔣️.json` |
| sourcing-module-slabs | `🧇️sourcing-module-slabs` | `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/🔣️.json` |
| sourcing-module-windows | `🪟️sourcing-module-windows` | `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🔣️.json` |
| stdio | `🗄️stdio` | `✏️s/🔌️plugins/🗄️stdio/🔣️.json` |
| trinity | `🔱️trinity` | `✏️s/🔌️plugins/🔱️trinity/🔣️.json` |

### Module Missing Or Partial (3)

| ID | Current state | Required generated runtime files |
| --- | --- | --- |
| draw | `🖍️draw/` absent | bridge, host shim, component JS, extracted core WASM, descriptor JSON/pack |
| layout | `📏️layout/` absent | bridge, host shim, component JS, extracted core WASM, descriptor JSON/pack |
| energy | descriptor JSON and host shim only; bridge/component JS/core WASM absent | bridge, component JS, extracted core WASM (then descriptor must be refreshed against those bytes) |

## Smallest Coherent Home Handoff

1. Regenerate the current `s` session through the existing registry script before a new shell start. This supplies extension `/🧩️extension-modules` URLs and dependency rows. Do not patch `🤖️generated/🟦️session.ts`.
2. Fix the immediate typed manifest bug in the demonstrator source builder: declare its seven direct dependencies (`cad`, `gis`, `procedural`, `process`, `puzzle`, `sourcing`, `stdio`) with the existing builder API; then regenerate its descriptor from the built component and stage it with `stagePluginDescriptor`. The package descriptor must be authoritative; neither `ShellHost` nor the browser should merge in generated registry facts.
3. For the all-fleet host, produce the 21 listed owner descriptors and materialize the three incomplete modules. Only after a source owner descriptor exists should the existing staging/mirroring routines copy it to runtime roots. Run extension mirroring after descriptor staging so `/🧩️extension-modules` contains the exact same accepted descriptor bytes.
4. Add a readiness preflight over the *selected session rows*, not a directory scan: plugin rows need the five runtime files; extension rows need the same files in their installed root; each descriptor's `manifest.pluginId` must match its row; and its dependency ID list must equal the generated Cargo `dependsOn` list. Refuse to start a full `s` session when this assertion fails. The preflight must use declared module IDs and paths, never discover arbitrary directories.
5. Harden only the plugin/extension descriptor routes so a registered module path ending in `🔣️.json` returns `404` when missing rather than falling through to the SPA HTML page. Keep the kernel's existing HTML rejection as defense in depth. This makes an availability failure observable but is not a substitute for descriptor production.

## First Acceptance Laws

1. A freshly generated `s` session has all 59 rows; each extension row's URL begins `/🧩️extension-modules/`, and every row carries the exact generated dependency vector.
2. The demonstrator native descriptor law asserts the seven runtime `manifest.dependencies` exactly and `AppRouter.build([cad, demonstrator])` accepts the CAD editor contribution with CAD first.
3. A full `s` asset preflight rejects the current 22 missing/partial rows by exact ID, then accepts only when all declared files and owner identities exist in the correct plugin/extension runtime roots.
4. An HTTP Vite law requests a missing registered descriptor and receives `404`/non-HTML; a valid descriptor receives `application/json`, has its matching `manifest.pluginId`, and is accepted before module import.
5. An extension-mirror law proves the descriptor bytes at `plugin-modules/<directory>` and `extension-modules/<directory>` are identical after a staged extension publication, with no raw `/🔌️plugin-modules` URL retained in the generated `s` session.

