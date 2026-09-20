# WG6 — wgpu hub sign-in, spaces, workspace, connection pill

Slice WG6 (fleet 4, session 5, 2026-09-20). Scope: G8 work items **WG-6** (hub sign-in / spaces /
workspace on the wgpu renderer) and **WG-5** (footer hub-connection pill), unblocked by AU3's live
`POST /auth/sessions` route (G8's "blocked" note is stale).

Status: **(in progress)** — sections are filled as each item lands.

## 1. Starting state (measured)

| claim | how measured | result |
|---|---|---|
| no inherited WG6 work | `ls 🗑️generated \| grep wg6`, `ls "$T" \| grep -i wg6` | nothing — this slice started from zero |
| `🔐️HubSignIn` / `🏘️SpaceBrowser` / `🔗️HubConnection` have no wgpu target | `find . -type d -name "🧊️wgpu"` under `🧱️elements` | confirmed: 15 element wgpu targets exist, none of those three |
| G8's "blocked on an unmounted `/auth/sessions`" is stale | `📓️au3-live-sign-in-integration.md` §1 + `🌎️hub/🔐️auth/🦀️.rs:29` `SESSION_MINT_ROUTE` | route was always mounted through a constant; AU3 drove it live (38 checks) |
| **WG-5's footer pill already exists** | `grep -n "s-hub-connection" 🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | a peer landed it: `ShellHubConnectionState` (`:14670-14698`), `hub_connection_state()` (`:8560-8587`), layout (`:20883-20895`), retained paint phase 4 (`:22314-22325`), en+de keys (`:24860-24873`) |
| …but it is **not** U1's fold | read `hub_connection_state` + `📓️u1-…md` §8 | it reads ONE `sync_status`, not `syncStatusByDocumentId`; no max-over-live `peerCount`; `signedOut` does not outrank transport states; the pill is not clickable |
| the wgpu shell has no credential sign-in at all | `🐚️Shell/…/🦀️.rs:9878-9906` (`poll_browser_identity`), `:10194` (native `identity_env`) | identity is an inherited local-bootstrap credential (native) or a same-origin cookie (browser) — there is no bearer, no hub choice, no sign-out |
| the directory CQRS path is real and target-neutral | `📇️directory-door/🦀️.rs` + `🐚️Shell/…/🦀️.rs:9850-9871` | `DirectoryClient<ShellDirectoryTransport>`, `ShellDirectoryCommandQueueV1` (bounded FIFO, byte-identical retry), `dispatch_directory_command` — a new surface must reuse these, not invent a transport |
| the wgpu a11y tree is a projection of the retained `UiTree` | `🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs:48` | so a surface built as `UiNode` gets the ARIA mirror for free; a hand-rolled paint-op sheet (the `🛂️SpaceAdministration` idiom) gets none |

## 2. Design

Three new element targets plus a tight shell hunk, mirroring AU2's four-layer split so the contract
is provable without a renderer and the renderer is provable without a hub.

**a. Pure contract twins, no renderer, no transport.** `🔐️HubSignIn/🎯️targets/🧊️wgpu/🦀️.rs` and
`🏘️SpaceBrowser/🎯️targets/🧊️wgpu/🦀️.rs` are straight Rust ports of `📇️directory/🔐️sign-in/🟦️.ts`
and `📇️directory/🏘️spaces/🟦️.ts` — token parser, origin normalizer, connection book, session
reducer, command builders in canonical field order, invite parsing, closed error classes, en+de text.

**b. The composed lane.** `🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs` owns the workspace state, the
generic `DirectoryTransport`-parameterized `run_hub_sign_in` / `run_hub_session_authority` /
`run_hub_sign_out` / `run_invite_redemption`, the `hub_connection_summary` fold (WG-5's real Rust
twin of `hubConnectionSummaryV1`), and `build_hub_workspace_ui` — a **`UiNode`** tree, not paint ops,
so the DOM-mirror ARIA projection announces the whole surface without a line of accessibility code.

**c. Fixture-shared parity.** The Rust tests read the **same** language-agnostic fixtures the React
twins read (`📇️directory/🔐️sign-in/🔣️.json`, `📇️directory/🏘️spaces/🔣️.json`) through `include_str!`
and drive the Rust functions with them. Same fixture in → same verdict out, enforced on both sides;
a fixture edit breaks whichever renderer drifted.

### Decisions worth defending

- **`UiNode`, not paint ops.** `🛂️SpaceAdministration`'s wgpu target is a flat paint program because
  it predates nothing — it is an overlay sheet. A hub workspace is a *panel*, and the panel path
  (`publish_shell_panel_document` → `panel_ui_records` → retained `UiTree`) is the one that feeds
  the accessibility projection, the retained hit registry and the `framework` action controller. The
  spec's suggestion to copy the `*_paint_ops` shape was written before this slice read
  `♿️accessibility/🦀️.rs:48`; following it would have shipped an inaccessible surface.
- **The token never enters `ShellState`'s serializable half.** The minted capability lives in one
  `HubSessionCapability` field consumed by the transport call and is never written to the prefs
  store, never in a `UiNode`, never in a `data_attributes` value. Only hub *identity* (origin, label,
  kind, last user) is persistable — a Rust test asserts the serialized book contains no token.
- **`retry-after` comes from the body, not the header.** `DirectoryTransport::http` answers
  `HttpResponse { status, body }` with no header map, so the wgpu twin can only read AU1's
  `retryAfterMs` out of the `semio.hub.auth.error/v1` body. Declared divergence, tested both ways:
  the header parser is ported and unit-tested for parity with the fixture, but the live path uses the
  body. Widening the transport trait for one header would have touched every other directory caller.
- **One `DirectoryClient` per hub origin.** Signing into a remote hub rebinds
  `ShellState::directory_client` to a client whose base URL is that origin and whose bearer is the
  minted capability, so every later CQRS command and space read goes to the hub the human chose —
  AU3 §4.4's "the hub is its own origin" defect, avoided at the source on this renderer.
- **No default language.** `hub_sign_in_text(Locale)` is an exhaustive two-arm match over
  `Locale::{En, De}`; adding a locale to the generated enum fails to compile until both columns
  exist, which is the compile-time equivalent of AU2's `registerUiTranslationBundles` type.

## 3. Landed changes (file:line)

### Added — three element targets, co-located per the convention

| file | lines | what |
|---|---:|---|
| `…/🧱️elements/🔐️HubSignIn/🎯️targets/🧊️wgpu/🦀️.rs` | ~600 | routes/bounds, en+de text table (exhaustive `Locale` match), 8 closed denial classes + status table, `retry-after` header and `retryAfterMs` body readers, `session.v1.<32hex>.<64hex>` parser, email/password/device admission, mint-body builder in AU1's field order, `DirectorySessionAuthorityV1` decoder, origin normalizer, local-only connection book (parse/serialize/upsert/remove/select), session state + reducer + `hub_sign_in_form_offered` |
| `…/🧱️elements/🏘️SpaceBrowser/🎯️targets/🧊️wgpu/🦀️.rs` | ~360 | `SpaceRow` projection + total ordering + filter + write/invite authority, member↔presence join, the three `DirectoryCommand` builders, invite token/link/redeem-path parsing, redemption status table with en+de texts, `SpaceBrowserPhase` + `space_browser_rows_usable`, chrome labels en+de |
| `…/🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs` | ~470 | `hub_connection_summary` (WG-5's fold), the four `DirectoryTransport`-generic calls (`run_hub_sign_in`, `run_hub_session_authority`, `run_hub_sign_out`, `run_invite_redemption`), `HubWorkspaceState`, `HubSessionCapability`, and `build_hub_workspace_ui` — the retained `UiNode` tree |
| `…/🔐️HubSignIn/🧪️tests/🔬️wgpu-unit/🦀️.rs` | 24 laws | fixture-driven |
| `…/🏘️SpaceBrowser/🧪️tests/🔬️wgpu-unit/🦀️.rs` | 16 laws | fixture-driven |
| `…/🔗️HubConnection/🧪️tests/🔬️wgpu-unit/🦀️.rs` | 24 laws | fold + retained tree |

### Changed

| file:line | change |
|---|---|
| `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` (`//#region 🔐️HubElements`, after the `space_administration` mount) | the three `#[path]` module mounts |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:458` | `shell_hub_device_instance_id` — the session id reduced to the hub's `^[A-Za-z0-9._:-]+$` grammar, so a sign-in can never fail on a local id format |
| `…:469` | `shell_hub_bootstrap_origin` — the wgpu twin of AU3 §4.4's `hubBootstrapOriginV1`: the HUB's origin, never the page's |
| `…:3191,3195` | `ShellState::hub_workspace` + `hub_session_capability` (the latter is the only place a minted token lives, and it is absent from every serializable projection) |
| `…:5554` | both fields initialized from the bootstrap-only book |
| `…:8001` | the `framework.hub` dock leaf, beside the chat leaf in `TopRight` |
| `…:9505` | one dispatch arm — `other if other.starts_with("hub")` → the hub lane |
| `…` (`//#region 🔐️HubWorkspaceLane`, after `📇️DirectoryLane`) | `handle_hub_workspace_action` + `run_hub_sign_in_turn` / `run_hub_sign_out_turn` / `run_hub_create_invite_turn` / `run_hub_redeem_turn` / `reload_hub_spaces` / `reload_hub_members` / `hub_online_user_ids` |
| `…` (`shell_owned_panel_leaves`, `publish_shell_panel_document`) | the hub leaf is dockable and has a body |
| `…:25171` | `panelToggle.hub` en+de |

### WG-5 — the footer pill, and who landed what

The pill itself (`s-hub-connection`, `ShellHubConnectionState`, layout, retained paint phase 4, en+de
keys) was **already on the tree when this slice started** — a peer landed it, and §1 records the
measurement. What it lacked was U1 §8's actual fold. During this slice a peer also rewrote
`hub_connection_state` (`🐚️Shell/…/🦀️.rs:8634`) to `shell_hub_connection_summary_v1(&self.hub_projection()).state`,
and that function (`:15026-15039`) now maps the shell's own `ShellHubRemoteV1` onto
**this slice's `HubDocumentRemote`** and delegates to **this slice's `hub_connection_summary`**. So
WG-5's fold is one implementation with 7 laws over it, and the shell owns only the adapter. That
integration is a peer's edit, not this slice's — recorded here because the fold it calls is this
slice's deliverable and the two must be read together.

## 4. Tests with real counts

(filling)

## 5. Honest gaps

(filling)

## 6. Files changed

(filling)
