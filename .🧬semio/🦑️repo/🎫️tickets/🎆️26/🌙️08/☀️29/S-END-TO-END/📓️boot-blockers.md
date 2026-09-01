# 🚧️ Why `s` does not render — the chain, and where it now ends

Every route to running `s` (os/dev server, storybook static, storybook dev) goes through the same
browser bundle. This is the chain of blockers found on the way, in the order they surfaced. Each was
hiding the next, which is why the picture only sharpened at the end.

## Cleared

| # | blocker | fix |
|---|---|---|
| 1 | `📜️script.ts` refused to start repo-wide: a peer's half-added `fixedFilenameContracts["cargo-integration-test"]` failed the repo's own taxonomy validator | removed the invalid half, kept their valid `fixedDirectoryContracts["cargo-integration-tests"]`; their exact diff preserved at `📓️peer-taxonomy-cargo-integration-test.patch` |
| 2 | storybook could not resolve `@semio-tech/ui-react/test` (Vite substitutes aliases by prefix, so it became a literal `<uiReactDir>/test`) | registered the more specific alias first, mirroring os/dev's `⚙️vite.config.ts` |
| 3 | Node built-ins (`node:crypto`/`fs`/`path`/`url`) pulled into the browser worker bundle | `🎭️actor/🚪️lifetime/🟦️component.ts`'s `import.meta.vitest`-only block dynamically imported the BUILD-time `🌐plugin-web-materialize.ts`. Vite follows dynamic imports regardless of the guard. Specifier moved into a variable + `@vite-ignore` |

Clearing 3 produced the **first successful storybook preview build** — `iframe.html` and an `assets/`
bundle now exist. That build had been broken long enough for ticket `26/07/27` to document it.

## Where it ends

The built preview loads and then dies before mounting:

```
Uncaught ReferenceError: Cannot access 'Vbe' before initialization    (assets/iframe-CVwfUATw.js)
```

`window.__STORYBOOK_PREVIEW__` stays `undefined`, `#storybook-root` stays empty, and no readiness
beacon is ever stamped — so `s` never boots.

Rollup names the cause in its own build warnings:

```
▲ Vite Circular chunk: 🟦️component -> 🧵️shard-client -> 🟦️component.
▲ Vite Circular chunk: 🧵️shard-client -> 🟦️ -> 🧵️shard-client.
▲ Vite Circular chunk: 🧵️shard-client -> 🟦️ -> 🟦️component -> 🟦️component -> 🧵️shard-client.
```

**It is a genuine source-level import cycle, not a chunking artifact.** That was tested: forcing the
whole preview into a single chunk (`manualChunks: () => "preview"`) removed all 7 circular-chunk
warnings, produced a different bundle — and the TDZ persisted, at a different symbol (`Gur`, in
`iframe-CdOQTKd2.js`). A cycle that breaks *within* one chunk is a real cycle with top-level side
effects, which no bundler layout can reorder into working code. The speculative chunking override was
therefore **reverted** rather than left in shared config as a fix that does not fix anything.

The cycle runs through `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`, which imports
`🚪️lifetime/🟦️component.ts`, `🚪️lifetime/🩹️patch/🟦️component.ts`,
`🪪️activation/🚪️instance/📥️output/🟦️component.ts`, `📄️page/🟦️component.ts` and
`📤️return/🟦️component.ts` — and something in that set imports back. Breaking it is a real refactor
in the actor layer, in files peers are actively editing, and is the one thing standing between this
work and a rendered `s`.

## Also still open

The Rust plugin fleet cannot be measured: `semio-framework-plugin` (864 errors) and
`semio-framework-os-infinite` (283) are mid-migration from two concurrent peer refactors — the
`protocol::Mutation` serde→`ToValue`/`FromValue` supertrait change, and a `ui/wgpu` glue rewrite that
moved exports (`BoundedActionFault`, `Mesh3dFault`, `checked_action_string_bytes`). 16 further errors
are an unwired `semio_framework_value_derive` dep, the same missing-Cargo-dep pattern cleared four
times already in this ticket.

---

# ✅️ Update — the browser chain is fixed; `s`'s shell now mounts and renders

Continuing past the diagnosis above, the cycle **was** broken and three further blockers behind it.

## 4. The TDZ — found exactly, fixed

The minified `Cannot access 'Vbe' before initialization` decoded to
`decorators:[Vbe,zbe,Hbe,...]` — the Storybook `preview` config's own decorator list.

`.storybook/preview.tsx` declares `const preview: Preview = { … decorators: [withAppearance, …] }`
at **line 60**, while every one of those decorators is an `export const` declared at **line 225+**.
The object literal is evaluated at module-evaluation time, so it reads all seven while they are still
in their temporal dead zone. Unconditional, and it broke the storybook **build and dev server alike**
— which is why `window.__STORYBOOK_PREVIEW__` was always `undefined` and every story rendered an
empty `#storybook-root`.

Fixed by moving the assignment to the bottom of the file, after the definitions
(`preview.decorators = [...]`). The preview runtime boots immediately after.

**The earlier "genuine source-level cycle" conclusion was wrong**, and the chunking work chasing it
was a dead end — correctly reverted. The real defect was ordinary declaration order in config code.
The `shard-client` lazy-getter change (a top-level `OwnedResidentLedger.prototype` read across a real
import cycle) is kept: it is a genuine latent TDZ of the same family, just not the one that was firing.

## 5. Stories drifted from the registry — fixed

`plugins.stories.tsx` asserts one literal `export const <PascalCase>` per `PLUGIN_BUILD_TARGETS`
entry. The registry had swapped four entries: `playbook-module-procedural` and
`sourcing-module-{beams,slabs,windows}` left, `block`/`demonstrator`/`energy`/`stdio` arrived. Synced
both the exports and the `EXPORTED_STORIES` map; verified zero missing and zero extra against the 33
build targets.

## 6. `/plugin-modules/` was not served — fixed

`OsBootHost` probes `/plugin-modules/<pluginId>/<wasmOut>.js`; storybook declared no `staticDirs`, so
every plugin story rendered the shell's own "plugin artifact missing" panel. Added the dev
materialize output as a static dir.

## What `s` does now

```
scopes: 1          .semio-scope[data-shell-id] present
levelBase: true    [data-level='base'] rendered
portal: true       [data-semio-portal-layer] rendered
beacon:            semioOsError = "s"   (fires correctly — reached a definite outcome)
body:              "No plugins loaded / Agent disconnected"
```

**The `s` shell mounts and renders in a real browser.** It reaches a definite, beacon-reported
outcome rather than a blank page.

## 7. The terminal blocker — the plugin binaries predate the current WIT world

Regenerating the host-side glue (`🟨️host-shim.js` and the per-plugin bridge, both pure string
generators — `🗑️generated/regen-shims.ts`) moved the error from
`bridge.createActorApi is not a function` to `Cannot read properties of undefined (reading 'poll')`.

The cause is not fixable from the host side:

| | exports |
|---|---|
| materialized `semio_s_plugin_space_component.js` (**Aug 17**) | `{ contributor, plugin }` |
| what the current bridge destructures | `{ reactor, jobs, checkpoint, describe }` |

The prebuilt artifacts are from the **previous WIT world**, before the reactor/actor model. No
host-side regeneration can bridge that — `semio-s-plugin-space` has to be rebuilt for
`wasm32-wasip2`, which needs the Rust fleet, which is currently 986 errors deep in two peers'
in-flight refactors (`protocol::Mutation` serde→`ToValue`/`FromValue`, and the ui/wgpu glue rewrite).

So the two halves of this ticket meet exactly here: the stdio mutation-leaf migration this ticket
delivered is part of what has to land before that rebuild can happen.
