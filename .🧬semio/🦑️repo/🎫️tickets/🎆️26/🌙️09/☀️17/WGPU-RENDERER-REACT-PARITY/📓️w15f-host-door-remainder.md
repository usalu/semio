# 🚪️ W15f — host/boot-door remainder (source + tests, wasm32 checked, NO activation)

Packet: the six items `📓️audit-w14-transport-residual.md` handed W15f — its rows **6, 19, 10, 21, 9,
20** (dispatch items 2, 5, 6, 7, 8, 9). Source-only; W14d owns live verification.

**Headline: three of the six were already closed, and one is a false gap.** Every item below was
grepped against React's own source BEFORE any edit, per the packet's instruction, and the audit's
claim is stated beside what the tree actually holds. Two real defects were found and fixed (item 2's
silent platform bug, item 7's unconsumed persistence), one real gap was ported (item 8's brand row),
and three rows are pinned by standing laws so they cannot silently regress into being true again.

All new laws live in ONE new file,
`🧱️elements/🐚️Shell/🧪️tests/🚪️wgpu-host-door-remainder/🦀️.rs`, registered from
`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` as `mod host_door_remainder_tests`.

---

## Item 2 — `command_host_platform()` reads the ⌨️HostPlatform door (audit row 6, P0) — **FIXED**

| | |
|---|---|
| React source of truth | React resolves the platform ONCE for the whole shell and filters `PlatformKeybinding.platform` against that one answer; the shared rule is `keybindingPlatformUsesMetaV1` (`🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts`), which W7a already ported into the `⌨️HostPlatform` door. |
| Defect (confirmed) | `command_host_platform()`'s `wasm32` arm re-derived the platform from `web_sys::window()`. The wgpu shell runs in the frame Worker, whose realm owns **no `window`** — the read was `None` on every machine, `unwrap_or_default()` was `""`, and the answer was **always `Linux`**. Two live callers: `handle_keyboard_async`'s chord gate and `command_search_items`. |

**wgpu change**

- `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:9224-9295` (`//#region ⌨️HostPlatform`): new
  `keybinding_platform_kind(&str) -> semio_framework::manifest::Platform` derived from the SAME string
  `keybinding_platform_uses_meta` reads; `const fn compiled_host_platform()`; a second `thread_local`
  `HOST_PLATFORM` initialised from it; `set_host_platform_kind`, `host_platform()`, and one
  `set_host_platform(&str)` that publishes BOTH readings.
- same file, `semio_wgpu_set_host_platform` (the `semioWgpuSetHostPlatform` wasm hook) now calls
  `set_host_platform(&platform)` — the page's existing read already carried the full string, so **no
  transport change was needed**.
- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `command_host_platform()` is now
  `crate::host_platform()` and nothing else. The `web_sys::window()`/`js_sys::Reflect` chain is gone.

**Law** — `the_shell_reads_its_platform_from_the_host_platform_door`: the function's own SOURCE
contains `crate::host_platform()` and no `web_sys::window()`; the door resolves
`MacIntel`/`macOS`/`iPhone`→`MacOs`, `Win32`/`Windows`→`Windows`, `Linux x86_64`/`""`→`Linux` with
`host_platform_uses_meta()` agreeing on each; and the palette's `os.toggleFullscreen` **description**
differs between a Mac and a Windows door (`f11` on the latter, not on the former) — the audit's own
demonstrated cost, asserted end to end.

Native only (`cfg!(target_os)`) is unchanged: the thread-local initialiser *is* the OS answer, the
same compile-time fact winit runs on.

---

## Item 5 — direct undo/redo chords (audit row 19, P1) — **ALREADY CLOSED; audit row stale**

The audit says "absent in `handle_keyboard_async` / `build_os_commands()` … not re-verified as fixed
by any Wave 1–13 report". It **is** fixed: W12c wired it.

| | |
|---|---|
| React source of truth | `🏛️ShellHost/🟦️.tsx:8726-8737` — the tail of `handleAppKeydown`, AFTER the app-keybinding loop: `(ctrlKey \|\| metaKey) && !altKey && key === "z"` (shift ⇒ redo) and the `mod+y` redo alias, both through the same `onAction` funnel the History panel's rows use. |
| wgpu | `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `handle_keyboard_async`'s last `if idle` block dispatches `shell_edit_verb_for(&action, modifiers)` through `dispatch_action` against the session's `controller_id`; `shell_edit_verb_for` + `ShellEditVerb` are in the `⌨️ShellShortcutTable` region. Matching uses `key_event_matches_chord`, whose `mod` token accepts `meta \|\| ctrl` and requires `alt == false` — React's gate exactly. |
| Pre-existing laws | `🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs:116,121` and `🧪️tests/⌨️wgpu-chord-example-role-parity/🦀️.rs:146`. |

**No source change.** New law added anyway, because the two properties the ROUTING depends on were
unpinned: `the_undo_redo_chords_match_reacts_gate_and_stay_shadowable` asserts both accelerators
answer on every platform, the alt/`mod+shift+y` refusals, the two action ids, and — the load-bearing
half — that `mod+z`/`mod+shift+z`/`mod+y` are **not** in `reserved_shell_chords_v1`, which is what
lets an app that declares `mod+z` shadow them the way React's tail does.

---

## Item 6 — OS-level file drag-and-drop (audit row 10, P1) — **FALSE GAP: React has none either**

The audit inferred this from a `DragEvent` import in the ui React target and admitted in its own §4
that React's handler "was not traced to a file:line". It was traced for this packet.

**Repo-wide grep result: `dataTransfer.files` / `dataTransfer.items` appear in ZERO `.ts`/`.tsx`
files.** Every drop handler in the React tree reads an in-app MIME:

| React handler | MIME |
|---|---|
| `🌐️World3dHost/🟦️.tsx:7159-7265` (incl. the one `window.addEventListener("dragover")` in the tree) | `CATALOGUE_DRAG_MIME` |
| `📐️Canvas2dHost/🟦️.tsx:871-893`, `🖥️Board2dHost/🟦️.tsx:1402-1423`, `🕸️NodeGraph/🟦️.tsx:1133-1138,3452` | `CATALOGUE_DRAG_MIME` |
| `🧩️BlockListHost/🟦️.tsx:105-145` | `PALETTE_DRAG_MIME` |
| `🖼️Panel/🟦️.tsx:136-153`, `🧭️PanelTabBar/🟦️.tsx:439-446` | `PANEL_TREE_UNIT_MIME` |
| `🎨️Canvas/🟦️.tsx:1703-1717` | `COMPOSE_WINDOW_TEMPLATE_MIME` |
| `🌳️Tree/🟦️.tsx`, `📊️Table/🟦️.tsx` | tree-unit / row drag mimes |

So the only file-open door on **either** renderer is the picker (`request-file-open` on wgpu,
`<input type=file>` on React). Building page-side `dragover`/`drop` listeners on wgpu alone would be a
**divergence**, the same one `📓️w1d` refused for `?mode=`. I did **not** build it.

**wgpu change: none.** **Law** — `neither_renderer_accepts_an_os_file_drop`: walks every `🟦️.tsx`
under `🧱️elements` and asserts none reads `dataTransfer.files`/`.items`, and that
`🚀️browser-boot/🟦️.ts` registers no `drop`/`dragover`/`dragenter`/`dragleave` listener; it also
asserts the two doors that DO exist (`request-file-open`, `download-media-export`) are still wired.
The failure message names the exact wgpu seam to build (page-side listeners → the existing host-io
upload door) for the day React grows one — at which point the law goes red and the port is owed.

---

## Item 7 — `windowPanes` / `namedLayouts` readers (audit row 21, P1) — **HALF REAL, FIXED; half a false gap**

### `namedLayouts` — real gap, fixed

| | |
|---|---|
| React source of truth | `NamedLayoutStore` (`🧰️framework/🔨️modules/🖥️platform/🟦️.ts:313-364`), constructed at `🏛️ShellHost/🟦️.tsx:3507` as `new NamedLayoutStore(session?.app.id ?? "framework-os", shellStorage)`, read into the Layout panel by `useNamedLayoutHost` (`📌️ChromePanels/🟦️.tsx:1125-1157`), applied by `applyNamedLayout`, which searches `[...builtinLayouts, ...userLayouts]`. Storage shape: a **flat** `namedLayouts[<appId>]` array inside `semio.os.config` (NOT the `{os, apps}` layer shape `dockLayouts`/`dockUi`/`windowPanes` use), read back through the filter `origin === "user"`. |
| Defect | W5a made the whole document round-trip through the storage door, so a layout saved in the React shell survived a switch to wgpu **on disk** — and no wgpu lane read it. Separately, the `saveCurrentLayout`/`deleteUserLayout` verbs (landed by the concurrent W15c) mutated `ShellState::user_layouts` and persisted **nothing**, so even a layout saved on wgpu died at the next boot. |

**wgpu change** (all in `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, beside the existing dock-store
helpers):

- `NAMED_LAYOUT_STORE_FALLBACK_APP_ID` (`"framework-os"`), `named_layout_store_app_id`,
  `read_named_layouts(&Value, Option<&str>)`, `write_named_layouts_in`,
  `load_named_layouts_from_store`, `save_named_layouts_to_store` — pure-over-the-document readers, the
  same shape `read_os_shell_config_layer` uses for the dock layers, with React's `origin: "user"`
  filter.
- `ShellState::load_persisted_named_layouts()` + `persist_named_layouts()` + `named_layout_by_id()`
  in the `🧭️PanelAnchorAccessors` region; `load_persisted_dock()` now ends by calling the loader, so
  the roster is restored on boot AND on every app change (React re-memoizes the store on
  `session?.app.id`).
- the `shell.layout.<id>` press arm now resolves through `named_layout_by_id` (app-declared **then**
  saved), React's `[...builtinLayouts, ...userLayouts].find(…)`.
- `persist_named_layouts()` added after the `saveCurrentLayout` and `deleteUserLayout` mutations.

### `windowPanes` — false gap

`WindowPaneStateStore` (`🖥️platform/🟦️.ts:517-589`) has **no production caller in React**: the only
references outside its own definition are in `🧰️framework/🧪️tests/🧪️docklayoutstore/🟦️.ts`. Giving
wgpu a reader would invent behaviour React does not have. The projection stays carried and unconsumed
on both sides — the audit's "carried, unconsumed" is true of React too.

**Laws** — `saved_named_layouts_round_trip_through_reacts_own_document` (flat per-app key, the
`framework-os` fallback, the `origin: "user"` read filter, sibling projections untouched) and
`window_panes_stay_consumer_less_on_both_renderers` (scans React's element tree for a
`WindowPaneStateStore` mount; the failure message names where the wgpu reader goes).

---

## Item 8 — brand registry port (audit row 9, P1) — **PORTED**

| | |
|---|---|
| React source of truth | The catalogue `SHELL_BRANDS` + `resolveShellBrandById` (`🧑‍💻dev/🏷️brand/🟦️.ts`, eight demonstrator brands); the facts the shell reads are `ShellBrand.windowTitle`, `.ephemeral`, `.replayIntroductionOnLoad`, and the predicates `shouldReplayIntroductionOnLoad` / `shouldPersistIntroductionSeen` / `isEphemeralShellBrand` (`🧱️elements/🐚️Shell/🟦️.tsx:296-310`). The demonstrated cost is `introductionSeenKey` at `🏛️ShellHost/🟦️.tsx:4159`: `brand ? \`${brand.id}:${session.app.id}\` : session.app.id`. |
| Defect | wgpu carried a bare `brand_id` and nothing else (`📓️w1d` gap 1), so on `aggregator` it wrote `ui.introduction.seen.<appId>` where React writes `ui.introduction.seen.<brandId>:<appId>` (`📓️w5a` §7.1), and it had no `replayIntroductionOnLoad`/`ephemeral` at all. |

**Design**: the catalogue stays TypeScript — porting eight brand definitions into Rust would be a
second catalogue that drifts. What crosses is the **resolved row**, exactly as `locks`/`defaults`
already do, over the boot-descriptor door W1d built. A brand added to the TS catalogue needs no Rust
edit.

**wgpu change**

- `🧊️renderer/🦀️.rs`: new `WgpuBootBrand { window_title, ephemeral, replay_introduction_on_load }`
  with `replays_introduction()` / `persists_introduction_seen()` (React's two predicates, `ephemeral`
  implying replay); `WgpuBootDescriptor.brand`; `brand.windowTitle` added to `validated()`'s bounded
  field list (13 → 14); native env seeds `SEMIO_BRAND_WINDOW_TITLE` / `SEMIO_BRAND_EPHEMERAL` /
  `SEMIO_BRAND_REPLAY_INTRODUCTION`; new reader `boot_brand()`.
- `🧭️boot-descriptor/🟦️.ts`: `WgpuBootBrand` type, `WgpuBootDescriptor.brand`,
  `WgpuBootOverrides.brand`, three new `WGPU_BOOT_META_NAMES` (`semio-brand-window-title`,
  `semio-brand-ephemeral`, `semio-brand-replay-introduction`) and their resolution in
  `resolveWgpuBootDescriptor`. **No frame-worker/transport change**: the descriptor already crosses as
  JSON.
- `🌐️server/🟦️.ts`: the three meta tags injected from the matching `SEMIO_BRAND_*` env.
- `🐚️Shell/…/🦀️.rs`: `ShellState::introduction_seen_key()` (React's rule); `request_introduction_read`,
  the two `dismiss_introduction` call sites, and the I/O lane's read/write arms all key on it;
  `should_auto_start_introduction` gained a fifth term `replay_on_load`, fed by
  `crate::boot_brand().replays_introduction()`; the seen WRITE is gated on
  `persists_introduction_seen()`.
- `🪟️winit-app/🦀️.rs:781`: the native window title is the brand's, `"Semio"` when unbranded.

**Laws** — `the_introduction_seen_key_is_brand_scoped` (bare app id unbranded, `"<brandId>:<appId>"`
branded, and the 4-case truth table for the two predicates); the existing
`📓️w1d` boot-axis law file `🧪️tests/🧭️boot-axis-parity/🦀️.rs` extended so
`the_descriptor_rust_twin_matches_the_typescript_shape` also compares `WgpuBootBrand` field-for-field;
a new TS case in `🧪️tests/📨️browser-frame-transport/🟦️.ts` ("carries the resolved shell brand row on
the boot descriptor") covering the meta resolution, the override precedence, the serve's injection and
the native env twins. The existing predicate law in
`🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs` was updated for the new arity and given two replay
cases.

**Not ported** (and why): `ShellBrand.logoSvg`/`faviconIcoPath`/`assetsDir`/`distDir`/`cnameHost` are
build- and DOM-chrome facts with no wgpu surface; `ShellBrand.introduction`/`tutorials` (a brand-owned
tour REPLACING the app's) is a content payload, not a boot fact, and belongs with the tour lane —
noted as a hand-off below. `locks`/`defaults` were already carried.

---

## Item 9 — arg-carrying Plugin/App/Mode palette commands (audit row 20, P1) — **ALREADY CLOSED; audit row stale**

The audit quotes `command_search_items` self-documenting a dead route. That comment is the **🩸️
history note** of the fix, not a live disclaimer: it reads "This *used to* SKIP every arg-carrying
Plugin/App/Mode-scope command on the premise that … That premise is stale".

| | |
|---|---|
| React source of truth | `searchItems` lists every `resolvedCommands` entry and executes via `handleAction`. |
| wgpu | `command_search_items` emits three shapes — zero-arg → `command:`/`os-command:`; one `Select` arg → one row per option, argument bound; anything else → React's `…`-suffixed row → `command-form:`. All three have live arms in `activate_search_item` (`os-command:` → `apply_os_command`, `command:` → `dispatch_command`, `command-form:` → `reveal_dock_tab` + `expanded_command_id`). |
| Pre-existing laws | `🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs` — `the_palette_lists_and_executes_every_in_palette_command`, `os_commands_keep_their_local_per_option_rows`, `picking_an_arg_carrying_command_opens_its_form`. |

**No source change.** New law `every_palette_command_shape_has_a_live_route` pins the two halves
TOGETHER: the three prefixes are routed in `activate_search_item`'s source, and every row
`command_search_items` actually emits carries one of exactly those three shapes — so a new shape
cannot be listed without a route (a live-looking row that silently no-ops), nor a route dropped from
under a listed row.

---

## Files touched

- `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` (⌨️HostPlatform door; `WgpuBootBrand` + descriptor field + env seeds + `boot_brand()`)
- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`command_host_platform`; named-layout store + accessors + `shell.layout.` resolution + two persist calls; `introduction_seen_key` + the seen lane + `should_auto_start_introduction`; test-mod registration)
- `🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts` (brand row, meta names, resolution)
- `🎯️targets/🧊️wgpu/🌐️server/🟦️.ts` (three brand meta tags)
- `🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` (brand window title)
- `🧱️elements/🐚️Shell/🧪️tests/🚪️wgpu-host-door-remainder/🦀️.rs` (**new**, six laws)
- `🧱️elements/🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs` (predicate arity + two replay cases)
- `🧪️tests/🧭️boot-axis-parity/🦀️.rs` (`WgpuBootBrand` in the nested-twin law)
- `🧪️tests/📨️browser-frame-transport/🟦️.ts` (brand-row boot-seam law)

## Gates

_(filled in below from `🗑️generated/w15f-*.txt`)_

## What I did NOT run

- **No activation, no serve, no probe, no live boot.** W14d owns puzzle3d live verification; the
  packet forbade activation. Nothing below is live-proven.
- The brand path is **not** live-proven end to end: no serve was started with `SEMIO_BRAND_*` set, so
  the meta → descriptor → `boot_brand()` hop is proven by unit law on both sides of the seam, not by a
  branded boot on `aggregator` (6023/6123).
- `windowPanes` was not probed in a browser; the absence claim is a source claim about both renderers.
- The React side of item 6 was established by repo-wide grep over `.ts`/`.tsx`, not by driving a real
  OS file drop onto a React canvas.

## Hand-offs for W14d

1. **Item 2 is the highest-value live check**: boot the wgpu shell on macOS Chrome, open ⌘K and read
   the **Toggle Full Screen** row's description. It must now read the `⌃⌘F` chord, not `F11`. Before
   this packet it read `F11` on every machine. Any app-declared Mac-scoped keybinding is the same
   check.
2. **Item 8, branded playground**: on `aggregator` (React 6023 / wgpu 6123), the tour's persisted flag
   must now be `ui.introduction.seen.<brandId>:<appId>` on both renderers. That needs the serve to
   export `SEMIO_BRAND_*` for the brand row — today only `SEMIO_BRAND` is exported by the dev
   pipeline, so on a live branded serve `ephemeral`/`replayIntroductionOnLoad` will still read
   `false` until the dev serve projects them (the wgpu serve reads the env; nothing sets it yet).
   **The seen KEY is already correct without that**, because it needs only `brandId`.
3. **Item 7**: save a layout in the Layout panel, reload, and confirm it is still listed and still
   applies. The panel ROWS are W15c's (`build_display_layout_ui`, `named_layout_rows`); the store
   under them is this packet's.
4. **Item 5/9 need no live check** beyond what the journey already does — `chord-undo`/`chord-redo`
   steps and a ⌘K arg-carrying command row.
5. A brand-owned `introduction`/`tutorials` (React's `brand.introduction ?? session.app.introduction`)
   is **still** unported — it is a content payload, not a boot fact. Worth its own packet if a branded
   playground's tour text is compared.

## Coordination

`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` was being edited live by W15c (Display panel bodies)
throughout. Every edit here was surgical and additive; nothing of theirs was reverted. Two concrete
interlocks:

- W15c landed `saveCurrentLayout`/`applyNamedLayout`/`deleteUserLayout` and the Display layout body;
  this packet added the persistence under them (`persist_named_layouts` after both mutations) and the
  boot-time restore. Their `current_named_layout`/`apply_named_layout` helpers and the
  `NamedLayout`/`UiDriverChrome` imports were mid-flight at my first check round — my own helpers use
  the fully-qualified `ui_wgpu::wgpu::NamedLayout` so they do not depend on, or collide with, whatever
  import W15c adds.
- The `⌨️HostPlatform` region in `🧊️renderer/🦀️.rs` is W7a's; this packet extended it rather than
  duplicating the rule, so there is still exactly one platform predicate pair in the crate.
