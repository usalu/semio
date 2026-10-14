# Host Producer Native Compiler Terminal

Registered changed native PID 98408/session 38728 ended naturally Nx 1 PREASSERT after strict TypeScript and custody source 1 pass/0 failures/12 assertions. Native Plugin library test compilation reports 19 diagnostics; selected producer assertions did not run. No diagnostic names a Host producer file or its new law. No unchanged replay follows.

```text
error[E0405]: cannot find trait `OpBinaryReceiving` in crate `::protocol`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🧩️app/🦀️.rs:428:222
    |
428 | ...Text + ::protocol::OpBinary + ::protocol::OpBinaryReceiving + 'static;
    |                                              ^^^^^^^^^^^^^^^^^ not found in `::protocol`
    |
help: consider importing one of these traits
    |
  2 + use crate::dsl::original_receiving::OpBinaryReceiving;
    |
  2 + use replication::OpBinaryReceiving;
    |
  2 + use semio_framework_os_kernel::original_receiving::OpBinaryReceiving;
    |
help: if you import `OpBinaryReceiving`, refer to it directly
    |
428 -     type ConfigMutation: ::protocol::Mutation<Self::Config> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + ::protocol::OpBinaryReceiving + 'static;
428 +     type ConfigMutation: ::protocol::Mutation<Self::Config> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + OpBinaryReceiving + 'static;
    |
```

```text
error[E0405]: cannot find trait `OpBinaryReceiving` in crate `::protocol`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🪟️surface/🦀️.rs:221:199
    |
221 | ...+ Send + ::protocol::OpText + ::protocol::OpBinaryReceiving + 'static;
    |                                              ^^^^^^^^^^^^^^^^^ not found in `::protocol`
    |
help: consider importing one of these traits
    |
  2 + use crate::dsl::original_receiving::OpBinaryReceiving;
    |
  2 + use replication::OpBinaryReceiving;
    |
  2 + use semio_framework_os_kernel::original_receiving::OpBinaryReceiving;
    |
help: if you import `OpBinaryReceiving`, refer to it directly
    |
221 -     type ConfigMutation: ::protocol::Mutation<Self::Config> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinaryReceiving + 'static;
221 +     type ConfigMutation: ::protocol::Mutation<Self::Config> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + OpBinaryReceiving + 'static;
    |
```

```text
error[E0405]: cannot find trait `OpBinaryReceiving` in crate `::protocol`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🪟️surface/🦀️.rs:916:199
    |
916 | ...+ Send + ::protocol::OpText + ::protocol::OpBinaryReceiving + 'static;
    |                                              ^^^^^^^^^^^^^^^^^ not found in `::protocol`
    |
help: consider importing one of these traits
    |
  2 + use crate::dsl::original_receiving::OpBinaryReceiving;
    |
  2 + use replication::OpBinaryReceiving;
    |
  2 + use semio_framework_os_kernel::original_receiving::OpBinaryReceiving;
    |
help: if you import `OpBinaryReceiving`, refer to it directly
    |
916 -     type ConfigMutation: ::protocol::Mutation<Self::Config> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinaryReceiving + 'static;
916 +     type ConfigMutation: ::protocol::Mutation<Self::Config> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + OpBinaryReceiving + 'static;
    |
```

```text
error[E0433]: cannot find type `DslValue` in this scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/../🧪️tests/🧬️mutation-fixtures/../🧬️mutation-fixtures-surface/🦀️.rs:1096:23
     |
1096 |     let mut args=Some(DslValue::Object(invocation.arguments.into_iter().collect()));let mut fault=None;
     |                       ^^^^^^^^ use of undeclared type `DslValue`
     |
help: consider importing one of these enums
     |
   3 + use crate::DslValue;
     |
   3 + use replication::DslValue;
     |
   3 + use semio_framework::DslValue;
     |
   3 + use semio_framework_value::DslValue;
     |
```

```text
error[E0053]: method `step` has an incompatible type for trait
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:204:81
    |
204 | ...ontext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `semio_framework_job::OriginalTurnFailure`, found `semio_framework_value::ValueError`
    |
    = note: expected signature `fn(&'a mut TestAppCommandJob<_, _>, &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure>`
               found signature `fn(&mut TestAppCommandJob<_, _>, &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'_>>, semio_framework_value::ValueError>`
help: change the output type to match the trait
    |
204 -     fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
204 +     fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<std::option::Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
    |
```

```text
error[E0053]: method `close_step` has an incompatible type for trait
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:247:37
    |
247 | ...t: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `dsl::RetainedCloneGrant`
    |
    = note: expected signature `fn(&mut TestAppCommandJob<_, _>, &mut semio_framework_job::StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>`
               found signature `fn(&mut TestAppCommandJob<_, _>, dsl::RetainedCloneGrant) -> InteractiveJobCloseStep`
help: change the parameter type to match the trait
    |
247 -     fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
247 +     fn close_step(&mut self, grant: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::InteractiveJobCloseStep {
    |
```

```text
error[E0053]: method `step` has an incompatible type for trait
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:555:148
    |
555 | ...b::StepContext<'_>) -> Result<crate::retained_command::ArtifactCommandWorkStep<TestApp>, Fault> {
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `semio_framework_job::OriginalTurnFailure`, found `semio_framework_dsl::Fault`
    |
note: type in trait
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️retained-command/🦀️.rs:194:92
    |
194 | ... &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<A>, semio_framework_job::OriginalTurnFailure>;
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: expected signature `fn(&mut TestRetainedChildCommandWork, &ArtifactCommandInputs<'_, _>, &mut StepContext<'_>) -> Result<_, semio_framework_job::OriginalTurnFailure>`
               found signature `fn(&mut TestRetainedChildCommandWork, &ArtifactCommandInputs<'_, _>, &mut StepContext<'_>) -> Result<_, semio_framework_dsl::Fault>`
help: change the output type to match the trait
    |
555 -         fn step(&mut self, input: &crate::retained_command::ArtifactCommandInputs<'_, TestApp>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<crate::retained_command::ArtifactCommandWorkStep<TestApp>, Fault> {
555 +         fn step(&mut self, input: &crate::retained_command::ArtifactCommandInputs<'_, TestApp>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<plugin_builder_contract_tests::TestApp<false, 2>>, semio_framework_job::OriginalTurnFailure> {
    |
```

```text
error[E0053]: method `step` has an incompatible type for trait
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:580:148
    |
580 | ...b::StepContext<'_>) -> Result<crate::retained_command::ArtifactCommandWorkStep<TestApp>, Fault> {
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `semio_framework_job::OriginalTurnFailure`, found `semio_framework_dsl::Fault`
    |
note: type in trait
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️retained-command/🦀️.rs:194:92
    |
194 | ... &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<A>, semio_framework_job::OriginalTurnFailure>;
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: expected signature `fn(&mut TestRetainedCommandWork, &ArtifactCommandInputs<'_, _>, &mut StepContext<'_>) -> Result<_, semio_framework_job::OriginalTurnFailure>`
               found signature `fn(&mut TestRetainedCommandWork, &ArtifactCommandInputs<'_, _>, &mut StepContext<'_>) -> Result<_, semio_framework_dsl::Fault>`
help: change the output type to match the trait
    |
580 -         fn step(&mut self, input: &crate::retained_command::ArtifactCommandInputs<'_, TestApp>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<crate::retained_command::ArtifactCommandWorkStep<TestApp>, Fault> {
580 +         fn step(&mut self, input: &crate::retained_command::ArtifactCommandInputs<'_, TestApp>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<plugin_builder_contract_tests::TestApp<false, 2>>, semio_framework_job::OriginalTurnFailure> {
    |
```

```text
error[E0053]: method `step` has an incompatible type for trait
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/../🧪️tests/🔬️tool-run/../🧪️tool-run-member/🦀️.rs:44:81
   |
44 | ...b::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, ValueError> {
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `semio_framework_job::OriginalTurnFailure`, found `semio_framework_value::ValueError`
   |
   = note: expected signature `fn(&'a mut ToyMemberJob, &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure>`
              found signature `fn(&mut ToyMemberJob, &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'_>>, semio_framework_value::ValueError>`
help: change the output type to match the trait
   |
44 -     fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, ValueError> {
44 +     fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<std::option::Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
   |
```

```text
error[E0053]: method `step` has an incompatible type for trait
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🏛️reserved/../../🧪️tests/🔬️app-artifact-reserved-tool-job/🦀️.rs:16:85
   |
16 | ...ontext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `semio_framework_job::OriginalTurnFailure`, found `semio_framework_value::ValueError`
   |
   = note: expected signature `fn(&'a mut TerminalJob, &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure>`
              found signature `fn(&mut TerminalJob, &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'_>>, semio_framework_value::ValueError>`
help: change the output type to match the trait
   |
16 -         fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
16 +         fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<std::option::Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
   |
```

```text
error[E0053]: method `close_step` has an incompatible type for trait
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🏛️reserved/../../🧪️tests/🔬️app-artifact-reserved-tool-job/🦀️.rs:26:39
   |
26 | ...nt:semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `dsl::RetainedCloneGrant`
   |
   = note: expected signature `fn(&mut TerminalJob, &mut semio_framework_job::StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>`
              found signature `fn(&mut TerminalJob, dsl::RetainedCloneGrant) -> InteractiveJobCloseStep`
help: change the parameter type to match the trait
   |
26 -         fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
26 +         fn close_step(&mut self,grant:&mut semio_framework_job::StepContext<'_>) -> semio_framework_job::InteractiveJobCloseStep {
   |
```

```text
error[E0425]: cannot find function `run_runtime_close_turn_with_clock` in this scope
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:86:13
     |
  86 |             run_runtime_close_turn_with_clock(&state, || readings.next().flatten());
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1515:1
     |
1515 | fn run_runtime_close_turn_inner<PA: PluginApp + 'static>(state: &std::sync::Arc<RuntimeCloseWorkerState<PA>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<RuntimeCloseStatus>,semio_framework_job::OriginalTurnFailure> {
     | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `run_runtime_close_turn_inner` defined here
     |
help: a function with a similar name exists
     |
  86 -             run_runtime_close_turn_with_clock(&state, || readings.next().flatten());
  86 +             run_runtime_close_turn_inner(&state, || readings.next().flatten());
     |
```

```text
error[E0425]: cannot find function `run_runtime_close_turn_with_clock` in this scope
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:107:9
     |
 107 |         run_runtime_close_turn_with_clock(&state, || {
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1515:1
     |
1515 | fn run_runtime_close_turn_inner<PA: PluginApp + 'static>(state: &std::sync::Arc<RuntimeCloseWorkerState<PA>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<RuntimeCloseStatus>,semio_framework_job::OriginalTurnFailure> {
     | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `run_runtime_close_turn_inner` defined here
     |
help: a function with a similar name exists
     |
 107 -         run_runtime_close_turn_with_clock(&state, || {
 107 +         run_runtime_close_turn_inner(&state, || {
     |
```

```text
error[E0425]: cannot find function `run_runtime_close_turn_with_clock` in this scope
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:119:9
     |
 119 |         run_runtime_close_turn_with_clock(&state, || Some(0));
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1515:1
     |
1515 | fn run_runtime_close_turn_inner<PA: PluginApp + 'static>(state: &std::sync::Arc<RuntimeCloseWorkerState<PA>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<RuntimeCloseStatus>,semio_framework_job::OriginalTurnFailure> {
     | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `run_runtime_close_turn_inner` defined here
     |
help: a function with a similar name exists
     |
 119 -         run_runtime_close_turn_with_clock(&state, || Some(0));
 119 +         run_runtime_close_turn_inner(&state, || Some(0));
     |
```

```text
error[E0425]: cannot find function `run_runtime_close_turn_with_clock` in this scope
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:128:9
     |
 128 |         run_runtime_close_turn_with_clock(&state, || Some(0));
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1515:1
     |
1515 | fn run_runtime_close_turn_inner<PA: PluginApp + 'static>(state: &std::sync::Arc<RuntimeCloseWorkerState<PA>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<RuntimeCloseStatus>,semio_framework_job::OriginalTurnFailure> {
     | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `run_runtime_close_turn_inner` defined here
     |
help: a function with a similar name exists
     |
 128 -         run_runtime_close_turn_with_clock(&state, || Some(0));
 128 +         run_runtime_close_turn_inner(&state, || Some(0));
     |
```

```text
error[E0425]: cannot find function `try_schedule_runtime_close` in this scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:144:14
    |
144 |     assert!(!try_schedule_runtime_close(&pool, &state));
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
```

```text
error[E0425]: cannot find function `run_runtime_close_turn_with_clock` in this scope
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:149:5
     |
 149 |     run_runtime_close_turn_with_clock(&state, || Some(0));
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1515:1
     |
1515 | fn run_runtime_close_turn_inner<PA: PluginApp + 'static>(state: &std::sync::Arc<RuntimeCloseWorkerState<PA>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<RuntimeCloseStatus>,semio_framework_job::OriginalTurnFailure> {
     | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `run_runtime_close_turn_inner` defined here
     |
help: a function with a similar name exists
     |
 149 -     run_runtime_close_turn_with_clock(&state, || Some(0));
 149 +     run_runtime_close_turn_inner(&state, || Some(0));
     |
```

```text
error[E0425]: cannot find function `run_runtime_close_turn_with_clock` in this scope
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:175:9
     |
 175 |         run_runtime_close_turn_with_clock(&state, || samples.next().flatten());
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1515:1
     |
1515 | fn run_runtime_close_turn_inner<PA: PluginApp + 'static>(state: &std::sync::Arc<RuntimeCloseWorkerState<PA>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<RuntimeCloseStatus>,semio_framework_job::OriginalTurnFailure> {
     | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `run_runtime_close_turn_inner` defined here
     |
help: a function with a similar name exists
     |
 175 -         run_runtime_close_turn_with_clock(&state, || samples.next().flatten());
 175 +         run_runtime_close_turn_inner(&state, || samples.next().flatten());
     |
```

```text
error[E0425]: cannot find function `run_runtime_close_turn_with_clock` in this scope
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:247:5
     |
 247 |     run_runtime_close_turn_with_clock(&state, || Some(10));
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1515:1
     |
1515 | fn run_runtime_close_turn_inner<PA: PluginApp + 'static>(state: &std::sync::Arc<RuntimeCloseWorkerState<PA>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<RuntimeCloseStatus>,semio_framework_job::OriginalTurnFailure> {
     | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `run_runtime_close_turn_inner` defined here
     |
help: a function with a similar name exists
     |
 247 -     run_runtime_close_turn_with_clock(&state, || Some(10));
 247 +     run_runtime_close_turn_inner(&state, || Some(10));
     |
```
