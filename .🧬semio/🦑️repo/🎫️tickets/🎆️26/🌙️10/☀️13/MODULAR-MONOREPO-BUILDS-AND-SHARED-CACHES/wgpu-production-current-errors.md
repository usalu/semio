# Current WGPU Cargo Preparation Diagnostics

Captured from actual prototype build attempt 3; this running log includes native Cargo preparation/list compilation, so these are current transitive preparation blockers, not necessarily direct wasm-target compilation errors.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/👷️job/♻️alias/🦀️.rs

1 diagnostics; first captured failure:

```text
error[E0753]: expected outer doc comment
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/👷️job/♻️alias/🦀️.rs:1:1
  |
1 | //! 🔗️ Generic retirement returns original aliases and terminal shells without executing active jobs.
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
  |
1 - //! 🔗️ Generic retirement returns original aliases and terminal shells without executing active jobs.
1 + // 🔗️ Generic retirement returns original aliases and terminal shells without executing active jobs.
  |
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs

4 diagnostics; first captured failure:

```text
error[E0053]: method `step` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:7423:90
     |
7423 | ...text<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `OriginalTurnFailure`, found `ValueError`
     |
     = note: expected signature `fn(&'a mut BoardFillJob, &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure>`
                found signature `fn(&mut BoardFillJob, &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'_>>, ValueError>`
help: change the output type to match the trait
     |
7423 -         fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
7423 +         fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<std::option::Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🦀️.rs

8 diagnostics; first captured failure:

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🦀️.rs:106:44
     |
 106 |             Self::Upload(upload) => upload.close_step(),
     |                                            ^^^^^^^^^^-- argument #1 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1634:12
     |
1634 |     pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
     |            ^^^^^^^^^^
help: provide the argument
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/📥️natural-file/🦀️.rs

3 diagnostics; first captured failure:

```text
error[E0433]: cannot find type `RetainedCloneStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/📥️natural-file/🦀️.rs:285:272
    |
285 | ...ifecycleStep::Progress(progress)=>RetainedCloneStep::Progress(progress),PluginLifecycleStep::Complete(progress)=>RetainedCloneSt...
    |                                      ^^^^^^^^^^^^^^^^^ use of undeclared type `RetainedCloneStep`
    |
help: a struct with a similar name exists
    |
285 -         let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{self.pending=None;if !self.outcomes.terminal_is_empty(){return self.outcomes.close_step(grant)}self.close_owned_turn(grant).map(|step|match step{PluginLifecycleStep::Progress(progress)=>RetainedCloneStep::Progress(progress),PluginLifecycleStep::Complete(progress)=>RetainedCloneStep::Complete(progress),PluginLifecycleStep::Blocked{..}|PluginLifecycleStep::AwaitingInput{..}=>RetainedCloneStep::Progress(Default::default())})})?;
285 +         let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{self.pending=None;if !self.outcomes.terminal_is_empty(){return self.outcomes.close_step(grant)}self.close_owned_turn(grant).map(|step|match step{PluginLifecycleStep::Progress(progress)=>RetainedCloneGrant::Progress(progress),PluginLifecycleStep::Complete(progress)=>RetainedCloneStep::Complete(progress),PluginLifecycleStep::Blocked{..}|PluginLifecycleStep::AwaitingInput{..}=>RetainedCloneStep::Progress(Default::default())})})?;
    |
help: consider importing one of these enums
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs

10 diagnostics; first captured failure:

```text
error[E0425]: cannot find type `PluginLifecycleFailure` in the crate root
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:81:127
   |
81 | ...urnV1<'_, '_>) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure>;
   |                                                         ^^^^^^^^^^^^^^^^^^^^^^
   |
  ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏛️ownership/🦀️.rs:38:1
   |
38 | pub enum PluginLifecycleStep {
   | ---------------------------- similarly named enum `PluginLifecycleStep` defined here
   |
help: an enum with a similar name exists
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs

2 diagnostics; first captured failure:

```text
error[E0425]: cannot find type `PluginLifecycleFailure` in the crate root
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1103:69
     |
1103 |         let maintenance = (|| -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure> {
     |                                                                     ^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏛️ownership/🦀️.rs:38:1
     |
  38 | pub enum PluginLifecycleStep {
     | ---------------------------- similarly named enum `PluginLifecycleStep` defined here
     |
help: an enum with a similar name exists
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🎟️context/🌉️call/🦀️.rs

1 diagnostics; first captured failure:

```text
error[E0404]: expected trait, found derive macro `semio_framework_value::RetireOwned`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🎟️context/🌉️call/🦀️.rs:44:29
   |
44 |  fn original_slot_demands<T:semio_framework_value::RetireOwned>(source:&ManuallyDrop<Option<T>>,retirement:&ManuallyDrop<Option<Cont...
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a trait
   |
help: consider importing one of these traits instead
   |
 2 + use crate::__value::retirement::RetireOwned;
   |
 2 + use replication::retirement::RetireOwned;
   |
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs

21 diagnostics; first captured failure:

```text
error[E0425]: cannot find type `PluginLifecycleFailure` in the crate root
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:2954:155
     |
2954 | ...StepContext<'_>)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
     |                                                        ^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏛️ownership/🦀️.rs:38:1
     |
  38 | pub enum PluginLifecycleStep {
     | ---------------------------- similarly named enum `PluginLifecycleStep` defined here
     |
help: an enum with a similar name exists
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🤝️group/🦀️.rs

2 diagnostics; first captured failure:

```text
error[E0425]: cannot find type `PluginLifecycleFailure` in the crate root
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🤝️group/🦀️.rs:193:146
    |
193 | ...<semio_framework_job::InteractiveJobCloseStep,crate::PluginLifecycleFailure>{
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^
    |
   ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏛️ownership/🦀️.rs:38:1
    |
 38 | pub enum PluginLifecycleStep {
    | ---------------------------- similarly named enum `PluginLifecycleStep` defined here
    |
help: an enum with a similar name exists
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/../🪜️close-ladder/🦀️.rs

4 diagnostics; first captured failure:

```text
error[E0425]: cannot find value `cx` in this scope
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/../🪜️close-ladder/🦀️.rs:643:122
    |
643 | ...self.instance_operation_owner.close_inference_step(cx,grant).map(Self::lifecycle_retained)}
    |                                                       ^^ not found in this scope

error[E0425]: cannot find value `grant` in this scope
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/../🪜️close-ladder/🦀️.rs:643:125
    |
643 | ...instance_operation_owner.close_inference_step(cx,grant).map(Self::lifecycle_retained)}
    |                                                     ^^^^^ not found in this scope

```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/../🪜️maintenance-ladder/🦀️.rs

4 diagnostics; first captured failure:

```text
error[E0425]: cannot find type `PluginLifecycleFailure` in the crate root
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/../🪜️maintenance-ladder/🦀️.rs:174:167
    |
174 | ...tainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^
    |
   ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏛️ownership/🦀️.rs:38:1
    |
 38 | pub enum PluginLifecycleStep {
    | ---------------------------- similarly named enum `PluginLifecycleStep` defined here
    |
help: an enum with a similar name exists
```

