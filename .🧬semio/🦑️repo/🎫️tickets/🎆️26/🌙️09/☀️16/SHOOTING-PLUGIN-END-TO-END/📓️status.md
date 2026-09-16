# 📸 Shooting plugin end to end — status

## 2026-09-16

- Ticket opened (manual on disk; repo MCP down).
- Native `cargo check -p semio-s-plugin-shooting --lib --tests`: passes clean (1m32s, `🗑️generated/check-shooting-native.txt`).
- Fault 1 (same as forms #1): playground block had no `app` → `VITE_SEMIO_APP_ID=""` → boot refusal `does not declare pinned app ""`. Added `app = "s.shooting.shooting@1/*#editor"` to the shooting `Cargo.toml` playground block.
- Fault 2 (same as forms #3): `ShootingPlayApp` never overrode `ArtifactEditor::command_from_action`, so every shell action (`addShot`, measures, `setCamera`, gumball, tree rows) was refused as "not framework-reserved". Added `args_bridge::command_from_action` in the editor root: snake-cases host keys, restores integral floats, aliases the host control contracts (`value`→`shot_id`/`asset_id`/`json`/`payload`/`id`, `pressed`→`value` for toggles, gumball `ids`→`asset_ids` with zero/one deltas, viewport `{windowId, camera}` nested or flat pose), stringifies the inspector `value` for `patchShots`/`patchAssets`, seeds `addShot`/`addAsset` defaults, maps `exportActiveShot`/`exportAllShots` → `ExportShots{all}`. Two unit tests (`command_from_action_round_trips_every_command_id`, `…_bridges_host_control_contracts`).
- Scripts: `📜️activate-shooting-react.sh` (`activate-shooting-react-dev`, port 6019), `📜️serve-shooting-react.sh` (`?plugin=shooting`), probes `🐍️shooting-console-dump-probe.mjs` (boot) + `🐍️shooting-interact-probe.mjs` (catalogue addShot → undo → shadow toggle → tree select). Launch entry `shooting-react-attach` added.

### Fault 3 (the real killer): every verb unclassified → plugin could not even be materialized
The first restage died in the plugin descriptor probe: `app-definition.interactive-job-classification: unclassified interactive command 's.shooting.shooting@1/*#editor:addAsset'; …` (35 verbs). Only `loadRequest`/`importAssetRequest` were bounded tools; the framework refuses UI dispatch of any non-`Migrated` verb and the descriptor probe aborts the guest outright — the staged shooting wasm in `🧑‍💻dev/🔌️plugin-modules` dated from Aug 18 for exactly this reason. Migration (all in `✏️editor/🦀️.rs`):
- `SHOOTING_BOUNDED_TOOL_IDS` = every command id (37); `bounded_first_step_tool_proofs!` rows for all; `.action_interactive_job(id, Migrated)` for all; `PUBLICATION_CONTRACTS` with the exact lanes each handler emits (`HostOnly` for effect-only verbs incl. the three `LoadDocument` ones, `Artifact`, `Config`, `Artifact+Config` for addShot/addAsset/importAsset/saveCamera/setActiveAsset).
- Store publication authority the bounded lanes need: `build_artifact_store_one_item_preparation_factory` + config twin via the framework's generic `bounded_config_store_one_item_preparation_factory` (dag/trinity precedent — no copied preparation factory), bounded document/config/draft owners + disposers, presence disposer/retirement (real `ShootingPresence`), transient no-ops.
- `exportActiveShot`/`exportAllShots` were ONE row (`ExportShots{all}`) with a payload-dependent `command_id` override; the proof join only admits generated row ids, so `exportAllShots` could never be authoritative. Split into two payload rows (`export-active-shot`/`export-all-shots`, shared `export(all, …)` body) and deleted the override.
- `setCamera`/`setCameraDraftLabel` now `Emit::amend_config` under one coalesce key per gesture family (64-edit config ledger; per-tick/keystroke edits would kill the session).
- `reset_document_effect` minted a `create_document_envelope` just to print its spr → guest trap on Drop; now `store::empty_document_spr` (fem2d shape).

### Fault 4: icon window "Unexpected token '<' … is not valid JSON"
`IconRenderHost` fed the guest's public mesh id (`/mesh/🧊️base.glb`) straight to `GLTFLoader`; the dev server only serves the catalog transport path, so vite's SPA fallback returned HTML. Now resolved through `meshAssetTransportUrl` exactly like `World3dHost` (`🖼️IconRenderHost/🟦️.tsx`).

### Icon framing
The icon rendered the asset as a dot: the scene auto-fits client-side (never published back as `setCamera` by design), the icon used the raw config camera (`[420,-420,320]`, ~10× too far for the demo base mesh). Added `IconRenderRequest.fit` (`🌓️theme/🟦️.ts`, `iconRenderCameraPose` in `⚛️react/🟦️.tsx`: keeps direction+zoom, re-targets the bounding-sphere centre, backs off with padding) and the shooting request sends `fit: {enabled: cfg.center_model, padding: 1.25}` — the icon twin of the scene's centre-model lane. `ui-react` typecheck has 661 pre-existing peer errors (ControlIcon/UiLabel churn), none in these lines.

### Tests
`cargo test -p semio-s-artifact-shooting-shooting --lib`: **344 pass / 2 fail** (was 244/102).
- Harness rewritten to the fem2d shape: registry-backed, `bind_instance_id`, self-closing `ShootingApp` newtype, `dispatch` settles the retained publication and applies `LoadDocument`, returns `Dispatched{result, lanes}` (`edited_document()`), `history_verb` for undo, `view(locale)` with the two window instances (chrome maps are keyed per instance).
- Fixture debt fixed in one pass (`🐍️canonicalize-shooting-fixture-floats.py`, 87 files): the JSON oracles compare `serde_json::Value`s, which distinguish `5` from the codec's canonical `5.0` for every f64 field; the config vectors also carried the retired `activeUtilityId` field/case; the panel fixture uses `en`/`de` while the test matched `en-US`/`de-DE`; the wasm/binary store tests never closed their stores.
- Remaining 2 failures are framework gaps (documented, not plugin faults): `ingest_operations_is_idempotent_for_shooting` — `assert_registered_ingest_idempotent` never binds the sender instance (`interactive-job.live-instance`); `two_instances_converge_disjoint_edits_via_backbone` — registry-less `paired_apps` cannot admit tool proofs and `paired_registered_apps` refuses `attach_backbone` (memory `project-store-drop-witness-and-mounted-test-harness`).
- `cargo test -p semio-s-plugin-shooting --lib`: 3/3.

### End-to-end proof (react dev serve 6019, restage 21:29Z; `🗑️generated/shooting-interact-5/`, `shooting-boot-3/`, `shooting-viewer/`)
`🐍️shooting-interact-probe.mjs`, six steps, zero fault lines:
1. boot: `data-semio-os-ready=shooting`, scene host (1 canvas, 2 meshes), icon host renders the SVG shot framed on the asset.
2. panels: Artifact/Catalogue/Inspection open.
3. Catalogue "PNG Rectangle" → `history patch applied … createShot "Shot 3"` + config `set-shot-selection`; Artifact tree shows 3 shots.
4. `mod+z` (shell also journals its own panel-tab switches, so two presses) → "Undo", tree back to 2 shots.
5. Window Options → Shadow checkbox → `changeSceneShadowEnabled new_enabled=false`, the measure re-publishes `false`.
6. Artifact tree "Overview Png" → `setShotSelection` (config lane), Inspection panel follows (Label/Format/Shape/Width/Height).
`🐍️shooting-viewer-probe.mjs`: navbar Viewer role → `window:shooting-view-scene` with 2 meshes, zero faults.

### Left as is
- The single boot 404 is the dev serve's `/🧩️extension-modules/watch` endpoint (not shooting).
- Serve left running on 6019 (`screen shooting-serve`); launch entry `shooting-react-attach`.
- `🔣️.json` plugin descriptor at the plugin root is regenerated by the registry generator on activate; `descriptor_is_fresh` passes.
