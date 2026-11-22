# B1: Stale eviction audit (read-only, 2026-11-22)

Scope: what retires cache entries today, on the fleet server and on each machine, against the goal "the fleet cache holds only builds of the coordinator's synced (checked-in) sources; stale entries are cleared". No code was edited. Paths: LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`. The remote cache is at `LIB/🏗️build/🌐️remote-cache/` (not `LIB/🌐️remote-cache/` as the brief said). Slot module: `LIB/🏗️build/🪢️slots/`.

## Verdict

Today nothing knows the synchronized tree. Retirement is slot supersession (newest *stored* key of a slot wins), plus age for unslotted entries, plus size budgets. Slot ids exclude source content, and "current" means "stored most recently", not "built from checked-in sources". A build of a dirty worktree therefore makes itself current and supersedes the synced build after 10 minutes. Deleted or renamed crates and removed Nx targets are never superseded, because nothing stores a newer key in their slot. Slotted entries are never aged out. The server has never received a slot sidecar in practice, so server-side supersession is unexercised on real data.

---

## 1. Server side (`192.168.178.70:6094`, store `/var/lib/semio-remote-cache/v1`)

### 1.1 How each kind is retired today

| Kind | Slotted? | Retired by | Where |
|---|---|---|---|
| Rust unit (`ac` from the fleet tier) | Only if the client sends `x-semio-slot` and it has the header from `unitBasicsV1` | Supersession (same slot, newer `storedAt`, beyond grace), then budget tiers | `🖥️server/🟦️.ts:198-199, 242`; `🧹️collector/🟦️.ts:45, 49, 52-56`; `🪢️slots/🟦️.ts:52-67` |
| Nx task (`nx`) | Yes, slot registered by `POST /<instance>/v1/slots` after the run | Same as above | `🖥️server/🟦️.ts:162-177`; `🪢️nx-slots/🟦️.ts:66-78` (client) |
| Bazel AC (`ac` from `/ac/`) | No slot header from the Bazel client, so always unslotted | Age only: `maximumAgeDays` (30), per entry idle time (mtime or ledger), `🧹️collector/🟦️.ts:46` | `⌨️command/🟦️.ts:136` |
| Bazel CAS / blobs (`cas`) | Never slotted (`slotted = slot!==undefined && kind!=="cas"`) | Only as orphans once no remaining entry references them (`reclaim`), and pinned while a live entry needs them | `💾️store/🟦️.ts:257`, `:184-214` (collect), `:217-240` (pinLive) |

Budget and triggers:
- Byte bound: `--max-bytes` (fleet unit passes 200 GiB, `🤝️shared-worktree/🗺️topology/🟦️.ts:35`). Default from policy (2.5 GiB per the earlier map; the policy file was not re-located in this pass).
- Planner order (`🧹️collector/🟦️.ts:38-58`): unusable results (dangling) → superseded beyond grace and not claimed (`:45`) → unslotted older than the age (`:46`) → when over bound, least valuable first: retained, unslotted, current; LRU; down to 90 % of the bound (`:36`, `:52-56`). Slotted entries are never aged out (`:46` only applies to unslotted).
- Grace: `SLOT_GRACE_MILLISECONDS_V1 = 600000` (`🪢️slots/🟦️.ts:13`). Entries stored or claimed within the grace are guarded (`🧹️collector/🟦️.ts:49`).
- Triggers: maintenance timer every `maintenanceSeconds` (300 s) runs `refresh` then `collect` (`🖥️server/🟦️.ts:314-320`); a write over bound runs `collect` (`💾️store/🟦️.ts:296, 326`); the collector lock is `.collector.lock` (`🧹️collector/🟦️.ts:35`).
- Claims: `POST /v1/slots` makes an existing key current again (`💾️store/🟦️.ts:246-251`). A read never does (`🏷️retention`/`🪢️slots` doc). The claim table is in memory and journal-backed (`🎟️claims`).
- Auditor, not in the retirement path: `auditRemoteCacheDirectoryV1` (`🏷️retention/🟦️.ts:89-101`), called only by the command at `⌨️command/🟦️.ts:248-250`. It takes `currentKeys` from outside, so it is the seed for a "synced keys" oracle.

### 1.2 Is there a "current synced tree" notion?

No. Searches for `generation`, `synced`, `manifest`, `dirty`, `checked-in` in the remote-cache, slots, unit-cache fleet and replica code find nothing relevant to staleness. The slot id (`🪢️slots/🧬️schema/🔣️.json` `SlotId`; `🛰️fleet/🪪️identity/🟦️.ts:142`) is `sha256(domain, toolchain, target, profile, package id, crate name, kind, features, cfg)` for Rust, and `(project, target, configuration, platform)` for Nx. Source content is excluded by design (the key carries it). `storedAt` is the server clock of the last store or claim (`💾️store/🟦️.ts:233-235`). So "current" = newest store, whatever the source. The replica daemon (`🤝️shared-worktree/🔁️replica`) has no generation or manifest id that the cache could use.

### 1.3 Live numbers (2026-11-22, measured on disk on the server host; HTTP `/status` is 401 for every token in `/etc/semio/remote-cache.env`, so the store was read directly)

- Server process: bun pid 49378 listens on `192.168.178.70:6094`, up about 3 h 51 min. `/healthz` answers `{"status":"ok"}`.
- `.server.lock` names pid 48393, which is dead. So `remoteCacheServerRunningV1` (`🧹️collector/🟦️.ts:132-137`) returns false while the server runs, and a collector run from outside would not defer supersession to the server. Stale lock, to be fixed.
- Entries: `semio/ac` 220 files, `semio/nx` 2 files, `main/ac` 1 file. Total about 2.4 MB on disk (`/var/lib/semio-remote-cache/v1`, 3.1 MB with overhead).
- Sidecars (`.meta`): directory does not exist. 0 slotted entries. `claims.jsonl`: 0 lines. So no entry on this server has ever been slotted or superseded.
- Newest file mtime 2026-11-15 06:45, oldest 2026-11-13 18:01. No write for about 7 days.
- Capacity: 200 GiB bound, 0.002 % used. Not near the limit. The spool-full skip reason is client-side (local spool `fl/up` has a cap of 4096), not server-side.
- Host CAS on this host (`~/.cache/semio/cas`): 96,928 objects, 34.0 GiB (35,692,780 KiB), above the 32 GiB policy cap (`BUILD/🗄️cas`). Partition `/System/Volumes/Data` 95 % used, 52 GiB free. That is above the 0.90 high watermark, so the host CAS is the real pressure point on this machine, not the server directory.

---

## 2. Local side (this machine, unit cache `.cache/b/r/m/u`, 8.1 GiB)

### 2.1 Mechanisms

- Rust unit entries (`e/<shard>/<key>/<variant>.json`, `runs/`): supersession runs as `supersedeUnitsV1` via `supersedeLocalUnitsV1` (`⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🪢️supersession/🟦️.ts:53-97`), at most every `UNIT_SUPERSESSION_INTERVAL_MILLISECONDS_V1 = 300000` (`:85`), called from `collectGarbageSingleFlightV1` (`🧹️gc/🟦️.ts:118`). Slot and storedAt come from the entry (`slot`, `createdMs`, `:35-41`). Grace = 10 min. Unslotted entries are classified `unslotted` and never superseded (`:57-59`).
- Slot writes: `publishUnitV1` writes `slot` only when `unitBasicsV1` succeeds (`📜️script.ts:206`; `🛰️fleet/🟦️.ts:28-31`). When it fails (compiler release undetermined), the entry is unslotted and only the lane rules apply.
- Lanes (`l/…`, `u/l/<lane>/…`): `planGcV1` (`🧹️gc/🟦️.ts`) removes idle lanes older than `laneIdleMilliseconds` = 24 h (`🧬️schema/🔣️.json`), then by lane share (`laneShare` 0.234375), incremental share (0.1), unit share (0.5), and free-space pressure (high-water 0.875, low-water 0.75). Lanes are not slot-aware.
- Nx: `supersedeNxCacheV1` (`🏗️build/🪢️nx-slots/🟦️.ts:42-63`), called by `settleNxSlotsV1` (`🏗️build/🌐️remote-cache/🧶️nx/🪢️slot-settle/🟦️.ts:33-40`) from `⚡️caching/🚀️bootstrap/📜️script.ts`. Only Nx entries that Nx lists in its database are judged. The local Nx cache `.nx/cache` does not exist on this machine, so nothing ran here.
- `pruneEntriesV1` (`💾️store/🟦️.ts:250-262`) removes entries whose objects are lost, and also `rmSync`s the whole objects directory `o` on each apply. Check that this is intended (see gap G8).
- Host CAS: `reclaimCasObjectsV1` removes blobs no remaining entry references (`🧹️gc/🪢️supersession/🟦️.ts:75-80`). No slot awareness, no generation.
- Stage outputs: `☁️stage-fleet/🟦️.ts:26` uses `slotIdV1("stage-output", …)`, so a producer exists. Its slots are not verified against retention in this pass.

### 2.2 Live numbers

- Entries: 1014 keys, 1014 variant files, 2.85 MB of JSON. All 1014 are slotted, 858 distinct slots. 858 current, 153 superseded within grace, 3 superseded and expired (would be removed at the next pass).
- Entry ages: newest and median are minutes old, oldest about 4.3 h. Claims rewrite `createdMs`, so this is not pure creation age.
- `gc.log`: 216 applied collections. Totals: 3,178 superseded keys, 92 lanes removed, 392 units removed. Last supersession stamp 2026-11-22 20:51.
- Lanes: 68,589 files under `l/`. Unit cache total 8,523,808 KiB (8.1 GiB).
- Fleet state `fl/`: upload spool `up` empty (0 files, cap `maximumSpool` 4096). `offered` 11,663 files, `cost` 4,118, `fb` 8.
- Volume (last `fill.jsonl` sample): 42.9 GB volume, 28.4 GB free.
- The fleet reachability record points at `http://192.168.178.70:6094` (`offload-reachable.json`).

---

## 3. Gaps (to guarantee "only current builds kept, stale cleared")

- **G1 Critical. No synced-state identity.** "Current" = newest store (`💾️store/🟦️.ts:233-235`; `🪢️slots/🟦️.ts:51-67`; `🪢️supersession/🟦️.ts:41`). A dirty build of a crate supersedes the synced build of the same slot after 10 min. Nothing prevents uploading dirty builds (uploader `📤️uploader/🟦️.ts:140` checks only usability).
- **G2 Critical. Deleted, renamed or removed units are never retired.** The last key of a slot is current forever, and slotted entries are never aged (`🧹️collector/🟦️.ts:46`). Stale entries leave only when budget pressure arrives. The same holds for Nx tasks of removed targets and for crates renamed (new slot, old one never superseded).
- **G3 High. Slot coverage is incomplete.** Bazel AC is always unslotted (age only). CAS blobs are never slotted (only orphan-reclaimed). Local entries without a readable `unitBasicsV1` (`📜️script.ts:206`) are unslotted. Wasm and component finals go through `stage-fleet` (slot domain `stage-output`) with no retention check here. Build-script outputs (`buildScript` in the unit-cache policy) were not checked. Nx tasks with empty platform (task hash carries none) get `platform` from the machine, so they are per-platform, which is fine.
- **G4 High. Server supersession is unexercised.** 0 sidecars, 0 claims, store idle since 11-15. The collector logic is covered by unit tests (`🧹️collector/🧪️tests`, `🏷️retention/🧪️tests`) but no real traffic has run through it.
- **G5 Medium. Stale server lock.** `.server.lock` names a dead pid while pid 49378 serves. Owner handover (`🖥️server/🟦️.ts` owner loop) should rewrite the lock; verify, then fix.
- **G6 Medium. Grace is a fixed 10 min and is not tied to sync transitions.** A machine that comes back after hours still holds old synced builds, which are current only if no newer key was stored in their slot. Once G1 is fixed, grace should apply to generation changes, not to time.
- **G7 Medium. Other platforms.** Slots include `platform` for Nx but not for Rust, which relies on toolchain and target in the slot parts. Entries of a platform no longer present are never retired by generation, only by age (unslotted) or budget. Needs per-platform generations.
- **G8 Medium. Local `o` deletion.** `pruneEntriesV1` removes the whole objects directory on every apply (`💾️store/🟦️.ts:262`). Confirm that no live entry depends on local objects before relying on it.
- **G9 Medium. Host CAS above cap and above high watermark on this host** (34.0 GiB vs 32 GiB; partition 95 %). Collection should be running (guard at 0.90). Verify the `casEnforceTickV1` trigger, or the cap is not enforced because pins protect everything.
- **G10 Low. Stale doc.** `BUILD/🗄️cas/🧬️schema/🔣️.json` comment says 120 GiB, value is 32 GiB (from the earlier CAS map).

---

## 4. Minimal clean design to close the gaps

Core idea: replace "newest store wins" with a **generation mark**. Retention keeps exactly the keys used by the current synced generation (and the previous one within grace). Slots stay as a dedupe hint, not as the authority.

1. **Generation identity (closes G1, G6, G7).** A generation is `G = sha256(platform, toolchain identity, synced source manifest)`. The synced source manifest is the checked-in tree at the coordinator's HEAD, read-only (for example `git ls-tree -r HEAD` hashes, or the replica manifest once it exists). Dirty or unsynced builds compute keys but never upload them and never join a generation. Files: new `🛰️fleet/🧬️schema` field, `🔁️replica` manifest export (read-only), `📤️uploader/🟦️.ts:140` (gate on synced flag), unit-cache `🛰️fleet/🟦️.ts` (carry `generation`).
2. **Generation manifest (closes G1, G2).** The build driver records the keys it touched for generation `G` (hit or stored; the `runs` log already has these) and publishes `(kind,key)` list plus `G` to the server: `POST /<instance>/v1/generations/<G>` (same auth as `/v1/slots`). The server keeps the manifests of the current generation per platform (plus previous within grace). Files: `🖥️server/🟦️.ts` (new route next to `:162-177`), `🏷️retention/🟦️.ts` (manifest sidecar), `🧹️collector/🟦️.ts:38-58` (new drop reason `stale`: key not in any live manifest and older than grace).
3. **Local mirror of the same rule (closes G2 locally).** `🧹️gc/🪢️supersession/🟦️.ts` and `🏗️build/🪢️nx-slots/🟦️.ts` use the local manifest of the current generation instead of "newest variant". Units and Nx hashes not in the manifest and older than grace are removed. Files: `supersession/🟦️.ts:53-82`, `nx-slots/🟦️.ts:42-63`, `slot-settle/🟦️.ts:33-40`.
4. **Complete slots (closes G3).** Give Bazel AC a slot derived from the Bazel action's mnemonic and target (or make the generation manifest the only rule for them, so no slot is needed). Put CAS blobs under the manifest through their referencing `ac` keys (already `refs`), not via slots. Make `unitBasicsV1` failure a visible refusal, not an unslotted entry: `📜️script.ts:206`, `🛰️fleet/🟦️.ts:28-31`. Wasm and component finals: check `☁️stage-fleet/🟦️.ts:26` keys enter the same manifest.
5. **Server lock truth (closes G5).** Owner loop rewrites `.server.lock` with its own pid after a successful takeover; `remoteCacheServerRunningV1` checks the live pid. File: `🖥️server/🟦️.ts` owner loop (near line 327), `🧹️collector/🟦️.ts:132-137`.
6. **Keep budgets as last resort (G9).** Verify the host-CAS enforcement tick and pins; add a test that a partition at 95 % triggers collection. Files: `🗄️cas/🚦️guard/🟦️.ts:94-97`, `🌡️partition`.
7. **Verification.** `repo remote-cache audit` (existing `auditRemoteCacheDirectoryV1`, `🏷️retention/🟦️.ts:89-101`) takes `currentKeys` from the latest generation manifest and must report `stray: []`. Add a language-agnostic test: build generation A, change one crate (generation B), assert the old crate's keys are gone after grace on server and local, and the other keys are kept.

Open items (not verified in this pass): the policy file for server defaults (`maximumAgeDays`, `maintenanceSeconds`, default `maximumBytes`); the `stage-fleet` retention; `buildScript` outputs; whether `casEnforceTickV1` runs at all on this host.
