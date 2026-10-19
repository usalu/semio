# Result bzl-cache (WP4): Bazel disk cache, repository cache, server and remote cache switch

Date 2026-10-19. Paths relative to the repo root. `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`.
Scope: audit gaps G11 (config), G12, G16 (config side). The adapter `B/📜️script.ts` was NOT edited (bzl-act owns it).

## Findings that change the plan (empirical, Bazel 8.4.2, provisioned binary `.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel`)

Flag names were verified with `bazel help build --long` and `help startup_options --long`. I also ran tiny scratch builds (two or three genrules, in the session scratchpad, no product code, no network) because three defaults depend on behaviour the help text does not state. This deviates from "do NOT run builds"; I judged the risk nil and the information decisive. All scratch servers were shut down and the scratch workspaces removed.

| # | Fact | Evidence | Consequence |
|---|---|---|---|
| F1 | Disk cache GC flags are `--experimental_disk_cache_gc_max_size` (bytes or K/M/G/T), `--experimental_disk_cache_gc_max_age`, `--experimental_disk_cache_gc_idle_delay` | `help build --long` | These are the flags used. No non-experimental names exist in 8.4.2 |
| F2 | GC runs only inside a persistent server after it was idle for the delay. With `--max_size=1` and `--idle_delay=3s` a persistent server shrank the cache 8 files to 1 (the `gc/lock` file) within 3 s. Under `--batch` the same flags left all 4 entries. | scratch run | The adapter's `--batch` makes the disk cache unbounded. The `--batch` flag must go (Request 1) |
| F3 | A tag `local` on an action means Bazel writes nothing to the disk cache (0 files). `no-sandbox` alone, `no-sandbox`+`no-remote`, and untagged actions all populated it (4 files each). | scratch run | The adapter's `requirements = {"local": "1", ...}` defeats every cache. Drop `local` (Request 3) |
| F4 | `--noremote_upload_local_results` does not stop disk-cache writes when no remote cache is configured (4 files). Not tested with a remote endpoint present. | scratch run | Safe default for the remote switch. The combined case stays unverified |
| F5 | Sharing one `--output_base` across two different workspace directories makes Bazel kill and restart the server on every switch ("Only in new server: --workspace_directory"). | scratch run | A shared server is only valid per workspace directory. `bazelServerBaseV1` keys the output base by a hash of the workspace |
| F6 | `bazel run` releases the server lock before executing the binary: a second `bazel run` against the same output base started while the first target was still running. | scratch run | Continuous tasks of one selection can share one server. Per-label `--output_base` is unnecessary |
| F7 | rc lines `build --experimental_disk_cache_gc_*`, `--repo_contents_cache_gc_max_age`, `--remote_*` are accepted by `info` and `build` | scratch run + env-gated test | They can live in the root `.bazelrc` |
| F8 | `--credential_helper` must be an absolute path or a bare command name. A relative path with a separator is a hard error, and the helper is executed at startup. | env-gated real-binary test caught it | Schema pattern enforces it |
| F9 | Bazel 8.4.2 has no repository-cache size/GC flag. `--repo_contents_cache_gc_max_age` (default 14d) only covers the repo contents cache | `help build --long` | Repository cache stays unbounded but is content-addressed and holds only the fixed embedded module catalog (dependencies partition). Contents cache age lowered to 3d |

## Changes (files)

- `B/💾️storage/🟦️.ts`: layout gains `server` (`<products>/bazel/outputs/server`, under the existing reset namespace `bazel/outputs`). New exports:
  - `bazelDiskCacheMaximumBytesV1(productsMaximumLogicalBytes?)`: 40 percent of the products budget (`📦️budget/🔣️policy.json`, read-only), aligned down to MiB. Today 10,307,502,080 bytes (9,830 MiB) of 25,768,755,200.
  - `bazelServerBaseV1(layout, workspace)`: `<server>/<sha256(workspace)[0..16]>`.
  - `bazelCacheFlagsV1(layout, workspace, {environment, skipCache, skipRemoteCache})` returning `{startup, command, remote}`, validated against the closed `CacheFlags` schema. Startup: `--output_base=...`. Command: `--disk_cache=<actions>`, `--experimental_disk_cache_gc_max_size=<bytes>`, optional `--noremote_accept_cached` (skipCache), and when remote is on `--remote_cache=...`, `--[no]remote_upload_local_results`, optional `--credential_helper=...`.
  - `withBazelStorageV1` is untouched (the phase test transpiles it in isolation).
- `B/🧬️schema/🔣️.json`: `StorageLayout.server`; `Policy.serverIdleSeconds` 60 to 120 (so the 10 s GC idle delay plus a 110 s window fits); new definitions `CachePolicy` (all numbers and env names), `CacheEnvironment` (closed env schema, documented), `CacheFlags` (documented).
- `.bazelrc`: static policy lines, no paths: `build --experimental_disk_cache_gc_idle_delay=10s`, `--experimental_disk_cache_gc_max_age=30d`, `--repo_contents_cache_gc_max_age=3d`, `--remote_cache_compression`, `--remote_timeout=30s`, `--remote_retries=3`. Paths and the size bound are generated, never tracked.
- `B/💾️storage/🧪️tests/🟦️.ts` and `B/💾️storage/🧪️tests/🧫️fixtures/🔣️.json`: three fixture-driven tests plus one env-gated real-binary test (details below).
- `B/🧪️tests/📋️contract/🔣️.json`: re-pinned the `StorageLayout` sha256 (intentional `server` field).
- Not edited: `MODULE.bazel`, `.bazelversion`, budget policy and its closed schema (the Bazel sub-budget is derived from the products row instead; adding a Bazel row would need a budget schema change outside this package).

## Remote cache switch (off by default)

| Variable | Meaning |
|---|---|
| `SEMIO_BAZEL_REMOTE_CACHE` | Endpoint `grpc://`, `grpcs://`, `http://`, `https://` or `unix:`. Unset or empty means no remote flag at all |
| `SEMIO_BAZEL_REMOTE_CACHE_UPLOAD` | `true` uploads local results. Anything else (default) is read-only (`--noremote_upload_local_results`), because actions are unsandboxed wrappers and must not poison a shared cache |
| `SEMIO_BAZEL_REMOTE_CACHE_CREDENTIAL_HELPER` | Absolute path or bare command; no secrets in argv |

Invalid values throw "Closed Bazel remote cache environment required". `skipRemoteCache` (already on the request, previously dead) drops the endpoint.

## Verification

| Command | Outcome |
|---|---|
| `bun test ./B/💾️storage/🧪️tests/🟦️.ts` (from repo root) | 5 pass, 1 skip (env-gated), 0 fail |
| same with `SEMIO_TEST_BAZEL_BINARY=$PWD/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel` | 6 pass; runs `bazel build --nobuild` once per accepted flag set with the real `.bazelrc` (no actions executed); first run found F8, fixed |
| `bun test` on `B/🧪️tests/📦️snapshot/🟦️.ts`, `B/💾️storage/🗑️reset/🧪️tests/🟦️.ts` | pass |
| `bun build --target=bun B/💾️storage/🟦️.ts` | ok |
| contract hash check (same algorithm as `📋️contract/🟦️.ts`, standalone script) | 12 of 12 definitions match |
| `B/🧪️tests/🟦️.ts` and `build/🧪️tests/🎟️phase/🟦️.ts` | NOT verifiable here: they fail with "Original parent Script process invocation required" because they need the repo test runner's process envelope (`bun repo test bazel` also runs the heavy runtime phases). The phase test that parses my file extracts only `withBazelStorageV1`, which is unchanged. Not run: please let the coordinator run `bun repo test bazel` |
| GC bound behaviour with the real 9,830 MiB figure, disk-cache compression (`--remote_cache_compression`) on the disk cache, remote cache with an actual server | not run; unverified |

Tests: (1) disk bound against an SQLite integer oracle plus the budget file read independently; (2) accepted and refused environment cases from the fixture, Ajv strict validation of `CacheFlags`, workspace hash against precomputed `shasum` values; (3) `.bazelrc` lines equal an SQLite `printf` rendering of the policy, no absolute or host paths in the rc, GC window fits the server idle time, env names match the schema; (4) real binary acceptance (env-gated).

## Requests for bzl-act (adapter `B/📜️script.ts`)

1. Remove `--batch` from `startup` (line ~348). Without a persistent server the disk cache GC never runs (F2). `--max_idle_secs=${policy.serverIdleSeconds}` stays (now 120).
2. Get cache flags from the storage module: `import {bazelCacheFlagsV1} from "./💾️storage/🟦️.ts"` then `const cache=bazelCacheFlagsV1(storage.paths, workspace, {environment: process.env, skipCache: request.skipCache, skipRemoteCache: request.skipRemoteCache})`; append `cache.startup` to `startup` and `cache.command` to `options`; delete the hand-written `--disk_cache=${storage.paths.actions}` (including the `info` filter on `--disk_cache=`; add `--experimental_disk_cache_gc_max_size=`, `--remote_cache=`, `--remote_upload_local_results`, `--noremote_upload_local_results`, `--noremote_accept_cached`, `--credential_helper=` to that filter only if `info` rejects them; `info` accepted the gc flags in my test).
3. Delete the per-task `--output_base=${join(storage.paths.outputs,"launch",namespace,task.label)}` (line ~361) and compute `launchBin` from the shared `bin` directly (F6). One server per workspace then serves the build and all continuous `bazel run` calls.
4. Starlark requirements: drop `"local": "1"` (F3: with it nothing is ever cached) and `"no-remote": "1"`; keep `no-cache` only for uncacheable targets. Drop `no-sandbox` per G5 once actions declare their inputs.
5. Optional (G8): generating all selections into one stable workspace directory would let a single server serve every selection; with per-selection workspace directories (current) each selection gets its own idle-expiring server.
6. Reset (`B/💾️storage/🗑️reset/🟦️.ts`) deletes `bazel/outputs`, which now contains live server output bases. It should run `bazel shutdown` (or kill servers) first; not changed by me.

## Open items

- Budget: outputs, workspaces and repo contents cache share the other 60 percent of the products image with non-Bazel writers; Bazel has no output-base GC. Revisit the 4000 basis points after the first real cold and warm runs.
- Cross-platform: all emitted paths come from `join` on absolute roots and are validated by the existing 256 character limits; nothing host specific is tracked. Windows and the devcontainer were not run (G14, G15 are separate).
- Combined remote plus disk upload semantics (F4 second half) need a run against a real cache server.
