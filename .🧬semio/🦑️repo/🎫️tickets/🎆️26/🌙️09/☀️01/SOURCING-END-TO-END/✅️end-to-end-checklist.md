# ✅️ End-to-end verification checklist

Run in order. Each step names the exact evidence that closes it — "it looked fine" does not.

## 0. Gate
- [ ] `cargo check -p semio-s-plugin-sourcing --target wasm32-wasip2` exits 0.
      Blocked all of 2026-09-01/02 by peers' `ToValue`/`FromValue` migration, not by this ticket.

## 1. Unit + fixture suite
- [ ] `cargo test -p semio-s-plugin-sourcing` — record `test result:` verbatim.
- [ ] No `panic in a destructor during cleanup` anywhere in the output. That abort masks every test
      after it, so a clean-looking tail is not evidence on its own.
- [ ] `descriptor_is_fresh` passes (it needs step 2's rebuild first — expect it red until then).

## 2. Rebuild the served artifacts
- [ ] `bun nx run @semio-tech/framework-os-dev:plugin -- sourcing`
- [ ] `🔌️plugin-modules/sourcing/semio_s_plugin_sourcing_component.core.wasm` mtime is NEW.
      It was pinned at Sep 1 12:30 while the crate would not compile — a stale wasm is exactly how
      this ticket's original symptom hid for five days.
- [ ] `🔌️plugin-modules/sourcing/🔣️.json` regenerated, and its 14 UI commands ALL read
      `interactiveJob: "migrated"` with no `batchOnlyPendingRewrite` left. The copy served on
      2026-09-02 still showed 6 migrated / 8 batch-only — that is the pre-migration build.

## 3. Boot
- [ ] `bun run dev:sourcing`, Vite ready on `127.0.0.1:6081`.
- [ ] Browser console at load: no `program load failed sourcing`, no `plugin.descriptor-invalid`.
      (`program load failed stdio` may persist — stdio's own component has not relinked since Aug 18.)
- [ ] Window chrome renders Pool / Curated / Preview / Grid with the `Demo` example selector.

## 4. The thing that was actually dead
Every one of these was unreachable before this ticket — `validate_ui_dispatch_classification` refused
any command not classified `Migrated`, which was all eight of them.
- [ ] Pool lists the ten demo stock kinds (beams, windows, slabs).
- [ ] Add an item to the curation (`curationAdd`) — Curated pane gains the row.
- [ ] Change its count (`curationSetCount`) — the number updates and survives a re-render.
- [ ] Remove it (`curationRemove`) — row disappears.
- [ ] Drag pool → curated (`dropOnCurated`) and curated → pool (`dropOnPool`).
- [ ] Filter by module (`setFilterModule`), by query (`setFilterQuery`), by typology, by minimum
      availability; sort the table (`sortTable`).
- [ ] Switch example (`setActiveExample`) between Demo and Empty.
- [ ] Undo/redo a curation edit — proves the document lane's `inverse` reached history.

## 5. Windows
- [ ] Grid renders the filtered stock in 3D. Watch specifically for
      `ui.fixed-capacity: fixed UI admission failed at mesh-window.scene`; the payload is measured at
      18,606 bytes against a 32,768 cap (see `measure-grid-scene.py`), so if this still fires it is a
      different `SurfaceEncodeError` variant that `MeshWindowKit::render` collapses onto one message.
- [ ] Preview renders the selected kind's mesh.

## 6. Close
- [ ] Screenshot the working curation flow.
- [ ] Ticket summary records what was verified live vs. only by test.
