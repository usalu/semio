# 🗄️ W5a — browser preferences and persistence through the page realm

Packet W5a of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. It closes `📓️w4a-boot-appearance-and-tour.md`
hand-off **1** ("P0 — the browser wgpu shell has NO persistent preference storage at all") and §5 item 3
("on the browser build the write will not persist").

Every gate under **VERIFY** was RUN, in the foreground, and its log is under `🗑️generated/w5a-*.txt`.

---

## 1 — The defect

`WebLocalStorage::new()` resolved `localStorage` through `web_sys::window()`. The wgpu shell runs inside
the frame Worker that owns the `OffscreenCanvas`, and **a Worker realm owns no `window`** — so the shim
resolved to `None` on every browser boot and:

* every `prefs_get` answered **nothing** — appearance, locale, terminology, theme id, custom themes,
  custom drivers, keybinding overrides, the compute worker count, the dock skeleton, `dockUi` and
  `ui.introduction.seen.<appId>` all read as empty;
* every `prefs_set` wrote to **nothing** — including the tour's Skip/Done answer, which is why a
  dismissed tour came back on every reload.

Both failures were silent: the shim's `call()` returned `Option` and every miss was discarded.

A **second, independent** defect was found while fixing it, and is fixed here too: the two keys React
keeps as FLAT `localStorage` entries — `ui.compute.workerCount` and `ui.introduction.seen.<appId>` —
were being written *inside* `semio.os.config`'s `preferences` map by this renderer. Even with working
storage, React's own `readStoredComputeWorkerCount` / `readStoredIntroductionSeen` could never have
found them, and the reverse was equally true: a tour dismissed in React was invisible here.

---

## 2 — Key table (key → React shape → wgpu reader/writer)

Every durable key React's shell stack declares, classified in ONE place —
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s `HOST_STORAGE_KEY_CENSUS` — and pinned against React's own constants
by `the_key_census_covers_reacts_own_key_list`.

### Carried (boot snapshot + door)

| key | React shape / writer | wgpu reader / writer |
| --- | --- | --- |
| `semio.os.config` | the ONE `OsShellConfigSnapshot` document (`🖥️platform/🟦️.ts`): `{version:1, preferences, namedLayouts, dockLayouts:{os?,apps}, dockUi:{os?,apps}, windowPanes:{os?,apps}}`, ≤64 KiB | `raw_prefs_get`/`raw_prefs_set` (whole doc), `os_shell_config_document`/`write_os_shell_config_document` |
| ↳ `preferences["os.config.ui-preferences"]` | a STRING holding `{"version":1,"events":[…]}` — `OsShellConfig.setPreference` + `commitUiPreferencesConfigMutation` (`🎚️UiPreferences/🟦️.ts`). Carries `setAppearance`/`setLocale`/`setTerminology`/`setTheme`/`setCustomTheme`/`setDriver`/`setCustomDriver`/`setLayout`/`setKeybindingOverride` | `prefs_get`/`prefs_set` → `read_ui_preferences_event_log` / `persist_ui_preferences` |
| ↳ `dockLayouts.os` \| `dockLayouts.apps[<appId>]` | `DockSkeleton {version:3, anchors}` — `DockLayoutStore` | `load_dock_skeleton_from_store` / `save_dock_skeleton_to_store` (W1h) |
| ↳ `dockUi.os` \| `dockUi.apps[<appId>]` | `DockUiState {version:3, anchors:{<anchor>:{visible?,size?,path?}}}` — `DockUiStateStore` | `load_dock_ui_from_store` / `save_dock_ui_to_store`, `persist_dock_ui_if_changed` (W1h) |
| ↳ `namedLayouts[<appId>]` | `NamedLayout[]` — `NamedLayoutStore` | **no wgpu reader yet** (gap, §7) |
| ↳ `windowPanes.os` \| `windowPanes.apps[<appId>]` | `WindowPaneUiState` — `WindowPaneStateStore` | **no wgpu reader yet** (gap, §7) — the key and the projection are carried, nothing consumes them |
| `ui.compute.workerCount` | FLAT decimal integer ≥1 — `readStoredComputeWorkerCount`/`writeStoredComputeWorkerCount` | `stored_field_get`/`stored_field_set` — **moved out of `preferences` by this packet** |
| `ui.introduction.seen.<appId>` | FLAT, literal `"true"` — `readStoredIntroductionSeen`/`writeStoredIntroductionSeen` | `stored_field_get`/`stored_field_set` in `advance_chrome_maintenance_step` — **moved out of `preferences` by this packet** |
| `SEMIO_RUNTIME_DIAGNOSTICS` | FLAT, truthy `1/true/on/yes` — `RUNTIME_DIAGNOSTICS_KEY` (`🏛️ShellHost/🟦️.tsx:1678`) | read by the PAGE (`🚀️browser-boot/🟦️.ts`'s `armUiTurnDiagnostics`, `stampShardWorkerDiagnostics`) and stamped on the Worker URL, which is what arms `semioWgpuSetRuntimeDiagnostics`. Already page-realm before this packet; now also in the snapshot, so the census is closed |

### Classified but NOT carried

| key | why |
| --- | --- |
| `ui.chrome.appearance`, `ui.chrome.layout`, `ui.chrome.locale`, `ui.chrome.terminology`, `ui.chrome.theme`, `ui.chrome.theme.snapshot`, `ui.themes.custom`, `ui.chrome.driver`, `ui.drivers.custom`, `ui.keybindings.overrides` | the standalone `🖱️ui` surface library's lane (`useUiTerminology`, `UiDriver`, the locale detector). **React's own OS shell reads none of them** — it persists through `os.config.ui-preferences`, and so does this renderer. Carrying them would be dead payload in the frame Worker |
| `semio.presence.client` | `sessionStorage`, and credential-bearing (a presence identity pack, `🛠️ShellHelpers/🟦️.tsx:534`). Off the boot snapshot on purpose; the door speaks `scope: "session"` so the lane can reach it when browser presence exists, which it does not (`📓️w1e` gap 2) |

### 🎓️ The tour key, exactly — for probe seeding

* **wgpu** always writes `ui.introduction.seen.` + `session.app.id`, value `"true"`.
  For the puzzle3d playground that is **`ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor`**.
* **React** writes `ui.introduction.seen.` + (`<brandId>:<appId>` when the playground has a brand, else
  the bare app id) — `🏛️ShellHost/🟦️.tsx:4128`.
  * `puzzle3d` playground (6013/6113) has **no brand**, so both renderers use the identical key above.
  * `aggregator` playground (6023/6123) has brand `entwerfen-mit-bestand-aggregator`, so React writes
    `ui.introduction.seen.entwerfen-mit-bestand-aggregator:s.puzzle.puzzle3d@1/*#editor` while wgpu
    still writes the bare form. **That divergence is W1d's brand gap, not a storage one** — the wgpu
    shell resolves no `ShellBrand` at all (`📓️w4a` hand-off 6) — and it is recorded in §7.
* To force a tour: `localStorage.removeItem("ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor")` (or a
  fresh profile). To suppress it: `localStorage.setItem(<that key>, "true")`. **Absent ⇒ the tour arms**
  (`should_auto_start_introduction`'s `!seen` term), any value other than `"true"` also counts as unseen.

---

## 3 — Door design

One bounded storage door on the EXISTING host-io mailbox, plus the synchronous cache a frame can read.

```
page                     frame Worker                       renderer wasm
🚪️host-io/🟦️.ts   ⇄  🎞️frame-worker/🟦️.ts  ⇄  🐚️Shell/…/🦀️.rs  //#region 🚪️StorageDoor
  storageHop()            semioWgpuHostIo bridge               HOST_STORAGE (thread-local BTreeMap)
  localStorage /          semioWgpuSetHostStorage              raw_prefs_get / raw_prefs_set
  sessionStorage                                               host_io_call → "storage" op
```

**Wire** (new `op` on the existing vocabulary, beside `download-media-export`, `request-file-open`,
`directory-http`):

| direction | JSON |
| --- | --- |
| request | `{"op":"storage","verb":"get"\|"set"\|"remove","scope":"local"\|"session","key":…,"value"?:…}` |
| answer (read / settled write) | `{"value":"<text>"}` or `{"value":null}` |
| answer (refused) | `{"error":"<message>"}` |

A refusal is never flattened into an empty read — "this key holds nothing" and "this call was refused"
are different outcomes, the same distinction the directory door keeps.

**Why a cache AND a door.** `prefs_get` is called from the frame build (the shell resolves appearance,
the tour flag and the dock skeleton while it paints) and the door is a `Promise`. So:

1. **Boot snapshot.** The page reads the whole carried census ONCE
   (`readWgpuHostStorageSnapshot`, `🧭️boot-descriptor/🟦️.ts`) and sends it with the boot message
   (`BrowserFrameWorkerBoot.storage`). The Worker applies it in the `runtime-environment` owned step,
   **before** `semioWgpuWorkerBootstrap` — so the first frame's reads are synchronous and there is no
   default painted and corrected a frame later. The embeddable door (`🎬️renderer-boot/🟦️.ts`) runs on
   the page and seeds the same hook itself.
2. **Write-through.** `raw_prefs_set` → `host_storage_set`: the cache write lands FIRST (so a read
   following a write in the same frame sees it), then one fire-and-forget `spawn_app_task` door hop. A
   refusal is `[DEBUG]`-logged, never swallowed; a preference write must never fault a surface.
3. **Cross-tab.** A `storage` event fires only for OTHER documents on the origin, so it is exactly the
   cross-tab case: the page re-reads the census and posts `host-storage`, which re-seeds the cache. This
   isolate's own writes never arrive that way — they went out through the door.

**Bounds.** Key must be in the census (allowlist, refused on BOTH sides — the door is not a general
`localStorage` proxy); value ≤ `HOST_STORAGE_VALUE_MAX_BYTES` = 64 KiB = `OS_SHELL_CONFIG_MAX_BYTES`;
snapshot ≤ 128 KiB, with oversized/uncarried entries DROPPED rather than failing the whole snapshot.

**Native is untouched**: `raw_prefs_get`/`raw_prefs_set`'s `cfg(not(wasm32))` arm is still the bounded
file page (`native_pref_read_page`/`native_pref_write_page`).

### Sites

| site | change |
| --- | --- |
| `🐚️Shell/…/🦀️.rs` `🗄️PrefsStore` | `WebLocalStorage`, its `thread_local` and the `PrefsStore` wasm impl **deleted**; the trait narrowed from `cfg(any(wasm32, test))` to `cfg(test)` (its only remaining implementor is the test `FilePrefsStore`) — that `wasm32` arm existed only for the shim. `prefs_get_bounded`/`prefs_set_bounded` deleted in favour of `stored_field_get`/`stored_field_set`, which are TOP-LEVEL. `prefs_get`/`prefs_set` keep their one key, `os.config.ui-preferences`, and now say so |
| `🐚️Shell/…/🦀️.rs` new `//#region 🚪️StorageDoor` | `STORAGE_DOOR_OP`, the two byte caps, `HostStorageKeyKind`, `HOST_STORAGE_KEY_CENSUS`, `host_storage_carries_key`, `StorageDoorVerb`, `StorageDoorScope`, `encode_storage_door_request`, `decode_storage_door_answer`, `decode_host_storage_snapshot`, `HOST_STORAGE`, `seed_host_storage`, `host_storage_get`/`_set`/`_remove`, `host_storage_write_through` (wasm32 / no-op), and `#[wasm_bindgen(js_name = semioWgpuSetHostStorage)]` |
| `🐚️Shell/…/🦀️.rs` `advance_chrome_maintenance_step` | the tour read/write use `stored_field_get`/`stored_field_set` (FLAT) |
| `🐚️Shell/…/🦀️.rs` prefs load/persist steps | `ui.compute.workerCount` reads/writes FLAT |
| `🧭️boot-descriptor/🟦️.ts` new `🗄️HostStorage` region | `WGPU_HOST_STORAGE_KEYS`, `WGPU_HOST_STORAGE_KEY_PREFIXES`, the two caps, `WgpuHostStorageScope`, `WgpuHostStorageSnapshot`, `wgpuHostStorageCarriesKey`, `readWgpuHostStorageSnapshot` |
| `🚪️host-io/🟦️.ts` | the `storage` op on `WgpuHostIoRequest`, `WgpuHostStorageAnswer`, `storageHop()` and its dispatch arm |
| `🚚️browser-frame-transport/🟦️.ts` | `storage` on `BrowserFrameWorkerBoot`, the `BrowserFrameHostStorage` message, `transport.setHostStorage()` |
| `🎞️frame-worker/🟦️.ts` | the `semioWgpuSetHostStorage` binding, applied in the `runtime-environment` owned step and again on every `host-storage` message |
| `🚀️browser-boot/🟦️.ts` | `hostStorage()`, into the boot message, plus a second `storage` listener (`republishHostStorage`), torn down on `pagehide` |
| `🎬️renderer-boot/🟦️.ts` | `publishHostStorage()` before the mount + its own `storage` listener |

---

## 4 — Tests

### Rust — new law file `🐚️Shell/🧪️tests/🗄️browser-prefs-persistence/🦀️.rs`, mounted from the Shell's `🗄️BrowserPrefsPersistenceTests` region. **11 tests, all RUN, all green** (`🗑️generated/w5a-tests.txt`).

| law | pins |
| --- | --- |
| `the_key_census_covers_reacts_own_key_list` | **the parity ledger** — re-reads every `const …STORAGE_KEY… = "…"` / `…DIAGNOSTICS_KEY = "…"` out of React's six source files and refuses any key the census does not classify, and any census row React does not declare. A key React adds fails this law instead of quietly going unserved |
| `the_page_reads_the_same_census_this_isolate_serves` | the TS census arrays equal the Rust `Carried`/`CarriedFamily` rows, and both byte caps match |
| `the_door_wire_carries_every_verb_and_refuses_what_the_census_does_not` | get/set/remove, both scopes, the request shape, and refusal at the ENCODER for an uncarried key and an oversized value |
| `an_empty_read_and_a_refusal_are_different_answers` | `{"value":…}` / `{"value":null}` / `{}` / `{"error":…}` / malformed / non-string |
| `a_seeded_snapshot_answers_the_first_frames_reads` | the boot snapshot carries the document, the worker count and the seen flag, drops an uncarried key, and its `os.config.ui-preferences` string replays to React's own `light` |
| `a_later_set_outranks_the_boot_snapshot` | **the precedence law** — snapshot → set → read, then remove, then a cross-tab re-seed; plus store-level refusal of an uncarried key and an oversized value |
| `a_refused_snapshot_entry_never_costs_the_rest` | a non-string entry, non-JSON, a JSON array, an oversized snapshot and an oversized value each cost only what they are |
| `every_carried_key_round_trips_in_reacts_own_encoding` | per-key encodings: `"true"` flat, decimal flat, `{"version":1,"events":[…]}` nested under `preferences` with all four sibling projections preserved, and `prefs_get_from` reading it back |
| `the_tour_answer_is_written_where_reacts_next_boot_reads_it` | **the coordinator's claim** — a fresh store arms the tour, `dismiss_introduction` ends it and queues the one bounded write, and the written value makes the next boot's `seen` true |
| `the_browser_preference_lane_is_the_page_door` | source: no `WebLocalStorage`, no `"localStorage"` literal left in the shell, both `raw_prefs_*` wasm arms call the store, the hook is exported, the write-through is fire-and-forget, the tour key is FLAT, and the nested accessors are gone |
| `every_page_door_hands_the_census_across` | source: the page services the op and refuses outside the census, both stores are addressed by scope, the Worker applies the snapshot inside the boot step (before the plugin graph) and on `host-storage`, and both page doors read and re-read the census |

Three laws read SOURCE, for the reason `🧭️boot-axis-parity/🦀️.rs` does: they assert that a
`cfg(target_arch = "wasm32")` path exists and is wired to three TypeScript artifacts, and that code
never compiles into a native test binary.

### TypeScript — new suite `🧪️tests/🗄️wgpu-host-storage-door/🟦️.ts` (8 tests, registered in `🧪️tests/🎚️config/🟦️.ts`'s `include` **and** the `test-browser-worker` script). **All 8 green.**

Reads/writes/removes against a real `Storage` implementation; sessionStorage addressed only on request;
uncarried key and oversized value refused loudly with the store left untouched; a realm with no store
answers a refusal, not an empty read; the snapshot carries every census member (including a scanned
`ui.introduction.seen.*` family member) and nothing else; oversized values dropped and the 128 KiB
ceiling honoured; a sandboxed `localStorage` getter that throws reads as `{}`; the census predicate
itself, including that the bare family prefix is not a key.

---

## VERIFY (all foreground, all RUN)

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4 --keep-going` | **0 errors**, 61 crate warnings, **none from this packet** | `🗑️generated/w5a-native-check.txt` |
| `cargo check … --lib --target wasm32-unknown-unknown -j 4 --keep-going` | **0 errors**, 69 crate warnings, **none from this packet** (69 warnings is also the proof type-checking reached the end of the module rather than aborting) | `🗑️generated/w5a-wasm-check.txt` |
| `cargo test … --lib -- --test-threads=1 browser_prefs_persistence` | **11 passed / 0 failed** | `🗑️generated/w5a-tests.txt` |
| `cargo test … -- --test-threads=1 browser_prefs_persistence prefs dock_ introduction tour skeleton appearance` | **106 passed / 3 failed** — all three pre-existing and already attributed by `📓️w4a` §VERIFY (see below) | same |
| `nx run …:lint` (NX_DAEMON=false) | **green** | `🗑️generated/w5a-lint.txt` |
| `…:generate-browser-boot` | **exit 0**; the artifact carries `setHostStorage` ×2, `host-storage`, `ui.introduction.seen.`, `ui.compute.workerCount`, `not a carried key` | `🗑️generated/w5a-generate-browser-boot.txt` |
| `…:generate-frame-worker` | **exit 0**; the artifact carries `semioWgpuSetHostStorage` ×2 | `🗑️generated/w5a-generate-frame-worker.txt` |
| `…:check-browser-worker` | **green** (the real TS gate: re-bundles both browser artifacts and byte-compares) | `🗑️generated/w5a-check-browser-worker.txt` |
| `…:check-frame-worker` | **green** | `🗑️generated/w5a-check-frame-worker.txt` |
| `…:test-browser-worker` | **87 passed / 2 failed** (7 files; was 79/2 over 6 files — the 8 new ones all pass, the 2 failures are unchanged and pre-existing) | `🗑️generated/w5a-ts-test-browser-worker.txt` |
| `tsc --strict --noEmit` over the six touched TS modules + the new suite | **0 errors in anything this packet authored or touched** in `🧭️boot-descriptor`, `🚪️host-io`, `🚀️browser-boot`, `🎬️renderer-boot` and the new suite; the 17 pre-existing errors in untouched regions of `🎞️frame-worker/🟦️.ts` (batch/shard-port narrowing) and `🚚️browser-frame-transport/🟦️.ts` (input normalization at `:151`, the text-chunk cast at `:906`) remain, exactly as `📓️w4a` recorded them | — |

### Failures that are NOT this packet's

1. `chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments` — window-cap
   silhouette geometry, W4b's concurrent lane. Failing before this packet (`📓️w4a` §VERIFY failure 1).
2. `ui_prefs_themes_i18n_tests::env_lock_ignores_unset_and_empty` and
   `…::shell_pref_locks_reads_the_four_lockable_envs` — **stale since W1d**: they set process env and
   expect `env_lock` to see it, but `env_lock` reads the boot descriptor (`📓️w4a` §VERIFY failure 2).
   Provably independent: this packet opened no `env_lock` code.
3. `test-browser-worker`'s two: `⏱️wgpu-ui-turn-budget` "keeps recorded-overrun traces behind the
   runtime diagnostics switch" and `⏱️wgpu-worker-step-budget` "reports what every Rust bootstrap phase
   executed for" — the latter greps `🎞️frame-worker/🟦️.ts` for `phaseUs=${step.elapsedUs}`, a literal a
   peer's uncommitted edit removed. Both were failing with the same two names in `📓️w4a`.

**One peer law was kept green on purpose, not edited.** `appearance_tour_and_footer_pill_tests::the_browser_appearance_door_is_wired_end_to_end`
asserts the literal `"storage", republishAppearance` in `🚀️browser-boot/🟦️.ts`. The storage re-seed is
therefore a SECOND `storage` listener (`republishHostStorage`) rather than a composed one, so W4a's law
still reads byte-true. Same treatment in `🎬️renderer-boot/🟦️.ts`.

---

## 5 — What the coordinator must confirm LIVE (nothing below was probed — this packet ran no serve)

The wasm needs a rebuild before any of it can be seen (the coordinator owns activations).

1. **The door seeds at boot.** The console carries exactly one
   `[DEBUG] wgpu-shell host storage seeded entries=<n>` per boot (and one more per cross-tab `storage`
   event). `n = 0` on a truly fresh profile; `n ≥ 1` once anything has been persisted. If that line is
   absent the snapshot never reached the wasm and everything below is moot.
2. **Tour dismissal persists across reload.** Fresh profile → the tour must paint (see §2's key; absent
   ⇒ arms). Click **Skip**, then reload: it must NOT come back, and
   `localStorage.getItem("ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor")` must read `"true"`.
   ⚠️ **If the tour still does not paint on a fresh profile**, the remaining cause is no longer storage:
   `should_auto_start_introduction` also needs `has_introduction`, i.e.
   `session.app.introduction.is_some()` on the wgpu side. That is W4a's arming lane, and the way to
   separate the two is the seen-flag read — with this packet the flag is readable, so a fresh profile
   that still paints no tour means the app definition carries no `introduction` in this build.
3. **Theme choice persists.** Choose Light (or Dark) in the wgpu shell's settings, reload: the choice
   must survive, and `JSON.parse(localStorage["semio.os.config"]).preferences["os.config.ui-preferences"]`
   must end with a `setAppearance` event carrying it. Then open the **React** playground on the same
   origin (6013) — it must boot with the same appearance, and the reverse.
4. **Dock override persists.** Rearrange/hide a panel, reload: the arrangement must survive, and
   `JSON.parse(localStorage["semio.os.config"]).dockLayouts` / `.dockUi` must carry the per-app layer.
   React on the same origin must open with that arrangement.
5. **Nothing outside the census is written.** After exercising the shell,
   `Object.keys(localStorage)` must contain only census members — no `semio.panelLayout.v1`, no private
   wgpu key.
6. **Native regression check.** `cargo check … --lib --bins --features native-bin` was not run here
   (W1d's hand-off, still open). The native prefs arm is unchanged, and `--lib` is green on both targets.

---

## 6 — Design notes worth keeping

* **Why the existing mailbox and not a new global.** The shell already reaches the page through
  `globalThis.semioWgpuHostIo` for file open/download and for every directory HTTP hop. A second bridge
  would be a second thing to keep alive across the Worker seam, the embeddable mount and the page.
* **Why an allowlist.** The shell is the door's only caller and its census is closed, so an unknown key
  is a defect worth refusing loudly rather than a value worth serving — and it keeps the door from being
  a wider hole into the origin's storage than the store it fronts.
* **Why the snapshot replaces rather than merges on re-seed.** The page's store IS the authority, and
  the only re-seed after boot is a `storage` event, which fires for other documents only — this
  isolate's own writes are already in both halves.
* **The appearance host term is now a fallback of a fallback.** `resolve_appearance_id`'s
  `crate::host_appearance_preference()` term (W4a) still exists and is unchanged; with the door seeding
  this isolate's own store before boot, this process's own term answers first. The host term survives
  for the embeddable door and for a realm whose snapshot was refused. `systemDark` still has to come
  from the page — there is no `matchMedia` in a Worker — so `semioWgpuSetHostAppearance` stays.

---

## 7 — Remaining gaps (honest)

1. **P1 — the brand prefix on the tour key.** React keys the seen flag `<brandId>:<appId>` on a branded
   playground; the wgpu shell resolves no `ShellBrand` at all (`📓️w1d` gap 1), so it always writes the
   bare app id. Unbranded playgrounds (puzzle3d, 6013/6113) match byte for byte; `aggregator`
   (6023/6123) does not. Closing it is the brand-registry packet's, not a storage one.
2. **P1 — `windowPanes` and `namedLayouts` have no wgpu reader.** Both projections are inside the
   carried `semio.os.config` document and survive a wgpu write untouched (`write_os_shell_config_document`
   rewrites the complete document), but React's `WindowPaneStateStore` and `NamedLayoutStore` have no
   twin here, so a saved named layout or window-pane state is preserved and ignored rather than applied.
3. **P2 — `host_storage_remove` has no production caller.** React's `StoragePort` is get/set/remove and
   the door mirrors it whole, but no wgpu lane deletes a preference: `OsShellConfig.reset()`'s only
   React caller is the ephemeral brand, which has no wgpu twin. It is the door's surface, not a live path.
4. **P2 — `scope: "session"` is served but unaddressed.** `semio.presence.client` stays classified and
   uncarried until browser document presence exists (`📓️w1e` gap 2).
5. **P2 — `SEMIO_RUNTIME_DIAGNOSTICS` is armed by the Worker URL stamp, not by the snapshot.** It has to
   be: the Worker needs it before the boot message arrives. It is in the census so the ledger is closed,
   and the two paths read the same key.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `…/🧱️elements/🐚️Shell/🧪️tests/🗄️browser-prefs-persistence/🦀️.rs` (new)
- `…/🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` (vitest `include`)
- `…/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts` (`test-browser-worker` list)
- `…/🧪️tests/🗄️wgpu-host-storage-door/🟦️.ts` (new)
- `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🤖️generated/🟨️.js`, `…/🎞️frame-worker/🤖️generated/🟨️.js` (regenerated)
