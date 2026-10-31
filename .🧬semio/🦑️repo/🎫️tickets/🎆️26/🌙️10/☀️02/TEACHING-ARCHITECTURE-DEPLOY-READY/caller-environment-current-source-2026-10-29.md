# Full Caller Environment Gate — Current Source

Session83522 authoritatively exited1. The exact normal target was SEMIO_LANGUAGE=en SEMIO_TICKET_DIR=<ticket> repo run repo:test-caller-environment --parallel=1, with NX_DAEMON absent (verified through registered Node before launch). No test filter, fake Original, environment daemon opt-out, Source change or retry was applied.

Actual result:11 tests in1file,4passed and7failed,22expect calls,16.08s. The ordinary prerequisite ui-rs generation passed. Source was served, generation52 published, normal Nx daemon graph calculation completed, and the actual caller-environment owner executed. Nx and the enclosing repository command reported failure, not false success.

## First Cause

The first failing law was “actual immutable Node daemon startup admits one producer across parallel clients”. Its genuinely owned Node child attempted to build the Node native core for linux-x64-f6e353a9518bd18e. The actual error was RuntimeCapabilityMissingV1: Runtime node lacks capability ffi: The Node native core did not build (compiler status1): linker cc not found, No such file or directory. The SDK namespace was363d3244f321fa2795bc45f2e92f6a01d86d8755485802968f6f9fca894af544.

Later OriginalAdmissionClosed messages explicitly named that first child failure. Those downstream tests cannot be treated as independent daemon/fallback regressions. The complete suite still failed and has not demonstrated all failure-isolation, fallback and next-invocation laws on this current runtime.

Evidence: 🗑️generated/caller-environment-full-current.log. Parent received the exact cause immediately. No linker shim, system package install, quota change, source guard relaxation or unchanged retry was attempted. The successful current course typecheck30943, complete course units33103 and real managed-backend persistence proof are separate completed scopes.
