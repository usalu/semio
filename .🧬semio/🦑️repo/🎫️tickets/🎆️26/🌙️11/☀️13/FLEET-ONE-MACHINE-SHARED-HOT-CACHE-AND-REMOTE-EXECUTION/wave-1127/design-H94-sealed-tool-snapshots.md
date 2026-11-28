# Design H94: sealed tool snapshots (the repo tool unbreakable by peers' edits)

## Problem
CLI door, kernels, MCP build surface, sidecar roles and `repo land` run from the live tree. A peer's half edit broke `repo` about 8 times on 11/27. The existing `🏛️sealed` store (per-entry capture of door and kernels) was never switched on: `defaultMode` is `live`, the store was empty, and sidecar roles, MCP and land are not covered; roles even reload onto the raw tree (H80 rule), so a half edit reaches them within 15 min.

## Decision
One immutable, content-addressed snapshot of the whole tool closure, promoted only after a gate, followed by every role. Module `B/🏛️sealed/🧰️tool-snapshot` (schema-first: `🧬️schema/🔣️.json`, policy and EN/DE texts `🔣️.json`). Product builds still read the live tree (`SEMIO_WORKSPACE_ROOT` stays the live root).

| Concern | Rule |
|---|---|
| Capture | Files below declared roots plus the import closure of declared entries (real finding: the tool imports `🔨️modules/⚙️runtime`, `📁️filesystem`, `🖱️ui`, even `💻️os/…` product files). Hash until two passes agree. `sourceDigest` = sha256 of `path<TAB>sha` lines sorted by UTF-8 bytes (code only; test edits never make a snapshot); `id` = sha256(sourceDigest, runtime). Blobs by content address, manifest = closed record. 4 700 code files, 44 MB, 8-17 s under load 30. |
| Gate (fail fast, bilingual rows, verdict record) | `syntax` (closure files, real transpiler), `imports` (closure whole), `typecheck`, `smoke` (declared tool tests), `self-run` (tool run from the snapshot answers `repo fleet status --json` and `repo land check` of a canary). The last three run in a land overlay that holds the snapshot, never in the live tree. Budget 5 min. |
| Promotion | Event log, one file per event (`promoted`, `rejected`, `rolled-back`, `fault`), written once, no lock. Current = fold of the log; `promoted` compares and swaps on `basedOn`; pointer file = cached projection (healed from the log). Claim folder per source digest: one machine gates, the others wait and adopt its verdict (lease 10 min, takeover when expired). A rejected tree is not gated again until a file changes. |
| Rollback | Previous promoted snapshot (`repo tool rollback [id]`), the left tree is blocked. Auto: 3 faults of the current snapshot within 5 min (a role ended in error within 60 s of starting from it) roll it back. |
| Follow | `resolve` materializes the promoted snapshot once per machine under the short machine-cache folder `<cache>/semio/t/<id10>` (hard links from the blob store, read-only, `node_modules` bound so `@semio-tech/*` packages are the snapshot's copy). Launchers map their entry to the newest run at every restart. Role watch = existing sidecar watch interface over the pointer: a role restarts only when ITS closure differs between the snapshot it runs and the newest promoted one, otherwise it adopts silently. Reload bounds: edit to promoted <= 10 min (45 s quiet, 3 min max deferral, 5 min gate); promoted to running <= 15 min (90 s + 3 min grace + 6 min drain); asserted in a test. |
| Store | `SEMIO_TOOL_STORE`, else the shared fleet store `S/tool/<group>` (replicas promote through it), else machine cache. Nothing is written inside the replicated tree. Run folders always in the machine cache. GC keeps current + 3 earlier snapshots, quiet 10 min, run folders kept while a live process marks them (`.use-<pid>`). |
| Bootstrap | No promoted snapshot: live tree runs, bilingual notice names `repo tool promote`. `SEMIO_TOOL_MODE=live` is the editor's opt-in. |
| Cross-platform | Node built-ins only, UTF-8-byte ordering, junctions on Windows, copy when hard links fail, path budget asserted (longest captured path 173 UTF-16 units; run prefix must stay below 83). |

## Integration (staged, lands after the seal through `repo land`)
1. Sidecar `🛰️sidecar/🟦️.ts` (patch `T/🧰️scripts/H94/sidecar.patch`): `remoteExecutionSidecarWatchV1` returns the pointer watch in sealed mode (so `fleet status` code freshness compares against the promoted snapshot); the launcher starts children through `toolStartOfV1` (newest run, env `SEMIO_TOOL_SNAPSHOT/RUN`) and reports a fault when a sealed child dies within 60 s. Role closures grow by 5 small files only (the gate sits in `🔁️promotion/🚀️flow`, which roles do not import; verified).
2. Replica daemon `🧷️sidecars/🟦️.ts` (patch `sidecars.patch`): `entryOf` maps the launcher script into the newest run (fallback live on any error), so the launcher itself is sealed.
3. Operations catalogue: register verb `tool` (status promote gate rollback fault resolve gc) -> `🧰️tool-snapshot/📜️script.ts`; add catalogue row, selection case, help snapshot, MCP golden vector. Not staged (catalogue edits are frozen and would restart the read); until then `bun <module>/📜️script.ts <verb>` works.
4. Promoter duty: a periodic `repo tool promote` on every machine (cheap when fresh; one gates, others adopt). Replace the sealed-door trampoline's per-entry capture by the tool pointer (phase B: door and kernels start from `resolve`; removes 10-40 s per-command capture; `🏛️sealed/🚦️trampoline` and `📥️execution` change, tests there must be rerun).
5. Flip: after one promotion exists on the fleet, `defaultMode` is already `sealed`; peers who edit the tool set `SEMIO_TOOL_MODE=live` for their own commands.

## Known limits
- The installed native `repo` binary (`~/.local/bin/repo`, owned by `repo-mcp:install`) and the launcher registry are read from the live tree before any of this starts. Gate `self-run` uses the installed binary against the snapshot, which catches the "registry malformed" class; a full seal of the registry is phase C.
- The unpinned Bun (1.3.14) garbles emoji import paths in `Transpiler.scan`; the closure scanner silently saw 2 files. The imports/syntax gates refuse to run on it (probe) instead of passing.
- Overlay gates need the land overlay (APFS clone; copy elsewhere, slower).
- Data files read by path (not import) must lie under a declared root; a missing one is caught by smoke/self-run only.
