# Result E58: scope-keyed dependency receipts (LANDED) and dependency generation hygiene (NOT solved, root cause found)

## 1. Landed: scope-keyed receipts (E51 design, adapted to the live files)
All E51 anchors still matched at 15:00. Locks bootstrap, caching-bootstrap, workspaces taken (alphabetical), patch applied in order bun, gen, oracle, gentest, locks released. Landing recorded in `T/landings.md` ("Wave 1122 E58").
- `workspaces/bun`: `scopesPlan` keeps `scopeDirectories`; shared `deepestScopeDirectory` (also used by `closurePlan`); `observedOwnership` returns `textOwners` (manifest path -> owning scope).
- Receipt schema `walkStamp` /3 -> /4, `Text.owner` required. Capture keeps only texts and input groups of the `required` scopes and drops frontier rows of foreign manifests. Verify gives `receipt-invalid` for a text/input outside `required`. Glue `manifestTextOwnerV1`. Python oracle gets `owners`, `scoped`, and the same verify rule.
- New test `generation/tests/scoping/🟦️.ts` (9 tests): owners law against Python, scoped capture schema-valid and equal to the Python selection, five-edit matrix (TS equals Python oracle), foreign stat drift, receipt-invalid. Two test fixes needed vs the staged copy: the fixture must `publishBunWorkspaceOwnership` before the readonly walk (published patterns drift otherwise); the shared fixture json `walk/3` -> `walk/4` (2 strings).
- Behaviour (required closures .=[.], sa=[.,sa], sb=[.,sa,sb]): root/framework manifest edit stales all; sa edit stales sa+sb; sb edit stales only sb; inert edit (scripts) stales nothing.
- One re-key: every existing receipt is `protocol-changed` once (expected, no migration); each machine rebuilds its receipts once on its next adoption.

### Tests (repo test-file)
scoping 9/0; generation main 17/0; ingress 10/0; heap-environment 2/0; profile 3/0; storage 47/0; storage retirement 14/0; workspaces bun main 100/0, closure 5/0, packed 7/0, receipt 4/0, frontier 2/0, refresh 29/0, churn 7/0; workspaces main 20/0; dependencies main 15/0; walk 5/0; adopt 8/0.
Not mine / unchanged: generation `followup` 7/1 ("failed write ... byte-identically": the writer was not touched; not baselined); `incremental` 8/11 (all "Managed Python test interpreter binding required" or "hardlink control", environment).

### Measured (script `T/🧰️scripts/E58/measure.test.ts`, scratch world in tmpdir, 603 manifests: 300 lib, 150 sa, 150 sb; never the real manifests; run through `repo test-file`, load about 20)
walk 1454 ms, capture of the 3 receipts 173 ms.
| edit | `.` | sa | sb |
|---|---|---|---|
| none | match 28 ms | match 34 | match 41 |
| inert scripts edit in sa/q0 | match 21 | match 38 | match 50 |
| exports edit in sb/q0 | match 28 | match 31 | STALE 17 |
| exports edit in sa/q0 | match 31 | STALE 11 | STALE 11 |
| exports edit in lib/p0 | STALE | STALE | STALE |
Before this landing every row except "none" was STALE for all three scopes (full owner walk plus republish of all receipts). So an inert edit costs one 20-50 ms verify per scope instead of walk + install chain; an sb-only edit leaves `.` and sa matching.

## 2. Not solved: dependencies.unready-generation / dependencies.orphans never get retired
No code landed for this part (a first attempt was reverted, tree equals before, no dead module left).
Root cause (evidence):
1. Retirement exists and is correct (`🔒️enforcement/.../⚙️setup/♻️retirement`, `retireObsoleteRepositoryDependencyGenerationsV1`, handles unready, retained-unready, inactive), but runs only inside `repo tools setup all` -> `setupRepositoryDependencyPublicationV1`.
2. There `dependencyStoreVerdictV1` returns `occupied` whenever a live holder exists and all holders are shared ("nothing needs exclusive access and none is possible, so nobody is waited for"): maintenance, including retirement, is skipped. Live proof: my `repo tools setup all` (15:05, 1:44) printed "The dependency store is verified current without exclusive access; 1 other process(es) hold it shared (process 62299 ... show projects)" and `.cache/b/d/m/j` still has 30 generation dirs. On a coordinator that always runs some repo command, no setup ever sees an unheld store, so garbage from 11-16/11-21 accumulates (6 unready of 29 h, orphans 9.9 GB).
3. The retirement owners are gated to the original command `tools setup all` (`repositoryCacheImagePublicationOptionsV1`: "Explicit original setup-all asset purpose required"). `repo cache gc --area=dependencies` calling the driver fails with exactly that error (tried live, reverted). So the normal `cache gc` / idle path cannot retire dependency generations; the ledger treats the `dependencies` partition as image-owned (kind partition, never delegated).
Recommended fix (needs a decision, touches the authority gate): either (a) a retirement-only authority variant that also accepts the original command `cache gc` (+ `--idle`) with the same exclusive-parent proofs, called from `runCacheLedgerV1` before `reclaimImages("dependencies")`, throttled by the existing stamp mechanism, or (b) per-generation leases so unreferenced generations can be retired under shared holders. Either way the whole graph read (pid 62299, forbidden to touch) holds the store shared, so confirmation that doctor clears needs that read to end.
Also: the doctor text of `dependencies.orphans` says "The collector removes them after their retention time", which is false today (E60 owns doctor texts).

## NOT verified
Real `repo dependencies ensure` after the re-key (not run: would walk and rebuild receipts of the live checkout; load/pid rules); verify timings are on a 603-manifest synthetic world, not the 324-manifest/100k-frontier live receipts; frontier `d` rows stay global (structural change in any scope still stales all); doctor warnings unchanged.
## Others must know
Receipt format /4: the first `repo` run per machine after replication re-keys all six dependency receipts once (one owner walk, expect minutes at load 20+). Peers stamping `walkStamp` /3 anywhere will fail `protocol-changed`.

# Round 2: per-generation holds, event-driven collection (LANDED, proven live)
## Baseline of generation `followup` 7/1
Pre-existing, not caused by my landing: the writer constant `relativeWriteCeilingBytes` is 8 MiB (unchanged by me; my storage edit only touched the capture mapping), the test wrote 300000 bytes and expected three yields. Fixed the stale test (16777217 bytes = three chunks): followup 8/0.
## Design (no authority gate widened)
- New member `📦️dependencies/♻️collection`: schema-first (`Policy`, `Key`, `Hold`, `Plan`, `Retirement`), module, tests. Rule (pure `planDependencyGenerationRetirementV1`, equal to a Python oracle on the shared JSON cases): retire a generation that no pointer references (current or retained list) and no live process holds, once a finished one rested 1 h (superseded) or an unfinished one 6 h (dead build).
- Holds: when a process verifies a dependency receipt successfully (`verifyJavascriptDependencyGenerationReceiptV1`, the single adoption point of every build and of the whole graph read) it writes `<store>/hold/<generation20>.<pid>.json` (best effort, removed at process exit, ignored and deleted when the pid is dead; a reused pid only keeps a generation longer).
- Collector `collectDependencyGenerationsV1` runs from `repo cache gc` (whole gc, `--area=dependencies`, `--idle`, hence every worker's collection tick). It needs no exclusive access to the store: it removes the ready marker first, then the tree, so an interrupted removal leaves an unfinished generation the next run finishes. Fails closed: no readable pointer means nothing is retired. Dry run prints "Would retire N". Cancel is checked between generations.
- Doctor: `dependencies.orphans` and `dependencies.unready-generation` texts and fix command now name `repo cache gc` and the real grace times (en/de).
## Proof
Unit: collection 4/0 (oracle equality, schema, held kept, dead-pid hold ignored, no-pointer refusal, cancel), cache-gc 19/0 and idle 4/0, doctor 17/0, generation 17/0, scoping 9/0, storage 47/0, dependencies 15/0.
Live (15:24): dry run said "Would retire 12"; `repo cache gc --area=dependencies` printed "Retired 12 dependency generation(s)"; `.cache/b/d/m/j` 30 -> 18 directories; `repo doctor --json` afterwards lists no `dependencies.*` finding (6 other findings: leases, load, graph, caches). Whole read pid 62299 was still alive (elapsed 42 min). Workspace `node_modules` symlinks point to pointer-referenced generations, which are never candidates.
## NOT verified
The whole read (started before holds existed) has no hold file; it is protected only because it uses pointer-referenced generations. A process that adopted a generation and then lost its pointer reference while running, before it wrote a hold (older binary), is protected only by the 1 h / 6 h grace. Installs running longer than 6 h would look like dead builds (unfinished generation mtime). Disk space freed was not measured (du was still running); the hold directory sits inside the dependencies image and is not in the roster checks of the setup retirement (setup tests were not changed or run beyond storage retirement 14/0 in round 1). Other machines get the code through the replica.

# Round 3: liveness decides, not age
1. Installer hold: `javascriptDependencyBindingsV1` (the point where the stage directory `j/<generation20>` of an install is determined, before any byte is written) now calls `holdDependencyGenerationsV1`. The collector already skips held generations, so a live install of any age is never retired and the 6 h grace applies only to unheld unfinished generations. Test: an unfinished 99999 s old generation held through that call survives, an unheld one goes, and the held one goes once its holder is dead (collection 5/0; storage 47/0, dependencies 15/0, generation 17/0).
2. Idle collection confirmed, no wiring needed: `remote-execution/sidecar` starts `startSidecarIdleCompactionV1` for roles `agent` and `cache` (every worker runs the agent), interval 3600000 ms (CACHE_GC_LIMITS_V1), first delay as configured; the pass spawns `repo cache gc --idle`, whose command path now runs `collectDependencies(false)` before the image reclaim (round 2). Not observed live: an actual hourly tick on another worker.
Hygiene: my `du` was killed (no process of mine left). Not verified: a pid-reused hold keeps a generation longer only; an installer of an older binary writes no hold (6 h grace applies).
