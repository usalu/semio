# 🧩️ Aggregator (`s.puzzle.puzzle3d@1/*#editor`) — fixture / window / interactivity diagnosis

Plugin: `✏️s/🔌️plugins/🧩️puzzle`, artifact tree `🗿️artifacts/🧊️3d`. Default example: `concrete-forest`.

**Headline: this app is in the best shape of the six. Real fixture, real render path, real commands.**

## 1. Editor and default windows

`.../✏️editor/🎭️modes/✏️edit/🦀️component.rs:24-42` — `layout()` is a row split: left third "Top",
right two-thirds "Perspective". Both are instances of ONE window kind.

`.../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️component.rs:30-34,43-60`:

```rust
pub const WINDOW_KIND_ID: &str = "puzzle3d-main";
pub const WINDOW_INSTANCE_TOP: &str = "puzzle3d-main-top";
pub const WINDOW_INSTANCE_PERSPECTIVE: &str = "puzzle3d-main-perspective";
pub const BODY_KEY: &str = "puzzle3d.play.composite";
surface_kind: SurfaceKind::World3d,
```

Two default window instances: **Top** (orthographic, `TEMPLATE_TOP`) and **Perspective**
(three-point, fov 50, `TEMPLATE_PERSPECTIVE`).

## 2. `setActiveExample` — `concrete-forest` is real

`.../✏️editor/🎮️commands/🛍️set-active-example/🦀️component.rs:7-23` branches on
`PUZZLE3D_EXAMPLE_CONCRETE_FOREST` (alias `"concrete"`) → `default_fixture()`, and
`PUZZLE3D_EXAMPLE_NAKAGIN` (alias `"nakagin"`) → `nakagin_fixture()`; empty id → `empty_fixture()`.

`default_fixture()` (`✏️editor/🦀️component.rs:279-281`) clones `CONCRETE_FOREST_EXAMPLE_FIXTURE`, a
`LazyLock` parsed (`:101,103`) from `PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT`, itself
`include_str!(".../📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio")`
(`🧬️schema/📸️snapshot/📝️text/🦀️component.rs:13`).

That 52-line DSL holds real content: object `seed-left-001` ("Hexagonal Cut Concrete Forest Left") with
`mesh-url="/mesh/🧊️hexagonal-cut-concrete-forest-left.glb"` and 11 nested vortices, 2 background
reference images, plus vortex/cable/attraction/kind-compatibility catalogs.

It is also the app's genuine initial document — `initial_snapshot()` (`:6455-6458`) and
`create_puzzle3d_app()` (`:6766-6767`) both seed with `default_fixture()`, asserted by the test
`initial_snapshot_is_the_concrete_forest_fixture` (`:7866`).

## 3. Document → surface

`resolve_object_mesh_url`/`collect_mesh_urls` (`:455-487`) take `object.mesh_url` (present on the seed
object) or fall back to `fixture.meta.kind_catalogs`. Geometry is fingerprint-cached (`:2118`) and
consumed at `:6619-6620`:

```rust
let (instances_json, meshes_json) = app.geometry_jsons(&envelope.fixture);
main::render(&envelope, &precompute, labels, instances_json, meshes_json)
```

`main::render` (`🪟️windows/🧊️main/🦀️component.rs:460-491`) builds `world3d_scene_extended(...)`
(camera, meshes, instances, selection, vortices, attractions, target volumes, references, brush preview,
interaction, LOD, chunking, environment/sun) and emits it via
`semio_framework_plugin::scene_surface(SURFACE_VIEWPORT, SurfaceKind::World3d, &scene)`, consumed by
the framework's `World3dHost` React component. **Both requirements (non-empty `meshUrl`, non-empty
`instances_json`) are satisfied for `concrete-forest`.**

## 4. Interactivity

No `todo!()`, `unimplemented!()`, `FIXME` or `TODO` anywhere under `🗿️artifacts/🧊️3d`. All 56 command
directories carry real logic — e.g. `🎮️commands/🔄️translate-selection/🦀️component.rs:13-20` and
`🎮️commands/🖌️accept-suggestion/🦀️component.rs:16-33` both mutate `ctx.scene` / dispatch to a real
precompute engine.

Selection/hover are framework-owned reserved verbs (`interactionSelect`, `interactionHover`,
`clearSelection`, `selectAll`, `setSelectionMode`, `setInteractionGranularity`), routed at
`✏️editor/🦀️component.rs:7069-7094` to `app.handle_action` — real, relocated by ticket
26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.

One documented limitation: `world_interaction_json(...)` (`🧊️main/🦀️component.rs:483-487`) leaves the
OS generic `pick_select_action`/`pick_hover_action` as `None` because this app emits its own bespoke
vortex-pick select/hover for the `"vortex"` interaction domain — marked as a follow-up.

## 5. Panels

- **document** (`📌️panels/📄️artifact/🦀️component.rs:130-186`, body `puzzle.3d.play.document`) — real tree
  from `fixture.objects` (+nested vortices), `.references`, `.target_volumes`, `.attractions`, with
  select/hide/lock actions wired. ✅
- **catalogue** (`📌️panels/🛍️catalogue/🦀️component.rs:126-139`, body `puzzle.3d.play.kinds`) — built from
  `puzzle3d_catalog_entries(&envelope.fixture, ...)`, which reads `fixture.meta.kind_catalogs`
  (`✏️editor/🦀️component.rs:548-550`). **GAP: `forest.dsl.semio:4` leaves `kind-catalogs` empty**, so all
  four catalogue sections render empty for the default document — nothing is draggable into the viewport
  out of the box. (`nakagin-capsule-tower` does populate them.)
- **inspection** (`📌️panels/🔍️inspection/🦀️component.rs:27-42`) — **documented regression**: it used to
  switch on `envelope.runtime.selection`; selection is now framework-owned and is not threaded into panel
  `render()`, so it always falls through to a schema/domain/object-count summary.
- **settings** (`📌️panels/⚙️settings/🦀️component.rs:40-55`) — real, from `envelope.runtime` (overlap
  budget, proximity radius, chunk size, grid spacing). ✅

## 6. Blank / inert paths

Only the two above (empty catalogue for this example; inspection detail loss). The single `"placeholder"`
hit at `🧊️main/🦀️component.rs:510` is an engagement-input textbox placeholder string, not a stub.

## 7. Tutorial `tracks.document`

Lives in `♻️mit-bestand/🧺️demonstrator/🟦️brand.ts` — `ENTWERFEN_MIT_BESTAND_TUTORIAL` (`:130-345`), with
`tracks.document: []` at `:277`. (Not `📦️index.tsx`.)

Expected element type — `TutorialArtifactEvent`, `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts:715-724`:

```ts
export type TutorialArtifactEventKind =
  | { kind: "edit"; forwards: readonly unknown[]; backwards: readonly unknown[]; description?: string; coalesceKey?: string }
  | { kind: "undo" } | { kind: "redo" }
  | { kind: "checkpoint"; message?: string }
  | { kind: "checkoutCheckpoint"; checkpointId: string }
  | { kind: "switchAlternative"; alternativeId: string }
  | { kind: "load"; documentDsl: string; previousDsl: string };
export type TutorialArtifactEvent = { at: number; kind: TutorialArtifactEventKind };
```

The comment at `brand.ts:113-129` states what must be captured: real `forwards`/`backwards` op JSON
mirroring a live `vcs::Edit`, timed against the already-authored narration/events/ui/camera tracks — i.e.
the edits behind the four annotational events at `brand.ts:266-269`: `addObjectKind` @110s,
`setVortexShow(show:"always")` @141s, `acceptSuggestion` @165s, `setFillCount(count:40)` @181s.
Authoring path: run the tutorial recorder against a live Aggregator session performing exactly that
sequence, then merge the captured `document` track (and reconcile `camera`) into this skeleton.
Hand-inventing op JSON is explicitly rejected as "silently wrong".

## 8. Verdict

A user today sees a real, visible concrete-forest board in two viewports and can transform / select /
brush / fill / accept-suggestion interactively. Remaining gaps, in priority order:

1. **Catalogue empty for `concrete-forest`** — populate `meta.kind-catalogs` in
   `📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio`, so kinds are draggable.
2. **Inspection panel shows no per-entity fields** — thread framework-owned selection into panel
   `render()` (cross-cutting; affects other apps too).
3. **Tutorial `tracks.document` empty** — needs a recorder pass, not hand-authoring.
4. Optional: wire generic `pick_select_action`/`pick_hover_action` alongside the bespoke vortex pick.
