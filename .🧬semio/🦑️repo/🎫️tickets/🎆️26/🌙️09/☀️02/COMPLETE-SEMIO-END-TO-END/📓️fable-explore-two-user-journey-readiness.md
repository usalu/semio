# Two-User Collaboration Journey — Dispatch Packet (reconciled 2026-09-05 ~21:40)

Lane `fable-explore-two-user-journey-readiness`. Read-only: no cargo/nx/bun run, no edit, no subagent.
Every "verified" claim below was checked directly against current source at read time; everything
else is explicitly marked as inference from a report or as a `terra` design packet (not an
implementation record). File paths use exact current line numbers I read myself; a few move as the
tree is being edited live by other sessions, so treat line numbers as "true at ~21:40", not eternal.

## 0. Headline reconciliation — the residual graph is stale in the good direction

`📓️terra-three-pillar-current-residual-execution-graph.md` (written **01:40**) and
`📓️terra-directory-event-page-two-process-journey-p0.md` (**03:26**) are the two documents this
dispatch was asked to treat as the current residual graph. Both predate a full day of Sol/Fable work.
Direct source checks show several of their "Still RED" verdicts are now **fixed at source**:

| Terra verdict (RED) | Current source finding | Verified how |
|---|---|---|
| "no invite redemption transaction... reusable indefinitely" | `hub_space_invite` claim is now a conditional `UPDATE … WHERE accepted_at IS NULL AND accepted_event_id IS NULL AND revoked_at IS NULL AND expires_at > ?2` at `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1412` | read exact line |
| "scoped directory socket revocation not implemented" | `SocketAudienceV1::DirectoryScoped(DocumentScope)` fully wired: `issue_scoped_directory_socket_grant` (`🚀️bin.rs:2620`), `directory_scoped_ws_v1` (`:4863`), binding-validity fence at `:2144` | read exact lines |
| "presence has no lease/expiry, ghost forever" | `PRESENCE_LEASE_TTL_MS = 15_000` at `🚀️bin.rs:641`; `socket_live_id`/`expires_at` fields on the presence slot (`:436`); `refresh_presence`/`expire_presence_for_live`/`close_presence_for_live` at `:1648/1689/1706` all compare `socket_live_id` before mutating | read exact lines |
| "Home/Space cannot ingest a bounded ordered directory page" | `applyDirectoryEventPageBootstrapV1` is called from `ShellHost/🟦️.tsx:1949` inside `handleDirectoryEventPageRef`, gated on `message.bootstrapEpoch !== owner.bootstrapEpoch`, and on success calls `refreshDirectoryHomeRef.current(current)` | read exact lines |
| "hub owns no inference route" | `/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs{,/…/events,/…/cancel,/…/approval}` are registered routes (`🚀️bin.rs:6332-6335`), `InferenceJobLedgerV1::open` is constructed in `HubState` init (`:6587`) | read exact lines |
| "browser rejects the only Map target because it hard-requires `react`" | `execution-target-lease` lane replaced that hard assertion; `documentOpenPlanAuthority` now admits `rendererTarget !== "react"` only when a live `DocumentExecutionTargetLeaseFieldsV1` lease is supplied, routing the verified `wasm` GIS target to an explicit `renderer-unavailable` terminal instead of a hard reject (`📓️fable-execution-target-lease.md` §Browser slice item 6; browser Worker tests pass per its Evidence section) | report + cross-checked route/schema exists in `🧬️schema/🦀️.rs` |

**What is genuinely still true from the residual graph:** no lane has produced a native (`cargo`) or
process pass for *any* of the above during this reconciliation window except where noted in §3. Every
row above is "fixed at source", not "proven by compiler/runtime". The distinction matters for the
gate design in §2.

Two compile-blocking incidents recorded in `📋️master-plan.md`'s Fable Fleet Wave section are both
**resolved in-tree**, with real (non-source-grep) compiler evidence from other lanes' sessions today:
`DirectorySpaceDetailV1` retirement fallout (resolved 13:21) and `Effect::RequestInferenceProposal`
non-exhaustive match in `🔌️plugin/🖥️host/📥️imports/🦀️.rs` (resolved 18:10, confirmed by
`📓️fable-gis-map-inference-ui-port.md`: `semio-framework-plugin-host --lib` 0 errors in 56 min, and
`semio-framework-os-kernel --lib` 0 errors). I independently confirmed the match arm exists at
`🔌️plugin/🖥️host/📥️imports/🦀️.rs:544` and that `directory_command_sha256`/`DirectoryCommandOutcomeV1`/
`DirectoryCommandResultV1` are exported from `📇️directory/🦀️.rs:29-31` (the symbols
`📓️fable-execution-target-lease.md` reported missing for the kernel test binary).

**What is NOT resolved:** `✅️acceptance-matrix.md`'s tail (edited ~21:30, the most recent write in the
whole ticket) records a **third**, currently-open compile risk: `fable-coordinator-repairs.md`
(21:05) hit `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:1249 E0308 expected
&WalWriterPermit, found &ArtifactId` from "the Sol fleet's live WAL-writer-permit packet (every db
module rewritten 20:10–20:35)". I re-read that file just now; line 1249 no longer contains that
pattern (only one `WalWriterPermit` occurrence remains, in a test helper at line 176) — the WAL fleet
is still actively rewriting these files, so **I cannot confirm from source alone whether `os-hub`
currently compiles**; the last concrete signal (21:05) says no, and nothing later in the ticket
re-confirms green. The acceptance matrix's own final row says exactly this: `"Hub native
qualification run | lane report pending | gate 3 (invite redemption native) building in a
lane-private target | no runtime evidence yet"`. **`📓️fable-hub-native-qualification.md` does not
exist on disk** — that lane (tasked with running presence-lease/invite-transaction/ordered-
publication/event-page/admin-journey native+process gates) is still in flight as of this read; do not
wait on it silently, but do not assume it will land green either.

## 1. Step-by-step journey table

Journey: create space → invite/add member → both open the same document → edit → presence →
reconnect → admin removes member → 4401.

| # | Step | Exact route / worker op / UI action that exists today | Evidence status | Blocker if any |
|---|---|---|---|---|
| 1 | Create space | `POST /directory/commands` → `DirectoryCommand::CreateSpace`, `post_directory_commands` (`🚀️bin.rs:4066`), now routed through `DirectoryService::execute_idempotent` (durable idempotency keyed on `(actor_user_id, request_id)`, `📇️directory/🦀️.rs`) | Source-verified (route+dispatch exist); TS unit tests green (267/267, `📓️fable-directory-command-receipt.md`); **zero Rust compile** for this lane's own code (explicit nonclaim) | No native/process run of the idempotency path itself |
| 2 | Invite / add member | Two paths exist: (a) `DirectoryCommand::CreateInvite` + `POST /directory/invites/{token}/redeem` (`post_redeem_invite`, atomic claim fixed, `🪶️sqlite/🦀️.rs:1412`); (b) direct `upsert-member` via the same command receipt route, or via the new Administration pane (`🛂️SpaceAdministration/🟦️.tsx`) | (a) redemption transaction fixed at source, exercised by 21 neutral traces + AJV in `📓️sol-invite-redemption-transaction.md`, **no native/container run credited** (Stdio BRep taxonomy blocker recorded there is now itself stale — see note below); (b) `space-administration-check source` green, 41 checks, **native laws written, not run** (`📓️fable-space-administration.md` nonclaim 2) | Terra's P0 explicitly recommends starting with (b) direct-member to avoid the still-heavier invite path; that is now source-ready but process-unverified |
| 3 | Both open the same document | `POST /spaces/{s}/documents/{d}/open-plan` (`issue_document_open_plan`, `🚀️bin.rs:2348`) → execution-target asset routes (`issue_document_execution_target_{manifest,component,descriptor}`, `:2547` region) → `GET /spaces/{s}/documents/{d}/socket/v1` (`document_ws_v1`) | Lease contract + browser Worker path source/browser-tested (`📓️fable-execution-target-lease.md`: 3 browser Playwright/Vitest tests pass, 66s+131s runs); **the two hub-side native laws for asset routes are written, registered, never executed** — blocked serially by the two incidents above, now also possibly by the WAL-fleet churn | Terminal state is honestly `renderer-unavailable` for the GIS Map wasm target — no WGPU/React pixel is rendered even when the lease succeeds. This is a deliberate, accepted downgrade, not a bug. |
| 4 | Edit | `DirectoryCommand`-driven mutation via administration pane, or a document-socket batch (existing `DocumentSocket` commit-then-fanout, `🚀️bin.rs:2985-3001` per `terra-three-pillar`) | Directory-side (member role changes, rename) source+browser tested; **generic document content edit for a real app (e.g. Flow addWidget) is explicitly RED** — `BatchOnlyPendingRewrite` still gates Flow's mutation (`terra-three-pillar` §P1-A) | Any edit that is not a directory-administration command (rename/role/invite) has no landed app-content mutation path today |
| 5 | Presence | Server-stamped lease: `install_presence_slot`/`refresh_presence`/`expire_presence_for_live`/`close_presence_for_live`, all keyed on `socket_live_id` (`🚀️bin.rs:1621-1710`); 15s TTL, one-second `authorization_tick` drives expiry | Source green: 17-vector neutral oracle GREEN (`📓️root-hub-presence-normalization.md`); **native laws implemented and registered but explicitly "have not yet run"** per that same report | Native/process proof pending; this is the strongest "landed today" item in the whole journey and also the least externally verified |
| 6 | Reconnect | Directory client `stream_acknowledged`/`DirectoryStream::acknowledge` (`🔌️client/🦀️.rs:859-868,1120-1143`); browser `DirectoryEventPageBootstrapV1` retry/rebootstrap state machine (`backbone-worker.ts:1764+`) | Both owners exist and are source/unit tested independently; **no test drives them together across one real reconnect** — `terra-directory-event-page-two-process-journey-p0.md`'s 8-step deterministic journey (gap injection, socket disconnect/retry, session rebootstrap, hub restart) is still explicitly unrun | This is the single largest remaining *design-complete-but-unexecuted* gap |
| 7 | Admin removes member | `execute_directory_command_fenced` shared membership-socket fence (`terra-three-pillar` "Directory append" row); scoped-socket route denies re-acquisition after removal per `📓️sol-scoped-directory-socket-membership-revocation.md` | Source+TS green (`77268` session: AJV/oracle 19/19, hostile schema 3/3, client-close 3/3); **native/process pending**, blocked historically by a Stdio BRep taxonomy include that I could not find any remaining reference to in `✏️s/🔌️plugins/🗄️stdio` — likely already repaired by the concurrent emoji-uniqueness taxonomy pass, but I did not re-run the gate to confirm | Native gate `os-hub:scoped-directory-socket-native-check` unrun; presumed-cleared blocker needs one real attempt to confirm |
| 8 | 4401 | `SocketAudienceV1::DirectoryScoped` binding-validity fence (`🚀️bin.rs:2144`) returns terminal `Revoked(scope)` never entering client reconnect backoff, both TS and Rust directory clients preserve the close code (`📓️sol-scoped-directory-socket-membership-revocation.md`) | Source/TS-verified only; native/process unrun (same blocker note as row 7) | — |

## 2. Minimal process gate — extend the existing hub runner, do not invent a second hub

**Do not build a new harness.** `startLocalHub` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:696`) and
`issueLocalCredential` (`:794`) already give you: a spawned real `os-hub` child process bound to
`127.0.0.1`, SQLite+FS storage when `isolatedSecuritySmoke: true`, and a private-channel credential
mint for up to 8 profiles per run. `proveDirectoryEventPageV1Process` (`:6521`) is the closest existing
analogue of the exact two-identity flow you need — it already does: spawn hub → mint A and B via
`issueLocalCredential` → `fetchLiveDirectoryEventPage`/`submitLiveDirectoryCommand` (`:5084-5129`) →
restart the hub on the same data root and re-read. `proveAdminLiveJourney` (`:2272`) is the existing
real-headless-Chromium + real-hub pattern to reuse if/when a browser leg is added (per
`terra-directory-event-page-two-process-journey-p0.md`'s own recommendation, item 4 of its "Smallest
Executable Packet").

**Concrete minimal extension** (one new exported prove-function beside `proveDirectoryEventPageV1Process`
in the same `📜️script.ts`, registered the same way — new `os-hub:two-user-collaboration-journey-check`
target in `📋️project.json` + `.vscode/🧩️launch.seed.jsonc`, ticket-local `SEMIO_TEST_ARTIFACT_DIR`):

1. `startLocalHub(repoRoot, root, [], { isolatedSecuritySmoke: true })` — SQLite+FS, no trusted catalog.
2. `issueLocalCredential` twice → A, B.
3. A: `POST /directory/commands` `CreateSpace`, then `upsert-member` adding B as `author` — reuse
   `submitLiveDirectoryCommand` (`:5113`), now returning a `DirectoryCommandReceiptV1` per the
   receipt-transport lane, not the old `DirectoryCommandResponse`.
4. A and B: `POST /spaces/{s}/documents/{d}/open-plan`, then the three execution-target asset routes,
   then `GET …/socket/v1` — **honest skip**: do not attempt to reach a rendered surface. Assert only
   the lease fields match `sameLeaseFieldsV1` and the terminal is `renderer-unavailable`, because the
   verified GIS wasm target has no browser/WGPU renderer wired yet (`openPlan=true` but
   `render=false`, distinct from the prompt's suggested `openPlan=false` — the plan/lease legs *do*
   succeed today; only pixel rendering does not exist).
5. A and B: send `ClientFrame::Presence`; assert roster visibility, then silence B and assert expiry
   at `PRESENCE_LEASE_TTL_MS` using the existing pattern from `📓️root-hub-presence-normalization.md`'s
   test-only tick gate (do not sleep-wait 15s; the source already has an injectable clock — reuse it,
   don't reinvent a wall-clock wait).
6. Disconnect B's document socket, have A commit one further command, reconnect B, assert exactly-once
   delivery via `stream_acknowledged`'s frontier law.
7. A removes B via `execute_directory_command_fenced` (`remove-member`); assert B's live scoped socket
   receives 4401 and cannot re-acquire a grant.
8. Restart the hub on the same data root (`finishLocalHub` + fresh `startLocalHub` same `dataDir`),
   confirm durable history for A, denied session for B.

**Steps that must be honestly skipped, and why:**
- **Rendered document content / any app-level edit.** No app (Flow, GIS Map) has a landed content-
  mutation factory wired to the document socket that this journey can drive; Flow's `addWidget` is
  still `BatchOnlyPendingRewrite` (`terra-three-pillar`). Skip with an explicit `contentEditSkipped:
  "no app content factory landed"` marker in the fixture, not silence.
- **WGPU/React pixel rendering of the GIS Map target.** The lease succeeds; the renderer terminal is
  `renderer-unavailable` by design. Do not claim `render=true`.
- **Invite-token redemption leg.** Prefer direct `upsert-member` (step 3) exactly as
  `terra-three-pillar` recommends, to avoid conflating this journey's pass/fail with the separately-
  owned invite-secret-delivery packet (which still has no replayable-token vault per
  `📓️fable-directory-command-receipt.md` nonclaim 1).
- **PostgreSQL/Neo4j.** SQLite only; both other backends are explicitly "source-only, never compiled
  or executed" per `📓️fable-space-administration.md` nonclaim 4 and `📓️sol-invite-redemption-
  transaction.md`'s "Native laws staged, not credited" section.

## 3. Rust-compile-evidence blockers vs. real missing code

**Purely a compile-evidence gap (source is written and believed correct, just never run through
`cargo` in this exact combination):**
- Directory command receipt transport: hub route, three backends, native client, WGPU queue — "none
  of this lane's Rust has ever been compiled" (`📓️fable-directory-command-receipt.md`, its own
  strongest nonclaim).
- Space administration: hub route, three backends, WGPU driver, space plugin — only
  `semio-framework-os-kernel` compiled clean (private target dir); hub route/backends/WGPU/plugin
  never reached a compiler (`📓️fable-space-administration.md` nonclaim 1).
- Execution-target lease: all three registered native laws written, zero executed — first blocked by
  the (now-fixed) plugin-host non-exhaustive match, then by the (now-fixed) kernel test symbols
  (`📓️fable-execution-target-lease.md`).
- Presence lease: source green (17/17 neutral vectors), native laws "implemented and registered but
  have not yet run" (`📓️root-hub-presence-normalization.md`).
- Scoped-socket revocation and invite-redemption-transaction native/process laws: registered, blocked
  historically by a Stdio BRep taxonomy include path that I could not find any remaining trace of in
  current `✏️s/🔌️plugins/🗄️stdio` sources (searched: no `brep`-named files under that tree at all) —
  **this reads as already repaired**, but I did not run the gate, so treat it as "presumed clear,
  unconfirmed" rather than fixed.
- `fable-hub-native-qualification`'s whole purpose was to burn down this exact list; its report does
  not exist yet, and the acceptance matrix's last line says it is mid-way through gate 3 of what is
  presumably 5.

**Real missing code (no amount of compiling will make it pass, because the feature is not written):**
- Flow's `addWidget` retained-child factory bind (`BatchOnlyPendingRewrite` is a real gate, not a
  build artifact) — needed for step 4 (edit) of any content-bearing journey.
- Any rendered surface for the GIS Map execution-target lease (WGPU renderer target matching `wasm`
  does not exist) — step 3's terminal is honestly `renderer-unavailable`, not a build gap.
- The 8-step deterministic reconnect/rebootstrap/gap-injection/restart journey in
  `terra-directory-event-page-two-process-journey-p0.md` has no test code at all yet (not written,
  not blocked) — it is the largest real gap, independent of any compiler.
- Replayable invite-token delivery (confidential receipt vault) — explicitly not designed yet
  (`📓️fable-directory-command-receipt.md` nonclaim 1).
- The one open question I could not resolve from source alone: whether `cargo check -p semio-hub
  --bin os-hub` currently succeeds at all, given the in-flight WAL-writer-permit rewrite noted at
  21:05 in `📓️fable-coordinator-repairs.md`. This is compile-evidence-shaped, not feature-shaped, but
  it gates literally every native/process step above, so it is the single highest-leverage thing for
  an implementer to check first, before writing any new test.

## 4. Dependency-ordered file list for an Opus implementer

Ordered so each step's compiler run also re-validates the previous step's untested Rust.

1. **First action, before writing anything:** `CARGO_TARGET_DIR=<private> cargo check -p semio-hub
   --bin os-hub --all-features --tests --message-format=short` from a clean private target dir. This
   single command settles §3's open question and should be re-run after every subsequent step below
   before trusting its own tests. Expected collision risk: the Sol WAL-writer fleet is still actively
   rewriting `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/**` (per `terra-wal-writer-guard-*` and
   `root-wal-*` reports dated up to 20:58 today) — coordinate timing or expect transient breaks
   unrelated to this journey.
2. `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` — add the new `os-hub:two-user-collaboration-journey-check`
   prove-function beside `proveDirectoryEventPageV1Process` in `📜️script.ts` (not `bin.rs` itself,
   unless a new test-only route/law is needed — none should be). **Small.** Collision risk:
   `fable-hub-native-qualification` is concurrently running native gates against this same binary;
   coordinate target-dir naming (memory note: lane-qualified `CARGO_TARGET_DIR`, not a shared
   `hub-target` default — two lanes already collided on that exact name today).
3. `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `📋️project.json`, `.vscode/🧩️launch.seed.jsonc` — register
   the new gate, mirroring `directory-event-page-v1-check`'s three-phase (`source|native|process`)
   shape. **Small.** No collision risk (append-only regions, but regenerate `launch.json` from the
   seed — never hand-edit it, per repeated peer collisions recorded today).
4. Run the native laws that are "written, not executed" for presence lease, directory-command-
   receipt, execution-target-lease, and space-administration (§3 list) — no code change, just
   execution, in this order (cheapest/most-isolated first): presence-lease native (no
   `semio-framework-plugin-host` dependency per `📓️fable-execution-target-lease.md`'s own kernel-only
   run) → execution-target-lease native → directory-command-receipt native → space-administration
   native. **Zero-to-small** (fixes are typically one missing re-export or symbol, as seen twice
   today). Collision risk: all four touch the same `semio-hub`/`semio-framework-os-kernel` crate
   graph as `fable-hub-native-qualification`; a private `CARGO_TARGET_DIR` per attempt is mandatory.
5. Write the new two-user journey process test itself (§2) in `📜️script.ts`. **Medium** (a genuinely
   new ~150-300 line prove-function plus fixture). Depends on step 4 passing first, since the journey
   test calls the same routes those native laws individually exercise.
6. Only after 1-5 are green: attempt the 8-step reconnect/rebootstrap/restart journey from
   `terra-directory-event-page-two-process-journey-p0.md` (gap injection, socket disconnect/retry,
   session rebootstrap, hub restart, all 8 acceptance laws). **Large** (new WGPU test-mode probe per
   that report's §"Files to Touch", plus a browser variant reusing `proveAdminLiveJourney`'s headless-
   Chromium pattern). Collision risk: this is the same file
   `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs`
   the GIS Map inference UI port lane (`📓️fable-gis-map-inference-ui-port.md`) also touched today for
   its own turn driver — re-read before editing, do not assume it matches this report's line numbers.
7. **Deliberately last, and only if the ticket's scope grows to include content edits:** bind Flow's
   `addWidget` off `BatchOnlyPendingRewrite` (`✏️s/🔌️plugins/🌊️flow/…/✏️editor/🦀️.rs:1922-1976`,
   local mutation law already at `…/🎮️commands/➕️add-widget/🦀️.rs:22-89`). **Large.** This is
   explicitly out of scope for "first runnable two-user journey" per `terra-three-pillar`'s own
   dependency graph (P1-A, after P0-D/E/F) — listed here only because the prompt's collision-risk
   roster named it; do not schedule it before step 6.

Other concurrently-running lanes named in the prompt and their collision surface, for awareness only
(none block the journey above directly): `fable-ai-map-proposal`/`fable-mcp-inference-bridge`/
`fable-gis-map-inference-ui-port` all live under `🌎️hub/💡️inference/**` and
`🔌️plugin/🧬️schema/📜️.wit` — orthogonal to directory/socket/presence files this journey touches, except
the shared `🔌️plugin/🖥️host/📥️imports/🦀️.rs` match arm (already resolved, §0) and the WGPU native-
entrypoint file (step 6 above). `fable-vcs-native-provider` touches only VCS's own package/provider
files — no overlap. The stdio emoji-uniqueness repair (`ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-
REPOSITORY` ticket, separate from this one) has already renamed `🏭️stdio` → `🗄️stdio` and is still
touching stdio artifact paths — expect further path churn there, unrelated to this journey.

## Inference vs. verified evidence — summary

Verified by direct source read at ~21:40 today (line numbers cited above): invite-redemption atomic
claim, scoped-socket audience/route wiring, presence-lease fields/helpers, ShellHost's
`applyDirectoryEventPageBootstrapV1` call site, inference routes + `InferenceJobLedgerV1` construction,
the plugin-host match arm fix, the directory-command symbol exports. Verified by report + spot-check
(not full independent re-derivation): every "N checks green" figure is taken from the named report's
own stated command output, not re-run by me. Inferred/unconfirmed: whether `os-hub` currently compiles
end-to-end (the WAL-writer-permit line has since moved/changed under concurrent edits); whether the
Stdio BRep taxonomy blocker for native invite/scoped-socket gates is actually cleared (no matching
path found, but the gate itself was not re-run); the exact current progress of
`fable-hub-native-qualification` beyond the acceptance-matrix's "gate 3 building" snapshot.
