# Result E35: adoption latency of single-target runs and `repo cargo metadata`

## Verdict
1. Dependency adoption on a WARM machine is not the cost: it is about 16 s. The 52 minutes of E28/E22 were a STALE dependency generation receipt (full ownership walk plus install chain), triggered by a real manifest edit. I did not change the stale path (see "Why not"). 
2. `repo cargo ... metadata --no-deps` paid a full workspace `cargo fetch --locked --offline` (registry custody, lock check) for nothing. Fixed: inputs phase 82 s -> 1.2 s; total 104 s -> 53 s (see numbers).

## Measured: `repo run repo:test-cache-reset --dry-run --exclude-dependencies` (script `T/🧰️scripts/E35/run.sh`, load 30-45, total 714 s, exit ok)
| t (s) | phase |
|---|---|
| 0-175 | graph slice read (inventory 9,659 observations; owners 3,600) |
| 175-194 | capacity (admitted 2,304 MB, the E18 ceiling works) |
| 194-278 | `build:source-admission` containing `dependencies:generation-verify` 221.6-236.7 s = **about 16 s** (6 passes of 2-4 s, 128 progress ticks, receipt MATCHED, no walk, no install, no sync lease) |
| 278-548 | Nx tool: storage planning 52 s, source index, source-verification, `nx-sdk-derived-namespace` passes, authorizer bundle checks |
| 548-690 | implementation-source read 130 s, Nx daemon graph, journal replay |
| 690-714 | Nx run, publish nx-graph |
So warm adoption = 2% of the run; Nx tool admission (270 s) and the graph slice (175 s) dominate. E28's split is visible: no new generation was built.

## Why a stale adoption takes tens of minutes (evidence, not fixed)
- Receipt `.cache/b/d/m/generation/<scope>.<sha>.receipt.json` per scope (6 scopes): `inputs` hold byte sha256 of about 310 `package.json` per scope plus patches, `membership.texts` 324 manifests, frontier 100,050 directory rows (lstat each), execution 7,806 provider edges. Any byte change of one manifest -> `input-changed bytes` -> mismatch -> full `currentOwner()` walk -> republish of all six receipts (seen: receipts rewritten 08:49-08:50 by peers).
- Change journal is OFF on this host: `~/Library/Caches/semio/repo/<id>/journal/state.json` says `backend none, ready false, reason canary` (fseventsd delivers 0 events, handover item 10) and the only vouch record is from Nov 20. So the O(1) vouch path (`openVouchV1`) never applies; every verification lstat-checks the 100k rows (fast when matching, about 2-4 s per pass) and every mismatch walks.
- Manifest churn is low (10 files in 10 h; last 07:50 = E28's own landing in `🛠️tools/package.json`, which staled E28's run). E8's stage digest is install-relevant only, but this run path (receipt verification) compares RAW bytes, so it does NOT use the same digest.
- Why not fixed here: tolerating byte changes in inert fields is NOT sound for this receipt. The source-owner manifest profile reads `exports` (provider edges) and `name`/`version`/dependency groups; `exports` is in the adopt module's inert list (`🪞️adopt` `inertManifestFields`, right for install, wrong for provider edges). A correct fix is a receipt field `profileSha256` = hash of the manifest profile (`javascriptDependencyInstallManifestV1` plus the exports resolution), compared in `verifyGenerationReceiptV1` for `package.json` rows; that is a receipt format change (schema, capture, verify, replay, oracle) and I did not land it blind. Recommended next step with the exact seam: `🧾️generation/🟦️.ts` lines for `receipt.inputs` and `membership.texts` checks.

## Landed: `repo cargo -- metadata --no-deps` does not adopt dependencies
Cause (profile `SEMIO_REPO_PROFILE=1`): `cargo.compiler.workspace-body` self 84.3 s with no inner phase; `SEMIO_CARGO_TIMING=1` phases: store 4.8 s, **inputs 82 s**, child (cargo) 3.6 s, consume 0.06 s. `withRepositoryCargoInputsV1` ran for every command: `ensureRepositoryCargoLocksV1`, then `withRepositoryCargoRegistryReadyV1` = `cargo fetch --locked --offline` of the whole workspace lock under registry and compiler-writer custody.
Change (schema first): `🏃️process/🦀️cargo/🚪️launcher/🧬️schema/🔣️.json` gains `dependencyFree` (`[{command:"metadata",flags:["--no-deps"]}]`) in `Policy` (closed, required) and `Configuration`; `📥️dependencies/🟦️.ts` gains `repositoryCargoDependencyFreeV1(args)` and an early branch in `withRepositoryCargoInputsV1` for resolution consumption without additional manifests: configuration views plus `admitRepositoryCargoInputsV1` only, no lock check, no fetch, no registry custody. Compiler kinds, `tree`, `fetch`, `pkgid`, and `metadata` without `--no-deps` are unchanged.
Test: `📥️dependencies/🧪️tests/🆓️dependency-free` 1/0 (10 argument vectors in JSON, cross-checked against an independent Python implementation run with `python3 -I`).
Measured (`T/🧰️scripts/E35/cargo.sh`, same command E34 used):
| run | load | total | inputs phase |
|---|---|---|---|
| E34 | 40-50 | 351 s | - |
| before, this lane | 40 | 346 s | not timed |
| before | 26-27 | 104 s | 82 s |
| after | 33-34 | 53 s | 1.2 s |
Remaining 53 s: boot + `build:source-admission` 27 s, store 7 s, `source.capture` 6-10 s, cargo child 4 s. Output verified: valid JSON, 633 packages, exit 0.
Not verified: a before/after at identical load (load differed); cargo calls that really need dependencies still pay the fetch (82 s at load 27): the registry-ready check should be a stamp memo, not a cargo spawn (next lever). The first `repo cargo` run after my schema edit failed with `/dependencyFree is not allowed` (Policy schema was closed) and was fixed within a minute; any run started between those two edits (09:21) may have failed.

## Test status
dependencies `🧪️tests/🟦️.ts` 11 pass / 1 fail: `wasmCompilerSelectionArgumentsV1 is not defined` in a peer's wasm module (not this change). launcher `🧪️tests/🟦️.ts` 10/2: isolated-store sweep test; I did not run a baseline, nothing of this change touches that path.

## Other lanes must know
- A peer's half landing (`adoptedReadOnlyDirectoryV1` export missing in `🔒️leases/🎛️control`) broke every `repo` command for a few minutes at about 09:12.
- Host: change journal unavailable (fseventsd), so no vouch; E8/E28 numbers at load 40-55 include it.
- Scripts kept: `T/🧰️scripts/E35/run.sh`, `cargo.sh`. Landing recorded in `T/landings.md`.

# Round 2

## (2) Cargo registry check with a persisted, content-keyed digest memo (landed, measured)
Root cause found by timing inside the wrapper (temporary `[DEBUG]` timers, removed): for a `repo cargo ... metadata` that needs dependencies the 63-82 s of the `inputs` phase were NOT the fetch (`cargo fetch --locked --offline` 5.6 s) and not the lock check (0.4-0.9 s): they were `sourcesIntact(true)` = `fetchedSourcesV1` hashing every file of every unpacked registry crate (65k files, 1.3 GB), 21 s before and 34 s after the fetch, because `archiveDigestV1` memoises only in process and its map is cleared at 16,384 rows (65k files thrash it even inside one process).
Change (`🏃️process/🦀️cargo/📥️dependencies/🟢️ready`): new member `🧊️digest-memo` (schema first: `Policy`, `Memo`; module `openDigestMemoV1`): a persisted map path -> `dev:ino:size:mtimeNs:sha256`, stored at `<store root>/cargo-digest-memo.json` (9.8 MB), loaded lazily, written atomically after each `sourcesIntact` (also on failure), answering only for the exact stamp (same trust model as the in-process memo: any in-place rewrite changes mtime/size/ino and is re-hashed); absent, oversized, malformed or ill-formed file = empty memo, a failed write never fails a build. `requestIdentity` now judges `--locked`/`--frozen` on the fetch requests (always `--locked`) instead of on the user's argv, so the existing `fetched` receipt (content-keyed by Cargo.lock, toolchain, configuration digests and the registry archives) also applies to calls without `--locked`; any change of a lock, archive or crate file falls through to the full path as before.
Test: `🧊️digest-memo/🧪️tests` 2/0 (stamp key and sha256 equal a Python oracle run with `python3 -I`; exact-stamp answers, restart persistence, a touched file misses, schema via Ajv; seven broken-file cases give an empty memo; unwritable path never throws). Existing `🧾️fetched` tests 16/0; dependencies main 11/1 (peer's wasm failure unchanged).
Measured (`metadata --format-version=1 --offline`, `SEMIO_CARGO_TIMING=1`, inputs phase): before 63-70 s; first run after the change (memo cold, writes it) 33 s; warm 5.4 s and 13.5 s (load 27-34, second run noisier). Warm = lock check 0.6 s + fetch 5.6 s (the receipt path is not yet taken for this request, see not verified) + crate stat checks.
Not verified: that the `fetched` receipt is now hit for non-`--locked` calls (the fetch still ran 5.6 s in my last timing; the memo, not the receipt, delivered the win); no test of the identity change itself; stat-only trust (a same-size same-mtime in-place rewrite of a crate file is not detected, identical to the in-process memo).

## (3) Warm single-target dry run, second measurement (script `T/🧰️scripts/E35/run.sh`, `--dry-run --exclude-dependencies`)
Total 666.7 s, load 22.5 at start, 40.6 at end (round 1: 714 s at 30-45).
| t (s) | phase |
|---|---|
| 0-17 | graph service start (peer landing restarted it) |
| 17-333 | graph slice read: 94 s inventory check (9,506 observations), a second slice after a service restart at 259 s (36 s), plan answered at 333 s |
| 333-334 | capacity: admitted 2,304 MB with no wait |
| 334-400 | `build:source-admission`; `dependencies:generation-verify` 359.5-370 s = **11 s**, receipt matched |
| 400-419 | Nx storage planning 20 s |
| 419-510 | Nx tool: source index/verification passes, one authorizer bundle rebuild (455-461 s, a peer landed in the authorizer closure) and its derivative recheck, namespace passes |
| 510-664 | Nx graph inventory and journal replay (80 s gap at 526-600 s), project graph, run |
Dependency adoption is 11 s (1.6 percent); graph slice (50 percent) and Nx admission plus graph (40 percent) are the cost. No dependency lease, install or sync appeared.

## (1) Dependency receipts keyed by a manifest profile (landed, live proven; walkStamp /2 -> /3)
Design (decisions backed by code reading): a `package.json` row of a receipt (per-scope `inputs.sources[]` and the global `membership.texts[]`) now carries `profileSha256` = sha256 of the manifest minus the inert top-level field `scripts`, canonically ordered (null for anything that is not a manifest object, which is then compared by exact bytes as before). I did NOT exclude `semio` although the install projection does: `semio.workspace.members/owners` define workspace membership, which the receipt's ownership tier depends on (a test pins this), and `exports`, dependency groups, `name`, `version`, `workspaces` stay in the profile (provider edges read them).
Seams changed (all in `⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation`): schema (`Source`, `Text` require `profileSha256`; `walkStamp` -> `semio.dependency-generation-walk/3`, the one deliberate re-key: every receipt on every machine is `receipt-invalid`/`protocol-changed` once, no migration code), new member `🪪️profile` (schema, `manifestProfileSha256V1`, tests), capture (`texts` profile derived in the storage glue from the file only while it still holds the exact retained text; `sources` profile from the captured bytes), and verification in three tiers:
- `inputs.sources`: a manifest whose bytes or inode changed is accepted when its live profile equals the recorded one (identity check dropped for profiled rows only; lock, patches, config keep exact bytes and identity);
- `membership.texts`: bytes or profile;
- frontier `s` rows of a manifest that has a profiled text row, and execution `provider` rows of such a file: stamp and inode drift tolerated for a regular file (the texts tier has already proved the profile equal). This was needed: a replica or atomic-save edit replaces the inode, which the frontier state tuple and the provider stamp both record; without it the profile alone changed nothing in practice (first live trial showed `provider-changed` on the edited manifest).
Tests (Python oracle `🧾️generation/🧪️tests/🐍️.py` extended with an independent profile implementation and the same relaxed source/text rules; its tiers stop at texts, so frontier and provider relaxations are TS-fixture only, stated here): profile member 3/0 (17 cases, TS equals Python, schema via Ajv, `semio` stays covered, excluded set is a subset of the install projection's); generation 17/0 with new cases: member `scripts` edit matches, member `exports` edit falls back `manifest-changed`, member dependency edit falls back, root manifest `semio` edit falls back `input-changed`, root manifest replaced by equal bytes (new inode) matches, provider member manifest `scripts` edit and replacement match; ingress 10/0; managers 94/0, concurrent 18/0, format-stamps 17/0. Existing tests whose law changed on purpose were adapted (appending a space to a manifest is now an inert edit: glue and ingress tests use a `version` edit). `🛡️followup` 7/1: the failing test is the cancellation test of `writeJavascriptDependencyGenerationFilesV1`, which this change does not touch; no baseline run.
Live (this machine, quiet at 17-19 load, `--dry-run --exclude-dependencies`, `T/🧰️scripts/E35/run.sh`):
| run | what | result |
|---|---|---|
| re-key | first run after landing | `receipt-invalid` at 189 s, walk, re-publish, run completed (10:27) |
| steady | no edit | 276 s total, **dependency verify 3 s, no stale** |
| edit, before provider tolerance | `scripts` key added to `⏯️tool-run/📦️packages/🦀️rust/package.json` | stale `provider-changed` on that manifest at 100 s; stale chain then ran 1,203 s until I cancelled it (Nx stage waiting on `resource-directory-admission`, a peer cost) |
| edit, after provider tolerance | same edit still present | **no stale**, dependency verify 295.8-306.8 s (11 s, matched), run completed in 870 s (load 28-32; slow in Nx admission, 8 passes of tools verification) |
The probe edit was reverted afterwards (file byte-equal to the original).
Honest limits of "only the affected scope(s)": the `texts` list is global (the same 324 manifests in every one of the six receipts), so a RELEVANT manifest edit (`exports`, dependency, `name`, `version`) still stales all six scope receipts; only the `sources` list is per scope. Making `texts` scope-local needs the ownership walk to say which scope owns each manifest; not done. What now holds: an inert edit (`scripts`, key order, whitespace, equal-bytes replacement, new inode) republishes nothing in any scope; a relevant edit republishes through the walk as before.
Not verified: `exports` edit live (fixture only); Windows/Linux (stat/inode semantics); the worker machines (they re-key once on their next sidecar restart: expect one full walk each, about 10 min on a 16 GB Mac under load, which is the price of the single re-key).
