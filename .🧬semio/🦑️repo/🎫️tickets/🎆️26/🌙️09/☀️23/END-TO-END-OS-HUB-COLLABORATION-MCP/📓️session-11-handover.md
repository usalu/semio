# Session 11 Handover (to session 12)

Session 11 (coordinator: Claude Code main chat, 2026-09-25 00:30–18:0x) ran W2, H9, C10, G10, S15, WG7, WG8, T12, R8, U5, Z2
(Opus) and six Sonnet audits. Every Opus executor was cut at ~18:0x by the account's **weekly limit (resets 2026-10-01 11:00)**;
the desktop restart at ~19:30 killed every remaining process. Session 11 launches nothing further; session 12 owns the fleet.
Full chronology: `📓️work-packages.md` §Session 11 (+ coordinator backlog, post-catalog wake list). Agent ids in
`📓️fleet-11-agents.md` resolve only from the session-11 chat.

## Decisions (binding unless session 12 revisits them)

| When | Decision |
|---|---|
| 01:15 | Landing window before one-tree rebuild; H9's owned ABI 13 → 14 exports means every host refuses older guests. |
| 01:2x | Open targets: the hub's Rust rule is the only authority (bootstrap TS twin deleted). vcs kind id `s.vcs.vcs` everywhere. Whole-catalog memory caps replaced by schema-declared per-component/per-actor bounds + lazy on-disk retention. |
| 01:3x | A1/A2 "wire `ServerInstance::Documents` for hub" closed by design: the hub's own router owns `/scopes/{scope}/document/ws`; `HubInstance::Documents = NoDocumentAuthority` stays. |
| 07:1x | Uninstalled plugin opening a hub document: option (a) — hub serves a trusted, content-addressed plugin module bundle per package in the catalog generation + hub-backed PluginSource (landed, S15). |
| 07:5x | Mixed catalogs (packages built before/after an ABI-compatible edit) are fine; `Table`/`TableRow` were appended to `Component` (serde tag by name). |
| 11:50 | Viewer (read-only) open targets are emitted by the same Rust rule; the shell picks editor vs viewer by grant. |
| 12:4x | Durable data (hub roots, catalog copies, credentials, detached-chain logs) under `.🧬semio/🌐hub/s11-<slice>-*` — the external low-disk cleanup (fires near 96 % disk) deletes ticket `generated` folders and SIGKILLs their holders. Keep the disk > 100 GiB. |
| 13:1x | A hub document resolves its plugin module by the serving catalog generation; a local copy is used only on equal content hash (landed, S15). |
| 13:2x | Whole-repo walks + normalization tests belong to `long` (approved); normalization mapping rebuilt on current names (done); 648 discovery + 3 layering-ratchet problems + trinity F1 → one rename slice after the `--packages all` publish (plan in `📓️wp-r8.md`). |

## Open items per slice (from each report's last state)

- **W2** (`📓️wp-w2.md`): restage verified 60/60 at 18:56 (per session 12). Next: catalog B2 from the current tree → restart 7800
  once on a current-tree `os-hub` with a fresh data root and a fresh `inference/gis-map-jobs.sqlite3` (old binary stack-overflows on
  any gis approval; old table caps inference input at 64 KiB — G10) → rest-warm 25 → `--packages all` (viewer targets + module
  bundles) → restart 7800. Also: two trusted-catalog laws collide on a process-wide registry in `--all-features` (H9 d); release
  chain as one registered command done (`plugin-registry:rebuild-all`); `build-s-react-release` + RB1 release-bundle probes.
- **H9** (`📓️wp-h9.md`): hub rejects writes after ~20 edits / `DB I/O process aggregate credit exhausted` after ~20 documents (likely one
  root cause: a lifetime aggregate budget that never refills) — top priority; creation-catalog kind labels all "Editor" (serve en/de
  labels from descriptors); P0-1 creation law on a real guest + DB4 pg/neo4j document lane + restart < 1 s; pipe allows 64 exchanges
  then the hub EXITS; loading hub ignores pipe EOF. Last step in flight: "tests used `_done` directly; use the new accessor".
- **C10** (`📓️wp-c10.md`): collab-e2e 10/10 + STEP 14 (writer/draw/puzzle 3d) on 7800 never green (0/13 blocked by staging, now
  fixed by restage4); hub-document undo `action-owner-mismatch`; note second-window commands refused; viewer leg (after
  `--packages all`); same-field conflict live; inspector ≤ 2 s live; zero-touch timed run on the final tree.
- **G10** (`📓️wp-g10.md`): hub quartet 19/19, durability 21/21, hub-agent-participant 17/17 done. Open: 4b browser half (human sees
  the agent's edit in the React shell, en + de) — script `wp-g10/g10-agent-edit-human-sees.ts`, needs 7800 on current binary +
  Space index fix; live roster badge.
- **S15** (`📓️wp-s15.md` §12–13): catalog-generation pinning + durable module store live on 8040; proof on canonical 7800 pending
  (its watcher died with the restart); Space index third cause = space plugin never sets `space_id` (→ T12, status unknown).
- **U5** (`📓️wp-u5.md`): windowed table landed; live 30-space Home proof pending (restage4 contains the space fix).
- **WG7** (`📓️wp-wg7.md`): last step "recording the accessibility fix while the queue drains"; wasm32 browser shell painted + signed
  in + document Live + second user sees the edit — check its report; backlog: wgpu agent reply (tag 10) + approval overlay parity.
- **WG8** (`📓️wp-wg8.md`): native 12-step wgpu gate 12/12 on catalog B. Open: async native document open (20 s frame block in
  debug), native app presence data (wgpu cursors to React), queued wasm32 checks (`.🧬semio/🌐hub/s11-wg8-captures/check-wasm-5.txt`),
  genesis-on-open for wasm32/React, 13 wgpu Shell test-layout contract rows. Last step: "detached surface renders for the settle
  lane's owed refresh".
- **T12** (`📓️wp-t12.md`): all S15 guest defects fixed (in restage4). Open: space `space_id` (first), F3 pdf `set-pattern` with a
  missing shading, 37 Python oracle cases (jack scene format first), 8 sequence rows + fem3d digest, 31 stories/benchmarks importing
  test fixtures. Last step: "now the leaf edit".
- **R8** (`📓️wp-r8.md`): last step "the validator in discovery" while deleting 48 stray tracked `.js` + dead configurable-descendant
  code; after `--packages all`: ui-contract `PagedList` (13 callers), `runtime_tree_retirement_*`, unbailed root `test quick`.
- **Z2** (`📓️wp-z2.md`): after `--packages all`: land B1–B4 (B4 = winit Linux backends, `semio-hub` does not compile on Linux) and
  the devcontainer timed zero-touch run (Docker cannot read `~/Documents`; run from a copy).
- **Rename slice** (new, after `--packages all`): R8's plan for 648 discovery + layering ratchet; trinity rewriting window config
  under a mode folder (T12 F1); ~35 tests with path corruption; register taxonomy `🔬️*` test dirs.

## Unowned backlog

Cold `docker build` of `🌎️hub/Dockerfile` + `docker run` `/healthz`; hub route input-validation fuzzing; capability-audit 29
findings across ~14 plugin descriptors; native wgpu (winit) accessibility; `.🧬semio/🌐hub` old roots (h4-cat, gm1-boot, …, ~8 GB)
can be pruned; the repo index held 5,077 staged files incl. hub data at 07:3x (auto-stager) — the user was told.
