# world_host_unit_3 GPU section (lane 5): blocked by cargo ingress

Status: root cause not reached; no files edited.

## Runs
Command (MCP repo cargo, store shared, prototype, parallelism 2):
`test -p semio-os-infinite-world-host --features testkit --test world_host_unit_3 -- on_a_real_gpu_a_section --nocapture`

1. 23m22s, exit 1. Dependency crates compiled (only warnings), then the shared generation moved twice ("Another process edited ... while it was read", restart 1 of 6), and the run ended with the Bun error below.
2. Immediate rerun, 15.9s, exit 1, same error.

Error (deterministic, raised by the repo cargo gate, not by rustc or the test):
`Closed compiler builtin-read import coverage refused: the rewrite did not count every built-in import. Fix: import fs, fs/promises and module with literal specifiers in the captured source.`
at `redirectCapturedBuiltinReadsV1` (`[eval]:116`), Bun v1.4.2. The `changes.length !== expected` check fails: some captured source (probably a peer's recent edit of a JS/TS file in the compiler capture) imports fs/fs/promises/module without a literal specifier.

## Not done
No test panic observed, so the runtime cause in the OriginalJobFixtureCaller loop or the prepared gate is still unknown. The full unit-3 target was not run.

## Next
Locate the captured source with a non-literal fs/fs/promises/module import (recent edits to repo tooling scripts) and fix it, then rerun the command above.
