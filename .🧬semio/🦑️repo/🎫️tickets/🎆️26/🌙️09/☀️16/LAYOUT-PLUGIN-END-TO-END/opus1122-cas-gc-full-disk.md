# cas gc on a full disk

## Cause
`repo cache cas gc` and `repo cache gc` run through the normal door: agent-session lock, capacity admission, the sealed kernel with workspace lease databases (`.cache/b/{c,l/m/data}/*.sqlite`, queue files, journals), `lease-registry/*.json`, `usage.json`. An strace of a dry run shows all of these writes happen before the collector process (which only then creates `cas/v1/gc.lock`). On 0 B free the lease-store creates fail with ENOSPC (relief cannot free enough), so the command died before collecting. The collector itself already tolerates ENOSPC, but was never reached.

## Fix
New module `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🆘️full-disk/🟦️.ts`: `reclaimFullDiskFirstV1` runs the host-store collection (`runCasCommandV1 gc`, delete only, no lease, no receipt) first when the command is a real `cache gc` / `cache cas gc` (not `--dry-run`, not `--area`) and the CAS partition has less than 64 MiB free. It is hooked in `runBuildCliV1` of `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts` before the agent-session attach and the capacity gate. Output goes to stderr (keeps `--json` clean), SIGINT/SIGTERM cancel it, failures are reported and the normal command then continues. The normal command afterwards works because space was freed.

## Tests
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🧪️tests/🆘️full-disk/🟦️.ts` with the language-neutral cases `🔣️.json` (12 cases: full, just below/at reserve, dry-run, area, status, verify, unmeasurable, other commands). ENOSPC is simulated by an injected partition measurement (0 free) and collector; one test runs the real cas gc on a simulated full partition and asserts nothing but the cas directory is written. Run: bun test of that file, 17 pass.

## Limits
Not reproduced on a real full disk. If the host CAS store holds nothing reclaimable, the later lease writes can still fail.
