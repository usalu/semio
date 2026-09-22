# Browser21 Surface15 Practical Fixture Audit

Read-only source audit on 2026-09-21. I did not prepare, activate, serve, build, drive a browser, or change source. A registered route below establishes a runnable target and a published example asset; it does **not** establish a current runtime result.

## Decision

The existing product playgrounds cover the listed application families through ordinary, manifest-declared example selection. They do **not** cover `DiffView` or `EventFeed` as app-visible component scenes.

The smallest correct remaining seam is one registered, first-party **surface-fixture app** with ordinary declared examples, not a Shell debug flag and not a browser-side scene injection. It should project two actual `Component::Surface` bodies through the normal app/guest protocol:

- `diff-view`: a nonempty before/after pair, explicit language and mode.
- `event-feed`: multiple ordered entries, an activation action, then both non-follow scrolling and follow-at-tail cases.

Each is selected through the existing navbar's typed `setActiveExample` action. The app owns the example documents and its production projection; the physical acceptance probe only opens the normal example picker, awaits the normal accepted frame, and drives public pointer/wheel/keyboard behavior. That keeps the specimen app-visible without placing debug behavior in an existing product or bypassing app, admission, presentation, and input authority.

Adding either scene to Draw, Flow, Note, Writer, Raster, GIS, or Layout solely for this test would change that product's domain UI. Calling the renderer builders from a native test, Storybook, or the React conflicts panel does not close this acceptance gap.

## Existing registered application routes

The generated registry is authoritative for each variant, default port, and declared example id. Its discovery deliberately narrows a crate's `📚️examples` to examples declared by the owning app, preventing sibling or test-only fixtures from becoming selectable ([discovery](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts:121>), [discovery](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts:134>)).

| Family / variant | Registry app and default React / WGPU port | Published example asset | Browser21 use |
| --- | --- | --- | --- |
| Draw / `draw` | `s.draw.drawing@1/*#editor`; 6064 / 6164 ([registry](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:347>)) | `demo`, `demo-session`; `demo` is an `ExampleSource` over the committed DSL ([source](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs:5>)). | Use the existing canvas adapter after its normal Demo selection receipt. |
| Flow / `flow` | Registry has no `app` field; port 6016 / 6116 and both examples are declared ([registry](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:710>)). Do not substitute the internal `flow-play` controller for registry metadata. | `demo`, `demo-session`; Demo has a committed DSL carrier ([source](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs:5>)). | Use the existing NodeGraph physical probe and its normal Demo selection. |
| Layout / `layout` | `s.layout.layout@1/*#editor`; 6079 / 6179 ([registry](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:1036>)). | `demo`, `demo-session`; Demo is an `ExampleSource` ([source](</Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs:5>)). | Use the existing Catalogue-to-Canvas physical route. |
| Note / `note` | `s.note.note@1/*#editor`; 6080 / 6180 ([registry](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:1093>)). | `demo`, `demo-session`; Demo is an `ExampleSource` ([source](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs:5>)). | Use the existing Ink/text physical probe and require a fresh Demo selection receipt. |
| Writer / `writer` | Registry has no `app` field; port 6062 / 6162 and `demo` are declared ([registry](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:1972>)). Its artifact editor identifies itself as `s.writer.writer@1/*#editor` ([editor](</Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:46>)); that is not a reason to override the registry omission in launch code. | `demo` has a committed DSL asset ([source](</Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs:5>)). | Add a browser adapter only after a unique visible text-edit workflow is specified; no need to invent a seed. |
| Raster / `raster` | `s.raster.raster@1/*#editor`; 6060 / 6160 ([registry](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:1245>)). | `demo`, `demo-session`; Demo also owns the PNG it references ([source](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs:11>)). | Existing app-visible specimen; needs a dedicated Paint2d browser adapter, not a new seed. |
| GIS map / `gis2d` | `s.gis.gismap@1/*#editor`; 6040 / 6140 ([registry](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:823>)). | `demo`, `demo-session`; Demo's normal source is committed ([source](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs:5>)). Tile proxy routes `/osm` and `/vt` are declared ([registry](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:842>)). | Existing TiledMap specimen; acceptance must record asset readiness and avoid treating upstream tile availability as a renderer pass. |
| GIS terrain / `gis3d` | `s.gis.gisterrain@1/*#editor`; 6083 / 6183 ([registry](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:860>)). | `demo`, `demo-session`; committed Demo source ([source](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs:5>)). The `/dem` tile proxy is declared ([registry](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:879>)). | Existing World3d/TiledMap-family specimen; include readiness evidence for the DEM response. |

## Canonical execution and URL conventions

For any row above, use the generated target exactly; do not hand-assemble its producer closure:

```sh
bun nx run @semio-tech/framework-os-dev:activate-<variant>-<react|wgpu>-dev
bun nx run @semio-tech/framework-os-dev:serve-<variant>-<react|wgpu>-dev
```

The serve target depends on activation, which depends on preparation. React preparation owns the registry session, web support, fonts, selected materialization, and declared engines; WGPU adds the renderer wasm, browser boot, and frame-worker producers ([target generator](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1024>), [WGPU closure](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1040>)). The default address is `127.0.0.1:<registry port>`; WGPU's owned server announces the canonical `?plugin=<variant>` URL and accepts only `--port` / `--host` overrides ([server](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts:9>)). React receives its selected variant and registry app id through the receipt-backed serve environment, rather than a caller-provided app id ([serve](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:45>)).

The normal seed route is the navbar's `setActiveExample` action. The typed builder sends `{ exampleId }` through the app action funnel ([ShellHelpers](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:413>)); `editor_with_examples` is the existing registration path that makes a fixture selectable in the normal navbar ([PluginBuilder](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:466>)).

## DiffView and EventFeed: confirmed fixture gap

A repository-wide source search for producer fields `diff_view: Some` and `event_feed: Some` returned only:

1. the generic UI component builders, which merely construct a supplied `DiffViewScene` or `EventFeedScene` ([builders](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:3720>));
2. the WGPU renderer unit tests, which hand-author `UiComponentSceneNode` fixtures ([DiffView test](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-diff-view/🦀️.rs:65>), [EventFeed test](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-event-feed/🦀️.rs:65>)); and
3. the React interpreter hosts.

There is no current plugin/artifact application producer in that result set. Therefore neither native renderer test is an app-visible route.

React's Settings Conflicts panel is not an exception: it reuses `ConflictDiffPreview` without a `DiffViewScene` ([DiffViewHost](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔺️DiffViewHost/🟦️.tsx:144>)). Its WGPU peer expressly has no diff preview ([Shell](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8243>)). A generated or injected shell conflict could exercise only divergent implementations and cannot be claimed as `DiffView` surface parity.

## Required fixture contract

The dedicated fixture app should declare its two `ExampleSource` entries through `editor_with_examples`, and each example must be a normal persisted fixture document. Its scene body must be authored by the app and rendered through the same guest/app response as every product surface. The physical test must:

1. open the fixture app with its generated React or WGPU target;
2. select the named fixture using the visible navbar example picker;
3. wait for the accepted/presented app frame and discover the mounted host under its authored node key;
4. assert the scene's role/name and visible rows/changed lines;
5. drive physical wheel/keyboard/pointer input and assert its public action/result; and
6. record the full activation receipt, selected example action, DOM/accessibility evidence, and WGPU structure/ledger evidence.

Use two examples rather than a debug control, query parameter, synthetic `UiComponentSceneNode`, or post-boot runtime mutation. The existing generic builders and renderer tests remain valuable renderer contracts, but they do not replace this app fixture.

## Practical Browser21 ordering

1. Run the registered Draw, Flow, Layout, Note, Raster, GIS2D, GIS3D, and Writer routes only after each target's activation receipt exists.
2. Keep published examples as the source of seed data; no new fixture is needed for those families.
3. Add the dedicated two-example fixture app to close only DiffView and EventFeed.
4. Mark a family accepted only after the paired browser receipt proves the route actually mounted and its specified interaction changed the app-visible result.


## Addendum: scale fixture and Puzzle activation

The existing `@semio-tech/framework-os-scale-fixture` is not an eligible substitute for the dedicated app fixture. Its crate declares `role = "test"`, its component package is `semio:scale-fixture`, and it declares no playground row ([Cargo.toml](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/Cargo.toml:13>)). Its source has no `Component::Surface`, `ExampleSource`, `setActiveExample`, or app definition. Converting that synthetic actor benchmark into a browser app would mix distinct ownership and require a new plugin/app contract anyway. It is therefore not the smallest proper Surface15 fixture seam.

For the next `puzzle3d` WGPU activation, the registry row selects plugin id `puzzle` ([registry](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:1173>)); the crate's generated Nx project is `@semio-tech/puzzle-plugin`. The narrow producer that rebuilds the selected guest and its staged browser module is:

```sh
bun nx run @semio-tech/puzzle-plugin:materialize-dev
```

`materialize-dev` depends on the guest's `component-dev` producer and plugin-web support, and writes only Puzzle's staged module ([generator](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:873>)). Once it completes, the already-used narrow handoff can preserve the same stage:

```sh
bun nx run @semio-tech/framework-os-dev:prepare-puzzle3d-wgpu-dev --excludeTaskDependencies
bun nx run @semio-tech/framework-os-dev:activate-puzzle3d-wgpu-dev --excludeTaskDependencies
```

Preparation confirms Puzzle's staged descriptor, bridge, and Nx marker ([preparation](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts:81>)); activation hashes those staged bytes into its receipt ([execution](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts:101>)). At serve time the selected component is also reported stale when source is newer than its stage ([freshness](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts:231>)). This avoids rerunning the WGPU host wasm producer while still making a stale Puzzle guest observable.
