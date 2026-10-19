# result-bzl-label: emoji paths in Bazel labels, settled empirically (Bazel 8.4.2, macOS arm64)

`B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`. `C` = this ticket.

## Verdict
Emoji (and any non-ASCII) names work as Bazel **target names**, so the adapter's `s/<path>` labels (file in the `s` symlink to the checkout, labelled inside the root package) are correct. No fallback to hashing was needed. Only Bazel **package** names must be ASCII, and the adapter never needs an emoji package.

## Empirical matrix (scratch workspace with the repo `.bazelrc`, provisioned binary, LANG=en_US.UTF-8 as the adapter sets on darwin)
| Case | Result |
|---|---|
| filegroup/genrule `srcs` = `s/🧰️framework/🛍️products/🦑️repo/🔨️modules/🟦️.ts`, `📜️script.ts`, `📋️data/🔣️.json` | OK, content digests correct, edit of an emoji file reruns the action, revert is a disk-cache hit |
| `glob(["s/🧰️framework/**"])` through the symlink | OK (lists emoji files; skips a directory containing an ASCII-package BUILD) |
| genrule output `🧰️out/🟦️-out.txt`, `declare_directory("📜️script.snapshot")` with `🧰️d/📜️script.ts` inside | OK |
| `//🧰️framework/🛍️products:🦑️pkg_gen` (emoji **package**) | **Error**: `invalid package name '🧰️framework/🛍️products': package names may contain A-Z, a-z, 0-9, or any of ' !"#$%&'()*+,-./;<=>?[]^_\`{|}~' (most 7-bit ascii characters ...)` |
| Sandboxed execution with emoji inputs/outputs | OK, runner `darwin-sandbox`; an undeclared emoji file is invisible in the sandbox (`No such file`) |
| Disk cache | clean + rebuild: `4 disk cache hit` (incl. tree artifact); emoji `--disk_cache` path also OK |
| C locale / `LANG=C` / UTF-8 | same results; keys identical across locales (C-locale rebuild hit the UTF-8 cache) |
| Emoji workspace path, emoji `--output_base` (under /tmp), `s -> ` emoji checkout | OK, sandboxed |
| Emoji `--install_base` (any locale) | **Crash**: `Error: Could not find or load main class com.google.devtools.build.lib.bazel.Bazel` |
| Nested `BUILD.bazel` in an emoji directory | **Not** a package boundary (an emoji directory cannot be a package); label across it is accepted |
| Nested `BUILD.bazel` in an ASCII directory | `Label '//:s/adir/x.txt' is invalid because 's/adir' is a subpackage` (existing adapter boundary demotion is right) |
| 29 odd names (space, `%=,+~@`, ZWJ sequence, skin tone, combining accent, CJK, NBSP, U+0085, keycap, `.🧬dot/`, `'$;!()#&*[]{}^|<>?"\``) | all accepted as target names, also by `ctx.actions.run`; **rejected only**: backslash (`target names may not contain '\'`) and control chars (tab: `may not contain non-printable characters`) |

Note: execution-log JSON shows mojibake (Bazel keeps names as Latin-1 byte strings); content and outputs are byte-exact.

Unexplained, not pursued: with `--output_base` placed under the ticket's `🗑️generated` tree (ASCII name too) darwin-sandbox actions failed with `Operation not permitted` on `stats.out`; the same flags with the output base in /tmp, or in an ASCII dir under `Documents/semio`, work. Emoji in the output base is NOT the cause. The adapter runs mostly unsandboxed tasks, but this location-dependence should be watched when the first real sandboxed noop/launch runs from `.cache/b`.

## Changes
- `B/🧬️schema/🔣️.json`
  - `Actions.sources.label` is now `^(?:[A-Za-z0-9_@=,+~%./\-]|[^\u0000-\u007f])+$`. The old `\u0080-￿` only matched UTF-16 code units; a code-point engine (Python, Rust, Go) rejects astral emoji. Found by the Python cross-check.
  - new `HostPaths` (const: macOS install base must be ASCII, with the empirical reason) and closed `HostPathPolicy`.
  - `Actions.uncached` and `CacheDecision` gained reasons `undeclared` ("cache not declared") and `unbounded` ("outputs not declared").
- `B/📜️script.ts`: new `bazelSourceLabelV1(path)` used by `renderBazelBuildV1`. `bazelCacheDecisionV1` now fails closed (coordinator request): order continuous, skipped, pure `noop` (always cacheable), `cache:false` -> `declared`, missing `cache` -> `undeclared`, `cache:true` without an `outputs` array -> `unbounded`, else cacheable. An explicit `outputs: []` counts as declared (pure checks stay cacheable). Fingerprint nonce logic already keys on `action.cache.cache`, so uncached tasks always rerun.
- `B/💾️storage/🟦️.ts`: `requireBazelHostPathsV1(platform, layout)` (exported), called by `bazelCacheLayoutV1`: on darwin a non-ASCII `install` path is refused with `ASCII Bazel host path required on darwin: install`. Linux/Windows untouched (untested there).
- Tests/fixtures:
  - `B/🧪️tests/📥️actions/🟦️.ts`, `🔣️.json`, `🧬️schema.json`: new `names` vectors (real checked-in `📜️script.ts` path, ZWJ, skin tone, CJK, combining, NBSP, odd ASCII, space, backslash, tab); `sourceClasses` extended with emoji files, backslash and tab (hashed); `decisions` rewritten for fail-closed (10 rows, SQLite oracle updated); graph targets made explicit (`lib:generate cache:true`, `app:build outputs`, `app:test cache:true outputs:[]`); policy test lists the five reasons.
  - Pure test: label rule vs independent Python `re.fullmatch`, label form, Starlark parse (Python `ast`) of the rendered filegroup.
  - Env-gated real-Bazel oracle (`SEMIO_TEST_BAZEL_BINARY`): builds the rendered emoji filegroup, asserts sandbox runner, output content, `N disk cache hit` after clean, rebuild on edit, and per-name accept/reject equal to the fixture.
  - `B/💾️storage/🧪️tests/🟦️.ts` + fixture `hostPaths` (8 rows, Python regex cross-check) and an env-gated darwin test proving the install-base crash and that an emoji output base runs sandboxed.
- Tools: `C/bzl-label-tools/{setup.sh,bz.sh,rc}` (scratch harness). Scratch workspace, output bases, caches and all scratch servers removed (`bazel shutdown`; none left running).

## Verification
- `SEMIO_WORKSPACE_ROOT=$PWD SEMIO_TEST_ARTIFACT_DIR=<C>/🗑️generated/... bun C/bzl-act-tools/wrap.ts ./B/🧪️tests/🟦️.ts ./B/💾️storage/🧪️tests/🟦️.ts --timeout 120000`: 72 pass, 13 skip, 0 fail.
- Same with `SEMIO_TEST_BAZEL_BINARY=$PWD/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel` and `-t real`: the actions oracle and the storage binary tests pass (3 pass, 0 fail, 122 s); the earlier standalone run of the label oracle passed too (67 s).
- `bun build --target=bun B/📜️script.ts` and root `📜️script.ts`: ok.
- Not run: `bun repo --tool=bazel run ...`, Linux/Windows, the contract/phase suites outside the wrapper.

## Open items
1. Sandbox `Operation not permitted` when the output base sits under the ticket tree (see above): needs a look when the first real sandboxed action runs from `.cache/b/o`.
2. Boundary demotion stays conservative: a `BUILD` in an emoji directory (not a package) is still hashed. Exact rule would only demote when the directory name is a valid package name; gain is nil today (only ticket smoke files).
3. Fail-closed cache: all 634 targets without `cache` now rerun until the manifests declare `cache: true` and `outputs` (cache-fw/cache-s/cache-rest). `cache: true` with no `outputs` key is `unbounded`; use `outputs: []` for pure checks.
4. `scripts` bz-label-tools/fixture helper was a one-off and was deleted (not idempotent).
