# Hub Spaces, Live Presence and Collaborative Studios — end‑to‑end program plan

## Context

`s` (semio OS) already has most of the machinery: an axum hub (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs`) with a `db::Database` event store, a `HubDirectory` (users/spaces/memberships/auth+sync sessions, sqlite/postgres/neo4j), a binary document WS lane (Hello/Commands/Presence/…), a TS backbone worker (`💻️os/🟦️backbone-worker.ts`) and a Rust sync actor (`🏪️store/🔄️sync`), per‑subset viewer/editor surfaces + AppRouter/OpeningResolver/"Open with…" in both shells (landed today by `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`), an `s.home` artifact with editor+viewer, a studio engine, and a launch.json generator.

What is missing to reach the requested end state:
- **Identity/hub binding is manual** (sync‑card `remote://host/space`, actor `"local"`, no user, no token; the same in the wgpu shell). No `S_HUB_URL`/`S_USER` env anywhere.
- **Home lists spaces from a wasm‑local catalog** (`LocalStorageBackbonePort`), not from the hub. Hub has no space list/create/delete API, no directory event log, no live directory stream.
- **No Space app** (table of artifacts of a space) — the `/spaces/{id}` route opens the workflow studio.
- **Presence is per document only**, not per app/viewer/editor; wgpu shell drops presence events.
- **No hub admin page/API**, `hub_sync_session` rows are written but never queried.
- **No auto save/check‑in policy**; checkpoints exist only via manual palette action.
- **Dev configs**: only one `s` launcher; hub default port collides with `s` (6070); hub `📜️script.ts dev` drops the launch env; two `dev s` processes would both run cargo plugin builds.

Dev decisions already taken (asked): **React + wgpu shells both in scope**; **admin page served by the hub binary at `/admin`** (+ vite dev launcher); **existing `🛠️dev🖥️s⚛️react` stays local‑only, only new 👤️1/👤️2 launchers bind the hub**; **directory = append‑only event log with SQL projections + live `/directory/ws` stream**.

Execution model mandated by the dev: this plan (Fable 5) → **Opus 5 main chat coordinates** → **Sonnet 5 workers** execute lanes → **Haiku 4.5** read‑only scouts/audits.

---

## 0. Frozen contract (coordinator copies into `📋️contract-freeze.md`)

### 0.1 Ports / env / ids
| Item | Value |
|---|---|
| Hub | `OS_HUB_PORT=8787` (fix defaults: `📚️library/📦️packages/🟦️typescript/📦️index.ts:1952` `OS_HUB_PORT=6070→8787`, `📦️bin.rs:819` `unwrap_or(6070)→8787`), `OS_HUB_DATA=${workspaceFolder}/.semio/hub-dev/`, `OS_HUB_ADMIN_TOKEN` optional (dev default: loopback peers are admin, logged loudly), `OS_HUB_ADMIN_DIR` optional |
| Admin vite dev | 8790 (`🛠️dev🗄️os-hub🛡️admin`, proxies to 8787) |
| s user1 / user2 | react **6072 / 6073**, wgpu **6067 / 6068** (6071 = multi harness; all four verified unused). Existing `s` 6070/6066 untouched |
| Client env | `S_HUB_URL=http://127.0.0.1:8787`, `S_USER=user1@semio.dev|user2@semio.dev`, `S_DATA_DIR=${workspaceFolder}/.semio/s-user1|s-user2` (per‑user folder lane) → vite `VITE_S_HUB_URL/VITE_S_USER/VITE_S_DATA_DIR`; wgpu native reads `S_*` directly; wgpu browser via `🟦️boot.ts` |
| Actor id | `user:{user_id}#{shell_session_id}` (hub groups by `user_id`) |
| Surface id | existing `<kind>@<standard>/<subset>#<role>` (e.g. `s.space.home@1/*#editor`) |
| Presence scope | `(space_id, document_id, surface)` carried **out‑of‑band as `?surface=` on the document WS URL** (`PresencePeer` flag byte is full and its wire file is peer‑leased — no wire change) |
| Document ids | space index doc = `index` in hub space `{space_id}`; artifacts = minted `mint_document_id()` |
| Routes (shell) | `/` home; `/spaces/{id}` → **Space app** (`s.space` editor/viewer); `/spaces/{id}/studio` → workflow studio (moved) |
| Row/element ids (e2e) | `data-row-id="space:<id>" / "artifact:<id>" / "peer:<actor>" / "history:<id>"`; ids `#s-home-create-space`, `#s-space-create-artifact`, `#s-space-share`, `#s-presence-peers`, `#s-checkin`; every new node has `data-ui-path` (wgpu parity join). No `data-testid`. |
| Channel | CHANNEL_VERSION is 11 (peer). Ours: **12**, tags appended after peer's (AppCommand ≥33, AppFrame ≥25, ArtifactCommand ≥17) — only if C9 must extend the app channel; prefer REST + directory WS to avoid it |

### 0.2 Directory (event‑sourced, JSON control plane)
Schema triad lives **in the OS framework** (hub depends on os kernel already; os must not depend on hub): `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/{🔣️component.json,🦀️component.rs,🟦️component.ts}` (+ `🛰️component.proto`,`🔗️component.graphql` if the taxonomy validator requires 5 leaves — check `🔣️taxonomy.json`).

```
DirectoryEvent { seq:u64 (dense,1‑based, backend‑assigned), id:uuid‑v7, hlc:{physicalMs,logical}, actor:{kind:"user"|"admin"|"system", id}, spaceId?, userId?, body, recordedAtMs }
body.kind ∈ user.created{userId,email,displayName} | space.created{spaceId,name,kind,visibility,ownerUserId} | space.renamed | space.visibility-changed | space.archived | space.deleted | member.upserted{spaceId,userId,role} | member.removed | invite.redeemed{spaceId,userId,inviteId,role}
DirectoryCommand.kind ∈ create-space{name,kind,visibility} | rename-space | set-visibility | archive-space | delete-space | upsert-member{spaceId,email,role} | remove-member | create-invite{spaceId,role,ttlSecs}→{inviteToken} | revoke-invite
DirectoryStreamMessage ∈ {kind:"event",event} | {kind:"connection",phase:"opened"|"closed",connection:ConnectionView} | {kind:"presence",spaceId,documentId,surface,actors[]} | {kind:"heartbeat",headSeq}
Read DTOs: SpaceView{id,name,kind,visibility,ownerUserId,role?,memberCount,documentCount,activeConnections,createdAtMs,updatedAtMs}, MemberView, UserView, ConnectionView{syncSessionId,spaceId,documentId,surface,actor,userId?,email?,role,connectedAtMs,presenceKnown}, DocumentView{id,headSeq,commitSeq,epoch}, InviteView
DirectoryReadModel { spaces: BTreeMap<spaceId, {…SpaceView, members[]}>, cursor:seq }  +  pure fold(model,event) (Rust + TS, parity fixture 💻️os/🧫️fixtures/📇️directory/🧾️events.json)
```
Laws (decider): atelier ⇒ ≤1 author; archive ⇒ no authors (emit `member.upserted{spectator}` per author, then `space.archived`); owner membership never removed; commands on deleted space ⇒ NotFound. Not event‑sourced: share tokens, auth sessions, sync sessions, invites (redemption *is* an event).

Hub HTTP/WS (JSON, `Authorization: Bearer <session>`; WS `?token=`):
`POST /directory/commands` → `202 {events[], result?}` (authz: create any session; delete/archive owner|admin; rest any author|admin) · `GET /directory/spaces` (member + public) · `GET /directory/spaces/{id}` (+members, documents(frontier), invites for authors) · `POST /directory/invites/{token}/redeem` · `GET /directory/events?since=&limit=` (visibility‑filtered) · `GET /directory/ws?token=&since=` (subscribe‑then‑replay, gap‑free) · `GET|DELETE /auth/sessions/me` (existing `POST /auth/sessions {email}` unchanged).
Admin (bearer `OS_HUB_ADMIN_TOKEN` or loopback default): `GET /admin/api/{overview,spaces,spaces/{id},users,connections,documents?space=,events}` · `POST /admin/api/commands` (actor admin) · `POST /admin/api/directory/rebuild` · `POST /admin/api/connections/{id}/close` (kick via `Notify`) · `POST /admin/api/users/{id}/sessions/revoke` · `GET /admin`, `/admin/{*path}` static SPA.

### 0.3 Identity (client)
New OS config facet `os.config.identity` beside `os.config.opening` in `💻️os/🎚️config/🧬️schema/` : `Identity{userId,email,displayName,hubBaseUrl,sessionToken,issuedAtMs}` + mutations `sign-in`/`sign-out` (fold over config op log, persisted local‑only in `S_DATA_DIR/os`). Boot: env → `GET /auth/sessions/me` with cached token → 401 ⇒ `POST /auth/sessions{email}` → `sign-in`. Hub unreachable ⇒ keep last identity, offline chip, backoff retry; never blocks UI.

### 0.4 Space app artifact
New plugin‑owned artifact `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/{🧬️schema,🚪️io,📚️examples,👁️viewer,✏️editor}` (scaffold via registry `new surface`), kind `s.space`, dialect `s.space.space@1/*`. `SSpaceSnapshot{schema,spaceId,artifacts:[{id,name,kindId,schema,dialect,createdAtMs,createdBy,updatedAtMs,updatedBy}]}`; mutations `create-artifact` (Fatal `mutation.duplicate-id`), `delete-artifact` (Error `mutation.target-missing`), `rename-artifact`, `touch-artifact` — all returning `MutationOutcome` per the peer contract. `project_space_index_to_collection()` feeds `resolve_workflow_artifact_document` (one source of truth). Persisted **shared** (hub doc `index`) + folder lane.

### 0.5 Save / check‑in policy
Every accepted `Apply` = Edit → hub relay (Ack Persisted) + folder snapshot; status pill `persisted | pending(n) | remote(connected|connecting|backoff|detached)` from `ArtifactSyncStatus`. **Auto check‑in**: per open editor session, idle ≥20 s or ≥200 uncommitted edits ⇒ `CommitCheckpoint{message:"auto",authors:[identity]}`; explicit `#s-checkin` action with message dialog; checkpoint on editor close with pending edits; viewers never checkpoint. After each checkpoint the shell dispatches `touch-artifact` to the space index. Uses existing `ArtifactCommand::CommitCheckpoint` / `commit_space_checkpoint`; **no store‑internal changes** (peer lease).

---

## 1. Ticket + coordination runbook (Opus coordinator)

1. Read `repo://goals`; `ticket_open({emoji:"🌐️", title:"Hub Spaces, Live Presence and Collaborative Studios", goal:"🎯r2602🎯runningsketchpad🎯runningsketchpadapps", client:"claude-code", llm:"opus-5", prompt:<program goal>})`. Record path `26/08/16/<SLUG>` = `$T`.
2. Create in `$T`: `📌️important.md` (notice to live sessions: what we add/never touch; cleared last), `📋️master-plan.md` (this file), `📋️contract-freeze.md` (§0), `📋️ownership-and-handoffs.md` (§3 lease table + lanes), `📋️worker-brief.md` (template §1.1).
3. Run waves as **batches of `Agent` calls with `run_in_background:false`, ≤6 per batch, all in one function_calls block; never end the turn with children in flight** (memory: children die with the parent turn). Sonnet workers: `subagent_type:"general-purpose", model:"sonnet"`; Haiku scouts/audits: `subagent_type:"Explore", model:"haiku"`. Never `isolation:"worktree"`.
4. After each batch: read every `📓️<lane>-report.md`, run the barrier set (§4) serially, write `📓️w<N>-barrier.md` (pass/fail, `git log --date=iso` attribution for every red, open `sharedFileRequest`s), then launch the next batch.
5. Red baseline mid‑wave: attribute (`git log --date=iso -- <file>`); peer in‑flight ⇒ keep lanes on unit‑level checks, poll every 15 min; ours ⇒ one narrow remediation Sonnet; never edit peer files; >2 red barriers on the same peer blocker ⇒ report to dev.
6. Close: all reports present, no `.log` in `$T`, registry `check` green, `📓️final-summary.md` (browser‑proven vs unit‑proven, honestly), empty `📌️important.md` (0 bytes, keep file), `ticket_close({path:"26/08/16/<SLUG>", summary, files:[…real paths, ≥1 non‑emoji‑only name…]})`.

### 1.1 Worker brief (embedded in every worker prompt)
Ticket folder `$T`; read `📋️contract-freeze.md` + your lane row; CLAUDE.md binds; edit ONLY inside your lease, otherwise STOP and write a `sharedFileRequest:` block (file, region, exact change, why) in your report and continue; re‑read the region before every `Edit`, never whole‑file `Write` on existing files, never revert foreign changes; NO git‑modifying commands, NO worktrees; **NEVER call `ticket_close`/`ticket_reopen`/`ticket_open`**; scratch/logs in `$T` as `.txt` (never `.log`); report `$T/📓️<lane>-report.md` with Changed files / Commands run + counts / Blockers (file:line + attribution) / sharedFileRequests; run only scoped checks (`cargo test -p <crate> --lib`, `bun nx run <project>:test`), never `cargo check --workspace`; `[DEBUG] ` prefix on temp logs, remove before reporting; if cargo blocks on the target lock, wait, don't kill; docstrings start with an emoji, regions `//#region 🔖️X`, en+de strings, schema‑first, Rust+TS twins.

---

## 2. Waves and lanes

Model key: **S** = Sonnet 5 worker, **H** = Haiku 4.5 read‑only, **O** = Opus coordinator. ≤6 concurrent workers per batch. Each lane: files it owns → exit criteria → verify → report `📓️<lane>-report.md`.

### W0 — baseline gate, contract, scouting (O + 3H + 2S, ≈1–2 h)
| Lane | M | Owns / does | Exit / verify |
|---|---|---|---|
| 0‑C | O | ticket + master files (§1); read peer `📌️important.md`/`📋️ownership-and-handoffs.md` of `MUTATION-OUTCOMES…` and `FULL-STDIO…`; decide C9 sequencing (§3) | `📓️w0-barrier.md` with §4.1 baseline results |
| 0‑S1 hub scout | H | `🌎️hub/**`, `🛢️db` public API (`create_document`, `catalog`, `health`), `bin.rs` test helpers `spawn_server/test_state` | `📓️scout-hub.md` |
| 0‑S2 client scout | H | ShellHost/Shell hub binding & actor sites, `PluginRuntime` `AppChannelClient` actor, `ProgramBridge` actor, `openArtifactWithAppRef`, presence heartbeat, `applyShellUri` routing, wgpu `🟦️boot.ts` env | `📓️scout-client.md` |
| 0‑S3 gate scout | H | dev `📜️script.ts` insertion points (`VerifyScript`, `DevScript`, `🔖️SpaceE2eVerify`), registry generate/check, `verify gate` current red steps (`🧪️w0-gate-baseline.txt`), taxonomy leaf requirements for new dirs | `📓️scout-gates.md` |
| 0‑A directory schema + fold | S | ★`💻️os/🔨️modules/📇️directory/{🧬️schema/*,🦀️component.rs,🟦️component.ts}` (events/commands/DTOs/read model/`fold`, JSON codecs), ★`💻️os/🧫️fixtures/📇️directory/🧾️events.json`, glue entry in `💻️os/📦️packages/🦀️rust` + re‑export in `💻️os/🟦️component.ts` (new region only) | `cargo test -p semio-framework-os --lib directory_fold_*`; vitest parity on the fixture |
| 0‑B ports/env fixes | S | `📚️library/…/📦️index.ts:1952` (8787), `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (env merge `{...process.env, OS_HUB_PORT: process.env.OS_HUB_PORT ?? …}`), `📦️bin.rs:819` default 8787 (one‑line, re‑read first — peer lease on other regions) | `bun nx run os-hub:dev` logs `:8787` with launch env honoured |

**Barrier W0**: §4.1 items 1,2,6 green ⇒ start W1. Items 3/4 (space native/wasm) red‑by‑stdio ⇒ record; browser lanes run in unit+hub‑test mode until green.

### W1 — hub log/API/admin API; identity + hub binding; space artifact (6S + 1H)
| Lane | M | Owns | Exit / verify |
|---|---|---|---|
| 1‑A hub directory core + backends | S | `🌎️hub/📇️directory/🦀️component.rs` (regions `🔖️Events`,`🔖️Decider`,`🔖️Service` — `DirectoryService{write_lock,broadcast}`; trait: remove `create_space/upsert_membership/remove_membership`, add `append_events/events_since/head_seq/rebuild_projections/get_user/list_members/list_active_sync_sessions/…invites`, `record_sync_session_open(space,doc,surface,user,role,actor)`), `🪶️sqlite`, `🐘️postgres`, `🌐️neo4j` (`🔖️EventLog` + `🔖️Projections` regions; sqlite `BEGIN IMMEDIATE`; seed via events) | `cargo test -p semio-hub --lib --features sqlite -- directory` incl. `event_log_replay_matches_projections`, laws test; pg/neo4j compile + tests where docker available (say so) |
| 1‑B hub REST/WS + presence per surface | S | `📦️bin.rs` **new regions only** `🔖️Directory`, `🔖️AdminAuth`, `🔖️Admin`; `HubState` new fields (`directory_service`, `session_kicks`, `admin_dir`); `handle_ws` (`?surface=` query, presence key `(scope,surface,actor)`, internal `enum Fanout{All,Surface}`, connection open/close → directory stream, force‑close stale sessions at boot); `Presence` arm; router; main; new tests. **Off‑limits**: `submit_commands`, `merge_policy_from_env`, `encode_messages`, `messages_for_error`, `ClientFrame::Commands` arm (peer 2‑E) | tests: `directory_ws_replays_then_streams_live`, `presence_roster_is_scoped_per_surface`, `admin_api_lists_spaces_users_connections_and_kicks`, `admin_loopback_default_and_bearer_when_configured`, `deleted_space_denies_ws_hello`, `auth_sessions_me_roundtrip`, `connection_events_reach_admin_stream` — `bun nx run os-hub:test-quick` → `🧪️w1-b-hub-test.txt` |
| 1‑C identity facet + directory client (TS) | S | `💻️os/🎚️config/🧬️schema/*` (+★`🪪️identity` triads `sign-in/sign-out`), `💻️os/🟦️component.ts` new region `🔖️HubBinding` (`foldIdentity`, `DirectoryClient`, `PersistenceBinding.hub.surface?`), `🟦️backbone-worker.ts` (new request kinds `directory-open|directory-command|directory-close`, `?surface=` on hub WS URL, `identityActorConfig`), vitest | `bun nx run @semio-tech/framework-os:test` (fake‑ws directory client reconnect/resume; identity fold vectors) |
| 1‑D identity + directory client (Rust) | S | ★`💻️os/🔨️modules/📇️directory/🦀️component.rs` region `🔖️Client` (`DirectoryClient` over the sync actor's http/ws transports; native tokio‑tungstenite + ureq‑free hand‑rolled http via existing transport — reuse `🏪️store/🔄️sync` `HubTransport` read‑only, no store regions), identity read API Rust twin, `hub_ws_url` `?surface=` in `🔄️sync/🦀️component.rs:403` (one line, sharedFileRequest if peer touched) | `cargo test -p semio-framework-os --lib identity_* directory_client_*` |
| 1‑E `s.space` artifact | S | ★`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/**` (scaffold `new surface`; schema 5 leaves + snapshot/diff/mutation triads; `SSpaceSnapshot`, 4 mutations w/ `MutationOutcome`; io; examples; viewer/editor stubs compile), `project_space_index_to_collection`, `plugin()` registration in `🪐️space/🦀️component.rs`, `📦️glue.rs`, TS `📦️index.ts` | `cargo test -p semio-s-plugin-space --lib space_index_*` (inverse/absorb laws, `assert_missing_target_is_error`, `assert_fatal_never_applies`, op‑text round trip); `bun ./📜️script.ts policy` 0 breaches |
| 1‑F dev configs generator + build lease | S | `📇️registry/📜️script.ts` (`PlaygroundEntry.userPorts`, parse/emit/check uniqueness), `📇️registry/🖥️launch.ts` (`DevLauncherEntry.users{namePrefixPattern,emailPattern,env}`, placeholder `@generated:s:users`), `🪐️space/📦️packages/🦀️rust/Cargo.toml` `user_ports = { react = [6072, 6073], wgpu = [6067, 6068] }`, `.vscode/🧩️launch.seed.jsonc` (placeholder, `devLaunchers.s.users`, hub launcher env+`serverReadyAction`→`/admin`, ★`🛠️dev🗄️os-hub🛡️admin` 8790, ★compound `🧭️compound🖥️s👥️users🗄️os-hub` = hub+👤️1+👤️2 (order 386.16), gate entry `⚖️gate🌎️collab-e2e`), dev `📜️script.ts` `DevScript`: `S_HUB_URL/S_USER/S_DATA_DIR` passthrough (`VITE_S_*` define in `⚙️vite.config.ts`), **plugin‑build lease** (`target/semio-dev-leases/plugin-build-<variant>.json` `{pid,port,startedAt,registryReady}`, `wx` create, stale if `process.kill(pid,0)` throws; second process waits `registryReady` ≤60 s then serves only, logs `[dev] plugin builds owned by pid …`; released on exit/SIGINT), wgpu `🧊️wgpu/📜️script.ts` env passthrough; then `bun nx run @semio-tech/plugin-registry:generate` | `@semio-tech/plugin-registry:check` green; `.vscode/launch.json` contains `🛠️dev🖥️s👤️1⚛️react`(6072) `👤️2`(6073) `🧊️wgpu` twins; run compound: second console shows the lease log |
| 1‑H audit | H | W1 diffs vs lease table; Rust↔TS parity of directory JSON | `📓️audit-w1.md` |

**Barrier W1**: `cargo check -p semio-hub --all-features -p semio-framework-os -p semio-s-plugin-space`; `bun nx run os-hub:test-quick`; `bun nx run @semio-tech/framework-os:test`; registry `check`.

### W2 — surfaces in both shells, presence, admin page, opening relay (6S + 1H)
| Lane | M | Owns | Exit / verify |
|---|---|---|---|
| 2‑A Home app (plugin, both shells via UiNode) | S | `🗿️artifacts/🏠️home/…/✏️editor/**` + `👁️viewer/**` (table via `build_table_scene`: name, kind, visibility, members, updated, origin hub/local, presence, actions; `HomeConfig += directory: DirectoryReadModel, clientId/clientName`; `HomeConfigMutation::{FoldDirectoryEvent,SetClient}`; ★commands `🆕️create-space`(dialog), `🗑️delete-space`(confirm dialog), `✏️rename-space`, `🤝️share-space`(upsert member/copy invite link), `📇️fold-directory-events`, `👥️presence-heartbeat`; open ⇒ `Navigate("/spaces/{id}")`; local‑only spaces flagged; en/de in `🗣️terminology`; drop VFS scene), `🪐️space/🦀️component.rs` (`SpaceUser{id:"local"}`→identity), `📦️glue.rs`, TS twins `🟦️component.ts` | `cargo test -p semio-s-plugin-space --lib home_*` (row per folded space; create emits `ReplayShellCommand("os.directory.create-space")`; delete dialog‑then‑command; de labels); `assert_viewer_never_mutates::<HomeViewer>` |
| 2‑B Space app (plugin) | S | `🗿️artifacts/🪐️space/…/✏️editor/**` + `👁️viewer/**` (main window `📋️artifacts` table: name, kind, subset, updated, authors, presence; ★commands `createArtifact{kindId,name}` (kind list = editor‑capable `ArtifactKindSpec`s; mint id; `CreateArtifact` + relay `os.open-artifact{artifactRef,role,documentId,spaceId}`), `openArtifact`/`openArtifactWith` (OpeningPreferences respected), `deleteArtifact`(dialog), `renameArtifact`, `📌️panels/👥️members` (invite by email+role, remove, visibility, copy link → `os.directory.*`), `foldDirectoryEvents`, `presenceHeartbeat`; en/de) | `cargo test … space_editor_*`; `assert_editor_and_viewer_share_dialect` |
| 2‑C React shell: identity, auto‑bind, directory relay, routing | S | `🧑‍💻️dev/🟦️component.ts` + `⚙️vite.config.ts` (`VITE_S_*`), `⚛️react/📦️index.tsx` (`bootFrameworkOs({identity})`, **append‑only** i18n block `ui.home.*/ui.space.*/ui.presence.*/ui.checkin.*`), `ShellHost/🟦️component.tsx` (`useIdentity()`; `shellActorIdRef`←minted; `openDocument` default bindings `[hub{baseUrl,spaceId,token,surface},folder{S_DATA_DIR/spaces/<id>}]`; sync‑card kept as override; `os.directory.*` shell commands → `DirectoryClient.command` with offline queue; `os.open-artifact{documentId,spaceId}` → open doc; `applyShellUri` `/spaces/{id}`→space app, `/spaces/{id}/studio`→studio; push directory events into home/space sessions as `foldDirectoryEvents` view_action), `PluginRuntime` (`AppChannelClient` actor param), `ShellHelpers` (`presenceClientIdentity`), ★`🎮️commands/📇️directory-*/🦀️component.rs` (7 OS command id+label leaves) — **ShellHost is peer‑leased for conflict UI regions (2‑D): touch only the listed regions, re‑read before every edit** | vitest in `⚛️react`: boot with identity ⇒ Hello.actor minted; `openDocument` bindings snapshot; `os.open-artifact{documentId}` ⇒ worker `open` with hub+folder; manual: navbar shows email |
| 2‑D wgpu shell: identity, auto‑bind, directory relay, presence consumption | S | `Shell/🧊️component.rs` (`identity` from `S_*` env; `POST /auth/sessions` via transport; `open_document(ref,bindings)` per D4; `Hello.actor`; consume `ArtifactEvent::Presence`→`presence_peers`; `os.directory.*` and `os.open-artifact{documentId}` handling; routing twin), `ProgramBridge/🧊️component.rs` actor, `🧊️wgpu/{📦️bin.rs,🟦️typescript/🟦️boot.ts}` | `cargo test -p semio-framework-os-renderer-wgpu shell_identity_*`; `cargo check -p semio-wgpu-native --features native-bin`; manual native run with `S_USER` |
| 2‑E admin page | S | ★`🌎️hub/🔨️modules/🛡️admin/{🧬️schema/*,🧱️elements/{🛡️AdminApp,🔑️AdminSession,🏛️SpacesPage,🙋️UsersPage,🔴️ConnectionsPage,📄️DocumentsPage,📰️EventsPage,📚️I18n}/🟦️component.tsx,📦️packages/🟦️typescript/{package.json (@semio-tech/hub-admin),📋️project.json (os-hub-admin dev/build/test),📜️script.ts,⚙️vite.config.ts (base /admin/, outDir 📤️dist, proxy /directory,/admin/api,/auth,/spaces → OS_HUB_URL ?? :8787, ws:true),🧪️vitest.config.ts,📦️index.tsx,🌐️index.html}}` (React + framework `🖱️ui` elements: `📊️Table`,`🔘️Button`,`💬️Dialog`,`✏️Input`,`☑️Select`,`🏷️Chip`,`📑️Tabs`,`🪵️Tree`; `DirectoryClient` from `@semio-tech/framework-os`; en/de), `📦️bin.rs` region `🔖️Admin` static SPA serving (`admin_dir` = `OS_HUB_ADMIN_DIR` ?? `concat!(env!("CARGO_MANIFEST_DIR"),"/../../🔨️modules/🛡️admin/📦️packages/🟦️typescript/📤️dist")`, traversal‑guarded like `extension_asset_path`, 503 hint if missing), hub `📜️script.ts`/`📋️project.json` build ordering (`os-hub-admin:build` before cargo in `dev`/`build`) | `bun nx run os-hub-admin:build && os-hub-admin:test` (SpacesPage rows from mocked client; en/de key coverage); `bun nx run os-hub:dev` then `curl :8787/admin` 200 |
| 2‑F presence chrome + `PresenceBar` | S | ★`🖱️ui/🧱️elements/👥️PresenceBar/{🟦️component.tsx,⌨️component.rs}` (avatars via `📻️TableAvatar`, `data-row-id="peer:<actor>"`, `#s-presence-peers`), ShellHost heartbeat region (`surface` in WS URL binding, label = identity display name, peers filtered), wgpu window chrome overlay of `presence_peers` | vitest PresenceBar; `cargo test -p semio-framework-ui`; two‑tab manual: 2 avatars |
| 2‑H audit | H | lease compliance, en/de completeness, no CRUD in directory path | `📓️audit-w2.md` |

**Barrier W2**: §4.1 scoped + **coordinator manual browser proof**: hub 8787 + user1 6072 + user2 6073 up (compound), user1 creates space → user2 home shows it; `🧪️w2-barrier-browser.txt` (console `[DEBUG]`, screenshots). If stdio still blocks wasm: record, proceed.

### W3 — check‑in, opening relay hardening, wgpu parity, e2e (5S + 1H)
| Lane | M | Owns | Exit / verify |
|---|---|---|---|
| 3‑A save/check‑in policy | S | ShellHost history/sync regions (`framework.history.checkpoint`, sync status pill), `ShellHelpers`, wgpu `Shell` history chrome, `#s-checkin` action + message dialog, auto‑checkpoint timers, checkpoint‑on‑close, `touch-artifact` dispatch; `📜️HistoryTable` **only if peer 2‑D idle** (author from identity) — else sharedFileRequest | vitest fake timers (3 edits→idle→1 `commitCheckpoint`; close⇒checkpoint); wgpu `cargo test`; e2e step 6 |
| 3‑B opening relay args (`documentId`,`spaceId`) | S | `🔌️plugin/🦀️component.rs` region `🔖️OpeningCommandRelay` (+tests), `🔌️plugin/🖥️host` relay handling, `📡️spr/🧵️channel` **only if a tag is needed** (CHANNEL_VERSION 11→12, appended tags; single commit; **peer‑leased — coordinator gates this lane on `MUTATION-OUTCOMES` being closed or its `📌️` releasing the region; otherwise the relay carries the ids inside the existing JSON `args` payload with NO channel change — preferred**), TS twin vectors in `💻️os/🟦️component.ts` | `cargo test -p semio-framework-plugin opening_command_relay_*`; vitest golden vectors identical |
| 3‑C collab e2e | S | dev `📜️script.ts` ★region `🔖️CollabE2e` + `VerifyScript` branch `verify collab`, `📋️project.json` target `collab-e2e` (§4.3) | `bun ./📜️script.ts verify collab` PASS → `🧪️w3-c-collab-e2e.txt` |
| 3‑D wgpu parity + native smoke | S | wgpu leaves parity (`data-ui-path` identical to React), parity probe suite entry for `s`, `🧊️wgpu/📦️bin.rs` `--smoke` (boot, sign in, fold directory, print `dumpStructure()` JSON, exit 0), `🧊️wgpu/📜️script.ts native smoke` | `bun ./📜️script.ts parity verify s` (structural PASS or documented BOOT status), `cargo run -p semio-wgpu-native --features native-bin -- --plugin s --smoke` prints rows |
| 3‑E hub bun integration | S | ★`🌎️hub/📦️packages/🟦️typescript/{package.json,📋️project.json (os-hub-ts),📜️script.ts,🧪️index.test.ts}` guarded by `HUB_E2E=1` (spawns hub on free port + temp `OS_HUB_DATA`; two sessions; create studio + upsert member; both `DirectoryClient.stream()` see events; two doc WS `?surface=` ⇒ 2‑peer roster, third surface sees none; admin connections lists both; kick; restart hub ⇒ spaces persist) | `HUB_E2E=1 bun nx run os-hub-ts:test-long` |
| 3‑H audit | H | evidence honesty (every pass has a log with counts), lease compliance | `📓️audit-w3.md` |

### W4 — audit, gates, close (O + 3H, ad‑hoc S remediation)
4‑A taxonomy/CQRS/i18n audit (H) · 4‑B evidence audit (H) · 4‑C lease/peer audit (`git diff --name-only` since W0 ∩ peer leases = ∅ or negotiated) (H) · 4‑I coordinator: `verify gate` no new red step, registry `check`, launch.json regenerated, `📓️final-summary.md`, close (§1.6).

Total ≈ 19 Sonnet lanes (max 6 concurrent), 3+1+1+1+3 Haiku, coordinator barriers between waves.

---

## 3. Region leases vs live tickets (checked at every barrier against peers' `📋️ownership-and-handoffs.md`)
| File / region | Peer | Ours | Rule |
|---|---|---|---|
| `📦️bin.rs` `submit_commands`, `merge_policy_from_env`, `encode_messages`, `messages_for_error`, `Commands` arm, `HubState.merge_policy` | MUTATION 2‑E (working tree modified) | 1‑B/2‑E/0‑B add **new regions/fields/arms/tests only**; `handle_ws`/`Presence` arm edits re‑read before each Edit; if `git log --date=iso -- 📦️bin.rs` shows a commit <30 min old, wait | additive only |
| `🌎️hub/📇️directory/**` | blanket "🌎️hub" claim, untouched | 1‑A | ours; announce in `📌️` |
| `🛢️db/**`, `🏪️store` (all regions), `📡️spr/{🎮️command,⚔️conflict,📜️history,🧵️channel,🧪️testkit,🧾️wire}`, `🌿️vcs` framework, `🗣️dsl/*`, `🔌️plugin` Emit/Exchange/runtime, `🔌️plugin/🖥️host`, `🏃️run`, `🎠️kernel/🟦️component.ts` | MUTATION | — | **forbidden** (3‑B relay region gated, see W3) |
| `💻️os/🟦️component.ts` `AppChannelCodec`/`AppChannelClient` | MUTATION 1‑C/2‑C | 1‑C new region `🔖️HubBinding` only | additive |
| React `ShellHost/ShellSync/Shell/ChromePanels/EventFeedHost/DiffViewHost`, `📜️HistoryTable`, `⚛️react/📦️index.tsx` i18n | MUTATION 2‑D | 2‑C/2‑F/3‑A listed regions only; i18n **appended** block | re‑read + additive |
| `💻️os/🎚️config/**` | MUTATION new triad | 1‑C new `🪪️identity` triad dir | additive |
| `✏️s/🔌️plugins/🗄️stdio/**`, `📜️world.wit` | FULL‑STDIO | — | forbidden; stdio wasm red is theirs |
| `✏️s/🔌️plugins/🪐️space/**` | MUTATION 3‑H fan‑out done (verify in their `📓️w3-h-remaining-report.md`) | 1‑E/2‑A/2‑B | ours after confirmation |
| root `📜️script.ts`, `.vscode/launch.json`, root `📋️project.json` | peer coordinator | 4‑I only; launch.json **only via seed + generate** | |
| dev `🧑‍💻️dev/📜️script.ts`, `📇️registry/*`, `🌎️hub/🔨️modules/🛡️admin/**`, `💻️os/🔨️modules/📇️directory/**`, `🖱️ui/🧱️elements/👥️PresenceBar` | none | ours | |

Protocol on a needed foreign touch: STOP → `sharedFileRequest` in report → coordinator does it if trivially additive and idle by `git log --date=iso` (recorded in our `📌️`), else defers to W4 or redesigns. Never edit a peer's ticket folder.

---

## 4. Verification

### 4.1 Baseline / barrier set (coordinator, serial, outputs `🧪️w<N>-baseline-<n>.txt`)
1. `cargo check -p semio-hub --all-features` — MUST be green.
2. `cargo check -p semio-framework-os-kernel -p semio-framework-os-kernel-db -p semio-framework-plugin -p semio-framework-os` — MUST.
3. `cargo check -p semio-s-plugin-space` (native) — SHOULD; stdio‑attributed red recorded.
4. `cargo check -p semio-s-plugin-space --target wasm32-wasip2` — browser precondition.
5. `cargo check -p semio-framework-os-renderer-wgpu -p semio-wgpu-native --features native-bin`.
6. `bun nx run @semio-tech/framework-os:test`, `bun nx run @semio-tech/framework-renderer-react:test`, `bun nx run @semio-tech/plugin-registry:check`.
7. `bun ./📜️script.ts verify gate` — no NEW failing step vs `🧪️w0-gate-baseline.txt`.
Never `cargo check --workspace` (peer churn).

### 4.2 Unit / integration
`bun nx run os-hub:test-quick` (bin.rs tests §W1‑B + directory backends), `cargo test -p semio-framework-os --lib`, `cargo test -p semio-s-plugin-space --lib`, `cargo test -p semio-framework-plugin opening_command_relay_*`, `cargo test -p semio-framework-os-renderer-wgpu --lib`, `bun nx run os-hub-admin:test`, `HUB_E2E=1 bun nx run os-hub-ts:test-long`.

### 4.3 End‑to‑end `bun ./📜️script.ts verify collab` (dev `📜️script.ts` `🔖️CollabE2e`; nx `collab-e2e`; launch `⚖️gate🌎️collab-e2e`)
Free port triple from 7400–7498 (or `S_COLLAB_{HUB,USER1,USER2}_PORT`); fresh `OS_HUB_DATA`; spawn hub (`OS_HUB_ADMIN_TOKEN=e2e-admin`), wait `GET /directory/spaces` 401/200; prebuild plugins once (mkdir lock, like parity harness) then two `dev` daemons with `SKIP_PLUGIN_BUILD=1`, `SEMIO_PLUGIN=s`, `S_HUB_URL`, `S_USER=user1|user2@semio.dev`, `S_DATA_DIR=<tmp>/u1|u2`; playwright chromium, **two `browser.newContext()`**; page‑error filter as studio e2e. Steps:
1. user1 `#s-home-create-space` (name, kind studio) → `[data-row-id="space:<id>"]`; user2 home shows the same row (≤60 s).
2. user1 opens space → `location.pathname.startsWith("/spaces/")`; `#s-space-share` → user2 email author → user2 opens space.
3. user1 `#s-space-create-artifact` (stdio‑free kind, e.g. `note`) → row `artifact:<id>` on both; both dblclick → editor (`[data-ui-path^="surface.editor"]`).
4. user1 types → user2 sees text; console `[DEBUG] remoteMutations` captured.
5. `#s-presence-peers` shows 2 `[data-row-id^="peer:"]` in both; user1 opens the viewer of the same doc in a new tab ⇒ viewer roster ≠ editor roster.
6. user1 `#s-checkin` (message) → `[data-row-id^="history:"]` count grows on both; `updated` column moves.
7. `GET hub/admin/api/connections` (bearer) ≥2 sessions incl. surfaces; `/admin` HTML lists both users; kick one → its shell shows reconnecting then reconnects.
8. Kill hub, respawn same `OS_HUB_DATA`, user2 reload ⇒ space + artifact rows persist.
Log to `$T/🧪️w3-c-collab-e2e.txt`; exit `PASS: collab e2e verified`.

### 4.4 wgpu (honest scope)
Browser wgpu structural parity via existing parity harness (`parity verify s`, swiftshader) — report BOOT status truthfully; native `--smoke` proves boot + sign‑in + directory fold + widget tree, not pixels. Manual native two‑process check with the wgpu 👤️1/👤️2 launchers recorded with screenshots in `$T`.

---

## 5. Risks / mitigations
- **Peer churn / red tree** (MUTATION‑OUTCOMES open, ~1000 modified files; FULL‑STDIO blocks `s` wasm because `semio-s-plugin-space` depends on `semio-s-plugin-stdio`): scoped baseline, attribution before action, browser lanes degrade to unit+hub proof, escalate to dev after 2 red barriers. Do not touch stdio.
- **`📦️bin.rs` shared with peer 2‑E**: additive regions, re‑read discipline, land after their commit if active.
- **Channel bump collision** (peer 10→11 done): avoid app‑channel changes (relay ids inside JSON args); if unavoidable, 11→12 in one commit, gated on peer close.
- **Two dev processes + cargo lock**: plugin‑build lease; e2e prebuilds once then `SKIP_PLUGIN_BUILD=1`; workers wait on lock, never kill.
- **Ports**: hub 8787 everywhere (fix both 6070 defaults); users 6072/6073, 6067/6068; e2e 7400+.
- **Presence semantics**: `?surface=` out‑of‑band; hub roster per `(space,doc,surface)`, commands still fan to all surfaces; wgpu shell now consumes presence.
- **db has no `delete_document`**: `space.deleted` = directory tombstone + access denial; note as follow‑up `🛢️db` ticket.
- **Loopback‑admin default**: log loudly; production must set `OS_HUB_ADMIN_TOKEN`; never behind a proxy without it.
- **Coordinator lifetime**: foreground batches only; Haiku audit after each batch.
- **Route change** `/spaces/{id}` → space app: update studio e2e expectations (3‑C).

## 6. Critical files
`🌎️hub/📦️packages/🦀️rust/📦️bin.rs` · `🌎️hub/📇️directory/🦀️component.rs` (+`🪶️sqlite/🐘️postgres/🌐️neo4j`) · ★`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/**` · `💻️os/🟦️component.ts` · `💻️os/🟦️backbone-worker.ts` · `💻️os/🎚️config/🧬️schema/**` · `✏️s/🔌️plugins/🪐️space/🦀️component.rs`, `🗿️artifacts/🏠️home/**`, ★`🗿️artifacts/🪐️space/**` · `📺️renderer/🧑‍🎨️engine/🧱️elements/{ShellHost/🟦️component.tsx,Shell/🧊️component.rs,PluginRuntime,ShellHelpers,ProgramBridge}` · ★`🌎️hub/🔨️modules/🛡️admin/**` · `🧑‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` + `⚙️vite.config.ts` · `📇️registry/{📜️script.ts,🖥️launch.ts}` · `.vscode/🧩️launch.seed.jsonc` · `📚️library/📦️packages/🟦️typescript/📦️index.ts:1952` · `🌎️hub/📦️packages/🦀️rust/📜️script.ts`.
