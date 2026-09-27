# S18 — os `s` frontend: matrix en/de, dialogs after sign-in, event-sourced named layouts, UX/a11y (session 14)

Slice S18, session 14, 2026-09-27 18:2x. Successor of S16 (`📓️wp-s16.md`) and S15 (`📓️wp-s15.md`); audit
`📓️audit-s13-os-frontend.md`, acceptance `📓️acceptance-s13.md` §1. Ports: hubs 8040–8049, serves 6540–6549.
Durable data/logs `.🧬semio/🌐hub/s14-s18-*`. Expendable captures `wp-s18/generated/`. Rules: `📓️session-14-preamble.md`
(+ 13/12). Permanent harnesses: `@semio-tech/framework-os-dev:{program-matrix,tool-run-matrix,hub-document-sweep}` (V1).

Status legend: **measured** = ran here, command + capture named; **unverified** = read from source only; **written, not run**.

## Session 14

| # | item | state | evidence |
|---|---|---|---|
| 0 | reconcile S16's in-flight tree edits | **done**: S16's two host edits (`💻️os/🟦️.ts` [DEBUG] removal, program-matrix pins) are complete + staged; landing row added (was missing). S16's unreported measured runs after 14:56: editors de **60/60** (57/60 + resume), viewers en **70/70**, tool-run en 4/18 (unclassified) | `.🧬semio/🌐hub/s13-s16-logs/matrix-s16-r1{de,de-resume,viewers}.txt`, `toolrun-s16-t1en.txt` |
| 1 | editor/viewer matrix en + de (local serve) + hub 7800 (B3) sweep; host reds fixed, guest reds routed | **en measured (current tree, 6540, chain-run-5 staging): editors 61/75, viewers 70/70.** Non-norm editors 58/60: `demonstrator/generation3d` (undo/redo refused `instance-retired` on its own window — intermittent: PASS alone, s18-g3d-a) and `stdio/csv` redo `ui.snapshot-details.arguments: snapshot details UI admission failed` (guest, LB2 — same as S16). norm 3/15 = stale norm fixtures (N1, same as S16). Found (host-side candidate, investigating): after a demonstrator program closes, the NEXT row's inputs (`clearSelection`/`redo`) are dispatched to the closed program's window (`demonstrator-3::procedural-main`, refused instance-retired) — s18-demo-a 6/7. de run in flight; hub sweep next | `.🧬semio/🌐hub/s14-s18-logs/matrix-s18-r1en.txt`, `🧑‍💻dev/🤖️generated/🧮️program-matrix/s18-r1en/`, `…/s18-demo-a/`, `matrix-s18-{g3d-a,demo-a}.txt` |
| 2 | matrix as a permanent harness (coordinate R10) | **landed (TS)**: the three os-dev matrix harnesses already were verbs + nx targets + rows (V1); the gap was zero-touch — they needed a serve someone started. New shared fixture `ensureDevServe` (R10: THE one fixture for every harness) + `withDevServe`; `--serve <url>` (+ `--hub <url>` for the sweep) per preamble rule 17; row spec sent to R10 (window-3 generator) | tsc 0 `generated/s18-tsc-harness-1.txt`; laws 26/26 `s18-law-local-hub-full.txt`; mutant red `s18-law-dev-serve-mutant.txt`; landing row |
| 3 | dialog closed by the post-sign-in Home re-bootstrap (coordinate C12) | **root-fixed + landed (TS), live proof pending.** Cause: a first human re-established Home (instance 1 → 2 ~5–8 s after sign-in; the owner closed instance 1's socket and re-bootstrapped `after=0`), based on a 09-20 premise ("a Home that refused identity-less STOPPED") that the 09-18 guest fix made false (Home renders signed-out as a state). Now `shellHumanChangeRecoveryV1`: refresh for a first human / new display name, re-establish only for another human or sign-out. C12 told | landing row; `generated/s18-law-surface-switch-{2,mutant}.txt`, `s18-tsc-item3-1.txt` |
| 4 | NamedLayoutStore → event-sourced `semio.os.config › namedLayouts` preference mutation (TS now, Rust twin prepared) | **TS landed**: `setNamedLayout` UI-preference mutation (schema-first, persistedShared, lane slot per app+layout, >4 KiB record stays on the device with an en/de notice); `NamedLayoutStore`/`mergeNamedLayouts`/`OsShellConfigSnapshot.namedLayouts` removed; Display host projects the preference; also fixed: user layouts were keyed by the SESSION app (saved under Home while a spawned program was focused). Boot broke 19:27–19:30 (importer edited 3 min after the export removal) → rule 20. Rust twin: prepared patch in progress | landing row; `generated/s18-law-ui-preferences-{2,mutant}.txt`, `s18-law-docklayoutstore.txt`, `s18-tsc-shell-2.txt` |
| 5 | a11y / customizability / multi-language audit of the shell + fixes | pending | |

### Infra (pids I started or inherited)

| what | pid | port | notes |
|---|---|---|---|
| (inherited from S16) serve `s react dev` local-only, HMR off | 23244 → vite 23285 | 6540 | **stopped 18:3x** (by pid) |
| serve `s react dev` local-only, HMR off (w2-detach) | 46078 → vite | 6540 | 18:3x, log `.🧬semio/🌐hub/s14-s18-logs/serve-6540.txt`; staging = chain run 5 (`[stale]` vs source for most plugins) |
| matrix en editors+viewers (nohup) | 47781 | → 6540 | done 20:1x, log `.🧬semio/🌐hub/s14-s18-logs/matrix-s18-r1en.txt` |
| matrix de editors+viewers (nohup) | 62131 | → 6540 | 20:3x, log `matrix-s18-r1de.txt` |

### Session 14 log

- 18:2x read preambles 14/13/12, AGENTS.md, `📓️fleet-14-agents.md` (no CHAIN LAUNCHED line yet → guest freeze not started),
  `📓️wp-s16.md`, `📓️wp-s15.md` (§0–2), `📓️audit-s13-os-frontend.md`, `📓️acceptance-s13.md` §1, fleet-13 log 14:00→.
  Load 107, 17 rustc, swap 6.2/7.2 GiB, 142 GiB free.
- 18:3x stopped S16's serve (23244/23285), fresh serve 6540 (46078), en matrix editors+viewers launched (47781, 145 rows).
- 18:4x C12 relay (item 3): C11 run b3-3 l.392–413 — after sign-in user1's directory socket `since=132` closes at once, then
  `event-page?after=0` + `typed-operation slots instance=2`. Read: the close is the EFFECT of Home instance 1 → 2 (owner cleanup
  closes instance 1's socket), not a wrong cursor: `since=132` = the page's `throughSeqInclusive` (hub head), user1's last
  visible event is 82. Cause chain (source, ShellHost): the primary session (Home instance 1) is established before the hub
  identity resolves; the directory-owner effect then bootstraps instance 1 as soon as identity + authority arrive
  (`refreshDirectoryHome` renders Home WITH identity, so its toolbar is live), while the human-change effect re-establishes the
  session on the lane (instance 2) because `sessionHumanKey` changed — the human acts on instance 1, which is retired seconds later.
- 18:4x–18:5x item 2 landed (table + landing row). Fixture path + signature + R10 row spec sent to main (`RELAY R10`).
- 18:5x–19:1x WG11 relay: `ensureDevServe` gained `renderer` (react|wgpu) + `profile` (dev|release), schema-first
  (`$defs.DevServeSpawnRequestV1`, `devServeCommandV1`, fixture `serves.spawns`); laws 27/27. R10: fixture = rule 18; R10's
  goal gate `serve-hold` verb wraps it (R10 relay).
- 19:0x–19:45 item 4 TS (table). 19:3x coordinator URGENT: every `s` boot failed (`does not provide an export named
  'NamedLayoutStore'`) — my 🖥️platform edit (19:27:06) removed the export 3 min before the ShellHost/⚛️react importers were
  edited (19:30:41). Closed at 19:30:41 (0 importers left); proof: the en matrix re-booted to Home (beacon ready:s) after 19:30
  and kept passing. Rule 20 since: tsc + one boot after every host edit.
- 20:0x item 3 (table). Human-change effect now keyed on the human, not a string.
- 20:1x en matrix done (table). 20:2x generation3d alone PASS (`s18-g3d-a`); all demonstrator editors `s18-demo-a` 6/7 — cad
  row's faults are inputs on the CLOSED generation3d window (`demonstrator-3::procedural-main`, instance 3) while cad is
  instance 4 → host-side stale-window dispatch after a spawned program closes (investigating). `--serve` verified: the
  runs reused 6540 (`[dev-serve] … already answers — reusing it, never stopping it`).
- 20:3x de matrix launched (62131).
