# Generated Source Taxonomy Execution

Date: 2026-09-12

## Result

The bounded persistent generated-source lane now has 29 implementation identities under semantic directories with anonymous, file-kind-only leaves. Every renamed output is produced at its canonical path, its old physical path is absent, and its live source consumers point at the canonical path. The portable fixture covers all 29 identities and is checked against both the repository taxonomy implementation and Ajv as an independent schema oracle.

The only tracked generated output in this set is the styling palette CSS. Its tracked move is represented by deleting `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette.css` and adding `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css`. The other 28 outputs remain generated and ignored. The styling Rust, Python, and C# outputs have exact ignore entries at their neutral paths; outputs below `🤖️generated` remain covered by the existing generated-root ignore.

## Exact output moves

| Contract | Previous path | Canonical path | Disposition |
|---|---|---|---|
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs` | ignored |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes.ts` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes/🟦️.ts` | ignored |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🐍️icons.py` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🖼️icons/🐍️.py` | ignored |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔷️Icons.cs` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🖼️icons/🔷️.cs` | ignored |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🟦️icons.ts` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🖼️icons/🟦️.ts` | ignored |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🤖️generated/🟦️metabolism_icons.ts` | `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🤖️generated/🖼️icons/🟦️.ts` | ignored |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🤖️generated/🦀️metabolism_icon_name.rs` | `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs` | ignored |
| `actor-typegen` | `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🟦️actor.ts` | `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🎭️actor/🟦️.ts` | ignored |
| `ui-contract` | `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract.ts` | `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` | ignored |
| `framework-manifest` | `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest.ts` | `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` | ignored |
| `ui-axes` | `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes.ts` | `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts` | ignored |
| `graph-catalog` | `🧰️framework/🔨️modules/🕸️graph/🤖️generated/🦀️registry.rs` | `🧰️framework/🔨️modules/🕸️graph/🤖️generated/📇️registry/🦀️.rs` | ignored |
| `graph-catalog` | `🧰️framework/🔨️modules/🕸️graph/🤖️generated/🔠️types.ts` | `🧰️framework/🔨️modules/🕸️graph/🤖️generated/🔠️types/🟦️.ts` | ignored |
| `schema-entity-catalog` | `🧰️framework/🔨️modules/🧬️schema/🤖️generated/🟦️entity-kinds.ts` | `🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/🟦️.ts` | ignored |
| `async-typegen` | `🧰️framework/🔨️modules/⏳️async/🤖️generated/🟦️async.ts` | `🧰️framework/🔨️modules/⏳️async/🤖️generated/⏳️async/🟦️.ts` | ignored |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🚦️palette-presence.css` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🚦️palette-presence/🎨️.css` | ignored |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🔤️palette-fonts.css` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🔤️palette-fonts/🎨️.css` | ignored |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🌓️palette-theme.css` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🌓️palette-theme/🎨️.css` | ignored |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🟦️tokens.generated.ts` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🔤️tokens/🟦️.ts` | ignored |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette.css` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css` | tracked |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🦀️tokens.generated.rs` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs` | ignored exactly |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/🎨️styling/🤖️generated.py` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🐍️.py` | ignored exactly |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔷️net/🖌️Elements.Styling/🤖️Generated/🎨️Palette.g.cs` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🔷️.cs` | ignored exactly |
| `plugin-registry` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🏗️framework.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🏗️framework/🟦️.ts` | ignored |
| `plugin-registry` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🖥️hosts.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🖥️hosts/🦀️.rs` | ignored |
| `plugin-registry` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts` | ignored |
| `plugin-registry` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts` | ignored |
| `plugin-registry` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🗿️artifacts.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🗿️artifacts/🦀️.rs` | ignored |
| `playground-session` | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🟦️session.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🎮️playground-session/🟦️.ts` | ignored |

## Producer and contract changes

- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts` and `📋️project.json` emit the seven asset identities, create their semantic directories, remove only producer-owned stale paths, and retain the separately owned shortcode JSON.
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts` and `📋️project.json` emit the actor type projection under `🎭️actor`.
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📜️script.ts` and `📋️project.json` emit the async type projection under `⏳️async`.
- `🧰️framework/📦️packages/🦀️rust/📜️script.ts`, `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts`, and `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts` emit the three manifest projections. The framework Cargo manifest's generated-source coordinate was updated with them.
- `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts` emits the graph registry and type projections. Its output manifest, schema, portable fixture, and suite use the two semantic paths.
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts` emits the TypeScript entity-kind projection under `🏷️entity-kinds`. The same shared producer's Go output was moved and verified by the Go execution lane.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts` emits all eight styling identities at their canonical paths and removes its prior files and now-empty directories. Python and C# now share the neutral `🔤️tokens` and `🎨️palette` owners with Rust and CSS. `🛂️adapters.manifest.json` describes the new adapters.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` now models nested generated directories recursively for generate, preview, stale cleanup, and freshness. It emits the five registry identities and exports the deterministic playground-session renderer.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` owns generate, preview, and exact-byte check for `playground-session`; `📋️project.json` registers all three Nx targets and their producer dependency.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` registers the generated semantic members, neutral styling owners, the `generated-source-topology` schema kind, every renamed generator output, the `playground-session` generate/preview/check contract, and the ordinary .NET project file admission. Obsolete language-owned .NET styling member kinds were removed. The strict loader accepts the catalog and all generator contracts.
- `.gitignore` carries the updated exact styling ignores. `.vscode/launch.json` was regenerated from the current plugin registry input after the package-body route/seed lane completed its concurrent launcher update.

## Portable regression

Added:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏭️generated-source-topology/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏭️generated-source-topology/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️generated-source-topology/🟦️.ts`

Registered through:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`

The test validates schema conformance through Ajv, unique old and canonical paths, semantic-parent ownership, file-kind leaf mapping, taxonomy generator-contract coverage, absence of implementation-leaf findings, physical absence of every previous path, and physical presence of every canonical path.

## Exact live consumer and authority files repaired

The following authored or authoritative files contain the corresponding canonical references after this lane:

- `🧰️framework/🛍️products/💻️os/📖️stories/🧭️coordination/🟦️.tsx`
- `.storybook/scopes.ts`
- `.storybook/stories/block/scene.ts`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📦️assets/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🟦️.ts`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts`
- `📜️script.ts`
- `🧰️framework/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/📬️mailbox/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️outputs.json`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts`
- `🧰️framework/🔨️modules/🕸️graph/🧫️fixtures/🔣️outputs.json`
- `🧰️framework/🔨️modules/🕹️interaction/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🌓️theme.css`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui.css`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🧩️component.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧬️typegen-export/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs`
- `🧰️framework/🔨️modules/🖼️assets/🎯️concepts/🟦️.ts`
- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🟦️.ts`
- `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs`
- `🧰️framework/🔨️modules/🧬️schema/⚛️component.rs`
- `🧰️framework/🔨️modules/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🚀️bin.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⚙️browser-build/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📚️storybook-plugins/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/⚙️config-graph.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/👁️watch-policy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/README.md`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🟦️.ts`

The UI execution lane separately owns and verified the two WGPU icon source consumers at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs` and `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔣️icon-name/🧾️value/🦀️.rs`. Both resolve the new generated icon leaf. That attribution remains in `📓️sol-ui-render-2026-09-12.md`.

## Verification evidence

- Strict taxonomy and generator-contract probe after all edits: `loadTaxonomy()` succeeded; `validateTaxonomy(...)` returned `[]`; `validateGeneratorContractsAgainstWorkspace(...)` returned `[]`.
- Direct portable regression after the neutral owner correction: 2 passed, 0 failed, 217 expectations, 855 ms.
- `@semio-tech/assets:build`: passed; generated 286 assets. Its freshness check passed with all 286 outputs fresh in 3.9 s.
- `@semio-tech/framework-actor-rs:typegen`: passed its Rust typegen test and refreshed the canonical actor output.
- `@semio-tech/framework-async-rs:typegen`: passed its Rust typegen test and refreshed the canonical async output.
- `@semio-tech/framework-rs:generate`: passed its typegen test and refreshed the manifest output in 46.7 s.
- `@semio-tech/ui-contract-rs:generate`: passed its typegen test and refreshed the UI contract output in 4 min 1 s.
- `@semio-tech/ui-rs:generate`: passed; direct freshness reports `ui axes are fresh (2 locales, 2 terminologies)`.
- `@semio-tech/framework-graph:generate`: passed and wrote nine manifests. Registered graph check passed 3 tests and 70 expectations and reported all nine generated manifests fresh in 13.3 s.
- Schema generation was regenerated through the shared producer by the Go execution lane; its focused entity-kind test passed. Direct freshness reports 58 entity kinds at SHA-256 `f237cdd640bd1ac2b4546726f78f33c90aaf0c2ed11e1f6115294c8e9c3be242`.
- Styling generation passed in 647 ms; direct freshness passed in 1.4 s and reports all generated styling artifacts fresh.
- Registered plugin-registry generation passed after the current launch seed, reporting 59 plugin crates, 61 playgrounds, and 49 framework packages. Its registered freshness check reports the generated catalog and launch bytes fresh.
- Registered playground-session generation passed with dependencies in 28.4 s. Direct check reports the generated source fresh. Direct preview exits 0 and emits schema version 1 with the canonical semantic directory node, the canonical anonymous TypeScript file node, and no stale removals.
- Earlier registered checks were blocked by a missing native Cargo manifest and missing source project while other lanes were editing the shared graph. Current isolated Nx runs reach and pass the generated-source topology, styling Python, styling .NET, package-body policy, styling freshness, and plugin-registry freshness targets. The earlier diagnostics were transient concurrent-work observations, not current failures.
- The direct framework and UI-contract freshness checks were also attempted. Both waited on shared Cargo package/artifact locks from concurrent work for several minutes. The framework process eventually began compiling `semio-framework-os-kernel`; both processes were stopped with exit 130 to release shared capacity. Their successful generator runs above are the completed producer evidence; no source failure was observed in these two interrupted attempts.

## Residue and later generated-output lanes

The live authored-source scan finds none of the 29 previous persistent paths outside the intentional `previousPath` regression fixture. Historical `.cursor` plans retain their historical coordinates and were not rewritten.

Generated or staged outputs outside this bounded packet remain distinct work:

- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🤖️generated/🦀️generated_value_bridge.rs` and the schema module's `🤖️generated.rs` are persistent generated source identities whose producer contracts need a later semantic move.
- OS development hashed JavaScript and CSS distributions are disposable, content-addressed build outputs.
- JCO `bridge.js` and generated declaration interfaces, plus flow wasm-bindgen browser and host bindings, are package adapter/build products. They require a producer-contract lane rather than fixed-name exceptions.
- `📇️registry/dist/sessions/<variant>/🟦️session.ts` is a staged per-variant distribution identity used by the demonstrator and development bundling paths. It is separate from the persistent `playground-session` source moved here.
- The existing WGPU frame-worker bundle can retain old source-coordinate comments until its owning bundle producer refreshes it. It is a derived bundle, not an authored or persistent source consumer changed in this lane.

No blanket fixed-filename admission, generated-directory exemption, compatibility reader, or migration script was added.

## Neutral Python and C# owner correction

The final Python and C# projections are owned by language-neutral styling domains:

- Python: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/🎨️styling/🤖️generated.py` → superseded intermediate `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/🎨️styling/🔤️tokens/🐍️.py` → `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🐍️.py`.
- C#: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔷️net/🖌️Elements.Styling/🤖️Generated/🎨️Palette.g.cs` → superseded intermediate `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔷️net/🖌️Elements.Styling/🤖️Generated/🎨️palette/🔷️.cs` → `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🔷️.cs`.

The original .NET owner contained only the generated C# projection. It had no project, solution, package, or authored metadata to preserve. The whole now-empty `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔷️net` chain is absent.

The Python package uses `pyproject.toml` and Hatch's wheel inclusion map to place the one canonical neutral source directly at `🎨️styling/🐍️.py` in the wheel. No generated or compatibility copy exists in the repository package. The tracked package initializer is now `🎨️styling/__init__.py`; its exact `python-init` fixed contract has `adapter-source` / `package-glue` disposition, selects the existing `python` grammar, classifies as the allowed `declaration` role with `trivia-only source` evidence, and introduces no package-boundary problem. This preserves the existing `import_module("🎨️styling.🐍️")` API while the source check imports the canonical `🔤️tokens.🐍️` identity.

The .NET package is `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🔷️dotnet`. Its `🔷️.csproj` includes `../../🎨️palette/🔷️.cs` directly as a linked compile item and emits `Semio.Framework.Ui.Styling.dll`; there is no second C# source. `Monorepo.sln` contains that project. The only package-boundary admission added was the existing `dotnet-project` file kind for the existing `🔷️dotnet` rule.

Native build orchestration lives at the neutral semantic owner `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🟦️.ts`. Both package-local `📜️script.ts` files now contain only `ScriptRouter` wiring to that owner. A focused live discovery probe found no `package-implementation`, `package-role-unresolved`, or `packaging-violation` problem below either native package root.

### Exact correction files

Updated:

- `.gitignore`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`
- `Monorepo.sln`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🛂️adapters.manifest.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/pyproject.toml`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/uv.lock`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/📋️project.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏭️generated-source-topology/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏭️generated-source-topology/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️generated-source-topology/🟦️.ts`

Added:

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/🎨️styling/__init__.py`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🔷️dotnet/🔷️.csproj`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🔷️dotnet/📋️project.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🔷️dotnet/📜️script.ts`

Removed:

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/🎨️styling/🐍️.py`
- the generated-only `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔷️net` tree

Generated and exactly ignored:

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🐍️.py`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🔷️.cs`

The styling producer's concurrently coordinated scan input was updated from `.storybook/framework/os/index.tsx` to the UI lane's neutral `🧰️framework/🛍️products/💻️os/📖️stories/🧭️coordination/🟦️.tsx`; ownership of the moved Storybook source remains with that UI lane.

### Correction verification

- Styling generation and final direct freshness: passed; `framework/ui/styling: generated artifacts are fresh`.
- Strict current authority: `loadTaxonomy()` succeeded, `validateTaxonomy(...)` returned `[]`, and `validateGeneratorContractsAgainstWorkspace(...)` returned `[]`.
- Portable fixture: direct and registered runs passed 2 tests, 0 failures, 217 assertions. Its two native-package rows resolve metadata source references exactly to the neutral sources and prove both superseded intermediate paths absent.
- Python: `uv lock --check` resolved one package; direct source and built-wheel imports both printed their success markers. The final wheel contains only `🎨️styling/__init__.py`, `🎨️styling/🐍️.py`, and wheel metadata. Registered `@semio-tech/ui-styling-py:test-quick` passed its dependency, generation, wheel build, source import, and wheel import tasks.
- .NET: direct restore and Release build passed with 0 warnings and 0 errors. Registered `@semio-tech/ui-styling-dotnet:build` passed its three-task graph and emitted `Semio.Framework.Ui.Styling.dll`; `dotnet sln Monorepo.sln list` reports the kind-only project path.
- Package bodies: the direct scoped discovery probe reports no implementation, unresolved-role, or packaging findings for either native package. Registered `@semio-tech/repo-lib:test-package-body-policy` passed 45 tests and 138 assertions.
- Registered `@semio-tech/ui-styling-tokens:check-generated`, `@semio-tech/repo-lib:test-generated-source-topology`, and `@semio-tech/plugin-registry:check-generated` all passed. The registry reports its catalog and launch bytes fresh.
- All four original/intermediate generated paths and the old full .NET owner are physically absent. Repository references to the two intermediate paths occur only in `supersededPaths` regression evidence.
