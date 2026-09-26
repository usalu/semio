# Session 13 — End-to-End Acceptance Ledger (Outcomes 1–4 + AGENTS.md cross-cutting)

Auditor **A13-accept**, Sonnet 5, read-only, foreground, 2026-09-26 ~19:3x–20:1x. No builds/servers/edits performed
beyond this file. Method: read `AGENTS.md`, `📓️session-13-preamble.md`, `📓️fleet-13-agents.md`, the canonical
`.tmp-ticket-0918/📋️g17-acceptance-ledger.md` (older numbering, same ledger *shape* — reused here) and
`.tmp-ticket-0918/📓️status.md` (skimmed for format only, not re-cited — it predates session 13 by many sessions),
and all five session-13 outcome audits in full: `📓️audit-s13-hub.md`, `📓️audit-s13-os-frontend.md`,
`📓️audit-s13-collaboration.md`, `📓️audit-s13-ai-mcp.md`, `📓️audit-s13-landing-inventory.md`. Cross-checked live,
read-only: `curl 127.0.0.1:7800/readyz` (confirms `runId 8d0ec7a5…`, `status:ready`, matches every audit's citation)
and `lsof -nP -iTCP -sTCP:LISTEN` (confirms only 7800/8040/8050 hub ports bound, matching the audits' snapshots).
Harness commands below were read directly out of `.vscode/launch.json` (grep + targeted `sed -n` reads); every
row name given is copy-pasteable into the Command Palette's "Debug: Select and Start Debugging" or `.vscode/launch.json`
search. Where no such row exists the criterion is marked **NO HARNESS**.

**How to read the tables.** Columns: criterion (observable/measurable) → harness (exact command / launch.json row) →
required environment → last recorded result (date + evidence path) → session-13 slice → zero-touch? (Z = yes, a dev
can run this from `.vscode/launch.json` alone with no other setup; N = no, needs something launch.json can't provide
— named in the cell).

---

## 0. Live environment this ledger assumes

- **Hub 7800**: catalog B2 (9 packages, 16 creatable kinds), binary from 02:31, hold pid 28673 / hub pid 54029.
  Confirmed live by this audit's own `curl` just now (`runId 8d0ec7a59c2099e5e20881ef06135591`). **This is not the
  all-package catalog** — every criterion below that says "all 34 plugins" is unmet on this hub by construction
  until W3's publish succeeds and 7800 (or a successor) restarts onto it.
- **Other live hubs at audit time**: 8040 (pid 17050, WG9/WG10 lineage), 8050 (pid 79010, WG9/WG10). Session-13 port
  plan (preamble rule 9) reserves 7800 for W3 only; every other slice's hub/serve pair is `80N0–80N9`/`65N0–65N9` per
  slice letter — see the preamble for the full table, not reproduced here.
- **Credentials / session data**: never inline in this ledger. Location: `.🧬semio/🌐hub/s13-<slice>-<name>/`
  (mode `700`, confirmed by `ls -la` this pass — e.g. `.🧬semio/🌐hub/hub-dev/`, `…/dev-hub-leases/`). Local
  bootstrap admin-relay credentials live under the hub's own `hold`-directory, never checked into git.
- **wasm32 build serialization**: `zsh .tmp-ticket/📜️fleet-mutex.sh wasm <slice> -- <cmd…>` — FIFO, one at a time,
  ≤25 min/hold (preamble rule 4). Any criterion whose harness touches a guest crate must go through this; it is
  **not** a launch.json row (session-scoped, ticket-local script) — flagged N below wherever it applies.
- **Disk guard**: pid 717 (lock-aware, preamble rule 19). Below 80 GiB free, no builds start.

---

## 1. Outcome 1 — working os `s` frontend, all plugins and artifacts

| # | Criterion | Harness | Environment | Last result (date, evidence) | Slice | Z? |
|---|---|---|---|---|---|---|
| 1.1 | Every plugin/kind **opens locally** (staged, no hub) | `.vscode/launch.json` → `🔁️rebuild-all🔌️plugin-registry` (`bun nx run @semio-tech/plugin-registry:rebuild-all`), then the editor/viewer matrix (see 1.6) | staged tree only, no hub | S15 matrix, 75/75 editors + 70/70 viewers PASS — but against the **session-11** restage4 tree (`📓️wp-s15.md`); tree has since moved through a second restage2→4 cycle in session 12 (`📓️wp-w2.md:446-465`). **Treat as plausible, not proven**, per `📓️audit-s13-os-frontend.md` §0.3 | S16 | Z |
| 1.2 | All **34 plugins / ~89 open-creatable kinds** are live-creatable on a hub | `🚚️publish-catalog🗄️os-hub` (`bun nx run os-hub:trusted-catalog-bootstrap --packages all`) | hub bound to the resulting catalog; wasm mutex for the guest builds inside the publish | **FAILED 4×**: 06:34 exact-pin, 11:43 usage-cut, 15:54 disk-guard self-inflicted deletion, **18:45 imperative wasm codegen — current blocker** (`browser actor artifact: unsupported import interface`, `🔌️plugin/🌐️browser-bundle/📜️script.ts:317`, `📓️session-13-preamble.md:19-22`). Only **9/34 packages (16/~89 kinds)** live on 7800 right now | W3 | N — the consolidated `rebuild-all → publish` chain is coordinator-launched (preamble rule 7, `python3 .tmp-ticket/wp-w2/w2-detach.py`), not a single dev-run row; a dev CAN launch the individual `🚚️publish-catalog🗄️os-hub` row but it will fail without W3's codegen fix landed first |
| 1.3 | Hub a dev would open **today** carries session-12's execution-target fixes (no stale binary) | manual: `curl 127.0.0.1:<port>/readyz`, diff `runId`/build time against the latest landing | 7800 restarted after landing + publish | **FAILED as of this audit**: 7800 confirmed live (`curl` this pass) still on the **02:31 binary**; none of S12-3e/1b/3h are on it (`📓️audit-s13-os-frontend.md` §0.2) | W3 (restart is inside its planned sequence) | N — no launch.json row does "assert this hub's build postdates landing"; **NO HARNESS**, propose `os-hub:local-bootstrap-launch-check` be extended to assert a build-timestamp/runId floor, or a new `os-hub:build-freshness-check` nx target |
| 1.4 | **Created + opened as a hub document**, per kind (bar b) | `🛠️dev🪐️os-s🔭️foreign-kind` (`bun nx run @semio-tech/framework-os-dev:s-host-foreign-kind-s -- <url> --tag launch <kind-list>`) | a live hub + `s` React serve at the URL passed | 9/16 kinds pass in German on the current 7800 binary (`b4`, `📓️wp-s15.md` §S12-3); other 7 fail `execution-target/component 503` because the fix (S12-3e) isn't on 7800 (§0.2) — re-checkable today, no new work needed, only the restart | S16 (re-run after 1.3) | Z (once a hub+serve are up) |
| 1.5 | **Edited + undo + redone** as a hub document, per kind (bar c) | same row as 1.4 (foreign-kind probe asserts `[0,1,0,1]` per kind) | same | same 9–10/16 rows pass; **writer's own hub-document edit/undo/redo is separately broken** — guest re-announce defect blocks typing (see Outcome 3 §3.1) | C11 (re-run) / LD (fix) | Z once 1.2/fix land |
| 1.6 | en+de + viewer matrix on the **tree about to be published** | S15's own matrix script (see 1.1) | live `s` serve, both locales | Stale — last real run predates the tree's second restage; **no `s` serve is currently up on any dedicated audit port** (`lsof` this pass, confirmed by `📓️audit-s13-os-frontend.md` §1) | S16 | N — S15's matrix driver is a `wp-s15/`-local script, **not** a launch.json row; **NO HARNESS as a permanent command** — propose promoting it to `@semio-tech/framework-os-dev:editor-viewer-locale-matrix` in `project.json` + a launch.json row, per AGENTS.md's "permanent scripts only in `📜️script.ts`" rule |
| 1.7 | **Exported/imported** through the `s` UI itself, per kind (bar d) | none found | — | **No session-12/13 slice measures this axis via the UI** — the only evidence is T12's offline round-trip oracle (plugin-crate level, not through `s`'s own commands) | unowned | **NO HARNESS** — propose `@semio-tech/framework-os-dev:s-host-export-import-<kind>`, same shape as the existing foreign-kind probe (1.4) |
| 1.8 | Architect's own `runAnalysis`/`Report`/`Validation`/`search`/`import`/`export` (and 36 other `BatchOnlyPendingRewrite` commands) are **reachable from a control in the running app** | T12's reachability census tooling (`wp-t12/`) | staged `s` | 42 commands unreachable: space-studio 24, architect 8, cad 5, home 4, animate 1 (`📓️wp-t12.md`, cited `📓️audit-s13-os-frontend.md` §4) | T13 or S16 (unowned per audit) | **NO HARNESS as a permanent command** — census tooling is ticket-local; propose `workspace:verify -- interactivity commands --reachable` alongside the existing `⚖️gate🪆️composed-child-refs` (`bun nx run workspace:verify -- interactivity apps --actions`) gate family |
| 1.9 | No production `unimplemented!()`/`todo!()` outside test files | `/usr/bin/grep -rn "unimplemented!\|todo!()" ✏️s 🧰️framework --include="*.rs"` | none | **CLEAN**: 2 hits, both test-only (`🧰️framework/🔨️modules/🔄️machine/🧪️tests/…`, `…/🖱️ui/🖌️render/🧪️tests/…`) | none needed | **NO HARNESS as a gate** — this is a manual grep every audit re-runs by hand; propose folding into `⚖️gate📦️dependencies` family or a new lint gate |
| 1.10 | No production runtime dependency on an external library (AGENTS.md) | `📦️check🔒️dependencies📃️literal-external` (`bun nx run workspace:verify-dependencies-literal-external`) / `⚖️gate📦️dependencies0️⃣` (`bun nx run workspace:verify -- dependencies literal-external`) | none | **2 open violations, source-confirmed this session**: `✏️editor/🦀️.rs:108` (reasoning wires editor) builds `serde_json` in production; `action-bus::optional_json_to_dsl` takes `serde_json::Value`, flagged `productionDebt` in `🔒️dependencies.json` (`📓️wp-t12.md`, cited `📓️audit-s13-os-frontend.md` §4). Sequence's own violation was fixed this session (proof the gate + fix pattern works) | T13 (unowned before this pass) | Z |
| 1.11 | `.vscode/launch.json` / `🧩️launch.seed.jsonc` registration for every new runnable command | spot-check: `/usr/bin/grep -c '"name"' .vscode/launch.json .vscode/🧩️launch.seed.jsonc` | none | 456 / 300 entries; `rebuild-all` present in both — the one new session-12 chain command checked. **Not exhaustively re-audited** (`📓️audit-s13-os-frontend.md` §5) | unowned, spot-check only | Z |

---

## 2. Outcome 2 — working hub backend (db, presence, auth, observability, operations)

| # | Criterion | Harness | Environment | Last result (date, evidence) | Slice | Z? |
|---|---|---|---|---|---|---|
| 2.1 | `cargo check -p semio-hub --all-features --tests` green on the **current tree** (not a stale binary) | `📦️check🗄️os-hub🚀️launch` (`bun nx run os-hub:local-bootstrap-launch-check`); direct `cargo check -p semio-hub --all-features --tests` | `CARGO_TARGET_DIR` per-slice, `CARGO_INCREMENTAL=0`, `nice -n 10` unless a landing slice | **Never run against the current staged tree**: H9's directory-latency/RW-gate/revocation-fence edits and DB1's whole write-path rewrite are `M`/`MM`, uncommitted, and per H11's own log "no build has compiled them yet" (`📓️wp-h11.md:24-25`, confirmed by `git status` this pass) | H11 (item 1, IN PROGRESS) | Z |
| 2.2 | Full hub/db nextest green on the current tree, incl. H9's new laws | `📦️test🗄️os-hub` (`bun nx run os-hub:test`) / `📦️test🗄️os-hub♾️all-features` (`bun nx run os-hub:test-all-features`) | same as 2.1 | Last clean number is **session-12's own 777/779** (`📓️wp-db1.md:26`) — predates all of §2.1's uncompiled edits; not a measurement of the current tree | H11 | Z |
| 2.3 | `a_withdrawn_delegation_admits_no_agent_edit_after_it_in_either_order` (revocation fence) compiles + passes | same as 2.2, targeted `cargo test -p semio-hub <law name>` | same | Law **written, not compiled** ("compile after publish 4", `📓️wp-h9.md:389-399`); comment confirmed present at `🌎️hub/🏗️bootstrap/🦀️.rs:8492` by direct grep this pass | H11 | Z |
| 2.4 | Document creation on **postgres** and **neo4j**, live | `⚖️gate🤝️two-client-document🐘️postgres` / `🕸️neo4j` (`bun nx run os-hub-ts:two-client-e2e -- postgres\|neo4j`); `⚖️gate📈️document-growth🐘️postgres` / `🕸️neo4j` (`os-hub-ts:document-growth-e2e -- postgres\|neo4j`) | live postgres/neo4j server (see `wp-db2.md`/`wp-db3.md` for how they were stood up) | **CLOSED, live**: `tc19` neo4j 2/2 PASS (6 ms revocation, 300+30 growth); `tc23` postgres all 300×16 KiB edits accepted (689 s), revocation 22 ms (`📓️wp-h9.md:298-354`) | closed — H9 | N — needs a running postgres/neo4j server outside the sqlite default; not zero-touch until that server is part of the devcontainer/compose, per AGENTS.md zero-touch rule |
| 2.5 | `--packages all` publish reaches rc=0; 7800 restarts onto it | same as Outcome-1 row 1.2 | — | same as 1.2 — FAILED 4× | W3 | N (same reason as 1.2) |
| 2.6 | db reopen/greeting-storm bound: 24 grown documents reopening at once meet the 30 s Welcome bound | DB1's throughput/storm-ratio law (fs/sqlite; no dedicated launch.json row found) | live hub under load | g17-style "0-0 missing Welcome" reproduced (`📓️wp-h9.md:420-428`); DB1's two throughput laws (storm ratio) still the only two db nextest reds at session-12 close | DB1 | **NO HARNESS as a launch.json row** — the storm/reopen probe lives only under `wp-h9/`/`wp-db1/`; propose a permanent `os-hub:reopen-storm-check` nx target |
| 2.7 | Idle-release / residency-LRU eviction actually fires under real memory pressure | none published yet | 34-package catalog, RSS watch | **Never exercised** — B2's 9 packages (240.6 MB) sit under the 256 MiB budget; the 34-package catalog (the real test) keeps failing before it gets there (`📓️wp-h10.md:144-145`) | new/unowned per hub audit P1-2 | **NO HARNESS** — propose an RSS-watch wrapper around the post-publish open-plan probe |
| 2.8 | Cold `docker build` for the hub image succeeds; `docker run` answers `/healthz` | no launch.json row found; ad hoc `wp-h10/docker-build-context.sh` + `docker-run-drill.sh` | Docker Desktop, ≥8 GB VM, `CARGO_BUILD_JOBS` tuned | **Never completed** — H10 reached attempt 6, stopped for build-quiet; `docker images` on this host = **zero** (confirmed this pass) | Z3 | **NO HARNESS as a launch.json row** — propose `os-hub:docker-build` / `os-hub:docker-run-check` nx targets so "zero-touch" (AGENTS.md) actually holds for this platform target |
| 2.9 | Native Linux hub build (`cargo check -p semio-hub`) | none found as launch.json row (would run the same `📦️check🗄️os-hub🚀️launch` row, but on a Linux host/container) | Ubuntu 24.04 container or native Linux | Proven once in a container copy with patches B1–B4 applied (`wp-z2.md:70`, 3 m 40 s check / 8 m 27 s build) — **B1-B4 never landed in the real tree** | Z3 | N — devcontainer/Linux build isn't reachable from a macOS dev's launch.json without the devcontainer itself working (see AGENTS.md zero-touch cross-platform requirement, §4 below) |
| 2.10 | Graceful shutdown/restart, live, all 3 backends | part of the same postgres/neo4j gates (2.4) plus a manual SIGTERM drill for sqlite | live hub | **CLOSED, live**: SIGTERM → exit 0.5–2.3 s, 0 live WAL writers left, restart spawned 22/207/31 ms; first-attempt reopen on all 3 backends (`📓️wp-h9.md:34`) | closed — H9 | Z (sqlite) / N (pg/neo4j, same reason as 2.4) |
| 2.11 | Backup/restore drill | no launch.json row found; ad hoc `wp-h10.md` script | live hub, tar | **CLOSED**: SIGTERM 277 ms → tar 164 MB/25 s → restore → ready 13.5 s, byte-identical, 5/5; relocation 5/5 (`📓️wp-h10.md:127-134`) | closed — H10 | **NO HARNESS as a permanent command** — propose `os-hub:backup-restore-drill` nx target so this stays a one-click regression check, not a re-derived script every session |
| 2.12 | Structured trace / observability on `/readyz` and boot progress | manual `curl 127.0.0.1:<port>/readyz` | live hub | **CLOSED, live** this session's own `curl`: schema `semio.hub.readiness/v1`, every feature gate present and matching the declared shape; boot-readiness streams from 68 ms (`📓️wp-h10.md:168-181`) | closed — H10 | Z |
| 2.13 | Hostile-input / malformed-body / general fuzz coverage | none published | — | **UNVERIFIED for 3 sessions running** — only specific enumerated vectors covered, no general fuzz harness | unowned (P2-3, hub audit) | **NO HARNESS** — propose a `cargo fuzz` target wired to a new `os-hub:fuzz-check` nx target once scoped |

---

## 3. Outcome 3 — collaboration between users over the hub

| # | Criterion | Harness | Environment | Last result (date, evidence) | Slice | Z? |
|---|---|---|---|---|---|---|
| 3.1 | Two consecutive local edits (typing) on an actor-relayed document never resend an already-seeded mutation id — text-editing collaboration works | `🛠️dev🤝️os-collab-e2e` (`bun nx run @semio-tech/framework-os-dev:collab-e2e`) | live hub + 2 clients | **BROKEN, confirmed in current source this session**: `flush_apply_outbound` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19169-19195`) re-sends `vcs.edits.last()` on every flush even when `dag.seed_applied` reports `Duplicate` (arm at 19180-19184 is `{}`, swallowed, but the send still happens). Collab-e2e sits at 4/14 steps (`📓️wp-c10.md` "Found 09:4x"). **No patch script exists anywhere under `.tmp-ticket/`** — LD must write it, not just land it (`📓️audit-s13-landing-inventory.md` §4 LD-1) | LD (fix, unwritten) / C11 (re-run) | Z once fixed |
| 3.2 | Late joiner / reopener on **native wgpu** and **wasm32 wgpu** sees full existing history (not 0 of N) | WG7's `s12-echo-suppression-{kernel,hub}-patch.py` (dry-run only; no launch.json row) then re-run of `⚖️gate🔐️hub-auth🧊️wgpu-live-journey` (`bun nx run @semio-tech/framework-renderer-wgpu:hub-live-journey-check`) | live hub + native/wasm32 wgpu shell | **BROKEN on 2 of 3 lanes**: React's echo-suppression-by-identity is landed; the Rust kernel twin (`admit_remote_envelopes`/`note_authored_envelopes`) and the hub twin (`HUB_CATCH_UP_ORIGIN`) are **0 `git grep` hits** — still only unapplied `.py` patches (`📓️audit-s13-collaboration.md` re-verification block) | WG9 (kernel/native) + H11 (hub origin stamp) | N — the WG7 patch scripts are ticket-local, not a permanent apply target |
| 3.3 | Same-field concurrent edit produces a visible, correct outcome on a **vigilant** hub | H9's `wp-h9/codemods/opaque-concurrency/land.sh` (`--dry-run`→apply, no launch.json row) | live hub + 2 clients editing the same field | **UNMEASURABLE**: `causal_head` is 0 `git grep` hits anywhere in the tree — the store-half fix is fully designed, dry-run-clean, but wholly unapplied (`📓️audit-s13-collaboration.md` §0, LD-2 / H9 item Qa) | LD (store half) + H11 (db half) — must land together | N |
| 3.4 | Live viewer (read-only) role enforcement | `c10perm1` harness (`wp-c10.md`-local; no launch.json row) | live hub, 2 identities, 1 with a Spectator/viewer grant | **BROKEN, unretried since session 11** (`📓️audit-s13-collaboration.md` §2 matrix row "Viewer read-only role") | C11 | **NO HARNESS as a permanent command** — propose promoting `c10perm1` into a `os-hub-ts:viewer-role-e2e` nx target |
| 3.5 | Native wgpu ↔ React collaboration: sign-in, create+door, presence, edits, undo, reload-converge | `⚖️gate🔐️hub-auth🧊️wgpu-live-collaboration` (`bun nx run @semio-tech/framework-renderer-wgpu:hub-live-collaboration-check`) | live hub 7800/successor + native wgpu shell + React `s` shell (`🧭️compound🖥️s⚛️react🗄️os-hub`: `🛠️dev🗄️os-hub` + `🛠️dev🪐️space⚛️react`) | **LIVE-PROVEN, 8/8 PASS** on 7800/B2 (`📓️wp-wg8.md` run 12, 17:47–18:01) | closed — WG8 | Z |
| 3.6 | Native wgpu ↔ React **peer-cursor** leg of the same pairing | `⚖️gate🧊️wgpu⏯️native-guest-journey` (`bun nx run @semio-tech/framework-renderer-wgpu:native-guest-journey-check`) or a dedicated cursor law (`a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board`) | same as 3.5, needs a canvas-kind document | **BLOCKED** — block2d (the only live cross-shell kind) has no board; law exists, never run to green | WG10 (recommended, per collab audit P1-5) | N — needs a canvas kind live on the hub first (depends on 1.2) |
| 3.7 | wasm32-wgpu ↔ wasm32-wgpu collaboration (additive, late-join, reconnect) | no dedicated single launch.json row found beyond the wgpu gate family above, run against a wasm32 build | live hub + 2 wasm32 wgpu browser sessions | **LIVE-PROVEN** additive (`s12c` 10/10, `s12f` 20/20) and reconnect (`s12i` 13/13, `📓️wp-wg7.md`); **late-joiner is BROKEN** (0 of N blocks — same root cause as 3.2) | WG9 | N — no single command drives two browser sessions zero-touch yet |
| 3.8 | Any shell ↔ any-**other**-shell-type pairing beyond native↔React (wasm32↔React, wasm32↔native) | none run | 2 different shell types, 1 hub | **NEVER ATTEMPTED** (`📓️audit-s13-collaboration.md` P1-5) | WG9 + WG10 + C11 | **NO HARNESS** for this specific pairing beyond the individual per-shell gates above |
| 3.9 | Reconnect after a 5–20 s connection shortage — no freeze, edits land during the cut, not ~5 s after | outage-probe (part of `wp-c10.md`'s own harness, not a distinct launch.json row) | live hub, simulated network cut | **PARTIALLY BROKEN**: the original DB-side cause is fixed and no longer reproduces; a **second, unlocated** stall exists — a staged offline edit isn't submitted to the worker until ~5 s after reconnect instead of during the cut (`📓️wp-c10.md` 10:0x-10:1x) | C11 | **NO HARNESS as a standalone command** |
| 3.10 | Long offline period → refused (`link-expired`), never silent data loss | `DocumentLinkShortage` law (native kernel test, part of `📦️test🖥️server` family; no dedicated launch.json row) | live hub, long disconnect | **LIVE-PROVEN** (React: `link-expired`; wasm32: long cut expires, never relinks) — `DOCUMENT_LINK_SHORTAGE_POLICY`/`retry_at`/`ceiling_at` already implement the bound-capped backoff in current source (`🏪️store/🔄️sync/🦀️.rs:1100-1260`, confirmed by direct read this session, **contradicting** WG7's own "buggy" log — treat source as correct) | closed | Z |
| 3.11 | Undo/redo of **another peer's** edit | none run | live hub, 2 peers, cross-undo | **LAW-ONLY since session 11** — every live proof is "each peer undoes only their own edit" | unowned (P2-3, collab audit) | **NO HARNESS for the live cross-peer case** |
| 3.12 | Human sees an agent's edit live, roster badge, en+de | `⚖️gate🌉️os-mcp🤝️hub-agent-participant` (`bun nx run @semio-tech/framework-os-mcp-rs:hub-agent-participant-check`) | live hub + MCP agent session + React shell | **LIVE-PROVEN**, en 8/8, de 8/8, screenshots, roster badge (`📓️wp-g10.md` S1) | closed — G10 | Z |
| 3.13 | Session-13 landing window actually lands the prepared session-12 patch sets before any of the above is trusted | `📓️landing.md` `# Session 13 Landing Window` table, one row per landed set (preamble rule 2) | compile-atomic, per-slice | **EMPTY** — header only, zero rows as of this audit's own read | LA/LB/LC/LD/LE | N — this is a manual, coordinator-tracked ledger, not a launch.json row by design |

---

## 4. Outcome 4 — AI integration over the semio MCP

| # | Criterion | Harness | Environment | Last result (date, evidence) | Slice | Z? |
|---|---|---|---|---|---|---|
| 4.1 | `semio` MCP server answers `initialize`/`tools/list`/`resources/list`/`prompts/list` with the official SDK | `🛠️dev🌉️os-mcp🤝️client-e2e` (`bun nx run @semio-tech/framework-os-mcp:client-e2e`); `🖱️mcpinspector🌉️os` (`npx @modelcontextprotocol/inspector --config .mcp.json --server semio`) | `.mcp.json` `semio` server reachable | **CLOSED, well-evidenced**: official `@modelcontextprotocol/sdk` 1.30.0 connected live, 6.2 s, 28 tools listed (`📓️wp-g10.md` S4); MCP TS suite 70/70 | closed | Z |
| 4.2 | `capabilities_search`/`capabilities_describe` return real (non-empty) descriptions against the **live served** gateway | `⚖️gate🌉️os-mcp🚨️capability-audit` (`bun nx run @semio-tech/framework-os-mcp-rs:capability-audit-check`) | live hub/gateway on the **current** descriptor set | **SOURCE FIXED, NOT LIVE** — D1's 782 en+de texts are committed in plugin Rust source, but the committed descriptor `🔣️.json` the gateway reads still has **no `description` key at all** for the same verb (`✏️s/🔌️plugins/🀄️wfc/🔣️.json:18359-18391` vs. source `…/✏️editor/🦀️.rs:756`, both checked directly this session). gis/stdio/vcs have **zero** `.action_describe` calls in source at all (0 grep hits) | LB (frozen patch) + W3 (describe-all/publish) + G11 (re-measure) | N — needs the describe-all/publish chain (same blocker as 1.2/2.5) |
| 4.3 | Prompt-injection / untrusted-content envelope on every artifact export and hub checkpoint | direct source check (`🌉️mcp/🗿️artifact/🦀️.rs:116,127,505,520`) + `⚖️gate🌉️os-mcp🤖️live-agent-loop` / `…🌐️de` | live gateway | **CLOSED, confirmed intact**: en/de "treat as data" tool descriptions + `untrusted_content(&provenance, …)` wrapping; live-agent-loop 26/26 en+de with a canary (`📓️wp-g10.md` S6) | closed | Z |
| 4.4 | Revocation ends an already-minted agent session/binding in **both** temporal orderings, both locales | `⚖️gate🔐️hub-auth🤝️live-sign-in` + the S4 revocation probe (`wp-g10.md`-local script, no dedicated launch.json row) | live hub carrying H9's compiled fix | **DESIGNED, LAW UNCOMPILED, NOT DEPLOYED** — same uncompiled-hub-tree gap as Outcome 2 row 2.3; G10's own S4 run found "refused in en, still edited in de" (asymmetric) against the old binary | H11 (compile) → G11 (re-verify) | Z once compiled |
| 4.5 | Rate limiting on auth/directory-command/invite-redemption/socket-grant routes | source check `🌎️hub/🔐️auth/🚦️rate-limit/🦀️.rs` + wiring at `🏗️bootstrap/🦀️.rs:8677-8699` | live hub | **CLOSED, well-designed, shipped**: 4 route classes, token-bucket, wired and admitted | closed | Z |
| 4.6 | Per-agent-session sustained tool-call volume has a dedicated budget (not just ledger backpressure) | none found | — | **GAP, not previously flagged** — only route-level limits exist; no tool-call-volume throttle once a socket is granted | G11 (its own item 4) | **NO HARNESS** — needs a decision (accept ledger backpressure, or design a new rate-limit class) before a harness can exist |
| 4.7 | Full mutation chain e2e: `prepare→invoke→snapshot→undo/redo→rollback→export`, live | `⚖️gate🌉️os-mcp🤝️hub-edit-durability` (`bun nx run @semio-tech/framework-os-mcp-rs:hub-edit-durability-check`); `⚖️gate🌉️os-mcp💬️agent-reply` (`…:agent-reply-check`) | live hub + MCP agent | **CLOSED, session-12 evidence** (live-agent-loop 21/21) — **not re-run this session**, no servers permitted for this audit | none needed unless regressed | Z |
| 4.8 | 7 `action_invoke` kinds (cad, architect `runReport`/`importProgram`, flow, sequence, space `home`/`importSpace`) that refuse over MCP get a session-13 owner and close | `wp-g10/g10-plugin-coverage.ts` (ticket-local; no launch.json row) | live hub, all 34 packages | **UNOWNED** — tagged "P8 lane parity" in session-12's table; P8 is absent from the session-13 fleet roster (`📓️fleet-13-agents.md`) | **NEW, unassigned** | **NO HARNESS as a permanent command** — propose promoting `g10-plugin-coverage.ts` to a nx target once owned |
| 4.9 | `os-hub` structured trace / metrics vocabulary matches its own README | manual `curl` + `🌎️hub/README.md` diff | live hub | Carried from an older session (G16, `.tmp-ticket-0918`) — `README.md:543` was found stale ("no metrics, no request tracing") vs. reality; not re-checked in session 13's own audits | unowned | Z (once someone checks) |
| 4.10 | wgpu agent-reply decodes AND renders correct pixels | none found beyond decode-level check | live hub + wgpu shell + agent session | **Decode/state-update confirmed (source-verified); pixels never confirmed** | WG9/WG10 or S16 | **NO HARNESS for the pixel-level assertion** |

---

## 5. AGENTS.md cross-cutting requirements

| # | Requirement | Harness | Last result | Slice | Z? |
|---|---|---|---|---|---|
| 5.1 | Zero-touch, cross-platform (devcontainer, native Win/macOS/Linux) | macOS: `📦️check🗄️os-hub🚀️launch` etc. (native, proven). Linux: no launch.json row — proven only in an ad hoc container copy. Windows: none. Docker: `wp-h10/docker-build-context.sh` (not a launch.json row) | macOS: green. Linux: container-only, patches unlanded (§2.9). Windows: **zero live evidence**, static-only. Devcontainer: blocked on the same Linux winit patch + a macOS Docker-Desktop `~/Documents` permission gap outside any agent's authority | Z3 | N for Linux/Windows/Docker — no zero-touch dev-facing row exists for any of the three |
| 5.2 | en + de for every user-facing string, no default language | `⚖️gate🌉️os-mcp🤖️live-agent-loop🌐️de` (env `S_OS_MCP_LIVE_LOCALE=de`); S15's locale matrix (§1.6) | 2 801 static sites swept, 0 identical en=de, 0 English leaks (U5); live matrix 75/75 en + 75/75 de but against a stale tree (§1.6) | S16 (re-run) | Z |
| 5.3 | Accessible UI (WCAG AA, keyboard, mobile/tablet no h-scroll) | `🛠️dev🪐️os-s🤏️pinch-a11y` (`bun nx run @semio-tech/framework-os-dev:s-host-pinch-diagram-contrast-s`) | WCAG AA 18/18, 29 keyboard tab stops, phone/tablet no h-scroll (U5, session 12) — no regression found this pass, not independently re-run | unowned this session | Z |
| 5.4 | Progress + cancellation for every expensive operation | TaskManager / plugin-install-progress checks (part of the S15/U5 matrices, no single dedicated row) | Measured DONE (TaskManager, install progress+cancel, execution-target retry bands, command-stall watch) — unchanged good state | none | Z |
| 5.5 | Local-first, event-driven CQRS+event-sourcing, no CRUD, no CRDT | `grep -rn "CRDT\|crdt" 🌎️hub 🏪️store` (manual, not a gate) | 0 hits, confirmed session 12, not re-run this session (no reason to expect a change) | none | Z (manual grep, cheap) |
| 5.6 | Short connection-shortages don't freeze the app | see Outcome 3 rows 3.9/3.10 | mixed — link-expiry closed, reconcile-after-shortage has an unlocated second cause | C11 | Z once 3.9 harness exists |
| 5.7 | Language-agnostic test + third-party oracle per feature | e.g. `test-parity --case <case>` rows (T12's oracle suite); MCP conformance vs. official SDK (4.1) | Real third-party-oracle evidence exists for MCP conformance and several plugin parity cases; **not exhaustively audited this pass** | spot-checked only | Z |
| 5.8 | `.vscode/launch.json` / `🧩️launch.seed.jsonc` registration for every runnable command | see Outcome-1 row 1.11 | 456/300 entries, one row spot-checked, not exhaustive | unowned, spot-check only | Z |
| 5.9 | No runtime dependency on an external library outside an interface | see Outcome-1 row 1.10 | 2 open violations, 1 fixed this session (pattern proven) | T13 | Z |
| 5.10 | No legacy/compat/shims/deprecations (greenfield rule) | manual code-reading (no dedicated gate) | No violation found this pass (W2's SDK version-pin work explicitly deferred rather than shimmed) | none | Z (manual) |

---

## 6. "NO HARNESS" summary — proposed homes

| Gap | Proposed permanent home |
|---|---|
| Editor/viewer/locale matrix re-run against the current tree (1.1/1.6/5.2) | `@semio-tech/framework-os-dev:editor-viewer-locale-matrix` nx target + launch.json row |
| Build-freshness assertion for a running hub (1.3) | Extend `os-hub:local-bootstrap-launch-check`, or new `os-hub:build-freshness-check` |
| Export/import through the `s` UI, per kind (1.7) | `@semio-tech/framework-os-dev:s-host-export-import-<kind>`, mirroring the existing foreign-kind probe |
| Command-reachability census (1.8) | `workspace:verify -- interactivity commands --reachable`, alongside `⚖️gate🪆️composed-child-refs` |
| `unimplemented!()`/`todo!()` production census (1.9) | Fold into the `⚖️gate📦️dependencies` family or a new lint gate |
| db reopen/greeting-storm bound (2.6) | `os-hub:reopen-storm-check` |
| Residency-LRU eviction at real scale (2.7) | RSS-watch wrapper around the post-publish open-plan probe |
| Docker cold build/run (2.8) | `os-hub:docker-build` / `os-hub:docker-run-check` |
| Backup/restore drill (2.11) | `os-hub:backup-restore-drill` |
| General fuzz/malformed-body coverage (2.13) | `os-hub:fuzz-check` (cargo-fuzz-backed, once scoped) |
| Guest store re-announce fix + law (3.1) | New law under `semio-framework`'s store test module — **must be written first**, no script exists |
| Viewer read-only role e2e (3.4) | `os-hub-ts:viewer-role-e2e` |
| Reconnect-during-cut submit-latency assertion (3.9) | New nx target once C11 isolates the root cause |
| Cross-peer undo (3.11) | New law + live probe once someone owns it |
| Plugin-coverage lane-parity sweep (4.8) | Promote `wp-g10/g10-plugin-coverage.ts` to a nx target |
| Per-agent-session tool-call rate budget (4.6) | Needs a design decision before any harness |
| wgpu agent-reply pixel assertion (4.10) | Pixel-diff step appended to the existing wgpu agent-reply check |

---

## 7. Harnesses that are NOT zero-touch from `.vscode/launch.json` alone (AGENTS.md: devs only use launch.json)

1. **`--packages all` publish + consolidated `rebuild-all`** (1.2/2.5) — coordinator-launched via
   `python3 .tmp-ticket/wp-w2/w2-detach.py` per preamble rule 7; a dev's own launch.json row (`🚚️publish-catalog🗄️os-hub`)
   exists but will fail until the imperative-codegen fix lands, and the *sequencing* (describe→materialize→generate→
   check→activate-s→verify-s→publish→restart) is not itself one launch.json row.
2. **Any wasm32 guest build/check** — must go through `zsh .tmp-ticket/📜️fleet-mutex.sh wasm <slice> -- <cmd…>`, a
   ticket-local FIFO mutex script, not a launch.json row.
3. **postgres/neo4j-backed gates** (2.4, 2.10) — need a running postgres/neo4j server; nothing in `.vscode/launch.json`
   stands one up, and AGENTS.md's zero-touch/cross-platform rule is not met for these two backends until that's true.
4. **Native Linux hub build and Docker image build** (2.8, 2.9, 5.1) — no launch.json row exists for either; only ad
   hoc ticket-local scripts (`wp-h10/docker-build-context.sh`, a container-copy check for Z2's patches).
5. **Windows hub boot** — zero live harness of any kind, zero-touch or otherwise.
6. **Two-human live collaboration runs** (3.5–3.8, 3.11) — require two live client sessions (browser/native) driven
   simultaneously; the individual per-leg gates are launch.json rows, but nothing orchestrates "two humans, one hub"
   as a single dev-runnable command.
7. **Backup/restore drill, storm/reopen probe, RSS-watch, fuzz check** (2.6/2.7/2.11/2.13) — all ad hoc `wp-*/`
   scripts today, not permanent commands (see §6 for proposed homes — this is itself an AGENTS.md violation:
   "permanent scripts only in `📜️script.ts`", "new runnable commands in `.vscode/launch.json`").
8. **Session-13 landing window itself** (3.13) — a manually maintained ledger (`📓️landing.md`), not a runnable check.

---

## 8. Final verification run — ordered plan for the coordinator, after the all-package publish

Sequenced for minimum wall-clock, grouped where independent. Durations are estimates from this session's own cited
measurements (cold boot 153.9 s per H10; 3d.puzzle creation 422 s pre-fix; a full hub/db nextest run historically
~15–25 min; a two-client e2e run 5–15 min; wgpu gates 10–20 min each) — treat as planning-grade, not guarantees.

1. **[~5 min, serial, gates everything below]** Confirm publish rc=0 and the new hub is up: `curl <port>/readyz`,
   check `runId`/build time advanced past this audit's snapshot (`8d0ec7a59c2099e5e20881ef06135591`, 02:31 binary).
   If this fails, stop — nothing below is meaningful yet.
2. **[~10 min, serial]** `📦️check🗄️os-hub🚀️launch` + `📦️test🗄️os-hub♾️all-features` — confirm H9/H11's compiled fixes
   (rows 2.1–2.3) are actually green on the binary just published, not just "compiled once."
3. **[parallel group A, ~20–30 min total]** Once step 2 is green, run concurrently (different ports, no shared state):
   - `⚖️gate🤝️two-client-document🐘️postgres` + `⚖️gate📈️document-growth🐘️postgres` (2.4)
   - `⚖️gate🤝️two-client-document🕸️neo4j` + `⚖️gate📈️document-growth🕸️neo4j` (2.4)
   - `🛠️dev🪐️os-s🩺️cold-boot` (row 1.1 precondition)
4. **[parallel group B, ~15–20 min total, depends on step 1 only]**
   - `🛠️dev🪐️os-s🔭️foreign-kind` across all newly-live kinds (1.4/1.5) — this is the biggest matrix, run per
     plugin-family batch if the harness supports it, else serially inside this slot.
   - `🛠️dev🪐️os-s🤏️pinch-a11y` (5.3)
   - `⚖️gate🌉️os-mcp🚨️capability-audit` (4.2) — re-run only after LB's `d1-frozen.py --apply` + a describe-all pass
     have actually landed; otherwise this will reproduce the same empty-description gap.
5. **[serial, ~10 min, depends on step 4's foreign-kind pass]** `🛠️dev🤝️os-collab-e2e` (3.1) — only meaningful once
   LD's guest-store re-announce fix has landed; run it anyway pre-fix as a regression baseline if time allows.
6. **[parallel group C, ~30–40 min total, needs two live client sessions each]**
   - `⚖️gate🔐️hub-auth🧊️wgpu-live-collaboration` (3.5, native↔React — already proven, re-run as a regression check)
   - `⚖️gate🔐️hub-auth🧊️wgpu-live-journey` (3.2, native/wasm32 late-joiner — will stay red until WG9's kernel+hub
     echo-suppression patches land; run to confirm the fix, not to discover it)
   - `⚖️gate🌉️os-mcp🤝️hub-agent-participant` + `⚖️gate🌉️os-mcp💬️agent-reply` (3.12/4.7)
7. **[serial, ~10 min]** `⚖️gate🌉️os-mcp🤖️live-agent-loop` + the `🌐️de` variant (4.3, 5.2) — confirms prompt-injection
   envelope and locale parity together on the fresh binary.
8. **[serial, ~5 min]** `📦️check🔒️dependencies📃️literal-external` (1.10/5.9) — cheap, run last as a final compliance
   sweep once all the above have potentially touched code.
9. **[whenever convenient, not gating]** Cross-platform (2.8/2.9/5.1) and Docker builds — long-running (build attempt
   6 alone took long enough to be stopped for build-quiet last time), independent of the hub/collab/MCP verification
   above; schedule on a separate machine-hour, not inside this critical path.

**Total critical path (steps 1–8): roughly 2–2.5 hours** if group A/B/C truly run in parallel and nothing regresses;
add the unresolved P0s (guest re-announce fix, echo-suppression kernel+hub patches, imperative-codegen fix,
description describe-all pass) as prerequisites that must land *before* this plan can start, per §1.2/§2.5/§3.1/§3.2/§4.2
above — this plan verifies a fixed tree, it does not fix anything itself.
