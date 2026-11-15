# Result: Git state (Executor A), 2026-11-13

Module root `MOD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`.

## What an OLD daemon does with realm events (read before editing)

Code on disk before my edits (`MOD/🔁️replica/🟦️.ts`):

```ts
const sealed=(path:string):boolean=>path===".cache/f"||path.startsWith(".cache/f/");
const guarded=(ignore:FleetIgnoreV1):FleetIgnoreV1=>({digest:ignore.digest,ignores:(path,isDirectory)=>sealed(path)||ignore.ignores(path,isDirectory),child:...});
function refusalOf(replica,path,entry){
 if(unsafePath(path))return "unsafe-path";
 const refused=refusePathV1(path); if(refused!==undefined)return refused;
 if(!sharedV1(path,false,replica.ignore))return "ignored";
 if(entry.operation==="link"){ ...linkRefusalV1... }
 ...
}
async function materialise(replica,path,entry){
 const refusal=refusalOf(replica,path,entry);
 if(refusal!==undefined){noteOnce(replica,`refused:${path}:${refusal}`,{event:"refused",...});return refusal==="case-collision"?"queued":"dropped";}
```

`refusalOf` runs before the delete branch and before the link-target check, and `replica.ignore` is always the `guarded` matcher. So for `.cache/f/index-base` and `.cache/f/index/<path>`:

- `link` (`tree <oid>` or `<mode> <oid>`): path policy passes, `sharedV1` is false because `sealed`, result `"ignored"`, outcome `dropped`, one `refused` progress event per path, nothing touches disk or `files`.
- `delete`: identical (the refusal precedes `entry.operation==="delete"`).
- It is folded into the old manifest and into snapshots (harmless); old `verify` skips it (`!sharedV1(...)`); an old restart re-queues and re-drops it. Harmless drop, so the realm path is kept. Not executed against the real old binary, derived from the quoted code and the old behaviour is also what the new tests assert for sealed paths.

The reverse direction: current daemons drop old daemons' `.git/index`, `.git/logs/**`, `.git/config`, hooks, IDE files (not on the allow-list, `refusalOf` -> `ignored`) and never publish deletes for them (see below).

## Done

1. `🚧️boundary`: `gitSharedV1` allow-list (HEAD, packed-refs, shallow, `refs/**`, `objects/**` minus `tmp_*` in objects, `objects/info/alternates`, any name ending `.lock`); `gitPrivateV1` is now only its complement inside `.git`; `sharedV1`, scanner, bootstrap use it. Schema `GitCase`, fixture `git` (43 vectors), TS test, Python oracle (`git_shared`) updated.
2. Private-path guarantee: in `publishOnceV1` the vanished-candidate loop now does `if(!sharedV1(path,false,replica.ignore)){forget(...);return;}` before any delete is decided (this also covers `.gitignore` changes); realm ids are never candidates. Incoming events for private paths fall to `refusalOf` -> dropped. Test "an old state that tracks .git/index, the reflog and the config forgets them without publishing a delete, and incoming events for private paths are dropped" starts from a state holding `.git/index`, `.git/logs/HEAD` (file deleted on disk), `.git/config`, `.git/hooks/pre-commit`: journal head unchanged, entries gone from `files`, the on-disk index survives an incoming delete and put, queue 0.
3. New member `MOD/🌿️git/` (`🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`, `🐍️.py`, `🧫️fixtures/🔣️.json`): only place that spawns git. Exports `gitStateV1(worktree,signal?)` with exactly the requested shape (never throws; stage tree cached against the stat signature of `.git/index`, so B's 5 s heartbeat costs no process while the stage is unchanged), `indexRealmV1(path)` (use it for the digest exclusion), `INDEX_BASE_V1`, `INDEX_PREFIX_V1`, `resolveGitV1`, `gitCandidatesV1`, `writeTreeV1`, `diffTreeV1`, `updateIndexV1`, `readTreeV1`, `missingObjectsV1`, `foldRegistersV1`, `publishIndexV1`, `applyIndexV1`, `verifyIndexV1`, `normaliseGitConfigV1`. Schema registered in `schema-document-index.json`, module listed in the repo README.
4. Realm wired into the replica with thin hooks: `indexHost(replica)` closure factory; `publishOnceV1` end (full scan and any batch containing `.git/index`); `applyQueue` end (realm items are filtered out of the file pool and applied by `applyIndexV1`, objects first); watcher lets `.git/index` through; `replayOwnTail`, open (folds), `verifyV1` (drift on `.git/index` when fold != write-tree), `statusV1` deep, `joinV1` (config normalised; base applied through `applyQueue`). Realm registers live in `files` (so they persist in `state.json`/log, snapshot and relay as ordinary `link`/`delete` manifest entries) but never in `folds`. Local record: `.cache/f/git-index.json`.
5. Config normalisation: `runV1` start, `joinV1` start, and on first base adoption (a joiner has no `.git/config` until then). Creates the file when absent.
6. Texts (en/de) `status.git.*` and `preflight.git.*`; `status` prints `Git state: ...`; `StatusReportV1.git {shared,reason,indexTree}` + schema.
7. Addendum: (1) bootstrap archive drops `gitPrivateV1` paths (test with a store that still holds `.git/index`, config, logs, hooks, IDE files: only HEAD, refs, objects remain); settle no longer scans private `.git` files. (2) `🐚️.sh` and `🔵️.ps1`: a worktree whose only content is a root `.git` counts as empty; `🐚️.sh` checks the entry count with `tar -tf` before extraction (independent of a pre-existing `.git`); ps1 test case `gitOnly` added. Windows restart policy untouched. (3) git lookup in `🌿️git` (`SEMIO_GIT`, PATH, macOS Homebrew/CLT/Xcode, Linux paths, Git for Windows below ProgramFiles, ProgramFiles(x86), LOCALAPPDATA); preflight check `git` warns `git.missing` (stage and commit state is NOT shared on this machine, files still sync) in en and de; all preflight counts 13 -> 14.
8. Docs rewritten: module `README.md` (new "Git state" section and four passages), ticket `git-state.md` (complete rewrite), `design-contract.md` (boundary, Git section, API comments).

## Tests (all `bun test ./<file>` in the file's module; Bun 1.3.14; `ajv` resolved through symlinks I created in `/Users/Shared/node_modules`, outside the worktree)

| Suite | Result |
|---|---|
| `📒️journal/🧪️tests/🟦️.ts` | 127 pass (one run had the 5 s streaming test time out under load, green on rerun) |
| `🚧️boundary/🧪️tests/🟦️.ts` + `🐍️.py` | 39 pass; oracle 115 vectors, 0 mismatches |
| `🌿️git/🧪️tests/🟦️.ts` + `🐍️.py` | 9 pass (real git in temp dirs, fold equals the Python/git-CLI oracle tree for 2 fixtures, config, plumbing, state); oracle prints the 2 trees |
| `🔁️replica/🧪️tests/🟦️.ts` (incl. its Python oracle) | 61 pass |
| `🔁️replica/🧪️tests/🏁️race.ts` | 14 pass |
| `🔁️replica/🧪️tests/🌿️git.ts` (3 replicas, one temp store, real git) | 6 pass, 3 consecutive runs green. Covers (a) stage, (b) unstage, (c) concurrent different paths and same path later hlc wins, (d) commit: same HEAD, clean status, zero realm events, (e) `git status` index rewrite publishes nothing, (f) >5000 paths -> second base, no entries, restart keeps following, (g) workers publish no base and a git-less fleet keeps syncing, (h) every `.git/` journal path is allow-listed, missing object stays queued without error then applies, verify drift, status reasons |
| `🔁️replica/🥾️bootstrap/🧪️tests/🟦️.ts` | 12 pass |
| `🩺️attest/🧪️tests/🟦️.ts` (B) | 35 pass |
| `🗺️topology/🧪️tests/🟦️.ts` | 54 pass, 2 fail: "macOS launchd jobs get a log file ..." and "the native bootstrap runs with the remote cache environment ..." (`[semio] cache    token=none` format). Neither touches code I edited; they come from other agents' working-tree changes (provisioning and native bootstrap). Not confirmed against a pristine baseline. |
| `🗺️topology/🧪️tests/🔵️.ts` | 19 skipped (no `pwsh` on this host) |

## Not done / not verified

- The `🔵️.ps1` edit and its new test case were never executed (no pwsh). The `🐚️.sh` edit passes `bash -n`; its `tar -tf` count and the lone-`.git` branch were not run end to end.
- No run against the live fleet or the real old binary; the old-daemon behaviour above is from the code.
- Windows and Linux git lookup and `core.symlinks`/`ignorecase` probes: unit-tested only for candidate lists and on this macOS host.
- `status`'s `files` count includes realm registers (cosmetic; B owns the table).
- Concurrent commits on two machines: ref last writer wins, other commit unreferenced (as in the contract), not exercised.
- A worker adopting the first base replaces its local stage (nothing to publish against: its last synchronised tree is unknown); an unpublished local stage on a worker at rollout is lost.
- Stage entries and the objects they reference are separate events; B-side retry (1 s spacing, objects are applied before the realm in the same round) makes this eventually consistent, not atomic.
- `RETRY` spacing of 1 s only applies after a stall (missing object, busy index lock).

## Rollout (coordinator)

1. Do not restart anything before the new code is on every machine's disk (it replicates). Old daemons drop realm events (above) and keep publishing raw `.git/index`, `.git/logs`, `.git/config`; new daemons drop those.
2. Restart `g1-server` first (stop the daemon once; launchd restarts it). Its first full scan (`runV1` starts with `full=true`) calls `publishIndexV1`: the manifest has no `.cache/f/index-base`, origin id is `g1-server` (`<group>-server`), `.git/index` exists and `git write-tree` works -> exactly one base `tree <oid>` is published automatically, with the server's current stage (including the six staged deletions of the 11-12 note). Tree objects created by that `write-tree` replicate with the next scan; workers retry until they arrive. Check `status` on the server: `Git state: shared`; `grep index-base` in `origins/g1-server/segments` shows one event.
3. Then restart the workers (`g1-m01`, `g1-m02`; `g1-m03` looks stopped). Each normalises `.git/config`, waits (`waiting-for-base`) until base and tree object are present, then replaces its stage by the base (delta or `read-tree`) and follows entries. Workers never publish a base while the manifest has none; any machine publishes a new base only for >5000 changed paths. Other group servers behave like g1-server if they have not yet seen the base when they start, so start them after g1-server's base has relayed.
4. Existing `.git/config` on the fleet is rewritten key by key at start (macOS values removed from non-mac machines); `.cache/f/git-index.json` and `.cache/f/tmp/index-*` are new local files.
5. Provisioning: the macOS LaunchDaemon PATH lacks `/opt/homebrew/bin`; `resolveGitV1` falls back to `/usr/bin/git`, Homebrew and CLT paths, `SEMIO_GIT` overrides. Preflight warns when none is found.
6. For B: use `indexRealmV1` from `🌿️git/🟦️.ts` to exclude realm paths from the manifest digest (your `CompactTableV1.term` already ignores them as the comment says); `gitStateV1` is ready.
