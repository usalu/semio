# Infra Unblock Result (2026-10-15)

Status: three Source defects fixed and tested, the finite-deadline contract identified and wired into the launch routes, canonical commands written down. A real compile did not complete: the rust image is 1,744,896 bytes (about 1.7 MB) short of the 4 GiB initial-headroom gate, and the graph image sits at its threshold. Both gates are intended refusals. Neither was bypassed or weakened.

Logs are in `🪓️monolith-split/🗑️generated/infra-*.log`.

## 1. "Finite original maintenance authority required"

- Where: `⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts` (`capture`, `control.remainingMilliseconds()===null`).
- Verdict: **intended refusal for an unbounded caller, not a defect.** It only fires on the pressure path.
  - Chain: `withRepositoryCompilerStoreV1` -> `withCompilerWorkspaceWriteV1` -> `withCompilerInitialHeadroomV1`. If the image has less than 4 GiB available, it calls `recoverCompactCompilerHeadroomV1`, then `retireInactiveCargoStoreV1`, whose `ps`/`lsof` captures need a finite budget.
  - The 16 GiB rust image is currently the pressure case.
  - With no `SEMIO_SCRIPT_POLICY`, `runBuildCliV1` (`🏗️build/📜️script.ts`) defaults to `maximumElapsedMilliseconds:0`, which is unbounded.
  - The cargo branch passes `maximumMilliseconds:0`, which keeps the invocation deadline.
- Correct caller path: author a finite `SEMIO_SCRIPT_POLICY`. This is the pattern in the peer notes (`sdk-fresh-publication-plan-current.md`) and in every authored launch entry.
- Wired in: `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, entries "Repo Cargo prototype" and "Repo Cargo release". Each now has `env.SEMIO_SCRIPT_POLICY={"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}`. launch.json still parses as JSON.
- Verified: with the policy, the run got past this error and performed real bounded recovery, deleting about 100 inactive compact units (about 440 MB logical) under the writer, store and physical locks.

## 2. Defects found behind it

### 2a. lsof escapes break the descriptor custody check (fixed)

- Symptom: `Physical compiler guard is absent from the complete descriptor inventory`.
- Cause: `lsof -F` prints non-ASCII bytes as `\xNN`. The store root `.🧬semio/🦑️repo/…` contains emoji, so the parsed paths never matched the guard's real paths.
  - As a result, the foreign-descriptor check silently skipped every row, and the guard check always failed.
- Fix: `decodeLsofPathV1` in `⚡️caching/🧹️pruning/📉️pressure/📂️descriptors/🟦️.ts`. It is exported and applied to each `n` record.
- Test: new `lsof escaped non-ASCII store roots preserve exact physical guard custody` in `…/📉️pressure/🧪️tests/🟦️.ts`.
  - It escapes the path byte-wise (independent oracle), accepts the exact guard, refuses a foreign descriptor and refuses an absent guard.
  - RED shown by temporarily reverting the decode. GREEN after the fix. The existing nine-case SQLite corpus still passes.
  - Command: `bun test ./<path> -t "lsof escaped|descriptor"` (the `./` prefix is required).

### 2b. Execution-contract validator lacked `enum` (fixed; this was `Repository target execution contract refused`)

- Cause: `conformsOwnerContractV1` in `🏗️build/📤️arguments/🟨️.mjs` did not implement `enum`.
  - The Native execution schema declares `scope` as `{enum:[production,test]}`. Every native Cargo target (`cargoTargets`) was therefore refused, for every Rust project, including the untouched `@semio-tech/repo-graphql-fs-rs:check`.
- Fix: added `enum` (and `uniqueItems`) to the validator.
- Tests: the existing Ajv-oracle tests in `🏗️build/🏃️execution/🧪️tests/🟦️.ts` were RED and are now GREEN.
  - `closed source and native execution declarations match the independent schema oracle`: 12 cases.
  - `generated Cargo commands declare compilation versus development-test ownership`.
  - `🏗️build/📤️arguments/🧪️tests/🟦️.ts`: 16 pass / 0 fail with `SEMIO_TEST_ARTIFACT_DIR` set.
- The earlier `Cannot find module '../../../📚️layers/🟦️.ts'` is not reproducible: the module now resolves.

## 3. `bun repo dependencies sync`

- "Live Original process issuance required" did not reproduce.
- The real usage error is that the command needs a selector: `bun repo dependencies sync --all` (or `--scope <owner>`).
- With `--all` it runs about 2 minutes (1:50 to 2:20), then aborts on `Bun current ownership changed during verification` (a directory added by a peer, for example `…/🌉️mcp/🏠️workspace/📦️package-assets`).
- That is the optimistic fence in `🗂️workspaces/🟦️bun/🟨️.cjs`. It is intended, and it needs a quiet tree. Run it in the coordinator's freeze window.
- It is not needed for Rust compilation.

## 4. Offline downloads

- Sanctioned route: `bun repo cargo --store=shared --configuration=prototype -- fetch --locked`.
  - Network is allowed by default: `CARGO_NET_OFFLINE` defaults to `false` in `📥️caller/🟦️.ts`.
  - Only explicit `-- … --offline` runs, and launch entries that author `network.offline:true`, are offline.
- Observation: the compiler store's `CARGO_HOME` (`rust/fixed/m/cargo/home`) already holds all 706 registry crates of the root `Cargo.lock`. `http-body-util-0.1.5.crate` is present; it was fetched at about 11:49 by a peer. The root lock needs no download now.
- I did not run the fetch. It is a cold-writer operation, so the same 4 GiB gate (section 5) applies, and the download would also eat the headroom.
- The nested lockfiles (`✏️s/*/Cargo.lock`, `🌎️hub`, `🎓️teaching`) were not checked.

## 5. Remaining blockers (intended, state-dependent)

| Blocker | Measured | Needed | Notes |
| --- | --- | --- | --- |
| Rust image initial headroom | 4,293,222,400 B available | 4,294,967,296 B | Short by 1,744,896 B. All inactive compact units are already retired. The remaining 3 units (about 27.9 MB) hold stale `-working` incremental sessions from killed builds (07:16, 11:56, 14:45), which are protected by design. |
| Graph image (Nx pool admission) | 1,067,xxx,xxx to 1,075,847,168 B | 1,074,790,400 B | It oscillates around the threshold. Every graph load costs roughly 7 to 10 MB. Recovery (`recoverBuildGraphImageHeadroomV1`) retires authenticated snapshots oldest-first only under non-waiting exclusive maintenance. While any peer `bun repo` run holds the graph image, it returns "Active graph readers retain required Nx pool headroom". |

- Rust image, sanctioned fix: the peer-owned historical retirement (`⚖️cargo-retirement-historical-preview|retire prototype` in launch.json). It has retained about 3.6 GB of authenticated historical logical bytes, and Root ran it successfully at 13:29.
  - It runs through the private headroom Nx workspace. It currently fails with `NX Original funded graph publication namespace required`. That comes from the installed Nx SDK (`…/b/t/m/nx/4ee0fbd…`), which predates Source while the peers' SDK republication is in flight (ABSTRACT-REPO-BUILD-TOOLS ticket, `sdk-current97/99`).
  - Retry it after the new SDK is published (`bun repo tools setup nx`, per `sdk-fresh-publication-plan-current.md`).
  - I did not retire historical data myself. That authority is Root's, and I did not touch the SDK.
- Design finding for the owners: killed builds leave dead `-working` sessions that the compact recovery will never reclaim.
  - Provably dead sessions could be reclaimed once the exclusive profile lock and the complete lsof census show nobody holds the profile. That would be a deliberate policy change in peer-owned pruning code, so I did not make it.
  - The 16 GiB image also cannot hold two 4 GiB compiler charges, the Nx tmp/graph data and the cargo home. The headroom gate therefore sits permanently at its edge.
- `Original selected owner registry changed`: caused by the coordinator's own carving agents, per the coordinator. It is not a defect.

## 6. How far runs got

| Route | Outcome |
| --- | --- |
| `bun repo cargo … check -p semio-framework-compiler` with policy | Passed the finite-deadline gate, ran compact recovery, then `Insufficient physical compiler initial headroom` (section 5). |
| `bun repo run @semio-tech/quiz-schema-rs:check …` with policy | Passed the execution-contract refusal and the Nx tool provisioning (it retired a released tooling generation and planned a derivative). Then it stopped at the graph pool gate or at registry churn. In a quiet moment (attempt 13) it got past the graph pool gate and reached `loadBuildGraphAttemptV1`, which then refused with `Original selected owner registry changed`. |
| `bun repo run` plist error | One transient `System image plist required` (empty `hdiutil info -plist` output while peers were attaching images). It did not recur. |

No real compile finished, so nothing is claimed beyond this. Peer processes also run raw `bun repo cargo` with no policy, and they will hit the same refusal once the pressure path triggers.

## 7. Canonical commands for the quiet window

From `/Users/ueli/Documents/semio`. macOS has no `timeout`; cap long commands with `perl -e 'alarm N; exec @ARGV' --`.

1. Set the finite policy and run the sanctioned launcher:
   ```sh
   export SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}'
   perl -e 'alarm 3700; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-compiler --message-format=short
   perl -e 'alarm 3700; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-quiz-schema --message-format=short
   perl -e 'alarm 3700; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-repo-cli-event-stream --message-format=short
   perl -e 'alarm 3700; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype -- test -p semio-framework-compiler --lib
   ```
   - The launch.json "Repo Cargo prototype|release" entries now carry the policy automatically.
   - The `--tests` and release variants are the same with `--tests` or `--configuration=release`.
   - Without the policy the command works whenever the image already has at least 4 GiB free, and fails with the finite-authority error otherwise.
2. Nx route, same policy (the owner is free text; `repository:build` also works):
   ```sh
   SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:build","maximumElapsedMilliseconds":3600000}' bun repo run @semio-tech/quiz-schema-rs:check --configuration=prototype --parallel=1 -- --lib --offline
   ```
   - On "Active graph readers retain required Nx pool headroom", retry in a loop with `sleep 20`. It clears when no peer run holds the graph image.
3. Dependency sync: `bun repo dependencies sync --all`, only in a tree-quiet moment.
4. Network fetch (if ever needed): `bun repo cargo --store=shared --configuration=prototype -- fetch --locked`.
5. Before the first compile, check headroom:
   ```sh
   python3 -c 'import os;s=os.statvfs("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m");print(s.f_bavail*s.f_frsize)'
   ```
   It must be at least 4294967296. If not, run `⚖️cargo-retirement-historical-preview prototype` and then `retire` from launch.json once the Nx SDK is current. Root must authorize the retire.

## Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📉️pressure/📂️descriptors/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📉️pressure/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📤️arguments/🟨️.mjs`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`

Temporary debug edits were reverted. Peer-owned `📜️script.ts` and guards are untouched.
