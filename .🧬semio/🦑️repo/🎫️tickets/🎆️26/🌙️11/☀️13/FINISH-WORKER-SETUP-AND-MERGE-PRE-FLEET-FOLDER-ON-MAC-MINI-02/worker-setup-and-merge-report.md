# Worker Setup And Merge Report (mac-mini-m4-02, g1-m07)

The join itself is recorded in `26/11/09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-m07-join.md`. This file covers what was on the machine, what was merged and what was removed.

## Before

| Tree | What it was | Fleet state |
|---|---|---|
| `~/Documents/semio` | bootstrap archive (sha256 `127d398c…e6f01c`, 273256 files) extracted by the `--plan` run of 20:45 | not joined, no `machine.json`, no replica service |
| `~/Documents/semio.pre-fleet` | the account's git clone, moved aside at 20:17 | never joined |
| `/Users/Shared/semio` | the first worktree of this machine | joined as `g1-m05` at 05:19, sync off since 05:24, services removed at 19:41 |

## Join

`provision-g1-m07.sh` (this folder) ran the provision script of a topology copy outside the worktree through an administrator prompt: adopt path, origin `g1-m07`, exit 3 (fleet ready, native build pending). Then:

- 29 names that macOS `tar` had extracted as NFD were renamed to NFC with `normalize-names.py` (this folder). `verify` went from 110 drifting paths to 1.
- `🗺️topology/📜️script.ts` was set back to `0644` (the fleet and the index say not executable).
- `preflight`: exit 0, none failed, 2 warnings (time synchronisation needs root, native build pending).
- `fleet`: view `44/0/22`, code `35240fac3f31`, the same as `g1-m01`, `g1-m02`, `g1-m03` and `g1-server`. The verdict of every active origin is `lagging (not settled)` because the tree is being edited.
- `verify`: exit 1 on one path, `undefined.stale`, which `g1-m02` and `g1-m01` rewrite every few seconds (finding in the join report).

## Merge Audit

`audit-old-tree.py` (this folder) compared every file of both old trees (without `.git`, `.cache`, `node_modules`) byte by byte with the joined canonical tree, and looked up the sha256 of every other file in the blobs of the store.

| Old tree | Same bytes in canonical | Older fleet version (blob in the store) | Neither |
|---|---|---|---|
| `~/Documents/semio.pre-fleet` | 269445 | 1847 | 119 |
| `/Users/Shared/semio` | 270560 | 1802 | 3 |

- `semio.pre-fleet`: `git status --ignored -uall` was empty, no stash, no commit that is not on `origin`, tip `5ececaa78b9` equal to the fleet tip, no own hooks, no exclude rules. All 1966 files that differ from the canonical tree are tracked at that commit, and the canonical `.git` holds it. The 119 files without a blob are committed files that the root `.gitignore` ignores (`*.pid`, `.nx` workspace data, `*.sqlite`, `jco` output). Nothing to merge.
- `/Users/Shared/semio`: one local file, `26/11/09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-m05-collision-mac-mini-m4-02.md` (written 19:41 with sync off). **Merged**: copied to the same path in the canonical tree and published by `g1-m07` at 21:15:47. The other two are the ignored native `repo` build of 05:21.

## Removed

- `/Users/Shared/semio` (12 GB; its two build volumes under `.cache/b` were detached first).
- `~/Documents/semio.pre-fleet` (9.1 GB).
- The 8.9 GB copy of the bootstrap archive in the scratch folder of the earlier session.

`~/Documents/semio` is the only semio tree on the machine. `origins/g1-m05` in the store was not retired: that id belongs to the collision note.

## Left As It Is

- `/var/log/semio-fleet-replica.log` and `semio-fleet-compact.log`: logs of the removed `g1-m05` system daemons, root-owned directory.
- `.cache/f/lost` in the canonical tree: 1720 archive versions that the fleet replaced during the adoption, and the old index.
- GitKraken still lists the removed `semio.pre-fleet` clone.

## Native Build

Not complete at the time of writing: `.cache/f/native-bootstrap.done` is absent, `preflight` shows the warning `native build`. `g1-m01`, `g1-m04` and `g1-m06` report the same state in their join files.

`native-bootstrap.sh` (this folder) runs step 9 of the provision script as `ueli` without the administrator prompt and writes the marker only after exit 0.

| Run | Reached | Stopped at |
|---|---|---|
| 1 (inside the provision run, 21:01) | dependency scopes | `The lock file of dependency scope ♻️mit-bestand is out of date … 🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript is a member the lock does not list`: the NFD directory name. Gone after the rename. |
| 2 (21:33) | dependency scopes | `Internal error: Exact privately produced preservation capsule required`: the sync moved the plain `node_modules` directory of 20:48 into the capsule `.pddc3bcee`, published the link into the dependency partition, and its own recheck then found another stamp on the capsule directory. `repo dependencies sync --all` afterwards exited 0 and removed the capsule. |
| 3 (22:02) | `workspace:setup` | `The graph service could not answer (timeout)` after 62 s on the loaded host. `repo service host start` afterwards exited 0. |
| 4 (22:13) | `workspace:setup` | the project graph slice was read after about an hour; `nx run workspace:setup` then failed 16 of 30 tasks with `RepoError: stale build.closure.source-differs {"path":"🧰️framework/🔨️modules/🏃️process/📖️texts/🔣️.json", …}` (exit 130): another machine changed a source of the build implementation while the run was in flight. |
| 5 (23:26) | `workspace:setup` | stopped by hand at 00:16 after 45 minutes of `Reading the project graph slice …: reservation 0 of 6627 … waiting for memory headroom (6627 of 2638 – 6635 MB reserved)`. |
| 6 (00:17, `SEMIO_GRAPH_MEMORY_MB=4096`) | `workspace:setup` | the override does not change this reservation (still 6627); stopped by hand at 00:36. |
| 7 (00:37, after `repo service host stop` and `start`: the resident graph service had grown from 125 MB to 1.3 GB) | `workspace:setup` | stopped by hand at 02:13 after 5507 s. The reported headroom reached 7687, 7813 and 7830 MB, above the 6627 MB asked, and the reservation stayed 0. |
| 8 (02:49, after deleting the learned slice peak and restarting the graph service) | `workspace:setup`, all of it | the slice was read in about 22 minutes; `nx run workspace:setup` then ran 11 minutes and failed 8 of 30 tasks (exit 130), each on a refusal of the repository's own build code, listed below. |

The two follow-up sessions of this ticket (probe removal, adopt path) were archived at 00:06 to free memory. The slice reservation that never gets granted belongs to the area of `🕸️graph-slice-slowness.md` (host memory watch of the slice pool).

The capacity door waited before every run (`need 6144 MB; about 4000 – 4900 MB spare above the 2048 MB floor`) and let the run through after 1 to 10 minutes.

To retry: `bash native-bootstrap.sh <log>` from this folder, or the provision script again.

## Git Unreadable After The Repack Of g1-server (02:04 – 02:47)

Symptom: GitKraken showed nothing for `~/Documents/semio`; `git log` said `fatal: cannot read commit object 5ececaa78b9…`, `status` of the replica said `Git state: waiting for objects of the shared stage that have not arrived`.

Cause, read from the journals in the store: at 02:03:59 `g1-server` published a repack of the shared `.git/objects`: `put` of `pack-afe8dcf2….idx` (58.8 MB) and `.rev`, of the cruft pack `pack-3d96785e…` and of `multi-pack-index`, and `delete` of 20 packs (60 files) and 38 loose objects. The `put` of `pack-afe8dcf2….pack` itself is in no journal; it had not arrived 43 minutes later. Every worker applied the deletes, so no worker could read `HEAD`. `gc.auto=0` and `maintenance.auto=false` are pinned, so the repack was started by hand or by a tool on `g1-server`.

Repair on `g1-m07`: the blobs of all deleted packs were still in the store. `list-deleted-packs.py` lists them, `restore-pack.sh` and `restore-packs.sh` (this folder) copied `pack-58516ce4…` (3.67 GB, which does not hold `HEAD`) and the 17 packs below 400 MB (which do) back into `.git/objects/pack`, sha256 checked, pack before index. The daemon published them (`g1-m07`, from 02:42), so the other workers get them back too.

After: `git log` shows `5ececaa78b92`, `git rev-list --count --all` 4337, `git write-tree` `47eebd16b9bf…` (the fleet's stage tree), `.cache/f/git-index.json` `shared` since 02:47. The three other 4 GB packs of the delete list (`303ed9a6…`, `50840966…`, `d4173d1d…`) were not restored.

Open: `pack-afe8dcf2….idx`, `.rev` and the `multi-pack-index` that names it are in every tree without the pack; git reads past them. The replica publishes a pack's index and the deletes of the packs it replaces before the pack itself (finding in the join file). In the store's `tmp`, an origin `g1-m05` (processes 3892, 34652; not this machine) keeps starting uploads of 1.3 – 2.3 GB that never finish.

## Why Runs 5 To 7 Never Got Their Reservation

`createSlicePoolV1` (`🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`) books `max(learned peak, 2048)` MB in the machine-wide capacity gate through `reserveGraphPlanCapacityV1` (`🏗️build/🌡️host-capacity/🟦️.ts`) before a slice worker starts. The learned peak is the file `$TMPDIR/semio-slice-peak-<uid>.json`; run 4 wrote `6627` into it. That booking is a second request beside the grant the run already holds (`need 6144 MB` at the capacity door, 8190 MB asked in run 8): 6627 + 6144 MB above the 2048 MB floor cannot fit on a 16 GB host, so the wait never ends and the progress line shows the host headroom, not what the gate has left. `SEMIO_GRAPH_MEMORY_MB` does not enter this path. Deleting the file made run 8 book 2048 MB and pass. Run 8 wrote `8561` into the file, so the next run on this host waits again until the file is deleted. Finding for `26/11/13/GET-MIT-BESTAND-DEMONSTRATOR-RUNNING-END-TO-END/🕸️graph-slice-slowness.md`: the plan of a run must be paid by the grant of that run, and the learned peak must not exceed what the gate can ever grant.

## What Stops `workspace:setup` Now (Run 8)

| Task | Refusal |
|---|---|
| `workspace:deps-js-all` | `Original JavaScript repository required` (`⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/📥️read-context/🟦️.ts:44`); before it: `The dependency generation receipt of scope . is stale: … ENOTDIR … lstat '/Users/ueli/Documents/semio/CLAUDE.md/package.json'` (`CLAUDE.md` is a symlink to `AGENTS.md`) |
| `@semio-tech/repo-lib:cpp-setup` | `Single-link regular artifact source required` (`🏃️process/📦️artifacts/📤️publication/📥️sources/🟦️.ts:20`) |
| `@semio-tech/ui-styling-py:deps` | `Closed TypeScript artifact layout required` (`⚡️caching/📦️artifacts/💾️storage/🟦️.ts:8`) |
| `@semio-tech/energy-oracle-py:deps` | `uv python install 3.12`: `Python downloads are not allowed (python-downloads = "never")` |
| `@semio-tech/repo-test-dotnet:deps` | `artifact:dotnet-support failed (1)` |
| `workspace:deps-build-tools` | `Internal error: Original Repo build caller required` (`repo tools setup all` below a cargo owner command) |
| `@semio-tech/framework-os-mcp-rs:build` | `Unknown or ambiguous selected Cargo package: Cargo.toml semio-framework-os-mcp` (`🗂️workspaces/🦀️cargo/🟦️.ts:386`) |
| `workspace:deps-cargo` | follows the above |

None of these is a setting of this machine: they are refusals in build code that is being changed on the fleet. The other workers with a join file are in the same state: `g1-m01` (exit 3, `DOTNET_ROOT` unbound), `g1-m04` (marker absent, native error open), `g1-m06` (exit 3, marker absent).

## State At 02:53 On 14 November

- `verify`: exit 0, `No drift: 273200 shared paths agree with the state and the store` (the `undefined.stale` probe is gone).
- `fleet`: `g1-m07` verdict `ok`, code `4e292b9e97b6` and view `369/12/349`, the same as `g1-server` and `g1-m06`.
- Git state `shared`, `HEAD` `5ececaa78b92`, stage tree `47eebd16b9bf…`.

## Runs 9 And Later, And The Fixes Made (14 November, 03:57 – 05:15)

Another machine changed the slice booking at 03:14 (`sliceBookingV1`: capped at `hostCapBasisPoints` of the physical memory, with a minimum to start with), so the reservation stall of runs 5 to 7 is fixed in the shared code. Run 9 (03:58, full bootstrap, 45 minutes) failed 9 of 30 tasks: the 8 of run 8 and `workspace:deps-trunk` (`Export named 'unsatisfiableHostRequestV1' not found`, a file another machine was editing; the export exists now). A single target through the build takes about 4 minutes now (`repo run workspace:<target>`), which was used from here on.

Causes found and what was changed:

| Task | Cause | Change |
|---|---|---|
| `workspace:deps-js-all`, `workspace:deps-build-tools` | They are the only workspace dependency targets without `metadata.semio.execution = orchestration`. Project assembly (`🧱️project-assembly/🟨️.mjs:388`) wraps every non-orchestration target of the root project whose command closure reaches a native driver in `cargo native owner-command`; below that wrap the child holds native owner capabilities, so `requireJavascriptDependencyStorageV1` (`workspaceRoot` absent) and `readOwnedBuildInvocationV1` refuse it. | `📋️project.json`: both targets declared `orchestration`, like `deps-cargo`, `deps-go`, `deps-python`, … **Not effective yet**: see below. |
| `@semio-tech/framework-os-mcp-rs:build` | `📡️stdio/🏗️staging/🧬️schema/🔣️.json` named the Cargo package `semio-framework-os-mcp`; the crate, `Cargo.lock` and `MCP_CARGO_PACKAGE` say `semio-os-mcp`. | Schema and its test corrected. Not rerun. |
| `@semio-tech/ui-styling-py:deps` | `🎨️styling/🏗️builder/🐍️python/🟦️.ts` was the only caller of `withArtifactScratchV1` whose request lacked `version:1`, which the request schema requires. | `version:1` added. Not rerun. |
| `@semio-tech/repo-test-dotnet:deps` | `🧪️tests/🔬️dotnet-package-commands/🟦️.ts` ran `dotnet restore 🧪️Semio.Repo.Test.csproj` in its own directory; the project file is in `📦️packages/🔷️dotnet`. | The command runs in the package root the script router passes. Not rerun. |
| `@semio-tech/repo-lib:cpp-setup` | The compiler resolves to `/usr/bin/c++`, the macOS shim with 78 hard links, which `Single-link regular artifact source required` refuses; the tools environment of a completed `repo tools setup all` is expected to name the compiler. | None: follows `deps-build-tools`. |
| `@semio-tech/energy-oracle-py:deps` | `uv python install 3.12` under `UV_PYTHON_DOWNLOADS=never` of the tool registry (`🧰️tools/🔣️.json:1144`), while the Python toolchain plan asks for `manual`. | None: open (which of the two policies is meant). |
| `workspace:deps-cargo` | follows the others | None. |

**Open blocker.** The graph does not pick up the changed root `📋️project.json`: `repo show project workspace` (policy `fresh`) and `repo run` still plan both targets with the native wrap and without the `execution` metadata, after a restart of the graph service, an atomic replace of the file and two new generations (34, 35: `24 dirty owner(s)`). Uncommitted edits of other project files are reflected (`framework-os-mcp-rs`). `repo reset` failed with `Internal error: Unknown closure reference publication: inference/selections/8d8f…/np/007e….json.fleet`. Until the root owner is inferred again, the first two fixes cannot be verified and the native build cannot complete.

## Verification Of The Fixes (05:16 – 06:00)

**Why the graph ignored the changed root `📋️project.json`.** The membership frontier of a generation records `[path, mode]` per member and no content stamp; generation 33 (04:54) saw the membership digest move and returned no dirty owner, and the stored root fragment (`e560675f…`, written 04:32) was carried through generations 33 to 35. Moving the fragment away fails the plan (`Internal error: Edge refresh requires the published fragment`; it was put back). A mode change of the file is a membership change of a known config and marks the owner dirty: `chmod 755`, one `repo show project workspace` (`graph: generation 36 published, 1 projects refreshed in 89 s`), `chmod 644`. After that both targets are planned without the native wrap. Finding for the graph: a content edit of the root project file alone does not dirty its owner.

| Target (`repo run …`) | Result after the fixes |
|---|---|
| `workspace:deps-js-all` | **passes** (`Successfully ran target deps-js-all`) |
| `@semio-tech/repo-test-dotnet:deps` | **passes** |
| `@semio-tech/ui-styling-py:deps` | **passes** on the second call (the first met `Relative filesystem fstatat failed (EPERM)` in the resident service, reference `e07192d7`) |
| `@semio-tech/framework-os-mcp-rs:build` | past the package name; now `Selected Cargo compiler path exceeds 256 characters in 2 selected units`: `…/.cache/b/r/m/u/l/q0c9bae6d18f/prototype/build/semio-framework-replication-retained-field-preparation/0000000000000000/out/semio_framework_replication_retained_field_preparation-….rcgu.o` is 262 characters below `/Users/ueli/Documents/semio`, the location of every Mac worker. The crate (`📡️replication/🎮️mutation/🧳️retained/🪡️field/🧳️preparation`, committed 8 November) needs a name at least 6 characters shorter, in both places of the path 3 each. Not changed here. |
| `workspace:deps-build-tools` | planned as orchestration, still `Internal error: Original Repo build caller required`: the command `repo ./📜️script.ts repo tools setup all` is a nested repo operation, and `readOwnedBuildInvocationV1` (`🏗️build/⌨️command/⚙️runtime/🟦️.ts:19`) admits a nested operation only from the declared launches of its parent; an orchestration target can declare child targets (`execution.children`), not `tools setup`. There is no script entry for the tools setup outside the repo runtime. Open: the target needs an entry of its own (like `dependencies/🏗️native/📜️script.ts sync <kind>` of its siblings) or the authority model needs a launch for it. |
| `@semio-tech/repo-lib:cpp-setup`, `@semio-tech/energy-oracle-py:deps`, `workspace:deps-cargo` | not rerun (first follows the tools setup, second is the open `UV_PYTHON_DOWNLOADS` policy conflict, third follows the others) |

**State of the native build.** `.cache/f/native-bootstrap.done` is absent. `workspace:setup` cannot pass on any Mac worker at `/Users/ueli/Documents/semio` while the path envelope refuses that crate and `deps-build-tools` nests a repo operation; neither is a setting of this machine.

## Crate Rename And Three More Runs Of `framework-os-mcp-rs:build` (06:01 – 06:48)

`rename-crate.py` (this folder) renamed the crate `semio-framework-replication-retained-field-preparation` to `semio-framework-replication-retained-field-prep` in 115 files (its manifest, the dependent manifest, two tests, one source, `📋️build-owners.json` and every `Cargo.lock` that lists it; the lock order is unchanged). The refused path is 248 characters with the new name. The rename is **not verified by a build**: none of the three runs afterwards reached the Cargo path envelope.

| Run | Stopped at |
|---|---|
| 06:04 (25 minutes, graph refresh after the rename) | `Internal error: Captured authorizer byte capacity exceeded` during `Verifying build tool integrity` (`🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts`) |
| 06:30 (13 seconds) | `The resident service met an unexpected failure (reference e07192d7)`: `Relative filesystem fstatat failed (EPERM)`, the same reference as at 05:44 |
| 06:30 (18 minutes) | `The code of the worktree does not start (Cannot find module '…/.cache/b/t/m/🔁️replica/⌨️command/🟦️.ts' imported from …/nx/.stage-GobUkq/.semio-authorizer-compiler.mjs)`, `nx:authorizers failed with exit code 1` |

Each run met another refusal of the authorizer and replica code that other machines were changing at that hour. `workspace:deps-build-tools` was not changed further: the README of the repo product documents that `workspace:setup` depends on it, so the nested `repo tools setup all` is the intended design and the build caller authority has to admit it; which envelope the task receives was not captured.

At 06:48 the new pack of the repack of `g1-server` (`pack-afe8dcf2….pack`) had still not arrived.

## 06:50 – 06:57

- GitKraken was opened on `~/Documents/semio` (`open -a GitKraken`); its log `~/.gitkraken/logs/app-2026-11-14.txt` shows `Open repository /Users/ueli/Documents/semio/: started` at 05:50:05Z and `finished` at 05:50:26Z without an error. The window was not seen: `screencapture` answers `could not create image from display` (no screen recording permission for this session).
- `repo run workspace:deps-build-tools` at 06:51 stopped before any task, like the run of 06:30: `The code of the worktree does not start (Cannot find module '…/.cache/b/t/m/🔁️replica/⌨️command/🟦️.ts' imported from …/nx/.stage-…/.semio-authorizer-compiler.mjs)`, `No replica code starts`. The text is the one of `🤝️shared-worktree/🔁️replica/🛟️supervisor/🟦️.ts:188`: the staged authorizer compiler runs replica supervisor code and resolves the replica command below the staging directory. Build files of `📚️library` (`⚡️caching/🦀️cargo/🗃️unit-cache`, `🏗️build/🪪️graph-state`, `🏗️build/🌐️remote-execution`, …) were being written by other machines in that hour. No task can be run through the build on this machine until that is resolved, so the envelope of the build-tools task could not be captured.

## Cause Of `The code of the worktree does not start` In Every `repo run` (found 07:00)

Still present at 06:58 (`repo run workspace:deps-build-tools`, 51 seconds). `📚️library/🏗️build/🪪️graph-state/📥️replica/🟦️.ts` (written 05:58 by another machine, staged as new) imports `🤝️shared-worktree/🔁️replica/🟦️.ts` and `💾️state/🟦️.ts`; `📍️cursor/🟦️.ts` and `📜️script.ts` of `🪪️graph-state` import it, and the source closure of the nx authorizer compiler reaches them. The compiler is one bundled program (`.cache/b/t/m/nx/.stage-…/.semio-authorizer-compiler.mjs`), so the replica code is inside the bundle: first the closure outgrew its limit (`Captured authorizer byte capacity exceeded`, 06:04), and since then the main guard of `🛟️supervisor/🟦️.ts:376` (`canonical(entry)===canonical(fileURLToPath(import.meta.url))`) is true inside the bundle, the supervisor starts as if it were the process entry, and resolves `🔁️replica/⌨️command/🟦️.ts` two levels above the staging directory. Not changed here: both sides are files another machine is working on, and the supervisor is the code the replica daemon of every machine runs. Either the cursor reads the replica state without importing the replica module, or the supervisor entry moves out of a module that library code can reach.

## Replica Book Module, Envelope Of The Build-Tools Task, Standalone Tools Setup (07:03 – 07:45)

**Authorizer breakage fixed.** `extract-replica-book.py` (this folder) moved `ReplicaBookV1`, `CompactTableV1`, `localPathsV1`, `LocalPathsV1` and the helpers they use (`widen`, `hashText`, `encoder`, `pageBytes`, `settleHeap`, `racyMilliseconds`) from `🤝️shared-worktree/🔁️replica/🟦️.ts` into `🔁️replica/📇️book/🟦️.ts`; the replica module imports them and re-exports the four public names explicitly; `🏗️build/🪪️graph-state/📥️replica/🟦️.ts` imports the book module. Two follow-up edits added what the first cut missed (the attest fold functions, `foldNameV1`, a type import of `ReplicaQueueV1`, `settleHeap`, `racyMilliseconds`); in between `self-check` failed twice (`emptyBucketsV1 is not defined`, `racyMilliseconds is not defined`) and the daemons kept their running code. Checks: `repo ./📜️script.ts self-check` passes (`25 modules load, 15 schemas compile`), `bun test ./🔁️replica/🧪️tests/🟦️.ts` 94 pass, 0 fail; the bundle of the graph-state reader no longer contains the supervisor; `repo run workspace:deps-build-tools` runs its task again (07:07, 07:12, 07:15).

**Envelope of the build-tools task (captured from the process environment).** The process `bun exec repo ./📜️script.ts repo tools setup all` holds `SEMIO_SCRIPT_PROCESS_INVOCATION` with policy owner `repository:build` and capabilities `arguments: ["nx","run","workspace:deps-build-tools"]`, `launches: [["nx","run","workspace:deps-build-tools"],["nx","run","workspace:deps-build-tools","--parallel=5"]]`, no `tool`. `readOwnedBuildInvocationV1` requires `arguments[0]==="repo"`, so the nested repo operation is refused with `Original Repo build caller required`. A repo caller is projected only for an orchestration target that declares `execution.children`, and its launches are the `repo run` requests of those child targets; there is no declaration for a nested `repo tools setup all`.

**Standalone `repo tools setup all` (07:16 – 07:44).** After 28 minutes of `Checking declared package providers` it ended with `Internal error: Bounded complete dependency directory required` (`⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts:168`, `repositoryDependencySelectedNamesV1`). So the tools setup does not complete on this machine by either route, and the toolchain that `cpp-setup` and the Python interpreter install depend on is not provisioned.

The native build stays pending; `.cache/f/native-bootstrap.done` is absent.

## Why The Standalone Tools Setup Stops (07:46 – 07:50)

`repositoryDependencySelectedNamesV1` lists the root of the dependencies partition (`.cache/b/d/m`) against `SelectedPlanProtocol.maximumRoles` = 16 (`⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json:2561`, last changed 5 November). The root holds 17 entries here: `.fseventsd`, `.metadata_never_index`, `bootstrap`, `bun-bin`, `bun-cache`, `bun-config`, `bun-global`, `bun-home`, `dependencies.json`, `dotnet`, `generation`, `j`, `nuget`, `pointer`, `python`, `storage-events.jsonl`, `tmp`. The 17th name raises `Bounded complete dependency directory required`. `dotnet` and `nuget` (03:19) and `python` (05:59) were created by the dependency tasks of .NET and Python that pass since the fixes of this ticket; `generation` and `pointer` come from `repo dependencies sync`.

Raising the bound alone does not help: the next line requires every name to be one of the declared roles (`directoryRoles`: `{{generationDirectory}}`, `{{recipe.cache}}`, `{{recipe.global}}`, `{{recipe.bin}}`, `tmp`, `{{recipe.home}}`, `{{recipe.config}}`, `bootstrap`; `fileRoles`: `dependencies.json`, `storage-events.jsonl`; `service`: `.fseventsd`) and refuses the rest with `Closed current dependency image roles required`. `.metadata_never_index`, `generation`, `pointer`, `dotnet`, `nuget` and `python` are not declared. The selected dependency plan of `publication/🟦️.ts` (31 October) and the writers of the partition (generation receipts, .NET, NuGet, Python) disagree about what the partition may hold. Not changed here: which roles the plan has to know, and what it does with them when it publishes or retires a generation, is a decision of that protocol.

With that, `repo tools setup all` fails standalone, and inside `workspace:setup` it is refused before it starts (envelope, section above). The toolchain is not provisioned, so `cpp-setup` and the Python interpreter install cannot pass, and the native build stays pending.
