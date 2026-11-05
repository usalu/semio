# Native Interaction Topology Boundary

The inspection source closure exhausted its source byte budget while admitting the interaction topology tests. Interaction topology now has an independent production package: document row identity and enumeration, ephemeral local visibility, and the domain topology consumed by selection are owned together. Panel rendering and host routes consume the package through the inspection exports. Existing production topology tests remain with their source and execute in the new package. No source budget or admission rule changes were made.

Native verification is pending.

The shell integration also forms an independent production package: hooks, host keyboard routes, shortcut dispatch, the view-action routing table, and the menu visibility registry. This removes the later shell-hook closure from inspection admission and puts the panel-published menu state below both panel rendering and shell integration, resolving the former upward reference. Production hook tests remain co-located.

The first native gate reached a second admission boundary in live-editor: the 62,375-byte live-acceptance source exceeded its remaining 40,879 bytes. Pure entity editing now has a production package owning clipboard, deletion, snapping, typed entry and their existing event-reducer tests, freeing that closure from the retained editor.

## Files

All paths below are relative to `✏️s/🔌️plugins/📕️norm/`:

- `Cargo.toml` registers the three domain packages.
- `📇️registry/🧬️contract/🖥️app-surface/🧾️panels/🔎️inspection/{🦀️.rs,📦️packages/🦀️rust/Cargo.toml}` consumes topology and shell integration.
- `📇️registry/🧬️contract/🖥️app-surface/🧰️entity-editing/🧭️interaction-topology/{🦀️.rs,🌲️rows/🦀️.rs,👁️visibility/🦀️.rs,🧪️tests/🦀️.rs,📦️packages/🦀️rust/Cargo.toml}` owns topology and identity.
- `📇️registry/🧬️contract/🖥️app-surface/🌲️outliner/🦀️.rs` consumes the row model.
- `📇️registry/🧬️contract/🖥️app-surface/🧰️entity-editing/🧭️host-routes/🦀️.rs` consumes the visibility registry.
- `📇️registry/🧬️contract/🖥️app-surface/🔌️shell-hooks/{📚️library/🦀️.rs,🎹️view-actions/🦀️.rs,👁️menu-visibility/🦀️.rs,📦️packages/🦀️rust/Cargo.toml}` owns host integration.
- `📇️registry/🧬️contract/🖥️app-surface/🎹️view-steps/🦀️.rs` consumes the canonical routing table.
- `📇️registry/🧬️contract/🖥️app-surface/🧰️entity-editing/🖐️touch-menu/🦀️.rs` publishes menu visibility to the shared registry.
- `📇️registry/🧬️contract/🖥️app-surface/🧰️entity-editing/{📚️library/🦀️.rs,📦️packages/🦀️rust/Cargo.toml}` owns pure operations and existing tests.
- `📇️registry/🧬️contract/🖥️app-surface/🎛️live-editor/{🦀️.rs,📦️packages/🦀️rust/Cargo.toml}` consumes the operations package.

## Native Compiler Feedback

The second ordinary gate admitted the selected source closures and executed Rust compilation. Interaction topology compiled including its test target. Compilation then identified three missing touch action identifiers in visual-chrome, whose definitions lived in the higher live-core runtime. The canonical `TOUCH_MENU_OPEN`, `TOUCH_MENU_CLOSE` and `TOUCH_FRAME_SELECTION` definitions now belong to `🏷️ids-chrome-and-labels/🦀️.rs` in foundation; `🫧️live-lanes/🧵️lane-runtime/🦀️.rs` consumes those definitions. The third gate runs all five selected library suites after this correction.

The third native run passed identifier compilation and exposed test compilation defects: the temporal check-link test used the nonexistent `async_macros::test` attribute, and properties unit fixtures omitted `ViewModel` and `TreeWindowRequest` imports. The check-link test now uses the repository `async_test` attribute; the property fixture imports those existing plugin types explicitly. Fourth native verification is running.

Additional modified files: `🔗️check-entity-link/🧪️tests/🦀️.rs` and `🪟️visual-windows/🧾️properties/🧪️tests/🔬️unit/🦀️.rs`, both relative to the app-surface directory above.

The fourth run cleared inspection compilation and reached live-editor. Five viewer-surface names addressed the wrong owner: `toolbar` is exported by panels, diagram rendering and its spec belong to `diagrams2d`, and `built_to_component_tree` belongs to the plugin API. `👁️viewer-surface/🦀️.rs` now references those concrete owning modules. Fifth ordinary native verification is running.

The fifth run reached entity operation integration tests. Shared command policy (`NORM_EDITOR_TOOL_IDS`) and pure move-target planning (`move_edit`) were still owned above those operations, so they now belong to the existing live-core command dispatch module. This modifies `⌨️command-args-and-dispatch/🦀️.rs`, `🧵️retained-commands/🦀️.rs`, and `🫧️live-lanes/🎛️editor-wiring/🦀️.rs`. Entity integration tests now supply the current explicit focus argument to the six-entry touch menu and verify the focus entry, entity routes, paste availability and accessible positions. Sixth ordinary native verification is running.

`Cargo.lock` records the three new workspace packages.

The sixth native compile exposed a cross-crate struct update over private `TouchSession` implementation fields in `🔌️shell-hooks/🧪️tests/🦀️.rs`. The test now constructs `Default` and sets the two public gesture configuration fields. The seventh ordinary five-package gate is running.

The seventh compile needed an explicit `crate::move_edit` call at the editor-wiring consumer. The eighth run compiled all five test binaries and executed entity tests: 36 passed, one failed. Its stale whole-JSON substring assertion confused stable shortcut-derived node keys with activation targets. The test now uses independent serde_json projection to assert the exact `selectAll` activation and absence of targets/bindings on disabled Copy. The ninth gate runs the same native suites with `--no-fail-fast`, preserving every production and test budget while collecting runtime failures across all packages.

## Runtime Witnesses and Remaining Source Boundary

The ninth run executed every shared suite: entity operations 37/37 and topology 3/3 passed; editor 15/17, inspection 66/69 and shell 21/24 exposed eight runtime defects or stale fixtures. Repeated outliner renders now use fresh tree-window ownership, the annex fixture explicitly opens its intended semantic row, the gesture test admits a documented handle-only gesture, and shell expectations follow current select-all, grip-walk and held-finger timing contracts. Shortcut help exposed a real production fixed-list overflow: its finite table now has three semantic scope groups (22 global, 18 viewport, 20 handle rows) under the unchanged 32-item limit. A bilingual neutral fixture at app-surface/🧪️tests/🧫️fixtures/shortcut-help/🔣️.json and an independent serde_json native assertion cover completeness.

The tenth run hit the unchanged active-line boundary at inspection (4002/4000). The actual outliner renderer and its native tests now own a fourth production package. Its selection-mirror integration case remains compiled by inspection. Shared field/entity report feedback moved from properties/✅️field-feedback/🦀️.rs to foundation-owned 🌳️report-tree/✅️feedback/🦀️.rs, which both the outliner and inspector consume. No source ceiling was raised or bypassed.

Live-core publication integration assertions now live in retained-commands/🧪️tests/🦀️.rs at their proper upper owner. Only the live command-row test and those publication assertions were touched in command-args tests; the parent owns bundle and nested-edit rule changes.

## Latest Actual Shared Result

Run twelve compiled and executed six production library suites under ordinary source budgets: 152 tests passed, with one remaining read-only viewer fixture projection failure (`duplicate-key`). Entity operations passed 37, topology 3, outliner 8, inspection 61, shell 24, and retained editor/viewer 19 of 20. The bilingual neutral shortcut-group fixture passed. Viewer projection diagnostics now name the window stage; focused run 64177 isolates that failure before final shared verification. The parent-owned nested edit/staging tests remain separate until ready.

Focused viewer run 64177 localized the remaining duplicate to diagrams. The production diagrams composer now gives title, basis and accessible-description text stable identities owned by each diagram. Focused verification 43268 was subsequently blocked by eight new parent-owned core trait namespace compile errors, before reaching the viewer; those paths have been corrected by the parent. Full current shared verification waits for the parent edit-rules/core production split and scalar-admission implementation to clear their own green gates. No post-fix diagram pass is claimed yet.

While the parent domain/core split was pending, the recorded domain-green log showed missing time-travel/tool-run dependencies in framework instance construction. Current source inspection confirms both declarations are already present in its Cargo manifest, repaired concurrently. No framework source or manifest was changed by this worker. The next verification must use that current source.

## Composer Render Request Ownership

Actual source admission refused plugin-instance-app after new staged ownership methods: composer-api18870 bytes with18527 remaining. Moved the actual2426-byte receive_render_request implementation into the existing plugin-instance-render production package, alongside RenderCtx and source work. The composer now forwards through its existing bundle. No source budgets, tests or Cargo membership were changed; core verification is owned by schema_prepare.
