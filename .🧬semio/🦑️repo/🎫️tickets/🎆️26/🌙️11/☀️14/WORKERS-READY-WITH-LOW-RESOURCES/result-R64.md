# R64: g1-m05 (Windows) stuck in `dependencies`

## Cause (proven by the log of try 6, 21:42Z)
`Internal error: Windows relative open failed (status 0x5; ERROR_ACCESS_DENIED; win32; root C:\; name C:\; role root; access 0xc0000000)`.
The relative-directory walk (`withArtifactWindowsRelativeDirectoryV1`, `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts`) opened the drive root `C:\` itself (the parent chain of the Source transition of `C:\semio\node_modules`) with GENERIC_READ|GENERIC_WRITE, because a root with no path parts is the selected directory and got write access. An unelevated user may not open a drive root for write: ERROR_ACCESS_DENIED. The earlier messages hid the origin (R62 added the code, R64 the origin, root, entry, role, access).

## Fix (landed in two landings, see landings.md "Package R64")
- Message: `artifactWindowsRelativeFailureMessageV1` names code (hex), symbolic name (table in the schema `RelativeWindowsFailureNames`), origin nt/win32, root, entry, role, access; bounded, control characters replaced.
- Root open: `artifactWindowsRootAccessLadderV1(depth)`: depth 0 tries read+write, then read, then reduced (0x120081); deeper roots try read, then reduced; the next rung is tried only after ERROR_ACCESS_DENIED. Operations that truly need write fail later with their own NT status.

## Proof
- Test `🧪️tests/🩺️windows-failure` (fixture, Python oracle `🐍️oracle.py`, ladder test): red before (export missing), green after. Existing `descriptor`, `denied-entry` and both standing preflights green.
- NOT proven: the syscall part (needs Windows; no fake of FFI exists in the module). Final proof is the next g1-m05 retry.

## g1-m05 afterwards
See the end of this file.

## State of g1-m05 at 22:28Z (try 7, started 22:00Z, with the fix)
Running 1500 s and past the point where tries 1-6 died (about 950 s, right after "Checking declared package providers: 7748 edges"); it is in `dependencies:store-ownership` with the exclusive lease. Outcome of the stage NOT yet known; `tools` and `workspace` NOT reached, so no Windows failures after `dependencies` were examined.

## Freeze note
Both landings happened before the coordinator's freeze (about 21:15 and 21:46Z). Files: only under `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/` (outside `🛍️products/🦑️repo/`), plus a new test folder. Evidence that readiness digests did not move: g1-m05 stage digest stayed `5124c49715b4` before and after (inputs are tool table, pins, bootstrap script, package.json, bun.lock, not this module); graph read-record unchanged (runner 57701, started 20:50Z). NOT proven: that no graph owner is stale, because `dependency-storage` (in products/repo) imports the descriptor module, so it may be in a tool closure; the standing preflights were green but only show no generated imports.
No further landing is made. `🧰️scripts/R64/land.py` and scratch stage copies are kept for reference.
