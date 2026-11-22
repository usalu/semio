# Result E24 (tests-caching)

## Verdict
Of the 138 files under `⚡️caching` that E19's table listed as failing, a fresh single-file run at about 05:25 (suite-runner path `sf.sh`, 126 files in the cluster batch plus 12) gave: 12 already green (vite, compiler phase/layout/ingress, lane, windows: the table was stale), about 125 red. This lane repaired 14 of them for real (all re-run alone afterwards), fixed 3 product or declaration defects, and classified the rest. NOT green yet: 4 files it touched (below), about 110 files in the two big clusters in `findings-E24.md` (live-store fence vs real toolchain/SDK; Original-envelope harness gap). Nothing was skipped or deleted.

## Fixed (file, cause, kind, result)
| file (under `⚡️caching`) | cause | kind | result |
|---|---|---|---|
| `🦀️cargo/📦️units/📥️inputs/🧪️tests` | message changed to `process.source.read-bytes-exceeded` / `read-files-exhausted`; wrote into another ticket's generated folder with root = checkout | stale expectation + hermetic dir | 3/0 |
| `🦀️cargo/📦️units/📋️plan/🔏️identity/🧪️tests` (json + schema) | plan canonical bytes changed (unit key covers every source file): goldens 11400/11340 -> 11190/11130 (sha recomputed by the product, size drop identical in both modes) | stale golden | 3/0 |
| `🦀️cargo/🗃️unit-cache/🧵️lane` + `🧹️gc/🧪️tests/🫀️live-holder` | gc listed the reserved lane `d` and could remove it | product bug | 2/0 (pool-claims 7/0, gc 14/0) |
| `🧹️pruning/📉️pressure` + root `🧬️schema/🔣️.json` | root schema still required `storage.cargo.budgetBytes` (moved to the cache ledger) | stale schema | 4/0 |
| `💾️storage/.../🧾️closures/🧪️tests/📁️census` | refusal messages carry no name any more | stale expectation | 3/0 |
| `💾️storage/.../🔌️nx/📦️outputs/🧪️tests/📋️registration` | test targets are uncached; suite table replaced the script command table | stale expectation | 2/0 |
| `🏗️build/🧬️schema/📥️authority.json` + `🔏️authority-seal` | `global` kind missing in the authority schema | product schema | 4/0 |
| `📦️artifacts/💾️storage/🧪️tests` | dotnet commands moved to `🧪️test/🧪️tests/🔬️dotnet-package-commands`; wasm/build outputs became `{projectRoot}/.🧾️store/*.json` | stale expectation | 15/0 |
| authorizers `📥️data`, `📥️data/📥️engine`, `📏️capacity`, `📈️growth` (+ authorizers schema, B workspace-scale schema) | schema bound 2048 -> 16384; fixtures used the checkout as root with an artifact dir outside it; 3 new ceilings exempted with en/de reasons | stale expectation + hermetic root + registry | 5/0, 1/0, 10/0, 11/0 |
| `💾️storage/.../⚙️setup/🧪️tests/⏳️busy-funding` (ps) | pid-reuse check compares the registered start with the OS start; test registered `startedAtMs:1` | stale expectation, bounded (3 s, no load dependence) | 23/0 |
| `🕸️graph/✅️verification/🧪️tests/🔣️.json` | `relativeScriptInputs` 5 -> 6 parameters | stale roster | partly (see below) |

## Priority 1: live-store pollution
- `🧹️pruning/.../📏️recensus`: now on `privateRepositoryWorkspaceV1().root`; the live store is no longer reached. Single test still times out at 120 s (expected refusal poisons the Original; finding 4).
- `📦️artifacts/🏗️native-build/📥️published`: now on the private workspace with `repository.invoke` and an owner/budget option; no live reach, but the private repository has no pinned Rust toolchain (finding 1).
- `🏗️build/🚀️bootstrap/📦️parsers` is under B (not touched).
- The fence reveals 12 more files that need the real toolchain/SDK (finding 1): that is a design decision, not a test edit.

## Still red among touched files
`📏️recensus` (timeout), `📥️published` (toolchain), `💾️storage/.../🧪️tests/🕸️units` (4 of 12 pass: 2 stale messages fixed, the live-store ones remain; now 5 pass/7 fail), `🕸️graph/✅️verification` (declaration `targetPolicy` 3 vs runtime 4, B graph lock held by E25).

## Not verified
Whole `⚡️caching` tree not re-run after all edits (only the files above, alone). The about 110 unaddressed files were classified from first errors only. hdiutil/capacity files (`📏️capacity/🧪️tests`, `♻️pressure`) were not made load independent: the first needs a funded-reservation fix, the second needs a bounded mount wait; not attempted. The 69 never-run raw-failing files were run (they are part of the 138 above: 16 had no root/faithful run).

## Other lanes must know
`findings-E24.md`: fence/toolchain decision (storage owner), `targetPolicy` d.mts (graph/E25), authorizers edits under E25's lock, new exempt rows (workspace-scale). Scripts: `T/🧰️scripts/E24/run-list.py` (batch runner over `sf.sh`, 3 parallel), `golden.ts`, `pressure.ts`, `holders.ts`. Generated logs deleted.

# Round 2 (coordinator decisions: read-only adoption, load independence, `repo test-file`)

## 1. Read-only adoption of the installed toolchain and SDK: module and tests landed, wiring into the sealed Rust generation NOT done
- New member `🧪️testing/🧫️private-repository/🧰️adoption` (schema first: `🧬️schema/🔣️.json` Policy and Adoption definitions, en/de refusal texts; `🟦️.ts` `adoptInstalledToolV1(checkout,kind,language)` and `requireAdoptedReadOnlyV1(path,operation,language)`). It finds the installed `rust-toolchain`, `nx-toolkit` or `nx-sdk` below the checkout's live tools store by plain reads only (no mount, lease, create or write) and identifies it by content (sha256 of `rustc`; the 64-hex directory name for Nx material).
- Tests `🧰️adoption/🧪️tests/🟦️.ts`, 5 pass (bare `bun test` from the checkout root, 0.2 s): closed schema rows; the Rust identity equals an independent `shasum -a 256` of the installed rustc; write, lease, mount and remove on adopted material are refused in English and German while a read is admitted, and the live directory mtime is unchanged; the live-store fence still refuses an image path of the live store; an absent kind is refused in both languages.
- NOT done: making `📏️recensus`, `📥️published`, `🕸️units` and the other toolchain-fenced files green with it. The blocker is structural: `withRepositoryRustToolchainOwnedV1` (`🚀️bootstrap/🛠️tools/🦀️rustup/📥️installation`) does not only locate a toolchain, it verifies a store-written receipt and seal, takes `rustup:home` leases on the store's image, registers a custody (`readers`, `compilers`, `programDigests` weak maps) that `repositoryRustCompilerIdentityV1` and every compiler-identity consumer require, and provisions by download when absent. A read-only adopted generation needs a new seam in that function: an `AdoptedGeneration` branch (schema-first, behind the adoption policy) that skips receipt publish, lease and download, builds the `RepositoryRustToolchainV1` row from the adopted directory, and registers a read-only custody whose `recheck` re-hashes the adopted rustc. That is a change of the sealed-custody contract (owner of `🦀️rustup`), not a test-harness edit, and I did not land it blind. The private invocation also has no `rust-toolchain.toml` at its code root (`readNativeCallerAuthorityV1(...).codeRoot` = the private root): the harness must write the pin file there. Both are the first two steps of that seam.

## 2. Load independence of `📏️capacity` and `♻️pressure`
- `♻️pressure`: PASSES (46 s via `repo test-file`, 2 rounds, with timestamps showing the 5 s image open and 3 s per round). Causes found: (a) the test budgets (560 s) exceeded the suite row budget (300 s), so the runner refused the Original deadline; budgets now 280 s / 200 s, test timeout 290 s; (b) the first open of the private `diagnostics` partition happened inside the timed body and hung to the 200 s preparation limit; `await repository.prepare("diagnostics")` at module level (outside the Original; nesting is refused) removes it.
- `📏️capacity`: product seam added, test still red. `withCompilerWorkspaceWriteV1(storage,leaseDirectory,control,body,{maximumWaitMilliseconds})` and `WriterRequest.maximumWaitMilliseconds` (schema, 0..1800000, closed) bound the headroom wait that was 30 minutes by policy; the second-writer law now passes `1500` and is refused after 2 s as it expects. The test still fails because the expected refusal of accepted work closes the whole Original ("first failure") and the following laws in the same Original cannot run. Splitting the first law into `registerIsolatedBuildTestV1` with a declared `/initial headroom/` rejection (tried) failed with "Live Original resource control issuance required" at once, so it was reverted; the three laws need separate Originals through the isolated-test registrar with the control issued inside the isolated body. Not finished.

## 3. `repo test-file` (E26) on the red files
Ran 111 red files of round 1 in one batch (2471 s, load 30-37): 4 pass (`🕸️graph/✅️verification` 2/0 now that the declaration was fixed by its owner, authorizers `📏️capacity` 10/0, `🚪️entry-law` 3/0, authorizers main 13/0 in 294 s), 107 fail. The "Original ... required" files are NOT turned green by `test-file` (it gives the suite envelope, not a private-repository caller or an installed SDK): they are the same fenced files as in finding 1 plus the Nx SDK files (`Original immutable SDK required`, `Original bounded Nx namespace required`, `Original captured runtime entry provider required`), i.e. they also need the adoption of the installed SDK (nx-sdk kind of the new module), a second consumer of the same seam. Per-file list of the red set: scratch `red.txt` (111 paths) is in the lane scripts note below; clusters by suite id: storage-opening(-processes) 11, tools-sdk-processes 5, bootstrap-dependencies 5, import-edges 4, cargo-units-processes 4, bootstrap-processes 4, artifacts-processes 4, storage-nx(-processes) 6, rest 1-3 each.

## 4. Not done in round 2
Wiring of the adoption seam (above); `📏️capacity` isolated laws; the remaining clusters by cluster (only batch-classified: stale-expectation clusters from round 1 such as `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage` and `📦️dependencies/🔌️managers` digest/vector drift were not started). Files edited: adoption member (3 new files), `📏️capacity/🟦️.ts` + schema (bounded wait), `📏️capacity/🧪️tests/🟦️.ts`, `♻️pressure/🟦️.ts` + `🔣️.json`. Locks `caching-storage` taken and released.
