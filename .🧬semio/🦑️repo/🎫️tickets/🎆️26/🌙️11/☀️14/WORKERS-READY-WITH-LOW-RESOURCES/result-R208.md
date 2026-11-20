# R208: why fleet owner records are stale on g1-server (living report)

Written from 2026-11-18 13:27 UTC. Scripts: `FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R208/` (`land.sh`, `*.base.*`, `*.new.*`, `law/`, read-only scanners `diff2.ts`, `cmp.ts`, `an3.ts`). Scratch: `scratchpad/R208/`. No download, no slice-lab run, no kill, no sub-agent; everything under `nice -n 10`.

## 1. The fresh read (started 13:37:23 UTC) and what it printed

- The read sat in `source.served` ("a peer builds the graph; this run waits as its front door alone") from 13:37 to 14:14 UTC, 37 minutes, the same defect R204 saw at 11:47 (yield wait: 3 yields x `wakeMilliseconds` 600 s, generation -1 = the pointer was unreadable when it queued). It reached the owners at 14:23. Nobody has found who the "peer" is; nothing was interrupted by me.
- From 14:24 the read prints the first failing row of each stale fleet record (R204 landing 2). At 14:42 UTC: 71 of 4785 owners done, 43 present, 0 fetched, 28 inferred here, 35 stale in the fleet (the counter), 48 stale-row lines for 28 distinct owners.

| first failing row | lines | distinct owners | what it is |
|---|---|---|---|
| `analysis` | 31 | 15 | provider `script-requirements` 23, `rust-references` 7, `module-imports` 1 |
| `file` | 17 | 13 | two files only, see below |
| `closure`, `readdir*`, `exists`, `field`, `resolve` | 0 | 0 | none |

R202's open question (closure rows) is answered for this read: no closure row is the first failing row of any stale record.

## 2. The `file` rows: real tree differences (case c), the judge is already per input

All 17 name one of two `Cargo.toml` files: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📦️packages/🦀️rust/Cargo.toml` (15 lines; edited on this tree 14:15:48 UTC, 4157 -> 4186 bytes, sha `d5136f0f34` -> `975970e7d1`) and `🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📦️packages/🦀️rust/Cargo.toml` (2 lines; edited 14:01:50). The records (producer `…fritz.box`, published 14:26 to 14:30, 11 to 15 minutes after the edit) carry the old sha: the producer inferred them from the tree before the edit reached it or before it started the owner. Those records are valid for THEIR inputs and correctly refused here; other agents edit the tree continuously, so a record is only as young as the inference that made it. Check made: `verifyStoredProjectRecordV1` judges row by row against the owner's own rows; the header digest covers `[recordStamp, null, key]` (key = project and selected inputs) and nothing wider; the authority digest only names the pool's published records (a different identity is "missing", never "stale"). The `analysis` rows are judged per file as well (stamp equal, else sha equal, else replay). So per-owner acceptance by input digest already holds; there is no workspace-wide or tooling-wide digest in the verdict. Nothing to change for case (c).

## 3. The `analysis` rows: not a tree difference, not reproducible outside the read

For every analysis stale line I compared the record against this tree (`cmp.ts`): the file the row names has the SAME size and sha256 as the record's source (`📜️script.ts` of the fixture generators `bmp-v3-any`, `semio-v1-cad|document|drawing|mesh`, `probe-stdio-semio-v1-base`, `sourcing-*`, `writer-plugin`, `hub-inference-binding|catalog-rs` `.rs`, mtime hours old). The provider revision is the one this tree registers (`script-requirements` revision `d21949bed41a` = sha256 of the declared stamp `semio.graph-inferrer.source-analyses/3`, computed by hand). In a fresh process that registers the same providers and localises the record exactly like the taker (`an3.ts`: `registeredProvidersV1` ids, `localRecordV1`, `observeInferenceSnapshotV1(() => observationRowFreshV1(row))`) all 62 analysis rows of all stale owners' records of the last 100 minutes HOLD. So the verdict "stale" of the farm child is a property of the child's context, not of the record or the tree. By elimination I could not decide between: the provider ids in the record not equal to the ids registered in the child (ids hash the module location), a provider not registered when the row is judged, a stat/read error of the observed world in the child, or a nested observation. Static reading of `sourceAnalysisFactHoldsV1` shows it returns a bare `false` for each of ten different reasons, which is why the printed row cannot tell them apart.

## 4. Landing (window 15:00 UTC): the verdict names its reason

Landed 15:00:00 to 15:00:22 UTC under `landing.lock` (owner claude-R208, hold 22 s), entry in `landings.md`. Under the lock: reason 3, stale-why 1, stale-row 2+2, adopted-verdict 4, analysis-stamp 3, final-verdict 3, take tests 7, stale-line 2, decided 4, import smoke ok. Semantics revision NOT bumped (no record shape or verdict changed). Services and reads run the old code until they restart; a read started after 15:00 prints the reason.

Change: `sourceAnalysisFailureV1(path, fact)` in `👁️observation/🟨️.mjs` returns undefined while the fact holds, else one token with data: `contract`, `provider-unregistered <recorded id>; registered <n>, same role <registered id or none>`, `revision <recorded12> <registered12>`, `shape`, `nested-rows`, `unreadable <code>`, `replay-result|replay-nested|replay-set <recorded sha10>><now sha10>`. `sourceAnalysisFactHoldsV1` is now exactly "no failure" (one function, so verdict and reason cannot differ). `firstFailingRowV1` adds `why` to an analysis row; the read line appends `; reason: <why>` / `; Grund: <why>` (schema text `{why}`; no change for other rows). Laws: `👁️observation/🧪️tests/🧐️analysis-reason` (3; red on the base: all 3) and `📥️take/🧪️tests/🧐️stale-why` (1; red on the base).

## 5. Answers to the three causes (a, b, c)

- (a) machine-local state in a row: none found as the first failing row in this read. The earlier causes (R184, R204) hold; `diff2.ts` over 1313 pairs of same-owner records from different producers (20 minute window) shows value differences only in `field`(21: a `wasm-producer-declaration` format vs `json` of `🏃️process/🔌️providers/📋️project.json`, a real producer-code difference), `directoryExists` (1: `🧰️framework/node_modules` present on the Linux producer, absent on another), `exists` (2: gitignored `🤖️generated/` paths again: `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`, `…/🖐️5d/🤖️generated/📇️registry/🦀️.rs`), `analysis` (2: real source edits). Row SETS differ much more (the producers record different rows for the same owner, e.g. one has the closure rows, the other the wasm-producer field) but a taker judges only the rows of the record it takes, so that is not a staleness cause. NOT fixed here, found: the two `exists` rows over `🤖️generated/` are the R184 cause in the `exists` kind (listing filter exists, `exists` probe does not apply it); no stale line of this read names them.
- (b) closure/analysis rows over a shared closure: no closure row failed first in 48 lines; closure identity unchanged.
- (c) real tree difference: the 13 owners with a `file` row (two edited `Cargo.toml`), correct rejection, per-input acceptance already holds.
- The dominant cause, 15 of 28 owners and 31 of 48 lines (analysis rows that hold when judged by a fresh process on the same record and tree), is NOT determined. The landed reason in the read line is the instrument that decides it at the next read.

## 6. Not verified, what the coordinator must do

- The effect on the stale count: not measured; the running read has the old code and is at 77 of 4785 owners (14:44 UTC, 35 stale, 0 fetched). Restart the read after the pool roles restarted (`kill -INT <pid in graph-server/read-record.json>`, a new read starts about 10 minutes later) and look at `; reason:` in `read.log`: `provider-unregistered … same role <id>` means the ids differ between the taker that localised the record and the child that judged it (fix: judge with the ids of the process that localised, or localise in the judging process); `unreadable <code>` means the observed world of the child cannot stat or read the file (fix in the child's world); `replay-result a>b` with a == the record's sha means the child saw other bytes than this process; `revision` means the parser revision differs per process.
- The 37 minute `source.served` wait ("a peer builds the graph", generation -1) of this read is the same defect as R204 section 1: nobody found the peer; it delays every read by 30 minutes or more and was not touched.
- Stale counts per 100 taken, `fetched`, rate with the new code: not measured (task 4 not done).
