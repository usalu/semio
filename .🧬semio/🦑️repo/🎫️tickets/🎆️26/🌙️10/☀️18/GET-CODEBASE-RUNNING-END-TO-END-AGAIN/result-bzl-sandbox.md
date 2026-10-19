# result-bzl-sandbox: darwin-sandbox `Operation not permitted`, root cause and fix

`B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`. `C` = this ticket. Bazel 8.4.2 (provisioned binary), macOS 27 arm64.

## Root cause (proven)
Bazel writes the darwin-sandbox profile (`<output_base>/sandbox/darwin-sandbox/N/sandbox.sb`, kept with `--sandbox_debug`) from its byte-wise (Latin-1) path strings but encodes every byte as UTF-8 a second time. The bytes of `.🧬semio` (`2e f0 9f a7 ac`) land in the profile as `2e c3 b0 c2 9f c2 a7 c2 ac` (xxd of the profile). The profile only allows `file-write*` for `/dev`, the temp dirs, `Library/{Developer,Caches,Logs}`, `/private/tmp`, `/private/var/tmp`, the sandbox's `execroot/_main` subpath and the literal `stats.out`. With a non-ASCII output base the last two never match the real (correctly encoded) path, so the first write of `process-wrapper` (`open(.../stats.out)`) and every output write fail with `Operation not permitted`.

It looked location dependent only because `/private/tmp`, `/private/var/tmp`, TMPDIR and `Library/Caches` are allow-listed for any spelling: a non-ASCII output base there works by accident. Control matrix (scratch genrule, `--spawn_strategy=sandboxed`, same binary and flags):

| Output base | Result |
|---|---|
| ticket tree `.🧬semio/.../🗑️generated/...` (non-ASCII, leading-dot emoji) | FAIL `Operation not permitted` on `stats.out` and on the output |
| `/Users/ueli/Documents/semio/bzlsb-🧰ob` (emoji, not allow-listed) | FAIL, same error |
| `/Users/ueli/Documents/semio/bzlsb-ob` (ASCII), workspace still under the emoji ticket tree | OK, `darwin-sandbox` |
| `/private/var/tmp/bzlsb-🧰ob` (emoji, allow-listed) | OK (accident) |
| ticket-tree output base + `--sandbox_base=<ASCII dir>` | OK (workaround; sandbox dir leaves the output base) |
| ASCII output base on the mounted products partition `.cache/b/p/m/...`, workspace on the partition with `s -> /Users/ueli/Documents/semio` (emoji + leading-dot inputs through the symlink, emoji output name, emoji tree artifact, `--disk_cache` on the partition) | OK: `5 processes: 1 internal, 4 darwin-sandbox`; after `clean` `4 disk cache hit`; an undeclared `s/` entry is invisible in the sandbox (`ls s/` shows only the declared `📜️script.ts`) |
| Exact production flags from `bazelCacheFlagsV1(bazelCacheLayoutV1({tools:.cache/b/t/m,dependencies:.cache/b/d/m,products:.cache/b/p/m}), ws)`: `--output_base=/Users/ueli/Documents/semio/.cache/b/p/m/bazel/outputs/server/<hash>`, `--disk_cache=.cache/b/p/m/bazel/actions`, gc size, repo `.bazelrc` | OK, `4 darwin-sandbox` |

Hypotheses ruled out: leading-dot component alone (no), TCC/`~/Documents` protection (ASCII dir in the same tree works), mounted disk image vs `/Volumes` (the `.cache/b/p/m` mount works), symlinked output dirs (not involved), async tree delete (not involved). The path quoting hypothesis was the right one, but the defect is Bazel's double encoding, not the `.sb` quoting rules.

## Fix: the production layout is already ASCII; make it fail closed
The real output base lives in the native partition (`<root>/.cache/b/p/m/bazel/outputs/server/<hash>`), which is ASCII whenever the checkout path is ASCII (`/Users/ueli/Documents/semio`). So no flag is needed (`--sandbox_base` would only move the sandbox out of the partition's finite budget). The remaining risk is a checkout or override that makes the output base non-ASCII, which now fails with a clear error at layout time instead of every sandboxed action failing mid-build.

## Changes
- `B/🧬️schema/🔣️.json`: `HostPaths.guarded` is now `["install","outputs","server"]` (and the `HostPathPolicy` enum), reason rewritten with the empirical double-encoding explanation (the old text wrongly said output bases and sandboxed actions accept non-ASCII names).
- `B/💾️storage/🟦️.ts`: unchanged; `requireBazelHostPathsV1` already iterates `guarded`. Darwin now throws `ASCII Bazel host path required on darwin: outputs` for a non-ASCII products root.
- `B/💾️storage/🧪️tests/🧫️fixtures/🔣️.json`: `hostPaths` rewritten (12 rows with `install`/`outputs`/`server`/`workspace`; incl. emoji output base, server-only, accent server, leading-dot ticket tree base refused; emoji workspace accepted; Linux/Windows accepted) and a new `sandbox` section (roots, workspace, `expectedRunner`, `forbiddenRcFlags`).
- `B/💾️storage/🧪️tests/🟦️.ts`:
  - host path test: Python `re.fullmatch` oracle now checks all guarded keys per row; products-root emoji refused with `...: outputs`; guarded list pinned.
  - new fixture test pinning the chosen flags: the production output base starts with `layout.server`, is ASCII, no `--sandbox_*`/`--spawn_strategy`/`--strategy` in the cache command flags, and the repo `.bazelrc` contains none of the fixture's forbidden flags (`spawn_strategy`, `strategy`, `genrule_strategy`, `sandbox_base`, `sandbox_add_mount_pair`, `sandbox_writable_path`, `ignore_unsupported_sandboxing`, ...). So the default `darwin-sandbox` strategy stays untouched.
  - env-gated real-binary test replaced: install crash on emoji install base; ASCII output base under the checkout runs `darwin-sandbox` with an emoji output name; emoji output base under the checkout fails with `Operation not permitted` and its `sandbox.sb` contains the double-encoded path and not the real one. If a future Bazel fixes the encoding, the last assertions fail and the guard can be relaxed.
- `.bazelrc`, adapter `B/📜️script.ts`: not edited (no flag needed; the adapter already takes `--output_base` from `bazelCacheFlagsV1`).
- Tools: `C/bzl-sandbox-tools/{setup.sh,bz.sh,prod.sh,prodbz.sh,layout.ts}` (scratch harness). All scratch workspaces, output bases (including the one created in `.cache/b/p/m/bazel/outputs/server/`, tree artifacts are read-only: `chmod -R u+w` before `rm`), disk caches and my servers were removed.

## Verification
- `bun test ./B/💾️storage/🧪️tests/🟦️.ts` (repo root): 7 pass, 2 skip, 0 fail.
- Same with `SEMIO_TEST_BAZEL_BINARY=$PWD/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel perl -e 'alarm 590; exec @ARGV' -- bun test ... --timeout 900000`: 9 pass, 0 fail (212 s; the new real-binary test alone 50 s).
- `bun build --target=bun B/💾️storage/🟦️.ts`: ok.
- Not run: `bun repo --tool=bazel run ...` (the adapter end to end), the actions/contract/phase suites, Linux/Windows.

## Open items
1. Sandboxed repo tasks can only write inside the execroot, the temp dirs, `/private/tmp`, `/private/var/tmp` and `~/Library/{Caches,Logs,Developer}`. Any task that writes into the checkout, bun caches, or `.cache/b/...` must keep `no-sandbox` (`host_state`) until G5 (declared inputs/outputs). If such a task is made sandboxed, add `--sandbox_writable_path=<ASCII path>` for the extra writable roots; writable-path flags are written through the same buggy encoder, so they must be ASCII too.
2. The adapter passes both `--output_user_root=<outputs>/<hash>` and `--output_base=<outputs>/server/<hash>`; `--output_base` wins. The `outputRoot` is dead weight there (bzl-ops/bzl-act).
3. Linux (`linux-sandbox` uses bind mounts, not a profile) and Windows were not tested; the guard is darwin only.
