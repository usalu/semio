# Terra Plugin/Artifact Runtime Census

## Verdict

**RED — the current generated registry is a 59-row build/marketplace inventory, not evidence that all declared plugins or artifacts can be authoritatively opened, rendered, or mutated.** This is a read-only source snapshot; no build, browser, native, or MCP runtime command was run for this audit. Existing D0/D1 transport work is deliberately not credited as catalog/provider runtime evidence.

The closest usable path is source-only: the browser can list a registry row, attempt to fetch its module, load a guest manifest, create a local app, then invoke the D1 opening relay. It has no all-row availability proof; hub lacks every native codec binding; the WGPU bridge deliberately rejects backbone attachment; and the MCP channel truthfully rejects generic plugin mutations today.

## Scope and evidence boundary

This census follows the live generated registry into package descriptors, browser shell/dev serving, native hub trust, WGPU, and MCP. “Source-present” below means an implementation branch exists in the current tree, not that it compiled or ran. “Proven” is reserved for a terminal runtime observation; there are none in this report.

Key authority boundaries:

| Boundary | Current source fact | Classification |
| --- | --- | --- |
| Registry identity | `PLUGIN_BUILD_TARGETS` and `EXTENSION_TARGETS` are emitted from the generated registry; only plugins become `PROGRAM_TARGETS`. | Source-present inventory only. |
| Browser marketplace | Shell maps registry plugins and every extension target into marketplace rows, then can invoke its module loader. | Source-present; a row is shown before an asset fetch succeeds. |
| Browser artifact opening | `replayShellCommand` installs/opens the app and calls D1 `openDocument` only when both `documentId` and `schema` are supplied. | Source-present; no per-artifact acceptance result is awaited by `openArtifactWithAppRef`. |
| Hub D0 authority | Startup supplies `linked_native_codec_bindings()` to the trusted catalog loader, but the function returns `Vec::new()`. | RED for every native codec/provider claim. |
| WGPU app rendering | The WGPU ProgramBridge rejects `attach_backbone` and `detach_backbone` as retired with no replacement. | RED for actual WGPU document attach/render/mutate. |
| MCP read/history | `PluginArtifactChannel` executes real guest turns for the limited wire framing it has. | Source-present only; not a catalog-wide successful action proof. |
| MCP generic mutation | `PureCommand` reaches the guest, whose only shared dispatch implementation rejects without an owner-qualified manifest command key. `TransactionPrepare` also has no config/draft-lane wire form. | RED for generic plugin mutation. |

## Registry census

The generated file contains **59 entries: 33 plugins and 26 extensions**. It carries identity, Wasm output, role, dependency, activation, contribution, execution-mode, and optional hash metadata; it does not carry the app/role/artifact-surface contract used at runtime. The only emitted host configuration is `s → { landingAppId: home, hostAppId: studio }`; that is an intentional shell-host configuration, not an all-plugin app binding.

### Descriptor-backed plugins — 29 rows, 107 declared app surfaces

These rows have owner-root `🔣️.json` plus descriptor pack and registry hash/execution metadata:

| Plugin | Declared apps | Registry activation status | Runtime classification |
| --- | ---: | --- | --- |
| animate | 2 | present | source-only |
| architect | 2 | present | source-only |
| cad | 2 | present | source-only |
| dag | 2 | present | source-only |
| demonstrator | 10 | none | source-only; no automatic artifact activation |
| draw | 2 | present | **RED asset parity** — no current dev module directory |
| energy | 2 | present | **RED asset parity** — directory lacks the registry-named JS module |
| fem | 4 | present | source-only |
| flow | 2 | present | source-only |
| forms | 2 | present | source-only |
| gis | 4 | none in generated row despite its descriptor’s `onArtifactKind: 2d.map` | **RED generated/descriptor drift** |
| imperative | 2 | present | source-only |
| layout | 2 | present | **RED asset parity** — no current dev module directory |
| lowpoly | 2 | present | source-only |
| mathematical | 2 | present | source-only |
| norm | 30 | present | source-only |
| note | 2 | present | source-only |
| procedural | 4 | none | source-only; no automatic artifact activation |
| process | 2 | present | source-only |
| puzzle | 6 | none | source-only; no automatic artifact activation |
| raster | 2 | present | source-only |
| reasoning-mindmap | 2 | present | source-only |
| remodel | 2 | present | source-only |
| s | 5 | present | host-configured shell source only |
| sequence | 2 | present | source-only |
| shooting | 2 | present | source-only |
| sourcing | 2 | none | source-only; no automatic artifact activation |
| vcs | 2 | present | source-only |
| writer | 2 | present | source-only |

The exact 107 count is the package-descriptor app count, not a count of validated document codecs. For example, GIS declares four apps and individual app dialects, but its package-level `.manifest.artifactKinds` is empty. The static descriptor census found the same package-manifest empty artifact-kind projection across the 40 descriptor-backed registry rows. This does **not** mean the artifact source folders are absent: e.g. GIS, Flow, Raster, Trinity, and others contain `ArtifactKindSpec`/`NativeCodecs` definitions. It means the provider declarations have not been carried to the catalog trust boundary.

The registry itself also drops the GIS activation information visible in its current owner descriptor: `✏️s/🔌️plugins/🌍️gis/🔣️.json` begins with `onArtifactKind(2d.map)`, while the generated GIS row has `activationEvents: []`. Treat that as current generated-data drift until generation/check is terminally rerun; do not infer browser activation from the descriptor alone.

### Plugins without descriptor/pack authority — 4 rows

`block`, `playbook`, `stdio`, and `trinity` have registry rows but no current owner-root descriptor pair, hash, or execution mode. Some contain rich artifact source code. None can presently be claimed as a descriptor-authorized catalog provider, automatically host-bound app, or trusted D0 codec.

### Descriptor-backed extensions — 11 rows

`cad-extension-aec-building`, `cad-extension-aec-building-energy`, `cad-extension-aec-building-structure`, `cad-extension-spatial-shape`, `flow-extension-brep`, `flow-extension-dictionary`, `flow-extension-list`, `flow-extension-logic`, `flow-extension-math`, `flow-extension-primitive`, and `flow-extension-text` have descriptor/hash metadata. Extensions correctly declare no standalone app surfaces, but their actual host contribution attachment is not proven by registry presence. Seven Flow entries also currently duplicate the `flow.extension` contribution string; that is a catalog normalization defect/risk, not evidence of two installed contributions.

### Extensions without descriptor/pack authority — 15 rows

`flow-extension-bim`, `flow-extension-draw`, `imperative-extension-control`, `imperative-extension-effect`, `imperative-extension-logic`, `imperative-extension-math`, `imperative-extension-text`, `playbook-module-procedural`, `process-extension-concrete`, `process-extension-metal`, `process-extension-robotic`, `process-extension-wood`, `sourcing-module-beams`, `sourcing-module-slabs`, and `sourcing-module-windows` are inventory-only. They have no package descriptor/pack hash/execution mode, and no static proof that the stated extension point is installed into its host.

## End-to-end cut-through

### Browser OS/React

`ShellHost` multiplexes `createDevPluginSource(registry)` and the extension source, maps every row into marketplace state, and marks a plugin `failed` only after `loadPluginModuleResilient` fails. `createDevPluginSource` returns the complete generated registry from `list()` and constructs a direct module URL from the registry entry; it does not ask whether the exact JS asset is currently present. The Vite hot-swap snapshot similarly calls a module “built” when it finds any `.core*.wasm` in the directory, without checking the registry-named JS output, descriptor, or descriptor hash.

The materializer itself writes the named JS module before the hot-swap marker. Therefore the current source-tree observations matter: `draw` and `layout` have no dev module directory, and `energy` lacks its expected `semio_s_plugin_energy.js`. A marketplace click may therefore point at an unavailable direct URL. This is a source-tree integrity finding, not a browser-network result.

`AppRouter` is built solely from **loaded** guest manifests. It can legitimately route only after the plugin module loaded. `openArtifactWithAppRef` creates the app after install, but sends its optional `openArtifact` notification with `void`/optional chaining; it does not await an authority acknowledgment before changing local session state. The later D1 `openDocument` call is conditional on an effect carrying both document id and schema. Thus browser behavior is a reasonable loader/relay skeleton but not an atomic “catalog app opened this authoritative artifact” contract.

### Native/Tauri/WGPU

No audited native shell can open an authoritative catalog artifact now. Hub startup passes an empty native binding vector to the trusted catalog loader. The loader correctly requires the package manifest artifact-kind count to equal the trust record’s native codecs and requires every declared kind/schema to be present. Empty package-level artifact projections plus empty runtime bindings prevent a valid artifact provider claim, even where a plugin’s Rust source has a codec implementation.

The WGPU ProgramBridge independently hard-fails backbone attachment. Its callers cannot use that failure as a successful program/document render path. This is a functional blocker separate from D0 catalog authority and should not be hidden by browser-only guest loading.

### MCP

MCP registry discovery intentionally skips every entry whose owner descriptor cannot load and reports a diagnostic; a bare catalog degrades to gateway-only instead of falling back to a fixture. This is honest but confirms that the 19 descriptor-less rows do not enter the real installed MCP catalog.

For a descriptor-backed plugin, MCP can find an editor app and instantiate a `PluginArtifactChannel`. `ReadHistory` and generic transaction lifecycle frames have real guest paths. Generic action mutation remains deliberately incomplete: the `PureCommand` guest dispatch rejects without an owner-qualified manifest command key, and `TransactionPrepare` has no config/draft-lane representation. There is no generic arbitrary-schema command either. MCP therefore cannot be credited as an all-plugin artifact mutation surface.

## Current gate assessment

| Gate | Registration | What its source actually establishes | Gap |
| --- | --- | --- | --- |
| `@semio-tech/plugin-registry:check-generated` | Nx + launch entry | Generated registry and launch bytes match renderer output. | No descriptor, served-module, codec, app/role, or open/mutate proof. |
| `@semio-tech/plugin-registry:check` | Nx | Runs registry validation, but descriptor/discovery findings may be warning-only while plugin areas are legacy/mixed. | A 59-row catalog can remain usable-looking with missing descriptor authority. |
| `@semio-tech/plugin-registry:catalog-complete -- --build-root <absolute>` | Nx + launch entry | Strictly validates a supplied isolated fresh raw/core/descriptor triplet and commit receipt for the registry entries. | Its source preflight passes `ownerDescriptors: "ignored"`; it does not itself prove owner-root descriptor parity, app/role routing, D0 codec binding, or an open/mutate journey. No run observed here. |
| Dev hot-swap tests | source tests in dev script | Snapshot behavior around core Wasm presence. | Does not require the exact registry JS/descriptor/hash artifacts. |
| MCP workspace tests | source tests | Some real-channel frame routing/rejection behavior. | Not an all-catalog artifact-create/open/mutate/reopen oracle. |

No registered non-vacuous gate currently proves this minimum product claim for every appropriate row: **descriptor and module exist → requested artifact dialect resolves to a provider → document plan is authority-approved → browser/native app attaches → one authorized mutation persists → a second principal/reopen sees the result; extensions prove one host contribution instead of a standalone app.**

## Smallest independent implementation packets

1. **Catalog identity and publication (independent; do first).** Make all 59 rows explicit about their state. Either publish a canonical descriptor+pack+hash+execution mode or remove the row from runtime catalog generation; do not retain inventory-only runtime rows. Deduplicate contribution tokens. Generate a catalog model that includes app refs, app roles, dialects, activation and extension-host identity rather than only build fields. Fail `check` on missing/mismatched descriptors for all declared runtime rows.

2. **Exact browser artifact availability (independent from D0).** Change hot-swap/build verification to require the registry-named JS, core Wasm, descriptor/pack, and matching hashes before advertising a plugin. Make marketplace status reflect verified availability, not just registry membership. Add a neutral source fixture for `draw`, `layout`, and `energy`-style partial rows and prove they cannot be offered as installable.

3. **Catalog-to-D0 codec/provider publication (depends on the active D0 codec/open-plan packet).** Generate package-manifest artifact-kind declarations from the same schema that creates app dialects; create an explicit native `NativeCodecBinding` table, bind it at hub startup, and ensure trust records exactly agree. Keep undecodable/unsupported formats out of the provider map. This packet must preserve D0’s hostile size/hash/schema/frontier limits and return a typed no-provider result, never a guessed codec.

4. **Atomic app/open relay (depends on D1 only for transport).** Give browser `openArtifactWithAppRef` an awaited, typed app/open acknowledgment. Do not commit the new local session until the selected app and authoritative D1 plan agree on package, dialect, schema, document and scope. Keep cancellation closing any late app/channel. The neutral oracle needs success, missing app, stale plan, wrong role/dialect, lost module, and cancellation after module load.

5. **WGPU event backbone (independent implementation, blocked by packet 3 acceptance for real documents).** Replace the explicit retired attach/detach rejection with the current event-driven bridge, preserving connection cancellation and no mutation before Session/D1 authority. A native fixture must open one catalog provider, render an observable state, mutate once, disconnect/reconnect, and verify no secret/receipt enters logs or display state.

6. **MCP owner-qualified command ABI (independent from browser/WGPU).** Add schema-declared command keys and an agent-owned action address; preserve bounded command/response pages, transaction ownership, principal propagation, and typed unsupported action errors. Extend the channel oracle with a real accepted command and durable/reopen visibility rather than merely proving it returns a non-host-fabricated rejection.

7. **One all-catalog acceptance matrix (last; consumes 1–6).** Generate language-neutral fixtures from the registry schema. For every plugin app/dialect, assert the exact result category—openable provider, intentionally non-artifact host, or unavailable with a reason. For each extension, assert host identity, exactly-once contribution, attach/detach and absence of standalone app routing. Run browser, WGPU/native, and MCP subsets through registered Nx/launch entries against a fresh catalog root. Treat missing assets, missing provider, unknown role, stale hash, cancellation, and second-user reopen as mandatory negative/positive cases.

## Acceptance and nonclaims

The current source has honest partial mechanisms: marketplace listing, module loading attempt, D1 opening relay, strict trusted-catalog checks, bounded MCP frame handling, and a fresh-artifact verifier. They are not sufficient to claim “OS frontend with all plugins, artifacts.” In particular, this audit makes **no** claim that any of the 107 app surfaces, 59 registry rows, or artifact source folders currently opens/renders/mutates at runtime.

Current D0/D1 work should be accepted separately only for the scopes backed by its own exact terminal evidence. Once those packets land, rerun the catalog census/gates: D1 transport alone cannot supply descriptor integrity, codec authority, browser asset parity, WGPU attachment, or MCP action semantics.

## Exact current source anchors

- Registry generated shape and host/program split: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts:40-114`.
- Descriptor/manifest authority and the intentionally ignored owner descriptors in `catalog-complete`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2460-2565,2678-2739,2760-2833`.
- Example package/app mismatch (GIS): `✏️s/🔌️plugins/🌍️gis/🔣️.json:1-20,6680-6739`.
- Browser source, install, loaded-only router, relay and marketplace: `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2757-2783`; `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️.tsx:1710-1810,3145-3190,4528-4570,4681-4710,5230-5265`.
- Dev availability scan/materializer ordering: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:367-385,994-1017`.
- Empty hub binding and startup use: `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:385-395,5238-5254`; trusted codec/manifest matching: `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:765-800`.
- WGPU deliberate unavailable bridge: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:275-286,521-525`.
- MCP discovery and real-but-incomplete channel: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📇️registry/🦀️.rs:1-106`; `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:545-563,915-985,1071-1140`.
- Registered gate/launch declarations: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json:1-64`; `.vscode/launch.json:6049-6079`.
