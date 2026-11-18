# Mangled scratch directories in the worktree root

Symptom: six dirs like `C:semio.cachescratch17900-process-MpfEwXsemio-native-3vHmpatree` in `C:\semio` (the Windows path with backslashes stripped).

Producer: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🧪️tests/🟦️.ts`, helper `nativeStep` (`mkdtempSync(join(tmpdir(),"semio-native-"))`, tmpdir is `C:\semio\.cache\scratch\<pid>-process-<rand>`). It spliced `tree`/`home` (backslash paths) unquoted into a `bash -c` string (`WORKTREE=${tree}; WORKER_HOME=${home}`). Bash ate the backslashes, giving `C:semio.cache...tree`, and the provision script's `mkdir -p "$WORKTREE/.cache/f"` created it relative to cwd. Same pattern in the marker test (`WORKTREE=${root}/tree`) and the swap test (`swap_shortfall_mb ${path}`).

Fix: added `posix(path)` (backslash to slash, docstring-marked) next to `const bash` and applied it at the four shell-boundary interpolations (nativeStep WORKTREE/WORKER_HOME, the script's `touch "${root}/ran"`, marker test, swap shortfall).

Verification: six dirs were 1 KB each, matched the exact pattern, removed. Re-ran `bun test ./🟦️.ts -t "native bootstrap|swap|completion marker"`: 6 pass, no `C:*` dirs reappeared. One unrelated failure (consent code 4 test, `spawnSync("bash",["-n","-c",provisionShell])` returns undefined status on Windows, large argv), pre-existing and not touched.
