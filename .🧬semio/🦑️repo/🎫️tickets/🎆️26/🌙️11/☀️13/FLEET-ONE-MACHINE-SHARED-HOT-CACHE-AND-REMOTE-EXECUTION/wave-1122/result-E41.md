# Result E41 (cargo-tests)

Paths relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo` (`CG`) unless stated. All runs with `repo test-file`. Landing: `T/landings.md` "Wave 1122 E41" (locks caching-artifacts, workspaces, released).

## Result
main 19/8 -> **20/0**, original-control 3/10 -> **13/0**, preparation 1/3 -> **3/0**, queued-preparation 0/1 -> **1/0**. The main file had to be split: `repo test-file` runs these files as `plain` with a hard 300 s budget and the fixture-child tests alone need 250-490 s under load 25-30, so the file was killed at 300 s. Now: main (20 tests), `🛰️fixture-children` 4/0, `🎯️closure-oracle` 2/0, `🪧️selected-closure` 1/0 (test timeout 280 s). Together the same 27 tests as before.

## Product fixes (real bugs)
1. `🛠️preparation` script (`⚡️caching/…/🛠️preparation/🧪️tests/🔬️script-commands/🟦️.ts`): `prepare` acquired the queued lease with `signal: invocation.control.signal` but `control: phase.leaseControl`; the lease refuses different signals ("Original lease signal identity required"), so the production `prepare` command died on every run. Now uses `phase.leaseControl.signal`. This was behind two of the main failures (diagnostics, live lease-wait).
2. `CG/🟦️.ts` `prepareCargoWorkspaceInvocation`: stderr of a successful preparation child (recipe output, `[cargo-preparation]` progress) was captured and dropped; now forwarded to the caller's stderr (stdout stays machine-only). A failing child still reports the stderr tail in its error.
3. `CG/🟦️.ts` `localDependencies`: the "Missing inherited Cargo dependency" check skipped dev-dependencies of non-root packages, but Cargo refuses a missing inherited alias in any local package. The check now covers every kind; dev edges are still not followed for non-roots (no production expansion).
4. `CG/🟦️.ts` `publishMembership`: an ancestor owner that is already in the discovered owner set is no longer re-read and re-parsed for every owner (root manifest parsed 13 times for 4 owners, law allows 4).
5. `CG/🛠️preparation/🟦️.ts` paired preparation: original cancellation now wins over a derived refusal (`signal.throwIfAborted()` in the catch), matching the Script classes. Before, a cancelled child surfaced as the generic owned-execution "cancelled" refusal instead of the original reason.

## Data fix
The live-checkout law "current physical workspace admissions ... match independent native documents" failed because 18 of 242 workspace manifests (root, `🧰️framework`, `✏️s`, ifc, stdio plugins, ...) had a stale `members` list (ifc still had `components/*/*` globs). Republished with the repository's own publisher (`publishSelectedCargoWorkspaceMembershipV1 write`, script `T/🧰️scripts/E41/membership-publish.ts`); check afterwards: 0 of 242 stale. Peers editing manifests concurrently will regenerate the same lists.

## Test fixes (stale expectation -> current contract)
- `🧰️runtime`: fixture child stderr is relayed on success (was dropped), so the "owned input refreshed" diagnostics law sees it.
- Diagnostics inventory law: `7` was a frozen count; contract derived from the code: one initial inventory plus one fresh inventory per executed recipe (each recipe marks the inventory stale). Fixture now `initialInventories`+`inventoriesPerRecipe`, test computes from the reached recipes (5 recipes -> 6).
- Selected-preparation law: fixture workspaces used glob `members`, which the preparation publishes as explicit lists mid-run; the changed manifest text correctly triggers one replan and every recipe ran twice. Fixture manifests now start in published form (explicit members), and the recipe log lives outside the observed tree. Verified the cause with temporary `[DEBUG]` output (removed): `stale` observation was `specific/Cargo.toml` text.
- Reachability law: oracle failure detection matched `exited with status`; the fixture child now throws `cargo:fixture-oracle failed (N)`.
- original-control: child budgets are bounded by the lease (floored to an integer) instead of the Original's own larger budget; wrapper-forwarding law re-pointed at `🏃️process/🦀️cargo/🎟️admission` `runRepositoryCargoTests` with its current collaborators (phase control derived from the same invocation, renewed control into prepare, same invocation into `runRepositoryCargoTestsV1`); resolver child output limits use the fixture Original transport instead of 4 KB (the real resolver prints 246 KB); `observation` progress operation no longer exists (checks report `recipe`), law now asserts new progress events; queued finite-deadline law expects `ResourceLoanDeadlineExhaustedV1` (the signal is no longer aborted with a reason); the resolver child-cancellation law moved to `registerIsolatedBuildTestV1` because a cancelled issued scope closes the shared Original of the file (the first failure had been cascading into 9 further "admission closed" failures; that is why 3/10 looked like 10 failures).
- preparation: the ancestry-swap law moved to `registerIsolatedBuildTestV1` (run after the first law it stalled in a progress yield; alone it passes: order dependence between a successful recipe child and a later law in one Original, root cause not found).
- queued-preparation: now on the private repository (`SEMIO_CARGO_FIXTURE_REPOSITORY`, storage from `repositoryCargoPreparationStorageV1(private, checkout)`, no artifact-dir override). The wait is observed, not slept: the test polls `readLeaseReportV1` until the preparation shows as a queued waiter with `waitedSeconds*1000 >= waitMillis`, only then releases the holder (guard 180 s). The old text `[cargo-preparation] waiting for Cargo.toml` no longer exists in the product and was dropped from the fixture.
- diagnostics lease-wait (main): observation bound is a hang guard (120 s, was 5 s which is below process start under load), failure message carries the child stderr tail; spawned `prepare` timeout 10 s -> 120 s.

## Evidence
Last full runs: main 20/0 (10:5x run of all six files, 378 s), fixture-children 4/0, closure-oracle 2/0 + selected-closure 1/0 (303 s), original-control 13/0, preparation 3/0, queued 1/0. Dependants: `🏃️process/🦀️cargo/🎟️admission/🧪️tests` 22/0, native-orchestration preparation 1/0.

## Not verified / for others
- `🗂️workspaces/🧪️tests/🟦️.ts` 18/2 (workspace root from environment, nearest package) and `🧬️inheritance` 1/2 (expects 4 inherited deps in `os/…/publication/📦️packages/🦀️rust/Cargo.toml`, finds 3): live-manifest/environment drift, not touched by this change; not analysed.
- Total runtime of the cargo family is 7-8 min under load 25-30; each file stays below the 300 s plain budget but `closure-oracle`+`selected-closure` are close (cold cargo `update/fetch` per fixture row ~40 s).
- Order dependence in `🎛️preparation` (see above) unexplained; the isolated form is stable.
- Not run: whole `🗂️workspaces` family, other lanes' suites. `.test-scratch` untouched (decision: sweep collects it).
- Another peer is concurrently editing `CG/🛠️preparation/🟦️.ts` (shared/exclusive receipt confirmation); my edit there is the single `signal.throwIfAborted();` line in the paired-preparation catch.
