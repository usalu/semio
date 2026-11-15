# Windows Worker: One Canonical Repository At `C:\semio`, Then Join (2026-11-13, revised 2026-11-14 23:31)

Facts: `1113-fleet-setup-existing-repo.md`, `1113-worker-prompts.md` (c), `1113-entry-prompts.md` (c). Not yet run on a real Windows machine.

```
You are on the Windows machine of the semio fleet. Goal: exactly one semio repository on this machine, at C:\semio, holding every local change of any older semio repository on this machine, joined to the fleet as a worker and in agreement with it. Work in an elevated PowerShell 7 as the login account and keep that account logged on.

Rules: no git command that changes a working tree or history (no checkout, switch, reset, clean, stash, merge, rebase, pull, commit, restore). The only git writes allowed are `git update-ref` for the refs/old-repo/* and refs/heads/old-repo/* refs named below. No taskkill by name. Do not edit AGENTS.md or fleet-setup.md. Never print a token. Delete nothing except the old repository in step 6, and only when its checks pass.

1. Find the repositories. A semio repository is a directory that contains .git, AGENTS.md and a folder .🧬semio.
   - Check C:\semio.
   - Search every fixed drive to depth 5 for other ones (skip Windows, Program Files, $Recycle.Bin, node_modules, target, .cache). List each with: path, `git -C <p> rev-parse HEAD`, branch, `git -C <p> status --porcelain=v1 -uall --no-renames | Measure-Object -Line`, `git -C <p> stash list | Measure-Object -Line`, total size.
   - If C:\semio\.cache\f\machine.json exists, this machine already joined: its "originId" is the origin and stays. Otherwise the origin is the next unused id: g1-m01 to g1-m07 are taken, so g1-m08 unless S:\v1\origins (after step 3 mounts it) shows that taken too; never reuse an id.
   Write this inventory to C:\semio-merge-<yyyyMMdd-HHmm>\inventory.md (a folder outside every repository; call it LOG).

2. Make C:\semio the repository.
   a. C:\semio missing or empty, one old repository OLD elsewhere: enable long paths first (`New-ItemProperty HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem LongPathsEnabled -Value 1 -PropertyType DWord -Force`), then move it whole. Same volume: `Move-Item -LiteralPath OLD -Destination C:\semio`. Other volume: `robocopy OLD C:\semio /E /MOVE /COPY:DAT /DCOPY:DAT /SL /R:2 /W:2 /NP /LOG:LOG\move.log` and confirm OLD is gone and robocopy reported no failed files. Then go to step 3. Nothing is merged in this case because nothing is left behind.
   b. C:\semio holds a repository and no other exists: go to step 3.
   c. C:\semio holds a repository and one or more old repositories exist elsewhere: merge each OLD into C:\semio with step 2c-1 to 2c-5, oldest HEAD commit date first. If C:\semio is already joined, first run from C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\🤝️shared-worktree\🔁️replica: `repo ./📜️script.ts sync --off --lang en`, and switch it back on with `sync --on` after the last merge.
      2c-1 Commits. Unite the objects: `robocopy OLD\.git\objects C:\semio\.git\objects /E /XC /XN /XO /R:2 /W:2 /NP /LOG:LOG\objects.log` (adds only files that are missing; never overwrites). Then keep every commit reachable under a name of its own (N is 1 for the first OLD, 2 for the second):
           - for each line of `git -C OLD for-each-ref --format="%(refname:short) %(objectname)" refs/heads`: `git -C C:\semio update-ref "refs/heads/old-repo/N/<branch>" <sha>`
           - `git -C C:\semio update-ref refs/old-repo/N/HEAD (git -C OLD rev-parse HEAD)`
           - for each line i of `git -C OLD stash list --format=%H`: `git -C C:\semio update-ref "refs/old-repo/N/stash-<i>" <sha>`
           - `git -C C:\semio fsck --connectivity-only` must report no missing objects for these refs.
      2c-2 Stage. Save what OLD had staged: `cmd /c "git -C OLD diff --cached --binary > LOG\OLD-N-staged.patch"` (through cmd so the bytes stay exact).
      2c-3 Local changes. The local changes of OLD are exactly the paths of `git -C OLD status --porcelain=v1 -uall --no-renames -z` (modified, added, untracked; ignore the deleted ones but list them in LOG\OLD-N-deleted.txt). For each such path that exists in OLD:
           - missing in C:\semio: copy it there.
           - identical bytes (compare SHA-256): nothing.
           - different, OLD newer by LastWriteTimeUtc: copy the C:\semio version to C:\semio\.cache\f\lost\old-repo-N\canonical\<path>, then copy the OLD version over it.
           - different, OLD older: copy the OLD version to C:\semio\.cache\f\lost\old-repo-N\old\<path>; C:\semio keeps its version.
           Preserve timestamps. Write one line per path with the decision to LOG\OLD-N-changes.tsv.
      2c-4 Local-only data. Files that git ignores (caches, toolchains, build output) are moved when C:\semio has no file at that path: `robocopy OLD C:\semio /E /MOV /XC /XN /XO /XD .git /SL /R:2 /W:2 /NP /LOG:LOG\OLD-N-ignored.log`. This never overwrites.
      2c-5 Verify OLD holds nothing unique: every path of 2c-3 is byte-identical in C:\semio or present under .cache\f\lost\old-repo-N\old; every sha of 2c-1 answers `git -C C:\semio cat-file -e <sha>`; the staged patch exists if `git -C OLD diff --cached --quiet` returned 1. Write the result to LOG\OLD-N-verify.md. If anything fails, stop here and report; do not continue to step 6 for this OLD.

3. Dry run of the join. Take the provision script from the kit of the store when S: is mounted (`S:\v1\bootstrap\kit\topology`, published by the group server from its sealed code, `published` is a JSON receipt, `SHA256SUMS` lists the files; verify them with `Get-FileHash`), else copy C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\🤝️shared-worktree\🗺️topology; either way into LOG\topology (outside the worktree). The script brings the fleet code of C:\semio to the sealed code of the fleet itself before its first fleet command (step 4b: it prints `fleet code <digest>: replaced R, added A, identical I`, keeps every replaced file under `.cache\f\lost\provision-code-<stamp>\`; also with -Plan), so an old checkout is no longer a reason to stop; stop only if C:\semio holds no fleet module at all (no 🤝️shared-worktree folder) or the script ends with exit code 5 ("the checkout's fleet code is older than this script and the store offers no newer sealed code"): report that and change nothing. Put the `fleet code` line into the report. Then:
   $env:SEMIO_LANGUAGE='en'; $env:SEMIO_FLEET_GROUP='g1'; $env:SEMIO_FLEET_NFS_HOST='192.168.178.70'; $env:SEMIO_FLEET_NETWORK='192.168.178.0/24'
   Remove-Item Env:SEMIO_FLEET_CACHE_ENDPOINT -ErrorAction SilentlyContinue
   pwsh -NoProfile -ExecutionPolicy Bypass -File LOG\topology\🥾️provision\🔵️.ps1 g1 <origin> -User "$env:USERDOMAIN\$env:USERNAME" -Plan *>&1 | Tee-Object LOG\plan.txt
   Read the plan by its classes. Adopt decides every differing file by its content, never by a modification time: `stale` (an older version of a fleet file, or a file the fleet deleted since, or unchanged since this repository's last commit: the fleet wins, nothing is published, the local bytes go to .cache\f\lost\adopt-<stamp>\stale), `edit` and `publish` (your local work, published), `conflict` (changed here and in the fleet since the base: the fleet's version stays, yours goes to .cache\f\lost\adopt-<stamp>\conflict and into the report). A repository that is days behind shows many `stale` files; that is expected and needs no action.
   Stop and report, changing nothing more, if the plan says the git relation is `unrelated` / no common history, or lists more than 50 `conflict` paths, or more than 1000 paths of new local work (`publish` plus `edit`, the plan prints both limits). Whatever the counts, paste the full conflict list into the report. Before the real run lift the local stashes and branches that live only in this repository's packed-refs into loose refs of their own (`git update-ref refs/old-repo/N/...`, as in step 2c-1); a differing `packed-refs`, `HEAD` or non-branch ref is decided for the fleet and the local copy is kept in .cache\f\lost\adopt-<stamp>\git.

4. Real run: the same command without -Plan, output to LOG\provision.txt. Do not pass -Server, -FirstMachine, -Force. Turn on Developer Mode beforehand if the account cannot create symbolic links. Exit 0 = done; exit 3 = fleet ready, native build pending: fix the named cause and run the same command again.

5. Check, from C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\🤝️shared-worktree\🔁️replica:
   repo preflight --lang en
   repo ./📜️script.ts status --lang en        (running daemon, synchronisation on)
   repo ./📜️script.ts verify --lang en        (exit 0)
   repo ./📜️script.ts fleet --wait 600 --lang en   (this machine's line must read ok; other machines may be stale)
   Then check that every path of LOG\OLD-N-changes.tsv whose decision was "copied" or "OLD newer" still has the OLD bytes in C:\semio (compare SHA-256 against OLD if it still exists, or against the hash you recorded). Report every path that does not.
   Read the adopt report C:\semio\.cache\f\lost\adopt-*.md and copy it to LOG.

6. One repository only. For each OLD that passed 2c-5 and step 5: confirm once more that it contains no file that is missing in C:\semio and not under .cache\f\lost (`robocopy OLD C:\semio /E /L /XC /XN /XO /XD .git /NP` must list 0 files to copy), then remove it: `Remove-Item -LiteralPath OLD -Recurse -Force`. If the check lists files, do not remove it; report them. In case 2a nothing is left to remove.
   Finally search again as in step 1 and confirm that C:\semio is the only semio repository on the machine.

7. Record everything in C:\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️09\SHARED-WORKTREE-FLEET-OVER-NFS\<origin>-join.md: the inventory, which case of step 2 applied, counts per decision of 2c-3, the refs created, the plan summary, provision exit code, status, verify and fleet output, the adopt report path, what was removed in step 6, and anything that stopped you. Keep the LOG folder. Report the fleet line of this machine and whether C:\semio is the only repository.
```
