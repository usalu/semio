# Session 15 Preamble

Session 15 of the repo goal (2026-09-29 19:0x, Claude Code fleet). Coordinator = main Claude Code chat (Opus 5.5).
Executors = Opus 5.5 agents, one slice each (same slice names as session 14; each continues its `📓️wp-<slice>.md`).
Auditors = Sonnet 5 agents, read-only. Ticket folder ASCII entry `/Users/ueli/Documents/semio/.tmp-ticket/`.
Fleet handles + coordinator log: `📓️fleet-15-agents.md`. **Rules 1–28 of `📓️session-14-preamble.md` apply** (and through them
sessions 12/13) unless overridden here.

## The Goal (four outcomes, all end to end)

1. Working os `s` frontend with all plugins and artifacts.
2. Working hub server backend (db, presence, auth, observability).
3. Working collaboration between users over the hub (React shell and wgpu shells).
4. Working AI integration for users over the **semio** MCP (`semio-framework-os-mcp`, `mcp__semio__*`), never the repo MCP.

## Situation at 19:1x

- The session-14 coordinator and every agent died ~18:45 (no handover beyond the reports). Detached processes survived:
  - **Chain** pid 57946 (`wp-w4/w4-chain.sh final`, log `.🧬semio/🌐hub/s14-w4-logs/chain-final.txt`; catalog `s14-w4-catalog-t6`,
    7800 root `s14-w4-hub-7800-t6`): hub-prewarm rc=0; wasm hold (57953) in rebuild-all 4/11 components (describe +
    materialize-dev), forward release lane stdio rc=0, gis rc=0, animate warming.
  - **Hub 7800 READY on p33** (hold 66344 / hub 66347, root `s14-w4-hub-7800-p33`) until the chain moves it onto t6.
  - Dev serve 67105 (vite) — owner unknown (S18 or C12); keep until its owner successor decides.
  - Orphaned lane jobs (their agents are gone; successors read the captures): native lane HELD by U6 `wp-u6/t4/r6-base.sh`
    (60943, renderer wgpu suite); queued native: LW1 (63907), S19 (62696), G12 (67185); overlay lane HELD by LB2
    `lb2-p17-proof.sh p17-s1` (70960).
  - A Cursor agent peer (`dev mcp stdio cursor`) is active (stdio pdf / norm areas in session 14c) — work alongside it.
- **Chain defect fixed by the coordinator 19:1x:** describe failed for all 8 stdio family packages (`stdio-<family>: manifest has
  unknown fields hostedArtifactKinds`) because `CATALOG_MANIFEST_FIELDS` in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts` lacked LB2 p15's field → added
  (+ array check). The chain's blind retry resumes rebuild-all `--from components`.
- **GUEST FREEZE (session-14 rule 27) stays ON** until W4 reports `final-publish.rc` = 0 and the coordinator writes
  **"WINDOW 5 OPEN"** in `📓️fleet-15-agents.md`. Guest-linked sets stay PREPARED (dry-run clean on the live tree) and are appended to
  `📓️t6-queue.md` (rounds after 3a = train T6 round 4+, then T7). Host-only fixes land compile-atomically (rules 20/22); hub/kernel-db
  edits must keep `os-hub:build-dev` green (the chain builds the hub from the tree right after publish).
- The repo MCP `ticket_reopen` answered "invalid tool params" at 19:1x; the ticket is still `open` (session 15 recorded here).

## Rules (session 15 additions)

29. **Report sections:** `## Session 15` right under the title block of `📓️wp-<slice>.md` (status table first, then a timestamped log);
    landing rows under `# Session 15` in `📓️landing.md`. Measured results only, with command + capture path.
30. **Orphans first:** a successor's FIRST step is to reconcile its predecessor's last in-flight step: orphaned lane jobs (read the
    capture when it ends — do not kill a job that holds a lane unless it is provably wedged; a queued orphan waiter whose result you no
    longer need may be killed ONLY if it is your slice's pid), half-applied tree hunks (finish compile-green or cleanly revert your own),
    prepared sets (dry-run on the live tree).
31. **Waiting:** one blocking wait per ≤ 10 min on a file/capture (`until … ; do sleep 30; done` inside ONE Bash call with a bounded
    loop); never short polls; never sleep-wait on another slice. Blocked → write it, message `main`, take your next item, or end the turn.
32. **Usage:** a 14+ agent Opus fleet has burned the 5-hour window in ~1–2.5 h before. Keep reports terse, don't re-read large files you
    already summarized, prefer finishing one item end to end, and END YOUR TURN (with the report written) when you only wait on the
    chain / another slice / a lane — the coordinator resumes you with `SendMessage`.
33. **Relays:** cross-slice → `SendMessage` to `main` starting `RELAY <slice>: …` (≤ 5 lines). Done/blocked → `SendMessage` to `main`
    (≤ 5 lines) before ending the turn.
