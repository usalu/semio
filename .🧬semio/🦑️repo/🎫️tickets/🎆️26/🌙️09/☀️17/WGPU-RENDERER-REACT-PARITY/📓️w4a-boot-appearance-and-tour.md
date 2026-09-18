# 🌓️ W4a — boot appearance, the introduction tour, and the footer's two missing pills

Packet W4a of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. It closes items 1, 2 and 4 of
`📓️w3c-black-chrome-regression.md` §8 ("remaining chrome differences vs React"). Line numbers are
post-edit. Every gate named under **VERIFY** was RUN, in the foreground, and its log is under
`🗑️generated/w4a-*.txt`.

Reference captures: React `🗑️generated/w3b-react/final.png` (cream page, `Welcome to Puzzle 3D`
over a blurred backdrop, Skip / `1 / 5` / Next) vs wgpu `🗑️generated/w3c-fix-3/run-1/final.png`
(dark page, no tour, no sync pill, no presence pill).

---

## 1 — Appearance: the renderer asked a `window` that does not exist

### Root cause

Two reads, both made in the wrong realm:

1. `prefers_dark_scheme()` was
   `web_sys::window().and_then(|w| w.match_media("(prefers-color-scheme: dark)")…).unwrap_or(true)`.
   The browser renderer runs inside the frame Worker (`🎞️frame-worker/🟦️.ts`), whose realm owns **no
   `window`**, so that read missed on every boot and fell through to `true`. The native arm was a
   bare `fn prefers_dark_scheme() -> bool { true }`. React's own function,
   `resolveElementsSurfaceChromeDark` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:1691`),
   falls through to **`false`** for the same "no window" case. That single inverted default is the
   whole "React boots LIGHT, wgpu boots DARK" difference.
2. **Second, quieter cause, found while fixing the first:** `WebLocalStorage::new()`
   (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, region `🗄️PrefsStore`) resolves `localStorage` through
   `web_sys::window()` too. In the Worker realm it therefore resolves to `None`, so **the whole
   persisted `os.config.ui-preferences` store is invisible to the browser wgpu shell** — a user who
   had chosen Light in React's shell was ignored even before the `"system"` fallback ran. See §5 for
   what that still costs beyond appearance.

### Fix

One published signal carrying exactly the two inputs a renderer cannot read for itself, fed by
whoever owns a window, and a resolution law that folds it in where React folds `preferences.appearance`.

| site | change |
| --- | --- |
| `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:9116-9205` | new `//#region 🌓️HostAppearance`: `HostAppearancePreference` (`Unset/System/Light/Dark` + `from_id`/`as_id`), `HostAppearance { preference, system_dark }`, a `thread_local` **defaulting to `system_dark: false`**, `set_host_appearance` (`:9176`), `host_appearance` (`:9181`), `host_appearance_preference` (`:9187`), the `#[wasm_bindgen(js_name = semioWgpuSetHostAppearance)]` hook (`:9197`), and `prefers_dark_scheme` reduced to one read of it (`:9202`) |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:19884` | `appearance_id` now returns `Option<String>` — React's `preferences.appearance` BEFORE its `?? "system"` seed |
| `🐚️Shell/…:19902` | new `resolve_appearance_id(&UiPreferences)` — the ONE law: `env_lock("SEMIO_LOCKED_APPEARANCE")` → this process's own store → the host's read of the same store → `"system"`. Its two call sites are the live preference load (`:16448`) and the test-only `load_ui_prefs_once` |
| `🪟️winit-app/🦀️.rs:761` | `publish_system_appearance(Option<winit::window::Theme>)` — updates only `system_dark`, and a `None` (a platform with no preference, and every wasm realm) changes nothing, so a page-forwarded value survives window creation |
| `🪟️winit-app/🦀️.rs:798`, `:908` | seeded from `window.theme()` in `resumed`, re-published on `WindowEvent::ThemeChanged` plus a `request_redraw` — the twin of React's ONE shared `matchMedia` `change` listener (`ensureElementsSurfaceChromeSystemListeners`) |
| `🧭️boot-descriptor/🟦️.ts:187-254` | new `🌓️HostAppearance` region: `WGPU_PREFERS_DARK_MEDIA_QUERY`, `WGPU_OS_SHELL_CONFIG_STORAGE_KEY`, `WGPU_UI_PREFERENCES_CONFIG_SCHEMA`, `WgpuHostAppearance`, `readPersistedAppearancePreference` (`:222`, replays the event log for its last `setAppearance`) and `resolveWgpuHostAppearance` (`:245`) |
| `🚚️browser-frame-transport/🟦️.ts:178`, `:184`, `:223`, `:505` | `appearance` on `BrowserFrameWorkerBoot` (an environment axis beside `locale`/`dpr`, deliberately NOT a boot axis — see below), the new `BrowserFrameHostAppearance` message, and `transport.setHostAppearance()` |
| `🎞️frame-worker/🟦️.ts:57`, `:308`, `:521` | the `semioWgpuSetHostAppearance` binding, applied in the `runtime-environment` owned step at boot and again on every `host-appearance` message |
| `🚀️browser-boot/🟦️.ts:44`, `:414`, `:489-496` | `hostAppearance()` on the page thread, into the boot message, plus a `matchMedia` `change` and a `storage` listener (React re-reads on `storage` too — `🎚️UiPreferences/🟦️.ts`'s `installBrowserStorageListener`), both torn down on `pagehide` |
| `🎬️renderer-boot/🟦️.ts:188-203` | the embeddable door runs ON the page, so it makes both reads itself, through the same hook, and keeps them live for the life of the mount |

**Why not the boot descriptor.** `semio-locked-appearance` already reaches the shell as
`descriptor.locks.appearance` (W1d) and is unchanged. The system preference and the persisted
preference are not boot axes: neither is per-navigation, both keep changing after boot, and neither
has a counterpart on React's `FrameworkOsBootOptions` — adding them to `WgpuBootDescriptor` would
have meant three new allowlist rows in `🧪️tests/🧭️boot-axis-parity/🦀️.rs` for axes no door wants.
They travel as an environment field beside `locale`, `width`, `height` and `dpr`, which is the shape
the transport already had for exactly this class of input.

### React reference

- `resolveElementsSurfaceChromeDark` — `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:1691`.
- the precedence — `🏛️ShellHost/🟦️.tsx:2166-2174`
  (`dispatch({ type: "SET_UI_APPEARANCE", value: locks.appearance ?? resolved.appearance })` over
  `resolveUiPreferences(preferences, { appearance: "system", … })`).
- the persisted shape — `🎚️UiPreferences/🟦️.ts` (`OsShellConfig` → `os.config.ui-preferences` →
  `{version:1,events:[…]}`), and the page-level reader React's own serve injects before first paint,
  `PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT` (`🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:425`), which
  `readPersistedAppearancePreference` mirrors step for step.

---

## 2 — Boot tour: `start_introduction` was `#[cfg(test)]`

### Root cause

`ShellChromeBuildState::start_introduction` carried `#[cfg(test)]` and its only callers were the five
in `🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs`. **Nothing in production ever armed a tour.** The
whole `🔖️ChromeOverlaysAndTour` twin — veil, info box, per-step reveal (`chrome_tour_frame_begin`),
advance-by-doing (`chrome_tour_complete_interaction`), the Escape/Enter/Arrow keys — was reachable
only from its own unit tests. The first-run persistence key was already correct and already read
(`ui.introduction.seen.<appId>`, byte-identical to React's
`UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX`); the step source was already correct
(`session.app.introduction`, re-read fresh every frame). Only the arming was missing.

Two further divergences fell out once it ran:

- **Skip did not persist.** React's `dismissIntroduction` (`🏛️ShellHost/🟦️.tsx:4348-4356`) writes
  the seen flag on Skip exactly as on Done. Here only a last-step *advance* marked it, and the
  click-Skip path called the answer-less `skip_introduction`, so a skipped tour returned on the next
  launch. Clicking **Next on the last step** had the same hole — it called `advance_introduction`,
  which just clears the state.
- **The card was missing two of React's four controls.** `UIIntroduction`
  (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:5778-6065`) renders Skip in the cap row (`close` slot), and a
  footer of Back (left, from step 2 on) / `stepIndex + 1 / steps.length` (centre) / Next-or-Done
  (right). wgpu painted Skip where Back belongs and **no step counter at all** — the `1 / 5` in the
  React capture had no twin.

### Fix

| site | change |
| --- | --- |
| `🐚️Shell/…:14960` | `start_introduction` is production code |
| `🐚️Shell/…:15001` | new `dismiss_introduction(app_id)` = React's `dismissIntroduction`: end the tour AND record the answer. `skip_introduction` survives as the answer-less variant for the two places a TUTORIAL takes over (Design Decision 8) |
| `🐚️Shell/…:15020` | new `should_auto_start_introduction(app_id, has_introduction, tutorial_active, seen)` — React's `shouldAutoStartIntroduction` (`🐚️Shell/🟦️.tsx:322`). Two of its seven terms are deliberately absent, not stubbed: `suppressed` is for embedded multi-shell hosts (this renderer mounts one shell per surface) and `replayOnLoad` is a BRAND property (wgpu has no brand registry — `📓️w1d`'s gap 1). React's `dismissedAppIds` set and its `seenOnDevice` read collapse into `seen`, because the in-memory `introduction_seen` map IS the projection of that storage key and `mark_introduction_seen` writes it before the store does |
| `🐚️Shell/…:16387` | new `ShellState::auto_start_introduction`, called from the ONE place the answer becomes known — the `ui.introduction.seen.<appId>` read landing in `advance_chrome_maintenance_step` (`:16424`). It can neither arm before the flag is readable nor re-arm afterwards: the read is requested only while the map lacks the key, and the key is written there |
| `🐚️Shell/…:18439` | the card: Skip moved to the cap row, footer Back (`shell.tour.back`, only when `step_index > 0`), the `i / n` counter, Next/Done, and both Skip and a last-step Done now route through `dismiss_introduction` |
| `🐚️Shell/…:10980` | the Escape key routes through `dismiss_introduction` too |

**The veil is deliberately NOT blurred and was left alone.** React paints `ui-veil`
(`backdrop-filter: blur(var(--veil-blur))`). This renderer paints `theme.veil(Level::Dialog)`, the
same level fill at the same `--veil-alpha`, as a solid quad — and `introduction_veil_bands`'s own
docstring (`🐚️Shell/…:15142`) records why: `run_blur_chain` only mips the MAIN draw list's scene
texture, chrome paints into the *overlay* list, and overlay glass regions composite *before* the
overlay's own instance pass. A glass veil there would frost the canvas and leave the chrome
unfrosted. That is a written architecture decision of the overlay lane, not an oversight, so this
packet did not overturn it. `Theme::veil_blur_px()` (`🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs:418`)
consequently still has no production consumer — see §5.

---

## 3 — Footer: `Remote: detached` and the presence pill

### Root cause

`render_footer_step`'s phase 3 body was wrapped in `#[cfg(not(target_arch = "wasm32"))]` and
`render_sync_status_and_checkin` carried the same attribute, because its input was the native-only
`store_sync::ArtifactSyncStatus`. The browser build therefore painted **no sync pill and no
check-in** at all. React's browser shell has no backbone attached either — its
`computeSyncPillState(null)` (`🛠️ShellHelpers/🟦️.tsx:2647`) reads `{ kind: "remote", remote:
"detached" }` and it shows the pill anyway. The gate was on the TYPE, not on the state.

The presence pill had never been emitted on either target: `ShellState::presence_peers` existed,
`presence_peer_rows_for_surface` existed (but was `#[cfg(test)]`, i.e. production-dead), the element
existed (`👥️PresenceBar/🎯️targets/🧊️wgpu/🦀️.rs`) — and no footer phase ever painted it. React keeps
`#s-presence-peers` mounted whenever a document is open (`🏛️ShellHost/🟦️.tsx:10634`).

The pill was also **English-only**, so a `de` shell read `Remote: detached` where React reads
`Remote: getrennt`.

### Fix

| site | change |
| --- | --- |
| `🐚️Shell/…:12071-12115` | new `//#region 🚦️SyncPill`: `ShellSyncRemote`, `ShellSyncPill` (React's `SyncPillState` plus this renderer's own `Recovering` arm, a declared divergence with no React twin) and `shell_sync_pill_text(pill, is_de)` — `syncPillText`'s truth table, **both** locales, through `shell_chrome_string` |
| `🐚️Shell/…:19736-19753` | the ten new `sync.*` chrome strings, term for term with `syncPillText`'s own `FrozenLabel`s |
| `🐚️Shell/…:7025` | new `ShellState::sync_pill()` — the native-only types stop HERE. Native folds `sync_bootstrap_progress`/`sync_status` exactly as `computeSyncPillState` does; the wasm arm answers `Remote(Detached)`, which is the state React's browser shell resolves |
| `🐚️Shell/…:7005` | new `ShellState::footer_presence_rows()`, giving `presence_peer_rows_for_surface` (now un-`cfg(test)`) its first production caller; wasm answers an empty roster |
| `🐚️Shell/…:12131` | `render_sync_status_and_checkin` is target-neutral and takes `(pill, presence, locale, …)`; a new `cursor.item == 1` arm paints `#s-presence-peers` between the sync pill and `#s-checkin` |
| `🐚️Shell/…:17778` | the footer's phase 3 is no longer `cfg`-gated |
| `👥️PresenceBar/🎯️targets/🧊️wgpu/🦀️.rs:151`, `:157`, `:167` | `presence_empty_label`, `presence_overflow_label` and `presence_bar_chip_text` — the one-line chip form of the roster, defined in terms of the same `LocalizedLabel`s `build_presence_bar_localized` uses (both now call the two label helpers), so the chip and the `UiNode` roster cannot drift. Re-exported from `ui_wgpu::wgpu` (`🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs:258`) |

The chip form is used rather than the `UiNode` tree because every other shell-owned footer pill
(`#s-sync-status`, `#s-checkin`) is an immediate-mode `ChromeGroupItem` for the same reason: these
are shell chrome, not plugin-declared nodes.

---

## 4 — Tests

New law file `🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs`, mounted at
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:20053` — **11 tests, all RUN, all green**
(`🗑️generated/w4a-tests.txt`).

| law | pins |
| --- | --- |
| `appearance_resolves_lock_then_store_then_host_then_system` | the full precedence (lock > own store > host read > `"system"`), that the host read is a fallback and never an override, and `resolveElementsSurfaceChromeDark`'s three-way table over both `system_dark` values |
| `an_unpublished_host_appearance_reads_light` | the default is LIGHT, the inverted-default defect itself; plus the `""`/`system`/`light`/`dark` wire round-trip |
| `the_browser_appearance_door_is_wired_end_to_end` | the hook is exported, no `match_media` call survives in the renderer, the Worker applies boot AND live values, the page makes both reads and keeps them live, and the page reads React's own `os.config.ui-preferences` key |
| `a_fresh_profile_arms_the_app_introduction_exactly_once` | `shouldAutoStartIntroduction`'s five rows, then the same four rows driven through a real `ShellState` + `ActiveSession` carrying a real two-step `IntroductionDefinition` |
| `dismissing_the_tour_records_the_answer_and_stops_the_read_that_would_re_arm_it` | Skip and Done both persist, and `request_introduction_read` then requests nothing — the mechanism that makes "once" true |
| `the_introduction_read_arms_the_tour` | `start_introduction` is not `cfg(test)`, and the read arm calls `auto_start_introduction` (the defect was a correct tour nothing started) |
| `the_tour_card_carries_reacts_own_controls` | Skip/Back/Next/Done labels, the `i / n` counter, the three hit ids, and that Skip/Done persist |
| `the_sync_pill_speaks_reacts_own_vocabulary` | `syncPillText`'s nine rows across both locales |
| `a_shell_with_no_backbone_reads_remote_detached` | `computeSyncPillState(null)` on a real shell, on both targets |
| `the_presence_pill_shares_the_elements_own_copy` | the element's own empty state in both locales, the visible cap and the overflow suffix |
| `the_footer_pills_are_not_gated_off_the_browser_build` | nothing between the footer phase and the pill renderer is native-gated, the wasm arm answers `Remote(Detached)`, and all three control ids are painted |

Three of the eleven read SOURCE rather than running code. That is the same instrument
`🧪️tests/🧭️boot-axis-parity/🦀️.rs` uses and it is the honest one here: what they assert is that a
`cfg(target_arch = "wasm32")` path EXISTS, and that code never compiles into a native test binary,
so running the native build could prove nothing about it (see `📓️`-level note: a native `cargo test`
is blind to wasm-gated code).

One existing law was updated for the new API, not deleted:
`🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs:205` —
`sync_pill_text_covers_persisted_pending_and_every_remote_state` now drives
`ShellState::sync_pill()` on a real shell instead of the deleted associated function, and keeps every
one of its assertions (including the non-live-outranks-pending priority row).

Eight stale `boot:` fixture literals in three TS suites (`🧪️tests/📨️browser-frame-transport/🟦️.ts`
×6, `⏱️wgpu-ui-turn-budget/🟦️.ts`, `🎮️wgpu-browser-input-wire/🟦️.ts`) still spelled the **pre-W1d**
loose axes (`pluginVariant`/`appRole`/`appMode`/`appExample`) instead of `descriptor`; they passed
only because bun strips types without checking them. They now build their descriptor through
`resolveWgpuBootDescriptor` and carry `appearance`, so the seam is type-correct again.

---

## VERIFY (all foreground, all RUN)

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **0 errors** | `🗑️generated/w4a-native-check.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | **0 errors** | `🗑️generated/w4a-wasm-check.txt` |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | **0 errors** | `🗑️generated/w4a-ui-check.txt` |
| `cargo test … --lib -- --test-threads=1 appearance_tour_and_footer_pill_tests` | **11 passed / 0 failed** | `🗑️generated/w4a-tests.txt` |
| `cargo test … --lib -- --test-threads=1 … chrome_overlays_tour_tests identity_directory_presence boot_axis_parity` | **74 passed / 1 failed** (the failure is not this packet's — see below) | same |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- --test-threads=1 presence` | **9 passed / 0 failed** | — |
| `nx run @semio-tech/framework-renderer-wgpu:lint` (NX_DAEMON=false) | **green** | `🗑️generated/w4a-ts-lint.txt` |
| `…:generate-browser-boot` | **exit 0**, artifact carries the new code | `🗑️generated/w4a-generate-browser-boot.txt` |
| `…:generate-frame-worker` | **exit 0**, artifact carries the new code | `🗑️generated/w4a-generate-frame-worker.txt` |
| `…:check-browser-worker` | **green** (the real TS gate: re-bundles both browser artifacts through the schema-owned-import resolver and byte-compares) | `🗑️generated/w4a-check-browser-worker.txt` |
| `…:check-frame-worker` | **green** | `🗑️generated/w4a-check-frame-worker.txt` |
| `…:test-browser-worker` | **79 passed / 2 failed** (both pre-existing — see below) | `🗑️generated/w4a-ts-test-browser-worker.txt` |
| `tsc --noEmit --strict` over the five touched TS modules + the three touched TS suites | **0 errors in anything this packet authored**; 18 pre-existing errors remain in untouched regions of `🎞️frame-worker/🟦️.ts` (batch/shard-port narrowing, `Error.stackTraceLimit`) and `🚚️browser-frame-transport/🟦️.ts` (input normalization at `:151`, the text-chunk cast at `:881`) | — |

The repo has **no `typecheck` target** on this project — `lint` is a color-literal/artifact-home
check and `check-browser-worker` is the bundling gate. The `tsc` row above was run from a scratch
`tsconfig` extending the repo's, over exactly the eight touched files.

### Failures that are NOT this packet's, each attributed

1. `chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments` — "gap
   baseline must sit under the cutout between tabs and controls". Window-cap silhouette geometry;
   this packet opened no silhouette code, and W4b is editing the Dock/Shell cap and pane chips
   concurrently.
2. `ui_prefs_themes_i18n_tests::env_lock_ignores_unset_and_empty` and
   `…::shell_pref_locks_reads_the_four_lockable_envs` — **stale since W1d**, provably independent of
   this packet: `env_lock` (`🐚️Shell/…:19293`, unchanged here) resolves through
   `crate::boot_locks()` and matches only the five `SEMIO_LOCKED_*` names, so
   `env_lock("SEMIO_WP14_TEST_EMPTY_VAR")` can never return `Some("en")` whatever this packet does,
   and a `std::env::set_var` after the thread's `BOOT_DESCRIPTOR` has initialised cannot reach it.
   They need re-pointing at the descriptor.
3. `⏱️wgpu-ui-turn-budget` "keeps recorded-overrun traces behind the runtime diagnostics switch" and
   `⏱️wgpu-worker-step-budget` "reports what every Rust bootstrap phase executed for" — both are
   source-greps against `🎞️frame-worker/🟦️.ts`, whose uncommitted peer edits removed the literal
   they look for (`phaseUs=${step.elapsedUs}` appears nowhere in the file). Both test files are
   themselves unmodified in the working tree, which is what settles the attribution.
4. `nx run …:normalized-presence-rows-source-check` — **red before this packet and for a reason
   unrelated to it**: its oracle (`📦️packages/🟦️typescript/📜️script.ts:241`) greps the SHELL source
   for five markers, but three of them (`fn presence_rows_require_each_normalized_surface_and_
   preserve_hub_color(`, `peer("c", None, 9)`, `vec![("b", Some(8))]`) live in
   `🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs`, a `#[path]`-mounted sibling file the oracle
   never opens. The two markers it CAN find are both intact. It needs re-pointing at the test file.

---

## 5 — What the coordinator must confirm LIVE (nothing below was probed — this packet ran no serve)

1. **The wgpu boot is now LIGHT on this host.** Re-take `🐍️w3c-wgpu-present-probe.mjs` and compare
   the page background against `🗑️generated/w3b-react/final.png`'s cream. This is the one claim that
   can only be settled in a browser: the fix moves two reads into the page realm, and only a real
   page has a real `matchMedia` and a real `localStorage`.
2. **The tour paints on a fresh profile.** A profile that has already answered it will NOT show it —
   clear `ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor` (or use a fresh browser profile) before
   judging. Expect `Welcome to Puzzle 3D`, `Skip` in the cap row, `1 / 5` centred in the footer and
   `Next` on the right, over a solid (not blurred, §2) veil.
3. **Skip/Done must not re-show the tour on the NEXT boot** — and here is the catch, which is a real
   remaining defect, not a doubt about this packet: **on the browser build the write will not
   persist.** `prefs_set_bounded` goes through the same `WebLocalStorage` that has no `window`
   (§1 cause 2), so `ui.introduction.seen.*` is written to nothing. Within one process the in-memory
   map holds, so the tour is answered once per LOAD; across reloads it will re-appear until the
   prefs store crosses the Worker seam. Confirm that behaviour rather than treating it as a
   regression.
4. **The footer now carries `Remote: detached` and `No one else is here`** between the utility rail
   and the bottom-left tab row. Check they do not collide with W4b's footer/pane chip work — both
   land in the same bottom-left band (`footer_tab_row_rect`'s `lead_x`).
5. **Native**: `cargo check -p semio-framework-os-renderer-wgpu --lib --bins --features native-bin`
   was not run here (W1d's own hand-off, still open). The winit edits are in the lib and the native
   `--lib` check is green, but the bin has not been compiled since.

---

## 6 — Hand-offs (ordered by what they cost)

1. **P0 — the browser wgpu shell has NO persistent preference storage at all.**
   `WebLocalStorage::new()` (`🐚️Shell/…`, region `🗄️PrefsStore`) reaches `localStorage` through
   `web_sys::window()`, which is `None` in the frame Worker. Every `prefs_get`/`prefs_set` in the
   browser build is therefore a no-op: appearance, locale, terminology, theme, custom themes,
   keybinding overrides, the compute worker count, dock layouts and the introduction seen-flag are
   all read as empty and written to nothing. This packet routes the ONE field it owns (appearance)
   around the gap; the honest fix is to serve the whole store over the existing `🚪️host-io` page
   door, which already crosses exactly this seam for file import/export. Until then every React↔wgpu
   "preference does not stick" report in the browser has this one cause.
2. **P1 — the overlay lane cannot paint a blurred veil.** `Theme::veil_blur_px()` still has no
   production consumer; React's tour, dialog and search overlays all sit on `backdrop-filter`. The
   blocker is structural and documented (`introduction_veil_bands`'s docstring): the blur chain mips
   only the scene texture and overlay glass composites before the overlay instance pass. Fixing it
   is the overlay/compositor lane's call, and it would also remove the geometric cutout machinery.
3. **P1 — the tour info box is fixed at 320×168, centred.** React's box is `w-fit`, anchored to
   `step.introduce` through `resolveIntroductionPlacement` (which this file already ports, at
   `🐚️Shell/…:15177`, and which nothing calls), and draggable by its cap handle. A step whose copy
   exceeds the fixed box is clipped by `chrome_text_complete_step`'s flow width.
4. **P2 — `normalized-presence-rows-source-check`'s oracle reads the wrong file** for three of its
   five markers (§VERIFY failure 4). One-line fix in `📦️packages/🟦️typescript/📜️script.ts:241`.
5. **P2 — the two `ui_prefs_themes_i18n` env-lock laws are stale since W1d** (§VERIFY failure 2):
   they set process env and expect `env_lock` to see it, but `env_lock` reads the boot descriptor.
   They should drive `apply_boot_descriptor` instead.
6. **P2 — `?mode=` and the brand registry** remain as `📓️w1d` left them; `boot_brand_id()` is
   threaded but nothing resolves a `ShellBrand`, so `replayIntroductionOnLoad` and `ephemeral` have
   no wgpu twin and `should_auto_start_introduction` carries no `replay_on_load` term.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `…/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs`
- `…/🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts`
- `…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `…/🧱️elements/🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs` (new)
- `…/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs`
- `…/🧪️tests/📨️browser-frame-transport/🟦️.ts`, `…/🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts`,
  `…/🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs` (re-exports only)
