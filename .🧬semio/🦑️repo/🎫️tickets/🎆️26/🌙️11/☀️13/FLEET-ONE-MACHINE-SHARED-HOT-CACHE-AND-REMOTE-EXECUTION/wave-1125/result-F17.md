# F17: remaining Windows spawns hidden, Go children hidden, services restart on crash

## Changed
1. Spawn sites: `windowsHide:true` added to 23 child-process calls under `🧰️framework` outside the F10 roots (list: scan of the hidden-spawn oracle over `🧰️framework`). One false positive, the method `spawn()` of `os/🌉️mcp/📡️stdio/🚪️gateway`, is renamed `startWarm`. The law `🗺️topology/🧪️tests/🪟️hidden-spawn/🔣️.json` now has the single root `🧰️framework` (was four), 3/3 with the Python oracle agreeing. Outside `🧰️framework` the same scan still finds 64 calls (39 `✏️s/🔌️plugins/🗄️stdio` generators, `🌎️hub` local-bootstrap and credential delivery, `🎓️teaching`, `🧪️tests`): not touched, they are not fleet, build or replica processes.
2. Go: `hideChild(command, shareConsole)` per OS in the launcher (`🪟️windows-host` sets `HideWindow` and `CREATE_NO_WINDOW`; `🐚️unix-host` is a no-op), used for the runtime probe (`📥️runtime-provision`), the macOS locale call and `RunChild`; `RunChild` keeps the shared console of an interactive launcher (only a windowless launcher hides the runtime). `ToolCommandContext` of `🌿️git` calls `hideConsoleWindow` (per-OS member dirs, on top of the batch command line), so git and every registered tool is hidden. `🧵️executor` hides `taskkill` the same way. Tests: `🚀️launcher/🧪️tests/🫥️hidden-children`, `🏠️workspace/🧪️tests/🫥️hidden-console` (each with a `.feature`).
3. Restart: `conhost --headless` loses the exit code, so tasks with `RestartOnFailure` (replica, cache, relay) now host `windowsSuperviseScriptV1` as `powershell.exe -EncodedCommand` under `conhost --headless`: `do{& <program> --% <args>;$c=$LASTEXITCODE;if($c -eq 0){exit 0};Start-Sleep -Seconds 5}while($true)`. A crash restarts after 5 s inside the console host; exit 0 ends the task (the replica's 1-minute time trigger still revives it). Mount and compaction tasks are unchanged (`cmd.exe` / `repo`, repeated by their own triggers).

## Proof
- Topology test file 74 pass (new test decodes the argument in TS and independently in Python); hidden-spawn 3/3; service test file 13 pass, 2 fail (real launchd on this Mac, unrelated).
- Go: `GOOS=windows go vet` of the executor and git per-OS files clean; launcher windows-host compiles except for symbols of sibling members; Go suite result below.

## NOT verified
- Nothing ran on Windows: `--%` handling of the quoted cache command, `-EncodedCommand` under `conhost --headless`, the 5 s restart and CREATE_NO_WINDOW effect are unproven there.
- Other Go `exec` sites are only unix (`ps`, `kill`) and left alone.

## USER (physical Windows box g1-m05)
1. After maintenance rewrites the tasks: `schtasks /Query /TN semio-fleet-replica /XML` shows `<Command>conhost.exe</Command>`, `<Arguments>` = `--headless powershell.exe -NoProfile -ExecutionPolicy Bypass -EncodedCommand ...`; same for `semio-fleet-cache` and `semio-fleet-relay` if present. The mount task still starts `cmd.exe`.
2. Decode the argument (`[Text.Encoding]::Unicode.GetString([Convert]::FromBase64String('<value>'))`): it must read `do{& powershell.exe --% ...`.
3. Crash test: Task Manager, end the `powershell.exe` or `bun.exe` of the replica; within about 10 s it must come back (new PID) with no window.
4. Watch 3 minutes of dependencies/readiness: no console flashes; `git` calls of `repo` show no window.
5. `repo fleet status`: m05 still syncs; `net use` mount still maps.

## Go suite status
The canonical Go run exited 0 without running tests (resident service BUILD_SOURCE_UNPARSEABLE). Launcher test passed in a scratch copy (macOS, stub hideChild); Windows branch only vetted; workspace hidden-console test not run. Re-run when the graph read works.
