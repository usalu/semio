# Draw Interaction Observable and Missing-Surface Specimens

Read-only source audit on 2026-09-20. No build, browser activation, or test was run.

## Draw: physical selection consequence

**Confidence: high.** The Draw canvas can produce framework-owned selection, but it cannot render that selection or report its count today.

The app starts with `selectDirect`; its retained canvas path turns a point query into an `interactionSelect` effect ([Draw editor/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs#L545)). The app copies the framework `strokes` selection into a command session before it dispatches a command ([Draw editor/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs#L1078)).

The smallest physical acceptance sequence is therefore:

1. Create or use one visible drawing layer and click it in the canvas with the default direct-select utility.
2. Focus the canvas window engagement field with id `drawing-canvas-engagement`, type a unique name, and submit Enter. This is a normal physical text action; the React adapter routes that input's `onSubmit` through `engagementSubmit` ([ShellHelpers/🟦️.tsx](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx#L1952)).
3. Assert the renamed visible layer-tree text under the stable family `drawing-play-layers.shape.<layer-id>`. The tree builds that row from `DrawingLayerBase.name` ([layers panel/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️layers/🦀️.rs#L80)).

`engagementSubmit` refuses unless exactly one framework-selected id exists, then emits `rename_layer` ([engagement-submit/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️engagement-submit/🦀️.rs#L16)). A changed tree label is consequently a concrete document-level proof of physical selection propagation, not a direct action injection.

Do not use the engagement status suffix as that proof. The status deliberately formats the actual top-level layer count but hard-codes `0 selected`, and `window_engagements_with_request_context` ignores `InteractionView` ([Draw editor/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs#L1662)). The live layer count remains a valid separate acceptance signal.

There is no properties-pane opacity slider. The Properties panel renders only the schema/utility/layer-count text ([properties panel/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs#L1)). `setSelectedOpacity` does apply to every session-selected id ([set-selected-opacity/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌫️set-selected-opacity/🦀️.rs#L17)), but it is exposed as a catalogue action, not as that pane's slider. It may be a later physical command-palette multi-select test; it is not the minimal current Draw adapter route.

The missing selection highlight is a shared baseline gap rather than a WGPU-only regression: `Canvas2dScene` transports only `layers_json`, and both hosts inspect per-layer `selected` in that JSON. React explicitly says its canvas has no layer pick/selection state ([Canvas2dHost/🟦️.tsx](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx#L923)); Draw now emits no selected field because its render function does not receive `InteractionView` ([canvas window/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs#L83)). WGPU consumes the same scene field. This should stay separate from the renderer parity packet.

## Smallest future product specimens

No plugin editor currently declares `SurfaceKind::DiffView` or `SurfaceKind::EventFeed`; the existing coverage is framework-host fixture coverage rather than an activateable product route.

### DiffView: VCS editor

The VCS plugin is the correct owner, not a generic renderer demo. It owns the `s.vcs.vcs@1/*#editor` app and an actual `demo` example ([VCS descriptor](../../../../../../✏️s/🔌️plugins/🌿️vcs/🔣️.json)). It already renders checkpoint history as `GraphTimeline` ([history window/🦀️.rs](../../../../../../✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📜️history/🦀️.rs#L1)), and its edit work already produces a before/after projection diff ([VCS editor/🦀️.rs](../../../../../../✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs#L390)).

The smallest specimen is a VCS-owned review/diff window over the existing demo's selected checkpoint versus current projection, encoded as `DiffViewScene { before, after, mode: Some("unified"), language: Some("semio") }`. Those are exactly the surface's owned fields ([scene contract](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs#L2585)). A real route can use the existing boot axes `plugin=vcs&app=s.vcs.vcs@1/*#editor&role=editor&mode=edit&example=demo`; this audit did not activate it.

### EventFeed: Architect editor

The Architect plugin is the smallest domain owner for a feed. It owns `s.architect.program@1/*#editor` and a `demo` example ([Architect descriptor](../../../../../../✏️s/🔌️plugins/🏛️architect/🔣️.json)). Its document already has typed `AuditEvent` records, and `audit_trail(program, None)` returns them newest first ([audit inference/🦀️.rs](../../../../../../✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs#L2145)).

The smallest specimen is to make the existing Architect Trace window emit an `EventFeedScene` from that already document-owned trail, preserving each audit event id and timestamp, choosing a stable icon/tone from its action, and leaving `activate_action: None` until Architect owns a meaningful open-event action. The trace window currently renders the same trail as a virtualized TextEditor/tree ([trace window/🦀️.rs](../../../../../../✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️trace/🦀️.rs#L1)). `EventFeedScene` explicitly supports a chronological `{id,timestampMs,iconId,title,detail?,tone?}[]` payload and optional activation ([scene contract](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs#L2631)). That preserves product ownership and avoids inventing renderer-only feed entries.

