# Exact Cargo Stage Environments

The exact Cargo runner now accepts a separate `nativeEnv` for executable discovery and exact test execution. Compilation retains the configured build environment; native overrides cannot redirect the helper-owned Cargo target. This permits large runtime test stacks without imposing the same stack reservation on every compiler worker.

The strict neutral fixture fixes distinct 8 MiB build and 256 MiB native stacks plus a shared preserved variable. The injected process-port assertions check the real runner's environment at every stage. Existing independent AJV and dual SHA-256 checks remain intact.

Test-first run 45389 exited 1: expected native stack `268435456`, received build stack `8388608`. It also exposed the cleanup-lease law's unawaited asynchronous cleanup and 5-second timeout under the loaded host. Both cleanup calls are now awaited and the scoped law has a 60-second budget; this fixture operates only inside its test-created miniature workspace.

After the runner change, 39300 exited 0: all 25 tests passed, covering real subprocess exit/timeout/cancellation/output-limit capture, strict fixture validation, active-evidence protection, and 19 hostile exact-law cases. No Rust compiler or product test success is implied by this TypeScript runner receipt.

Current native product runs were already dispatched with their old environments. Consumers are being moved to `nativeEnv` at their next execution boundary; this change does not retroactively alter a running compiler.
