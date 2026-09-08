# Native Test Compiler Storage

runExactCargoLaws now defaults to the working workspace target directory, so a ticket changes evidence placement without creating another compiler store. Explicit Cargo directories still work, and directories containing the source workspace are refused. Executable fingerprints, exact test discovery, stage-specific environments, bounded captured output, cancellation and active evidence leases remain in place.

The language-neutral vectors and independent AJV/SHA-256 oracles were updated before implementation. The old default failed the new target-path expectations. The updated public test-exact-cargo-laws target passed 26 tests and 596 assertions. A repeat of the genuine Scale native consumer using this default is recorded separately. Explicit per-ticket compiler overrides in other scripts and editor entries remain to be removed; this report does not claim that broader cleanup is complete.

## Runtime Receipt Correction

The genuine Scale repeat passed all six native/Wasmtime laws, but its receipt exposed a default-path bug: resolving `target` against the caller’s working directory created a package-local compiler store. The helper now resolves its default against the repository workspace root, preserving explicit custom target directories. The independent path expectations failed before this correction and the public exact-law target passed afterward: 26 tests, 596 assertions (`native-test-workspace-green.log`). A second genuine Scale repeat is running to confirm the actual receipt uses the root target directory.

## Shared Native Store Runtime Confirmation

The genuine Scale UI-patch marshalling check completed successfully after the default-directory correction (`scale-native-workspace.log`). All six selected laws passed, and the list, build, per-law and final receipts identify `/Users/ueli/Documents/semio/target` as `cargoTargetDir`. The executable was `/Users/ueli/Documents/semio/target/debug/deps/semio_framework_plugin_host-0053a75c880c876b`, SHA-256 `9f37e41b0798ae42ab5195a110980b7f1fcbffa1f6ab56da07d57cd3e1d2f21e`. Compiler state remained outside the ticket; diagnostics remained inside its generated directory.
