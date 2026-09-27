# WP-C13 — Viewer Role Enforcement And Cross-Peer Undo/Redo

Session 14 slice C13 (coordinator `main`). Scope: outcome 3 rows **3.4** (viewer / read-only role, broken since session 11) and
**3.11** (undo/redo of another peer's edit). Ports: hubs 8170–8179, serves 6670–6679. Scripts `wp-c13/`; expendable captures
`wp-c13/generated/`; durable data and logs `.🧬semio/🌐hub/s14-c13-*`. Rules: `📓️session-14-preamble.md` (+ 13/12).
Handovers: `📓️wp-c10.md` (c10perm1), `📓️wp-c11.md` (item 4 roles probe written, never run; item 8), `📓️wp-ld.md`.

## Session 14

| # | Item | Status |
|---|---|---|
| 1 | Row 3.4 — viewer role: open, presence, live updates; every edit path refused (UI disabled + hub refusal incl. crafted ops), en + de feedback; root cause; law + live probe; permanent harness (R10) | in progress |
| 2 | Row 3.11 — cross-peer undo/redo: schema-first semantics (own ops only, rebased over later foreign ops; declared conflict outcome), law + live two-peer harness for note, writer, draw | pending |

### Infra (pids I started)

| What | pid | Port | Notes |
|---|---|---|---|
| serve `s` dev → 7800 (`wp-c13/serve.sh`, w2-detach) | 57602 (vite 58391) | 6670 | log `.🧬semio/🌐hub/s14-c13-logs/serve-6670-7800.txt` |

### Log (session 14)

- 18:3x read preambles 14/13/12, AGENTS.md, `📓️fleet-14-agents.md` (no "CHAIN LAUNCHED" line yet → guest freeze not started),
  `📓️audit-s14-state.md` rows 3.4/3.11/§8, `📓️acceptance-s13.md` §3, `📓️wp-c10.md` (c10perm1), `📓️wp-c11.md`, `📓️wp-ld.md`.
  Load 98 (peers' builds). Found: C11 wrote `wp-c11/probe-c11-roles.mjs` (writer, spectator, 15:37) but never ran it (no capture).
- 18:4x serve 6670 → 7800 (see Infra). Coordinator rule 17 (harness contract) + rule 18 (`ensureDevServe`) read.
- 18:5x **Permanent harness**: the viewer journey is a journey of the existing os-dev `verify two-human` verb (rule 17, no new
  directory): `--journey edit|viewer` (`🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts`, region 🔖️Journeys; the former per-kind edit legs are
  `editJourney` unchanged; acceptance check id `two-human-viewer`; both serves through S18's `withDevServe`). The crafted-write leg
  signs in as B over HTTP and writes straight onto the document socket with the hub probe client
  (`🌎️hub/🤝️integration-harness/🟦️.ts`: `HubProbeDocument.submit` answers the Ack instead of asserting acceptance, `plan` exposed).
  Typecheck: os project `tsc` 0 errors in my files (1 peer error in layout diff, `generated/tsc-os-1.txt`, 12 min at load 100);
  narrow config `wp-c13/tsc/tsconfig.json` (my 2 files + closure) 0 errors in my files (3 peer errors in acceptance orchestration,
  `generated/tsc-c13-1.txt`).
- 19:07–19:25 **viewer journey run `c13viewer1` (7800/B3, en, serve 6670) — reproduced, 3 kinds measured, then cancelled for the fix:**
  `bash run-two-human.sh c13viewer1 viewer en …` → `generated/c13viewer1.txt`, `generated/c13viewer1/{report.json,console.txt}`.
  2d.block / 2d.drawing / 2d.puzzle each **5/10**: PASS create, B opens, window focus, "viewer edit attempts change nothing"
  (hub head 0→0), **hub refuses a crafted write** (plan as B: role `viewer`, write false; Ack `Rejected: unauthorized: no grant
  allows Write … for roles ["spectator"]`, head unchanged, A unchanged). FAIL: B holds the **editor** app (chip `editor:Editor`,
  `block2d-board`, no `#viewer` surface), offers `addHandleKind`/`undo`/`redo`, presence B = 0 peers, B never sees A's edit, no
  read-only feedback.
- 19:2x **ROOT CAUSE (measured):** the React shell resolves the Space index's `os.open-artifact` (no role) to the preference default
  (editor), creates the editor session and requests `…#editor` in the open intent. The hub issues a Spectator only viewer
  surfaces (`surface_writable` → `resolve_document_open(…, writable=false)`), so the plan answers **503 `component-unavailable`**
  (`probe-c13-viewer-plan.ts` → `generated/probe-viewer-plan-1.txt`: no surface → 200 `#viewer` write=false; `#editor` → 503;
  `#viewer` → 200). The worker treats that 503 as transient (`🔁️execution-target-retry`) and retries silently; B keeps an
  unattached local editor (its edits go to a local instance, never to the hub; no presence; no live updates). The hub side is
  correct (plan, gate, crafted write refused); c10perm1's 503 (session 11) was the same defect, then masked by catalog A.
- 19:3x **HOST FIX (React shell TS, open during the freeze):** `🏛️ShellHost/🧭️opening/🟦️.ts` `sharedDocumentOpeningRoleV1(events,
  spaceId, userId)` — folds the space's own directory events (`foldAll`) to the caller's member role (owner → author) and decides
  editor vs viewer with the hub's declared access policy (`🌎️hub/🔐️auth/🛡️access-policy/🔣️.json` via its TS twin
  `hubAccessPermits(…, "document.write")`, space kind ignored exactly like the hub's `surface_writable`). `🏛️ShellHost/🟦️.tsx`
  `os.open-artifact` → a read-only caller opens with `role: "viewer"` (the viewer app, `…#viewer` requested); `os.open-artifact-with`
  asking a read-only caller for the editor is refused with the new localized reason `view-only-access` (en "You can only view
  documents in this space — editing needs the Author role." / de "In diesem Space können Sie Dokumente nur ansehen — zum
  Bearbeiten ist die Rolle Autor nötig.", `📣️replay-refusal/🟦️.ts`).
