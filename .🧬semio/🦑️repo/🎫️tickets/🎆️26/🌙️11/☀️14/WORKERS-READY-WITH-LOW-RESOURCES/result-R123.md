# R123: test runs leaked attached disk images

## Cause
Private test repositories (`.cache/test-sandbox/r<6>`, `🧫️private-repository`) attach `.cache/b/<partition>/fixed.dmg` images and nested `tmp/gf-*` images. They were removed only by `afterAll` (never runs on kill/timeout) and by a reaper that ran only when a later test created its own private repository, judged the owner by pid alone, and retired through the storage protocol, which refuses ("partition has readers") while a nested image is mounted. So dead owners left images and directories for hours, and the capacity gate counted them as `attaches` (maximum 32).

## Built (all under `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep/`)
- Schema `🧬️schema/🔣️.json` (Owner, LedgerRow, Report, messages EN/DE); `🟦️.ts`: owner row `{pid,start}` (identity by pid + `ps lstart`), ledger `.images.jsonl` (device, image, pid, start), `classifyAttachedImagesV1` (ours / leaked / foreign), `sweepLeakedSandboxImagesV1` (detach leaked, deepest image path first, then remove dead sandbox dirs only if no image of them remains attached), `📜️script.ts` entry (`--root`, `--dry-run`).
- `🖼️images`: rows now carry `device`.
- Private repository: owner written with start time, exit hook detaches and removes, reaper replaced by the sweep, retirement detaches before `rm`. Roster `invalidateNativeImageRosterV1` registers attaches in the ledger inside a private repository.
- Gate (`host-capacity/🎫️gate`): when `attachedImages >= maximum` it runs the sweep (option `sweepImages`, default the real sweep scoped by `SEMIO_WORKSPACE_ROOT`) at most once per progress interval, lowers the sample by what it detached, and the waiting line appends "N disk images are attached: a are ours, b belong to dead test sandboxes and c are foreign" (EN/DE).
- Doctor: repair `host-leaked-images` (policy, rule `host.images-many` fix now automatic: `repo doctor --fix --only=host`, dry run counts only).

## Proof
- `image-sweep/🧪️tests` 4: real nested HFS images, owner process SIGKILLed, ledger rows, sweep detaches both nested first, removes the sandbox, a live sandbox stays; pid reuse (start mismatch) is not the owner; classification and EN/DE text.
- `host-capacity/🧪️tests/💿️leaked-images` 3 (swept and admitted at once, split line EN/DE, throwing sweep never stops the gate).
- Harness runs: gate 31, roster 16, private-workspace 6, doctor 17/14/9, preflights 2 and 10 pass; sandbox main 21 pass directly (2 fail under the harness only: long TMPDIR, "Short test sandbox root required").

## Sweep now
First unscoped run happened by accident inside my own test (before I added `under`): it detached 19 leaked dead-sandbox images. Then the real entry (`📜️script.ts --root=/Users/ueli/Documents/semio`): 0 images left to detach, 9 dead sandbox directories removed, 0 kept. `hdiutil info | grep -c image-path`: 30 at start (09:34 UTC listing 39 at 09:31), 11 after; the 11 are the real checkout images. Sandboxes with live owners: none existed.

## Deviations and unverified
- I edited live files before taking `landing.lock` (the lock and test run followed at 10:00; see landings.md entry). Red-first was not shown cleanly: the module did not exist before its test.
- The ledger is written at roster invalidation (after attach/detach in the attaching process), not at each `hdiutil attach` call site; the sweep decides by sandbox owner, not ledger.
- Doctor repair only runs while the count is at the warn threshold. Not tested: Linux/Windows (no images there), the 5-minute `reclaim` of `st` sandboxes is unchanged. The two agent-scratch images (R58) are foreign and are never touched.
