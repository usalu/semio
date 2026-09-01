# Native Resident First-Access Allocation Diagnostic

## Actual R2 Failure

The complete R2 report was read: eight native cases passed; short/foreign refusal first failed allocation1 versus0 and then aborted during live probe Drop. This is not native admission green. The fixture now records both refusal allocation counts in a fixed array and asserts them only after the original typed consumer has been transferred, its payload retired, and both roots closed. The expected counts remain exactly `[0,0]`; no destructor guard was weakened.

## Pinned Local Standard-Library Source

`rust-toolchain.toml` pins nightly-2026-07-07. Its installed std source under `/Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/sync` was read in full for mutex/mod.rs, mutex/pthread.rs and once_box.rs. The macOS unix selection is pthread. `Mutex::new` creates an empty OnceBox, whereas `try_lock` calls `get`, whose first initialization allocates `Box::pin(pal::Mutex::new())`. The current resident `prepare_admission` calls root.access/Mutex::try_lock before checking the zero/short grant. Thus there is an actual source path to lazy allocation before refusal, not merely a guess from the observed count. No Rust compiler was invoked for this source inspection.

## Additional Actual-Boundary RED Staged

A tenth native case records allocation `(phase,size,alignment)` into a fixed eight-entry thread-local Cell array; the allocator observer does not format, allocate a Vec, symbolize a backtrace or call a mutex. It isolates root construction, first zero-item prepare, and repeated zero-item prepare. It closes the original empty root before assertions, then prints the fixed observations. Production remains unchanged for this diagnostic. Expected counts are `[0,0,0]`; actual layout and phase attribution require the next native run.

The proposed repair is an inline single-attempt atomic access gate in the sole resident authority, not prewarming or excluding an uncharged OS mutex allocation. It must have no spin/retry loop, no heap backing, and explicit busy/poison outcomes. Private consumer and receiver admission, exact-layout page backing, and original enclosing-root/fault handoff remain separate mandatory work; this access repair does not certify them.

## Actual Diagnostic R3 And Repair Candidate

The complete R3 report was read:10 executed,8PASS/2FAIL/0skip,.071s,Nx1, no abort. The old refusal case now reaches its unchanged post-retirement `[0,0]` assertion with actual `[1,0]`. The fixed observer recorded constructor0/first-refusal1/repeat0, with exact event `(phase2,64bytes,8alignment)`. This confirms the observed allocation occurs at the first refused root access, consistent with the pinned source path; it is not a captured native stack trace.

After this actual RED, the root access field was changed from std Mutex to a private inline `ResidentAccess<T>`: one compare_exchange attempt, Acquire/Release exclusion, fixed poison flag, UnsafeCell payload, and non-Send borrowed guard. Guard unwind makes poison sticky and releases only the access flag. There is no spin loop, allocation, warm-up or public poison clear. The remaining external consumer Mutex is unchanged and remains part of the expressly unproved public-alias boundary, not a valid terminal consumer witness.

Two additional narrow regression laws cover held-access refusal with zero allocations and a scalar gate's actual mutation-then-panic/sticky poison. The latter uses a trivially destructible scalar, does not dispose an unknown fault or clear a live resident root, and does not claim arbitrary domain fault cleanup. Current candidate has12 tests; no native result for this repair is claimed before the next compiler run.

## Actual R4 Native Result

The compiler-owned R4 full report was read:12PASS/0FAIL/0skip,.027s,Nx0. This includes the unchanged zero-allocation refusal/first-access laws and the two added access regressions. The runner did not emit passing-case stdout, so no unprinted numeric layout result is asserted here. The source snapshot was `f00ac674...` with tests `07f891cf...`, and its two existing Wasm checks are still pending at this entry. This is the allocation-free access/structural ownership scope only, not stable public-consumer retirement, registered destination funding, unknown-fault cleanup, or the actual native Opening parent.

The subsequent R5 report was read: existing wasm32-wasip2 check passed in.84s and wasm32-unknown-unknown check passed in.97s, overall Nx0. Existing fetch_update warnings remain. This is compilation for those two targets, not executed Wasm admission/runtime behavior. The nextest artifact directory from R4 contains metadata only; no passing-case diagnostic transcript was recovered or fabricated.

## Updated Neutral Schema Gate

`bun x nx run @semio-tech/value-resident:test` actually exited0 after the fixture/schema gained `nativeOwnership.firstAccessAllocations:[0,0,0]`. This validates the declaration and existing shared TS/model laws, not the unexecuted tenth native diagnostic.

```text

> nx run @semio-tech/value-resident:test

> bun ./📜️script.ts test

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] Native ownership neutralTrace=7 oracle=Ajv+Immer actualNativeExecution=false unknownFaultFinalDisposal=false
[DEBUG] Resident capacity=6 actualOverflow=2 ownerReader=1 partialExtent=4 simultaneousRawUiScratch=1 postedCancel=1 unsubmittedCancel=1 transferredViewFault=1 controlAxes=3 childClose=5 childFault=2 privateDispatch=5 quarantine=11 domainRecord=1 recordOverflow=3 finalizerFrontiers=8 admissionFailures=5 admissionBootstrap=7 firstFault=4 resourceWrapper=5 terminalAliasDetach=1 strictTS=0 oracle=Ajv+Immer+Buffer+BigInt



 NX   Successfully ran target test for project @semio-tech/value-resident



```
