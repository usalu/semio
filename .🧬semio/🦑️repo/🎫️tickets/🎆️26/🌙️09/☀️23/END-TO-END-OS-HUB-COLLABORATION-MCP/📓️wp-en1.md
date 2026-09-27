# WP EN1 — Energy epJSON (LB-F2) + Draw Verb Descriptions

Session 13, slice EN1 (Claude Code fleet, Opus executor), ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP. Coordinator = `main`.
Scope: two unowned outcome-1 items from `📓️audit-s13-window3-inventory.md`, prepared as ONE patch for landing window 3
(`wp-en1/en1-apply.py`, `--dry-run` default / `--write`; payloads under `wp-en1/payload/`). No repo-tree edits before window 3.

## Session 13

| Item | Status | Evidence |
|---|---|---|
| LB-F2 energy epJSON (`energyplus-run-{600,600ff,900,900ff}` 28/32) | investigating | — |
| 3 draw verbs without en/de description (`editFill`/`editPath`/`editSelection`) | pending | — |
| en1-apply.py dry-run on live tree | pending | — |
| overlay verification | pending | — |

### Log

- 15:5x started; read AGENTS.md, preamble rules 1–36, window-3 inventory, LB's LB-F2/D1 entries. Chain final at hub-prewarm.
- 15:5x **LB-F2 root cause is NOT the exporter's ideal-loads branch** (read-only): the exporter writes
  `ZoneHVAC:IdealLoadsAirSystem` whenever `model.ideal_loads` is non-empty, and the committed `🏛️bestest-600/900/🔋️model.json`
  carry one each. The parity orchestrator (`🧪️test/⚖️parity/📋️orchestration/🟦️.ts` `subjectRawInputs()`) folds ALL passed
  subject results of a case into ONE `implementation → rawPath` map (`Object.fromEntries`, last wins), so every
  `@oracle-input-subject-raw` oracle scenario of the case reads the LAST subject scenario's bytes (= `energyplus-run-900ff`'s
  epJSON). That explains LB's three symptoms at once: no ideal loads in "600"/"900", IDENTICAL free-float temperatures for
  "600FF"/"900FF", and parity 3/16 (schema-validity/nothing-dropped oracle facts computed from the wrong document).
  Still to measure: whether the correctly-routed 4 documents agree with the honeybee reference (LB's number 6.18/48.25/29.37 is
  900FF's own document vs ref 1.25/44.30/25.13 → a real ~4 K gap may remain).
