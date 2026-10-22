# result-dep-receipt (wave 1022a, executor dep-receipt): dependency generation receipt keyed by what makes the generation, never by the code that verifies it

## Status

- Fix LIVE since **03:13:06 CEST (10-22)** (schema + both generation modules + incremental + sync-gate report + bootstrap report + tests, installed within one second by `dep-receipt-tools/install.py`). Follow-up at **03:15:32** (descriptor reads, see 2.5), docstring emoji uniqueness at about 03:16. No git writes, no sub-agents, no process of mine left running. Nothing committed.
- First new-format receipt published by a real command at **03:18:01** (`primeA`, scope `.`), a second one at 03:25:25 (topology churn by peers, see 4).
- Verified live (section 4): two concurrent `bun repo show targets --json` started right after a harmless edit of a repo-library file passed the dependency stage without any wait on `javascript-dependency-build`, while another process (old code, pid 54769) held that lease EXCLUSIVE for the whole window.
- Open for others (section 5): old-code processes still in the build queue will each still run the 22 CPU-min sync (they run the old verifier); loader false positive for manifests under `🧬️schema/`; one stale pre-existing law.

`L` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `D` = `L/⚡️caching/🚀️bootstrap/📦️dependencies`, `C` = this ticket folder.

## 1. Root cause (read from the code, then confirmed by the live runs)

1. `D/🧾️generation/🧬️schema/🔣️.json` made the receipt carry `implementation`: the sorted entry list plus a content-addressed `.closure.ndjson` of every file of the runtime import closure of the walk (`captureBuildSourceClosureV1` over 6 entries, up to 2,048 files, about 1,000-2,000 in practice, sha256 + bytes per file). `verifyGenerationReceiptV1` re-hashed all of them as its FIRST tier and answered `implementation-changed` on any difference; `captureGenerationReceiptV1` additionally refused to seal (`implementation-unsettled`) while any closure file was younger than 5 s.
2. Consequences chain, as seen by the fleet:
   - every live edit of any file in that closure (build library, workspace walk, storage, leases, loader-adjacent files: a dozen agents all day) made the receipt `stale`;
   - a stale reader receipt costs every command a complete ownership + provider walk (30-65 s, 20-60 CPU-s) and can never publish a new receipt while a closure file is "unsettled";
   - the sync gate `verifyJavascriptDependencySyncCurrentV1` (`D/🧾️incremental/💾️storage/🟦️.ts`) calls the same verifier per scope, so a closure change also turned "sync current" into `stale("generation:<scope>")` and made the producer redo the complete walk under the EXCLUSIVE `javascript-dependency-build` lease (22 CPU-min measured by graph-perf);
   - queued waiters decide "is my work still necessary" (`wait.ready` = cheap `recheck`) with the same verifier. The holder publishes a receipt sealed with ITS closure; every waiter has a different closure (new live edits) so it sees `implementation-changed`, takes the lease in turn and repeats the complete sync: the serial convoy of 1,300+ s per command.
3. Nothing else in the generation identity depends on code bytes: `javascriptDependencySourceOwnerRuntimeV1` hashes schema constants + bun/node versions + platform/arch; the physical generation key is Source bytes + recipe + runtime. Only the receipt (the O(changed) proof) was keyed by the closure.

## 2. What changed (all live, schema-first)

### 2.1 Schema `D/🧾️generation/🧬️schema/🔣️.json`
- `Protocol.const.implementation` (entries, extraFiles, closureSuffix, maximumFiles, reader) is gone; replaced by the declared, versioned stamp `walkStamp: "semio.dependency-generation-walk/1"`. Bump it only when the walk semantics change; this is the one place implementation identity enters the receipt.
- `Receipt`: `implementation` removed, `walkStamp` (string 1-128) required. `Evidence.name` pattern is `(frontier|execution)`; `Implementation` and `ClosureRows` definitions removed.
- `mismatchClasses` / `MismatchClass`: `implementation-changed` -> `protocol-changed`. First tier in `tiers` is now `protocol` (O(1), "never the bytes, paths or timestamps of the code that performs the walk, the installation or the verification").
- New `Protocol.const.languages` (`en`, `de`) and `Protocol.const.texts` (EN/DE, same placeholders): one message per mismatch class plus detail variants (`input-changed.bytes|identity|appeared`, `manifest-changed.bytes|unreadable`, `membership-changed.state|directory|listing`, `provider-changed.identity|target`, `runtime-changed.<field>`), `absent`, `seal`, `stale`.

### 2.2 `D/🧾️generation/🟦️.ts` (neutral)
- Removed: `GenerationImplementationV1`, the closure tier, the `implementation-unsettled` deferral, `compareBytes`, closure evidence. Receipt carries `walkStamp`.
- Verify tier order: `protocol-changed` (detail = the stamp the receipt was sealed with) -> `runtime-changed` (detail names the field: bun, revision, node, platform, arch, sha256 = installer recipe/profile) -> producer configuration -> bindings -> ready -> inputs -> manifests -> membership -> provider replay.
- New `describeGenerationMismatchV1(language, {class,path,detail})` (no hidden default language; unknown language throws) and `generationCauseV1`.
- Repository DATA (input files, manifests) is now read through a descriptor (`readGenerationData`), see 2.5.

### 2.3 `D/🧾️generation/💾️storage/🟦️.ts`
- Removed `captureJavascriptDependencyGenerationImplementationV1`, `javascriptDependencyGenerationImplementationFileV1`, `repositoryRoot`, replay pseudo-closure. `JavascriptDependencyGenerationSubjectV1` is now only `{ambient}`; `captureJavascriptDependencyGenerationReceiptV1(workspace, store, control, walk, observed, startedAtMs, subject?)` (the `invocation` and `implementation` parameters are gone: the capture no longer runs the build source-closure reader at all, which also removes its per-publish cost and its dependency on admitted resolution maps).
- New `reportJavascriptDependencyGenerationMismatchV1(scope, mismatch, language?)`: `[DATA] Store dependency generation receipt stale {class,path,detail}` on stderr plus one human line in `SEMIO_LANGUAGE`/locale language (EN first, DE second). The stale branch of the publisher uses it.
- Retire sweep deletes any unreferenced `<64 hex>.<kind>.ndjson` (so orphaned `.closure.ndjson` of the previous format disappear, no compat code).

### 2.4 Users of the verdict
- `D/🧾️incremental/💾️storage/🟦️.ts`: the sync gate result carries `cause` (`generationCauseV1`); stale reason stays `generation:<scope>`.
- `D/💾️storage/🟦️.ts` (producer, sync gate): `[DATA] Store dependency sync stale {reason,cause}` plus the localized line.
- `L/🏗️build/🚀️bootstrap/🟦️.ts`: a reader that finds a mismatching receipt now says why (once per command, not on the publish pass). Reader and waiter logic untouched: with a closure-free receipt `fast.status==="match"` (cheap tier) and the existing `recheck` coalesces queued builders as designed.

### 2.5 Loader interaction found and fixed on the way
The old stale line of the 03:01 probe was `class=unreadable detail="Uncaptured implementation schema read refused"`: the loader's captured `readFileSync` refuses every `.json` below a `🧬️schema` directory that is not an admitted image, and 20+ workspace manifests live there (e.g. `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🟦️typescript/package.json`). The verifier read manifests/inputs with `readFileSync`, so such a receipt could never verify under the real loader. Verify now reads repository data through `openSync/readSync` (law: the neutral module may not contain `readFileSync`). Loader false positive itself is left to loader-fix (5.3).

## 3. Tests (language-agnostic, with a third-party / independent oracle)

Files (all under `D/🧾️generation/🧪️tests/`): `🔣️.json` (fixture: closure keys removed, new cases and `reasons`), `🟦️.ts`, new `🐍️keying.py` (independent Python verifier + Python renderer), `🚪️ingress/🟦️.ts` and `🛡️followup/🟦️.ts` (subject shape), `D/🧾️incremental/🧪️tests/🟦️.ts` (suffix list).

Fixture cases (JSON, no TS needed to read them): every "tool source ... edited" case (stand-in files `impl/entry.ts`, transitive import, dynamic import, cjs asset, json asset, module-owned schema, removed, replaced by symlink, rewritten, two files edited together) -> `match`; `lock edited` -> `mismatch:input-changed` path `w:bun.lock` detail `bytes`; `lock replaced by equal bytes` -> `identity`; `bunfig appears` -> `appeared`; `member manifest edited` -> `manifest-changed pkgs/b/package.json bytes`; `workspace npmrc appears` -> `configuration-changed w:.npmrc`; `walk protocol stamp changed` -> `protocol-changed` detail `.../0`; runtime/revision -> `runtime-changed` detail `bun`/`revision`. 23 of the 62 fixture rows carry `oracle:true`: the Python verifier (`🐍️keying.py verify`, reads only the sealed receipt JSON and the live tree) must return the same verdict, path and detail as the TypeScript verifier.
New laws: (a) the sealed receipt has exactly the schema's required keys, `walkStamp === protocol.walkStamp`, evidence files are `frontier` + `execution` only, schema/neutral modules contain no closure reader, no `implementationFile`, no `loadedAtMs`, no `readFileSync`, and the reader function slice contains neither `acquire*Lease` nor `"exclusive"` (a reader never takes the exclusive resource); (b) reasons: every class (+ `absent`, `seal`, `stale`) has EN and DE text, same placeholders, EN != DE, 8 fixture rows (EN and DE literals) equal the TypeScript renderer and the Python renderer.

Commands and outcomes (plain `bun test ./<file>`; the generation test file now runs without the repo test target because the closure capture, which needed the admitted resolution map, is gone; before my change 10 of its 12 tests failed with "Actual admitted Source resolution map required"):

| file | result at 03:16 |
|---|---|
| `D/🧾️generation/🧪️tests/🟦️.ts` | 11 pass, 1 fail (pre-existing, not mine: "stamp and anchor ... storage laws stay pinned" asserts `projectListing(` occurs 2x in `🗂️workspaces/🟦️bun/🟨️.cjs`, a peer edit made it 6x) |
| `.../🚪️ingress/🟦️.ts` | 10 pass |
| `.../🛡️followup/🟦️.ts` | 7 pass |
| `D/🧾️incremental/🧪️tests/🟦️.ts` | 14 pass, 1 fail (pre-existing "incremental scan ledger": needs a parent Script process invocation, identical before the change) |
| `D/🚦️concurrent/🧪️tests/🟦️.ts` | 16 pass |

A scratch copy (`/private/tmp/dr1022`, short path because of the 256-unit source path law; removed) was used to prove the change before it went live; `tsc -p` over the changed files showed no error in them (only missing-copy noise).

Example of the reason text (real renderer):
- EN `The dependency generation receipt of scope . is stale: An installer input changed its content: w:bun.lock.`
- DE `Der Abhängigkeits-Generationsbeleg des Bereichs . ist veraltet: Eine Installer-Eingabe hat ihren Inhalt geändert: w:bun.lock.`

## 4. Live verification (timestamps CEST, 10-22)

Tools: `C/dep-receipt-tools/{run1.sh,pair.sh,tier-cost.mjs,stage.py,stage_tests.py,schema_edit.py,install.py}`, logs `C/🗑️generated/dep-receipt/` (`pairs.log`, `try1-A.*`, `try1-B.*`, `primeA.*`, `probe0.*`).

| time | event |
|---|---|
| 02:59:50 | `probe0` (old code) `show targets --json`: after 90 s the log line `receipt stale class=unreadable detail="Uncaptured implementation schema read refused"` (2.5); no wait on `build` because no manifest changed; died later on a loader refusal |
| 03:13:06 | fix live |
| 03:16:19 | `primeA` (`show targets --json`): capacity wait 66 s, walk, **new-format receipt published 03:18:01**, run ended 03:20:24 on a downstream loader refusal (`⚡️caching/🦀️cargo/📜️script.ts` import of routing, peer territory) |
| 03:21:13, 03:23:58 | harmless edits of `L/🏗️build/🚀️bootstrap/🟦️.ts` and `L/🗂️workspaces/🟦️bun/🟦️.ts` (one appended newline each); two concurrent pairs died at `Authenticated captured build command entry required` because loader-fix edited `L/🏗️build/📥️source/🔏️closure/🟦️.ts` at 03:22:20 and 03:24:11 (not a dependency fault; those pairs are inconclusive) |
| **03:25:30** | `pair.sh try1`: edit (newline appended to `L/🗂️workspaces/🟦️bun/🟦️.ts`), then two concurrent `bun repo show targets --json` (`try1-A`, `try1-B`). Both passed `build:dependencies` and ran `build:discover` (610 events), `build:cargo-owners`, `build:commands`, `build:command-source-imports`; wall 117 s and 155 s including 10 and 16 capacity waits; stderr/stdout contain **0** lines `Store dependency concurrency ... waiting resource build`, 0 stale lines. Both ended at 03:27:27 / 03:28:05 on the cargo script-import loader refusal (not dependency related) |
| 03:20:44-03:28+ | `leases status` showed `javascript-dependency-build` EXCLUSIVE + `javascript-dependency-bindings` EXCLUSIVE held by pid 54769 (old-code `--tool=bazel sh`, 453 s at 03:28:16) for the entire window: the old behaviour would have queued both runs behind it |
| 03:29 | all three edits restored byte-identical (bootstrap sha `09dbfe71c7fa1aa9` equals the pre-edit sha) |

Cost of the cheap path measured on the real receipt (`tier-cost.mjs`, 77,850 frontier rows, 3,002 provider rows, 326 manifests): frontier stat 2.3 s wall / 0.9 CPU-s, provider replay 0.15 s, manifests 0.11 s: about 2.6 s wall, 1 CPU-s per verification against 30-65 s / 20-60 CPU-s for a walk and 22 CPU-min for a sync. Receipt 1.15 MB, frontier evidence 21 MB.

The "change bun.lock -> stale with reason" half is proven by the fixture (section 3, TypeScript and Python agree), not on the live `bun.lock` (not edited: it would trigger a real sync for the fleet).

## 5. Open / hand-over

1. Old-code processes. Processes started before 03:13 still run the old verifier (it requires `implementation`, so it calls my receipts `receipt-invalid`) and, once they finish a sync, publish old-format receipts (the retire step keeps the newest per scope, so a late old writer can make the next new-code reader do one more walk + republish; self-healing but noisy). In the build queue now: pid 54769 (`--tool=bazel sh`, holds build+bindings EXCLUSIVE, running its 22 CPU-min sync), 54240/53780/71377/54173 (`bazel run` / `show`, 55-80 min old). They are peers' processes and I did not touch them. Recommendation for the coordinator: restart the long-queued old-code commands after 54769 finishes (new code coalesces on the receipt immediately instead of syncing again).
2. The first run after the schema change pays one complete walk to publish the new-format receipt (done at 03:18:01); later runs verify in about 3 s.
3. Loader false positive (loader-fix): `capturedRead` refuses every `.json` below a `🧬️schema/` directory, including ordinary workspace `package.json`. Other readers of such manifests that go through `fs.readFileSync` of an implementation module will hit the same refusal; narrowing it to non-`package.json` or to files the implementation actually imports is the clean fix.
4. Still not changed (graph-perf item 2b): a command that merely waits for the build lease keeps its machine-admission grant (3.5-5 GB). With coalescing working the wait is rare, but the grant should be released while waiting.
5. A genuine manifest/lock change still needs a real sync (exclusive build lease, 22 CPU-min, graph-perf fixed the walk portion). That is correct behaviour; the convoy of redundant syncs behind it is what is gone. Not exercised live (needs a manifest change).
6. Pre-existing failing laws listed in section 3 belong to the owners of `🗂️workspaces/🟦️bun/🟨️.cjs` and the incremental scan test.

## 6. Follow-up 05:10-05:40: "edits to the closure or its 🧪️tests dirs keep invalidating the receipt" (coordinator, from result-loader-own.md)

Verdict: PARTLY REAL, root-caused and fixed (live 05:13:55 CEST). Pid 29009 (cold graph generation) was not touched.

Evidence (the stale reporter from section 2.3 made this readable; `rg --no-ignore -a "receipt stale" C/🗑️generated`): since 03:13 the only stale classes seen in peers' logs are
- `membership-changed ... detail=listing` at `L/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests`, `.../🔏️authorizers/🧭️sites/🧪️tests` and `L/🏗️build/🕸️graph/📥️inference/🔖️stamps` (apps-a-x 05:10, bzl-snapshot 05:10, loader-own 05:10, bzl-tail 04:13, demo-libs 03:30, apps-b 03:25): a peer created a new SUBDIRECTORY (new test case dir, stamps dir) in a directory of the repo-tool tree. That is not the old closure hash and not a `package.json` inside the closure; it is the workspace-layout frontier: the receipt recorded, per directory, a digest of "package.json + every non-hidden subdirectory + symlinks", so ANY new or removed directory anywhere (78k directories walked) invalidated it, although a directory without `package.json` or symlink in its subtree cannot change ownership. Every such edit cost the next command one complete walk (30-65 s) plus a republish, and in the sync gate (`verifyJavascriptDependencySyncCurrentV1`) it turned "current" into "stale" which, when a build is requested, runs the full producer under the EXCLUSIVE build lease.
- `receipt-invalid` / `unreadable` only at 03:17-03:19 (old-format receipt, the loader schema-dir refusal of 2.5), none since. No old-code process is alive any more (oldest `bun --eval` started 04:54), so "old-code processes republishing old-format receipts" is not current. Editing files (content) inside the tool tree or its tests dirs does NOT invalidate (fixture cases of section 3, and `frontier-diff.mjs`: at 05:11 the receipt published 05:05:50 had 2 touched directories, both listing-equal).
- Separate cause in `slice-demo/d11.log`: `Store dependency sync stale {"reason":"absent"}`: the SYNC receipt (not the generation receipt) was absent at that moment, so that command's producer ran the full walk under the build lease (waits of 960-980 s visible there). That is a sync that really had to run (no sync receipt published yet after the manifest churn); not caused by closure edits.

Fix (schema-first, protocol stamp bumped to `semio.dependency-generation-walk/2` since the frontier row layout changed, so every older receipt answers `protocol-changed` with the exact stamps, EN/DE):
- Frontier directory rows now are `["d",path,dev,ino,mode,mtimeNs,ctimeNs,settled,projectionDigest,prunedDigest,relevant]`. `relevant` = the directory holds a `package.json` or a symlink, or any child directory that does (computed bottom-up at capture); `prunedDigest` = the projection digest without child directories that are not relevant. New schema constant `maximumScan` (65,536 directories) bounds the live scan.
- Verify: unchanged fast path (stamp equal and settled -> skip); on a changed directory the plain digest is compared first; if it differs, child directories that the receipt knows as irrelevant (flag 0) or that are NEW and whose live subtree (bounded scan with the walk's own projection, so hidden/opaque/`node_modules` are skipped exactly like the walk) holds no `package.json` and no symlink are dropped, and the pruned digest decides. A missing or file-replaced directory row with `relevant=0` no longer fails. Anything that could change ownership still falls back: new package manifest anywhere in a new subtree, new symlink, a changed listing of a relevant directory, scan budget exceeded, unreadable directory.
- Cost: unchanged (same O(directories stat) + bounded relist, relist budget `maximumRelist` still counts).

Tests (live, same files as section 3): fixture `🔣️.json` gains `empty directory added under a source directory matches`, `directory with a source file added matches`, `source directory removed and recreated empty matches`, `source directory without manifests removed matches` (was "falls back"), and the fall-back guards `directory with a nested package manifest added falls back` (membership-changed, path `pkgs/a/src`, detail `listing`) and `symlink added to a source directory falls back`; the independent Python walker in the main test now also computes `pruned` and `relevant` per directory and must equal the frontier rows 9 and 10. Results at 05:14: generation 11 pass / 1 pre-existing fail (the `projectListing(` count law), ingress 10, followup 7, incremental 14 pass / 1 pre-existing fail, concurrent 16.

Live check (05:24-05:40): created the empty directory `L/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/dr-probe-empty` (exactly the case that produced the 05:10 stale lines), then ran `bun repo show targets --json` (`m2`, log `C/🗑️generated/dep-receipt/m2.*`). It passed the dependency stage without any stale line and without holding or queueing for any `javascript-dependency-*` lease; at 05:40 it was queued only behind `repository-graph-facts:shard:7` (pid 43147, the cold generation; 3 more waiters) and I stopped my two waiting runs (`m1`, `m2`) to give back their 5 GB grants, then removed the probe directory (05:40:23). The receipt in the store is the new format (`walk/2`, 77,860 rows) published 05:18.

Open: while the cold generation (43147/29009) holds `repository-graph-facts` exclusive, every graph command waits there with its 5 GB capacity grant (graph-perf item 2b), independent of this fix. A genuinely new package (new `package.json`) in a tests dir still, correctly, invalidates the receipt.
