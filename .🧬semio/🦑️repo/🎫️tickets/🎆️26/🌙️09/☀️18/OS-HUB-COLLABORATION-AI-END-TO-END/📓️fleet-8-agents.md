# Fleet 8 — agent handles (coordinator session 8 [92e5fa], launched 2026-09-22 ~11:00)

Resume a cut worker with `SendMessage` to its id from THIS session (ids do not resolve elsewhere).
Port plan: C8 hub **7621** (shared with CE3, never restarted) + serves 6190 (`s`) / 6191 (`gis2d`), optional own hub 7671; S11 hub **7641** (`s10-boot`) + serve 6071; TC3e hub **7651** (stdio,gis,note); CE3 hub 7621 + serve 6196; HT16 private suite `target-ht16`, optional own hub 7681; G18 checks in `target-g18`.
Wasm mutex order (`📜️mutex-ordered.sh` stamps): c8 `…110000` → s11 `…110100` → tc3e `…110200` → ce3 `…110300` → ca1 `…110400`.

| slice | model | agent id | report |
|---|---|---|---|
| C8 | opus | a750c0105eba37ffb | 📓️c8-two-users-collaborate-live.md |
| S11 | opus | a4758d651ccb4d768 | 📓️s11-thirty-five-kinds-inside-s.md |
| TC3e | opus | a941002772cf80aef | 📓️tc3e-three-package-hub-and-note-creation.md |
| CE3 | opus | a55dd9a24f97e8af4 | 📓️ce3-four-mcp-gates-green.md |
| CA1 | opus | aef88a663eb1614d4 | 📓️ca1-capability-audit-zero.md |
| HT16 | opus | a49dd17b89346cb51 | 📓️ht16-hub-startup-stall-bound-and-frontier-identity.md |
| FP11 | opus | a7c8cc5c46c8c51da | 📓️fp11-plugin-lib-zero-red-and-lanes.md |
| G17 | sonnet | ae2663cd29d551c38 | 📋️g17-acceptance-ledger.md |
| G18 | sonnet | aa22b8b6855f3c892 | 📓️g18-tree-compile-health.md |
| G19 | sonnet | a98d817a70e247071 | 📓️g19-ai-user-experience-audit.md |
| AC1 | opus | a94a8ada3b832ccee | 📓️ac1-agent-reply-channel.md |
| M10 | opus | ada87c88a97697963 | 📓️m10-agent-commits-hub-edit.md |

Wave 2 (11:2x): AC1 (agent free-text reply channel in `AgentChatPanel`, serve 6197) and M10 (agent commits a `Commands` frame on a hub document, extension inference roster, serve 6198) from G19's ranked list. G17/G19 done. Held back until load allows: DB4 (document on a Postgres-backed hub), RB1 run 7 (release `s` bundle, last in the mutex).
