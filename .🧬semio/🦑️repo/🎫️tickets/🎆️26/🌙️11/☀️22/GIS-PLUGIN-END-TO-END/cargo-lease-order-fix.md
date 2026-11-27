# Cargo lock resolution: lease order, capacity admission and starvation

## Cause

1. Lock order violation. `repo cargo -- update|generate-lockfile` (request kind `lock`) runs through `withRepositoryCargoResolutionV1` in `📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts`. `withRepositoryCargoResolutionOwnedV1` took the registry lease (cargo-coordination, rank 2) and then wrapped the WHOLE body in `withCompilerWorkspaceSharedWriteV1`, i.e. the shared `repo:compiler-writer:` custody (image, rank 7). The launcher body (`resolved` in `🚪️launcher/🟦️.ts`) then called `acquireCargoBuildLeaseV1`, which requests the Cargo store lease and the `semio.cargo.custody-lease/v1` custody (rank 2) while the rank-7 writer is held: the `[lease-order]` line. A resolution never compiles into a build directory, so it needs no store/profile/custody lease at all. (Compiles and `resolve` commands were already correct: their writer custody wraps only the fetch, or is offered to the consumer after its Cargo leases.)
2. Starvation (timeouts, exit 124). The evidence logs show two separate waits, no deadlock cycle:
   - Registry: the lock path took `cargo:registry:<home>` EXCLUSIVE even offline. The queue shows `position 3 of 4, behind the request of process 40649 (shared, queued 856 s ago)`: an exclusive request waits for every shared compile reader, and a long reader (a whole compile holds it from planning to consumption) starves it past the 600 or 900 s timeout. An offline resolution writes no registry index, so it needs no exclusivity.
   - Capacity: every `repo cargo` asks the machine-wide gate for the `cargo` class (512 MB) and then `cargo-orchestration` (1536 MB reserved). On the 16 GB host with graph-plan and compile runs ahead of it, a lockfile refresh waited 166 s to 444 s (`not enough free memory`, `this agent session already holds its share`). A resolve-only update uses a few dozen MB and compiles nothing.

## Fix

- `📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts`: the lock body is now `resolution` (executes cargo, no build lease) and `resolved` (resolve kind) wraps `resolution` with `acquireCargoBuildLeaseV1` as before. `withRepositoryCargoResolutionV1(inputs,resolution)`. Under the writer custody the body now requests nothing of a lower rank.
- `📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts` (`withRepositoryCargoResolutionOwnedV1`): registry lease is a shared cohort reader when the fetch arguments are offline (`--offline`, `--frozen` or the offline grant), exclusive otherwise (an online update may write the index); the shared writer custody stays inside it. Docstring states the invariant.
- `📚️library/🏗️build/🌡️host-capacity/🟦️.ts`: new `lightCargoResolutionV1(args)` (subcommands read from the launcher schema `Configuration.const.subcommands.lock`: `update`, `generate-lockfile`, the first word after `--`), used by `buildCapacityAskV1`, which already answers undefined for light commands. No cargo ask, no cargo-orchestration reservation, no disk admission for them. Compiling and fetching/metadata subcommands keep their ask.

## Tests (run with `repo test-file <file> --lang=en`)

- `📚️library/🏗️build/🌡️host-capacity/🧪️tests/🪶️light-cargo/🟦️.ts` (new): lock subcommands ask nothing, others keep `cargo`/`run`; Python oracle agrees; `withBuildCapacityV1` runs the body with no handle, no output and no disk admission. 2 pass.
- `📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/🔒️fetch-order/🟦️.ts` (existing suite, 2 tests added): AST proof that the launcher lock body has no `acquireCargoBuildLeaseV1` and the resolve body has exactly one; behaviour test of `withRepositoryCargoResolutionOwnedV1` (offline: shared cohort, online: exclusive, order registry, writer, body, writer release, registry release). 4 pass.
- `🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🪜️order/🔣️.json` (existing lease-order suite, 2 cases added, real leases under `SEMIO_LEASE_ORDER=enforce` plus the Python policy oracle): `lock-resolution` (registry, writer) granted; `build-custody-under-resolution-writer` (registry, writer, build) refused with a lease-order error. 1 pass.
- Regression runs: host-capacity `🧪️tests/🟦️.ts` 19 pass, `🦀️cargo-scaling` 16 pass, `🚦️native-cargo-grant` 4 pass. `🚪️launcher/🧪️tests/🟦️.ts`: 10 pass, 2 fail on `lstatSync(artifacts).dev===lstatSync(bounded).dev` against `fixture.diagnostics.sameFilesystem` (a filesystem device layout assertion on this host, in the isolated-store sweep test; not touched by this change, not investigated further).

## Validation

`🗑️generated/lease-order-validate` is `$T/lease-order-validate.sh`: 4 concurrent `timeout 600 repo cargo --store=shared --configuration=prototype -- update --offline --workspace --manifest-path <plugin>/Cargo.toml` on the first four manifests of `stale-locks.txt` (wfc grid2d, wfc 2d, writer, mathematical equation); logs `🗑️generated/leaseorder-1..4.log`, statuses `leaseorder-status.txt`.

| manifest | exit | wall time |
| --- | --- | --- |
| wfc grid2d | 0 | 14 s |
| wfc 2d | 0 | 15 s |
| writer | 0 | 178 s |
| mathematical equation | 0 | 221 s |

No log contains `[lease-order]` or `Lock order violated`, and none contains a `[capacity]` line (before: every log had one, up to 444 s of capacity waiting). The two slow runs were not lease-order or capacity waits: they sat in `native:preparation-entry` holding `cargo-preparation:<workspace>` (exclusive) while the preparation's `native-storage:` image lease queued behind another process (position 2 of 2); that is ingress preparation contention with other agents' compiles, left as is (follow-up: a lockfile-only invocation probably needs no native preparation either). Before the fix: 4 of 29 runs hit 900 s, others up to 160 s.

## Not changed

`resolve` commands (`metadata`, `tree`, `fetch`, ...) keep their capacity ask and build lease (their writer custody wraps only the fetch, so no violation); they could be given the same light class later. Online `update` keeps an exclusive registry lease.

## Follow-up: native preparation gate and launcher suite

### Native preparation skipped for offline lockfile-only runs

The wait on `cargo-preparation:<workspace>` (exclusive) was the `synchronize` child that `prepareCargoWorkspaceInvocation` (`🗂️workspaces/🦀️cargo/🟦️.ts`, called from the launcher as `cargo.body.prepare`) spawns for every command in the schema list `🧬️schema/🏃️invocation/🔣️.json`; it logs as `native:preparation-entry`. The gate is `cargoWorkspacePreparationInvocationV1`: it now also returns no request when the new `cargoLockOnlyInvocationV1(args)` holds (command in the launcher schema's lock subcommands, `update` or `generate-lockfile`, with `--offline` or `--frozen`). Online lock commands and every other command keep their preparation (the lockfile refresh itself resolves the manifests, so no owner input is needed offline).
Test: `🗂️workspaces/🦀️cargo/🧪️tests/🪶️lock-only/🟦️.ts` (new, 2 pass): vectors against a Python oracle, plus a real repository-marked workspace where `cargoWorkspacePreparationInvocationV1` selects no preparation for the two offline lock vectors and selects one for online lock, build, metadata and fetch.

### Launcher suite: the assertion and the fixture were wrong, not the code

`🚪️launcher/🧪️tests/🟦️.ts`, test "isolated Cargo stores are removed ...". Three test defects, all host-independent (the real `.cache/b/o/m` APFS volume and the private repository's own `o` image are distinct devices by design; the NFS `/Volumes/semio-store` is not involved):
1. `bounded` was `join(process.cwd(), ".cache/b/o/m/tests")`, but the artifacts live under the private repository root (`.cache/test-sandbox/<id>/.cache/b/o/m/tests/...`) whenever the runner's cwd is the real checkout, so `relative(bounded, artifacts)` escaped with `..` (that was the failing assertion at column 447, before the `dev` comparison was even reached). Now `join(repository.root, ...)`, which is the same path when the cwd is the private root.
2. The fixture image directory was created inside `artifacts`, i.e. inside the already open persistent `o` image of the test invocation, which `openNativeImageStorageV1` rightly refuses ("Nested persistent image requires actual parent scope"). It now uses `repository.scratch("cl-")`, outside every image.
3. The final check counted every entry of the isolated directory, but the lease implementation keeps one `<hash>.sqlite` lease database per resource there. It now counts only store directories, which is what the law (every isolated store is removed or swept) is about.
Result: 11 pass, 0 fail (was 10 pass, 2 fail); the sweep and removal code needed no change.

### Re-validation (4 concurrent, `timeout 600`, `$T/lease-order-validate.sh`)

| manifest | exit | wall time (before follow-up) | wall time (after) |
| --- | --- | --- | --- |
| wfc grid2d | 0 | 14 s | 10 s |
| wfc 2d | 0 | 15 s | 10 s |
| writer | 0 | 178 s | 11 s |
| mathematical equation | 0 | 221 s | 11 s |

No log contains `[lease-order]`, `Lock order violated`, `preparation-entry` or a `[capacity]` line.

## Follow-up: a long build or test no longer blocks a resolve

Finding: `repo leases status` shows the long `cargo test` (pid 11170 under bun 3514, 40+ minutes) holding `cargo:registry:<home>` SHARED (cargo-coordination, rank 2), which is correct for a reader (`withRepositoryCargoRegistryReadyV1` consumes it as a shared cohort reader; the exclusive lease of a build is only the short `cargo:registry-fetch:` one around its fetch). The blocked party was the other side: `withRepositoryCargoResolutionOwnedV1` still requested `cargo:registry:<home>` EXCLUSIVE for every ONLINE `cargo update` / `generate-lockfile`, and an exclusive request waits for every shared reader, so it queued behind the 40 minute reader (and, through writer preference, other readers queued behind it).
Fix (`🏃️process/🦀️cargo/📥️dependencies/🟦️.ts`): a lock resolution now holds the registry only as a shared cohort reader, offline or online, then enters the shared compiler writer; no exclusive registry lease at all. Cargo's own package-cache lock serializes an online index write and readers re-verify their receipts, as they already do beside a ready fetch. A first variant that added the exclusive `cargo:registry-fetch:` lease around the body was rejected by the lock-order check in the real run (the body's storage recheck requests the rank-7 image mapping while the rank-8 fetch lease is held), so the fetch lease stays with the ready fetch.
Tests (`🔒️fetch-order/🟦️.ts`, 5 pass): the order test now asserts shared cohort, writer inside, never exclusive, for offline and online; the new regression holds a real shared registry reader for the whole test and proves a concurrent offline and a concurrent online resolution complete within the 5 s race instead of blocking.
Real run while pid 11170 was still running (not touched): online `repo cargo --store=shared --configuration=prototype -- update --workspace --manifest-path <writer>/Cargo.toml` exit 0 in 50 s with no `[lease-order]` line (`🗑️generated/online-update.log`); the lease-order suite `🪜️order` still passes.
