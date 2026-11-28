# 🧭 Handover Wave 1127 (checkpoint 07:06 CET, 2026-11-27)

Resume from: this file, `coordination.md` (timeline, lane table, decisions), `brief.md` (rules + addenda 00:15 scoped searches, 00:40 memory gate, 01:40 landing slots :00-:10, 04:15 graph lock queue), audits `audit-A1..A5`, results `result-H*.md`, landings `../landings.md` ("Wave 1127 …").

## State 07:06
- Ready 11/14: m01 m02 m03 m06 m08 m10 m11 m12 m13 m14 m15 (start of wave: 8). Not ready: m04 Linux (tools; scoped install fixed inode need, 107 GB stale 🗑️generated swept by H28 sidecar), m05 Windows (native stage), m07 (capacity head-of-line → H46 backfill landed 07:00), m09 never provisioned (USER).
- Measured (A5 06:25): offload out 494 / 15 min (was 37), unit-cache hits 233k vs misses 31k / 15 min, skip storm 0, Nx 14 hits (was 4), divergences +39/h (was +53/h), stalls fixed (false stall + CPU-only).
- Proven end to end: remote-hit with identical sha-256 (H40), test files offloaded in parallel to 3 workers 33.9 s → 11.2 s (H44), Nx same hash on two origins (H37), dashboard fleet window 14/0 (H24).
- Weekly usage 37 % at 07:05.

## Key root causes found this wave
Cache server down = capacity 507 + hand-over cadence (H3/H14/H17/H40); parallel rustc frontend → nondeterministic .rmeta (H15); Nx hash included timed stderr (H18); exclusive repo-wide cargo-preparation (H31); slice-read non-shrinkable 5.9 GB ask (H13); guard blind to cargo shard images (H47); held profile lease pinned every unit (H41); lane-own regressions: H23 scopes (fixed H26), H5 nested workspace (H24), H21-unrelated, 05:00 folder move (H46).

## Running at checkpoint
H9 whole-graph seal (edge-heal fix; then repo-mcp build+install → coordinator restarts MCP), H43 narrow slice 170 s, H48 lease-order deadlock in nested images (units 21/5), H49 divergences after 04:00 + threads arg verification, H50 suite-runner offload + double budget stretch.

## Open (ordered)
1. Whole-graph seal + repo-mcp rebuild/install (H9).
2. Units test 26/0 (H48), divergence rate → ~0 (H49).
3. m04/m05/m07 to Ready; m05 partition census → capacity-exceeded partition (H7 rounds).
4. Remaining: r image in-place growth; actor_input decoder peak 1836 MB (threads policy verification H49); suite runner offload (H50); Nx cross-machine hit proof from logs after 06:05 (H37 skip lines).

## USER (unchanged from 1125)
m09 provisioning; `repo fleet hostname apply` on coordinator + m08/m10/m13/m14 (duplicate LocalHostName); m08 fseventsd sampling needs sudo (result-H22.md); Docker/Zotero data optional.

## Update 14:40 (coordinator context 73 %, weekly 60 %)
- Ready 11/14 (m01 m02 m06 m07 m08 m11 m12 m13 m14 m15 + one of m03/m10 at times); m03/m10 workspace (slice memory on 16 GB), m04 Linux slow dependency payload capture (H87 r2 optimizing), m05 Windows in workspace stage. m09 USER.
- Landed since 07:06 (highlights): H49/H55/H68/H69/H81 cache determinism (key contract v2, rendering-only streams advisory, worker rust-src remap, RUSTFLAGS out of keys) → divergences ~1/h; H73 fleet prefetch; H74 `repo land` (mandatory since 14:10); H75 latency budgets + diagnostics channel + no-[DEBUG] law; H76/H70 identity-checked stops + reaper + stop-sender records; H80 every role on current code ≤ 15 min after a landing; H82 guard floor/sealed-source lease/host-derived per-project cap; H83 fleet status 11.8 KB; H57/H46/H10 admission priority/backfill/stale restart; H63/H64 dependency image restore + destructive ops sandbox-only for tests.
- Seal: H9 retired; H84 continues (lands H60/H78 staged graph patches, fixes "Generator output has no Build producer", narrows framework-os-dev inputs, runs proof read, then repo-mcp build+install). Declare a quiet window when H84 reports the read's pid.
- Running: H84 (seal), H86 (units last 4 + manifest miss), H87 r2 (m04 payload capture), H88 (shared-child registry/broker + Windows job objects).
- Open backlog: W/audit-A7-open.md (+ W/result-H79.md, findings-H79.md). Follow-up cards proposed: wasm split, app-mount harness + browser matrix.
- Final audit owed after the seal: A-lane comparing with A4/A5/A6 (+ H73 prefetch re-measure once fleet refilled, Nx hits).

## Update 19:45 (coordinator context 86 %, weekly 74 %)
- Freeze (16:50) LIFTED at 20:00 slot (brief addendum 19:35). Only lane H101 (landing train, agent ab71a03103a6c3b90) lands; queue order and stage paths in brief addendum 19:35 + H101 prompt: H89 stageR → H88 stage9 → H92 → H96 → H98 → H90 → H87 heal → H86 diag → H91 (--rebase) → H99 → H100 → H95 → H94 → H78 enum. Later: H103 stages (hand-over trigger narrowing; per-owner env components phase 1).
- Seal: not achieved; read 10 (pid 48808) runs since 13:16 with restarts; environment keeps moving (outside peers, ticket 27/PREVENT-OUT-OF-APPLICATION-MEMORY-PAUSES…). Structural fix designed (W/design-H102-incremental-graph-publication.md); H103 (a50017b09d8a3ce6b) implements quick win + phase 1. H84 (a800a698410f65aa0) watches read, reports by 21:00, builds+installs repo-mcp if it seals.
- Running lanes: H101, H103, H84. Side sessions (user-started): split-plugin-wasm (local_63003e1e…), fail-fast app-mount harness (local_629e5261…).
- Incidents today worth remembering: H94 hard-linked 8,765 live files in a scratch tree (removed 19:40); outside peers edit during freezes; repo land overlay false reds (fixed in H89 stageR).
- After the landing train: final live audit (A-lane) vs A4/A5/A6, H73 prefetch re-measure, Nx hit proof, then close ticket with summary (ticket_close with explicit path).

## Update 23:15 (coordinator context ~90 %, weekly ~80 %)
- Landed via the landing train (H101, agent ab71a03103a6c3b90): H89 stageR 20:01, stageS 21:01, stageT2 22:01, stageU 23:01 (repo land overlay now equals live; control stageC green); H103 s1 21:09 (graph read code identity = its import closure); H103 s2 landing (per-owner edgesReads, flag componentEdgeValidity off). Remaining queue in H101: H88 stage9, H92, H96, H98, H90, H87 heal, H86 diag, H91, H99, H100, H95, H94, H78 enum.
- Proof read 10 stopped by coordinator 23:08 (old code, could not seal: heal non-converging because the read's own waves move the environment). NEXT: after H103 s2 is live, start a proof read with componentEdgeValidity=on (stand-alone lane, run detached), then repo-mcp build+install if it seals.
- A8 (23:05, audit-A8-final.md): Ready 9; regressions: offload out 0/15 min, upload-entry-gone storm (m10, m15), CAS 115 % cap, 11 origins' agent role code behind disk, m01 offline 6 h. Lanes H105 (a1b931dbcb3f96694, CAS/upload loss) and H106 (aed40d033b1ff2860, convergence/m01/offload) running.
- Side sessions: forms harness landed test-matrix-forms 23:07; wasm split building.
- Close-out still owed: final measurement after H105/H106 + landing train, ticket summary, ticket_close with explicit path; delete W/🗑️generated folders we created.

## Update 02:35 (coordinator context ~95 %)
- Running: H101 landing train (queue: state fix, H92, H90, H96, H98, H88, H87, H86, H91, H99, H100, H95, H94, H78, H106, H105-B, componentEdgeValidity policy flip); H108 proof read watcher; H109 (m13 status/ready role); H111 (verify H110 + lend test); H112–H116 (wave-4 feedback). All stage-only; hand stage paths to H101.
- Done tonight: overlay false reds closed (H89 stageU + H107 state fix), H103 s1/s2 live, H105 A (cache eviction vs reserve) applied + cache role handed over, H110 foreign grant rows isolated (live), stale proof read stopped, stale git index.lock removed (user request).
- USER: m01 login, m05 Defender exclusion, m09 provisioning, hostname apply.
