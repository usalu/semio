# Generation Camera Exact Owner Migration Plan

## Scope and evidence

This is a bounded read-only source plan for the Generation2D and Generation3D editor camera fields. No Cargo, browser, or native test ran for this report. Each route below is source evidence only; it does not prove an observed multi-window collision or a runtime screen result.

It corrects [Node Graph Window Owner Follow-up Audit](node-graph-window-owner-follow-up-audit.md). That audit now records both the three live Generation2D readers and the Generation3D graph's current writer/reader disconnect. The shared typed NodeGraph viewport transport remains separately proven by [Node Graph Viewport Owner Repair](node-graph-viewport-owner-repair.md); typed transport does not choose a retained state owner.

## Current camera inventory

### Generation2D editor

| Value and current owner | Actual writers | Actual renderer readers | Result |
| --- | --- | --- | --- |
| Generation2dConfig.camera, one app-config CameraJson | NodeGraphViewport converts its typed Viewport2d and emits Generation2dConfigMutation::SetCamera. | generation2d-main passes it to NodeGraphScene; generation2d-preview passes it to Canvas2dScene; generation2d-generate-preview passes it to Canvas2dScene. All three are reached through one ConfigView snapshot in generation2d_render_body. | One persisted camera controls a NodeGraph and two different Canvas2d windows. |
| Document fixture.camera | The artifact UpdateCamera mutation exists in the snapshot/mutation contract. No editor command producer was found in this audit. | No inspected Generation2D editor renderer reads it. | It is an authored document facet, not evidence for a live editor-window camera owner. |
| Canvas pointer and wheel commands | generation2d-preview and generation2d-generate-preview both declare canvasPointerDown, canvasPointerMove, canvasPointerUp, and canvasWheel. Each command has an empty payload and returns Emit::default(). | None. | They are current no-output routes, not camera producers. Source does not establish whether every host gesture reaches them, only that an invocation cannot persist or alter a camera. |

The edit and generate previews are separate concrete window kinds:

| Window kind | Body | Surface | Camera reader |
| --- | --- | --- | --- |
| generation2d-preview | generation2d.play.preview | Canvas2d | edit-preview Canvas2dScene |
| generation2d-generate-preview | generation2d.play.generate-preview | Canvas2d, with a TextEditor fallback for non-layer output | generate-preview Canvas2dScene |

They share a surface family and label, not a window kind or a view-state lifetime. The edit preview renders evaluated drawing handles and the generate preview renders generated output. They must have separate exact owners. A shared Canvas2d camera owner would preserve cross-kind coupling even after app config is removed.

The read-only Generation2D viewer is a separate application. Its preview emits a fixed Canvas2d camera of zero, zero, one and does not read Generation2dConfig.camera. It is not a consumer to migrate in this slice.

### Generation3D editor

| Value and current owner | Actual writers | Actual renderer readers | Result |
| --- | --- | --- | --- |
| Generation3dConfig.camera, one app-config CameraJson | NodeGraphViewport emits SetCamera. SetActiveExample reads target.fixture.camera and writes the whole app config through config_after_example_load. | No inspected Generation3D editor renderer reads this field. | The field is persisted and written, but is disconnected from the rendered graph. |
| Document fixture.camera | Bundled examples carry it. The procedural-main graph renderer reads document.fixture.camera for NodeGraphScene. The generic artifact UpdateCamera mutation also exists. | procedural-main only. | It is the current graph scene input and is an authored fixture value, not a per-window live view. |
| Generation3dConfig.preview_camera, one app-config Generation3dPreviewCamera | The World3d setCamera action emits SetPreviewCamera. The manifest assigns that action to both preview kinds. | procedural-preview and generation3d-generate-preview both call preview_camera_json, which reads the app field. | One persisted World3d pose controls both distinct preview window kinds. |
| Editor presence camera and preview_camera declarations | No non-schema editor command producer was found in the inspected command tree. | No inspected editor renderer reads these presence records. | They are not a demonstrated competing live owner. Do not use the declarations to justify a migration or a runtime claim. |

The Generation3D graph and previews have three distinct concrete kinds:

| Window kind | Surface | Current camera reader |
| --- | --- | --- |
| procedural-main | NodeGraph | document.fixture.camera |
| procedural-preview | World3d | Generation3dConfig.preview_camera |
| generation3d-generate-preview | World3d | Generation3dConfig.preview_camera |

The two World3d previews are distinct kinds even though both use the same world camera record and camera command. Their evaluated payload, transient/evaluation lifecycle, and app mode differ. They need separate exact camera records.

Generation3D's read-only viewer is also separate. Its Generation3dViewConfig and viewer SetCamera command own the viewer preview camera and its viewer renderer reads that viewer config. It must not be coupled to, or used to implement, the editor migration.

## The authored seed is not a persisted window view

Generation3D SetActiveExample is the decisive source path:

1. It reads a target bundled snapshot and takes target.fixture.camera.
2. generation3d_fixture_operations deliberately ignores fixture.camera. The existing fold contract asserts that an example selection does not author the document camera through the artifact lane.
3. config_after_example_load currently copies target.fixture.camera into Generation3dConfig.camera, while preserving the global preview camera.
4. The procedural-main renderer still reads document.fixture.camera, so the current config write has no direct renderer consumer.

The target example camera is therefore an authored selection seed. It cannot be relabelled as the user's restorable live graph view. Existing window configuration must never be continuously derived from fixture.camera during rendering or reopen.

The replacement rule is:

- An absent new procedural-main WindowConfig may be initialized once from the then-current document fixture.camera through an explicit window-open provisioning transaction.
- An existing exact config, including a reopened config pack, wins over the authored seed.
- SetActiveExample remains valid without a trusted procedural-main ViewModel. In that case it publishes only its artifact/transient work and does not mutate any window camera.
- If product behavior requires the initiating procedural-main window to refit to a selected example, add an explicit, captured-current-window action that writes only that exact owner. It must never change every main window or write a document camera.

The existing WindowConfigOwner default pattern is static, so document-dependent initialization cannot be hidden in Default or in a renderer fallback. It needs a first-open transaction with a clear “owner absent” predicate. Rendering must only read the resulting exact snapshot.

## Target ownership schema

Use the existing Flow Main WindowConfigOwner pattern: a typed owner declares one WINDOW_KIND_ID, registration installs it in the WindowConfigOwnerRegistry, render obtains the addressed config from ConfigView.window, and mutation publication validates ViewModel.window_id against the registered concrete kind before emitting WindowConfigMutation::of. No command payload accepts a window id.

Use the shared Viewport2d record for 2D fields. Do not create another CameraJson-shaped persisted window record only to convert it back to Viewport2d at the NodeGraph boundary.

| Editor | Exact owner and state | Replaces |
| --- | --- | --- |
| Generation2D | Generation2dMainWindowConfigOwner for generation2d-main with viewport: Viewport2d | Generation2dConfig.camera for NodeGraph |
| Generation2D | Generation2dEditPreviewWindowConfigOwner for generation2d-preview with viewport: Viewport2d | The same app field for edit Canvas2d |
| Generation2D | Generation2dGeneratePreviewWindowConfigOwner for generation2d-generate-preview with viewport: Viewport2d | The same app field for generate Canvas2d |
| Generation3D | Generation3dMainWindowConfigOwner for procedural-main with viewport: Viewport2d | Generation3dConfig.camera and the document camera renderer dependency |
| Generation3D | Generation3dEditPreviewWindowConfigOwner for procedural-preview with camera: Generation3dPreviewCamera | Generation3dConfig.preview_camera for edit World3d |
| Generation3D | Generation3dGeneratePreviewWindowConfigOwner for generation3d-generate-preview with camera: Generation3dPreviewCamera | The same app field for generate World3d |

The duplicated preview-camera field type is acceptable because it describes two independent exact persisted records. The records must have distinct owner schema identities and Pack envelopes; a common Rust value type is not a common store owner.

After those owners exist:

- Generation2dConfig drops camera and SetCamera in Rust, JSON Schema, TypeScript, GraphQL, proto, and WIT facets.
- Generation3dConfig drops camera, preview_camera, SetCamera, and SetPreviewCamera in the same facets and removes their mutation-schema leaves.
- Existing app config values such as show mode, LOD, sun, and selected generation are outside this camera slice. Their continued app scope is not a proof that the camera move is incomplete or correct.

## Actual command and render migrations

### Generation2D

NodeGraphViewport must become a retained/direct exact-window route. It captures the trusted ViewModel from the operation request, requires generation2d-main, reads that exact main config, replaces only its Viewport2d, and emits one addressed WindowConfig mutation. The raw app-config handler cannot remain the final destination after SetCamera is removed.

generation2d_render_body currently forwards one app ConfigView snapshot to every body. Its request-context path must instead resolve the owner appropriate to the rendered body:

- generation2d-main reads only Generation2dMainWindowConfigOwner;
- generation2d-preview reads only Generation2dEditPreviewWindowConfigOwner;
- generation2d-generate-preview reads only Generation2dGeneratePreviewWindowConfigOwner.

The Canvas2d scenes lower only their own exact Viewport2d to camera_x, camera_y, and zoom. They do not sample the main graph config.

The four Canvas pointer/wheel commands require no camera-owner rewrite today because they return empty Emit and carry no coordinates or viewport. Keep their no-output behavior explicit in the migration test. A later interactive Canvas2d camera feature must introduce a typed viewport-bearing command and a kind-checked exact-window route; it must not revive a shared Generation2dConfig.camera.

### Generation3D

NodeGraphViewport must address only a captured procedural-main owner and replace its Viewport2d. The flow renderer must read that exact record rather than fixture.camera. The fixture value participates only in first-open seed provisioning described above.

The existing SetCamera command is already declared solely on procedural-preview and generation3d-generate-preview. Its retained/direct route must capture and validate the caller’s ViewModel, then address the matching edit-preview or generate-preview owner. preview_camera_json should accept that exact preview state, so neither World3d renderer can access app config for its pose.

SetActiveExample must remove config_after_example_load’s camera assignment. It must preserve the existing artifact rule that fixture operations do not mutate fixture.camera, keep existing exact camera packs unchanged, and avoid any broad replacement of preview records. Its document-derived seed is consumed only by the explicit absent-owner initialization path.

The existing exact Generation3D preview-evaluation transient owner remains unchanged: it owns computed evaluation text for procedural-preview, while the camera is a separate persisted local WindowConfig concern.

## Schema-first execution slices

1. **Generation2D complete camera cutover.** Define and register all three exact window schemas before deleting Generation2dConfig.camera. Rewire all three renderer body branches and the NodeGraph action in the same change. The two Canvas preview owners belong in this slice because leaving either on the app field would retain the alias.
2. **Generation3D main graph and seed boundary.** Define procedural-main config, the single-use authored seed provisioning transaction, and the exact NodeGraph route. Remove the write-only global graph camera and make the graph renderer read its exact config.
3. **Generation3D World3d preview split.** Define both preview owners, make SetCamera kind-addressed, and redirect edit/generate renderers. Delete the global preview camera only when both have moved.
4. **Contract cleanup.** Remove obsolete app-config mutation records and their all-language schema facets only after the exact owner Pack codecs, wrong-kind rejection, and request-context render paths are in place. Do not leave app-config compatibility fields or adapters.

Each exact owner slice needs Rust, JSON Schema, TypeScript, GraphQL, proto, and WIT definitions, whole-record mutation/inverse codecs, bounded store owners/disposal, owner registration, and exact inner Pack identity rejection. It must follow the already-open exact WindowConfig Pack identity correction rather than weakening its identity check.

## Required proof targets

Proposed focused law names:

    generation2d_main_camera_isolates_and_reopens_exact_windows
    generation2d_edit_preview_camera_isolates_and_reopens_exact_windows
    generation2d_generate_preview_camera_isolates_and_reopens_exact_windows
    generation2d_canvas_input_commands_publish_no_camera_mutation
    generation3d_main_camera_uses_exact_window_config_and_authored_seed_once
    generation3d_edit_preview_camera_isolates_and_reopens_exact_windows
    generation3d_generate_preview_camera_isolates_and_reopens_exact_windows
    generation3d_set_active_example_preserves_exact_window_cameras

For each camera law, construct two same-kind concrete ids, give them different cameras, render both scenes, capture exact config Packs, recreate an app, reload the corresponding Packs, and render the same separate values. Include at least one cross-kind case, so a main/preview or edit/generate camera cannot silently share a record.

Every command law must reject missing, stale, and wrong-kind ViewModel identities; must not accept a caller-supplied window id; and must verify that a viewport-only action leaves document and app-config Pack/SPR bytes unchanged. The Generation3D example law additionally needs these cases:

- a selected example’s fixture.camera initializes an absent main owner once;
- changing the document or selecting another example does not overwrite an existing main owner;
- a reopened main-owner Pack wins over fixture.camera;
- SetActiveExample leaves both World3d preview owner Packs unchanged.

The current node_graph_viewport_sets_camera and every_command_round_trips_through_text_and_binary filters are useful transport/codec checks but do not establish exact-window isolation, render ownership, or reopen. They must be retained only after their expectations no longer name removed app-config mutations.

## Space follow-up

Space remains outside the Generation implementation slice, but it remains inside this goal. Its static workflow camera map key is not an exact WindowConfig owner. The current source says same-kind split panes do not exist, so no two-window incident is claimed. That limitation does not make a workflow window concern app-owned: Space still needs the future kind-checked exact Workflow WindowConfig migration and a reopen law before same-kind Workflow instances are introduced.
