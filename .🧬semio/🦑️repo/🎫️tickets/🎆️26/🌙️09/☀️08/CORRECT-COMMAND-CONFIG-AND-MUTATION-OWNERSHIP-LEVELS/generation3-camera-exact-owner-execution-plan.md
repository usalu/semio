# Generation3d Exact Camera Owner Execution Plan

## Boundary

This is the follow-on source plan requested while Generation2d acceptance runs. It makes no Generation3d production change. The editor has three camera-consuming concrete window kinds: `procedural-main`, `procedural-preview`, and `generation3d-generate-preview`. The separate read-only viewer remains outside this cutover.

The main graph currently renders `document.fixture.camera`, while `NodeGraphViewport` writes `Generation3dConfig.camera`; the app camera is therefore disconnected from its renderer. Both World3d previews render the same `Generation3dConfig.preview_camera`, and `setCamera` writes that shared app field. `SetActiveExample` copies `target.fixture.camera` only into the disconnected app camera. `generation3d_fixture_operations` deliberately excludes camera, so selecting an example does not change the authored `document.fixture.camera`.

## Exact schema owners

Create and register three closed, schema-first `WindowConfigOwner` modules beside their windows:

| Exact kind | Owner | State |
| --- | --- | --- |
| `procedural-main` | `Generation3dMainWindowConfigOwner` | `viewport: Viewport2d` |
| `procedural-preview` | `Generation3dEditPreviewWindowConfigOwner` | `camera: Generation3dPreviewCamera` |
| `generation3d-generate-preview` | `Generation3dGeneratePreviewWindowConfigOwner` | `camera: Generation3dPreviewCamera` |

Each owner needs a distinct schema id and envelope id, Rust/JSON Schema/TypeScript/GraphQL/proto/WIT facets, an explicitly tagged closed Snapshot mutation, whole-record inverse/diff behavior, the normal 2 MiB bounded owner/disposer path, and exact Pack identity checks. The preview value type may be shared, but owner identities and stored lifetimes remain distinct. Default preview poses must preserve today’s authored values `[4,-4,3]`, target `[0,0,0]`, and FOV `45`.

## Main graph seed boundary

The main owner has a document-dependent first-open rule rather than a document-dependent `Default`:

1. Capture a trusted `ViewModel` for a concrete `procedural-main` instance.
2. Check the exact owner partition for that instance before publication.
3. If an exact owner snapshot or restored Pack exists, publish nothing.
4. If the exact owner is absent, construct one `Generation3dMainWindowConfig` from the current `document.fixture.camera` and publish one addressed WindowConfig Snapshot.
5. Rendering reads only that exact WindowConfig after provisioning. It does not fall back to `fixture.camera` on every render.

The provisioning trigger must be an explicit retained window-open transaction at the editor boundary. It must not be hidden in `Default`, renderer code, or `SetActiveExample`. The completed seam audit found that the current framework cannot express this transaction: `SurfaceVisible` only stores the narrowed `ViewModel`, `ArtifactApp` has no window-open callback, and the first `WindowConfigOwnerRegistry::capture` eagerly constructs `State::default()`. That capture happens before the app renderer receives `ConfigView`, erasing the distinction between an absent owner and a default-valued existing owner. Generation3d production implementation must wait for a coordinated framework capability that exposes exact absence and publishes the seed before the first render. See `generation3-window-open-provisioning-seam-audit.md`.

This rule preserves the current authored document camera for the first open and preserves user-restored window state thereafter. Two newly opened main instances may each seed from that document value, then diverge independently.

## Command and renderer cutover

Move `NodeGraphViewport` to the retained reducer. Require captured context and the exact `procedural-main` kind, read the exact main owner, replace only its `Viewport2d`, and emit one WindowConfig lane result. The raw handler becomes an explicit no-op. Change the tool publication contract from Config to WindowConfig.

Move `setCamera` to a retained exact-window route. The caller’s trusted kind selects either the edit-preview owner or generate-preview owner. Missing, stale, main-kind, or unrelated contexts fail; the payload never supplies a window id. The raw handler becomes an explicit no-op. The publication contract becomes WindowConfig.

In `generation3d_render_body`, retain the current app config for LOD/show mode/sun/selection and resolve the exact camera owner separately for each camera surface. The main renderer accepts its exact `Viewport2d` and no longer reads `fixture.camera`. Each preview renderer accepts its corresponding exact camera state, and `preview_camera_json` accepts `Generation3dPreviewCamera` rather than `Generation3dConfig`.

## SetActiveExample invariant

Remove only the camera assignment from `config_after_example_load`; preserve selected-generation reset plus existing LOD/show mode/sun behavior. `SetActiveExample` continues through its cooperative retained route and keeps its artifact/transient evaluation work.

It must not publish any WindowConfig mutation and must not replace, delete, or reseed existing main or preview packs. Because the command does not author `target.fixture.camera`, a newly opened `procedural-main` after the switch seeds from the unchanged current document camera. A target example camera must not be claimed as a seed unless a separate, explicit document mutation actually authors it. An already-open main instance, including one restored from Pack, wins unchanged.

After all three render and command routes are live, remove `Generation3dConfig.camera`, `preview_camera`, `SetCamera`, and `SetPreviewCamera` from every app-config language facet and obsolete tests. Preserve the `setCamera` command itself because it is the typed World3d host gesture; only its retained owner changes. Do not change existing action classifications without handler evidence.

## Acceptance sequence

1. Add a neutral fixture and JSON Schema for two main, two edit-preview, and two generate-preview instances with distinct values. Make independent Ajv and JSON Patch checks pass first, including closed state/mutation rejection.
2. Add the three owner codecs and hostile missing/foreign-owner/wrong-component/wrong-version Pack cases.
3. After the framework seam exists, prove first-open main provisioning: an absent owner seeds once from the current authored document camera; an existing owner is unchanged after document replacement and after `SetActiveExample`; a restored owner wins. Prove that `SetActiveExample` alone does not adopt the target example camera.
4. Exercise actual retained `NodeGraphViewport` and `setCamera` routes. Assert one WindowConfig lane and zero Config lanes; reject missing, stale, and every wrong kind.
5. Render all six concrete instances and assert exact NodeGraph viewport and World3d pose isolation. Reopen their packs and render the same six values.
6. Run `SetActiveExample` with all six packs present and assert byte-identical WindowConfig Pack/SPR for every owner, while its existing artifact/transient work still completes.
7. Assert document and app-config Pack/SPR remain unchanged around viewport-only commands. Close both app instances on explicit 2 MiB threads.
8. Remove obsolete app camera facets and rerun the focused native route plus the existing example-switch/fold and Generation3d editor checks affected by the schema change.

The focused native test must report the concrete retained route receipts. Raw handler calls alone do not prove routing, and direct owner mutation calls alone do not prove command addressing.

Future Generation3d oracle/native launch routes are reserved at `311.234` and `311.235`; they should be registered only when the implementation targets exist.
