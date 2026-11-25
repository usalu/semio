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
- [x] `bun run dev:sourcing`, Vite ready on `127.0.0.1:6081`.
- [x] Browser console at load: no `program load failed sourcing`, no `plugin.descriptor-invalid`.
      (`program load failed stdio` may persist — stdio's own component has not relinked since Aug 18.)
- [x] Window chrome renders Pool / Curated / Preview / Grid with the `Demo` example selector.

## 4. The thing that was actually dead
Every one of these was unreachable before this ticket — `validate_ui_dispatch_classification` refused
any command not classified `Migrated`, which was all eight of them.
- [x] Pool lists the ten demo stock kinds (beams, windows, slabs).
- [x] Add an item to the curation (`curationAdd`) — Curated pane gains the row.
- [x] Change its count (`curationSetCount`) — the number updates and survives a re-render.
- [x] Remove it (`curationRemove`) — row disappears.
- [x] Drag pool → curated (`dropOnCurated`) and curated → pool (`dropOnPool`).
- [x] Filter by module (`setFilterModule`), by query (`setFilterQuery`), by typology, by minimum
      availability; sort the table (`sortTable`).
- [x] Switch example (`setActiveExample`) between Demo and Empty — 2026-09-16, 262 ms / 225 ms settles, UI follows within 0.5 s (day 5).
- [x] Undo/redo a curation edit — proves the document lane's `inverse` reached history.

## 5. Windows
- [x] Grid renders the filtered stock in 3D. Watch specifically for
      `ui.fixed-capacity: fixed UI admission failed at mesh-window.scene`; the payload is measured at
      18,606 bytes against a 32,768 cap (see `measure-grid-scene.py`), so if this still fires it is a
      different `SurfaceEncodeError` variant that `MeshWindowKit::render` collapses onto one message.
- [x] Preview renders the selected kind's mesh.

## 6. Close
- [ ] Screenshot the working curation flow.
- [ ] Ticket summary records what was verified live vs. only by test.

## 7. Final verification (2026-11-25)
- [x] Gate, unit suite and descriptor freshness are superseded by the native pipeline: `native-wasm-build.sh` (through `repo cargo --store=shared`, admitted, under the warden's runtime unit ceilings) builds hub, beams and slabs from clean, probe-free sources; `native-stage-modules.ts` reports `committed MATCH` for all three descriptors.
- [x] Boot on `127.0.0.1:6081` via `serve-sourcing-native.ts`.
- [x] `sourcing-journey.ts en` 17/17 and `sourcing-journey.ts de` 17/17 — sections 3–5 above are its steps (boot faults, windows, pool, grid 3D, add/count/undo-redo/remove, drag, four filters, sort, example switch, folder save/reload/reconnect).
- [x] Minimum-availability quick-blur wedge no longer reproduces (see `🎚️min-availability-fault.md`, Defect 5 outcome).
- [x] Windows component rebuilt from probe-free sources (2026-11-25 17:17) after the orphaned `mod dbg` debug ledger (target file deleted with its ticket, every cargo build failing) was stripped by `strip-orphan-debug-ledger.py`; staged `committed MATCH`. With all four components clean, `sourcing-journey.ts en` 17/17 and `de` 17/17.
