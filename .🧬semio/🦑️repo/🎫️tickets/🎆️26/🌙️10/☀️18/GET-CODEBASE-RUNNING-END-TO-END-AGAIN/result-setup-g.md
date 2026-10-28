# result-setup-g (wave 1026g, executor setup-g): zero-touch native prerequisites

Resumed 18:55 after the coordinator restart; everything below was re-verified at 18:55-19:05 with single-file tests and `tsc` only (no official `bun repo` run, host quiet window respected).

FOR gate-g: batch ready `C/setup-g-tools/staged` (12 edited library files + 4 new; `python3 -I setup-g-tools/staged/stage.py check|apply|revert|status`; all 12 edited files currently equal their base, no peer drift, all 12 anchors unique). Tools `C/setup-g-tools/` (`gen-toolchain-fixture.py` hand-authored scenario table, `mktsconfig.py`, `rss-guard.sh`, `staged/build-overlay.py|mkscratch.py|stage.py`).

## Problem
Node's native system core (nodeffi-g, 18:08) is built from the pinned Rust toolchain, but nothing checked or prepared its prerequisites: a machine without a Rust compiler or the platform C linker (macOS stub `/usr/bin/cc` without developer tools, Windows without MSVC build tools, Linux without `cc`) failed late and cryptically; `repo tools setup all` did not build the core, and the Cargo helpers (rustc scheduler, unit-cache compiler wrapper) were only built on the first `repo cargo` call.

## LIVE (outside library/process, already on disk)
- `runtime/🧲️native/🧰️toolchain` (new, schema + `🟦️.ts` + tests `🐍️.py` oracle, `🔣️.json` 23 hand-written scenarios): pure probes for `cc`, `developer-tools` (`xcode-select -p`, `DEVELOPER_DIR`), `msvc-linker` (PATH `link.exe`, `VSINSTALLDIR`, `vswhere` with the x64 / ARM64 component), `rust` (`SEMIO_RUSTC`, PATH, `CARGO_HOME/bin`, `~/.cargo/bin`; `rustup` counts). All side effects behind one ports interface, so every platform is tested on any host; at most one short child process per probe, bounded by a timeout. Tests 5/5.
- `runtime/🟢️node/🧲️native/🏗️addon/🛠️prepare` (new): `prepare` builds the core once per source digest (built core left alone, a prebuilt `SEMIO_RUNTIME_NATIVE_ADDON` stands in), names EVERY missing prerequisite at once in EN + DE before any compiler starts, then the one locked, atomically published build; `verify` is read-only (exit 1 until built). Tests 4/4. On this Mac `verify` answers ready (exit 0).
- `runtime/🧬️schema/🗣️messages.json`: the EN/DE texts.

## STAGED (library, via gate-g)
1. `tool-setup` table: group `all` gets row `node-native-core` (pin = native protocol version from the core's schema, probe by hook, setup = the prepare script) and a new `steps` array with step `compiler-helpers` (kind `cargo-helpers`, all three platforms, optional, 1200 s) + messages `warm`, `warmed`, `warmFailed` (EN/DE); schema gets `Step`.
2. `tool-setup` engine: `toolStepsV1(platform)`, `warmToolStepsV1` (sequential, cancellable between and inside steps, an optional step failing is reported and the setup continues, a required one fails it), `toolStepSummaryV1`.
3. `tool-setup` command: after `repo tools setup` (no names = all) succeeded, the steps run; `prepareRepositoryCargoHelpersV1` is new in `process/cargo/native-helpers` and builds, inside the same issued compiler store, capacity grant and storage custody, exactly what `repo cargo` builds on demand (rustc scheduler release build, unit-cache wrapper: `/bin/sh` stand-in on POSIX, native executable on Windows).
4. `preflight`: policy requires `rust` on every platform, `developer-tools` on darwin, `msvc-linker` on win32 (and `cc`/`pkg-config` on Linux as before); new optional `apt` map (`cc` -> `build-essential`, `rust` -> `curl ca-certificates`) in the schema; the probe of the runtime module replaces the plain PATH search for ids it knows; remedy texts EN/DE for `rust`, `developer-tools`, `msvc-linker` name the exact command (bootstrap script per platform, `xcode-select --install`, Visual Studio Build Tools workload); matrix, remedies and fixture tests extended (the remedy test now walks the six-tool pool for all 3 platforms x exchange x container cases, 817 expects).
- `.devcontainer/Dockerfile` already installs `build-essential` (nothing to change); the bootstrap scripts already install rustup + the pinned toolchain (sh, ps1), so the remedies point at them.

## Verification (this session, scratch copy `mkscratch.py` = overlay files real, everything else symlinked)
Single-file `bun test`: tool-setup 22, steps 7, dispatch 5, preflight 3, toolchain scenarios 4, matrix 4, remedies 4, zero-touch 36, runtime toolchain 5, prepare 4: all pass, 0 fail. `tsc --strict` on the touched files (tool-setup, command, native-helpers, preflight, runtime toolchain + prepare; `--listFilesOnly` confirms they are in the program): 0 errors. Expected values in all fixtures are hand-written, none computed by the code under test; the Python oracle re-derives the probe verdicts independently.

## NOT verified (needs official runs after 19:35)
- A live `repo tools setup all` run with the step (builds the rustc scheduler: cargo, capacity admission) and Windows/Linux branches of `prepareRepositoryCargoHelpersV1` are not executed; only the probes are exercised on all platforms through the ports table.
- Laws that scan the whole repository (language-catalogs: new texts are bilingual; direct-tool-declarations / zero-touch-closure: new dynamic import tool-setup -> process/cargo) should be run once by gate-g after `apply`.
