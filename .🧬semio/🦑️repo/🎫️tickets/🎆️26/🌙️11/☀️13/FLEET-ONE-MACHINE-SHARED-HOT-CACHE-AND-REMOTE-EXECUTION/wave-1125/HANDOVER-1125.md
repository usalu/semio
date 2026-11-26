# 🧭 Handover Wave 1125 (21:20 CET, 2026-11-25)

Resume from: this file, `coordination.md` (timeline + decisions), `brief.md` (rules + addenda 17:30 dangling imports, 18:00 quiet window, 19:15 atomic writes / no live experiments), `../landings.md` sections "Wave 1125 …", audits `audit-A1..A7.md`, results `result-F*.md`.

## State at 21:07
- Ready 9/14: m01, m06, m07, m08, m10, m11, m12, m13, m14 (was 1/14 at 16:27).
- Not ready: m02, m03 (workspace running), m04 (Linux, inode-short: USER), m05 (Windows deps; F14/F33/F41 fixes reach it on next start), m09 (never provisioned: USER, `prompt-g1-m09.md`).
- Whole graph: NOT sealed (gen 260). F6's read `repo graph --graph-policy fresh` (pid 96993, since 19:10, log in coordinator scratchpad F6-graph.log) still running at 21:00, 5.5 GB vs 3 GB grant. After seal: `repo run repo-mcp:build` → `repo run repo-mcp:install` → restart repo MCP server.
- Weekly usage 65 % at 21:20 (24 % at 16:30).

## Landed this wave (highlights; details in landings.md)
F1 debug #[path] mounts removed + source-transient retry classes; F3 bun-cache reflink (EACCES); F4 disk/inode need from measured runs, pressure-ordered collectors; F5/F25 stall patterns + progress labels; F9 warden identities; F10/F17 hidden Windows spawns, Go launcher, restart loop; F11/F16 crate root + OUT_DIR remaps, cache namespace main; F12 units in timings; F13 beyond-host never local + CPU headroom; F14 binding resume; F19 slice heap paging; F20 admission retry; F21 failed task output in logs; F22 typed partition refusals + slot wait; F23/F27 stale fact repair + tombstones; F24 dwg_snapshot_objects split; F26/F39 closure temps + module split; F28 single 32-bound; F29 Go suites green; F30/F43 read liveness, farm, quarantine; F31 50 build-owners rows; F32 journal identity; F33 failure attribution + Windows rescan; F34 lock-aware unit collector; F36 heap diagnostics; F37 ArtifactPersistedOperation on 50 plugin types; F38 train reds; F40 key components; F41/F42 pid+start identity; F44 no IP literals, addresses.json, resolver.

## Running at 21:20
F45 (verify F42's untested identity conversion; hard startMs error risk), F46 (dwg editor + lib tests), F47 (all clients onto resolver; USER: semantic names = origin ids via DNS-SD `_semio-fleet._tcp`, no OS hostnames).

## Open (ordered)
1. Seal + MCP rebuild (above). Parent heap grows ~2.6 MB/project (F36 diagnostics landed; census of next read names the retainer).
2. m02/m03 workspace outcome; m05 next attempt on new code.
3. F42 remaining 63 pid-only files (law baseline shrinks only).
4. Test reds: budget tests (unknown, key-reference, inference, publication-fallback), headroom load timeouts, suites routing laws (unrouted new files), step-overdue 18/4.
5. Unit cache misses across origins: grep `[units] miss … differs in` in worker logs after rebuilds (F40).

## USER actions
- g1-m04: `df -i /home/deck/semio/.cache`; `sudo du --inodes -x -d3 /home/deck/semio 2>/dev/null | sort -n | tail -20` (result-F4.md Round 4); service-bootstrap sudo (wave-1122/result-E64.md).
- g1-m09: run `prompt-g1-m09.md` on that Mac.
- g1-m05: Defender exclusion for C:\semio; Windows task checks (result-F10.md, result-F17.md).
- Re-provision with name-based store mount (result-F44.md, then result-F47.md); avahi on Linux.
- Archivist: clear 2738 staged stray index entries (replica/sa, -inf) at next commit (result-F8.md).

## Update 23:28 (weekly 79 %)
- Ready 9/15: m01, m06, m07, m08, m10, m11, m12, m13, m14. New origin g1-m15 (m09 replacement) in workspace; m02/m03 workspace; m05 Windows dependencies (new code after next start); m04 USER (inodes).
- Landed since 21:20: F45 (identity tolerance for records without startMs, boot supervisor self-contained identity), F47 (one resolver for all fleet clients, roles by origin id, no IP/hostname literals, conflict-gated DNS-SD advertising — currently none advertise), F48 (fleet status host-name column), F49 (capability-deps generator dedupe + manifest parse law), F50 (signed-fleet test hermetic), F51 (`repo fleet hostname apply`, origin-derived, asks before sudo), F52 (one launcher per sidecar role, endpoint keep-last).
- Hostname incident: F47's DNS-SD advertising (21:54) surfaced macOS "local hostname in use" dialogs because m08/m10/m13/m14 share `Uelis-Mac-mini`; advertising disabled 22:09; design in `design-fleet-naming.md`. Coordinator LocalHostName is currently `semio-g1-m08` (user ran the example here) → USER runs `repo fleet hostname apply` on the coordinator and on m08/m10/m13/m14.
- Seal: coordinator read `repo graph --graph-policy fresh` started ~23:06 (log coordinator scratchpad coord-graph.log), facts 684k/842k at 23:28. On seal: `repo run repo-mcp:build` → `repo run repo-mcp:install` → restart repo MCP.
- Running: F46 (dwg editor/lib tests).
