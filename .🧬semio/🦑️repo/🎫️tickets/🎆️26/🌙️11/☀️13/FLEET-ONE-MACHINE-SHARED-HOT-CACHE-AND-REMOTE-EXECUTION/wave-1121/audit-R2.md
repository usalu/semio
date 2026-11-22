# Audit R2: portable argv remote actions, Windows refusals removed

Source root: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/` (paths below are relative to it).
Auditor run on macOS (darwin, bun 1.3.14). No source edited, no git modifying commands, nothing deleted.

## 1. Test files (re-run, sequential, `bun test ./<file>`)

Logs: `wave-1121/🗑️generated/audit-R2/summary.txt` and one `.log` per file.

| File | Pass | Fail | Result report |
| --- | --- | --- | --- |
| 🧱️actions/🧪️tests/🟦️.ts | 6 | 0 | 6 |
| 👷️worker/🧪️tests/🟦️.ts | 4 | 0 | 4 |
| 🔒️sandbox/🧪️tests/🟦️.ts | 6 | 0 | 6 |
| 🧪️tests/🔎️why/🟦️.ts | 10 | 0 | 10 |
| 🗓️scheduler/⚖️placement/🧪️tests/🟦️.ts | 7 | 0 | 7 |
| 🗓️scheduler/🧪️tests/🟦️.ts | 8 | 0 | 8 |
| 📡️wire/🧪️tests/🟦️.ts | 8 | 0 | 8 |
| 🚀️offload/🧪️tests/🟦️.ts | 27 | 0 | 27 |
| 🚀️offload/🧪️tests/🎯️determinism/🟦️.ts | 5 | 0 | 5 |
| 🚀️offload/🧪️tests/🧰️toolchain/🟦️.ts | 4 | 0 | 4 |
| 🧪️tests/🔧️bazel/🧭️flags/🟦️.ts | 4 | 0 | 2 (count drift, see D9) |
| 🧪️tests/🔎️refusal-reason/🟦️.ts | 3 | 0 | 3 |
| 🛰️agent/💽️disk/🧪️tests/🪟️volume/🟦️.ts | 2 | 0 | 2 |

All 13 files green (exit 0). The determinism test only runs on this darwin host; no Windows or Linux run exists, so the Windows and cross-OS claims are untested.

## 2. Review findings

Leftover references: production sources are clean. `remoteExecutionActionPlatformWhyV1` and `action-unsupported` appear only as negative assertions in `🧪️tests/🔎️why/🟦️.ts:69-70`. `/bin/sh` and `tool.environment` appear only in test fixtures and in unrelated sidecar readiness code (`🛰️sidecar/🚦️ready/🟦️.ts:561-564`, `bash` bootstrap, not part of R2). The schema JSON, texts and docs are clean. Docstrings in the changed sources start with an emoji; no comments inside definitions were found.

Windows path mapping: `remoteExecutionNativePathV1` and `win32` mapping are correct for drive letters. `..` segments are rejected by `relative()` (`🧱️actions/🟦️.ts:23`) and by `safeRelative` (`👷️worker/🟦️.ts:119-123`), including the `win32` form. A `path.win32` table test exists, but it calls the pure function directly, not the worker path (see D1).

Tool lookup: `rustc.exe` resolution is correct. `detectWorkerRustcV1` picks `policy.tool.windowsExecutable` for `windows` hosts (`🚀️offload/🧰️toolchain/🟦️.ts`), and `resolveExecutable` tries PATHEXT (`👷️worker/🟦️.ts:139-141`).

Symlink to copy fallback: functional, but unsafe, see D2.

Digest across OS: not identical in general, and this is not tested, see D5 and D6.

## 3. Defects

| ID | Sev | file:line | Finding |
| --- | --- | --- | --- |
| D1 | High (Windows) | `👷️worker/🟦️.ts:218` and `:223` | `expandRemoteExecutionRootV1(…, root)` uses its default `posix` flavour, so on native Windows the expanded values are mixed-separator: `posix.join("C:\\r\\root","a/b/c.rs")` gives `C:\r\root/a/b/c.rs` (verified with bun). Every expanded argument and env value (`--remap-path-prefix`, `-L dependency=`, `--out-dir`, sources, `OUT_DIR`) is affected, so debuginfo prefixes and crate hashes can differ from a local Windows build. The fix is to pass `{join,sep}` as the flavour, as line 141 and line 221 already do. The `🧱️actions/🟦️.ts:58` default `posix` is a trap. |
| D2 | High (Windows; confinement) | `🔒️sandbox/🟦️.ts:62-65`, `👷️worker/🟦️.ts:194`, `:211` | Input symlink targets are never checked (`links.push` at `:194`). When `symlink` fails (unprivileged Windows, or any error), the fallback `cp(resolve(dirname(path),target), path, {dereference:true})` runs unsandboxed in the worker. An absolute or `..`-escaping target therefore copies host files into the input root, where the sandboxed action can read them and return them as outputs. Fix: reject any target whose resolution leaves the input root, before placing it, on every OS. The copy fallback should also refuse targets outside the root. |
| D3 | Medium | `🧱️actions/🟦️.ts:107-118` | Environment values that contain `@root` are validated only for the `CARGO_PKG_` (any text), `PATH`, `HOME`, report and `ABSOLUTE_NAMES` cases. For any other name, `@root/../../x` passes, and the worker expands it (`👷️worker/🟦️.ts:218`) to a host path outside the root. Impact is limited to string values, since rustc does not open env values as files, but the stated invariant "`..` escapes must be rejected" is not enforced. Fix: apply the `relative()` check to every env value containing `@root`. |
| D4 | Low | `🧱️actions/🟦️.ts:23`, `:91`, `:41`, `:46` | `relative()` does not reject backslash-led or `\\?\` forms, and sources, `--extern` and `-L` paths skip `REL_PATH` (only `--remap-path-prefix` uses it). A source such as `\\?\C:\x.rs` or `a:stream.rs` passes validation and is then mapped with `win32`. This is not a traversal (the `..` check still applies), but it is not a validated relative path. Fix: apply `REL_PATH` to all path-bearing arguments. |
| D5 | Low | `🚀️offload/📜️action/🟦️.ts:52`, `:57` | `is_executable` comes from `mode & 0o111`. On Windows this is always false, so an input whose exec bit is set on posix gets a different input-root digest on Windows, and the action digest is not identical across OS for the same inputs. |
| D6 | Low | `🚀️offload/📜️action/🟦️.ts:40` (`remaps`, `SEMIO_REMAPS`) | The remap targets are the requester's native absolute paths, so they enter the action digest in native spelling (backslashes on Windows). The claim "digest identical across OS" holds only when the requester paths match. This is not verified. |
| D7 | Low | `🧱️actions/🟦️.ts:66-69` (`ROOT_TEXT`, `[^=]*`) | The `@root` path match swallows commas, so `@root/a,@root/b` maps to one path and the second `@root` is not expanded. No producer seen emits this, but it is a latent mismatch with the documented "ends at the next `=`" rule only if commas separate paths. |
| D8 | Info/Low | `🧱️actions/🟦️.ts:81`, `:82`, `:84`, `:102`, `👷️worker/🟦️.ts` refusal texts | New refusal reasons are English only. CLAUDE.md requires bilingual UI texts; these are diagnostic refusals, so this may match the existing style. |
| D9 | Info | `🧪️tests/🔧️bazel/🧭️flags/🟦️.ts` | Report says "bazel flags 2", the file runs 4 tests (all pass). The bazel parent file `🧪️tests/🔧️bazel/🟦️.ts` is not in the R2 list and was not run. |

Not blocking: the `resolveExecutable` win32 path (`👷️worker/🟦️.ts:139-141`) and `remoteExecutionToolOfProgramV1` are correct. The `placeRemoteExecutionLinkV1` copy fallback also runs on `EEXIST`, which is harmless here.

## 4. Verdict

Tests green (13/13 files). Not ready to land as-is for native Windows: D1 (mixed separators in every expanded argument) and D2 (copy fallback follows unvalidated input symlinks, unsandboxed) need fixes first. D3 is a validator gap against the stated invariant. The rest are Low or Info.
