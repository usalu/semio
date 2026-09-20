# O3 — the wasm32 browser wgpu artifact-open relay

Slice O3 (G10 §C slice N2, §D Outcome 1 row "The wasm32 browser wgpu shell can lazy-install and open a
foreign-kind artifact"). Predecessor context: `📓️o1-multi-plugin-hub.md` §3/§6, `📓️o2-activation-follow-ups.md`
§0.5 decision 5 and §5 gap 3, `📓️g8-wgpu-parity-spec.md`.

Main file: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
(26 537 lines at slice start). Sibling slice WG6 edits the same file for hub sign-in/spaces chrome —
every edit here was made against a fresh read and kept to six seams. The `git diff --stat` on that file
(2 321 lines changed) is overwhelmingly WG6's; O3's own hunks are the six listed in §2.

## 0. Starting state (measured)

No predecessor O3 work existed: no `📓️o3-*.md`, no `🗑️generated/o3-*` captures.

What the tree actually had, read at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`:

| symbol | line (before) | gate |
|---|---|---|
| `OpenArtifactRelayTarget` | 528 | `cfg(not(wasm32))` |
| `open_artifact_relay_target` | 541 | `cfg(not(wasm32))` |
| `find_dialect_app` | 627 | `cfg(not(wasm32))` |
| dispatcher arm for `os.open-artifact{,-with}` | 9735 | `cfg(not(wasm32))` |
| `handle_open_artifact_relay` | 10011 | `cfg(not(wasm32))` |
| `resolve_activation_owner_app` | 10771 | `cfg(not(wasm32))` |
| `switch_to_app` | 10787 | `cfg(not(wasm32))` |
| `open_document` | 8913 | `cfg(not(wasm32))` |
| `default_bindings_for_current_session` | 8945 | `cfg(not(wasm32))` |
| `document_host: ArtifactHost` (field) | 3083 | `cfg(not(wasm32))` |

`install_plugin` / `cancel_plugin_install` / `ShellPluginInstall` are already target-neutral (O2), and
`crate::program_bridge::resolve_artifact_kind_activation_owner` + `PLUGIN_ARTIFACT_KIND_ACTIVATIONS` are
generated and ungated. So the *lazy install* door O2 built had no caller on wasm32, which is exactly
what N2 describes.

### Why the document half is a different packet (measured, not assumed)

`open_document` needs `ArtifactHost`, imported at `:37` under `#[cfg(not(target_arch = "wasm32"))] use
store_sync::sync::{…}`. That module is mounted in the kernel crate at
`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:280` behind
`#[cfg(all(feature = "sync", not(all(target_arch = "wasm32", target_env = "p2"))))]` — i.e. it *is*
architecturally allowed on `wasm32-unknown-unknown`, and `🏪️store/🔄️sync/🦀️.rs:3430` really does carry a
`mod wasm_actor` with a `web_sys::WebSocket` hub transport used by `spawn_actor` at `:3960`. Two things
block it today:

1. The renderer crate declares `semio-framework-os-kernel` with `features = ["sync", "ureq"]` **only**
   under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`
   (`🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml`); the wasm32 build gets the kernel without `sync`,
   so `store_sync::sync` does not exist there at all.
2. The kernel's `sync` feature is `["dep:tokio-tungstenite", "dep:semio-framework-os-services",
   "dep:ureq", "tokio/rt", "tokio/time"]`. The first two are already `cfg(not(wasm32))`-gated optional
   deps, so they no-op on wasm32 — but `ureq` is declared in the *unconditional* `[dependencies]`
   table, so enabling `sync` for wasm32 would pull a native-socket HTTP crate into a browser build.
   Its only use sites are `📇️directory/🔌️client/🦀️.rs:1446+`, all under
   `cfg(all(feature = "ureq", feature = "sync", not(target_arch = "wasm32")))`, so the root fix is one
   manifest move (`ureq` → the `not(wasm32)` target table), not a shim.

Beyond the manifest, the shell-side surface is large: `open_document` also reaches
`checkpoint_before_detach` (`:8728`), `detach_sync_backbone_internal` (`:8375`), `current_shell_actor`
(`:8595`), `refresh_history_snapshot` (`:8609`), `bind_wgpu_document_socket_surface` (`:680`) and the
`sync_channel`/`sync_status`/`presence_*` fields (13 `sync_channel` sites) — all native-gated, plus the
event-drain loop that has to exist or the channel would be opened and never read. That is the packet
handed on in §5, with the exact entry points above.

## 1. Design

**Decision 1 — the relay has two halves and only the second one is native.** Resolving the owner of an
artifact kind (generated table), installing it on demand (`install_plugin`, target-neutral since O2) and
switching the session to its app (`create_app` + `refresh_ui`, both target-neutral) is pure catalog +
program-bridge work the browser build already links. Binding the newly opened session to a *hub
document* is the only part that needs the `ArtifactHost` backbone. Splitting there — rather than leaving
the whole relay native — is what makes "lazy-install and open a foreign-kind artifact" real in the
browser, which is N2's own acceptance sentence.

**Decision 2 — the browser refuses the document half out loud, it does not drop it.** A relay that
carries `documentId`/`schema` on wasm32 now opens the app and then raises the shell's own
`show_transient_notice` banner with code `open-artifact.browser-document`, EN + DE through the existing
`shell_chrome_string` table. Silently ignoring the caller's `documentId` would be the stub; an honest,
localized terminal is the same posture `attach_backbone` already keeps ("carries the honest 'not
implemented yet' error for the whole mechanism").

**Decision 3 — progress and cancel come from O2's state machine, not a second notion.** The browser
relay's install runs through `ShellState::install_plugin`, so it advances `ShellPluginInstall`'s
`Resolving → Loading` phases and checks the retained `CancelToken` at every step boundary; the wgpu
chrome's `ShellChromeFramePhase::PluginInstall` band (O2 §3) paints it and its cancel control fires
`cancel_plugin_install`, which on wasm32 also withdraws the frame Worker's own request
(`crate::program_bridge::cancel_js_plugin_install`). No new progress surface was added.

**Decision 4 — `directory_home` stays native.** `switch_to_app`'s only native dependency was handing the
current view state back to the retained Home projection, a native-only field. That block is now the one
`cfg(not(wasm32))` statement inside an otherwise target-neutral function, instead of the whole function
being native.

## 2. Fixes (file:line — post-edit line numbers)

All in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
unless stated.

1. `:534` `OpenArtifactRelayTarget` — `cfg(not(wasm32))` removed.
2. `:545` `open_artifact_relay_target` — `cfg(not(wasm32))` removed. The whole schema-first parse
   contract (role normalization, all-or-nothing coordinate pairs, surface/role agreement) is now
   compiled for the browser.
3. `:630` `find_dialect_app` — `cfg(not(wasm32))` removed (a pure manifest scan).
4. `:9764` the `handle_replay_shell_command` arm for `os.open-artifact` / `os.open-artifact-with` —
   `cfg(not(wasm32))` removed, and the stale comment above it ("the ONE half of this funnel that is
   still native-only") deleted.
5. `:10048` `handle_open_artifact_relay` — `cfg(not(wasm32))` removed, docstring extended with the
   two-half rule; `:10091-10100` the document half split into a native arm (unchanged
   `default_bindings_for_current_session` + `open_document`) and a wasm32 arm that raises the localized
   `open-artifact.browser-document` notice.
6. `:10821` `resolve_activation_owner_app` — `cfg(not(wasm32))` removed.
7. `:10838` `switch_to_app` — `cfg(not(wasm32))` removed; the retained-Home hand-back
   (`:10848-10853`) carries the gate instead.
8. `:24696-24697` two new `shell_chrome_string` rows, EN + DE, in the file's own "wgpu-only additions"
   block next to `plugin.install.*`.

## 3. Tests and checks

New: 6 laws appended to
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs`
(73 → 152 lines; the file is mounted at `🎯️targets/🧊️wgpu/🦀️.rs:25717` as `mod plugin_install_tests`).
Unlike the four pre-existing install laws, five of the six are **not** `cfg(not(wasm32))`-gated, because
the code they exercise now compiles on both targets:

| law | asserts |
|---|---|
| `an_app_only_relay_parses_into_a_dialect_with_no_document` | the shape a browser playground sends parses to a dialect + default `Editor` role, no document coordinate |
| `a_parsed_relay_kind_resolves_to_its_declared_owner` | parsed kind → `resolve_artifact_kind_activation_owner` = `cad`, and the generated table names more than one owner |
| `half_a_coordinate_pair_is_refused` | `opening.partial-app-ref`, `opening.partial-document-ref`, `opening.invalid-artifact-ref`, `opening.invalid-args` |
| `the_explicit_spelling_requires_an_app_ref` | `os.open-artifact-with` refuses an app-less relay; the explicit form round-trips plugin/app/role |
| `a_surface_ref_must_agree_with_every_other_role_spelling` | `opening.role-mismatch`, `opening.app-mismatch`, and the surface suffix normalized off the stored coordinate |
| `the_browser_document_refusal_is_localized` | the new chrome key resolves to distinct EN and DE text (an untranslated key would paint the key itself) |

Commands (one cargo at a time, foreground, machine load ≈ 145 at launch, 42 GiB free):

```
CARGO_PROFILE_WASM_DEV_DEBUG=false \
  cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown
cargo test  -p semio-framework-os-renderer-wgpu --lib plugin_install_tests
```

Captures: `🗑️generated/o3-wasm32-check.txt`, `🗑️generated/o3-native-relay-tests.txt`.

**Attempt 1 (01:33) — blocked upstream by a peer, not by this slice.** The wasm32 check failed after
2 m 25 s with 1 error in `semio-framework-trace`, a crate this one depends on unconditionally:

```
error[E0502]: cannot borrow `counters` as immutable because it is also borrowed as mutable
  --> 🧰️framework/🔨️modules/⏱️trace/📝️record/🦀️.rs:444:25
error: could not compile `semio-framework-trace` (lib) due to 1 previous error
```

That is slice OB1's file (hub observability on the framework `⏱️trace` module) caught mid-edit. Per
preamble rule 3 nothing was "fixed" here; re-reading the file at 01:44 showed the peer had already
landed the fix at 01:37 (`let has_capacity = counters.len() < COUNTER_EVENT_CAPACITY;` hoisted above the
`match`), so the blockage was ~4 minutes of peer churn. `🗑️generated/o3-wasm32-check.txt` holds the
failed capture.

**Static verification that needed no build:** the wgpu install band O2 added
(`ShellChromeFramePhase::PluginInstall` at `:20254`, `plugin_install_banner_text` `:18571`,
`plugin_install_tone` `:18588`, the cancel control at `:22941`) carries **no** `cfg` gate, so the
progress + cancel surface the brief asks for is already painted on wasm32 — this slice gives it its
first browser caller rather than adding a second progress notion.

**Attempt 2 / native run: (filling)**

## 4. Runtime evidence

(filling)

## 5. Honest gaps

1. **The document half is still native-only.** The browser now refuses it out loud instead of dropping
   it. The follow-up packet is specified in §0: (a) move `ureq` to the kernel's
   `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` table so the `sync` feature is selectable
   on wasm32; (b) add a wasm32 `semio-framework-os-kernel { features = ["sync"] }` entry to the renderer
   crate; (c) un-gate `document_host`/`open_document`/`default_bindings_for_current_session` plus the
   six helpers and the `sync_channel` drain named in §0. `🏪️store/🔄️sync/🦀️.rs`'s own module doc already
   claims a browser actor exists — whether it *compiles* for `wasm32-unknown-unknown` was not measured
   in this slice.
2. **Browser persistence bindings are unspecified.** `default_bindings_for_current_session` reads
   `identity_env.data_dir`, which has no browser meaning; a browser binding set is hub-only by
   construction. That decision belongs with the packet above, not to a `cfg` flip.
3. (filling — runtime proof status)

## 6. Files changed

```
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs          (8 hunks, §2)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs (+79 lines, 6 laws)
```

No `Cargo.toml`, no generated file, no `🗑️generated` sweep, no git-modifying command.
