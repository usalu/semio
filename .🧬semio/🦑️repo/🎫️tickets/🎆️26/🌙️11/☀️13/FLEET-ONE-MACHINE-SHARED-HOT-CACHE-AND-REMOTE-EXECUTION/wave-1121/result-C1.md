# Result C1: Remote Cache Status, Semio Settings, Bazel Signing Proxy

Landing record with file hashes and test counts: `../landings.md`, section "C1". Lib = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, RC = `Lib/🏗️build/🌐️remote-cache/`.

## 1. `repo remote-cache status`

- Finding: the `/status` request was already signed (`remoteCacheFetchV1` wraps `fleetSignedFetchV1`; probes were authorised). `server` stayed null because the server's document failed `ServerStatus`: `fleet.transport` and `peers` were not in the schema, and `askServerV1` swallowed the shape error.
- Schema (`Lib/🏗️build/🏃️execution/🧬️schema/🌐️remote-cache.json`): declares `fleet.transport` and `peers`; new `ServerFault`; `Status.serverFault` (null or `{reason, status, message}`), `remote.hits` and `remote.misses` (sum over ac, cas, nx); `Rendering.localOnly` removed.
- `RC/⌨️command/🟦️.ts` `askServerV1` returns `{server, fault}`; reasons `unreachable`, `denied` (401/403), `unsupported` (404/405), `failed`, `malformed` (not JSON or schema mismatch); messages bilingual (`RC/📖️texts/🔣️.json`, `status.fault.*`). Cancelling still throws.
- Tests (RC `🛰️fleet/🧪️tests`): signed status against an in-process fleet server that requires signatures reports entries, bytes, maximumBytes, hits, misses; unsigned machine gets `denied`/401, dead endpoint `unreachable`, foreign JSON `malformed`.
- Note: `status` has no `--json` option (output is always the JSON document); `repo remote-cache status --json` is refused as unknown option.

## 2. Settings honoured

- Decision: contract fields stay; `timeoutSeconds` and `retries` are now honoured by the semio engine, `compression` and `download` are truthfully unsupported (the engine stores raw archives and has no top-level/all download split).
- Semio engine (`Lib/🏗️build/🔧️tools/🐙️semio/🌐️remote/🟦️.ts`, `🔌️backend`): `semioHttpRemoteV1` takes `timeoutMilliseconds`, `retries`, `backoffMilliseconds`; transport failures and 5xx are repeated, abort is honoured. Rendering: applied `timeoutSeconds=N`, `retries=N`; unsupported `["compression","download"]`.
- Unit cache fleet client (`Lib/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client`): `readFleetSettingsV1` reads `SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS` and `SEMIO_REMOTE_CACHE_RETRIES`; timeout caps every non-transfer call (blob transfers keep size-based time), retries cap the ride-through attempts. `fleetRemoteEnvironmentV1` (`🚪️launch`) exports both from the contract. Behaviour change: with the contract default `retries=3` the ride-through of a restarting server is about 3 attempts (a few seconds), not up to the 12 s policy budget; raise `SEMIO_REMOTE_CACHE_RETRIES` to ride longer.
- Nx: left unchanged and honest. The pinned Nx self-hosted cache reads only `NX_SELF_HOSTED_REMOTE_CACHE_SERVER` and `..._ACCESS_TOKEN` (the repo's Nx tool says none of timeout, retries, compression, download exist); I did not find other env vars and did not read Nx sources again, so `unsupported` stays all four.
- Tests: unit-cache fleet client (timeout cut 3 s to 1 s, retries 0 gives 1 attempt, 2 gives 3), semio remote (silent server is a miss within the contract time, 503 repeated, rendering lists applied fields).

## 3. Bazel signing proxy

- New `RC/🔏️signing-proxy/🟦️.ts` (tool-independent): `startRemoteCacheSigningProxyV1`, `acquireRemoteCacheSigningProxyV1`. Loopback 127.0.0.1, ephemeral port; only GET/HEAD/PUT on `[/instance]/ac|cas/<sha256>`; Host must be loopback; body cap 512 MiB (413); forwards with the machine's signing `fetch`; any upstream failure, timeout, 401/403 or 5xx opens a down window (15 s): reads answer 404 at once, uploads 507 (not retried by Bazel), the next request after the window asks the upstream again.
- `Lib/🏗️build/🔧️tools/🏗️bazel/🌐️remote-cache/🟦️.ts`: `startBazelRemoteCacheProxyV1(env)` resolves the contract; only for a fleet-identity machine (`SEMIO_REMOTE_CACHE_SIGNED=1`) it acquires (reuses if healthy, in-process) the proxy; otherwise or on any failure returns no url. `bazelRemoteCacheV1` takes `proxy`: signed + proxy renders `--remote_cache=<proxy>/<instance>` plus the usual flags and no credential helper; signed without proxy renders no remote flags (local build). `localOnly` is gone; status shows `--remote_cache=http://127.0.0.1:<proxy>/main` in `applied`.
- `📜️script.ts` `runBazelTasksInStorageV1`: proxy acquired before the cache flags, released in a `finally` after the build. Bazel flags get `proxy` through `bazelCacheFlagsV1`.
- Limits: reuse is within one process (a proxy dies with its process; sharing across processes would need a supervised daemon). No real Bazel binary ran; the e2e Bazel test is skipped here (`SEMIO_TEST_BAZEL_BINARY` unset). The proxy tests use Bazel-style PUT/GET on `/cas` and `/ac` against an in-process signed server.

## Tests

See landings.md. Not mine: nx single-flight native client timing flake (759 > 800 under load, passes alone), unit-cache counters schema test and the `🔢️count` helper picked up by globbing, bazel sessions concurrency test (mojibake path in another agent's tree).

## Live

At verification time nothing listened on 192.168.178.70:6094 (cache server down on this machine, not started or stopped by me), so live `repo remote-cache status` printed the typed fault `unreachable` with a bilingual message. Before my change the same server answered a signed `/status` with 200 (113662 entries, 147.6 MB, counters), which only the old schema refused. Re-run when the server is back to record the stats.
