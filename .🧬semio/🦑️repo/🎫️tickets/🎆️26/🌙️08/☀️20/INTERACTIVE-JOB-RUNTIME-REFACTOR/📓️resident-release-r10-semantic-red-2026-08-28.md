# Resident Release Baseline R10: Semantic RED

## Actual Result

One canonical invocation compiled and executed18 tests across one native library binary:17 passed,1 failed,0 skipped. Nextest run IDbbe56eb1-0650-4c52-93cc-9538528635fc, exhaustive profile, footer0.089s. Nx exited1. The only failed test was tests::release_baseline::resident_current_api_charge_remains_after_allocator_return. This is an executed semantic RED, not an OS-kernel compiler failure.

The existing no-argument controller supplies only --lib. It does not supply --no-fail-fast; the output explicitly says “Cancelling due to test failure:6 tests still running,” followed by an actual18-completed footer. No flag/profile was injected to alter this behavior. All18 completed in this invocation; this is not a no-fail-fast claim.

The source/compiler hold was released at the terminal after the postcapture. No production fix, retry, Wasm check, future-seven test mount, capacity change or additional native command was performed.

## Actual Observation And Assertion Boundary

The new observer runs only after the existing System::dealloc call actually returns. The baseline's Data iteration produced:

| Observation | Actual value |
| --- | --- |
| Before charge: Data | bytes152, slots1, owners1 |
| Before charge: Control | bytes0, slots0, owners0 |
| Actual allocator return | count1, size152, alignment8 |
| Allocated bytes | 152 before;0 after |
| After actual free: Data charge | bytes0, slots0, owners0 |
| After actual free: Control charge | bytes0, slots0, owners0 |
| Cleanup before the assertion | terminaltrue; both partition usages zero; allocated0 |

The intended assertion at baseline line85 then failed: after[index] was(0,0,0), expected the original(152,1,1) until a separately granted refund. Earlier assertions confirmed one actual allocator return, the expected allocation size and all cleanup fields. There was no secondary Drop abort.

Only the Data iteration ran. The Control iteration occurs after that failing assertion and is unexecuted; neither the test name nor the fixture's two declared partitions gives it runtime credit. The current implementation refunds by the return of the same close_step that frees the page. This run does not validate a future separate Free/Refund implementation, Store FIFO binding, original Opening parent funding, or full callback timing.

## Exact Command And Captured Inputs

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 SEMIO_COVERAGE=0 SEMIO_TEST_LEVEL=exhaustive bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache
```

Read completely before execution: Dag's mounted-source report, the actual baseline, current allocator/hook/include and canonical package router/Cargo manifest. Native authority remained508b78726ae6747f476fdb7d60938b3d2349ea300ef8fc55d555502a3500c49f. Mounted tests987e2ba2933b15a79a3334b799e35830a3af99cf0b565babb338d4912f67ec1a, actual path-included ticket baseline2e73f918e3100c7a232edf22032edfe87ab225ba2d40fa232c3814d5c4420c6f and its include_str JSON2c82d7ad51115a6c5d2dc85bec5d0b2c31818275dcd4f68d7995d6556dcf828c all matched the authorized release.

Fresh72-file pre/post capture includes those actual compiled ticket inputs, all resident domain inputs, root Cargo/toolchain/runner/configuration and selected loader provenance. Schema/controller are provenance, not Rust includes. No local Rust runtime dependency was added; the package still has only serde_json as a dev dependency. The future-seven Rust file is not included. WGPU paths inherited from the selected phase receipt are loader provenance, not WGPU compilation.

All72 SHA/byte/device/inode/mtime tuples are identical; each read was stable. Resident domain re-enumeration is unchanged at16 members. Full captures, not merely abbreviated hashes, are retained:

- [Before capture](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release-r10-before-2026-08-28.json)
- [After capture with empty drift and membership delta](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release-r10-after-2026-08-28.json)
- [Exact original tool chunks, command and terminal in JSON](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release-r10-tool-output-2026-08-28.json)
- [Readable full stdout/stderr/footer](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-resident-release-r10-2026-08-28.md)

The JSON preserves original ANSI/CR bytes; readable Markdown may normalize CRLF. The failure's stdout/stderr is captured by this invocation. Passing per-test stdout was not captured and must not be inferred from a metadata directory.

## Artifact Metadata Readback

The newest artifact directory below was observed after the terminal and contains only binaries-metadata.json. The actual metadata identifies the native aarch64-apple-darwin library in the same master target. It is not per-test stdout and not a separately executed test list.

```json
{
  "observedNewest": {
    "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-2FweoO",
    "mtime": "2026-08-28T02:32:09.310Z",
    "files": [
      "binaries-metadata.json"
    ]
  },
  "metadata": {
    "rust-build-meta": {
      "target-directory": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad",
      "build-directory": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad",
      "base-output-directories": [
        "debug"
      ],
      "non-test-binaries": {},
      "build-script-out-dirs": {},
      "build-script-info": {},
      "linked-paths": [],
      "platforms": {
        "host": {
          "platform": {
            "triple": "aarch64-apple-darwin",
            "target-features": "unknown"
          },
          "libdir": {
            "status": "available",
            "path": "/Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib"
          }
        },
        "targets": []
      },
      "target-platforms": [
        {
          "triple": "aarch64-apple-darwin",
          "target-features": "unknown"
        }
      ],
      "target-platform": null
    },
    "rust-binaries": {
      "semio-framework-value-resident": {
        "binary-id": "semio-framework-value-resident",
        "binary-name": "semio_framework_value_resident",
        "package-id": "path+file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%8C%B1%EF%B8%8Fvalue/%F0%9F%92%BE%EF%B8%8Fresident/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%A6%80%EF%B8%8Frust#semio-framework-value-resident@0.1.0",
        "kind": "lib",
        "binary-path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_value_resident-ca9d6776d76d4aa3",
        "build-platform": "target"
      }
    }
  }
}
```

## Exact Terminal Output

```text
> nx run @semio-tech/value-resident-rs:test

> bun ./📜️script.ts test

────────────
 Nextest run ID bbe56eb1-0650-4c52-93cc-9538528635fc with nextest profile: exhaustive
    Starting 18 tests across 1 binary
        FAIL [   0.060s] (12/18) semio-framework-value-resident tests::release_baseline::resident_current_api_charge_remains_after_allocator_return
  stdout ───

    running 1 test
    test tests::release_baseline::resident_current_api_charge_remains_after_allocator_return ... FAILED

    failures:

    failures:
        tests::release_baseline::resident_current_api_charge_remains_after_allocator_return

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.01s
    
  stderr ───
    [DEBUG] current-api resident partition=Data before=[ResidentResources { bytes: 152, slots: 1, owners: 1 }, ResidentResources { bytes: 0, slots: 0, owners: 0 }] afterActualFree=[ResidentResources { bytes: 0, slots: 0, owners: 0 }, ResidentResources { bytes: 0, slots: 0, owners: 0 }] allocatorReturn=ReturnedFree { count: 1, size: 152, alignment: 8 } allocatedBefore=152 allocatedAfter=0 cleanup=Ok((true, [ResidentResources { bytes: 0, slots: 0, owners: 0 }, ResidentResources { bytes: 0, slots: 0, owners: 0 }], 0))

    thread 'tests::release_baseline::resident_current_api_charge_remains_after_allocator_return' (10318619) panicked at 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🧪️baseline/🦀️.rs:85:9:
    assertion `left == right` failed: actual free must leave original bytes/slots/owners charged until a later granted refund
      left: ResidentResources { bytes: 0, slots: 0, owners: 0 }
     right: ResidentResources { bytes: 152, slots: 1, owners: 1 }
    stack backtrace:
       0: __rustc::rust_begin_unwind
       1: core::panicking::panic_fmt
       2: core::panicking::assert_failed_inner
       3: core::panicking::assert_failed::<semio_framework_value_resident::ResidentResources, semio_framework_value_resident::ResidentResources>
       4: semio_framework_value_resident::tests::release_baseline::resident_current_api_charge_remains_after_allocator_return
       5: semio_framework_value_resident::tests::release_baseline::resident_current_api_charge_remains_after_allocator_return::{closure#0}
       6: <semio_framework_value_resident::tests::release_baseline::resident_current_api_charge_remains_after_allocator_return::{closure#0} as core::ops::function::FnOnce<()>>::call_once
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

  Cancelling due to test failure: 6 tests still running
────────────
     Summary [   0.089s] 18 tests run: 17 passed, 1 failed, 0 skipped
        FAIL [   0.060s] (12/18) semio-framework-value-resident tests::release_baseline::resident_current_api_charge_remains_after_allocator_return
error: test run failed
Warning: command "bun ./📜️script.ts test" exited with non-zero status code


 NX   Running target test for project @semio-tech/value-resident-rs failed

Failed tasks:

- @semio-tech/value-resident-rs:test

Hint: run the command with --verbose for more details.
```

Root and Dag were notified immediately of terminal/release, the Data-only boundary and unchanged72 inputs. No native work remains active in this lane.

