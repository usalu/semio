# R5: is the fleet cache ready for retention `enforce`? (2026-11-24, 17:20–17:55)

T = the ticket folder. Store = `.cache/b/p/m/fleet/cache-server/v1` (live server pid 67402 at the snapshot, 79985 since 17:34). Scripts: `T/wave-1122-dist/R5/` (`snapshot.py`, `server-judge.ts`, `oracle.py`, `analyse.py`, `blobs.py`, `held.py`, `registry.py`, `sample.py`). Outputs: `T/wave-1122-dist/🗑️generated/R5/` (snapshot of 17,271 entries taken 17:29–17:30, verdicts, oracle, rows, blob sizes, `sample1.md`; about 98 MB, the coordinator may drop them).

Rules I kept: the live store and the host CAS were only read (readdir/stat/read; I did not use `scanRemoteCacheMetaV1`, which deletes sidecars). Nothing was evicted or restarted, the mode is still `dry-run`, nothing was written in `/Users/ueli/.sinc/play`, there was no scratch build, and every test went through `repo test-file`, one file run at a time. The play gate is still `[run] site-build 15:14:08` with no `[ok]` after it.

## Verdict for the user (short)
- The judge is correct on the live data. An independent Python model of the rule disagrees with the server judge on **0 of 17,271 entries**. A sample of 30 stale and 30 live entries was checked against the tree from scratch, and every verdict held.
- I found **one real false-stale defect, and it only shows up in `enforce`.** Tombstones from an `unreached` retirement, or from an edit that was later reverted, made later CURRENT units stale for good, along with everything that depends on them. **Fixed and tested** (section 2c). Flip only after the cache sidecar runs code from 17:44 or later. The running pid 79985 started at 17:34; it takes the fix at its next closure self-restart.
- `dry-run` already changes things. `pinLive` leaves stale and unreached entries out of the pin set in every mode, so the host CAS sweep has already taken their blobs: **6,015 of the 6,029 stale entries and 190 of the 191 unreached entries are already missing at least one blob**. Today, enforce mostly deletes entry files that can no longer be used.
- `enforce` still does not reach "only the current files". **682 undeclared entries are never judged** (section 3), including the play clone's units, whose sealed wrapper writes Declaration v1.
- Flipping enforce cannot hurt the running play build. Its lookups today made **0 fleet hits** (1,913 misses, 1,910 local hits).

## 1. Live dry-run numbers and the trend (`.retention/sweeps.jsonl`, 687 sweeps today, all `dry-run`, `retired` = 0 in all)
| time | entries | declared | live | grace | stale (extern / edited / deleted) | unreached | undeclared | would-refuse (per process) |
|---|---|---|---|---|---|---|---|---|
| 00:00 | 4,162 | 3,740 | 3,338 | 315 | 81 / 2 / 0 | 4 | 422 | 2 |
| 03:01 | 7,549 | 7,476 | 3,066 | 822 | 3,539 / 34 / 0 | 15 | 73 | 6 |
| 08:00 ("this morning") | 8,523 | 8,061 | 6,070 | 1,007 | 944 / 26 / 0 | 14 | 462 | 39 |
| 11:03 | 15,597 | 14,295 | 9,122 | 25 | 4,680 / 97 / 0 | 371 | 1,302 | 0 |
| 13:01 | 31,224 | 28,403 | 13,983 | 445 | 12,607 / 386 / 3 | 979 | 2,821 | 1 |
| 15:52 (peak) | 50,843 | 45,884 | 20,730 | 595 | 21,113 / 1,466 / 3 | 1,977 | 4,959 | 13 |
| 16:38 | 50,521 | 49,368 | 12,064 | 496 | 34,143 / 1,805 / 3 | 857 | 1,153 | 22 |
| 17:24 | 16,743 | 16,075 | 9,238 | 628 | 5,933 / 82 / 0 | 194 | 668 | 2 |
| 17:43 (last) | 20,882 | 20,331 | 12,420 | 273 | 7,155 / 277 / 0 | 206 | 551 | 3 |
- Over the day, stale entries made up between 0 % and 72.8 % of the declared ones (median 14.8 %). The size of the stale set follows edits to core files. At 17:30, **4,582 of the 5,947 extern-stale entries** trace back to a single edit of `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs` at 17:19:03.
- The `would-refuse-stale-on-arrival` counter resets with every server process, and there were 17 processes today. Summed over all of them it comes to **200** entries today.
- `.retention/evictions.jsonl` today: 226 collections took **199,012 entries, all for budget reasons**, including **1,286 entries of the protected class** (graph or input-manifest entries beyond the 10 % reserve). Last collection: 49.3 GiB of referenced blobs against a bound of 54.9 GiB.
- `repo fleet status` at 17:52 showed no retention line. Its fleet reader did not answer within 5 s at load 62, so `sweeps.jsonl` is the source used here.

Snapshot 17:30, by class (server judge; the oracle gives the same):
| verdict | kind / emit | count |
|---|---|---|
| live | input-manifest | 3,934 |
| live | graph | 2,324 |
| live | rust-unit link / metadata / other | 3,175 / 70 / 61 |
| live | build-script-run | 69 |
| grace | rust-unit / build-script-run | 724 / 12 |
| stale:extern | rust-unit link / other, stage-output | 5,935 / 9, 3 |
| stale:edited | rust-unit | 82 |
| unreached | rust-unit (registry, build-script executables) / build-script-run | 128 / 63 |
| undeclared | see section 3 | 682 |
Targets, where the `.semio-deps` blob still exists (std/core crate hashes of nightly-2026-07-20): native `aarch64-apple-darwin` and `wasm32-wasip2` both show up among both stale and live entries.

## 2. Correctness

### 2a. Whole store
- `R5/server-judge.ts` runs the server's own `judgeStalenessV1` over the read-only snapshot. `R5/oracle.py` is an independent fixed-point model written from the user's rule: an entry is current when every file it read equals the tree and every entry whose output it consumed is current. Registry entries are live while a live root reaches them.
- Result: identical counts, and **0 disagreements on all 17,271 judged entries**.
- Every extern-stale verdict comes in through a **blob edge on a code-emitting (`link`) consumer**: 5,936 link, 11 other, and 0 come in through `units` (name) edges.
- For a `link` unit, the key contains the raw digest of each rlib/rmeta extern (`📜️script.ts:140`; interface identity is used only for metadata-only units). So once the producer's source is edited, a build of the current tree computes a different key, and the stale entry can never be hit again. That makes the verdict sound.
- No stale entry was read after its root file changed. The check compared each entry's mtime, which `read` refreshes, against the mtime of its root file.

### 2b. Sample of 30 stale and 30 live entries, re-proved from first principles (`R5/sample.py`, seed 1124; full table in the appendix)
- **Stale (30):**
  - 7 edited: each declared source was re-hashed in the tree and differs.
  - 23 extern: each chain was followed to its root (up to 9 deep), and the root's file was re-hashed and differs. The roots are `📋️list` (×11), `🛠️tool-machine` (×4), `🔁️follow` (×4), `✏️editing` (×2), `🔢️units` and `🗂️natural-file…`.
  - 6 of them also have the current-tree rebuild of the same slot in the store: same sources, a different key, and externs that no longer name the stale blob. That is direct proof that no current build can hit the stale entry.
  - The sample covers 2 derive users (consumers of the `dsl_derive` proc-macro: `semio_s_artifact_flow_flow_diff`, `…_flow_editor_leaves`), a stage output and wasm32-wasip2 units.
- **Live (30):**
  - 13 workspace units, 3 metadata units and 2 derive users (`semio_os_dsl`, `semio_os_spr`): every declared source re-hashes equal, and every extern is held by a live or grace producer.
  - registry or host units and build-script executables (`rust-unit` without sources): reached from a live root.
  - 3 build-script runs: reached through the executable or OUT_DIR edges.
  - 5 input manifests: the crate entry file is present.
  - 4 graph records.
- **Wrong verdicts: 0.**

### 2c. Defect found and fixed: the tombstone rule turned retirements into permanent false stale verdicts (enforce only)
- **Old rule.** In `enforce`, every entry dropped with reason `stale` had its blobs written to `.retention/retired.json`. That included **unreached** entries, because the planner labels them `stale` too (`🧹️collector/🟦️.ts:47`). A consumer of a blob that nobody holds and that sits in the tombstones was then judged `stale:extern … gone` forever (`🍂️staleness/🟦️.ts`, old line 48).
- **Two real paths to a false stale on a current entry:**
  - (1) **Registry unreached.** After a core edit, all consumers of a registry unit R in the store are stale, so R is retired as unreached and tombstoned. A machine then rebuilds the current workspace unit against R from its own local cache and publishes it. The server judges this current unit stale, retires it 10 minutes later, and the same happens to everything downstream. This repeats on every republish.
  - (2) **Edit then revert.** The retired content becomes current again, but its tombstone stays.
  - The 191 unreached entries today, mostly registry units after the `📋️list` edit, are exactly the pattern in (1).
- **Proof.** The new test `🏷️retention/🧪️tests/🪦️tombstones/🟦️.ts` failed 0 pass / 2 fail on the old code ("app-v2 … stays live" received `false`).
- **Fix (schema first):**
  - New `Tombstone` definition in `🪢️slots/🧬️schema/🔣️.json`: `{blob,path,sha256}` or `{blob,toolchain}`. Types `StaleCauseV1` and `TombstoneV1`.
  - The judge gives every stale verdict the **cause** at the root of its chain, and a tombstone counts only **while its cause still holds** against the tree.
  - The store writes tombstones only for state `stale`, never for `unreached`, and stores the cause with each one.
  - Python oracle and shared fixture updated: one scenario rewritten, one new scenario (cause gone, cause holding, toolchain cause).
- **Tests** (one at a time, load 63–66): tombstones 2/0; staleness (fixture + oracle) 5/0; retention main 17/1, then 18/0 (the known 300 ms grace-timing flake from R2/R3); restorable 10/0; collector 8/0; slots 4/0; blob-bound 4/0; graph-class 8/0; server 10/0; unit-cache fleet stale e2e 1/0; gc supersession 8/0; manifest 5/0.
- Landing with sha256 values: `T/landings.md` "Wave 1122-dist R5". The live `retired.json` does not exist (no enforce has ever run), so there is nothing to migrate.

### 2d. Design risks still open (no instance today, so not blockers today)
1. **Interface-keyed metadata units** (`cargo check`/clippy).
   - The key uses the producer's interface digest, so after an edit that only touches a function body, the old consumer is still a hit for the current tree.
   - Its declaration, however, names the producer's raw rmeta digest, so the judge calls it `stale:extern`.
   - Today: 70 live metadata entries, **0 stale**.
2. **OUT_DIR consumers.**
   - The key covers the OUT_DIR file contents, but the declaration names the run key.
   - A build script edit that produces identical OUT_DIR files would retire consumers that are still hit-able.
   - Today: **0** extern verdicts through `units` edges.
3. **Unreached registry units.**
   - Liveness is "a live workspace entry consumes it", not "pinned by the current `Cargo.lock`".
   - Today 128 registry or executable units and 63 runs are unreached. Their names are in the current `Cargo.lock` (e.g. `tokio` 1.53.3, `parley` 0.5.0, `swash` 0.2.10; for the 23 whose manifest blob is gone the name could not be read).
   - Their consumers in the store are stale (14), unreached (98 chains), in grace (17) or already evicted (59).
   - Under enforce they leave until the next current consumer is published. They already lose their blobs in dry-run through the pin exclusion.
   - **Decision for the user:** accept this, or tie registry liveness to the lock file (name + version + features).

## 3. "undeclared" (682 at 17:30; 668 in the 17:24 sweep; between 52 and 4,959 today, median 315)
| class | count | what | still produced? | what enforce does |
|---|---|---|---|---|
| root hints (`{"version":1,"roots":[…]}`, 37 B, no refs; `⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🟦️.ts:115`) | 255 | published by the unit cache with no declaration | yes (newest 16:38); 31 older than 24 h, oldest 11-22 | keeps them until 30 days idle; 0 bytes of blobs |
| REAPI offload action results (`.semio-offload/root`, no sidecar) | 254 | results of remote actions, keyed by input root and command | yes (16:52–17:26) | budget tier 1; age rule only when idle 30 days |
| play units with Declaration **v1** | 153 (32 with content only in the play tree, 119 equal in both trees) | play's sealed wrappers (`/Users/ueli/.sinc/play/.cache/b/r/m/u/sealed/*`) still emit `version:1`; the server keeps no declaration, so they are never judged and never pass through the publish gate | yes, for as long as play builds | kept; **2.97 GiB of host blobs pinned** (the undeclared set is in the pin set) |
| stage results without a declaration | 15 | `stage.json` with no `retention` (16:53–17:26), from older stage code | yes | kept |
| other | 5 | older than 24 h | no | kept |
- They are not left over from before Declaration v2 (pre-R1 entries are gone, evicted by budget), and they are not K7 manifests (all 3,934 manifests are declared).
- **Proposed rule, applied in every mode:**
  - (a) A declaration of an unknown version from a known producer is refused at publish with 422 `undeclared-version`, so the play clone's v1 uploads stop. They gain nothing anyway: 0 fleet hits.
  - (b) Root hints become part of their unit: declare kind `input-manifest` with the crate entry file as the source, the same as K7 manifests.
  - (c) Offload action results get `maximumAgeDays` 1 when idle, because they can only be hit by an identical input root.
  - (d) Anything else undeclared and older than 24 h is retired.
- **What (d) plus (c) would retire today:** 36 entries (31 root hints and 5 others older than 24 h). After the play chain ends, (a) plus (d) also take the 153 play units and **2.97 GiB** of pinned blobs.

## 4. Play snapshot entries (`/Users/ueli/.sinc/play`, read-only)
- **What play does with the fleet cache today:**
  - Lookups (`y/lookups*.jsonl`): 1,913 misses, 1,910 local hits, **0 fleet hits**.
  - Uploads (`x/uploader.jsonl`): 6,550 entries uploaded, plus retries and failures (238 failed after 4 attempts, 888 deferred because the cache was down).
- **Where they end up:** all of play's sealed wrappers declare v1, so its entries are stored **undeclared**. They are not answered 422 even under enforce. 153 are in the store now.
- **Entries with play-tree content that carry v2 declarations:**
  - 11 edited roots with content equal to the play tree (for example `semio_ui_styling` built from the pre-05:52 `🔤️tokens/🦀️.rs`, and `semio_s_artifact_stdio_contract_editing`, `semio_s_plugin_norm_compliance_core`), plus **964 extern dependents**. All are already judged stale.
  - The producer is an older checkout on the fleet: `g1-m08` claimed 2 of these keys at 17:36. Which checkout it is cannot be proven from the records.
- **Effect of enforce on the running play build: none measurable.**
  - Play does not read from the fleet (0 hits) and has its local hits.
  - Its own uploads are undeclared, so they are neither retired nor refused.
  - Graph heads are kind `graph`, which is never stale.
  - Its local unit cache (host-CAS holder `unit-cache-15344…`) is untouched by server retention. Only 0.04 GiB of the retire set's blobs is held by play.
- **Still, flip after `[chain exit`:**
  - Flipping the policy file restarts the sidecar on its closure change.
  - The cleaner attribution matters more than any time it would save.

## 5. Capacity effect today (snapshot 17:30)
- **Server store (entry files plus sidecars):** the retire set is 6,220 entries = **5.0 MB of entry files + 28.9 MiB of sidecars**, out of 16.2 MB + 46.1 MiB in total. The undeclared set (if rule 3 is adopted) is 0.36 MB + 0.04 MiB.
- **Host store:**
  - Blobs still present that only the retire set references: **5.26 GiB**.
  - Of these, 4.22 GiB are held by the main checkout's local unit cache. They are **not freed** by server enforce; the local mirror `gc.staleness.mode` (still dry-run) governs them.
  - 0.04 GiB are held by play, 0.08 GiB pinned, and **0.92 GiB are held by nobody**. The CAS7 sweep takes those within minutes anyway, because dry-run already leaves stale entries unpinned.
  - **Net: enforce frees about 1 GiB sooner; the rest is already freed or held locally.**
- **Indirect effect, the larger one:**
  - In enforce, stale entries become tier 0 of the blob bound. Today they sit in the declared tier among the live entries.
  - The 199,012 budget evictions today took live entries by least recent use, while stale ones stayed in the index. 4,125 live entries already miss blobs (pin budget), including 1,412 input manifests with all their blobs missing.

## 6. Go/no-go checklist
| # | Check | State now |
|---|---|---|
| 1 | Judge equals the independent model on the live store; sample 30+30 has no wrong verdict | **met** (0 / 17,271; 0 / 60) |
| 2 | Tombstone defect fixed and the server running that code (store `🟦️.ts` sha256 `6a0adb46257e…`, landed 17:44) | **not yet live**: pid 79985 started 17:34. Wait for the sidecar's next closure restart, then check that the start time is after 17:44 |
| 3 | Play chain ended (`[chain exit` in `.🧬semio/🌐hub/play-cdn-1122/pinned/supervisor.out`) | **no** (`[run] site-build 15:14:08`) |
| 4 | Rule for undeclared entries (section 3) | **open**: without it 682 entries / 2.97 GiB stay outside "only current files" |
| 5 | Decide on unreached registry units (section 2d.3) | **open**: accept reachability, or tie it to the lock file |
| 6 | Latent false stale for interface-keyed check units and OUT_DIR-only build script edits (2d.1, 2d.2) | **accept for now**: 0 instances today; fix before heavy `cargo check` fleet use |
| 7 | One journal line per retired entry (today only counts + 8 examples per sweep, and per-entry lines for graph entries only) | **recommended before the flip**, so any wrong retirement can be proven afterwards |
| 8 | Toolchain rule | inactive (`SEMIO_REMOTE_CACHE_SERVER_TOOLCHAINS` unset on the server). Conservative, no action needed |
- **Flip:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json` → `remoteCache.server.staleness.mode`: `"dry-run"` → `"enforce"`. The cache sidecar reads it at start, so the change takes effect at its own closure restart. For a hand-run server: `repo remote-cache serve --staleness=enforce`.
- **Rollback:** set the key back to `"dry-run"` and **move `.retention/retired.json` aside**, because the judge reads tombstones in every mode. Retired entries are not restorable; they are rebuilt on the next miss.
- **What is journalled:**
  - `.retention/sweeps.jsonl`: one line per sweep with mode, counts per state and reason, `retired`, `refused` (`stale-on-arrival`) and 8 examples.
  - `.retention/evictions.jsonl`: one line per collection with entries, bytes, reasons (`stale` = retired, `budget`), plus one line per evicted graph or manifest entry.
  - `.retention/retired.json`: tombstones with their cause.
  - `GET /status` → `retention`, and the `repo fleet status` retention line.
  - Client side: skip reason `stale-on-arrival` in each machine's `uploader.jsonl` and in the skip counters.
- **Recommendation:**
  - **No-go today** because of items 2 and 3.
  - **Go** once 2 and 3 are met and 4 is decided. The retirements it would make are correct today, and it cannot hurt a play build that does not read the fleet.

## Appendix: the 60 sampled entries (seed 1124; "blobs" = present in the host CAS; target "?" = `.semio-deps` blob already reclaimed)

| # | verdict | entry | crate | kind/emit | target | blobs | evidence |
|---|---|---|---|---|---|---|---|
| 1 | stale:extern | `main/ac/33/33765d69659ef89dd07da15989e9d9e4485e6b57d9f63a8049122e3d88f48344` | semio_s_artifact_norm_en1991_checks_case_inputs | rust-unit/link | ? | missing 6/6 | chain 6 deep to semio_framework_value_paged_list (c30e7f8b34bc); root file 📋️list declared fe87409ad0 tree eee9d373f0; no current rebuild in store |
| 2 | stale:extern | `semio/ac/74/74784cd1853ec695d4a6cb2855f57bcca490f570292f96e5da819a5de6fcb3d9` | semio_framework_tool_typing_edit | rust-unit/link | ? | missing 4/6 | chain 7 deep to semio_framework_value_paged_list (fee79830aa57); root file 📋️list declared fe87409ad0 tree eee9d373f0; no current rebuild in store |
| 3 | stale:extern | `semio/ac/ed/ed22b94de39199f30d21e9c94636fed91960d30d3bab9814d29c577167710a12` | semio_ui_scene_surface | rust-unit/link | ? | missing 4/6 | chain 7 deep to semio_framework_value_paged_list (faaaa6d21a8a); root file 📋️list declared fe87409ad0 tree eee9d373f0; no current rebuild in store |
| 4 | stale:extern | `semio/ac/8d/8dc4c68d004523303bdfd80cb935c4a8d14b5de44efab98df85489d76b6546b0` | semio_framework_graph_manifest | rust-unit/link | ? | missing 5/6 | chain 6 deep to semio_framework_value_paged_list (fee79830aa57); root file 📋️list declared fe87409ad0 tree eee9d373f0; no current rebuild in store |
| 5 | stale:extern | `semio/ac/0c/0c25fba37c3204c572e086069f9dc9e6d316d44373faa435a7c865f2836ce95f` | semio_os_directory_check_in | rust-unit/link | ? | missing 3/6 | chain 4 deep to semio_framework_value_paged_list (6a1a22bfcf8e); root file 📋️list declared fe87409ad0 tree eee9d373f0; current rebuild 7e3a31f46ef3: same slot+sources, key differs, externs no longer name the stale blob |
| 6 | stale:extern | `main/ac/65/65107f4e6c5bfe865c04d11533173d548e508020fbd53c038adfb6a6fd1dffb7` | protocol | rust-unit/link | ? | missing 4/6 | chain 7 deep to semio_framework_value_paged_list (6d30b724f4e7); root file 📋️list declared fe87409ad0 tree eee9d373f0; no current rebuild in store |
| 7 | stale:extern | `main/ac/7e/7eef5db91aaa89faa6a0fcc769f23f71706a76fc51b2e53a50c7a4e895e0f2e7` | semio_framework_io_sqlite_model | rust-unit/link | ? | missing 3/6 | chain 5 deep to semio_framework_value_paged_list (c30e7f8b34bc); root file 📋️list declared fe87409ad0 tree eee9d373f0; current rebuild 8d2b809e19c1: same slot+sources, key differs, externs no longer name the stale blob |
| 8 | stale:extern | `semio/ac/2b/2b0f33ad25e6b205438235c4ca562bf909312935b6b86536ced55b10c6b1dde2` | semio_ui_contract_operations | rust-unit/link | ? | missing 4/6 | chain 6 deep to semio_framework_value_paged_list (8714eb155d75); root file 📋️list declared fe87409ad0 tree eee9d373f0; no current rebuild in store |
| 9 | stale:extern | `main/ac/24/240fc12c8e1d61feaa1ae666e5d38dff0a403eab48b3bc16a2a4ae9de808eba7` | semio_plugin_instance_ladders | rust-unit/link | ? | missing 4/6 | chain 2 deep to semio_plugin_instance_render (98f65d49ef4b); root file 🛠️tool-machine declared c82bb3a105 tree 7acbfdd070; current rebuild cc5984d425ea: same slot+sources, key differs, externs no longer name the stale blob |
| 10 | stale:extern | `semio/ac/10/10a4b730b1bd8aa4c345cc5b0807277a5c90029c54bb2023b50710ceaa2e2f38` | semio_plugin_typing_edit | rust-unit/link | ? | missing 6/6 | chain 2 deep to semio_plugin_tool_machine (423547f1c4ed); root file 🔁️follow declared 5b60e2b74c tree 19892b848b; no current rebuild in store |
| 11 | stale:extern | `main/ac/d0/d01da2a30d63b61abcc5b8e08190c793b7b59777fed4fc3f0423e1a252c3f48d` | semio_plugin_instance_dispatch | rust-unit/link | ? | missing 5/6 | chain 5 deep to semio_plugin_decl_natural_file_media (b1d84a89697f); root file 🗂️natural-file-and-media-artifacts declared eddf773899 tree 621caca157; no current rebuild in store |
| 12 | stale:extern | `main/ac/56/5664c9224f8a644efcdaf5632663c48f8127f6760a8ab09d42747d4882059fbc` | semio_s_artifact_stdio_step_ladder | rust-unit/link | wasm32-wasip2 | missing 1/6 | chain 4 deep to semio_s_artifact_stdio_contract_editing (3739d5c5c598); root file ✏️editing declared 70a8e47efe tree 4b63c00a27; no current rebuild in store |
| 13 | stale:extern | `main/ac/05/0578c684dbe3fc1e9b52951597efefc6419aa0a1267e4cf240800a2edca3b34e` | semio_plugin | rust-unit/link | ? | missing 4/6 | chain 2 deep to semio_plugin_instance_render (98f65d49ef4b); root file 🛠️tool-machine declared c82bb3a105 tree 7acbfdd070; current rebuild 53d2e4af3c00: same slot+sources, key differs, externs no longer name the stale blob |
| 14 | stale:extern | `main/ac/87/8764365fd84a641958435492cc120347eb185232940b49a48d70c89f28b8a768` | semio_s_artifact_stdio_semio_model_member | rust-unit/link | wasm32-wasip2 | missing 1/6 | chain 5 deep to semio_s_artifact_stdio_contract_editing (3739d5c5c598); root file ✏️editing declared 70a8e47efe tree 4b63c00a27; current rebuild 84061afd86ec: same slot+sources, key differs, externs no longer name the stale blob |
| 15 | stale:extern | `main/ac/48/48618058d889b6c74e4055a074130e54aee98e922f04b04a9873f6bf8896795a` | semio_s_artifact_norm_en1995_mut_deck_prestress_more | rust-unit/link | ? | missing 6/6 | chain 5 deep to semio_plugin_tool_machine (45b8d09377f1); root file 🔁️follow declared 1d954981e5 tree 19892b848b; no current rebuild in store |
| 16 | stale:extern | `main/ac/bd/bd50a98e60b194be9a8312b7a90f3dc56133039a4088ef2cbe29246992603ccf` | semio_s_artifact_norm_en1995_mut_tapers_more | rust-unit/link | ? | missing 6/6 | chain 5 deep to semio_s_plugin_norm_compliance_core (f81f726aad28); root file 🔢️units declared b4978c724f tree 1b25ab129a; no current rebuild in store |
| 17 | stale:extern | `main/ac/fc/fc2a5d01e13a911e8828591d757a3ac4ca049e61f3c530905fa5778175452227` | semio_plugin_typing_edit | rust-unit/link | ? | missing 4/6 | chain 2 deep to semio_plugin_tool_machine (7416cd1d76d7); root file 🔁️follow declared 1d954981e5 tree 19892b848b; current rebuild 30d6c2cd73ed: same slot+sources, key differs, externs no longer name the stale blob |
| 18 | stale:extern | `main/ac/65/65a1780190a89e80fcfae217efa01e4b62d18a7fe2878482ad0060caca55c9ad` | semio_s_artifact_norm_en1993_examples | rust-unit/link | ? | missing 6/6 | chain 3 deep to semio_plugin_instance_render (98f65d49ef4b); root file 🛠️tool-machine declared c82bb3a105 tree 7acbfdd070; no current rebuild in store |
| 19 | stale:extern | `semio/ac/5a/5a20f1a1b0efeea25eb95466fc7654d7eaea1950bcbf9d4939671d3645a7ed6f` | semio_s_artifact_flow_flow_diff | rust-unit/link | ? | missing 5/6 | chain 4 deep to semio_plugin_tool_machine (a7829829e258); root file 🔁️follow declared 5b60e2b74c tree 19892b848b; no current rebuild in store |
| 20 | stale:extern | `semio/ac/a1/a108ffc6025fc4023537e98409f81c02f01fb4663667c625852da3f0ae0344b3` | semio_s_artifact_flow_flow_editor_leaves | rust-unit/link | ? | missing 5/6 | chain 7 deep to semio_framework_value_paged_list (fee79830aa57); root file 📋️list declared fe87409ad0 tree eee9d373f0; no current rebuild in store |
| 21 | stale:extern | `main/ac/99/99877e099b1afa05b868f262b55cfd5cb91d3e0f065d5b997c0006a63a317dc2` | semio_s_artifact_norm_en1990_editor | rust-unit/other | ? | missing 4/4 | chain 4 deep to semio_plugin_instance_render (98f65d49ef4b); root file 🛠️tool-machine declared c82bb3a105 tree 7acbfdd070; no current rebuild in store |
| 22 | stale:extern | `semio/ac/57/5705697fe90a20a4ca15b951b73c256eba617a34d5c3a4c3cbfe34fdcad56547` | - | stage-output/other | ? | missing 2/2 | chain 9 deep to semio_framework_value_paged_list (faaaa6d21a8a); root file 📋️list declared fe87409ad0 tree eee9d373f0; no current rebuild in store |
| 23 | stale:extern | `semio/ac/28/2845e62aa157e0fca65c7574456ef86e0a16c7ed41391c3d174f4376c07cce79` | semio_plugin_describe | rust-unit/other | ? | missing 2/4 | chain 5 deep to semio_framework_value_paged_list (e498bfd27bac); root file 📋️list declared fe87409ad0 tree eee9d373f0; no current rebuild in store |
| 24 | stale:edited | `main/ac/cb/cba93445992fe3a9cdd2e7488f7fb49b3eea634f90e35a5ed95cd1d7410ee2ff` | semio_ui_styling | rust-unit/link | native | missing 5/6 | edited now: 🔤️tokens declared d2ba4a8e95 tree 64c24d72df |
| 25 | stale:edited | `main/ac/b9/b98e3e4764882d096e708270557e35fa527d9171cd091e8b788a73d4b5a5785e` | semio_s_artifact_norm_din16798_mutations | rust-unit/link | ? | missing 6/6 | edited now: 🧬️mutations declared 28d1468182 tree 55824995bf |
| 26 | stale:edited | `semio/ac/90/909263787ad91b5436c110c4e11d0aa338971a8ca358051e3dd4645b239edad8` | semio_s_artifact_procedural_3d_import_document | rust-unit/link | ? | missing 6/6 | edited now: 📥️import-document declared ff9bf9d57a tree 2ff9c5d923 |
| 27 | stale:edited | `main/ac/6d/6d30b724f4e790094ca5ed2205747db2bb923542bdd1dc109fd3533ab0757747` | semio_framework_value_paged_list | rust-unit/link | ? | missing 6/6 | edited now: 📋️list declared fe87409ad0 tree eee9d373f0 |
| 28 | stale:edited | `semio/ac/af/af2de51e7e27998bd2c17718874513326128d139b782d1a8a3e820885b09369c` | semio_renderer_wgpu_fixtures | rust-unit/link | ? | missing 6/6 | edited now: 🐚️shell declared 1164b1ebae tree 2e26014d50 |
| 29 | stale:edited | `main/ac/a0/a0d9542a551090ad797efba69f423d1cbcfeed574157c6935b6dec45f5cf59af` | semio_plugin_tool_machine | rust-unit/link | ? | missing 6/6 | edited now: 🔁️follow declared 5b60e2b74c tree 19892b848b |
| 30 | stale:edited | `main/ac/23/233dc3eb45a078dd82a498e500215fced81af1e404742f45b696928b67b90def` | semio_plugin_instance_render | rust-unit/link | ? | missing 6/6 | edited now: 🛠️tool-machine declared c82bb3a105 tree 7acbfdd070 |
| 31 | live | `semio/ac/34/3454257b92139495f4449a29b4a22aae490345c4deec29fca3651b4f5022d3da` | semio_os_channel | rust-unit/link | ? | missing 5/6 | 21 sources re-hashed equal; 12/13 externs held by a live/grace producer |
| 32 | live | `semio/ac/f0/f0822bd070c66fbc517ceb99648d64d6d4397826a0614865e5ccdbf360c99047` | semio_s_artifact_procedural_3d_transforms | rust-unit/link | ? | missing 4/6 | 7 sources re-hashed equal; 4/18 externs held by a live/grace producer |
| 33 | live | `main/ac/a7/a73e08e5a421568e1cc2e090d73ac8eebe644eedb257844631650d074de0d7d5` | semio_framework_job_return | rust-unit/link | ? | missing 4/6 | 8 sources re-hashed equal; 3/9 externs held by a live/grace producer |
| 34 | live | `main/ac/cb/cb093dede5291bc1368ab252e0d4932990bd0e32bc34c45a47e23f79ad9ed811` | semio_framework_value_native_storage | rust-unit/link | native | complete | 5 sources re-hashed equal; 1/1 externs held by a live/grace producer |
| 35 | live | `semio/ac/7c/7c5d225ec4d3ebce4629ee076fbfc1ce8ba16b643b9f90520351b4d92fd25848` | semio_framework_value_native_clone_backing | rust-unit/link | ? | missing 4/6 | 2 sources re-hashed equal; 4/4 externs held by a live/grace producer |
| 36 | live | `main/ac/b0/b0952b9d456e19c781e75a85f559622291586bfad467deac806f1346985dbf9a` | semio_s_artifact_norm_en1993_mutations_json_schema | rust-unit/link | ? | missing 5/6 | 2 sources re-hashed equal; 0/0 externs held by a live/grace producer |
| 37 | live | `semio/ac/b2/b2dfe2d9f0178f728ebd90c7e464808e20cee1ca2010ded38bacd3e08f28d79a` | semio_framework_replication_retained_key | rust-unit/link | native | complete | 2 sources re-hashed equal; 0/1 externs held by a live/grace producer |
| 38 | live | `semio/ac/92/923775dca1b34ef67c8ad9659569d61756e7d6610860ae8347a6903c54c270b4` | semio_framework_trace_memory_testing | rust-unit/link | native | complete | 1 sources re-hashed equal; 1/1 externs held by a live/grace producer |
| 39 | live | `semio/ac/1c/1c831be60bb9ffe4839a1ba4dc706092d1d1ad9950d2ca0b835330802bbd5aac` | semio_framework_value_native_slice_equality | rust-unit/metadata | native | complete | 3 sources re-hashed equal; 1/1 externs held by a live/grace producer |
| 40 | live | `semio/ac/f0/f059cbf614727be6d44c5f17d45db054012e6b2d51031ef258ecd80c1fcd9924` | semio_framework_value_native_shared_field | rust-unit/metadata | native | complete | 3 sources re-hashed equal; 7/7 externs held by a live/grace producer |
| 41 | live | `semio/ac/fa/fa968c335aec3397637f4f8697352279a39d5de5d9e41b559c464d2fce4ad9be` | semio_framework_value_retirement | rust-unit/metadata | native | complete | 16 sources re-hashed equal; 5/5 externs held by a live/grace producer |
| 42 | live | `semio/ac/39/39324ae67c0364148b712ba63160db2ec602ee7d7ff21deaf6378341f5ecadd8` | semio_os_dsl | rust-unit/link | ? | missing 4/6 | 23 sources re-hashed equal; 11/11 externs held by a live/grace producer |
| 43 | live | `semio/ac/aa/aa98792875dc404afee8ae99c5c9e12b03262bb3e718e5fddad5ea61db2760ee` | semio_os_spr | rust-unit/link | ? | missing 4/6 | 4 sources re-hashed equal; 11/11 externs held by a live/grace producer |
| 44 | live | `semio/ac/20/206e2bb0a8d00ad9376d91b21cc4336f03052f64ad38384e2c5b09ba0d9a8544` | unicode_script | rust-unit/link | native | complete | 0 sources re-hashed equal; 0/0 externs held by a live/grace producer; reached from a live root (no sources) |
| 45 | live | `semio/ac/52/5205a71347dbee0be7b396ca67a42419d983f7a693548739c6a4b71ed4e831a2` | build_script_build | rust-unit/other | ? | complete | 0 sources re-hashed equal; 4/4 externs held by a live/grace producer; reached from a live root (no sources) |
| 46 | live | `semio/ac/f8/f8035ed2525684bdc627a03611cfe3c2d5fa94d60e17b49c9fab302ec0c4e50e` | build_script_build | rust-unit/other | ? | complete | 0 sources re-hashed equal; 4/4 externs held by a live/grace producer; reached from a live root (no sources) |
| 47 | live | `semio/ac/7c/7cde19e18105e9ce19473edf98ba153c75bf4d9b36e47fbc42614750d696f0d8` | errno | rust-unit/link | ? | complete | 0 sources re-hashed equal; 1/1 externs held by a live/grace producer; reached from a live root (no sources) |
| 48 | live | `semio/ac/93/934eb24a39ff0a24a2d9c040e5dc36ba07bead0f09256d487cb5f94d5137832d` | crypto_common | rust-unit/link | native | complete | 0 sources re-hashed equal; 2/2 externs held by a live/grace producer; reached from a live root (no sources) |
| 49 | live | `semio/ac/f8/f8c9512d2de0057f3728dff292a52dbe196dcbc9eb533515ba641fb0f0d1e6e0` | - | build-script-run/other | ? | complete | 0 sources re-hashed equal; 1/1 externs held by a live/grace producer; reached from a live root (no sources) |
| 50 | live | `semio/ac/cf/cf18f3b89e38ebbdeb7cd24271b7b14fca6048ad0bee850d8b3e3cbdbb51bf01` | - | build-script-run/other | ? | complete | 0 sources re-hashed equal; 0/1 externs held by a live/grace producer; reached from a live root (no sources) |
| 51 | live | `semio/ac/41/41d6e937392497f04d373b350355f0050882e107494f684ed624cc70b788ca0c` | - | build-script-run/other | ? | complete | 0 sources re-hashed equal; 1/1 externs held by a live/grace producer; reached from a live root (no sources) |
| 52 | live | `main/ac/51/51532fc2cfdfcc2e521cb1b5b8a364ba20d6797cfec34c6d31ca1541a45ff055` | - | input-manifest/other | ? | missing 1/1 | 1 sources re-hashed equal; 0/0 externs held by a live/grace producer |
| 53 | live | `semio/ac/ab/abe0612c5ef2990955b72a0fdbac035bb823c691724794a867b76ddccbbe1fc5` | - | input-manifest/other | ? | complete | 1 sources re-hashed equal; 0/0 externs held by a live/grace producer |
| 54 | live | `main/ac/77/77ced0c1e6d4981682d0327ec38619d3840a5d9f68682b73cfb5f039777a3f17` | - | input-manifest/other | ? | complete | 1 sources re-hashed equal; 0/0 externs held by a live/grace producer |
| 55 | live | `main/ac/f7/f7ee2802d11cc8f5accedfc74fb7a3c4e9df82227b4e1181c7737bda0222d4f9` | - | input-manifest/other | ? | complete | 1 sources re-hashed equal; 0/0 externs held by a live/grace producer |
| 56 | live | `main/ac/5a/5a4a339a47aa0db8d59cf1e6d1703b6fbfd8400549c3fbd969e8e7f74eeb9adf` | - | input-manifest/other | ? | complete | 1 sources re-hashed equal; 0/0 externs held by a live/grace producer |
| 57 | live | `main/ac/89/89680678a73fb24832dfb2a1a5340bd1ac94341be06872232a0db7db06446e70` | - | graph/other | ? | missing 2/3 | 0 sources re-hashed equal; 0/0 externs held by a live/grace producer |
| 58 | live | `main/ac/ab/ab15518fb2afc5ab90850c30adcb13f574d678f0e39879ceeedd916d665f0909` | - | graph/other | ? | complete | 0 sources re-hashed equal; 0/0 externs held by a live/grace producer |
| 59 | live | `main/ac/1e/1ec00e80fc12f93e667726de89aae4cb9028a341e66c6c4cf99bda0307a4ace6` | - | graph/other | ? | missing 2/3 | 0 sources re-hashed equal; 0/0 externs held by a live/grace producer |
| 60 | live | `main/ac/cf/cf70bf3dc589339c2c6b9d9eff7ed9209f5453f7f7be1dd3b53ac580a504e675` | - | graph/other | ? | complete | 0 sources re-hashed equal; 0/0 externs held by a live/grace producer |
