# Plugin Checkpoint Tail R4 Native RED

## Actual Result

Exactly one selected checkpoint/restore test: **0 passed, 1 failed, 523 skipped**, .296s; SIGABRT (.277s), Nx exit1. The first failure is the live catalog constructor: generated_migrated=false while owner, controller, schema, typed_join and uniqueness agree. During its unwind the Interaction ArtifactStore lacks the required terminal-empty shell witness and its Drop panics, causing SIGABRT. These are separate primary catalog and secondary owner-cleanup defects. No restored publication, result ACK, UI completion or full checkpoint success is claimed.

## Source and Selection

[Selected inputs](./📓️plugin-checkpoint-r4-selected-inputs-2026-08-28.md): 751 pre-dispatch hashes and 113 sibling Store hashes captured during the held compile; neither is mislabeled an atomic complete closure. Main04f85d0047b46c6b0d6884aee175f787f70f0eeeae6531b6b90069edc3ac1935 and Store0ed0d7a78c833c1081825c598de3a5dde36ecc858a2e1448c5695899358efd0d match releases. Existing target/jobs2, exhaustive/no-fail-fast, no budget/stack/profile changes. Concurrent foreign stdio compilation used workspace target/debug, not this retained target; no timing certification.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-checkpoint-tail-r4-2026-08-28.md'
```

Raw [R4 stream](./🧪️member-plugin-checkpoint-tail-r4-2026-08-28.md); full untruncated tool output below. The separately authorized restart-two regression follows on the same held source and does not substitute for this failing tail.

## Full Actual Tool Output

```text

> nx run @semio-tech/framework-plugin:test --args=exhaustive checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume --no-fail-fast -- --nocapture

> bun 📜️script.ts test exhaustive checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume --no-fail-fast -- --nocapture

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] plugin-runner-oracle cases=6
────────────
[32;1m Nextest run[0m ID [1mbcef9eaf-fb6e-4237-b7c6-b9dbddb5c660[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m1[0m test across [1m1[0m binary ([1m523[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/1) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume[0m

running 1 test

thread 'component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume' (9173486) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:19006:18:
tool proof catalog must exactly join migrated generated declarations to live concrete factories: Fault { origin: Framework, code: FaultCode("interactive-job.catalog-authority"), severity: Error, message: "tool factory proof rejected tool 'applyCountFromTask': owner='semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::TestApp<true>' expected_owner='semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::TestApp<true>', controller='s.test.synthetic@1/*#editor' expected_controller='s.test.synthetic@1/*#editor', schema='semio.test/v1' expected_schema='semio.test/v1', factory='TestRestartFactory<true>' registered_factory='semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::TestRestartFactory<true>', owner_file_present=true, generated_migrated=false, unique=true, typed_join=true", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], retryable: false }
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: <core::result::Result<(alloc::string::String, alloc::vec::Vec<semio_framework_plugin::component::app::QualifiedBoundedFirstStepProof>), protocol::diagnostic::Fault>>::expect
   4: <semio_framework_plugin::component::app::VcsArtifactApp<semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::TestApp<true>>>::with_registry_on_bus::{closure#0}
   5: <semio_framework_plugin::component::app::VcsArtifactApp<semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::TestApp<true>>>::with_registry::{closure#0}
   6: semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::test_restart_publish_and_close::{closure#0}
   7: semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume::{closure#0}
   8: semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume::__semio_async_test_block_on::<semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume::{closure#0}>
   9: semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume
  10: semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume::{closure#0}
  11: <semio_framework_plugin::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume' (9173486) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106402b34 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x106415ea8 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x106407260 - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x1063e9b0c - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1063fc5d0 - std[87758e35c17852a5]::panicking::default_hook
   5:        0x1063fc8f8 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x1063e9bc0 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x1063df278 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x1063ea1ec - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x10644b790 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x104528e98 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<protocol[45777f736795148b]::wire::frames::InteractionState, semio_framework_plugin[22fb8bccf0b2bad]::component::app::InteractionConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x1044152a0 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<protocol[45777f736795148b]::wire::frames::InteractionState, semio_framework_plugin[22fb8bccf0b2bad]::component::app::InteractionConfigMutation>>
  12:        0x105316840 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::plugin_runtime::plugin_builder_contract_tests::TestApp<true>>>::with_registry_on_bus::{closure#0}
  13:        0x105312064 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::plugin_runtime::plugin_builder_contract_tests::TestApp<true>>>::with_registry::{closure#0}
  14:        0x1042a873c - semio_framework_plugin[22fb8bccf0b2bad]::component::plugin_runtime::plugin_builder_contract_tests::test_restart_publish_and_close::{closure#0}
  15:        0x1042ea3c0 - semio_framework_plugin[22fb8bccf0b2bad]::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume::{closure#0}
  16:        0x1042951d0 - semio_framework_plugin[22fb8bccf0b2bad]::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume::{closure#0}>
  17:        0x105b7174c - semio_framework_plugin[22fb8bccf0b2bad]::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume
  18:        0x1042aa0d8 - semio_framework_plugin[22fb8bccf0b2bad]::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume::{closure#0}
  19:        0x10458e8cc - <semio_framework_plugin[22fb8bccf0b2bad]::component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  20:        0x105f978cc - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  21:        0x105fa2c00 - test[ee52d9429afbedb2]::run_test::{closure#0}
  22:        0x105f9dae0 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  23:        0x105fa50c8 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:        0x106402488 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  25:        0x188071c58 - __pthread_cond_wait

thread 'component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume' (9173486) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
[31;1m     SIGABRT[0m [   0.277s] (1/1) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume[0m
────────────
[31;1m     Summary[0m [   0.296s] [1m1[0m test run: [1m0[0m [32;1mpassed[0m, [1m1[0m [31;1mfailed[0m, [1m523[0m [33;1mskipped[0m
[31;1m     SIGABRT[0m [   0.277s] (1/1) [35;1msemio-framework-plugin[0m [36mcomponent::plugin_runtime::plugin_builder_contract_tests[0m[36m::[0m[34;1mcheckpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume[0m
[31;1merror[0m: test run failed
Warning: command "bun 📜️script.ts test exhaustive checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume --no-fail-fast -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-plugin failed

Failed tasks:

- @semio-tech/framework-plugin:test

Hint: run the command with --verbose for more details.


```

