# WP-C11 — Collaboration Between Users In The React `s` Shell

Session 13 slice C11 (successor of C10: handover [📓️wp-c10.md](📓️wp-c10.md), audit [📓️audit-s12-collaboration.md](📓️audit-s12-collaboration.md)).
Ports: hubs 8020–8029, serves 6520–6529. Scripts `wp-c11/`; expendable captures `wp-c11/generated/`; durable data and logs
`.🧬semio/🌐hub/s13-c11-*` (logs `s13-c11-logs/<tag>`). Builds/tests prefixed `nice -n 10`.

## Current-Tree Hub Recipe (C11, 19:4x — for any live-testing slice)

Hub 8021 = catalog B2 (9 packages, 16 creatable kinds) served by a CURRENT-TREE `os-hub` (19:07 build, includes H9 Qd:
directory list / event pages fast). Boot 11 min at load 80.

| what | value |
|---|---|
| binary | `.🧬semio/🌐hub/s13-c11-bin/os-hub-hub-8021` — signed copy of `.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/os-hub` as of 19:07 (source sha256 `e2686559b4c7e2e74f141735a4fd4bb951b5289154217c69124683e7ed9b09d8`). **Copy this file; do not rebuild** — landing slices change hub-native codec crates, a later build may refuse B2 |
| rebuild (only if you must) | `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<your target> nice -n 10 cargo build -p semio-hub --bin os-hub` |
| catalog | clone of `.🧬semio/🌐hub/w2-catalog-b2/trusted-catalog` (generation `f485bf7e…e2e1`): copy `generations/<gen>` (`cp -c -Rp`) + `current.json` into `<fresh data root>/trusted-catalog/` (dirs `0700`) |
| launcher | `zsh .tmp-ticket/wp-c11/c11-hub.sh <port> "/Users/ueli/Documents/semio/.🧬semio/🌐hub/w2-catalog-b2" <os-hub binary> <name>` → data `.🧬semio/🌐hub/s13-c11-<name>`, state `…-state` (`hold.txt`, `status.txt`, `ready.json`, `admin-capability.json`; `touch …-state/admin-request` → fresh admin capability in ≤ 5 s); hold = `c11-hub-hold.ts` detached (setsid). Copy the script and change `NAME`/paths for your slice prefix |
| users | provisioned before boot with `printf '%s' <password> \| OS_HUB_DATA=<root> <binary> credential set --email <e> --display-name <n>`: `user1@semio.dev` / `gm1-local-dev-pass-1`, `user2@semio.dev` / `gm1-local-dev-pass-2`, `user3@semio.dev` / `gm1-local-dev-pass-3` |
| env | none required (credential sign-in on; `OS_HUB_MERGE_POLICY=vigilant` optional) |
| serves | `zsh .tmp-ticket/wp-c11/serve.sh s <port> http://127.0.0.1:<hub port> dev` (join-only, `nice -n 10`) |

## Session 13

| # | Item | Status |
|---|---|---|
| 1 | collab-e2e 10/10 + STEP 14 (writer, draw, puzzle3d peer cursors + selections), presence symmetry, join/leave/expiry on 7800 | 8021 run `c11collab-1` (20:31): STEP 1, 3, 5, 7 PASS; STEP 4/8/11/12/13 FAIL (edits dead: envelope wire, rule 23); 9/10 SKIP (external hub); **STEP 2 FAIL → root-caused + fixed** (a human added to an existing space never received its `space.created`; `access-changed` + Home origin replay, below; live proof after the rebuild); STEP 6/14 see log |
| 2 | Opening flakes of session-12 two-browser runs (S12-3g, "document closed", "target changed", session never switches) | **not reproduced on the current tree**: 8021, 10 two-browser opens (A via the creation saga, B via the Space index row; note ×7, writer ×2, draw ×1) all mounted, 0 "document closed" / "target changed" / never-switched; S12-3g gone (S15 09:4x + C10 route ledger). More samples come with every later run |
| 3 | G-P1-4: B's inspector reflects A's remote edit ≤ ~2 s, live | pending |
| 4 | Durable collaborative undo/redo by two users + viewer read-only + removed member, live | pending |
| 5 | Two-browser matrix over every hub-creatable kind on 7800 (A creates, B opens via Space index, both edit, own undo, presence, reload converges), en + de | 8021 dry run `c11mx2` (note, writer): open/presence/window-focus/A→B/reload PASS; found the author-self-repaint defect (below) + harness gaps (identical writer args for both humans, set-verb undo expectation) → fixed in the harness; **live EDIT legs blocked** since ~20:17 (landing `observed` envelope field: the tree's host refuses B2 guests' batches) until W3's rebuild + publish |
| 6 | Zero-touch clean-state `dev s` / ▶️start with a local hub, timed (after the all-package publish) | waits for W3's `--packages all` publish |
| 7 | Coordinator (audit s13 P1-4): an edit staged during a cut reaches the worker ~5 s after reconnect → verify LD's fix live, drop the outage probe's "stamp relative to the cut" workaround | waits for LD item 3 + restage |
| 8 | Coordinator (audit s13 P2-3): one human's undo would erase a DIFFERENT peer's still-pending edit — define + prove the correct outcome live (law exists since session 11) | pending |
| 9 | Coordinator (audit s13 P1-2): same-field conflict live, viewer role live, peer cursors live | viewer/cursor legs: pending; conflict leg after LD P0-1 + H11/LD Qa restage |

### Infra (pids I started)

| What | pid | Port | Notes |
|---|---|---|---|
| serve `s` dev → 7800 | 15754 | 6523 | `wp-c11/serve.sh`, log `s13-c11-logs/serve-6523.txt` |
| serve `s` dev → 7800 | 15755 | 6524 | log `s13-c11-logs/serve-6524.txt` |
| hub 8021 (B2 catalog clone + 19:07 current-tree `os-hub`, fresh root, user1/2/3 = 7800 passwords + `gm1-local-dev-pass-3`) | hold 27066 / hub 27070 | 8021 | `wp-c11/c11-hub.sh`; data `s13-c11-hub-8021`, state `…-state` (admin capability, `admin-request`); binary `s13-c11-bin/os-hub-hub-8021` (source sha256 `e2686559…`); ready 11 min at load 80 |
| serve `s` dev → 8021 | 34902 | 6525 | log `s13-c11-logs/serve-6525.txt` |
| serve `s` dev → 8021 | 34909 | 6526 | log `s13-c11-logs/serve-6526.txt` |

### Log (session 13)

- 19:07 read AGENTS.md, preambles 13 + 12, `📓️wp-c10.md`, `📓️audit-s12-collaboration.md`, S15 S12-3g, W2 Hub Handoff. Hub 7800 READY
  on B2 (hold 28673 / hub 54029, runId `8d0ec7a5…`). Nothing of C10's is running; ports 8020–8029 / 6520–6529 free. Load 23, 1 rustc,
  111 GiB free.
- 19:09 serves 6523/6524 (dev lane) → 7800. The dev lane is stale against the tree (`[stale] space, stdio, trinity, wfc, writer …
  source-changed`) — expected while the landing runs; hub documents load their module from the hub catalog anyway.
- 19:13 matrix `c11mx1` (7800, note + writer, en): FAIL before any kind — user1's Home never listed the new space. Measured
  (`probe-c11-home.mjs`, `c11-hub-http.ts`): 7800's B2 binary answers user1 (member of ~53 spaces) `GET /directory/spaces` in
  **84 s** and `GET /directory/event-page/v1?after=0` in **17.7 s** (64 KiB, 121 events, `hasMore`); the hub process idles at
  0.3 % CPU. Home shows "Retrying directory update through sequence 0" and lists only the local Demo Studio; the link pill
  flips online ↔ reconnecting every ~10 s. user2 (6 spaces) gets its rows. Root = H9 Qd (whole-log fold per list, per-event
  membership reads, quadratic sealing) — fixed in the tree, not in 7800's 02:31 binary → **every 7800 run with user1 waits
  for W3's restart on a current-tree `os-hub`** (main told 19:4x). Harness: the matrix now finds a new space by NAME in a
  windowed table (`waitNamedRow`, pages every `table-window-scroll`).
- 19:21–19:33 own hub **8021** (see Infra): fresh root, B2 catalog clone, 19:07 current-tree `os-hub` (includes H9 Qd), users
  user1/user2/user3; ready in 11 min at load 80–90 (catalog guest-codec interpretation at ~13 % CPU). Serves 6525/6526 → 8021.
  Home on 8021: `event-page?after=0` answers at once, socket `since=3`, rows in < 2 s.
- 19:4x coordinator notes (audit s13): items 7–9 added to the table.
- 19:36–19:53 matrix `c11mx2` on 8021 (en, note + writer): note — create/open (saga) PASS, B opens via Space index PASS, presence
  2/2 PASS, every window takes focus/commands PASS (0 faults), A→B PASS (hub head 0→2), **B→A FAIL**: B's own view did not
  change (A saw it, head 3); B undo/redo + reload converge PASS. writer — both humans `setText` the SAME pinned text, so B's edit
  was invisible (harness defect); faults `commitCheckpoint refused … action-state-unconfirmed`, `undo refused … action-state-
  unconfirmed`, `redo refused … action-owner-mismatch`. Also measured: every hub note check-in is refused `codec-refused` after
  101–112 s (hub log `server.document.check-in`, both humans; the span drops the underlying `AuthorityError`) and the worker
  polls its status every 100 ms meanwhile (`DOCUMENT_CHECK_IN_POLL_MS`, 245 GETs in 55 s from one human) → routed.
- 20:0x **FOUND — an author sees its own hub-document edit only ~20 s late.** `probe-c11-selfpaint.mjs` (8021, note, ABBA): every
  edit reaches the PEER in 1.6–5.0 s but repaints the AUTHOR's own windows only after 21.5–24.8 s. `[DEBUG] c11` worker trace
  (`c11self4`): the addBlock command (`act seq=5`, 1 mutation, relayed at +0.1 s) is followed by patches for the artifact /
  inspection / history PANELS only — no window patch; the windows (`note-composite@2`, `note-navigator@2`) are patched only in the
  NEXT action's turn (`seq=6`, the shell's check-in `commitCheckpoint` ~20 s later). The guest's command turn does not re-render
  the author's window; a remote delivery does (why the peer is fast). Session 12 masked it: the author then ingested the hub's
  echo of its own batch (a "remote" delivery → window render); since the echo is dropped by identity (WG7) nothing repaints.
  **Root fix (worker, TS host):** `refreshDocumentSurfaces(child, assertCurrent, windows)` replaces `refreshPanelSurfaces` — after
  an app command or any mutating action the actor re-projects every verified WINDOW (own window context, `windowVisibleEvents`,
  shared with `renderSurface`) plus every panel; remote deliveries keep panels only (measured: they re-render windows).
  Unchanged surfaces answer no patch. Law: the actor cold-pair law (`browser document actor transfers one verified cold pair…`)
  now asserts one window re-projection after the mutating early command and one per later app command, none after the
  zero-mutation UI intent. **Run blocked**: the suite dies earlier on LD's in-flight envelope change (test literals lack
  `observed`/`target`, 20:28) → routed via main; live re-proof after the publish.
- 20:2x **FOUND — every hub-document edit now fails** (`c11self5`, writer `setText`): `act error … DocumentBackboneBatchError:
  document backbone batch: observed-flag` at `readDocumentBackboneEnvelopeBatchAtExact` → `action-state-unconfirmed` → actor
  closed → later actions `action-owner-mismatch`. Cause: LD's landing added `observed`/`target` to the envelope wire (host TS
  now), B2 guests emit the old wire. Coordinator: by design (no compat); live EDIT testing waits for W3's rebuild + publish +
  fresh hub roots. Continuing with non-edit legs + host fixes; edit battery scripted for right after the publish.
- 20:0x rule 22 (memory): stopped serves 6523/6524 (pids 15754/15755 + vites 15973/15988); hub 8021 + serves 6525/6526 kept (in use).
- 20:31 collab-e2e `c11collab-1` on 8021 (serves 6521/6522 by the harness; my 6525/6526 stopped for memory): STEP 1, 3, 5, 7 PASS;
  STEP 4/8/11/12/13 FAIL (every edit refused, envelope wire change); **STEP 2 FAIL**: user2's Home was not told live about the
  share. Root cause (`probe-c11-membership-page.ts` over HTTP): a human ADDED to a space created before their directory cursor
  receives only `member.upserted` (page after=47 → `[48 member.upserted]`; the full replay has `[44 space.created, 45, 48]`) and
  `authorizationGeneration` stays 1 → Home cannot build the row until a reload. Also: the Home guest refused an after=0 page under
  the same authority (`frontier-race`), so the worker's existing `rebootstrap-required → wake(true)` path could never succeed.
  (A space created after the reader's cursor was fine — why `c11mx2` passed.) Coordinator: my pushback on bumping
  `authorizationGeneration` accepted (it is the auth-session revocation generation; the worker retires the directory AND every
  open hub document on a change) → schema-first `access-changed`.
- 20:5x–21:2x **ROOT FIX — access-changed (schema-first, hub-emitted, event-derived):**
  - kernel schema `📇️directory/🧬️schema` (Rust `DirectoryStreamMessage::AccessChanged { space_id, change: DirectoryAccessChange }`,
    re-exported; TS twin; JSON schema `oneOf` row, `change ∈ {granted, revoked}`, `spaceId` non-empty).
  - hub `🌎️hub/🏗️bootstrap/🦀️.rs` (H11 informed, their 3 signatures untouched): the GLOBAL directory socket's replay and live loops
    follow every event with the frame `directory_access_change_for_reader` owes the socket's own reader — a grant/redemption naming
    it for a space whose `space.created` this socket never delivered (`created_on_socket`), every removal naming it (the removal
    event itself is skipped for the removed reader, so it was never told); the frame borrows no space authority (space-less,
    admitted like the reader's own events), never goes on the shared bus (`directory_message_visible` → false).
  - consumers: worker `directoryStreamWakeV1` (new, `📇️directory/🟦️.ts`) → `origin` for `access-changed`/`rebootstrap-required`;
    wgpu Shell → `rebootstrap = true`; MCP remote → revoke on its space's `revoked`, descriptor refresh on `granted`; kernel
    directory client `track` ignores it.
  - Home guest (`✏️s/🔌️plugins/🪐️space/…/🏠️home/…/🎚️config/🦀️.rs`, S16 agreed, one hunk): a page with `after_seq_exclusive == 0`
    rebuilds the projection under any authority; an incremental page must continue the held frontier (`frontier-race`).
  - Laws: shared fixture `🌎️hub/🧫️fixtures/🔑️directory-access-changed-v1` (9 cases, 5 wakes). TS `🔑️directory-access-changed`
    (Ajv admits every message/frame + 2 hostiles refused; independent reference of the reader rule; worker wake per row) **3/3 PASS**
    (`generated/law-access-changed-ts-1.txt`). Rust (written): hub bin-unit `a_membership_event_naming_the_reader_owes_it_exactly_
    the_fixture_access_change` (pure core over the fixture + wire shape), socket law `a_reader_added_to_an_older_space_is_told_its_
    access_changed_and_its_own_new_space_is_not` (sqlite; grant → frame after the event; own space → none; removal → revoked), and
    the existing global-revocation law now expects `access-changed {A, revoked}` before B's event. Home guest law (apply-directory-
    event-page unit): the old "same-authority after=0 is stale" assertion became after=3 → `frontier-race` + an origin-replay rebuild
    case. Checks: space-home native `--lib --tests` rc=0 (124 warnings) and wasm32-wasip2 `--lib` rc=0 (before the kernel variant);
    kernel + MCP + hub + wgpu `--lib --tests` on build-fleet-b running.
