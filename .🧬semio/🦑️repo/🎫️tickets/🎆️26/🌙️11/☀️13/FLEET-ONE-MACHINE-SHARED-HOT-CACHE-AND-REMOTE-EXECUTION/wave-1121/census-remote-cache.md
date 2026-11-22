# Census: Remote Cache Status, Contract Gaps, Bazel Signing, Unit Cache Skips

Read-only audit. No source, git state, or tickets were changed.

Shorthand: `LIB` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `RC` = `LIB/🏗️build/🌐️remote-cache`, `UC` = `LIB/⚡️caching/🦀️cargo/🗃️unit-cache`.

## 1. Why status shows `server`, `store`, `entries` as null

Two separate causes:

- **`server` (remote `/status`) is null because the request is refused, not because the endpoint is down.** `RC/⌨️command/🟦️.ts:174-185` (`askServerV1`) GETs `${endpoint}/status` and returns `null` on any non-200 or any throw. The live endpoint answers `GET http://192.168.178.70:6094/status` with **401** (checked with a read-only curl), while `/healthz` answers 200. The server guards `/status` with `authenticate(...)` (`RC/🖥️server/🟦️.ts:139-141`). The status call goes through `remoteCacheFetchV1` (`RC/🛰️fleet/🟦️.ts:112-124`), which signs only when fleet facts name the endpoint and a `cache-read` signer loads. Unsigned or unauthorised, it gets 401 and `askServerV1` returns null. The same `catch` also swallows `requireRemoteCacheShapeV1("ServerStatus")` failures (line 179), so a schema mismatch looks identical to a refusal. That is a silent failure, and it should be surfaced.
- **`store` is null because it only reads this machine's own server directory.** `RC/⌨️command/🟦️.ts:189-197` scans `<all.server.directory>/v1` if it exists. Client machines have no such directory, so `store` is null by design. `entries` in `remote` is `server?.entries`, so it is null whenever `/status` is refused.

Should it query remote stats? **Yes.** The command's own docstring (line 187) says it reports "what the running server itself reports (entries and bytes as the server counts them)". Right now the remote half is always null on a client. Fix: sign the `/status` request with the `cache-read` role (confirm that role may read `/status`, which I did not verify), and return a reason (`401 denied`, `shape`, `unreachable`) in place of a bare null.

## 2. Why compression, download, timeoutSeconds, retries show as "unsupported"

- **Nx (`LIB/⚡️caching/🚀️bootstrap/🛠️tools/🌐️remote-cache/🟦️.ts:11-14`)** hardcodes `unsupported:["compression","download","timeoutSeconds","retries"]`. Its docstring says Nx 23 has no switch for them. That claim is taken on trust here; it is consistent with the env-var-only mechanism (`NX_SELF_HOSTED_REMOTE_CACHE_SERVER`, `..._ACCESS_TOKEN`). Honest as written. Nx-side timeouts and retries cannot be set through this path.
- **Semio engine: the claim is wrong.** The semio fleet client already implements both:
  - Timeouts: each call is bounded by `AbortSignal.timeout(timeout)` per operation, with per-operation budgets from `fleet.timeouts` (`UC/🛰️fleet/📡️client/🟦️.ts:~66-80`).
  - Retries: a ride-through loop with jittered exponential back-off (`UC/🛰️fleet/📡️client/🟦️.ts:73`, using `rideThroughInitialMilliseconds` and `rideThroughMaximumMilliseconds`).
  - Signing: `signedFetch` (`UC/🛰️fleet/📡️client/🟦️.ts`, `createFleetClientV1`).
  - Compression: the unit cache stores raw bytes, so `compression` really is unsupported.
  - Download: the `toplevel|all` modes do not map to per-unit fetches, so `download` is also genuinely unsupported.
  - **Gap:** `readFleetSettingsV1` (`UC/🛰️fleet/📡️client/🟦️.ts:~10-17`) reads only endpoint, instance, token, access, upload and signed root. It ignores `SEMIO_REMOTE_CACHE_TIMEOUT`/`_RETRIES`, and the budgets come from the fixed `FLEET_POLICY_V1`. So the contract's `timeoutSeconds` and `retries` are not honoured even though the client could honour them.
  - **Also:** I could not find any semio renderer. `tool:"semio"` and `mechanism:"semio…"` match nothing under `LIB`. The `tools()` provider in `RC/⌨️command` is only wired to Nx and Bazel (`RC/⌨️command/🟦️.ts:18-22`). The "semio engine" row in your output must come from code outside this checkout, or from a stale build.
- **Fix:** have the semio renderer report `timeoutSeconds` and `retries` as applied, and feed the contract values into `readFleetSettingsV1` and `FLEET_POLICY_V1`. Leave `compression` and `download` unsupported.

## 3. Bazel signing gap

Bazel is forced to `localOnly` at `RC/🔧️tools/🏗️bazel/🌐️remote-cache/🟦️.ts:20` when `SEMIO_REMOTE_CACHE_SIGNED=1`. The reason is that Bazel's credential helper receives no HTTP method, so it cannot sign. The helper path (`bazelCredentialHelperV1`, lines ~11-16) only emits a static `Authorization` header. `--remote_header` is rejected in the code comment because it writes the token to Bazel's java.log and build event stream.

Options:

1. **Credential helper that signs.** Not feasible. The helper does not get the method, and the signature covers it.
2. **`--remote_header` with a signed header.** Not feasible for the same reason, and it leaks a header to disk.
3. **Unsigned read-only route on the LAN.** The server already has an `inGroup:"trusted-lan"` mode (`RC/🖥️server/🟦️.ts:~23`). It weakens the trust model, so it is not recommended.
4. **Loopback signing proxy. Recommended.** One per machine, started by the build (or by the fleet daemon). Bazel gets `--remote_cache=http://127.0.0.1:<port>/<instance>` and no credentials. The proxy forwards each request with its real method through the existing `fleetSignedFetchV1` (`RC/🛰️fleet/🪪️identity/🟦️.ts`), so it signs correctly and inherits the ride-through retries and timeouts. It is tool-independent, reusable for other clients, and it removes the `localOnly` branch. Cost: one small loopback server and its lifecycle.

## 4. `cacheSkipped` and the rustc miss rate

**What `skipped` means.** It is the `skipped` counter in `UC/🛰️fleet/📊️counters/🟦️.ts:8-9`. It means the unit was compiled locally and not shared, and each occurrence is counted in `UC/📜️script.ts` or in the uploader. The reasons, from the code:

- **Breaker (fleet tier not used):** `fleetSkipReasonV1` returns `down`, `denied` or `error` (`UC/🛰️fleet/🟦️.ts:46`). Recorded at `UC/📜️script.ts:158`, reason `breaker: <why>`. This is the one most likely to dominate: every unit compiled while the endpoint is down or refusing is counted as skipped, which fits the 401 finding above.
- **Path-dependent (`real-paths`):** the compiled bytes contain a real machine path, so the unit is keyed on that path and not shared (`UC/📜️script.ts:200`, `leakedRootKindsV1` at `UC/🛰️fleet/🟦️.ts:122`).
- **Root-key failed:** `UC/📜️script.ts:198`.
- **Uploader:** `diverged: the fleet holds other bytes` and `upload-failed` (`UC/🛰️fleet/📤️uploader/🟦️.ts:137,143`).
- **Bypass (not counted as skipped, but logged):** `rustcBypassV1` (`UC/🔏️key/🟦️.ts:53-63`) returns `no-source-file`, `no-crate-name`, `no-out-dir`, `query`, `no-artifact-messages`, `no-emit`, `crate-type:*`, or `native-link`. Logged at `UC/📜️script.ts:101`.
- **Publish-skip (not counted as skipped, but not stored):** `no-outputs`, `incomplete`, `interface`, `refused`, `no-dep-info`, `error` (`UC/📜️script.ts:188-201`).

I could not measure the breakdown on this fleet. To get it, run `unit-cache stats` on one machine. It prints `notShared` by reason (the test at `UC/🧪️tests/🔁️churn/🟦️.ts:100` shows the format).

**Why the miss rate is high. Code-level hypotheses, not verified against live keys:**

- **Machine-specific platform in the remote key.** `platformOfV1` (`UC/🛰️fleet/🪪️identity/🟦️.ts:63-72`) includes `os`, `arch`, `host`, `rustc digest`, `toolchain`, `linker` (on darwin, the **darwin major version**, so machines on different macOS majors never share linked units) and `cpu` (only with `target-cpu=native`). Different macOS majors across 50+ machines would produce structural misses.
- **Non-portable roots.** `portableRootsV1` (`UC/🛰️fleet/🪪️identity/🟦️.ts:98-108`) knows only the cwd and the workspace. Other machine paths (cargo home, rustup toolchain, `$HOME`) that reach the arguments or the environment are not tokenised, so the key differs across machines. `keyEnvironmentV1` (`UC/🔏️key/🟦️.ts:103-107`) passes env values through, with only the lane root neutralised.
- **Read-only machines never seed.** `effectiveUpload` requires `access==="read-write"` and `upload` on (`RC/🟦️.ts`, `remoteCacheEffectiveUploadV1`). Every read-only machine's compile is a miss that is never published. Count how many of the 50+ machines are read-write.
- **Cold keys.** The first machine to build any key always misses, so a fleet-wide hit rate of about 34% is partly structural.

**How to confirm.** `unitKeyPartsV1` (`UC/🔏️key/🟦️.ts:116`) prints per-part digests (`toolchain`, `args`, `source`, `externs`, `environment`) so two machines' keys can be diffed for the same crate. Platform differences show up in the `remoteKeyV1` input, not in `unitKeyPartsV1`, so diff both.

## 5. Tests in `RC`

No runner matched by default: the test files are named `🟦️.ts`, so `bun test` in `RC` reports "0 test files". I ran `bun test` with the 25 test files passed explicitly (every `tests/**/🟦️.ts`, excluding fixtures).

- **Result:** 174 tests across 25 files. **170 pass, 3 skip, 1 fail.** 13,906 expect() calls, 99 s.
- **Failure:** `🏷️retention/🧪️tests/🔗️restorable/🟦️.ts`, "property: after any collection, store-driven or external, every live entry of every kind restores whole…". It **timed out at 5000 ms**, which is a load-dependent property test, not an assertion failure. Re-run it alone before treating it as a regression.
- **Log:** `wave-1121/🗑️generated/rc-tests.log` (inside the ticket's generated folder).

## Summary of recommended changes

1. Sign `/status` with the `cache-read` role. Replace the silent null with a reason, and stop swallowing shape errors.
2. Honour `timeoutSeconds` and `retries` in the semio fleet client (`readFleetSettingsV1` plus the policy budgets). Keep `compression` and `download` unsupported.
3. Bazel: add a loopback signing proxy per machine. Drop the `localOnly` branch.
4. Diagnose the skip and miss rates with `unit-cache stats` on one machine and `unitKeyPartsV1` diffs. Check macOS majors, cargo-home paths, and the read-write machine count first.
