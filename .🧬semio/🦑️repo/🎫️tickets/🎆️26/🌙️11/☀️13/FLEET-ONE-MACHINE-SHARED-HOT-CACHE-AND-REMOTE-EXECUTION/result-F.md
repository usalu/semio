# Package F: the build graph is computed once per fleet state

LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, new member M = `LIB/🏗️build/🪪️graph-state/`. Measured on the live tree `/Users/Shared/semio` (group server, load 15 to 26) on 2026-11-13.

## 1. What exists today and why other machines do not get the graph

Two stages cost minutes on every machine:

1. The portable build graph (`loadBuildGraphV1`): an incremental store of 5079 owners (`generations`, 742,000 observation rows, 36 MB of fragments, 55 MB fact index, measured on a real store). Delta refresh already exists (plan `cold|clean|delta`, only stale owners are recomputed).
2. The Nx project graph (3.5 to 6 min daemonless): `NxScript` pins the portable graph (`SEMIO_BUILD_GRAPH_PATH` / `SEMIO_BUILD_GRAPH_DIGEST`), Nx derives `project-graph.json`, `file-map.json`, `source-maps.json` in a namespace, and a local receipt (`semio-nx-generation.json`, `nxGenerationHoldsV1`) lets the next run reuse them with `NX_FORCE_REUSE_CACHED_GRAPH=true`. Nothing shared this stage.

`B/🕸️graph/🕰️generations/🌐️remote-exchange` shares only stage 1 and gives nothing to other machines, for these reasons (each read in code):

- HTTP 401 on every client until package A (known).
- The action key is `sha256([domain, identity, absolute workspace root])` (`generationExchangeKeyV1`): a machine at another root never finds it, and its documents would be wrong there anyway: 44,759 absolute root paths in 5088 fragments, and absolute paths in the `file` rows of the fact index.
- `restoreRemoteGenerationV1` runs only for a builder whose local store has **no pointer at all** (`loadGenerationGraphV1`): a machine with any old generation never takes a newer remote one.
- `publishGenerationV1` skips when the remote note has more owners or an equal or higher `generation` counter. That counter is a per-machine count of local builds: not comparable between machines, so one lineage blocks every other.
- One key per identity (catalog providers only): every state of the workspace shares one slot.
- It covers neither the pinned snapshot nor the Nx graph products, the 3.5 to 6 min stage.
- Restored rows carry the producer's `mtime/ctime/ino`; this is harmless (`fileFactHolds` falls back to comparing the sha256 on a stat mismatch), so a rebased restore would be accepted by content.
- The pinned portable graph also holds the root: 18,732 occurrences in the 365 MB / 686 records of a real snapshot (`nativeOwnerCommand.arguments`, `SEMIO_PLUGIN_CATALOG`, `cwd`), so `SEMIO_BUILD_GRAPH_DIGEST` differs per root and cannot be a fleet key.

## 2. What was built

All new files are in M; nothing imported M before the landing. Schema first: `M/🧬️schema/🔣️.json` (Policy, Frame, State, Note, Health, bilingual TextCatalogue).

| Part | File | Does |
|---|---|---|
| State identity | `M/🟦️.ts` | `classifyGraphInputV1` (ignored / name / content) derived from the roster rules the graph walk uses (`ROSTER_RULES_V1.descends`); `graphStateOfV1(frame, files)`; two order independent folds (sum of SHA-256 terms mod 2^256): `names` over every non-ignored shared path, `content` over path + sha256 of every file whose bytes an inference can read; `graphStateAfterV1` (O(changed) update from a path diff); `graphPlatformV1`. |
| Manifest source | `M/📥️replica/🟦️.ts` | `readReplicaFilesV1(worktree)` reads `.cache/f/state.ckp` + `state.wal` with the replica's own `ReplicaBookV1`/`CompactTableV1`/`readCheckpointV1` (checksummed); `replicaHealthyV1` gate on `.cache/f/health.json` (fresh within 30 s, store ok, no failure, pid alive). Retries when the daemon rotates the checkpoint between the two reads. |
| Fallback source | `M/🌳️tree/🟦️.ts` | git inventory (tracked + untracked not ignored, the shared universe) and sha256 of the content files with a stat memo. |
| Fleet exchange | `M/🛰️exchange/🟦️.ts` | `neutralizeRootV1`/`expandRootV1` (root to token in raw, slash and JSON-escaped spelling, only at whole path prefix); brotli blobs of at most 32 MiB; entry = Bazel action result (stdout = Note, output files = blobs) under `sha256([domain,"entry",kind,digest])`; `decideGraphEntryV1` (lookup, claim, long-poll wait, takeover on lapse, bounded wait, every fault = compute locally); `publishGraphEntryV1` (blobs first, then the result with the slot header, then release). Uses B's `createFleetClientV1` (timeouts, breaker, claims) and C's `slotIdV1`. |
| Nx graph step | `M/🧶️nx/🟦️.ts` | `graphFleetSettingsV1` (A's `resolveRemoteCacheV1`; `SEMIO_FLEET_GRAPH=off` disables), `rootlessGraphDigestV1` (digest of the pinned snapshot with the root neutralised, per-record memo), `nxGraphStateV1`, `beginNxFleetV1`/`finish` (install a hit + write the local receipt under `withNxGraphAuxiliaryCapacityV1`, or hold the claim and publish after a successful run), cost memory for the wait bound. |
| Generated inputs | `M/🧾️generated/🟦️.ts`, `M/📜️script.ts` | `generatedInputsDigestV1` and `repo M/📜️script.ts digest-generated <dir>...`; `... state` prints the identity of the worktree and how it was reached. |
| Wiring | `LIB/⚡️caching/🚀️bootstrap/📜️script.ts` | 6 anchored additions (script `🧰️scripts/F/patch-script.py`, sha256 `aa13e33bcaab` to `37ae79a5dbae` at landing): before Nx runs, when the local receipt does not hold, ask the fleet; on a hit set the reuse flags; after a run whose receipt was written publish; release in `finally`. |
| Declaration | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json` | runtime input `digest-generated` over the project's `🤖️generated` tree in `namedInputs.default`. |

Landing recorded in `landings.md` (Package F). Tests: M `🧪️tests/🟦️.ts` 13 pass, `🧪️tests/🧶️nx.ts` 5 pass, Python oracle `🧪️tests/🐍️oracle/🐍️.py`.

## 3. The input set (item 1) and why it is exact enough

`graphMembers` in `🏗️build/🔣️policy.json` says presence of only four basenames, lock files and 14 source suffixes matters; "every other file reaches inference only through observed facts". The observed facts are the rows of the generation. I decoded the 742k rows of a real fact index (5079 owners) and listed which paths the content-bearing kinds (`file digest analysis field typescriptText`) read: extensions `.rs .ts .tsx .mts .cts .js .mjs .cjs .json .toml .mod .patch .py .lock .feature .css .graphql .proto .wit .sty .tex .sh .ttf .otf`, no other. `Policy.contentExtensions` is that list plus a few safe additions; any file outside it can only change the graph by appearing or disappearing, which the `names` fold covers (directory listings). Over-inclusion only costs hits, under-inclusion would be unsafe; the list is derived from the data, not guessed.

Inputs outside the manifest (9,900 rows below `node_modules`, `dist`, `.cache`, `.bun`: dependency installs and generated files) are not in the shared state. They enter the key through `Frame.extra` (hex digests the caller supplies: the dependency generation receipt digest for `node_modules`, `generatedInputsDigestV1` for generated trees) and, for the Nx stage, through `identity` (digest of the Nx and plugin manifests) and `implementation` (plugin sources) of the existing generation claim.

Exactness limits, stated honestly: the key is the identity of the exact bytes of every file an inference may read, so any `.rs/.ts` edit changes the **portable-graph** state key although most edits leave the graph unchanged (cutoff keeps the nodes). That is why the Nx stage is keyed by the root-neutral digest of the **result** (`rootlessGraphDigestV1`), which is stable across such edits, and why the file-level state key is meant for stage 1 (restore the newest compatible generation, then let the existing delta machinery recompute only what changed).

## 4. Platform and path independence (proved which)

- Path: neutral documents are byte-identical at any root. Proof with the real Nx 21.6.11 (`/Users/ueli/Documents/semio-kinan-wip/node_modules/nx`, daemonless): the same three-project workspace computed natively at two different roots gives byte-identical `project-graph.json`, `file-map.json`, `source-maps.json` after `neutralizeRootV1`, apart from the `computedAt` field (script not kept in the worktree; it ran from the scratchpad). Installing graph files computed at root A into the data directory of root B and running `NX_FORCE_REUSE_CACHED_GRAPH=true nx show projects` printed the installed graph (253 ms), while the same directory without the flag recomputed from the changed sources (469 ms): Nx accepts exactly these three files, without `nx_files.nxt` or the database. Unit test `the portable graph digest equals at every root...` and `collect and install round trip across roots`.
- Platform: **not neutral.** `📥️inference/🧱️project-assembly/🟨️.mjs:383` calls `withHostPlatformOutputsV1(base, process.platform)` for projects that declare `hostPlatformOutputsV1`, so the graph content depends on the OS; `file-map.json` also holds Nx content hashes, which differ across line-ending conversions. Therefore `Frame.platform` (`<os>-<arch>`) is part of every key. The seven macOS machines share; Linux and Windows share with nobody until a second machine of their platform joins. Proving two platforms equal needs a second OS; not possible here.

## 5. Numbers (this machine, load 15 to 26)

State identity of the live tree (272,989 shared files, 118,745 visible to the graph, 106,362 content files):

| Source | Time |
|---|---|
| replica checkpoint + log read, fold included | 0.63 s (read 0.11 s, fold 0.49 s) |
| tree fallback (git inventory 16 s + sha256 of 106k files 5.7 s; second run with stat memo 22.8 s: the stats cost what the reads did) | about 22 s |

Both sources agree: replica digests vs tree digests for every content file (about 106,360), 0 differences, 0 missing (script `🧰️scripts/F/probe4.ts`; the fold of my reader equals the daemon's attested `manifest` digest whenever sampled at the same instant: 2 of 4 samples equal, the others differed only by concurrent edits).

Nx stage, real pinned graph (365 MB / 686 records): root-neutral digest 1.67 s cold, 1 ms with the record memo. A synthetic 158 MB `project-graph.json` (built from real project records, roots inside) plus the real 20 MB `file-map.json`: collect + neutralise 0.51 s, claim 4 ms, compress + upload 0.36 s, lookup + download + decompress 0.10 s, expand + install 0.60 s (script `🧰️scripts/F/measure-exchange-158mb.ts`; the synthetic graph compressed to 2.1 MB, a real graph will compress less). A fleet hit therefore costs about 2.5 s (digest 1.7 s without memo) against 3.5 to 6 min computed.

Single flight: 8 separate processes started at once against one cache server: 1 computed (3 s simulated), 7 waited and received the upload after 3.02 to 3.03 s (script `🧰️scripts/F/fan.ts`, `child.ts`). Test `two concurrent cold starts compute once` and `the first machine computes and publishes, the second installs...`.

Property test of the key (`the key changes exactly when a graph-relevant file changes`): edit of each of 40 content files changes the digest; bytes of 40 name-only files, of the 4 ignored files, an added `node_modules`/`.git` path, and reordering do not; a new name-only file, a new content file, a removal, a rename each change it; platform, tooling, implementation, kind and `extra` each change it. Independent Python model of both folds and the digest agrees on 5 random states of up to 321 files; `graphStateAfterV1` equals the full recompute after 30 random edits/removals/additions.

## 6. Incremental (item 2), cost stated

- Stage 1: the existing machinery is incremental (membership frontier + journal hints, per-owner fact verification, `delta` plan). It recomputes only stale owners. No change made; the file-level state key and `graphStateAfterV1` give the O(changed) identity but are not yet wired into stage 1 (see 8).
- Stage 2: Nx has no incremental graph. A new portable-graph digest costs one full Nx recompute (3.5 to 6 min), now once per fleet behind the claim instead of once per machine; all other machines wait (bounded by twice the last local cost, 60 s to 10 min) or compute locally.

## 7. Gitignored generated inputs (item 3)

`generatedInputsDigestV1` digests the bytes of every file below the given directories, ignored or not (path NUL sha256, byte-sorted, root independent, stat memo, symlink target). `repo M/📜️script.ts digest-generated <dir>` prints it (0.16 s for the registry's `🤖️generated`, `f4b16026...`). The plugin registry project now lists it as a runtime input of `namedInputs.default`, so its tasks hash the generated catalogues (`🔌️plugins.json`, `🎠️playgrounds.json`, `🗂️sources.json`) and neither falsely hit nor stay unshareable. Test: equals a Python model, equal at two roots, changes on edit, add and missing directory. Only this project is declared; other projects that read their `🤖️generated` trees need the same one-line input (list them by `grep` for `🤖️generated` in their scripts; I did not audit all 1271).

## 8. NOT verified

- No real `repo` command through the changed path, and no cold-vs-hit time to first task in two scratch checkouts. A cold `repo show projects` on the live tree waited 30 minutes with 0.7 % CPU in the shared-lease queue of `native-fixed-image` (`.cache/b/l/m/data/5faafe25...queue`, entries from pid 1062 of 17:47 and my pid), the graph partition `.cache/b/g` was never created, and I stopped it. The patched `NxScript` block is verified by transpile, review and the unit tests of what it calls, not by running it. A scratch checkout of 273k files at another path cannot boot `repo` here either (no installed dependencies in other checkouts).
- Nx 23.2.0 with the semio patch accepting externally installed graph files (only stock Nx 21.6.11 proven). The publication journal (`publishGraphFilesV1`) is bypassed by a plain rename into a namespace the process already holds; two concurrent runs of the same namespace on one machine could interleave, but they would write equivalent files.
- Stage 1 is not shared. What it needs (all inside `🕰️generations`/`remote-exchange`, which I did not touch): key by identity + platform instead of the root; decode every document and replace the root in fragments, frontier, fact index (codec `encodeFactIndexV1`) and manifest names; restore also when the local store is behind (compare a state-derived note, not the per-machine counter); then the existing sha fallback of `fileFactHolds` validates the rows. I did not write it: it cannot be validated without a running stage 1 on this machine.
- Windows, Linux, the cross-OS question, a real compression ratio, behaviour with the real fleet credential (package A), taxonomy/name gates for the new directories, `tsc` over the edited files (no `typescript` package here).

## 9. What the other packages must know

- A: I use `resolveRemoteCacheV1(environment,{root})` only. While no credential works the step prints `[fleet] The fleet cache is not used for nx-graph: fault.` and computes as before. Live check once A works: on machine 1 `repo run <project>:<target> --dry-run` prints `[fleet] Published the nx-graph of state <12 hex> ...`; on machine 2 the same command prints `[fleet] Installed the nx-graph of state <12 hex> ...` and reaches Nx in seconds; `repo M/📜️script.ts state` prints the identity and the source (`replica` or `tree`).
- B: I reuse `createFleetClientV1` (`getResult/getBlob/putBlob/putResult/claim/renew/release/await`) and claim kind `ac` with the 64-hex entry key. If its signatures change, `M/🧪️tests/🟦️.ts` fails first.
- C: slot domain `graph-state` is passed to `slotIdV1` (works at runtime; the schema enum `Domain` lists only `rust-unit`, `nx-task`; please add it). Parts: kind, platform, tooling.
- G: my `script.ts` edit and yours touch different lines; the Nx front is independent of the graph step.
- Everyone: `SEMIO_FLEET_GRAPH=off` disables the graph step; the memo files live in `.cache/f/` (ignored).

---

# Addendum 2026-11-14: coordinator follow-up (items 1 to 5)

## Item 1: the real run on the live tree

- `repo remote-cache` authorisation: `resolveAuthorisedRemoteCacheV1` against `http://192.168.178.70:6090` answers `authorised: true` (token from the fleet facts, status 404 on the probe route). So the real cache can be used; I did not publish to it because no graph run completed.
- Cold `repo show projects` on `/Users/Shared/semio`, three attempts, none reached a graph:
  1. 22:00: 30 min in the lease queue, see section 8 above (the coordinator's explanation: the first dependency install).
  2. After that install (36 min, `bun install --frozen-lockfile` in `.cache/b/d/m`): `repo show` stopped at once with `Authorizer source closure capacity exceeded: files=2049/2048`. Cause: the closure of the build runtime had grown to the limit; my files were part of it (9 of them, 7 new: `remote-exchange/🧳️portable`, the graph-state core, schema, exchange, client, nx step, root). I reduced my share (see below); a run with my Nx step stubbed out passed this check, the committed state with my step passes it as well after the reduction (the check ran inside the third attempt).
  3. 34 min later the same command failed with `Fixed Store dependency installer failed` for scope `♻️mit-bestand` (`bun install` status 1, stderr only as a hash in the log; 1,516 s system CPU). Fourth attempt: waited 11 min for memory capacity behind another agent's `ensure` job (6.3 GB), then ran the graph build: **296 of 4775 projects inferred after about 25 minutes, 23 projects in the last 120 s**. I cancelled it with SIGINT (leases released) because it would have taken hours and starved other agents.
- Therefore there is **no measured cold-vs-hit time through `repo`**. What exists: stage costs (section 5 above), the cold dependency cost of a fresh machine (36 + 34 min installs on this machine), and the inference rate of a cold stage 1 (about 0.2 projects/s at the start, 4775 projects). A hit costs about 2.5 s for the Nx graph (measured with a 158 MB graph) and, for stage 1, the restore measured below.
- The patched `script.ts` block still ran only in my tests; the real command never reached the Nx launch. It did not break `repo show` before that point.
- Incident on my side: a background probe command restored `script.ts` from a copy taken 40 minutes earlier. I compared afterwards (G's and my markers present, no landing record mentions another `script.ts` edit in that window) but cannot exclude that a concurrent edit was overwritten; whoever edited `📜️script.ts` of the bootstrap between 23:16 and 23:52 should check. It also removed `landing.lock` at that moment (it was mine).

## Item 2: graph read behind the dependency lease

Findings (read in code, one live observation):
- The wait is not an ordering accident of the graph step. `⌨️command/🏃️task/🤝️shared/🟦️.ts:33`: `buildOperationRequiresV1(command)==="dependencies"` runs `ensureBuildEnvironmentV1` (`🏗️build/🚀️bootstrap/🟦️.ts`), which inspects the installed parser material (`InstalledParserEntries`: the TypeScript parser packages, exact locked versions) and runs the install when it is `material-required`. `show` is classified as needing it.
- The reason is real: verifying or deriving any generation row calls `analysesPrepared()` in `🕰️host/🟦️.ts` (`holdsByStat`, `deriveAgain`), which imports the inference producer and the installed parsers. A fleet-restored generation is verified through the same `holds`. So reading even a cached graph needs the parser material, and the launcher boot alone (no `node_modules`) is not enough.
- What would let a machine that takes the graph from the fleet skip the install: (a) split `holdsByStat` so that rows of kind `file digest field exists readdir lstat` (sha or stat comparisons, no parser) are checked before `analysesPrepared()` and only `analysis` and `derived` rows wait for the parser; (b) classify `show`/`run --dry-run` as `dependencies-lazy`: start the load, call `ensureBuildEnvironmentV1` on the first `analysesPrepared()`. I did not make this change: it moves the install out of the invocation order of every command and has no test world for the lazy case; the host test world (`createMockHost`) has no parser. Hand-over, not landed.
- Package H restores installs from the fleet cache; until it does, a cold machine pays the install before any graph read. Measured here: 36 min and 34 min.

## Item 3: portable-graph stage shareable (landed, `landings.md` Package F landing 2 and 3)

`remote-exchange` rewritten around a root-neutral generation:
- Key: `sha256([domain, registry identity, platform])`, no path.
- Documents are moved to the root token and back by `🧳️portable` (`transformGenerationV1`): JSON fragments and frontier as text, the fact index through `decodeFactIndexV1`/`encodeFactIndexV1` (its rows hold the absolute paths), every document renamed after its new bytes, manifest rewritten and renumbered (`generation = max(local, remote) + 1` on restore).
- Order: no per-machine counter. The note carries a root-independent `signature` (identity, implementation, environment, membership, edge environment of every owner) and `publishedMs`. Publish only when the signature differs and the cache holds no newer upload than the local pointer; restore into an empty store, or when the store has another signature and is older than the upload by more than 60 s. Wall clocks only order uploads; a wrong order costs work, not correctness, because the load verifies the restored rows by content (`fileFactHolds` compares the sha256 when a stamp differs).
- Proof through the real load of the mock world (`remote-exchange/🧪️tests`, 6 tests): a generation built at one root is restored at another root and the load re-infers 0 owners; a store that is behind (22 projects vs 20) takes the newer state and re-infers only the edited owner (1); an equal state or an older upload is left alone; foreign registry or platform is refused; read-only and dead-server behaviour unchanged. `🕰️generations/🧪️tests` 30 pass.
- Real data: a real generation of this repository (5083 documents, 378 MB expanded, 5079 owners, taken from the old graph image): neutralize 10.0 s, no root string left in any document; expand to another root 11.2 s, every document carries the new root, signature unchanged (`🧰️scripts/F/real-generation-transform.ts`). Costs a publisher about 10 s once per new state, a restorer about 11 s plus the content verification of the rows.
- Not done: the restore has not run against a real stage 1 (no graph run completes on this machine); the content verification time of 472k observed paths after a restore is not measured.

## Item 4: other projects that read `🤖️generated`

Audit (`find` below `🧰️framework` and `✏️s`, pruning `node_modules target dist .cache .git .nx`): 35 `🤖️generated` directories. One is the framework-wide `🧰️framework/🤖️generated` (193 MB, `process/` run output, below 1,107 projects): treated as machine-local output, not an input, listed in `Policy.generatedSkipped`. The other 34 lie in modules; 107 projects sit at or below their parents. `🧰️scripts/F/declare-generated.py` added the runtime digest input (`digest-generated`) to `namedInputs.default` of 106 projects (landing 3; text insertion, JSON re-validated, idempotent; the registry project had it from landing 1). Not verified through a graph run; the declared form is the one 25 of these files already use for `default` and the one `sharedGlobals` uses for runtime inputs. Extended property test `M/🧪️tests/🧾️generated.ts`: every project beside a generated directory declares that directory's digest (a missing one fails with the pair), and the digest of a real generated directory is stable, equals after a copy, and changes with a byte, a new file and a removal.

## Item 5: Nx 23.2.0

Found the way A did: the plain `nx@23.2.0` package from the bun cache; I installed it with its dependencies in a scratch directory (`bun install nx@23.2.0`, outside the worktree), once stock and once with the repo patch through `patchedDependencies` (`⚡️caching/🩹️patches/nx@23.2.0.patch`).
- Stock Nx 23.2.0 (`SEMIO_TEST_NX_NODE_MODULES=<scratch>/node_modules`, test `real Nx` in `M/🧪️tests/🧶️nx.ts`, skipped without the variable): the three graph files computed natively at two roots are equal after neutralizing (`project-graph.json`, `source-maps.json` byte for byte apart from `computedAt`; `file-map.json` equal as JSON, Nx 23 emits its keys in varying order). Graph files installed from the other root are reused with `NX_FORCE_REUSE_CACHED_GRAPH=true` (249 ms; the project edited afterwards is not seen) and recomputed without it (480 ms; the edit is seen). Passes (6 pass with the other tests; 7 expectations).
- Patched Nx 23.2.0: cannot run outside the repo launcher (`Original immutable Native snapshot authority required`), so the acceptance of installed graph files by the patched client is **not verified**; it needs the real `repo` run that did not complete.
- Earlier proof with Nx 21.6.11 (section 4) agrees.

## Measured seconds and what stays unverified (for the coordinator)

Cold vs hit through `repo`: not measured (no run reached a graph; section Item 1). Measured parts: replica state key 0.63 s vs tree walk about 22 s; Nx graph fleet hit about 2.5 s for 158 MB (digest 1.7 s without memo, download 0.1 s, install 0.6 s) vs 3.5 to 6 min computed (audit figure, not re-measured); portable-graph restore: root rewrite 11 s for a real 378 MB generation, content verification of the rows not measured; cold machine: dependency install 36 min, graph inference started at about 0.2 projects/s of 4775. Unverified: any real `repo` graph or Nx run through the new code, patched Nx 23.2.0, the lazy-parser change (hand-over only), 106 project files through a real graph, Windows and Linux, real compression of a real graph, the time of the wall-clock ordering under skew, and the possibility that my `script.ts` write-back overwrote a concurrent edit.
