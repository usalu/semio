# 🏛️ Fable — Space Administration (packet sections 1, 3, 4)

Lane `fable-space-administration`, ticket `26/09/02/COMPLETE-SEMIO-END-TO-END`.
Packet: `📓️terra-space-administration-ui-current-p0.md` §1, §3, §4 (authoritative), with
`📓️terra-author-space-administration-page-receipt-p0.md` consulted for backend-read and cursor design.
Section 2 (`DirectoryCommandRequestV1`/`DirectoryCommandReceiptV1`, `post_directory_commands`) belongs to the
sibling lane `fable-directory-command-receipt`; this lane only **consumes** those types by name.

---

## 1. What changed

### §1 — bounded, receipt-bound administration page replaces the unbounded detail shape

| File | Change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs` | **Deleted** `DirectorySpaceDetailV1`. Added `DirectorySpaceAdministrationPageV1` (internally tagged on `access`), its member/invite/document/public-document windows, `DirectorySpaceAdministrationCapabilitiesV1`, `DirectorySpaceAdministrationSectionV1`, `DirectorySpaceAdministrationPageErrorV1`, `canonical_unsigned_json`/`receipt_matches`/`validate`/`parse_canonical_json`, and the constants `…PAGE_MAX_ROWS = 64`, `…PAGE_MAX_BYTES = 48 * 1024`, `…CURSOR_MAX_BYTES = 1024`, `…PAGE_SCHEMA`. |
| `…/🧬️schema/🟦️.ts` | TypeScript twin + `parseDirectorySpaceAdministrationPageV1` (exact-field, control-character, ordering, canonical-bytes and SHA-256 receipt checks). |
| `…/🧬️schema/🔣️.json` | JSON-schema `$defs` for the page, its windows, rows, capabilities and the opaque cursor; `DirectorySpaceDetailV1` def removed. |
| `…/📇️directory/🦀️.rs`, `…/📇️directory/🟦️.ts` | Module-root re-exports updated (detail removed, administration types added). |
| `🌎️hub/📇️directory/🦀️.rs` | Added safe row records `SpaceAdministrationMemberRow` / `SpaceAdministrationInviteRow` (display/metadata columns only — no password hash, SSO subject/provider, selector, secret digest, revoke reason or accepted-event id), the constants `SPACE_ADMINISTRATION_PAGE_MAX = 64` / `…FETCH_MAX = 65`, two new `HubDirectory` trait methods, and their `HubDirectories` dispatch with the limit guard. |
| `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`, `🐘️postgres/🦀️.rs`, `🌐️neo4j/🦀️.rs` | `list_space_administration_members_page` (keyset `user_id ASC`) and `list_space_administration_invites_page` (keyset `created_at DESC, id DESC`) in all three backends. Each query selects **only** display/metadata columns, so no credential- or secret-bearing record is ever constructed at the backend boundary. |
| `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` | `get_directory_space` rewritten: strict query admission (empty, or exactly `cursor=<opaque>`), domain-separated session binding, HMAC-bound opaque keyset cursor (`space_administration_cursor_*`, new `HubState.space_administration_cursor_key`), bounded reads through `list_admin_space_summaries_page` + the two new primitives + `list_document_descriptors_page`, deterministic byte fitting to ≤48 KiB, `seal_space_administration_page_v1`, and `revalidate_space_administration_caller` **after** the reads and **before** the response. `invite_view` deleted (no remaining caller). |

Page shape (canonical key order): `access`, `schema`, `sessionBindingSha256`, `authorizationGeneration`,
`spaceId`, `space`, `[members]`, `documents`, `[invites]`, `[capabilities]`, `receiptSha256`.
Only the `author` variant carries `invites` and `capabilities`; `member`/`public` omit them **structurally**
(they are not empty placeholders — the variant has no such field).

Post-read recheck outcomes: revoked/rotated session ⇒ `401`; role downgrade ⇒ `403`; membership loss or a
private space ⇒ `404` (non-enumerating); malformed/foreign/tampered cursor ⇒ `400`; deadline ⇒ `504`;
a single row that cannot be fitted ⇒ `413`.

**Consumers migrated** (verified by a repo-wide grep at 13:26 — see §3):
`DirectoryClient.space()` → `spaceAdministrationPage()` (TS, `💻️os/🟦️.ts`);
`DirectoryClient::space()` → `space_administration_page()` (native, `📇️directory/🔌️client/🦀️.rs`, with
`CanonicalDirectorySpaceAdministrationPageV1` preserving the exact response bytes);
`🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` (now fails closed with `InvalidResponse("space administration page exceeds one bounded window")` rather than silently truncating);
`🌉️mcp/🏠️workspace/🦀️.rs` test fixture harness; the MCP `🔣️authenticated-hub-descriptor-index.json` fixture
(migrated to `canonicalBody` strings with real receipts, because `serde_json::Value` re-serializes keys
alphabetically and would never satisfy the canonical-bytes check);
`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`'s `space_public_boundary_real_routes_…` law.
The admin SPA's `AdminClient.space` is a **different** route (`/admin/api/*`) and was intentionally untouched.

### §3 — one shell-owned retained `DirectoryAdministrationOperation`

Late addition this pass: a scoped **4401** now retires the operation. The revoked-scope callback previously
posted only `directory-scope-revoked`, so a membership revocation on the administered space left an
authoritative-looking pane alive until the next read happened to answer 403/404.
`revokeDirectoryAdministrationForScope(spaceId)` closes that: it terminates the operation as
`denied`/`forbidden` when — and only when — the revoked scope's space is the one being administered.

| File | Change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🟦️.ts` | Closed request/response types: `DirectoryAdministrationPhaseV1`, `DirectoryAdministrationSectionV1`, worker requests `directory-administration-{open,refresh,submit,acknowledge,close}`, worker responses `directory-administration-state` and `directory-administration-capability`. `CanonicalDirectorySpaceAdministrationPageV1` + `DirectoryClient.spaceAdministrationPage`. |
| `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts` | New `//#region 🔖️SpaceAdministration`: `DIRECTORY_ADMINISTRATION_CAPACITY = 1`, the retained operation, `postDirectoryAdministrationState`, `terminateDirectoryAdministration`, `liveDirectoryAdministration`, `directoryAdministrationPageTermination`, `loadDirectoryAdministrationPage`, `openDirectoryAdministration`, `submitDirectoryAdministrationCommand`, `acknowledgeDirectoryAdministrationCapability`, `closeDirectoryAdministration`; dispatcher cases; `closeDirectory()` retires the operation as `stale`. |
| `…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | Native twin beside `ShellDirectoryRunner`, driven once per frame from `pump_directory_events`: `ShellSpaceAdministrationPhaseV1`, `ShellSpaceAdministrationTurnV1`, `ShellSpaceAdministrationOperationV1` (finite `turn()` driver), `shell_space_administration_{member_removable,invite_revocable,label,status,controls}`, `ShellSpaceAdministrationControlV1`; `ShellState.space_administration`/`space_administration_epoch`; `open_space_administration`, `close_space_administration`, `acknowledge_space_administration_capability`, `pump_space_administration` (drives the real `DirectoryClient`, called from `pump_directory_events`). |

States `Loading → Ready(author|member|public) → Submitting → Receipt → Refreshing → Ready`, with
`Cancelled`/`Denied`/`Stale`/`Failed` terminal. Every terminal transition erases the page bytes, the receipt
and the invite capability **before** the renderer is notified. Administration mutations are **never**
auto-retried: an indeterminate transport settles `Failed` ("unknown outcome / refresh required"), so a
`create-invite` can never mint a second invitation behind the operator's back. Only an exact server receipt
advances the operation, and a receipt is always followed by a mandatory page refresh.

The retained operation is deliberately **not** the sibling lane's `directoryCommandQueue`: it owns its own
`AbortController` and calls `DirectoryClient.command` directly, so it never shares the FIFO's retry policy.

### §4 — role-aware Home rows, `manageSpace`, and the administration pane

| File | Change |
|---|---|
| `✏️s/🔌️plugins/🪐️space/🦀️.rs` | `HomeSpaceRow` gains `role: Option<DirectorySpaceRole>`; `home_space_rows(directory, client_id)` fills it from the hub-confirmed fold via the new `caller_role`; local-catalog rows carry `None`. |
| `…/🏠️home/…/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs` | `row_actions` now gates Rename/Share/Delete **and** the new Manage on `row.origin == "hub" && row.role == Some(Author)` — hub origin alone is no longer a capability. |
| `…/👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs`, `…/👁️viewer/🦀️.rs` | Threaded `client_id` through the viewer render. |
| `…/✏️editor/🎮️commands/🏛️manage-space/🦀️.rs` | New command leaf: relays `os.directory.open-administration` with the exact `spaceId`, emits no mutation, faults on an empty id. |
| `…/✏️editor/🗣️terminology/🦀️.rs`, `…/✏️editor/🦀️.rs`, `🪐️space/📦️packages/🦀️rust/🦀️.rs` | EN/DE `action_manage`; `manageSpace` registered in `app_commands!`, retained tool ids, publication contracts, extent, wire parse, manifest `shell_action` (EN "Manage Space" / DE "Space verwalten"), `InteractiveJobClassification::Migrated`, and the window action refs; module mounted. |
| `…/🧱️elements/🛂️SpaceAdministration/🟦️.tsx` | **New** React pane. Renders solely from the canonical page. `spaceAdministrationCapabilities` (author-only), `spaceAdministrationMemberRemovable` (owner removal disabled **before** dispatch), `spaceAdministrationInviteRevocable`, `spaceAdministrationDispatchable`. Full EN/DE bundle with no default language, `<h2 tabIndex={-1}>` focus restoration on every settled phase, one `role="status" aria-live="polite"` region, semantic `<button>`s with `aria-label`, and `<label for>`-bound role `<select>`s. |
| `…/🏛️ShellHost/🟦️.tsx` | `ShellSpaceAdministrationStateV1`, the pure `reduceShellSpaceAdministrationState`, `shellSpaceAdministrationRequest` (returns `null` for anything the canonical page does not authorize), `copyDirectoryInviteCapabilityV1`, `SHELL_SPACE_ADMINISTRATION_INVITE_TTL_SECS`; `os.directory.open-administration` relay arm; worker-message handling that parses the canonical bytes before reducing; `dispatchSpaceAdministrationIntent`; unmount/identity-change effect that closes the operation and retires the epoch; the pane mounted in the shell chrome. |

Neither renderer derives authority from a locally stored role: the React pane and the WGPU control builder
both read `capabilities` off the server-sealed page, and a `member`/`public` page has no such field.

---

## 2. Verification — exact commands and results

Run from the repo root unless noted. Machine under heavy concurrent peer load (20+ cargo processes).

| # | Command | Result |
|---|---|---|
| V1 | `bun nx run os-hub:space-administration-source-check --skip-nx-cache` | **PASS**, 68.6 s wall (13:33), then re-run after the scoped-4401 fence was added |
| V2 | `bun ./📜️script.ts space-administration-check source` (in `🌎️hub/📦️packages/🦀️rust`) | **PASS** — `space-administration-oracle: AJV=2 vectors=4 cursors=8 hostiles=9 source-hostiles=9 component-schema=9 sha256=1 binding=1` / `space-administration-check: checks=41 phase=source` |
| V3 | `bun …/vitest.mjs run --config 🧪️tests/🟦️.ts --testTimeout 60000` (in `💻️os/📦️packages/🟦️typescript`), **run 3×** | **PASS ×3** — `Test Files 3 passed (3)`, `Tests 259 passed (259)`, 14 s each (13:50). Repeated deliberately: an earlier fixed-turn settle helper failed 1-in-N under load, so it was replaced with a quiescence-based one (see below). |
| V4 | `bun …/vitest.mjs run --config 🧪️tests/🟦️.ts --testTimeout 30000 -t "space administration"` (same cwd) | **PASS** — `Tests 4 passed | 253 skipped`, 27.12 s |
| V5 | `SEMIO_TEST_LEVEL=long bun …/vitest.mjs run --config 🧪️tests/🟦️.ts --testTimeout 60000 "space-administration"` (in `…/🎯️targets/⚛️react`) | **PASS** — `Test Files 1 passed (1)`, `Tests 10 passed (10)`, 62.32 s (rerun at 13:46) |
| V6 | repo-wide grep for `DirectorySpaceDetailV1` / `parseDirectorySpaceDetailV1` over `🧰️framework`, `🌎️hub`, `✏️s` | Only the **two intentional negative assertions** in `📜️script.ts:6979,6987` (`!rust.includes(…)`, `!route.includes(…)`). No production or test consumer remains. |
| V7 | arity grep: `home_space_rows(`, `main::render(`, `HomeSpaceRow {` across `✏️s` | All three call sites carry the new `client_id` / `role` arity; every test literal supplies `role`. |
| V8 | ad-hoc Bun script parsing every `canonicalBody` in the migrated MCP fixture through `parseDirectorySpaceAdministrationPageV1` | **PASS** — 3 pages (`publicWithoutMembership` public, `memberReady` author, `sameDocumentOtherSpace` author), all byte-canonical with valid receipts and none carrying `selector`/`secretDigest`/`passwordHash`/`ssoSubject`. Since the Rust `parse_canonical_json` enforces the same canonical bytes and receipt, this is strong (not conclusive) evidence the Rust MCP test will accept them. |

### Native laws present but unrun
`✏️s/…/🏠️main/🦀️.rs` carries `a_hub_row_stamps_the_space_row_id_and_carries_dispatchable_row_actions`
(now asserting **5** buttons — open + rename + share + delete + manage — and that the `manageSpace`
descriptor carries the row's own `spaceId`) and `spectator_and_unbound_hub_rows_only_carry_open`
(a spectator row and a role-less hub row each expose **only** `openSpace`). These are the direct
role-blindness regression laws for §4; they were **not executed** (see nonclaim 1).

### V1 oracle coverage (language-agnostic, Bun + AJV + `node:crypto`, no Rust involved)
Neutral fixture `🌎️hub/🧪️fixtures/📇️directory/🏘️space-administration-page-v1/{🔣️.json,🧬️.schema.json}`
(mirrors the `📅️event-page-route-v1` folder layout).
- **4 vectors**: author positive; member omission; public omission; author 64-row window boundary.
- **Byte-exact session binding** recomputed independently (`semio/hub/directory-space-administration/session-binding/v1\0` + length-prefixed session id, user id, generation, expiry, space id), plus a length-prefix **aliasing** probe (`session-admin-0`+`1user-admin-01` vs `session-admin-01`+`user-admin-01` — same concatenation, different digest).
- **8 cursor-admission cases**: empty, valid, `cursor=`, duplicate param, percent-encoded, `+`, wrong param name, non-grammar bytes.
- **9 hostiles**, each proven rejected: substituted receipt, trailing whitespace, unknown field, space mismatch, reversed member order, reversed invite order, 65-row window, oversize page, secret-shaped extra field.
- **Secret-absence assertion** on every sealed vector for `selector`, `secretDigest`, `inviteToken`, `passwordHash`, `ssoSubject`, `ssoProvider`, `sessionId`.
- **Shared-component schema-first parity** (`AJV=2`): the directory component's own
  `🔣️.json#/$defs/DirectorySpaceAdministrationPageV1` — the schema the Rust and TypeScript twins are both
  derived from — is compiled separately and must admit all 4 sealed vectors and reject 5 hostiles,
  including a `member` page carrying an `invites` window and a `member` page carrying `capabilities`.
  Without this the component `$defs` could drift away from *both* implementations unnoticed.
- **9 source-boundary hostiles**: each removes exactly one production fence (row ceiling, TS parser export, post-read revalidation, cursor query guard, Postgres invite page, Neo4j member page, worker capability erasure, worker scoped-4401 retirement, Home author-role gate) and the oracle must reject it.

### V4 browser-worker laws (real `DirectoryClient` + fake authenticated transport, no bypass token)
1. `loading → ready → submitting → receipt → refreshing → ready`; the `submitting` and `receipt` states still
   carry the **first** page bytes (no page mutation before the refresh) and `submitting` carries **no**
   `receiptSha256`; exactly **one** POST is issued (no double dispatch).
2. The one-shot invite capability is posted **exactly once** across two acknowledgements, and
   `inviteCapabilityPending` is cleared afterwards.
3. `401` ⇒ `denied`/`unauthorized` and `403` ⇒ `denied`/`forbidden`, both with the page erased.
4. Close cancels; every later `submit`/`refresh` for the retired epoch issues **zero** requests.
5. A scoped **4401** for the administered space retires the pane as `denied`/`forbidden` with the page
   erased, while a revocation for **another** space leaves the operation `ready` and untouched.

### V5 ShellHost / pane laws
Reducer: superseded epoch ignored; all four terminal phases erase page + receipt + capability marker;
receipt only when the worker reports one. Intent mapper: owner removal, unknown member, revoked invite,
member-page invite creation, mid-submit dispatch and denied-state dispatch **all** return `null`; `close`
always maps. Pane: `role="status"`/`aria-live="polite"`; owner Remove disabled, non-owner enabled; revoked
invite Revoke disabled; `<label for>`-bound select dispatches `set-role`; copy affordance appears only while
the capability is pending and disappears after acknowledgement; a `denied` phase erases every control, keeps
the localized status text, and leaves focus on the pane heading.

---

## 3. Registered gates

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`: `SpaceAdministrationCheckScript`, registered as `space-administration-check` (accepts `source` | `native`).
- `🌎️hub/📦️packages/🦀️rust/📋️project.json`: `space-administration-source-check`, `space-administration-native-check`.
- `.vscode/🧩️launch.seed.jsonc`: `⚖️gate🏛️space-administration📐️source` (order 411.0999) and `⚖️gate🏛️space-administration🦀️native` (order 411.09995), placed in the existing `4_gate` group beside the sibling directory gates. **Not** hand-edited into the generated `launch.json`.

---

## 4. Nonclaims — what is NOT proven

1. **Partial Rust compile receipt.** The shared cargo build-dir lock was fully contended all afternoon
   (12+ queued peer `cargo check` invocations, all at 0% CPU), so two foreground attempts on the default
   target dir produced no diagnostics at all. Escaping it with a **private** `CARGO_TARGET_DIR` under the
   scratchpad worked:
   `CARGO_TARGET_DIR=<scratch>/hub-target CARGO_BUILD_JOBS=4 cargo check -p semio-hub --bin os-hub --message-format=short`
   (cold, ~25 min).
   - ✅ **`semio-framework-os-kernel` compiled: 40 warnings, ZERO errors.** That crate contains
     `📇️directory/🧬️schema/🦀️.rs` (the new `DirectorySpaceAdministrationPageV1` and the deleted
     `DirectorySpaceDetailV1`), `📇️directory/🔌️client/🦀️.rs` (`space_administration_page` +
     `CanonicalDirectorySpaceAdministrationPageV1`) and the module re-exports. This is a real receipt for
     §1's shared wire law and the native client.
   - ❌ **The build stopped before `semio-hub`** on an unrelated peer error (see §5): the run emitted
     exactly **one** error, and it is not in any file this lane touched.
   Therefore **still unverified by compiler**: the hub route (`🚀️bin.rs`), the three directory backends,
   the WGPU shell driver, the space plugin, the MCP consumer, and every Rust test I added. A `--lib` check
   also does not compile `#[cfg(test)]` modules, so even the kernel's own test module is unverified.

2. **The four hub native laws were not run.** `space_administration_page_v1_route_returns_the_author_windows_with_a_canonical_receipt`,
   `…_denies_a_spectator_the_author_windows`, `…_denies_a_removed_member_and_leaks_no_rows`,
   `…_rejects_a_noncanonical_query_and_a_foreign_cursor` are registered in the `native` phase of the gate
   but never executed. `bun nx run os-hub:space-administration-native-check` remains unrun.
3. **The two WGPU native laws were not run** (`space_administration_operation_drives_a_real_directory_client_through_its_finite_turns`,
   `…_terminates_on_denial_stale_and_indeterminate_transport`). They drive a **real** `DirectoryClient` over
   the existing `DirectoryBootstrapFakeTransport`; they do **not** exercise `NativeDirectoryTransport`
   itself, which needs a live HTTP listener. The packet's "actual `DirectoryClient<NativeDirectoryTransport>`"
   wording is therefore only partially met.
4. **Postgres and Neo4j are source-only.** Their two new methods are feature-gated and were never compiled
   or executed. Only their presence is fenced by the oracle.
5. **No process/browser run.** No hub was started, no page was fetched over real HTTP, no shell was booted.
   Acceptance item 5 (the two-user process law) was not attempted.
6. **The WGPU pane is a control-set builder, not a rendered surface.** `shell_space_administration_controls`
   produces the capability-derived, EN/DE-labelled, keyboard-addressable control set; wiring it into a
   window kind and its wgpu draw path is not done.
7. **The native frame-loop wiring is source-only.** `pump_directory_events` now begins with
   `self.pump_space_administration().await`, so the retained native operation advances one bounded turn
   per frame and never spins (the driver returns `false` at `Idle`/terminal). That call site is written
   but, like every other Rust change here, not compile- or run-verified by me.
8. **`administrationRevision` is deliberately absent.** The sibling receipt packet proposes a durable
   per-space revision counter; the authoritative §1 list does not require one, so this lane binds cursors to
   user/session/generation/space/section only, and relies on keyset ordering (rather than a pinned snapshot)
   for insert/delete stability. A page pair straddling a mutation is therefore *consistent per row* but not
   *snapshot-isolated*. Recorded as a deliberate scope decision, not an oversight.
9. **Documents ride in the page.** §1 lists only member and invite windows; a bounded `documents` window was
   added because retiring the detail route otherwise left `🌉️mcp/🏠️workspace/🔗️remote` with no way to read a
   space's document catalog. It uses an offset cursor over the pre-existing
   `list_document_descriptors_page`, not a keyset one.

---

## 5. Peer interactions

- **Emoji-uniqueness rename tool.** Between my writes and this pass, a repo-wide tool renamed
  `🧱️elements/🏛️SpaceAdministration` → `🛂️SpaceAdministration` (collision with `🏛️ShellHost`) and
  `🧪️fixtures/📇️directory/space-administration-page-v1` → `🏘️space-administration-page-v1`. It also
  rewrote the ShellHost import, the React test import and the oracle's fixture path. Verified all three
  resolve; no action needed beyond confirmation.
- **Sibling lane `fable-directory-command-receipt`.** Landed `DirectoryCommandRequestV1` /
  `DirectoryCommandReceiptV1` / `directoryCommandSha256` / `sealDirectoryCommandRequestV1` in the shared
  schema and the `DirectoryClient.command` receipt path. This lane consumes them by name; I did not touch
  `post_directory_commands`, `DirectoryCommandResponse`, or the worker command queue.
- **A peer extended my worker region** with a two-phase invite handover: the single
  `directory-administration-acknowledge` request became `directory-administration-capability-request` +
  `directory-administration-capability-result { transferEpoch, copied }`, with
  `directory-administration-capability-rejected { code: "capacity" | "already-settled" | "mismatch" }`
  and the retained fields `inviteCapabilityStatus` (`available` | `copying` | `failed`) and
  `inviteTransferEpoch`. This closes the gap where a clipboard write that *failed* was indistinguishable
  from one that succeeded, so I kept it and re-verified the whole lane against it: they had already
  migrated `shellSpaceAdministrationRequest` (now `copy-invite-capability` →
  `…capability-request`), the pane's copy button (disabled while `copying`), and my four worker laws.
  V3/V5 above are reruns **after** that change (258 and 10 tests, both green).
  Their change did break my contiguous-literal source fence for the terminal erasure; I replaced it with a
  **structural** fence (`directoryAdministrationTerminateErases`) that extracts the
  `terminateDirectoryAdministration` body and requires every retained field to be nulled before the phase
  settles — robust to added fields, still armed against a removed erase (source-hostile 6 confirms).
- **External blocker — `semio-framework-plugin-host` does not compile.** The one error in my whole
  `cargo check -p semio-hub --bin os-hub` run is a peer's, and it is deterministic and source-visible:

  ```
  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs:428:14:
  error[E0004]: non-exhaustive patterns:
    `component::actor_bindings::semio::framework::effects::Effect::RequestInferenceProposal(_)` not covered
  error: could not compile `semio-framework-plugin-host` (lib) due to 1 previous error; 9 warnings emitted
  ```

  The inference lane added `request-inference-proposal` to the WIT (`🔌️plugin/🧬️schema/📜️.wit:603`) and
  updated **one** of the two host match sites (`🖥️host/🦀️.rs:2642`), leaving `wit_effect_to_kernel` in
  `🖥️host/📥️imports/🦀️.rs` without an arm (`grep -c RequestInferenceProposal` on that file = **0** at
  15:0x). `semio-hub` depends on the plugin host, so **nobody** can compile the hub until that arm lands.
  I did **not** fix it — it is another lane's mid-flight WIT change and adding an arm would overwrite
  their in-progress mapping. Per the brief I waited 10 minutes and retried once (see below).

- **Scratchpad target-dir collision (worth flagging to the coordinator).** My retry did not fail on the
  peer error — it printed `Blocking waiting for file lock on build directory` and was killed at the tool
  ceiling. Cause: the sibling lane `fable-execution-target-lease` is running
  `cargo test -p semio-framework-os-kernel --lib execution_target` with
  `CARGO_TARGET_DIR="$S/hub-target"` pointing at the **same** session scratchpad path I chose, so the two
  lanes serialize on one private build-dir lock and SIGTERM each other's cargo (my first run exited 143).
  Lanes that escape the shared `target/` lock should use a **lane-qualified** directory name; `hub-target`
  is evidently a collision-prone default.

- **Retry outcome.** Because the plugin-host arm is still absent in source (verified by grep immediately
  after the wait), a further rebuild could only reproduce the same error, so I stopped rather than spend
  another ~25 minutes of contended CPU to re-derive a known result.

---

## 6. Files touched

`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/{🦀️.rs,🟦️.ts,🔣️.json}`
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/{🦀️.rs,🟦️.ts}`
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧫️fixtures/🔣️authenticated-hub-descriptor-index.json`
`🧰️framework/🛍️products/💻️os/{🟦️.ts,🧵️backbone-worker.ts}`
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🟦️.tsx` (new)
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🏛️space-administration.test.tsx` (new)
`🌎️hub/📇️directory/🦀️.rs`, `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`, `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`, `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`
`🌎️hub/📦️packages/🦀️rust/{🚀️bin.rs,📜️script.ts,📋️project.json}`
`🌎️hub/🧪️fixtures/📇️directory/🏘️space-administration-page-v1/{🔣️.json,🧬️.schema.json}` (new)
`✏️s/🔌️plugins/🪐️space/🦀️.rs`, `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/🦀️.rs`
`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/{🦀️.rs,🗣️terminology/🦀️.rs}`
`…/✏️editor/🎮️commands/🏛️manage-space/🦀️.rs` (new)
`…/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs`
`…/👁️viewer/🦀️.rs`, `…/👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs`
`.vscode/🧩️launch.seed.jsonc`

No `[DEBUG]` logs were left behind by this lane. No git-modifying command was run.
