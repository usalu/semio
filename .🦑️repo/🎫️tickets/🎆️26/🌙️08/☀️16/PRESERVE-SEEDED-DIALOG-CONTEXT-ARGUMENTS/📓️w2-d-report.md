# W2-D report — wgpu shell: identity, auto-bind, directory relay, OS commands, presence, routing

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` — the
  bulk of the lane, all within the leased identity/binding/routing/presence/OS-command regions. New
  module-level `//#region 🔖️IdentityPure` (before `ShellTypes`): `mint_shell_session_id`,
  `resolve_identity_env` (native: `IdentityEnv::from_process_env`; wasm32: reads the
  `semioWgpuSetHubEnv`-fed `BOOT_HUB_ENV` thread_local), `shell_actor`, `default_persistence_bindings`,
  `directory_command_from_action`, `fold_directory_events_action`, `open_artifact_relay_target` (+
  `OpenArtifactRelayTarget`), `presence_peer_rows_for_surface`, `S_SPACE_INDEX_DOCUMENT_SCHEMA`/
  `S_SPACE_INDEX_DOCUMENT_ID`/`space_index_dialect`/`find_dialect_app` — all pure/free functions, all
  native-only (`#[cfg(not(target_arch = "wasm32"))]`) except the wasm32 env-intake pair. New
  `ShellState` fields (native-only, in a new `//#region 🔖️Identity` inside the struct): `identity`,
  `identity_offline`, `identity_env`, `shell_session_id`, `identity_bootstrap_rx`, `directory_client`,
  `directory_events_rx`, `pending_directory_commands`, `presence_peers`, `presence_surface` —
  initialized in `ShellState::new`. New `impl ShellState` region `//#region 🔖️DirectoryAndIdentity`:
  `handle_replay_shell_command`, `handle_open_artifact_relay`, `dispatch_directory_command`,
  `queue_pending_directory_command`, `flush_pending_directory_commands`,
  `dispatch_directory_event_batch`, `bootstrap_identity`, `poll_identity_bootstrap`,
  `open_directory_stream`, `pump_directory_events`. New `open_document`, `default_bindings_for_current_session`,
  `switch_to_app`, `current_shell_actor` methods. Edited: `boot()` (calls `bootstrap_identity()`),
  `pump_sync_events` (drains directory events + presence capture), `publish_presence_heartbeat`/
  `attach_sync_backbone` (real actor id instead of `wgpu-{instance_id}`), `detach_sync_backbone_internal`
  (clears presence on detach), the two `HostEffect` match loops inside `dispatch_action`/`dispatch_command`
  (new `ReplayShellCommand` arm), `apply_shell_uri` (studio vs bare `/spaces/{id}` routing), `render_footer`
  (presence bar call). New free function `render_presence_bar` (footer chrome). New test module
  `//#region 🧪️IdentityDirectoryPresenceTests` (`identity_directory_presence_tests`, 9 tests).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml` —
  native-only deps: `semio-framework-os-kernel` gained the `ureq` feature (alongside the pre-existing
  `sync`) so `os_directory::client::native::NativeDirectoryTransport` compiles; `tokio` gained `rt`/`net`/`time`
  (explicit rather than relying on Cargo's feature unification from the kernel crate's own `sync`
  feature) for this crate's own directory-stream background thread's current-thread runtime.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🟦️boot.ts` —
  new `resolveBootHubEnv()` (mirrors `resolveBootAppRole`'s defensive `VITE_S_*`-then-URL-param read,
  `?hub=`/`?user=`/`?dataDir=`) and a guarded `bindings.semioWgpuSetHubEnv(...)` call right before
  `semioWgpuMount`, mirroring the existing `semioWgpuSetAppRole` call immediately above it.

**Not touched**: `📦️bin.rs` (native identity intake needs zero plumbing there — `IdentityEnv::from_process_env`
reads `S_HUB_URL`/`S_USER`/`S_DATA_DIR` straight off the process env, which `📜️script.ts`'s `trunkEnv()`/
`nativeEnv` already full-spread into the launched native binary per lane 1-F's own report — no new wiring
needed); `ProgramBridge/🧊️component.rs` (see "What is NOT done" below — its only `"actor": "local"`
hardcodes are wasm32-only browser-plugin-exchange code, out of this lane's native-only identity scope);
`📦️glue.rs` (deliberately avoided — not in this lane's lease; the wasm32 `semioWgpuSetHubEnv` export
lives inside `Shell/🧊️component.rs` instead, since `#[wasm_bindgen]` doesn't care about module nesting).

## Task items — status

1. **Identity** — done (native). `IdentityEnv::from_process_env()` (native) / the `BOOT_HUB_ENV`
   thread_local fed by `🟦️boot.ts`'s new `semioWgpuSetHubEnv` call (wasm32, env-intake plumbed but
   mint/restore itself stays native-only, see "What is NOT done"). `bootstrap_identity()` spawns a
   background OS thread (`mint_or_restore` + `pollster::block_on`, no tokio needed since `ureq`'s
   native transport is its own blocking-thread-per-call) and reports back through
   `identity_bootstrap_rx`, polled non-blockingly every frame by `pump_directory_events` (itself
   folded into the existing `pump_sync_events` — glue.rs's frame loop already calls that every tick,
   so no second call site was needed outside this lease). Hub unreachable ⇒ `mint_or_restore` degrades
   to the cached identity + `Offline` status (1-D's own logic) — surfaced as `identity_offline`, never
   blocks, never panics. No hub env ⇒ `bootstrap_identity` returns immediately, unchanged local-only
   behaviour. `ArtifactActorConfig.actor` now threads `shell_actor(identity, session_id, instance_id)`
   (contract §C0 grammar when identity exists, the old `wgpu-{instance_id}` default otherwise) at
   every call site that used to hardcode it (`attach_sync_backbone`, `open_document`,
   `publish_presence_heartbeat`'s `PresencePeer.actor`).
2. **Auto-binding** — done (native). `open_document`'s `bindings` param is caller-supplied (never
   silently defaulted internally, keeping its signature explicit) but every call site computes them via
   `default_bindings_for_current_session()` → the pure `default_persistence_bindings(identity, space_id,
   data_dir, surface)`: `[Hub, Folder]` with identity+space, `[Folder]` with a data dir but no identity,
   `[]` otherwise — unit-tested (see Verify). `attach_sync_backbone` (the manual `remote://` sync-card
   override) is untouched and still always passes an explicit array from `parse_persistence_binding`,
   matching the brief's "keep the manual sync card as an override." Confirmed the `PersistenceBinding::Hub.surface`
   field lane 1-D added is wired through end-to-end: `default_persistence_bindings` sets it from
   `surface_app_id(dialect, role)` (`semio_framework::manifest::surface_app_id` — already `pub`, reused
   rather than reimplemented) whenever identity+space are both present.
3. **Directory lane** — done (native). `open_directory_stream` opens `/directory/ws` once per shell (a
   dedicated background thread with its own current-thread tokio runtime — `tokio-tungstenite`'s WS
   transport needs a real reactor, `pollster::block_on` alone doesn't provide one; mirrors
   `🏪️store/🔄️sync`'s own native-actor precedent per `📓️w1-d-report.md`), auto-reconnecting per
   `DirectoryStream`'s own backoff (the loop just sleeps `after_ms` between dials, as its doc asks).
   `pump_directory_events` drains the channel and folds `DirectoryEvent` batches into whichever session
   is CURRENTLY mounted via `foldDirectoryEvents` (`fold_directory_events_action` — same action
   name/payload shape as the React shell's `dispatchDirectoryEventBatch`, verified against
   `📓️w2-c-report.md`). The seven `os.directory.*` OS commands relay through `directory_command_from_action`
   (verb-for-verb match against React's `directoryCommandFromAction`, including the `share-link` →
   `create-invite` sugar) → `DirectoryClient.command()`, with a bounded (64) in-memory offline queue
   (`pending_directory_commands`) flushed opportunistically every frame once a client exists — dropped
   with a warning (not queued) when no identity is signed in at all, matching the React shell's own
   distinction between "not signed in" (drop) and "signed in but hub down" (queue).
4. **OS commands / opening relay** — done (native), same caveat as 2-C: the seven `os.directory.*`
   relays are wired end-to-end. `os.open-artifact`/`os.open-artifact-with` intercepted via the SAME
   `HostEffect::ReplayShellCommand` arm (added to both `dispatch_action`'s and `dispatch_command`'s
   effect loops — contract §C6 calls it "a surface command," which in this shell's architecture can
   originate from either boundary): `handle_open_artifact_relay` parses `documentId`/`spaceId` off the
   relay args and calls `open_document` with the session's own default bindings, pinning `open_space_id`
   first when `spaceId` rides along. Same schema-mapping limitation 2-C documented (no general
   dialect→schema formula; `s.space` is the one mapping this lane knows, `documentId` itself is the
   best-effort fallback) — this shell had zero `ReplayShellCommand` handler at all before this lane
   (verified: no match arm anywhere pre-edit), the same pre-existing gap 2-C found on the React side.
5. **Presence** — done (native). `ArtifactEvent::Presence { peers }` (previously dropped at the old
   `🧊️component.rs:~1968` with a "documented follow-up" comment) now populates a shell-LOCAL
   `presence_peers: Vec<PresencePeer>` field — the shared kernel `ViewModel` is untouched, per the
   brief's explicit instruction. `presence_surface` tracks which canonical surface (`surface_app_id`)
   the currently attached document's hub binding used; `presence_peer_rows_for_surface` is the pure
   filter that only ever returns rows when the attached surface matches the surface being rendered (so
   a stale roster from a just-closed document/surface can never leak into whatever opens next — both
   fields are cleared on every detach). Rendered by a new `render_presence_bar` free function in the
   footer chrome, right-aligned, using the SAME `ChromeGroupItem`/`render_chrome_group`/
   `measure_chrome_group_item` immediate-mode primitives `render_footer_utility_nodes` already draws
   with (id `s-presence-peers` on the roster it walks from `ui_wgpu::wgpu::build_presence_bar`, per-peer
   `peer:<actor>` folded into a registered, non-actionable `HitTarget` for hit-testing/e2e discoverability).
   **Design note**: the generic `render_ui_node` pipeline plugin surfaces use needs `&mut GpuContext` —
   this footer's own immediate-mode callers (`render_footer(&self, ...)`) don't carry one, and
   threading it through would touch `render_footer`'s single call site and beyond, outside this lane's
   lease's blast radius for a presence-only feature. `render_presence_bar` is the honest, self-contained
   substitute: it still consumes `build_presence_bar`'s own `UiNode` tree as its source of truth for
   ids/labels, just paints it with the plainer primitives already proven safe in this exact footer.
6. **Routing** — done (native), same s.space-not-compiling caveat 2-C hit. `apply_shell_uri` now
   distinguishes `/spaces/{id}/studio` (the ONLY behaviour this route had before this lane — kept
   byte-identical) from a bare `/spaces/{id}`, which now resolves the `s.space` app via
   `find_dialect_app` against `s.space.space@1/*` (editor, falling back to viewer) and opens its index
   document (`open_document(S_SPACE_INDEX_DOCUMENT_ID, S_SPACE_INDEX_DOCUMENT_SCHEMA, ...)`) with the
   session's own default bindings — mirrors `📓️w2-c-report.md`'s `applyShellUri` decision exactly.
   **Could not observe end-to-end**: `find_dialect_app` returns `None` today because lane 1-E's
   `s.space` artifact crate does not currently compile (`📓️w1-e-report.md`'s own documented blocker,
   `WorkflowMutation: SemanticMutation` not satisfied — a framework/peer gap, not this lane's), logged
   via `eprintln!("[DEBUG] ...")` rather than silently failing. The routing/resolution code itself is
   dialect-id-driven and activates the moment that crate links, with zero further shell-side change —
   same posture 2-C's own report takes.

## Verify — unit tests (5 required + verb coverage)

New `identity_directory_presence_tests` module, 9 tests, one file per Verify bullet:
- `shell_actor_uses_contract_grammar_when_identity_present_else_local_default` — **actor id shape**.
- `default_bindings_are_hub_plus_folder_with_identity_and_space` /
  `default_bindings_are_folder_only_without_identity` / `default_bindings_are_empty_without_space_or_data_dir`
  — **default-bindings decision with and without identity**.
- `directory_command_from_action_covers_every_frozen_verb` — all 7 verbs including the `share-link`
  sugar (extra coverage beyond the required list, mirroring 2-C's own test).
- `fold_directory_events_action_reaches_the_controller_with_the_events_payload` — **the directory-event
  fold reaching a session**.
- `open_artifact_relay_target_parses_document_and_space_ids` — **the `os.open-artifact{documentId}`
  path**.
- `presence_rows_are_scoped_to_the_attached_surface` — **presence roster filtering by surface**.

**I could not run these tests, or `cargo check`/`cargo test` for this crate at all.** See "Blocker"
below — every attempt aborted before rustc ever reached `semio-framework-os-renderer-wgpu` (or any of
its own dependents), because a HARD, non-optional dependency (`semio-s-plugin-puzzle`) fails to
compile for reasons entirely unrelated to this lane. I did a careful manual review of the full diff
against every type/signature I read from source (`store_sync::PersistenceBinding`,
`os_directory::{schema,client,identity}`, `ui_wgpu::wgpu::{PresencePeerRow,PresenceRole,build_presence_bar,
ChromeGroupItem,render_chrome_group,measure_chrome_group_item}`, `semio_framework::{ArtifactDialect,
manifest::{AppRole,surface_app_id}}`), cross-checking field names, generic bounds, `Send`/lifetime
implications of the two background threads, and the `#[cfg]` gating on every new item — documented
inline in `🧪️2-d-cargo-check-1.txt`'s companion notes below. This is NOT a substitute for a real green
compile; treat the code as **unverified** until the blocker clears.

## Blocker — `semio-s-plugin-puzzle` fails to compile, unrelated to this lane

`cargo check -p semio-framework-os-renderer-wgpu` (and `--features native-bin`) both abort with
**exactly 4 errors, all inside `semio-s-plugin-puzzle`**, before rustc ever compiles anything in the
wgpu crate itself (confirmed via `grep -c "^error"` on both full logs — 4, zero mentioning `Shell/`,
`ProgramBridge/`, `bin.rs`, or `boot.ts`):

```
error[E0277]: the trait bound `Puzzle2dMutation: SemanticMutation<Puzzle2dPlaySnapshot>` is not satisfied
error[E0277]: the trait bound `Puzzle3dMutation: SemanticMutation<Puzzle3dPlaySnapshot>` is not satisfied
error[E0277]: the trait bound `Puzzle5dMutation: SemanticMutation<Puzzle5dPlaySnapshot>` is not satisfied
error: could not compile `semio-s-plugin-puzzle` (lib) due to 3 previous errors; 266 warnings emitted
```

`puzzle` is a hard (non-optional, no feature gate) dependency of `semio-framework-os-renderer-wgpu`
(`Cargo.toml`: `puzzle = { path = "...", package = "semio-s-plugin-puzzle" }`), so there is no way to
check this crate without it compiling first. Attribution: `git status --porcelain -- "✏️s/🔌️plugins/🧩️puzzle/"`
shows 22 uncommitted `M` files under the puzzle plugin's mutation-diff leaves, all touching `dsl::Mutations`-
derived types — squarely `🗣️dsl/**`/mutation-derive infrastructure, which `📌️important.md`'s "What we
never touch" list (and `26/08/16/MUTATION-OUTCOMES-…`'s own ownership table) assigns to the
`MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` ticket, not this one. `stat -f "%Sm"` on
the actual failing file (`🧬️mutations/🦀️component.rs`) shows it last touched **2026-08-16 17:35:40**
(~8.5h before this check, quiet — not a live in-progress edit I should wait out) while still leaving
the dependent `puzzle` crate broken; not mine to fix (outside this lane's lease and this ticket's
ownership entirely). Retried the check twice more (`🧪️2-d-cargo-check-1.txt`,
`🧪️2-d-cargo-check-2.txt`) ~15 minutes apart — identical 4 errors both times, no change.

Logs: `🧪️2-d-cargo-check-1.txt`, `🧪️2-d-cargo-check-2.txt`, `🧪️2-d-cargo-check-native-bin.txt` (all
three identical in root cause). `cargo check -p semio-framework-os-kernel --features ureq,sync`
(the ONE piece of this lane's Cargo.toml change that lives in a crate NOT blocked by puzzle) **is
GREEN** — 18 pre-existing warnings, 0 errors — confirming the `ureq`+`sync` feature combination itself
(and every type I import from `os_directory::{client,identity}`) is sound; this is the strongest signal
available that my Cargo.toml edit is correct even though the wgpu crate itself couldn't be checked.

## What is NOT done

- **No compiler confirmation for `Shell/🧊️component.rs`, its new tests, or the wasm32 `semioWgpuSetHubEnv`
  export** — blocked entirely by the above. This is real, unmitigated risk: manual review does not
  replace `rustc`. Re-run `cargo check -p semio-framework-os-renderer-wgpu` and
  `cargo test -p semio-framework-os-renderer-wgpu --lib identity_directory_presence` (or the crate's own
  test filter convention) the moment `semio-s-plugin-puzzle` is green again.
- **Browser (wasm32) identity/directory is env-intake only** — `🟦️boot.ts` reads `VITE_S_HUB_URL`/
  `VITE_S_USER`/`VITE_S_DATA_DIR` (defensive; this target is Trunk-served, not Vite-bundled, so these
  resolve in practice only via the `?hub=`/`?user=`/`?dataDir=` URL-param fallback) and calls the new
  `semioWgpuSetHubEnv` export, but `mint_or_restore`/`DirectoryClient`/the directory stream are ALL
  native-only (`#[cfg(not(target_arch = "wasm32"))]`), matching this shell's pre-existing architecture
  (`document_host`/`sync_channel`/`sync_status` are native-only too — the browser wgpu build already
  relies on "host-shim" relaying per `attach_sync_backbone`'s own wasm32 branch, not a native
  `ArtifactHost`). A real browser identity path needs `os_directory::client::browser::BrowserDirectoryTransport`
  (already compiled-and-typechecked by lane 1-D, never runtime-tested) wired the same way the native
  path is here — a documented, scoped follow-up, not attempted this wave.
- **`ProgramBridge/🧊️component.rs`'s two `"actor": "local"` hardcodes (lines ~486/~498) are untouched.**
  Both are inside `#[cfg(target_arch = "wasm32")]`-gated functions (`handle_action_js`/`handle_command_js`,
  the browser JS-plugin-exchange path) — the NATIVE plugin-exchange path (`mod wasm_program_exchange`,
  `#[cfg(not(target_arch = "wasm32"))]`) carries no per-invocation "actor" JSON field at all (it uses
  `AppCommand::Command { seq, command, view_state }`, a WIT-wire type with no actor slot), so there was
  nothing to thread there. Updating the wasm32-only "local" strings without a real wasm32 identity to
  supply them would have been cosmetic and misleading, so left as-is — consistent with the browser-wgpu
  scoping decision above.
- **No `data-ui-path`-equivalent DOM attribute for the wgpu footer's presence chips** — this shell has
  no DOM at all; `peer:<actor>` is embedded in the registered `HitTarget.control_id`
  (`"framework.presence.peer:<actor>"`) as the closest reachable analog, not a literal attribute (per
  2-F's own report, `data-ui-path` only exists on the generic `Interpreter`'s rendered nodes, which
  `render_presence_bar` deliberately bypasses — see task 5's design note).
- **`ArtifactSyncStatus`/pending-command-count chrome for the directory lane's offline queue** — tracked
  in `pending_directory_commands.len()` but not surfaced by any UI this lane owns, same as 2-C's own
  "not yet rendered by any chrome" note for `directoryPendingCommands` — left for 3-A/whoever builds the
  "row shows pending" affordance.

## sharedFileRequest

None. Every edit landed inside this lane's own lease (`Shell/🧊️component.rs`'s identity/binding/routing/
presence/OS-command regions, `ProgramBridge/🧊️component.rs` — read-only this wave, `bin.rs` — untouched,
`🟦️boot.ts`). `📌️important.md` was re-read but needed no edit (I don't own it and made no foreign-file
touch that would require a coordinator notice).
