# Scout: client shells (lane 0-S2)

> ⚠️ Written by the coordinator from the 0-S2 agent's findings (the agent reported but did not write
> the file). Treat every line/number as a **pointer, not gospel**: re-read the region before you edit
> it. Where the scout said "not found", verify yourself before concluding the thing does not exist.

## 1. React shell — `💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx` (~5795 lines)

Regions: `🧲️Header` 1–7 · `🔌️Adapters` 9–443 · `FrameworkOsShell` 445→EOF, with nested
`🎥️TutorialOverlayHosts` 509–551, `🎥️TutorialRecorder` 553–664, `🐚️ShellMount` 666–747,
`🔌️PluginRuntime` 966–1471, `🎥️TutorialOrchestration` 2952–3283, `💾️uiPrefs` 3351–3397,
`🔖️SurfaceRoles` 3531–3693, `🔖️ThemeMutators` 3695–3844, `🚗️DriverMutators` 3846–3896,
`🧰️FooterUtilityLeaves` 4260–4303, `🔄️SyncLeaf` 4305+.

- **Per-tab actor** — line 866: ``const shellActorIdRef = useRef<string>(`client-${Math.random().toString(36).slice(2)}`)``.
  This is what must become `user:{userId}#{sessionId}`.
- **`openDocument`** — 2531–2558. Builds the `BackboneWorkerRequest`:
  `{ kind: "open", documentId, schema, bindings, watchExternal: true, actor: shellActorIdRef.current }`.
  Bindings today come only from the caller.
- **`attachSyncBackbone`** — 2573–2594 (deprecated sync-card adapter): `remote://host:port/space` →
  `[{ kind: "hub", baseUrl: "http://host:port", spaceId }]`, `folder://p` → `[{kind:"folder",path:p}]`,
  `file://p/x.json` → folder of its parent. Keep it as a manual override.
- **`HostEffect` switch** — 2173–2425. Handled kinds: `requestSync`, `setPanel`, `setActiveUtility`,
  `setActiveTool`, `patchWorld3dChrome`, `openDialog`, `navigate`, `loadDocument`, `openExternalUrl`,
  `downloadMediaExport`, `iconRenderExport`, `requestFileOpen`, `dispatchAction`, `requestMediaFrames`,
  `invokeExtension`, `spawnPluginInstance`, `openPluginInstance`. **Check whether `ReplayShellCommand`
  is handled here or in the action funnel** — the opening commands relay through it
  (`🔌️plugin/🦀️component.rs` `🔖️OpeningCommandRelay`, ~14438–14499, `relay_opening_command("os.open-artifact", …)`).
- **Action funnel** — `onAction` 2623–2944; shell-intercepted ids before the plugin sees them
  (recovery 2625, tutorials 2649/2657/2662, world nav 2689, utility 2709, tool 2742,
  `FRAMEWORK_SYNC_CONTROLLER_ID` 2768, space import 2810, app spawn 2815, panel tabs 2823), plugin
  dispatch 2843–2889. This is where `os.directory.*` must be intercepted.
- **`openArtifactWithAppRef`** — 3613–3693: opens an app instance but **not a document** (the known gap
  the `documentId`/`spaceId` relay args close).
- **Presence heartbeat** — 3292–3322: dispatches `{ controllerId: studioSessionControllerId,
  action: "presenceHeartbeat", args: identity }`; `connectedAtMs` ref at 867, cursor ref updated at 942.
- **Routing** — `applyShellUri` 2451–2504; parses `/spaces/{spaceId}` (2480) and
  `/spaces/{spaceId}/instances/{instanceId}` (2413) into `openSpaceIdRef` / `openInstanceIdRef`.
- **Shell i18n** — `🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` ~3328–3346
  (`createShellI18nInstance`: `fallbackLng: "en"`, `supportedLngs: ["en","de"]`, per-shell bundles
  registered into an initially empty `resources`). Example keys: `framework.history.undo|redo|checkpoint`.

## 2. `💻️os/🟦️backbone-worker.ts` (~943 lines)

- `BackboneWorkerRequest = ({kind:"open"} & ArtifactActorConfig) | {kind:"close",documentId} | {kind:"send",documentId,message}`;
  `BackboneWorkerResponse = {kind:"event",documentId,event} | {kind:"ready"}` (~1169–1172 per scout;
  the file is 943 lines, so treat the numbers as approximate and grep the type names).
- `connectHub` 339–381: URL `${wsBase}/spaces/{space}/documents/{doc}/ws` (343) — **this is where
  `?surface=` is appended**. Hello (350–362): `wire_version`, `protocol_version`, `schema`,
  `pack_schema_hash`, `actor`, `token`, `resume_token`, `frontier`. Backoff constants 76–77:
  `HUB_RECONNECT_MIN_MS = 500`, `HUB_RECONNECT_MAX_MS = 30_000`, doubling at 378.
- `relayMutationsToHub` 389–396 (batches + `pendingBatches` by `batchId`); `handleAck` 401–428.
- `handleHubFrame` 430–487: `Welcome` 432 · `SnapshotChunk`/`Done` 443–445 (ignored) · `Commands`
  447–453 (filters own actor, emits `remoteMutations`) · `Ack` 455–458 · `Preview` 460–462 ·
  `Presence` 464–477 (decodes peer blobs) · `CreditGrant` 479–482 (ignored) · `Error` 484–486.
- Folder lane: `folderBinding` 122–125, poll interval 1500 ms, PUT/GET through `BACKBONE_ENDPOINT_PATH`.
- `ArtifactSyncStatus = { persisted: boolean; pendingMutations: number; remote: "detached"|"connecting"|"live"|"backoff" }`
  (~113–120) — the source for the status pill.

## 3. `💻️os/🟦️component.ts`

- `PersistenceBinding` at **1061**: `{kind:"folder",path} | {kind:"hub",baseUrl,spaceId,token?}` — gains
  an optional `surface`.
- `ArtifactActorConfig` 1064–1075 (`documentId`, `schema`, `bindings`, `watchExternal?`, `actor`,
  `packSchemaHash?`).
- `parseRemoteBackboneUri` 42–50 / `buildRemoteBackboneUri` 52–54; `BACKBONE_ENDPOINT_PATH` 22
  (`/semio-backbone`), `BLOB_ENDPOINT_PATH` 276 (`/semio-blob`).
- `AppChannelCodec` / `AppChannelClient` regions are **peer-leased** — grep their region markers and
  stay outside them. Lane 0-A's new `🔖️Directory` region is at the end of the file.

## 4. wgpu shell — `🧱️elements/Shell/🧊️component.rs` (~11360 lines)

- `parse_persistence_binding` ~1587–1610 (`remote://host:port[/space]`, `folder://`, `file://`).
- `attach_sync_backbone` ~1757–1774: builds `ArtifactActorConfig { document_id, schema, bindings,
  watch_external: true, actor: format!("wgpu-{}", session.instance_id) }` (actor also at 1738), calls
  `document_host.open(...)`, `runtime.register_host_backbone("actor://{doc}")`,
  `plugin.attach_backbone(...)`.
- **`ArtifactEvent::Presence` is dropped at ~1677** with a comment that the ViewModel carries no
  presence yet — lane 2-D fills this in with a shell-local `presence_peers` field (do **not** widen the
  shared kernel `ViewModel`).
- Env reads: `read_preference_path` ~9784, prefs dir ~9688–9691 (`SEMIO_PREFS_DIR`, `XDG_CONFIG_HOME`,
  `APPDATA`, `HOME`). Native entry `🎯️targets/🧊️wgpu/📦️bin.rs` (e.g. `SEMIO_PLUGIN_MODULES` at 11) and
  browser entry `🎯️targets/🧊️wgpu/🟦️typescript/🟦️boot.ts` (~169–175).
- Action/command dispatch ~1932–1989 / 2043: `program.handle_action(...)` / `program.handle_command(...)`,
  then host effects applied.

## 5. Plugin session actor

- React: `🧱️elements/PluginRuntime/🟦️component.tsx` **334** —
  `channels.set(instanceId, new AppChannelClient(handle, instanceId, appId))`; the 4th `actor` argument
  is omitted today and defaults to `"local"`.
- wgpu: `🧱️elements/ProgramBridge/🧊️component.rs` (~498) — the `WasmPluginRuntime::exchange` path;
  guest-side fallback `"local"` lives in `🔌️plugin/🦀️component.rs` ~13632–13642 / 14581.

## 6. Table primitive for plugin surfaces

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs`:
`build_table_scene(surface_id, controller_id, TableScene) -> UiNode` at **4232**; `TableScene`
(3415–3426): `columns_json`, `rows_json`, `selection_json?`, `row_drag_mime?`, `drop_action?`,
`sort_json?`; `TableCell` (3439–3444): `Text | Number | Stepper | Buttons`. A working plugin call site:
the imperative play editor (`build_table_scene(SURFACE, APP_ID, TableScene::base(columns_json, rows_json))`).

React `📊️Table` (`🖱️ui/🧱️elements/📊️Table/🟦️component.tsx`) already emits **`data-row-id`** (~201, ~262).
The scout could **not** find `data-ui-path` on table rows — assume it is missing and add it where the
wgpu↔React parity join needs it (verify first).

## 7. Config facet template — `💻️os/🎚️config/`

```
🎚️config/🧬️schema/{🔣️component.json,🛰️component.proto,🔗️component.graphql,🟦️component.ts,🦀️component.rs}
🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️component.rs
🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs
🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/…
```
Wiring in `💻️os/🟦️backbone-worker.ts`: `OPENING_PREFERENCES_SCHEMA = "os.config.opening"` (~137), the
document id equals the schema id (singleton), `openingPreferencesActorConfig(actor)` returns
`{ documentId, schema, bindings: [], actor }` (~145–147 — **empty bindings = persisted local-only**),
and `foldOpeningPreferencesEvent(base, event, decodePayload)` (~157–165) folds `remoteMutations`.
The new `🪪️identity` facet mirrors this exactly, except its bindings point at the folder lane under
`S_DATA_DIR/os` so the session token survives a reload.
