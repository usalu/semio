# Lazy extension activation (2026-11-23)

Follows `📓️flow-extension-boot-needs-2026-11-23.md`. The generation3d boot read every flow extension, the flow hub and (through the tag rule) `playbook-module-procedural` plus `playbook` before any actor started. It now reads the primary plugin and its hard requirements only; everything else is deferred and activated on demand. No cargo or wasm build was run; the tests are TypeScript (`bun test`).

## Decision: one vocabulary, no parallel mechanism

| Concept (already existed) | Role now |
|---|---|
| `dependsOn` / `depends-on` | Hard requirement. Always loaded before the dependent. |
| `consumes` / `contributes` tags | Who may extend whom. A contributor of a consumed tag is booted eagerly **unless** it declares activation events. |
| `activationEvents` (`on-artifact-kind:`, `on-extension-request:` ...) | A plugin with at least one event is activated by that event, never at boot. This was already the meaning for artifact kinds. |
| `extends` | Unchanged (host edge, first dependency). |

New, minimal additions:
- Catalog-level event `on-node-kind:<namespace>` (`ON_NODE_KIND_ACTIVATION_PREFIX`, `nodeKindActivationEvent`; `brep.bool.cut` is `on-node-kind:brep`). Host-only like `manual`: it never reaches a guest as a WIT event, so no Rust ABI change. The descriptor gate requires an `extends` host for it.
- Authored in each extension's `Cargo.toml` as `[package.metadata.semio] activates-on = ["on-node-kind:brep"]`, merged into the existing `activationEvents` column by `parseSemioActivatesOn` (discovery). No new catalog column, so the strict row schemas and the Rust catalog readers are untouched.
- Descriptor dependency edge kind: `activation: "required" | "on-demand"` (default `required`) in `🧬️schema/🧩️plugin-modules/🔣️.json` `$defs.dependency`. An `on-demand` edge is not loaded at boot, is not a load-graph requirement, and must target a plugin that declares activation events (refused otherwise: `on-demand dependency without activation events`).
- The same schema now also holds `$defs.onDemandRequest` (node-kind, tag, event, plugin), `$defs.onDemandStatus`, and the admission policy enumerations (`dependencyActivations`, `onDemandPhases`, `onDemandUnavailableReasons`, `onDemandMissingOptional: report-unavailable`).

## Boot plan (both hosts)

`expandPluginRegistry` (`kernel/🗂️plugin-registry`): primary, plus contributors of consumed tags that declare no activation events, plus the closure over `required` edges. New helpers `requiredDependencies`, `isActivatedOnDemand`, `onDemandPluginEntries` (what the plan left out), `onDemandActivationClosure` (what starting one plugin needs first).

`prepareWgpuPluginModules` (`wgpu/🧩️plugin-modules`):
- seeds with the expansion (catalog `dependsOn`, tags and events are passed in), enqueues `required` edges only;
- merges the catalog-declared hard dependencies into the graph edges. Procedural's Rust declares no `.depends_on`, so without this the flow hub would have been dropped from the plan once the nine extensions stopped pulling it in;
- returns `deferred` (what a request may activate) and `installed` (where required dependencies are looked up). A deferred descriptor is never read at boot, so an unstaged one cannot fail the boot;
- `assertWgpuPluginPlan` and `orderPluginRegistryEntries` check required edges only.

The React host runs the same `expandPluginRegistry` in `ShellHost/🛂️directory-route/🟠️shell-uri`, and the session file now carries `activationEvents` (`🎮️playground/🧭️session`).

## Classification of procedural

- `🌎️hub/🧩️compositions/🌀️procedural/.../Cargo.toml`: `depends-on = ["flow"]` (was the nine extensions). The nine remain reachable through `consumes = ["flow.extension"]`, and the dev build closure (`runtimeComponentClosure`) still builds and stages them: they are installed, not booted.
- Nine `flow-extension-*/Cargo.toml`: `activates-on = ["on-node-kind:<ns>"]` with `<ns>` = bim, brep, dictionary, draw, list, logic, math, `core` (primitive), text.
- `playbook-module-procedural` already declared `on-artifact-kind:s.playbook.procedural`, so it and `playbook` are no longer pulled into a 3D boot.
- Generated catalog: `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/{🔌️plugins.json (row 43), 🗂️sources.json, 🦀️catalog.json, 🧩️plugins/🟦️.ts}`. The real generator was run natively with `catalog-regenerate-native.ts` (dump in `🗑️generated/lazy-catalog-dump`); its output also shows unrelated peer drift (gis now depends on `stdio` in source, playground example labels lose their emoji), which was left alone. `lazy-catalog-patch.py` applies only the lazy-activation deltas, and the result matches the generator output byte for byte on those rows.

## On-demand activation

Kernel `⏳️on-demand-activation` (shared by both hosts):
- `resolveOnDemandOwners(candidates, request)`: owners ascending by id (same determinism as `artifactKindActivationOwner`).
- `createOnDemandActivator({candidates, installed, active, activate, onStatus})`: dependency closure first, phases `resolving, loading, active | unavailable | cancelled`, progress as closure position, joins a request already in flight, `cancel(pluginId?)` settles the caller at once, a caller `AbortSignal` is honoured, a failed plugin can be requested again. `request` never rejects: a missing or unpublished optional plugin yields `unavailable` with a reason (`not-installed`, `dependency-missing`, `load-failed`).
- `admitOnDemandRequest` validates a wire request; `onDemandStatusLabel(status, locale)` en/de text (no default language, an unknown locale tells English); `onDemandStatusAnnouncement` gives the ARIA role (`status`/polite while loading, `alert`/assertive when unavailable).

WGPU host: `createWgpuOnDemandActivator(prepared, ...)` reads and admits the descriptor (identity, version, required dependencies active), then mounts the module. The frame Worker (`🎞️frame-worker`) creates it at boot and exposes three doors: `semioWgpuActivateOnDemand(requestJson)` returns the outcome JSON, `semioWgpuCancelOnDemand(pluginId?)`, `semioWgpuOnDemandStatuses(locale)` (label plus announcement). The existing `semioWgpuInstallPlugin` door (the Rust shell's `install_plugin`, with its install band and cancel) now goes through the activator for a deferred plugin, so a cross-plugin open of a deferred extension gets descriptor admission first and shows the existing install band.

React host: `ShellHost/🚀️plugin-lifecycle/⏳️on-demand-activation` builds the same activator over `installPlugin` (module acquisition, install band, per-plugin abort). Phases other than `resolving` and `active` raise the shell's transient notice in the UI locale (`on-demand.<id>` key; `error` tone when unavailable). `usePluginLifecycle` returns `activateOnDemand` and `cancelOnDemand`; `semioReactActivateOnDemand` and `semioReactCancelOnDemand` mirror the WGPU doors.

## Tests (22, all pass)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/⏳️on-demand-activation/🟦️.ts` with the language-agnostic fixture `🧫️fixtures/⏳️on-demand-activation/🔣️.json` (catalog rows, expected reads, plan and deferred set, request-to-owner table, valid and invalid wire values, label texts):
- plan: primary, eager contributors and hard closure only; reads exactly the expected descriptors; the declared hard dependency joins the plan with no descriptor edge; a missing required dependency is still refused; an `on-demand` edge is neither read nor a graph requirement; an `on-demand` edge to a plugin without events is refused;
- parity: React registry expansion, kernel planner and WGPU boot plan select the same plugins; request resolution agrees on every fixture case;
- activation: node kind starts its extension (descriptor, mount, states, progress); join in flight; cancel and re-request; caller signal; missing optional descriptor reports unavailable without throwing; required dependency started first and missing one reported; wrong descriptor identity refused;
- labels and announcements en/de/region/unknown;
- schema against the independent validator `ajv` (draft 2020-12): requests, dependency edges (also through the real admission) and statuses; the enumerations match the labels;
- the committed catalog: procedural `dependsOn = [flow]`, nine extension events; `parseSemioActivatesOn` accepts and refuses.

Also green: discovery tests (10 of 10, with `SEMIO_TEST_ARTIFACT_DIR`), generated-projection (6 of 6), and the previous `prepareWgpuPluginModules` laws in `🧩️composition`. Five tests in `🧩️composition` fail before and after this change: four GIS ones (gis now depends on `stdio` in source but the committed catalog is not regenerated, peer work) and the serve command shape. `host-activation` (`prepare-hub-react-prototype builds one component`) is unrelated. A case was added to the kernel suite (`🧪️createturnoutcomebroadcast`, `expandPluginRegistry`); the kernel vitest runner needs an explicit policy environment and was not run, the same logic was checked under `bun test`. `tsc --strict` is clean for the kernel module, `plugin-modules` and the touched frame-worker files (one error at `🧵️state/🟦️.ts` `consumedTopicsForApp`, in code this change did not edit).

## What takes effect, and what is still open

Needs no Rust or wasm rebuild:
- the catalog (generated JSON and TS), the kernel and engine TypeScript, the frame Worker bundle. A running 6118 prototype needs its **TypeScript browser bundle rebuilt** (the frame Worker and shell, the vite or bundle step) and the session file regenerated at serve; the already staged `procedural` and `flow` components stay valid. The `Cargo.toml` metadata edits change the component build inputs, so `repo run <component>:component` would rebuild them if invoked, but it is not required to boot.
- With this change generation3d needs only `procedural` and `flow` staged. `brep` and `math` stage as on-demand extras.

Still open, needs Rust (and then a wasm rebuild of the renderer or flow hub):
- The trigger from inside the wasm: the flow hub or palette must call `semioWgpuInstallPlugin` (the existing `install_plugin`) or `semioWgpuActivateOnDemand({kind:"node-kind", nodeKind})` when a node kind is unknown, and `semioReactActivateOnDemand` equivalent in the React shell (the React shell UI is also produced by the wasm). Until then a 3D example that uses `brep.*` shows no activation of `flow-extension-brep` by itself; calling the door from the console or a host seam works. The shell's own install band (WGPU) and the transient notice (React) already show the states.
- Optional: `procedural`'s `plugin()` could declare `.depends_on("flow", tree_pin!())` so the descriptor also carries the hard edge; the boot already honours the catalog declaration.
