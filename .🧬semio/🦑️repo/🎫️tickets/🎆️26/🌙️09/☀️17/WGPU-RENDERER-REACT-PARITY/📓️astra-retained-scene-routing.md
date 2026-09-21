# Retained Scene Routing

The retained hit registry wraps World3d, NodeGraph, TiledMap, and Board2d. The temporary World3d-only exception restored the first kind, but the three other bespoke dispatchers still receive Chrome ownership before their pointer and wheel handlers run. The independent source audit is `📓️terra-retained-surface-routing-owner-audit.md`.

The existing neutral engine-surface-retention fixture now drives a native law through actual live surface admission, hit publication, pointer ownership, and wheel reachability. It additionally requires a retained hit with a forged unrelated window owner to remain Chrome. Filter: `retained_engine_hit_provenance_reaches_each_dedicated_pointer_and_wheel_route`.

Its third-party browser counterpart mounts canvases for the final retained surfaces in the same fixture, sends trusted Chromium pointer/wheel events, and requires the concrete surface and window IDs in both events. Nx-scoped Bun test passed 1/1, 8 assertions, 9 unrelated tests outside the filter, 1.62 seconds. Receipt: `🗑️generated/astra-runtime/retained-scene-routing-chromium-1.log`.

Native67 stopped at compile failure in the in-flight settings service exports; no tests ran. Native68 will run this intended failing law and the four new pane-owner laws after those exports are complete. Production must carry explicit scene provenance from the canonical retained registration through staging/publication; suffix guessing alone would accept colliding ordinary document controls. Modal and panel precedence must remain intact. No production repair for Graph/Map/Board ownership has been made yet.
# Native68 Failure and Source Repair

The new actual shell provenance law failed in Native68: a live NodeGraph retained hit resolved to Chrome. The Chromium oracle passed one selected test with eight assertions.

The canonical retained registration now carries an optional scene identity, created only from World3d, NodeGraph, Board2d, or TiledMap component records. The interpreter retains it alongside the clipped pointer rectangle; the shell publishes it atomically with hit ownership. Admission checks the exact rectangle, origin owner, live scene kind and bounds, and the state owner where present. Ordinary retained controls retain chrome ownership. Generic retained scrolling yields to a valid scene hit so the dedicated wheel route can run. Existing UI fixture replay additionally verifies provenance against the actual mounted node.

These edits are source-coherent and Rustfmt-parsed. Native69 is running; fresh runtime/browser acceptance is still required.
# Dedicated Dispatch Still Requires Exact Scene Capture

Terra's follow-up found that the renderer still scans all overlapping live scene bounds after the corrected Shell ownership gate. It also clears the Interpreter scene pointer owner before release dispatch. Therefore the provenance gate alone does not establish complete gesture parity.

A new schema-first overlap fixture and native behavioral law now drive actual AppInteractionState pointer-down and pointer-up. Two overlapping World surfaces are published and painted through the real retained document path. The law measures admitted World interaction generations: only the topmost surface should receive the two pointer edges, including release beyond both surfaces. This is intentionally registered before the dedicated dispatcher repair. The browser oracle will verify the same fixture against actual DOM pointer capture. Native69 may include this new RED law if its renderer crate had not begun compiling before registration.
# Exact Dispatcher Implementation Plan

The remaining dispatcher repair will carry a validated scene target beyond the two-way Chrome/Surface classification. Press selects the current canonical retained target, captures its concrete window, scene kind, surface, and originating retained identity. Move and release use that capture even outside its rectangle; release consumes it exactly once. Hover without capture selects one current target and only sends leave to previously hovered peers. Wheel resolves one target per notch and stores that target in its retained frame cursor. A removed owner or stale source identity refuses dispatch; an unmapped `.pane` or `.map` string is not scene authority.

The canonical document projection already states the identity contract explicitly: `surface_scene_node` in UI WGPU reconcile derives `surface_id` from the owning document surface and preserves the authored leaf key separately in `pane_id` (`surfaceHostIdentityV1`). World state currently records the actual host window only under `tool_run_trace_window_id`, written by `render_world3d_surface_step`. That field must be promoted to an explicit origin contract or origin admission must be validated against the canonical projection; a trace-oriented field name alone is insufficient proof. No new arbitrary capacity is needed.

Native70's capture fixture failed before input because `paint_tree_pointer_document` dropped the newly painted World's retained dynamic owner. The helper now temporarily borrows the Shell's World map and returns it after paint, keeping the exact created owner. The law uses that real painted state rather than creating another state. This fixture repair is source-only until the next renderer run.

# Native71 Actual RED and Exact Target Repair

Native71 Nextest aa0cc1da-31cb-4d8f-bcd9-4a110cd921b1 ran five focused laws: three passed and two failed in 247 ms, with 1256 outside the filter. The real World overlap law reached pointer dispatch and failed because the lower surface admitted one press; only the upper surface should receive either edge. The Chromium pointer-capture oracle passed one selected law with two assertions (10 unrelated tests outside filter), 1.151 s.

The source repair now carries a ScenePointerTarget from canonical registration through Shell publication. The target retains window, surface, kind, node, key, and document generation. Press claims one exact target; release consumes that capture after resolving it; move uses the capture outside bounds; wheel stores one target in its retained cursor and executes one scene branch. Replaced node keys, kinds, and document generations cannot inherit that target. World cancel now follows the captured scene rather than cancelling every World relocate drag. Bare .pane/.map fallback admission has been removed. Existing tests that blessed unregistered bounds or suffixes now explicitly require refusal; actual published-scene and real pointer tests remain separate.

Rustfmt parsing and scoped diff checks passed. Native green and fresh browser acceptance remain outstanding. Map cancel still requires a dedicated non-successful cancellation API audit, and multiple-pointer interactions/hover and lifecycle replacements need the follow-up packet.

The actual World fixture now also drives motion+wheel, chrome, modal, secondary press/menu, and cancelled capture sequences. Three old source-string wiring tests were replaced with these real AppInteractionState probes. Expanded Chromium capture oracle passed six fixture scenarios, 13 assertions, one selected test/10 outside,994ms. Captured liveness now additionally requires that the exact scene target remain in Shell's published registry, so a closed owner cannot inherit a late edge. Rust green remains pending.
# Capture Lifetime and Native Lock Receipt

The independent Chromium oracle now passes eleven tests/60 assertions, including eight authored scene sequences and sixteen simultaneous CDP touch captures followed by old-node removal and fresh-node capture (2.02 s). The native fixture uses the same fixed capacity and actual retained document replacement.

Native78 exposed a root integration deadlock: clipped registration already held UI_ENGINE, then called the public target factory that acquired the native mutex again. Two native stack samples agree. Target construction now accepts the already-borrowed Ui in that path; public callers retain the one-lock factory.

Native79 executed the new replacement capacity law and failed at its intended assertion: all sixteen old keys become non-live but retain their slots, rejecting the successor. Only after that actual failure, root changed admission to prune same-pointer and non-live targets before checking the unchanged sixteen-slot capacity. No gesture is sent to a successor while pruning. A fresh native run remains required.
