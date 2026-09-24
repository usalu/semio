# N2 — wasm32 browser wgpu shell: lazy-install and open a foreign-kind artifact

Slice N2 (G10 §C slice N2 / O1-4). Predecessors: `📓️o2-activation-follow-ups.md` §0.5/§5.3,
`📓️o3-wasm32-wgpu-artifact-open-relay.md` (landed the app half), `📓️wgr-wgpu-hub-and-open-relay.md` §2/§5.
Private target dir `.tmp-ticket-0918/wp-n2/target`, captures `wp-n2/generated/*.txt`.

## 0. Inherited state (measured)

- First run cut at ~12:50 by the account session limit; resumed 16:29. Nothing had been edited or started before the
  cut (report skeleton only). No servers, no cargo of this slice alive.
- **Freeze (coordinator, binding):** no edits to the stdio/gis crates, the os-kernel (pack/store) crate, or the
  framework plugin crate. Seams that need a frozen crate are designed renderer-side; the frozen hunk is recorded
  here as *blocked-by-freeze*.
- O3 (`📓️o3-…md`) already un-gated the **app half** of the relay: `OpenArtifactRelayTarget`,
  `open_artifact_relay_target`, `find_dialect_app`, the `os.open-artifact{,-with}` arm, `handle_open_artifact_relay`,
  `resolve_activation_owner_app`, `switch_to_app` compile on wasm32; install goes through O2's
  `install_plugin` → `acquire_plugin_program` (wasm32: `crate::program_bridge::install_js_plugin` → frame-Worker
  door `semioWgpuInstallPlugin`). WGr measured the wasm32 check green (09-20 07:43) but never ran it in a browser.
- Still native-only (measured by grep on `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, 29 857 lines): the **document half**
  — `open_document`, `default_bindings_for_current_session`, `document_host: ArtifactHost`, `sync_channel`,
  `pump_sync_events`, `detach_sync_backbone_internal`, `refresh_history_snapshot`, `current_shell_actor`,
  `mint_sync_binding_generation`; on wasm32 the relay raises `open-artifact.browser-document` instead.
- Root causes of that gate:
  1. the renderer crate enables the kernel's `sync` feature only for non-wasm32, so `store_sync::sync`
     (`ArtifactHost`, `ArtifactEvent`, `PersistenceBinding`, `backbone_worker_wire`) does not exist on wasm32;
  2. the kernel's `sync` feature drags `dep:ureq` from its unconditional `[dependencies]` table (O3 §0) —
     moving it is a kernel-crate edit → **blocked-by-freeze**;
  3. the kernel's browser `wasm_actor::connect` is an empty body (`🏪️store/🔄️sync/🦀️.rs` `WasmActor::connect`),
     i.e. the Rust in-process actor has no browser hub transport; the browser's real hub document authority is the
     TS backbone worker `🏪️store/👷️worker/🟦️.ts` (React's `ensureBackboneWorker`) — kernel crate → frozen;
  4. `ProgramBridgeEntry`'s JS backend has no `bind/retire/receive_document_backbone`, `apply_mutations`,
     `load_app_document_archive` (native-only in `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`).
- Item 3's `"manual"` call site: `🐚️plugin-bridge/🟦️.ts` `createApp` already used `activationReasonForAppId(appId)`
  (O1/O2); the remaining `"manual"` was the request actor (`ensureRequestActor`, then line 1538).
- The wgpu browser dist (`🎯️targets/🧊️wgpu/📦️packages/🦀️rust/dist/wasm-dev`) and every `dist/runtime/wgpu/dev/*`
  receipt are **absent** (swept since WGr's 09-20 run); the shared `dist/dev/🔌️plugin-modules` root holds 63 staged
  plugin/extension module dirs, which a wgpu serve mounts as its module root.

## 1. Design

1. **One relay body, the platform split at the document host's transports.** The browser build links the SAME
   kernel `ArtifactHost` the native build does (its `wasm_actor` exists for exactly this: "One `ArtifactHost` per
   host process (wgpu native, tests, or the browser wgpu build)"). `open_document`, `detach_sync_backbone_internal`,
   `pump_sync_events`, `ShellSyncChannel`, the presence heartbeat step and the sync pill become target-neutral; the
   only platform seam is a declared capability, `SHELL_DOCUMENT_TRANSPORTS { folder, hub }`
   (native `{true,true}`, browser `{false,false}`), checked by `document_bindings_admitted` BEFORE any actor exists.
   The browser actor has no filesystem and a hub `connect` with an empty body, so a folder or hub binding there is
   refused by name (`document-binding.folder-unavailable` / `document-binding.hub-transport-unavailable`) instead of
   silently persisting nowhere; the empty binding set (the declared `EphemeralLocalOnly` class — what a single-plugin
   playground with no identity/space/data dir computes) is served identically by both hosts.
2. **The sync card's attach is the same body.** `attach_sync_backbone` duplicated `open_document` line for line
   (native) and faked success with a `[DEBUG]` log on wasm32 ("relayed via host-shim"); it now parses the card's uri
   and calls `open_document(…, backbone_uri: Some(uri))`.
3. **The JS program bridge gets the document door the native kernel client already has.** `bind/retire/
   receive_document_backbone`, `apply_mutations`, `load_app_document_archive` gain a `ProgramBridgeBackend::Js` arm
   calling new `WgpuJsBridge` members (`documentBackbone`, `receiveDocumentBackbone`, `applyMutations`,
   `loadAppDocumentArchive`, `loadAppArtifactPack`). The TS half drives the guest with the SAME
   `DocumentBackboneBindingCommandV1` control React's `bindDocumentPort` sends (`encodeDocumentBackboneControlV1` +
   `requireDocumentBackboneReceiptV1`), inside one serialized actor call (typed-operation settle, UI patch intake,
   spawned-job drain). `u64` binding generations cross as decimal strings.
4. **Guest backbone messages reach the shell instead of being dropped.** `wireEffectToFriendly` returns `null` for a
   `send-message` to `Backbone{uri}` because React's document port consumed it earlier; the wgpu bridge had no port,
   so every guest document message died there. `wgpuBackboneMessageEffect` projects it onto the host
   `SendMessage { target: Backbone { uri }, payload }` byte for byte; all six effect projections of the bridge go
   through `wgpuHostEffect`, so the shell's existing `route_document_backbone_effects` receives it on both targets.
5. **History seed stays where its door is.** `refresh_history_snapshot` resets the projection on both targets; the
   `ReadHistory`/`ReadConflicts` seed (`seed_history_snapshot`) is native-only because the JS bridge has no
   `exchange` door — the browser projection folds every later `history_patch` exactly as native.
6. **Progress + cancel** are O2's `ShellPluginInstall` phases painted by `ShellChromeFramePhase::PluginInstall`
   (target-neutral since O2); the relay's install runs through `install_plugin` → `acquire_plugin_program` →
   frame-Worker door, so no second progress notion was added. A refused document attach paints the localized
   `open-artifact.document-failed` notice on both targets (it was a native debug log only).
7. **Item 3.** The request actor activates `on-extension-request:<capability>` (the WIT payload is the extension
   point that pulled the actor up) via `extensionRequestActivationReason`; `createApp` keeps
   `activationReasonForAppId(appId)`.

## 2. Changes

### 2.0 Landing discipline (freeze + mutex)

The Rust half lives in one anchored, reversible script, `🐍️n2-relay-edits.py` (`apply|revert|status [bridge|relay]`,
67 exact hunks, refuses on a missing/ambiguous anchor). Group `bridge` = the JS program-bridge document door (compiles
on wasm32 without the kernel's `sync`); group `relay` = Cargo + shell + renderer pump + the O3 law it supersedes. The
wasm32 compile can only be proven inside a fleet-mutex hold, so the Rust edits are kept OUT of the shared tree between
holds (applied → native check → native tests → reverted, 16:54–17:03) and re-applied by `📜️n2-wasm32-gate.sh` inside
one hold (bridge: keep on green; relay: evidence check, always reverted while the kernel hunk is frozen).

### 2.1 Item 3 — real activation reason for the request actor (landed, tested)

- `🐚️plugin-bridge/🟦️.ts` — new exported `extensionRequestActivationReason(capability)` →
  `on-extension-request:<capability>` (the WIT `activation-event::on-extension-request(string)` payload is the
  extension point; the capability is the point an inbound `invoke` pulled the actor up for).
  `ensureRequestActor(capability)` activates with it instead of `"manual"`; `invoke` passes its `capability`.
  `createApp` keeps `activationReasonForAppId(appId)`. No `"manual"` activation remains in the wgpu bridge.
- Test: `🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts` — new law "activates a request actor on-extension-request …":
  exact reason string + the exact WIT `activate` envelope `activationEventEnvelope` derives from it.

## 3. Checks and tests

```
$ SEMIO_TEST_LEVEL=long npx vitest run --root <⚛️react>/📦️packages/🟦️typescript --config <abs>/⚛️react/🧪️tests/🎚️config/🟦️.ts \
    🔬️wgpu-extension-dispatch 🎬️wasm-plugin-install 🎬️activation-owner
 Test Files  3 passed (3)
      Tests  33 passed (33)          # 16:32, capture wp-n2/generated/n2-vitest-activation.txt
```

```
$ … vitest … 🧩️package-integration 🔬️wgpu-extension-dispatch 🎬️wasm-plugin-install 🎬️activation-owner
 Test Files  4 passed (4)
      Tests  60 passed (60)          # 16:47, after the bridge door + projection; n2-vitest-bridge.txt
$ bun ./📜️script.ts check-frame-worker      # 🎞️frame-worker.js is fresh (regenerated 16:46)
$ npx tsc --noEmit -p <⚛️react>/📦️packages/🟦️typescript/tsconfig.json   # 0 "error TS" over the whole program (17:22);
                                                                      # plugin-bridge + both touched suites + 🔗️binding are in it

# Rust (all hunks applied), native, private target dir — 16:54–16:59, re-run 18:05 with 69 hunks
$ cargo check -p semio-framework-os-renderer-wgpu --lib            # EXIT=0, 127 warnings (all pre-existing peers'), n2-native-check.txt
$ cargo test  -p semio-framework-os-renderer-wgpu --lib -- document_relay_tests document_backbone_effect_tests plugin_install_tests
test result: ok. 17 passed; 0 failed; 1362 filtered out        # n2-native-tests.txt (391 warnings ⇒ the crate really compiled)
```

Laws (all target-neutral source; only the native run is measured):
`📂️wgpu-document-relay` — a host serving both transports admits every binding kind; a host without transports
serves the empty set and refuses hub/folder by code (and hub before folder in a mixed set); a playground's default
bindings are empty → `EphemeralLocalOnly` → admitted on this build; this build declares its own transports
(native `{true,true}`, wasm32 `{false,false}`); the refusal notice is localized EN/DE.
`📡️wgpu-document-backbone-effect` — the shared fixture's projected host effect parses into
`Effect::SendMessage { Backbone { uri }, payload }` and the full bridge answer (`requestedEffects`) parses through the
exact `from_json_str::<InvocationResult>` call the wasm32 `document_backbone_effects` makes — the Rust twin of the TS
projection law over the SAME `🧫️fixtures/📡️wgpu-document-backbone/🔣️.json`.
`🎬️wgpu-plugin-install` — O3's 11 laws unchanged except `the_browser_document_refusal_is_localized`, superseded by
the target-neutral `a_refused_document_attach_is_localized`.

Nx generator law (cache-contracts: every taxonomy `inputPatterns` path is an input of the contract's generate and
check targets): `bun nx show project @semio-tech/framework-os --json` → `generate-wgpu` and `check-wgpu` both carry
`🔌️plugin/📡️backbone/🔗️binding/🟦️.ts` (inferred from the taxonomy row added for the bridge's new import).

Kernel `sync` on wasm32 (16:50, through the mutex, private target dir), `n2-kernel-sync-wasm32-check.txt`:

```
$ cargo check -p semio-framework-os-kernel --lib --features sync --target wasm32-unknown-unknown
getrandom-0.2.17/src/lib.rs:346:9: error: the wasm*-unknown-unknown targets are not supported by default,
  you may need to enable the "js" feature.                        EXIT=101
$ cargo tree -p semio-framework-os-kernel --features sync --target wasm32-unknown-unknown -i getrandom@0.2.17
getrandom v0.2.17 └── ring v0.17.14 ├── rustls v0.23.41 │ └── ureq v2.12.1 │ └── semio-framework-os-kernel
```

Gate attempt 1 (17:40, mutex hold ≈ 15 s, `n2-gate-attempt1.txt` / `n2-wasm32-bridge-check-attempt1.txt`): the
bridge group failed `cargo check --target wasm32-unknown-unknown` with 2× `E0433 cannot find … protocol` — the
renderer root aliases the kernel as `protocol`/`store_sync` only for non-wasm32 (`🧊️renderer/🦀️.rs:26-31`). Reverted
automatically; both aliases are now un-gated by the script (`protocol` in the bridge group, `store_sync` in the relay
group) and the gate was re-queued (17:41).

The ONLY path pulling it is `ureq` (native HTTP + rustls/ring), which the kernel's `sync` feature enables from its
unconditional `[dependencies]` table (O3 §0 predicted this). That is the frozen kernel-manifest hunk — §6.

## 4. Runtime evidence

**Not run — waits on the coordinator's wasm go.** At 17:51 the coordinator froze the wasm lock for this slice until
W1's catalog A is published; my re-queued gate (pid 21417, queued 17:41, never acquired) was withdrawn by killing my
own wrapper (its trap removed the ticket; it then looped on the vanished ticket and was killed by pid).

Ready to run, in order, once the go arrives:

1. `zsh .tmp-ticket/📜️fleet-mutex.sh wasm n2 -- zsh .tmp-ticket-0918/📜️n2-wasm32-gate.sh` — one hold: bridge group
   wasm32 check (keep on green) → relay-group evidence crate (`wp-n2/relay-wasm32-probe`, always reverted) → renderer
   wasm (trunk) + browser boot + frame worker + `prepare`/`activate forms wgpu dev` (NO `materialize`, so the shared
   plugin-modules root and play's lane receipts are untouched; forms and note are staged with core wasm).
2. Serve detached outside the mutex on 6391: `cd 🧑‍💻dev/📦️packages/🟦️typescript && nohup bun
   ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts serve forms dev --port 6391 >
   wp-n2/generated/n2-serve.txt 2>&1 & disown`.
3. `SEMIO_PROBE_URL=http://127.0.0.1:6391/?plugin=forms bun .tmp-ticket-0918/🐍️n2-wgpu-foreign-open-probe.mjs` —
   boots forms, stands in for the space index's `openArtifact` guest with ONE `ReplayShellCommand{os.open-artifact
   {artifactRef:"s.note.note@1/*", documentId, schema}}` appended to the resident bridge's next answer (hook injected
   by rewriting one line of the served frame-worker bundle in flight; no repo file carries it), then records: note
   module fetched only after the trigger, the install band phases, the session switch in `dumpStructure` / the DOM a11y
   mirror, same `performance.timeOrigin` (no reload), the document notice, and a second foreign open (dag) cancelled
   from the band's `shell.plugin-install.cancel` control.

With the relay group blocked (§6.1), the browser build under test carries the landed state: app half (O3) + install
door/band (O2) + bridge door; the document half answers `open-artifact.browser-document` (O3's refusal).

## 5. Measured vs tested vs unverified

| claim | status |
|---|---|
| request actor activates `on-extension-request:<capability>`; no `"manual"` left in the wgpu bridge | **tested** (vitest law, exact WIT envelope) |
| guest `send-message → Backbone{uri}` reaches the shell as `SendMessage{Backbone}` byte for byte | **tested** both halves over one fixture (vitest + Rust native) |
| JS bridge document door: exact `u64` generation, operation refusal, JSON answer shape | **tested** (vitest, `🧩️package-integration`) |
| bridge TS typechecks | **measured** (`tsc` 0 errors over the react package program) |
| frame-worker bundle regenerated and fresh; generator input law holds | **measured** |
| relay un-gating + seam compiles natively; 17 relay/install/effect laws pass | **measured** (native check + test, private target dir) |
| bridge group compiles for `wasm32-unknown-unknown` | **unverified** — attempt 1 failed on the `protocol` alias (fixed in the script); re-check waits on the go |
| relay group compiles for `wasm32-unknown-unknown` given the kernel hunk | **unverified** — evidence crate ready, waits on the go |
| kernel `sync` does not compile for wasm32 today, only because of `ureq` | **measured** (check EXIT=101 + `cargo tree -i`) |
| browser: lazy install + switch to a foreign kind in the same session, band + cancel | **unverified at runtime** — probe ready, waits on the go |
| browser: document half binds an ephemeral local document | **blocked-by-freeze** (§6.1) — not landable, not observable |

## 6. Honest gaps

### 6.1 BLOCKED-BY-FREEZE — kernel manifest hunk (`semio-framework-os-kernel`, frozen crate)

`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`: move
`ureq = { version = "2", optional = true }` from `[dependencies]` into
`[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`. Nothing else changes: `sync`/`ureq` keep naming
`dep:ureq` (a target-gated optional dep is a no-op for wasm32), and every `ureq` use site is already
`cfg(all(feature = "ureq", feature = "sync", not(target_arch = "wasm32")))` (`📇️directory/🔌️client/🦀️.rs`). Once it
lands, `python3 🐍️n2-relay-edits.py apply relay` lands the shell half unchanged (its Cargo hunk adds
`semio-framework-os-kernel { features = ["sync"] }` + `tokio { features = ["sync"] }` to the renderer's
`cfg(all(target_arch = "wasm32", target_os = "unknown"))` table). Enabling `sync` from the renderer today would drag
`ureq` + rustls + ring into the browser bundle and needs a `getrandom 0.2 "js"` unification to compile at all — a
compatibility shim, not taken.

### 6.2 BLOCKED-BY-FREEZE — browser hub transport in the kernel's `wasm_actor`

`🏪️store/🔄️sync/🦀️.rs` `WasmActor::connect` has an empty body, so even with 6.1 the browser `ArtifactHost` serves
local-only (`EphemeralLocalOnly`) documents; a hub binding is refused by `document_bindings_admitted` rather than
opened dead. The seam belongs in the kernel (an injected socket transport the actor dials, which the renderer would
implement over its existing page-owned duplex door `🔌️socket-door` — the same door the directory stream and the MCP
bridge use). Kernel crate → frozen.

### 6.3 Proofs waiting on the coordinator's wasm go (exact list)

1. `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` with the bridge group
   applied (lands the JS-backend document door; until then it lives only in `🐍️n2-relay-edits.py`).
2. The relay-group evidence check through `wp-n2/relay-wasm32-probe` (proves the one relay body compiles for wasm32 once
   §6.1 lands).
3. Renderer wasm build + `prepare`/`activate forms wgpu dev`, then the browser probe (§4).
4. The target-neutral laws compile for wasm32 (`--tests`) — they are cfg-free, but no wasm32 test build ran.

### 6.4 Other gaps

- The TS side is LANDED while its Rust caller is not: `documentBackbone`/`receiveDocumentBackbone`/`applyMutations`/
  `loadAppDocumentArchive`/`loadAppArtifactPack` exist on every wgpu JS bridge handle but nothing calls them until the
  bridge group lands; `wgpuBackboneMessageEffect` is live now (a backbone message the shell does not own yet is logged
  and dropped by the shell's effect funnel exactly as before, instead of in the bridge).
- `loadAppArtifactPack` was a pre-existing hole: `ProgramBridgeEntry::load_app_document_pack`'s wasm32 arm looked up a
  bridge function that did not exist; the bridge now provides it (callable once the build carries it).
- The browser history projection is not seeded from `ReadHistory`/`ReadConflicts` (no JS `exchange` door); it folds
  later `history_patch`es only.
- No user-reachable UI in a hub-less single-plugin playground emits `os.open-artifact` (the space index needs hub
  directory events), which is why the runtime probe injects the guest's effect.
- The React DOM host's `createApp` activation reason (`🔌️PluginRuntime/🟦️.tsx`, O2 §5 gap 1) is N1's, untouched.
- Two new law files exist in the tree but are mounted only by the script (`📂️wgpu-document-relay`,
  `📡️wgpu-document-backbone-effect`); until the groups land they compile nowhere.

## 7. Files changed

Landed (in the tree now):

```
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts        (activation reason, backbone effect projection, document door)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js (regenerated)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json (frameWorkerSources input)
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json                                            (browserProfile + inputPatterns row)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/📡️wgpu-document-backbone/🔣️.json      (new, shared TS/Rust fixture)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts         (+2 describe blocks)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts             (fake handle + door law)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📂️wgpu-document-relay/🦀️.rs (new, mounted by the relay group)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🧪️tests/📡️wgpu-document-backbone-effect/🦀️.rs (new, mounted by the bridge group)
```

Prepared, applied only inside checks (`🐍️n2-relay-edits.py`, 69 hunks): `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`,
`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`, `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`, `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml`,
`🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs`.

Ticket folder: `🐍️n2-relay-edits.py`, `📜️n2-wasm32-gate.sh`, `🐍️n2-wgpu-foreign-open-probe.mjs`,
`wp-n2/relay-wasm32-probe/` (evidence crate), `wp-n2/generated/n2-*.txt`. No git-modifying command; no worktree; no
kernel/plugin/stdio/gis crate edited; no pid killed except my own mutex wrapper (21417).
