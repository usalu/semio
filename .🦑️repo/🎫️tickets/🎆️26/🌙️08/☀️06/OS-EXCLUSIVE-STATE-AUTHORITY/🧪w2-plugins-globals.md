# W2 Plugins — Globals / Mutex Session Purge

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`  
**Date:** 2026-08-06  
**Scope:** `✏️s/🔌️plugins/**` only (Wave 2 plugins)

## Fixed (this pass)

| Site | Change |
|------|--------|
| `🏭️process/…/process3d/⚙️engine` — `PROCESS_BREP_HOST` `OnceLock<Mutex<ProcessKernelSession>>` | Removed process singleton; each public entry (`processed_mesh`, `processed_volume`, `export_process3d_model`, `import_process3d_model`) builds a fresh `ProcessKernelSession` (memo is per-call; kernel handles are replay-derived from document, not cross-call). |
| `🖍️draw/…/draw/⚙️engine` — `DRAW_ID_COUNTER` `AtomicU64` | `create_draw_id(prefix, material)` uses `DefaultHasher` over caller-supplied bytes (content-addressed ids). |
| `🧩️puzzle/…/3d/⚙️engine/🖌️brush` — `PUZZLE3D_UUID_COUNTER` | `brush_object_id(fixture, payload)` hashes fixture length + placement payload (test `successive_brush_placements_never_collide_on_object_id` retained). |
| `🧩️puzzle/…/◻2d/…/edit` — `PUZZLE2D_FIXTURE_JSON_CACHE` `LazyLock<Mutex<…>>` | Removed; `cached_fixture_json` serializes per call (pure, no cross-tick memo). |

## Deferred (reason)

| Site | Reason |
|------|--------|
| `🪐️space/🏠️home` — `shared_studio_ports` `OnceLock<Arc<Mutex<…>>>` + `HomeApp::studio_ports` | Needs `SHomeDocument` / `HomeConfig` binding ops (`space_id → folder path`) and `resolve_studio_document` callers (incl. `apps::space`) passed host-owned home projection — not only plugin-local. |
| `📐️cad` — `CAD_BREP_HOST` `OnceLock<BrepEngineHost>` | Kernel handles must survive across calls until `DocumentSession.engines` / host `engine-derive` injection (Wave 1b). |
| `🖍️draw/🌊️flow/🌀️procedural` play apps — `Mutex<DrawSession>` / `Mutex<FlowEvalSession>` | `FlowEvalSession` is explicitly non-serializable and ties to OS `FLOW_SESSION_GEOMETRY` mutex in framework flow core; move to host `DocumentSession` + config snapshot lane (integration). `DrawSession` needs `gesture_session_json` on `DrawConfig` + fsm snapshot codec. |
| `🪐️space` — `SpaceApp::presence_peers` `Mutex` | Move to `SpaceConfig` JSON registry + `SpaceConfigOperation` on heartbeat (collab hub bridge pending). |
| `🪐️space` — `thread_local! STUDIO_TEST_APP` | Test-only; replace with explicit `SpaceApp` fixture in testkit when trait allows. |
| `🏭️process/…/workpiece` — `PROCESS3D_PREVIEW_CACHE` | Move to `Process3dConfig` memo field or document-derived cache key. |
| `📏️layout` — `ENGINE` `OnceLock<Mutex<LayoutEngine>>` | Per-document `EngineCache` or stack-local engine per derive. |
| `🧩️puzzle/🧊️3d` play — `PUZZLE3D_MESH_REGISTRY`, app `Mutex` caches | EngineCache / config memo ops. |
| `🌀️procedural3d/⚙️engine` — `static LAST: Mutex<String>` (non-test) | Remove debug global or route through document op. |
| `🏛️architect` — `ID_COUNTER` | Content-addressed ids like draw/puzzle. |
| `🎞️animate` — `SOBJECT_ID` / `UPDATER_ID` | Document/config ops. |
| Readonly catalogs (`CAD` interaction `OnceLock`, `TRINITY_JACK_MANIFEST`, Typst fonts in animate, example `LazyLock<String>` JSON) | OK as immutable lazy **if** no mutable session inside; prefer `const` / `include_str!` parse per call where cheap — track separately from authority breaches. |

## `rg` snapshot (authoritative globals still under `✏️s/🔌️plugins/**`)

After this pass, `LazyLock|OnceLock|static.*Mutex|static.*AtomicU64|thread_local!` still hit (non-exhaustive): home catalog ports + studio registry, cad brep host, flow/draw/procedural play mutex sessions, space presence + test thread_local, layout engine singleton, process3d preview cache, puzzle3d mesh registry + app mutex caches, procedural LAST mutex, animate counters/Typst OnceLocks, architect ID counter, remodel/cad readonly OnceLocks, puzzle example JSON LazyLocks.

## Verification

| Package | `cargo check -p … --lib` | Notes |
|---------|--------------------------|--------|
| `semio-s-plugin-process` | **blocked** | Fails in `semio-s-3d` / brepkit kernel compile (parallel M1 brep work — `BrepkitKernel.registry`, `GeometryHandle::content_addressed`, etc.) — not introduced by this diff. Log: `🧪w2-plugins-cargo-process.err` |
| `semio-s-plugin-draw` | **blocked** | Same `semio-s-3d` chain via `semio-s-2d`. Log: `🧪w2-plugins-cargo-draw.err` |
| `semio-s-plugin-puzzle` | **blocked** | Same. Log: `🧪w2-plugins-cargo-puzzle.err` |

`DEVELOPER_DIR=/Library/Developer/CommandLineTools` used for `cc`.
