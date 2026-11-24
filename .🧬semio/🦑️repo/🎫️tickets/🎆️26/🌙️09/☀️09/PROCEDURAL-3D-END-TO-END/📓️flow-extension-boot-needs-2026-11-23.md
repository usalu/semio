# Flow extension boot needs for generation3d (2026-11-23)

Read-only audit. No file was edited and no build or heavy command was run.

## Verdict

- **Minimal boot set today:** `procedural` (the primary, already staged), the flow hub `flow`, and the nine flow extensions `flow-extension-{bim,brep,dictionary,draw,list,logic,math,primitive,text}`. The procedural plugin declares all nine as hard dependencies, and every extension declares `flow` as its dependency. Missing any of them fails the boot.
- **The registry also pulls two more descriptors that the ticket's staging list omits:** `playbook-module-procedural` and its dependency `playbook`. They enter through the tag-contributor rule, not through a declared dependency. The next 404 after staging the ten is therefore predicted to be `playbook-module-procedural/🔣️.json`. This is inferred from code and was not run.
- **Used by the 3D examples:** the flow hub (every example runs as a flow host), `brep` (9 of the 11 listed examples) and `math` (3 examples). `bim`, `dictionary`, `draw`, `list`, `logic`, `primitive` and `text` are not used by any 3D example or fixture.
- **Eager or lazy:** **eager**. The whole declared closure, plus every installed contributor of a consumed tag, is read as descriptors before any actor starts. The architecture has the pieces for lazy activation (`activationEvents`, and extension `extends` activation), but the descriptor layer does not use them. The tag-contributor rule and the hard `depends-on` list are over-eager for the 3D playground. Making it lazy needs changes in the registry and in the descriptor schema (exact seams in section 4).

## 1. Where the selected-module graph is computed

| Step | File (evidence) |
|---|---|
| Generated catalog rows (74 plugins and extensions) | `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json`. Row 43 is `procedural`: `dependsOn` lists the nine `flow-extension-*`, `consumes` = `forms.questionKind`, `flow.extension`, `activationEvents` = `on-artifact-kind:*generation{2d,3d}`. Every `flow-extension-*` row has `contributes: ["flow.extension"]`, `dependsOn: ["flow"]`, `activationEvents: []`. The `flow` row has `consumes: ["flow.extension"]` and no `dependsOn`. The `playbook-module-procedural` extension row has `contributes: ["playbook.blockKind","forms.questionKind"]` and `dependsOn: ["playbook"]`. |
| Session copy of the same 74 rows | `.cache/b/p/m/browser/prototype/sessions/generation3d/🔌️plugins.json` (same data) |
| generation3d playground row | `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json`, index 33: `pluginId: procedural`, `ports: {react: 6018, wgpu: 6118}`, 11 examples |
| Procedural declared deps (source of truth) | `🌎️hub/🧩️compositions/🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml` lines 18-30: `consumes = ["forms.questionKind", "flow.extension"]`, `depends-on = [bim, brep, dictionary, draw, list, logic, math, primitive, text]` |
| Procedural runtime declarations | `🌎️hub/🧩️compositions/🌀️procedural/🦀️.rs`: `FLOW_EXTENSIONS` table (nine slugs; `primitive` maps to extension id `core`), and `builder.flow_extension(...)` for each (lines ~102-104) |
| Catalog composition (all 74 targets, no selection) | `✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts` lines 7-16 (`composeSpecificOsCatalogV1`: `PLUGIN_BUILD_TARGETS` plus `EXTENSION_TARGETS`) |
| Boot module list = all catalog targets when no `plugins` override | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts` line 58-60: `modules: (options.plugins ?? options.catalog.targets)`. Frame worker: `🎞️frame-worker/🔌️boot-and-plugins/🟦️.ts` lines 153-159 |
| Wgpu registry (descriptor BFS, plan assertion) | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧩️plugin-modules/🟦️.ts`: `prepareWgpuPluginModules` lines 34-83 (queue seeded line 48-52 from `expandPluginRegistry`; sequential `await readDescriptor` line 64; dependency enqueue lines 68-73), `assertWgpuPluginPlan` lines 85-92 |
| Expansion rule (dependencies plus tag contributors) | `🧰️framework/🔨️modules/🎠️kernel/🗂️plugin-registry/🟦️.ts` lines 14-34. Line 18-19: `contributorEntries` = every entry whose `contributes` intersects the primary's `consumes`. Lines 23-31: transitive `dependencies` walk |
| Planner (closure phase) | `🧰️framework/🔨️modules/🎠️kernel/🎮️playground-boot/🟦️.ts` lines 113-116 and 129-133 |
| Missing dependency and ordering | `🧰️framework/🔨️modules/🎠️kernel/🕸️plugin-graph/🟦️.ts` line 69 (`transaction.dependency-missing`); lines 198-209 (`orderPluginRegistryEntries` blocks dependents of a fault) |
| 404 on descriptor | `🧰️framework/🔨️modules/🎠️kernel/🛂️descriptor-admission/🟦️.ts` line 85-93 (`plugin.descriptor-unavailable` when `!response.ok`) |

Flow for generation3d: `primary = procedural` (`bootVariant` resolves to the plugin). `expandPluginRegistry` seeds the queue with `procedural`, then every contributor of `forms.questionKind` or `flow.extension` in the module list, in list order. `flow-extension-bim` is first alphabetically, which matches the 6118 failure on `bim`. The `dependencies` walk then adds `flow`.

## 2. What generation3d actually uses

3D examples live in `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`. The playground lists 11 examples. Nine have a `.dsl.semio` asset in that folder. `demo-session` has only a `.cmd.semio` command script. `demo` is not in the 3D examples folder.

Node kinds (`neuron-kind=` in `*.dsl.semio`):
- `brep.*` in 9 examples: hexagonal-mushroom-column, sphere-cut-with-torus, box-shell-preview, box-fillet-preview, rectangle-extrude-volume, mesh-workbench, sphere-box-fuse, face-sweep-extrude, rectangle-wire-preview (25 occurrences).
- `math.vector` in 3 examples: hexagonal-mushroom-column, rectangle-extrude-volume, face-sweep-extrude.
- No `bim.*`, `logic.*` or `dictionary.*` references anywhere under the generation artifacts.

3D fixtures and schema:
- `🧬️schema/🗂️catalogue/` (editor palette, not usage): `brep-*.json` (brep), `math-*.json` (math: `math-arithmetic`, `math-list`, `math-values`, `math-vector`), `mesh-*.json` and `analysis-*.json`. The owning extension of `mesh-*` and `analysis-*` was not verified.
- `🧪️tests/🧫️fixtures/📐️geometry-dependencies.json` uses `math.add`. `preview-eval-run.json` (generation2d) uses `draw.shape.rect` and `draw.style.stroke`. The 2D demo example `🎬️demo` (under the generation2d tree) uses `draw.*`, so `draw` is needed by 2D, not 3D.
- `text.*` appears only in a Rust test (`🎬️demo-session/🧪️tests/🧩️example/🦀️.rs`), not as a node.

Result: the 3D playground needs the flow hub (structural host), `brep` and `math`. Seven of the nine extensions are not used by 3D, but the boot declaration still requires all of them.

Staged state: `.cache/b/p/m/browser/prototype/modules/` contains only `🌀️procedural`, `🖨️raster`, `🧵️shard` and `🪞️vendor`. There is no `extension-modules` folder under it. The ticket's closure list in `TF/wgpu-native-modules.ts` line 13 hard-codes 11 ids (`flow` plus nine extensions plus `procedural`). It does not include `playbook`, `playbook-module-procedural`.

## 3. Eager or lazy

Eager evidence:
- `prepareWgpuPluginModules` docstring, line 33: "Reads trusted descriptors in withdrawable turns and restricts the planner to supplied modules." The loop at lines 53-76 reads every queued descriptor in sequence before the plan exists.
- `descriptorDependencies` (lines 20-31) refuses any dependency key other than `pluginId` and `version`. The descriptor schema has no optional or soft dependency, so a declared `depends-on` is always a hard load requirement.
- `expandPluginRegistry` (line 18-19) adds every contributor of a consumed tag. For generation3d this pulls in the nine extensions and also `playbook-module-procedural` (`forms.questionKind`), which has nothing to do with flow.

Lazy intent in the docs:
- `🧰️framework/🔨️modules/🎠️kernel/🪪️identity/README.md` lines 75-78: the kernel queries installed extension descriptors at plugin activation for `extends == plugin_id`. This is activation-time and lazy, but it applies to `extends` extensions only. Flow extensions use the `contributes` and `consumes` path, which the registry resolves at boot.
- The procedural row has `activationEvents: on-artifact-kind:*`, which implies the plugin activates only when a 3D artifact opens. The dependency closure is still read at boot regardless of those events.

Verdict: eager loading is intentional for ordering and trust admission. It is a design gap, not a correctness bug, for three reasons: the activation events are ignored for descriptor reads; the hard `depends-on` list encodes "used by the procedural plugin" as "needed at boot"; and the tag-contributor rule loads unrelated contributors (`playbook-module-procedural`) into a 3D boot.

Exact places to make it lazy:
1. `🧩️plugin-modules/🟦️.ts` lines 48-52: seed the queue with the primary plugin only, not with `expandPluginRegistry`'s full closure.
2. `🗂️plugin-registry/🟦️.ts` lines 18-21: stop auto-adding tag contributors. Defer them until a kind from that extension is referenced.
3. `🧩️plugin-modules/🟦️.ts` lines 68-73: enqueue only dependencies that are needed now. Keep the `depends-on` edges, but mark the nine flow extensions as optional or on-demand. That requires a schema change in `🧬️schema/🧩️plugin-modules/🔣️.json` and in `descriptorDependencies` (line 20-31), because today it rejects any extra edge key.
4. `assertWgpuPluginPlan` (lines 85-92) and `🎮️playground-boot` closure (lines 113-116) must then accept a plan that leaves deferred extensions out.
5. Procedural side: `Cargo.toml` lines 19-30 `depends-on`, and `🦀️.rs` `builder.flow_extension` loop (lines ~102-104), would move the nine extensions from hard to on-demand declarations.

## 4. Practical consequence for 6118 today

No cheaper path exists without a code change. To boot the 3D WGPU playground as the code stands, stage the flow hub, the nine flow extensions, and also `playbook` and `playbook-module-procedural` (predicted from code, not run). Stage only `brep` and `math` if the 3D examples are the only target, but that is not enough for the boot. The boot is over-eager: it needs all ten flow components, and two more descriptors, to use three.

## Files cited

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧩️plugin-modules/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧩️plugin-modules/🛂️admission/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🔌️boot-and-plugins/🟦️.ts`
- `🧰️framework/🔨️modules/🎠️kernel/🗂️plugin-registry/🟦️.ts`
- `🧰️framework/🔨️modules/🎠️kernel/🎮️playground-boot/🟦️.ts`
- `🧰️framework/🔨️modules/🎠️kernel/🕸️plugin-graph/🟦️.ts`
- `🧰️framework/🔨️modules/🎠️kernel/🛂️descriptor-admission/🟦️.ts`
- `🧰️framework/🔨️modules/🎠️kernel/🪪️identity/README.md`
- `✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts`
- `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json`
- `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json`
- `🌎️hub/🧩️compositions/🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/🧩️compositions/🌀️procedural/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️catalogue/`
- `.🧬semio/.../PROCEDURAL-3D-END-TO-END/wgpu-native-modules.ts` (line 13 closure list)
- `.🧬semio/.../PROCEDURAL-3D-END-TO-END/🗑️generated/wgpu-6118-console.txt` (404 evidence)
