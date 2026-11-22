# Result E66: cache divergence (read live 15:28 CET)

## 1. Live state (MCP `remote_cache_status` + `remote_cache_evidence`, evidence rows converted to CET)
Server restarted 15:25:05 CET (startedAt 14:25:05Z). Counters: lifetime 143 since 01:53:11 CET, byClass unit 80, owner-pool 35, project-inference 16, closure 12. Evidence 118 rows (unit 23, owner-pool 20, closure 11, project-inference 64 "superseded", not divergences). AC conflicts since restart 0; evidence is what I analysed.

| class | newest divergence (CET) | since 11:14 CET | state |
|---|---|---|---|
| unit (cargo) | 02:55 | 0 | fixed (E3/E4); no unit divergence for 12.5 h |
| Nx / Bazel | none ever | 0 | nx 0 puts; no divergence |
| project-inference | 13:53 (superseded) | superseded, not counted | newest-wins since E3 round 4 |
| owner-pool (wave manifest) | 15:23 | 5 (13:22, 13:53, 14:14, 15:21, 15:23) | cause found, fixed here |
| closure (record closure bundles) | 15:11 | 3 (12:27, 12:48, 15:11; all refused origin g1-server) | cause found, NOT fixed |

So new divergences still appear, but only from graph owner records (owner-pool and closure), none from cargo units, Nx or Bazel since 02:55.

## 2. Causes (both outputs fetched read-only and diffed; script `T/🧰️scripts/E66/fetch-evidence.ts`)
- owner-pool: one wave id is the digest of the owners, so every machine that reads the same owners publishes the same manifest key with its own `head.br` (`publisher`, `createdMs`), `session.br` (publisher's runtime bytes) and `units.br` (cost sheet `megabytes` measured per machine). Newest evidence 15:23 (m12 kept, m08 refused): head differs in publisher/createdMs, units in 20+ `megabytes` lines. Not hermetic by design; first writer wins and the second upload (about 340 KB) is wasted and counted.
- closure (key `2ac06c19…`, g1-m12 kept vs g1-server refused, same recipe `native-target` of `✏️s/…/◻️2d/…/rust`): the bundle records the walk observations of the inferring process; kept holds 1952 pooled walk strings, refused 98 (1884 only in kept, 30 only in refused), table rows 9530 vs 9579. The observed set depends on what the process had already walked (other projects' `script.ts` walks), i.e. history of the process, not on the recipe. Spurious extra observations also over-invalidate the closure. Root cause lives in the closure recorder of `🕸️graph/📥️inference/🧭️script-inputs` (2044 lines); NOT fixed (outside a safe slice of this lane, needs per-owner read scoping). All 3 current ones come from g1-server running a different code digest than the kept writer.

## 3. Changes
1. owner-pool single writer (`🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts`, schema +3 policy keys): `publishOwnerPoolV1` claims the manifest state first (`tryGraphEntryV1`): stored means verify by read and skip the write; held waits up to 8 s for the holder's manifest; granted writes under the hold; fault, or a renewal by the same writer, writes as before (409 path unchanged). Landed under the graph lock, see landings.md.
2. Status tells current from historical (`remote-cache/🧾️provenance/🟦️.ts`, server type, `🏃️execution/🧬️schema/🌐️remote-cache.json`): `divergences.current` = divergences of this server process (since its start, i.e. the code of today, `since` = start), `lastHour` = sliding window over the kept rows; lifetime `count`/`byClass` stay as the history. Event-sourced: rows with `atMs` persist, the current window restarts with the process.

## 4. Proof
- New test `owner-pool/🧪️tests/🔁️single-writer` (4/0, run 4 times): control uses the plain entry publisher from two real signed fleet identities and reproduces the divergence (count 1, differing head/session/units, as in the live evidence); with the fix a second machine writes nothing, the pointer lists the wave once, first writer's manifest stays and verifies, concurrent publishers end with 0 divergences, the writer's own renewal still rewrites (createdMs refreshed). Independent cross-check = the control through the unmodified entry publisher against the real cache server.
- provenance 6/0 (new: restart opens a new current window, `lastHour` equals a naive filter of `divergences.json`), by-class 3/0 (current/lastHour over the real server), nx 1/0, remote-cache 27/0, server 10/0, owner-pool main 32/0, early-wave 7/0, keep-alive 3/0, heavy 5/0.

## 5. Before / after
Before (15:28): lifetime 143, indistinguishable from today (status could not tell fixed from live). After code: status shows `current` and `lastHour` next to the lifetime 143; at 15:28 `current` would read 0 (server up 3 min, newest divergence 15:23).

## NOT verified
- No live effect of the owner-pool change: machines pick it up on their next supervisor restart; the server counters must be re-read in about an hour (expect owner-pool `current` 0; closure still appearing).
- The live status with `current`/`lastHour` needs a server restart onto the new code.
- Closure root cause (history-dependent observation set) unproven by a two-scratch-dir build; no fix landed. Windows/Linux not examined.
- Cross-platform byte-identity of cargo outputs was not re-measured here (no unit divergence since 02:55 is the only evidence).

## Others must know
Divergence times in `remote_cache_evidence` rows are epoch ms (jq `strftime` prints UTC, CET = +1 h). A closure divergence is harmless for correctness (first writer kept, both valid) but costs hit rate; the owning lane for graph closures should scope walk observations per owner. `repo remote-cache status` has no `--json`; use the MCP tool.

# Round 2 (closure bundle determinism)

## Finding (both bundles of the 15:11 closure divergence unpacked with `unpackBundleV1`, script `T/🧰️scripts/E66/analyze-closure.ts`)
Kept (m12) vs refused (g1-server) bundle of one recipe: `value`, own `rows` and `sidecar.facts` are identical. Three things differ:
1. `sidecar.members` (the walks known to include a layer): 2153 vs 190 distinct walk strings. It is process history (every walk the process derived or loaded before, merged by digest). Only the walks that are facts of the entry's own layers can matter when the entry is resolved (`resolve`/`plan` test membership against `walked`, built from those facts): members intersected with them are equal on both sides. This was the pooled-string difference 1952 vs 98.
2. Row order inside layers: 5 of 7 layers hold the same rows in another order.
3. Layer 0 and 2 (the project roster observations): 5 rows only in kept, 54 only in refused (`widgets/rust/project.json` exists on one side, `spr/channel/archive/ingress/…` on the other). That is the tree of the two writers (checkouts that differ in time), a legitimate difference.

## Changes (landing "E66 round 2" in landings.md)
- `script-inputs/🟨️.mjs`: `canonicalSidecarV1`, used by `closureTravelV1.export`: exported walk members are the entry's own walks, in code unit order. The local store and local resolution are untouched (no history is lost locally; sibling-entry deduplication of the row log still uses the full local members).
- `portable/🟦️.ts` `packBundleV1`: rows and every layer in `canonicalRowOrderV1` (the same order records already use).
- `provenance/🟦️.ts`: class `closure` joins the superseding classes (newest write replaces, replaced one kept as evidence, counted under `superseded`), because after 1 and 2 the only remaining difference is the tree of the writer; the importing machine already adopts a bundle only when it holds in its tree. Updated the by-class test.
- Not changed: derivation itself (what the recipe reads). I did not prove that the layer rows of the roster are exactly what the recipe needs; the roster rows are what the engine records today.

## Proof
- New `script-inputs/🧪️tests/closure-bundle-canonical.test.ts` 2/0 (RED before the change): two machines (two scratch roots) derive one entry with the layer rows in opposite orders and different prior knowledge of walks (5 vs 5 other walks, overlapping), export through the real `closureTravelV1.export` + `packBundleV1`, bundles byte-identical, and the exported sidecar holds exactly the entry's walk; independent Python oracle (`🐍️oracle/🔢️canonical.py`, UTF-16 code unit order) checks rows and layers sorted and members within facts and sorted.
- Dependants: closures 3/0, entry-lane 5/0, shared-intake 4/0, streamed-intake 2/0, portable 13/0, provenance 6/0, by-class 3/0, nx 1/0, server 10/0. `closure-answer-refresh.test.ts` has 1 failing test that greps `project-nodes/🟨️.mjs` for source text (not touched by me; peer drift).
- Not done: a real two-process inference of a repository recipe (the whole read on this Mac was not disturbed); the proof uses the store/export path with controlled entries.

## Live read after rollout (16:20 CET, MCP `remote_cache_status`)
Server restarted onto the new status code at 15:57:29 (startedAt 14:57:29Z); `current`/`lastHour` are live. Lifetime 148 (owner-pool 40, closure 12, unit 80, project-inference 16). `current`: 1 owner-pool (since 15:57); `lastHour`: 2 owner-pool, 0 closure, 0 unit; superseded {project-inference 1}. The two owner-pool rows (16:04 m08, 16:19 m10 refused, kept m12, key 4fcec…) are still written by machines that run the older code: the server restart does not restart the machines. Closure: none since 15:11 (3 h 05). Expect owner-pool `current` to stay at the old-code rate until every supervisor has restarted; re-read in an hour. Not verified: which machines already run the new code (attest code digests differ per origin at every read).
