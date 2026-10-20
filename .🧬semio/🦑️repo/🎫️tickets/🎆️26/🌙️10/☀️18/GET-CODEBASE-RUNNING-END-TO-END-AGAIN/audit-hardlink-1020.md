# Hardlink audit 10-20: who gave 1794 tracked manifests a second link

Auditor: hardlink-audit (read-only). Scope: build library, process module, Bazel adapter, nx SDK tooling, ticket tool scripts of this ticket and of ARCHITECTURE-COURSE-ON-BAZEL. No repo writes, no bun/bazel/cargo runs, no `.cache` or `/private/tmp` scans.

## Bottom line

No code path found that hardlinks a workspace `package.json` or `📋️project.json` directly. Two mechanisms can create links whose second name is later removed by cache GC, and the timing fits the bun/dependency-sync path best. Confidence is medium-low; the culprit is not proven. The fastest way to prove it is a `link(2)` trace during the next sync (see Verification).

Current state: all of `package.json`, `📋️project.json`, `✏️s/package.json`, `🌎️hub/package.json` and the revision reader file have nlink=1 (checked with `stat -f %l`).

## Timeline (local time, 10-20)

| Time | Event | Source |
|---|---|---|
| 15:44-15:49 | ARCHITECTURE x-*-tools staged, x-release dry-runs, x-keys, x-ingress checks | mtimes in ARCHITECTURE-COURSE-ON-BAZEL |
| 15:47 | chain11 (`show projects`) starts, `chain11.log` | GET-CODEBASE `🗑️generated/chain11/` |
| 15:48 | `deps4` `lock --all` rc=0 (`bun install --lockfile-only --backend=hardlink`) | `result-deps4.md` line 16 |
| 15:50-15:52 | nlink=2 appears on 1794 manifests (ctime window); x-proctor build2 at 15:50:55, x-site `closure-ov.json` 15:52:04, x-site `overlay-test.sh` 15:52:33 | `coordination.md` line 331; mtimes |
| ~15:50 | chain11 `show projects` refuses: "Bounded physical Source revision required" | `🗑️generated/chain11/chain11.log` |
| 15:57-15:58 | deps4 sync attempt 1 starts, obsolete-generation GC runs ("retiring obsolete generations"); second links removed 15:58 | `result-deps4.md` lines 17, 22; `coordination.md` line 331 |
| 16:03 | kernel panic (not link-related) | `coordination.md` |

## Ranked suspects

### 1. bun `--backend=hardlink` in the dependency sync (best timing fit, unverified)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:123` builds `bun install --frozen-lockfile|--lockfile-only --<lifecycle> --linker=isolated --backend=hardlink --cache-dir <cache>`. The recipe is pinned in `…/📦️dependencies/💾️storage/🧬️schema/📥️install.json` (`ProducerRecipeProtocol`, `"backend":"hardlink"`).
- bun's hardlink backend links files from its cache into `node_modules`. The repo's `bunfig.toml` sets only `linker="isolated"`, `globalStore=false`.
- Fits: the only link-creating code that runs over the whole 436-member `workspace:*` tree; the sync was active 15:47-15:58; the removal at 15:58 matches the generation GC that started at 15:57; `coordination.md` line 186 records `EPERM … failed to link package` from `bun --backend=hardlink`.
- Gap: the producer writes manifest bytes into its generation directory (`storage/🟦️.ts` ~line 140-152, `files.push({path, bytes})`), so it does not link repo files itself. A link to a repo manifest requires bun to hardlink a workspace-member source file, which I could not confirm from repo code. This is the open question.

### 2. nx SDK derivative copy: `link(source, output)` for shared files (in-library, but store-internal)

- `…/📦️sdk/📥️source/🔗️derivative/🟦️.ts` line 48: for every file whose `nxSdkDerivativeDispositionV1` is not `"copy"` (i.e. any path not under `node_modules/nx/`), `await link(source, output)`. The end check requires "Exact derivative copy or shared source inode".
- The source tree is the pristine SDK under `store.root/sdk/<64-hex>` (`withNxSdkDerivativeProjectionV1` checks `directory==join(store.root,"sdk",basename(directory))`). So this links store files, not workspace manifests. Ruled out for the 1794 repo manifests unless the store path holds repo files, which the code does not do.
- It is still the strongest deliberate hardlink producer in the library. Its second names are removed when a derivative is retired, which would produce exactly the "appears, then vanishes" pattern on the store side.

### 3. Remote-cache store `keep` mode (low)

- `…/🏗️build/🌐️remote-cache/💾️store/🟦️.ts:138`: `link(temporary, final)` then `unlink(temporary)` in `finally`. Both paths are under the store root. Final nlink returns to 1. Not a repo-manifest path.

### 4. Fixture blob materialize hardlink fallback (low, unsafe API)

- `…/🧪️test/🗄️blob-storage/🟦️.ts` lines 70-87: `materializeFixtureBlob` runs `rmSync(targetPath)`, tries `cpSync(…FICLONE_FORCE)`, then `linkSync(source, targetPath)` unless `mutable:true`. Any caller passing a repo path would delete it and link a blob into it. The only exporter is `🧪️test/📦️packages/🟦️typescript/🟦️.ts` line 135; I found no internal caller.

### 5. Test-only hardlinks (low, all temp-scoped)

- `ingress-perf-tools/staged/revision.test.ts` lines 20-21, 48-49 (`linkSync`/`os.link` of `events.jsonl`, under `mkdtemp(tmpdir())`).
- `…/📤️receipt/🧪️tests/📦️child/🟦️.ts` line 13 (`link(file, alias)` under `mkdtemp(SEMIO_TEST_ARTIFACT_DIR)`).
- `…/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts` lines 153-155 (`linkSync` of a lease database, `rmSync` afterwards).
- `…/♻️retirement/🧪️tests/🟦️.ts` line 15 (`linkSync`).
- None of these touch manifests by construction. A test that ran with a real repo `directory` would be the risk; I did not confirm one.

## Ruled out

- Ticket tool scripts (C and ARCHITECTURE-COURSE-ON-BAZEL, `*-tools/`, `staged/`, `apply.py`, `overlay.py`, `mkoverlay.sh`, `mirror-test.py`): the only link operations are `ln -s` (`fixups6-tools/overlay.sh`, `x-site-tools/mkoverlay.sh`, `x-proctor-tools/staged/check.sh`, `x-dev-tools/staged/check.sh`, `bzl-sandbox-tools/prod.sh`), `rsync -a` (no `--link-dest`), `shutil.copy2`, `copytree(symlinks=True)` (`x-ingress-tools/apply.py` mirror), and `cp -c -R` (`lib-defects-tools/overlay.py`, clonefile, does not change nlink). Symlinks cannot raise nlink. No `cp -l`, `cp -al`, `os.link`, `linkSync` or `rsync --link-dest` in any ticket tool.
- Bazel adapter (`🧰️framework/…/📚️library/🏗️build/🔧️tools/🏗️bazel/`): no link call; `.bazelrc` has no hardlink or disk-cache option. Bazel's internal hardlink behavior (disk cache, sandbox) is not verified. No Bazel source is in the repo.
- Process module `💾️state/🧾️records/🟦️.ts`: uses `link(temporary, path)` pairs for persistent records (nlink=2 by design, with a pair check). It applies to state records under the state directory, not manifests. It is a real hardlink producer in `🏃️process`; I did not find a caller that targets manifest paths.
- `x-release-tools/staged/check.sh` line 16 notices a `hardlink mirror` on the revision reader and preloads a stand-in. This is a symptom report from a peer, not a cause.

## Prevention

1. Dependency sync: never use `--backend=hardlink` for a producer that can see tracked files. Use `--backend=copyfile` (or clonefile where it works) for the generation install, or keep the generation outside any tree that includes tracked manifests. Add a guard before and after each sync: refuse if any tracked `package.json` or `📋️project.json` has `stat -f %l` > 1, and name the offending path.
2. nx SDK derivative: `copyNxSdkDerivativeV1` should `copyFile(…, COPYFILE_FICLONE)` for `"shared"` files instead of `link`, or keep `link` only when the source root is provably the store `sdk/` directory. Add a check that the source is under `store.root/sdk`.
3. Blob materialize: remove the hardlink fallback (copy only), and refuse a `targetPath` outside a test-owned run directory.
4. Tickets: add a tool gate forbidding `cp -l`, `ln` (non-`-s`), `os.link`, `linkSync` and `rsync --link-dest` in `*-tools/` and `staged/`, and run `stat -f %l` over the tracked manifest list in each `check.sh`.
5. Replace the `no-nlink-revision.ts` stand-in with the cleanup gate once the source is fixed, so the reader keeps refusing nlink!=1.

## Verification (not run; read-only audit, no heavy processes)

- Best proof: trace link calls during the next sync with `fs_usage -w -f filesys` (needs sudo, so the user must run it) filtered for `link`, or with a `DTrace`/`dtruss` probe on `link(2)`. This names the process and path that creates the second name.
- Cheaper: poll `stat -f %l` on the tracked manifests during the sync and log the first non-1 path with `date`, then check whether the second name is under the bun cache generation or the store `sdk/` directory.

## Files consulted (key)

- `🧰️framework/🔨️modules/🏃️process/💾️state/🧾️records/🟦️.ts` (link pair)
- `…/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` lines 123, 244
- `…/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🟦️.ts` lines 36-48
- `…/📚️library/🏗️build/🌐️remote-cache/💾️store/🟦️.ts` line 138
- `…/📚️library/🧪️test/🗄️blob-storage/🟦️.ts` lines 70-87
- `GET-CODEBASE…/coordination.md` lines 186, 331; `result-deps4.md` lines 13-22; `🗑️generated/chain11/chain11.log`
- `ARCHITECTURE-COURSE-ON-BAZEL/x-release-tools/staged/check.sh` line 16; `x-ingress-tools/apply.py` lines 138-200
