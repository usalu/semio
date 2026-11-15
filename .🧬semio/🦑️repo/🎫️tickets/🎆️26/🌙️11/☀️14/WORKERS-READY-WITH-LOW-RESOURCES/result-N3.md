# Result N3: Language Dependency Tasks (2026-11-14, 11:50 to 13:00)

## Verdicts first

| Target / item | Passes | rc | Time | Verified how |
|---|---|---|---|---|
| `@semio-tech/energy-oracle-py:deps` (live, `repo run`) | **yes** | 0 | 12:18 min wall (35 s in the task, the rest is the capacity queue and the graph read); before the change rc 1 in 9:20 | pinned CPython 3.12.13 and pinned uv fetched into an empty tools partition, `uv sync` ran with downloads off |
| `@semio-tech/ui-styling-py:deps` | **not verified** | 1 (twice) | 7:28 and 1:27 | both runs stopped after the graph read with `The disk is full` of the resident service: the graph cache image `.cache/b/g/m` is 12 GiB and 100 % full (40 MiB free); the message names `/` (92 GiB free) because the failed write carries no path. Not a Python fault; owner N4 (graph state). The code change is in place and covered below |
| `@semio-tech/repo-test-dotnet:deps` | **not verified** | not run | none | blocked by my own mistake, see "Damage I caused" |
| tools-setup python rows (`repo tools setup toolchains`) | **not verified** | not run | none | `repo tools setup python-main` is not a command (`Unknown build tool`); the whole `toolchains` selection downloads every pinned tool, so only the shared provisioner is proven (native tests and the live energy run use the same code) |
| 4. `scopesPlan` symlink patch + loud receipt fault | **yes** (unit level) | 0 | bun 100 + 8 tests, 9 s | tests below; a real `repo dependencies ensure` on Linux was **not** run |
| `DOTNET_ROOT` unbound (exit 3) | **fixed earlier in the file, proven now** | 0 | 3 tests, 0.1 s | static rule on every tracked shell script plus the real system bash 3.2 |

## Damage I caused (needs a decision)

The existing test "the .NET installer downloads through Python when curl is missing" (`🔩️native/🥾️bootstrap/🧪️tests/🧰️toolchain/🟦️.ts`) passed the whole process environment to the bootstrap script. This Mac exports `DOTNET_ROOT=/Users/ueli/.dotnet`, so the fake installer of the test wrote `payload` and a 10-byte `#!/bin/sh` script **over `/Users/ueli/.dotnet/dotnet`** (12:14). Every `dotnet` on this Mac (`~/.local/bin/dotnet` links to it) is therefore broken; SDKs, runtimes and `hostfxr` under `~/.dotnet` are intact. The test now sets `DOTNET_ROOT: ""` and passes with a decoy `DOTNET_ROOT`.

Repair, not run because it downloads files and I had no permission for that: `n3-scripts/repair-dotnet-host.sh` (downloads the pinned `dotnet-install.sh`, checks its sha256 `082f7685…`, installs the 10.0.8 runtime, about 30 MB, into a temporary directory and copies only the `dotnet` file back; the fake is kept as `dotnet.fake-n3.bak`). A copy of the fake is in the scratchpad. Until it is repaired, `repo run @semio-tech/repo-test-dotnet:deps` cannot be verified on this Mac.

## 1. Python interpreter policy (decision and implementation)

Decision: the interpreter is a **pinned archive**, never a download of uv. `UV_PYTHON_DOWNLOADS` is `never` everywhere (the tool registry was right, the toolchain plan, the uv manager recipe, and the styling builder said `manual` and ran `uv python install` themselves; the `manual` mode was the only reason a second policy existed). Fresh machine: `python-build-standalone` release `20260408` (the build uv 0.11.6 pins), `install_only_stripped` archives for the six hosts and the two series 3.12.13 and 3.14.4, size and sha256 pinned, fetched through `port.fetch(url, signal, pin)` = `pinnedFetchV1` (fleet tier first, then the publisher), unpacked by the same code as CMake/Go/uv (checksum gate, link check, receipt `.toolchain.json`, one rename) into `<tools>/python/interpreters/cpython-3.12.13-macos-aarch64-none`, which is exactly the directory uv recognises as managed installation (`UV_PYTHON_INSTALL_DIR` is unchanged).

Schema first: `toolchains/🧬️schema/🔣️.json` gets `Cpython` and `InterpreterDistribution`; `python.cpython` in `toolchains/🔣️.json`; archive names may contain `+`. Pins cross-checked against three independent sources: uv's own `download-metadata.json` at tag 0.11.6, the publisher's `SHA256SUMS` (fixture `publisher.cpython`, 12 lines, test "agrees with the publishers' own checksum listings") and the independent Python oracle (`interpreter_row`: home, binary, url, identity). Mutation tests reject a foreign release host and a missing or wild `directory`.

The class, not the instance. All four callers of `uv python install` are gone:

| Caller | Before | Now |
|---|---|---|
| `energy-oracle-py:deps` (plugin `DepsScript`, registry uv, `never`) | `uv python install 3.12` failed | `repo "<native deps script>" sync python-energy-oracle` (new kind in the `NativeDependencyKind` enum, target declared `orchestration` like `deps-python`); `DepsScript` and `installArguments` removed |
| `workspace:deps-python` (kind `python`) | manager recipe `provision` = `uv python install` | `provisionInterpretersV1` before the sync (injectable, `interpreters` parameter) |
| `repo tools setup python-main`/`python-oracle` | `uv python install` with `manual` | the same provisioner, store-owned port |
| `ui-styling-py:deps` builder | own `uv python install`, `manual` | the same provisioner, `never` |

The new project `🎓️teaching/…/⚡️energy/🏗️authoring/📦️packages/🐍️python` (not mine, has a `pyproject.toml` but no project) is not yet in the toolchain plan; `toolchain-pins` "the manifests of the repository are exactly the projects of the policy" fails because of it and not because of this change. `🎓️teaching/🏛️architecture/🧍️staffage/…/🐍️.ts` runs `uv` with `never` and relies on a machine interpreter; not touched.

Not removed: the generic `provision` operation of the package manager protocol (its schema, fixtures and cross-language tests); the uv recipe still defines it, nothing calls it. Removing it is a protocol change for every manager implementation.

Proof (numbers):
- `toolchains/🧪️tests` 29 pass (627 expects), `tools/🧰️native/🧪️tests` 17 pass: provisioning into an **empty** temp tools root, once, found again offline without a fetch, corrupt archive / offline / wrong version refused with nothing published, series in order and `provision=false` reports the missing one.
- Real archives, empty temp root, real uv 0.11.15 (`n3-scripts/real-interpreter-proof.ts`): 3.12.13 in 4.3 s and 3.14.4 in 4.1 s (second run, 17 and 18 MB), `uv python find` and `uv sync` with `UV_PYTHON_DOWNLOADS=never`, `UV_PYTHON_PREFERENCE=only-managed`, venv python 3.12.13 / 3.14.4. Without the interpreter the same uv answers `No interpreter found for Python 3.12 in managed installations ... downloads are set to 'never'` (the former failure on a fresh machine).
- Live: `repo run @semio-tech/energy-oracle-py:deps` rc 0, environment `energy-oracle-7199e41be1f101e3`, `Using CPython 3.12.13`, openstudio 58.5 MiB wheel installed.
- Longest member path of the archive 111 characters; below `/Users/ueli/Documents/semio/.cache/b/t/m` the deepest file is 202 characters (limit 256).

Tests changed or added: `toolchain-pins` (builder forbids downloads, no `uv python install` literal in the three callers, `provisionInterpretersV1` present), `native-dependencies` fixture (new row `python-energy-oracle`, target and `orchestration` metadata read from the real project file, injected provisioner called with `["3.12"]` / `["3.14"]`). **The native-dependencies test itself was not run** (it needs the graph, same blocker as above); the fixture and code were checked by `bun build` only.

## 2. .NET

- The worker's change to `🧪️tests/🔬️dotnet-package-commands/🟦️.ts` (restore runs in the package root, where `🧪️Semio.Repo.Test.csproj` lives, `context.root` passed as `directory`) is correct: the csproj is in `📦️packages/🔷️dotnet`, the old working directory was the tests folder. Not run (see damage).
- `DOTNET_ROOT` unbound: bash 3.2 (the macOS system bash) reads `$DOTNET_ROOT…` as a variable named with the three bytes of the ellipsis in a UTF-8 locale. Reproduced: `env -i LANG=en_US.UTF-8 /bin/bash -c 'set -u; X=1; echo "$X…"'` fails with `X…: unbound variable`; the script already has `${DOTNET_ROOT}…` (g1-m06). New test `🔩️native/🥾️bootstrap/🧪️tests/🐚️bash-three/🟦️.ts`: no `$name` before a non-ASCII character in any tracked shell script, the rule detects exactly the failing spelling, and the real system bash fails on the unbraced and accepts the braced one (3 pass).
- Fresh machine without .NET: the existing test "the .NET installer downloads through Python when curl is missing" (empty `HOME`, no `curl`, local https server, `ensure_dotnet`) passes now that it no longer inherits the host `DOTNET_ROOT`.

## 3. `Relative filesystem fstatat failed (EPERM)` in the resident service

- Worker's `version:1` is right (`withArtifactScratchV1` request schema requires it; all other callers pass it).
- Cause (decided from code and the worker notes, **not reproduced**: this Mac keeps its tree in `/Users/Shared/semio`, and the worker logs under `~/Library/Caches/semio` are on the workers): `fstatat` on a directory descriptor returns EPERM only from a mandatory access control, on macOS the file access consent (TCC) for `~/Documents`. The consent belongs to the responsible process: the user allowed `repo` (g1-m03 report); a service started by a launch agent or by a `repo` whose signature changed (the binary was replaced, see the in-place overwrite notes) runs without it, a service started from the consented app has it, which fits "failed on the first call, passed on the second" (a different service instance). The descriptor code (`withArtifactPosixRelativeDirectoryV1`) has no path race that yields EPERM.
- Change: a typed, bilingual refusal. `failureRefusalV1` classifies EPERM below `Documents`, `Desktop` or `Downloads` of the user on macOS as `access-denied` (fields folder, reference, log) naming the settings pane, `repo service restart` and the alternative of keeping the checkout outside those folders; every other failure is unchanged. New test `🏡️resident/🧪️tests/🧯️failure/🟦️.ts` (3 pass), `🏛️service` 37, `📨️protocol` 23, `🚪️client` 8 pass.
- Not done: no probe at service start and no change of how the service is started (needs a worker to observe which parent holds the consent); the wrong partition in the disk-full text (`/` for a write without path) is a separate defect for N4.

## 4. Linux `ENOTDIR … CLAUDE.md/package.json`

- Patch of g1-m03 verified: `scopesPlan` considers a root entry only when it holds a `package.json` (a symlink that does is still refused), and ENOTDIR counts as missing in the state, replay and authenticate readers. The existing test did **not** prove it (its owner list was empty, the loop never ran); it now declares `owners: ["*/package.json"]` and passes, a second test proves the symlink-to-directory with a `package.json` is refused with `Bun installation owner is a symlink: plain-link` and a plain symlink is no owner. bun workspace tests 100 pass.
- Loud failure: `requireSoundGenerationProofV1` (generation module) throws `GenerationReceiptFaultV1` when the receipt proof is `unreadable` or `evidence-invalid` (the capture machinery broke), carrying the stale text in the declared language; both swallowing places (`publishJavascriptDependencyGenerationReceiptV1`, `publishJavascriptDependencyGenerationReceiptsElseV1`) rethrow it, so `repo dependencies ensure` ends non-zero. A proof that merely differs stays the quiet `stale` outcome (tree changed under the replica). Test "followup fault" (EN and DE, 13 quiet classes, wiring) in `🧾️generation/🧪️tests/🛡️followup`, 8 pass.

## Not verified

`ui-styling-py:deps` and `repo-test-dotnet:deps` live runs; `test-native-dependencies`; a real Linux `dependencies ensure`; a worker run of the EPERM; Windows (the pins and tar.gz extractor cover it, nothing ran there); schema catalog regeneration (`repo ./📜️script.ts schema generate` fails on another agent's `nativeBuildUnitGraphInputRequestsV1` import, so the hashes of the changed schema files in `🔣️schema-catalog.json` are stale for the toolchains, resident and native-orchestration schemas).

## What others must know

- N4: the graph image `.cache/b/g/m` is full (12 GiB, 40 MiB free); every `repo run` that reaches the project graph read fails with the (misnamed) disk-full text until generations are collected.
- N1: `tool-setup/⌨️command` python branch now calls `provisionInterpretersV1`; `locateToolV1`, `toolRowV1` and `NATIVE_TOOLCHAINS_V1` imports removed there.
- The bootstrap toolchain tests must not inherit the host environment into scripts that write below `$HOME`-like variables.
- Scripts kept in `n3-scripts/`: `real-interpreter-proof.ts`, `repair-dotnet-host.sh`, `edit_policy.py` and `rows.json` (how the pins were produced; one-shot, already applied).
