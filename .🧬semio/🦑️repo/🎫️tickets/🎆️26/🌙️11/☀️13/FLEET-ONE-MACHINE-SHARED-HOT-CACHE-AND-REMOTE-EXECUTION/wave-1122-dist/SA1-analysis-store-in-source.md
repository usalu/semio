# 🗄️ SA1: Source Analysis Store Written Into the Checkout (`🔁️replica/sa/`)

Lane SA1, 2026-11-25 01:32-01:4x. Lock `lock.graph-closure` taken 01:32:25, released 01:40:34. The stale locks `lock.graph-closure` (owner `GR23 2026-11-25 00:45:25`) and `lock.worker-ready` (owner `W15 (successor W14) 2026-11-24 23:00:45`) were left by lanes that died with the reboot; both were removed.

`B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, `PN` = `B🕸️graph/📥️inference/📦️project-nodes/`, `SA` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/sa/`.

## Who wrote there

- Process: the **owner-pool children** of this machine. The fleet launchd agent runs `boot.mjs run`, which starts the replica supervisor `🤝️shared-worktree/🔁️replica/🛟️supervisor/🟦️.ts --boot-child run` (pid 928) with **cwd = `🔁️replica`**. The supervisor starts the remote-execution sidecars (`B🌐️remote-execution/🛰️sidecar/🟦️.ts agent|graph|spool`). The `agent` sidecar starts the owner pool (pid 19364, cwd = `🔁️replica`), and the pool spawns one child per owner (`PN🌐️owner-pool/🧒️child/🟨️.mjs`). All of them inherit that cwd.
- Call site: `PN🌐️owner-pool/🔬️inference/🟨️.mjs:39` (before the fix): `analysisStore: analysisStoreDirectoryOfV1(sent.closureStore)`, where `sent` is the session the **publisher** serialized into `.cache/f/owner-pool-waves/<wave>/session.bin`.
- The bad session: decoding the two live waves gave:
  - `7c0535698da359ae`: `workspaceRoot = "C:\semio"`, `closureStore = "C:\semio\.cache\b\g\m\inference\selections\8d8f21d7…\ce"`. It was **published by a Windows fleet machine** (session.bin 00:30).
  - `ae443d857a329196`: `/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f…/ce` (this machine, harmless here).
  
  On macOS, `path.dirname("C:\\semio\\…\\ce")` is `"."`, so `join(".", "sa")` = `"sa"`. `useSourceAnalysisStoreV1` then writes against the process cwd, which is `🔁️replica/sa/`. The directory was created at 00:37:49, seven minutes after the Windows wave appeared.
- Write bursts: from 00:37 until the panic, at 01:19:15 (30 s after the post-reboot sidecar boot), and at 01:37:23-01:37:55. The last burst came from a pool child spawned **before** the fix (children started at 01:33:16, 01:34:01 and 01:34:46 still ran the old code; the fix landed at 01:34:51 and 01:35:03). New children write to `.cache/f/sa` (first file at 01:37:32), outside the checkout. Since 01:37:55 nothing has been written to `SA`.
- Not the cause: the GR22/W15 landings between 00:05 and 00:31 (script-inputs and closure-graph). Nothing they changed touches the analysis store path. The pool code itself (`ownerPoolClosureStoreV1` replaces `closureStore` but not `analysisStore`) is from GR16 (2026-11-24 09:58). The defect stayed latent until a publisher on another platform sent its own path.
- Also wrong in principle, even between macOS machines: a child on another machine derived its analysis store from the **publisher's** absolute path. That path is either absent there, or it points into a directory the child does not own.

## Fix (landed, compile-atomic)

- `PN🟨️.mjs` `analysisStoreDirectoryOfV1` (line ~302): `undefined` for a non-string. For any closure store path that is not absolute on this machine (relative, empty, or another platform's spelling), it now throws `refuseBuildV1("invalid","graph.analysis-store-closure-store-relative",{closureStore})` instead of resolving it. Added the `isAbsolute` import.
- `B📖️texts/🔣️.json`: new key `graph.analysis-store-closure-store-relative` with EN and DE texts.
- `PN🌐️owner-pool/🔬️inference/🟨️.mjs`: new exported `ownerPoolSessionV1({sent, root, shared, base})`, used by `openOwnerPoolInferenceV1`. It derives `analysisStore` from the **shared machine closure store** it binds (`machinePathV1(root,".cache","f","owner-pool-closures")`, so `.cache/f/sa`), never from `sent.closureStore`.
- The other two callers (`PN🟨️.mjs:282` farm/worker sessions, `:659` the parent) use `closureStoreDirectoryV1(workspaceRoot)`, which is absolute (`resolve`d in the storage member). With the refusal, a future relative path fails loudly instead of writing into the cwd.

## Tests (`repo test-file`, one at a time, 1-minute load 15-25)

- NEW `PN🌐️owner-pool/🧪️tests/🗄️analysis-store/🟦️.ts` (+ `🔣️.json` fixture, `🐍️oracle.py` with Python `os.path` as the independent oracle): **3 pass / 0 fail**, 51 expects. It covers: absolute paths derive `sa` beside the store, the same as Python; `ce`, `""`, `./ce`, `.cache/…/ce` and the Windows spelling are refused with EN+DE texts; a pool session's `analysisStore` is `join(dirname(shared.directory),"sa")` for a Windows, relative, foreign-absolute or missing sent closure store.
- Regression: `🗃️shared-closures` 2/0, `🏊️project-farm/🧪️tests/🪆️captured` 8/0, owner-pool main `🧪️tests/🟦️.ts` 32/0.
- `B📖️texts/🧪️tests/🟦️.ts`: 2 pass / **1 fail**. The failure already existed and is not mine: `closure.derived-provider-differs is unused`. No code names that key any more; it belongs to another lane's closure work. My key is used as `"graph.analysis-store-closure-store-relative"`.

## Cleanup

- Quiet checks: the newest mtime under `SA` was 01:37:55 at 01:40:19 and still 01:37:55 at 01:43:38 (0 new files). Over the same span, new children wrote 1,284 files to `.cache/f/sa`, which proves the fix is live. `SA` (4,824 files) was **deleted from the working tree at 01:43:41**; the git index was not touched.
- Stray `sa`/`ce` directories under `🧰️framework/🛍️products/🦑️repo/🔨️modules/` modified after 2026-11-24 23:00 (searched with `-maxdepth 12`, pruning `node_modules`, `.cache` and `🗑️generated`): **none** apart from `SA` itself and its shard `SA/ce` (a two-hex shard of the same store, not a closure store). Nothing at the repo root or at `🧰️framework/` either.

## Git index (read-only facts for the user)

- `git diff --cached --name-only -- 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/sa | wc -l` = **2737** staged entries (checked 01:40, and again after the deletion at 01:43:41: still **2737**). Those index entries now point to files that no longer exist in the working tree.
- To unstage later, from a terminal at a calm moment: `git restore --staged -- '🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/sa'`.

## Host notes / open items

- After the reboot no storage image was attached. `repo test-file` failed with `typescript-compiler-absent` and `repo dependencies ensure` refused, until `repo doctor --fix --only=storage` attached the 8 images.
- The graph image is below its free-space floor: 86 MB free, floor 1 GiB (doctor error). Optional graph caches refuse writes until it is collected: `repo cache gc --dry-run --area=graph`, then `repo doctor --fix --only=caches`.
- Still running with the pre-fix code: the pool runner 19364 and the sidecars, started before 01:35. They never derive an analysis store themselves; the next restart for new code picks the fix up.
- Pool children on other macOS/Linux fleet machines run with cwd in their own `🔁️replica` and wrote there through the same path, which replicates to every machine. They pick up the replicated fix with their next child. If `SA` reappears, check those machines.
- `.gitignore` has no rule that would keep a stray `sa/` out of the index. I did not add one: the root cause is fixed, and a refusal is better than hiding the files.
