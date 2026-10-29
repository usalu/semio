# 1029 diskcap result

STATUS: DONE (owner split green, capacity action pending the coordinator's quiesce decision). About 0.82 GiB freed safely. Per the coordinator, the recovery-owner manifest blocker is now FIXED by splitting the owner projects (section "Owner split"; verification state at its end). No cache reset was run. The rust image is 48 GiB and is pinned by its receipt. The infra already has an elastic policy that would create it at 60 GiB. It can only be recreated while no cargo holds the image. The on-demand reset command `repo run repo-cache-reset:rust` is currently dead because of a separate bug (see Blocker). That bug needs a coordinator decision.

## Capacity mechanism (how the 48 GiB came about)
- Image: `.🧬semio/🦑️repo/⚡️cache/rust/fixed/fixed.dmg`, raw UDRW, 51538558976 bytes, case-sensitive APFS, mounted at `…/fixed/m`.
- `image.json` (the receipt, created 10-22 17:21) pins `maximumPhysicalBytes` 51539607552 (48 GiB).
- Declared policy: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`.
  - `partitions.rust.capacityBytes` = 48 GiB.
  - `elastic.rust`: need 48 GiB, headroom 25 %, host-free reserve 50 %, max 64 GiB, align 1 GiB, `automatic` true, `pressureFreeBytes` 6 GiB.
  - Code: `…/📦️budget/📐️elastic/🟦️.ts`. Schema description: `…/📦️budget/🧬️schema/🔣️.json`, `elasticPartition`.
- Elastic sizing was committed on 10-28 (f87d49a21e8), six days after this image was created. The size is applied only when an image is created. An existing image keeps its receipt size (`effectiveCapacityV1` reads the receipt). There is no online grow or resize path in the infra. `hdiutil resize` is used only to stretch a freshly seeded image at birth (`🍎️image/🪶️sparse`).
- A new image would get `elasticCapacityV1(rule, 48 GiB, hostFree 123.9 GiB)` = 60 GiB. This is checked by running it. The repository total budget adjusts by +12 GiB automatically (`repositoryCacheBudgetV1`). Test fixtures `elasticRustCapacity` and a Python oracle already assert 60 GiB. The policy is therefore not undersized relative to the declared design. The running image just predates it.
- Automatic regrow runs on every ordinary `repo` command (`rust.regrow` phase in `🏗️build/⌨️command/⚙️runtime/🟦️.ts`, calling `regrowElasticImageIfQuiescentV1`). It fires only if all three hold:
  - the image is quiescent: one non-waiting exclusive try on lease `native-fixed-image:<dir>` succeeds,
  - mounted free space is below 6 GiB,
  - the journal admits whole retirement.
  - With a continuous fleet it never fires.
- Reset discards the whole image, i.e. all cargo units, lanes and CAS. Recreate means a cold rust cache, then a rebuild.
- On the host, the dmg is allocated at 48 GiB (du). Retiring it returns 48 GiB. A new image grows with its content (sparse birth).

## Self-pruning (it already evicts)
- Unit-cache GC: `…/🦀️cargo/🗃️unit-cache/🧹️gc/` with policy in `🗃️unit-cache/🧬️schema/🔣️.json` (`Policy.gc`).
  - Water marks: high 87.5 %, low 75 %.
  - Emergency: collect before a compile starts if free < 4 GiB.
  - Idle lanes are removed after 24 h. Incremental share 10 % and CAS share 25 % are enforced.
  - The watching collector is alive: `u/gc.log` is `running`, and `u/fill.jsonl` samples free space every 30 s.
- The 87 % you saw is exactly the high-water mark, so this is steady state, not a leak.
- `fill.jsonl` 10-24 → 10-29 (6637 samples) shows a sawtooth between about 5 and 24 GiB free. Samples below 4 GiB: 38. Below 2 GiB: 17. Below 1 GiB: 9. The minimum was 0.04 GiB at 10-29 05:04, which is the ENOSPC burst. Bursts from about 10 concurrent compiles outrun the 30 s collector. A 60 GiB image moves the free floor at high-water from 6 GiB to 7.5 GiB. The infra retries once on ENOSPC (`outOfSpaceRetries` 1).

## Usage breakdown (05:25, image about 39 GiB used / 48)
| Path under `…/fixed/m` | GiB |
|---|---|
| `u/l` (8 per-agent lanes, units plus incremental) | about 22 at 05:20; 5.96 GiB largest lane `p943174/b/prototype` |
| `u/o` (CAS objects) | 4.56 |
| `cargo/` (home 1.64, prototype 3.22, preparation 0.31) | 5.24 |
| `cs/v` | 3.94 |
| `nx/` (tmp 1.1, artifacts 0.23, native 0.04) | 1.36 (about 0.5 after my cleanup) |
| `b/p` | 0.91 |
| `u/a` (admissions) | 1.32 |
| `components`, `published` | 0.56, 0.20 |

Lane `b/<profile>` sizes in GiB, with incremental in brackets:
- `p943174 prototype` 5.96 (4.33)
- `pb7b3d7 prototype` 4.00 (2.87)
- `pf402df wasm32-wasip2/prototype` 3.74 (3.70)
- `pc3f3ff wasm32-wasip2/prototype` 3.21 (3.09)
- `p9109b7 prototype` 2.78 (2.32)
- `p924c76 prototype` 0.81 plus `wasm32-wasip2/prototype` 0.41

Incremental is about 14 GiB of the lane space. All incremental sessions are younger than 2 h, so none is stale.

## Freed
- Stale incremental sessions older than 120 min: 0 found, so 0 B freed. The GC already removes them.
- Nx artifacts older than 12 h: 141 entries, about 0.109 GiB.
- Old Bun transpiler cache files under `nx/tmp/node-compile-cache` older than 12 h: 17884 files, about 0.713 GiB. They are regenerated on demand.
- Zero-byte `.hm` stamp files older than 12 h under `nx/tmp`: negligible.
- Total about 0.82 GiB. Free space moved from 6.3 to 9.1 GiB, but that also includes the concurrent GC.
- No build unit was touched (free space was above 3 GiB). No root was deleted. No process was killed.

## Blocker found (now fixed): recovery commands could not run
- `readNativeRepoRecoveryPlanV1` reads the owner manifest with `maximumOwnerBytes` 65536 (CLI `🧬️schema/🔣️.json`). `⚡️caching/📋️project.json` had grown to 105717 bytes (51245 on 10-23, 73879 on 10-25). Every reset or retire target was refused with "Bounded physical Source revision required". Reproduced with the read-only probe `C/1029-tools/diskcap/plan.ts`.
- Coordinator decision: never raise `maximumOwnerBytes`. Split instead.

## Owner split (done, limit untouched at 65536)
Two small owner projects now sit next to the code they run. The six recovery targets left `⚡️caching/📋️project.json` (now 102035 bytes; it is no longer a recovery owner, so the bound does not apply to it).

| Project (name) | Manifest | Bytes | Targets (old id → new id) |
|---|---|---|---|
| `repo-cache-reset` | `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📋️project.json` | 3837 | `repo:cache-reset-graph` → `repo-cache-reset:graph`, `…-nx` → `:nx`, new `:rust`, new `:products` |
| `repo-cache-retire` | `⚡️caching/🧹️pruning/🏪️budget/📋️project.json` | 1940 | `repo:cache-retire-inactive-preview` → `repo-cache-retire:inactive-preview`, `repo:cache-retire-inactive` → `repo-cache-retire:inactive` |

- Target names were renamed, not kept, because `repo:cache-reset-rust` under the new project would stutter. The Nx project name had to change because `repo` is the big caching project. Usage is now `repo run repo-cache-reset:rust` etc.
- `rust` and `products` reset targets were declared by the CLI protocol and the schema description since 10-25 but never existed in a project manifest. They exist now, with the same shape as graph and nx.
- The script commands are unchanged (`repo ./📜️script.ts reset <profile>`, `repo ./📜️script.ts cache-retire-inactive <preview|retire>`), and so are their script routers and cwd.
- `inputs`: reset targets use `{projectRoot}/**/*`. Retire targets keep `default` and `reads`, with `namedInputs.reads` copied into the new project.

References updated (rg of the old ids, no hit remains outside tickets):
- CLI protocol and schema `🏗️build/🚀️bootstrap/⌨️cli/🧬️schema/🔣️.json`: `maintenance` and `reset` project, owner, scriptRoot and targets; `MaintenanceOwner` and `ResetOwner` (name const, four reset targets, new inputs const); `RecoveryCaptureCase` (three documents, `maintenance` kind, `preview` target).
- CLI tests and fixtures: `🧫️fixtures/🔣️.json` (about 100 ids; the capture documents are now reset, retire and mcp manifests; new rows for the rust and products profiles and requests), `🧪️tests/🟦️.ts` (SQL oracles now cover rust and products; the maintenance branch of the capture check; target keys), `🧪️tests/🧮️arguments/` (JSON rows, Python oracle returns the real reset target instead of a hard-coded `nx`, schema enums), `🧪️tests/🥒️.feature`.
- Pre-graph runtime data companions (the CLI module reads its owner manifests at run time): `⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json` `RuntimeDataCompanions`, plus `🧪️tests/🧫️fixtures/🔣️.json` (`uniquePaths` 45 → 46) and `🧪️tests/📥️data/🔣️.json`.
- Pruning tests: `🧹️pruning/🏪️budget/🧪️tests/` read the new local manifest; fixture target ids and the feature table renamed.
- Messages and docs: `…/🏪️store/📇️format/🟦️.ts` (two refusal messages now say `repo-cache-reset:graph`), `…/📦️budget/🧬️schema/🔣️.json` (`automatic` description).
- Registry: `📋️build-owners.json` gained the two rows via the repo's own `reconcileBuildOwnerRegistry` with scope `⚡️caching` (dry run first; findings: none). Probe: `C/1029-tools/diskcap/owners.ts`.

## Verification
- Read-only probe `repo C/1029-tools/diskcap/plan.ts`: `repo-cache-reset:{graph,nx,rust,products}` and `repo-cache-retire:{inactive-preview,inactive}` are admitted and return the right kind, target and owner directory (the manifests also validate against `ResetOwner` and `MaintenanceOwner`). The old ids, `repo-cache-reset:dependencies` and foreign projects return null.
- Pre-graph companions: probe `C/1029-tools/diskcap/companions.ts` captures the CLI module through the real authorizer capture. All three declared manifests (mcp, reset, retire) are captured, and the CLI fixtures file and the old caching manifest are not. Probe `C/1029-tools/diskcap/capture.ts` checks the three capture documents against `ResetOwner` / `MaintenanceOwner` / `Owner` and the plan kind and target: all true.
- Nx accepted the new projects (graph of 571 projects planned) and `📋️build-owners.json` is consistent (reconcile findings: none).
- `repo run repo:test-cargo-retirement` (reads the new local manifest): GREEN, 12 pass, 0 fail.
- `repo run repo:test-build-command-line` (second run after fixing my own test breakages): the CLI file `🚀️bootstrap/⌨️cli/🧪️tests/🟦️.ts` is 8 pass, 1 fail. The one failure is `capacity-admitted recovery …` with "Cannot find module './🏛️sealed/📥️execution/🟦️.ts'" from the test file. That import is in `🏗️build/📜️script.ts` and the failure comes from the test's own loader, not from my change. Because this test aborts before its capture block, I ran that block's checks separately with `capture.ts` (above). The other four failing files of the suite (`⌨️command` ingress with "capabilities-differ", `🚪️door` startup, two `🧭️invocation` files) do not mention recovery ids. I did not touch them and treat them as unrelated peer breakage.
- First-run breakages that were mine and are fixed: the prebirth test iterated bare target names; `ResetCase` / `ResetArgumentsCase` enums lacked rust and products; the pruning test read `test-cargo-retirement` from the new project.
- `repo run repo:test-authorizers` and `repo run repo:test-storage-budget` fail in this environment in tests that do not involve the moved manifests (sandbox "Native storage path escapes enforcing boundary", nx partition provisioning, the first runtime-companion row `📋️data/🟨️.mjs`). I did not chase them. My companion rows are verified by `companions.ts` instead.
- No cache reset was run.

## Recommended action for the coordinator (grow to 60 GiB)
Only do this if a cold rust cache is acceptable. Everything cargo or wasm has to rebuild, then the 32-component activation runs on the larger image.
1. The recovery owners are fixed (above); no limit approval is needed.
2. Quiesce: start no new `repo cargo` or `repo run`, and let running cargos finish.
3. Run `repo run repo-cache-reset:rust`. It takes a queued exclusive maintenance wait on the image, so new commands queue behind it. It is bounded to 600 s (`maximumMilliseconds`), so all cargos must end within about 10 min. It prints `retiredImages:1`.
4. The next `repo cargo` creates the image at 60 GiB (sparse birth). Check `image.json` `maximumPhysicalBytes` = 64424509440 and `df` on the mount.
5. If you do not want to lose the warm cache: keep 48 GiB. During activation limit concurrent cargos to about 4–5 (the sawtooth minimum correlates with the number of concurrent compiles) and keep the host floor above 40 GiB. Do NOT run `hdiutil resize` or `diskutil apfs resizeContainer` on the live image. The receipt, journal allocation sums and the leases pin the old size.

## Files
- Created: `C/1029-tools/diskcap/{plan.ts,owners.ts,verify.sh}`, the two owner manifests, this file.
- Edited: see "References updated".
