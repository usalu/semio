# Actual Browser Native Compile Refusals

Owned exact Browser output repair67341/session89261 remains live until natural terminal. Strict TS and portable output/Source laws passed2/0 with45 assertions. Compilation currently reports plugin mandatory-context migrations; the actual native Browser output law has not run. No graph/cache reset or alternate engine is used.

| Defining Source | Errors Observed |
| --- | ---: |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs | 17 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs | 10 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs | 21 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎞️media/🦀️.rs | 6 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🦀️.rs | 3 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs | 2 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/../🪜️maintenance-ladder/🦀️.rs | 1 |

## Exact Compiler Causes

```text
error[E0061]: this method takes 3 arguments but 4 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:2695:18
     |
2695 |     instance.app.submit_media_export_checked(port_id, expected_parent_document_id, expected_base_revision,cx).await
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^                                                              -- unexpected argument #4 of type `&mut semio_framework_job::StepContext<'_>`
     |
note: method defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:470:14
     |
 470 |     async fn submit_media_export_checked(&mut self, _port: &str, _expected_parent_document_id: &str, _expected_base_revision: u64)...
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:2695:5
     |
2695 |     instance.app.submit_media_export_checked(port_id, expected_parent_document_id, expected_base_revision,cx).await
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `semio_framework_job::OriginalTurnFailure`, found `MediaError`
     |
     = note: expected enum `Result<_, semio_framework_job::OriginalTurnFailure>`
                found enum `Result<_, MediaError>`

error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:2701:18
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:2701:18
     |
2701 |     instance.app.poll_media_export(handle,cx).await
     |                  ^^^^^^^^^^^^^^^^^        -- unexpected argument #2 of type `&mut semio_framework_job::StepContext<'_>`
     |
note: method defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:474:14
     |
 474 |     async fn poll_media_export(&mut self, _handle: &ArtifactMediaExportHandle) -> Result<ArtifactMediaExportPoll, MediaError> {
     |              ^^^^^^^^^^^^^^^^^
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:2701:5
     |
2701 |     instance.app.poll_media_export(handle,cx).await
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `semio_framework_job::OriginalTurnFailure`, found `MediaError`
     |
     = note: expected enum `Result<_, semio_framework_job::OriginalTurnFailure>`
                found enum `Result<_, MediaError>`

[script] {"owner":"procedural:Host:original-browser-output-native","command":"native:owner-command","stage":"running"}
[native:owner-command] running elapsedMs=390120
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:1160:98
     |
1160 |         let mut session = match crate::reserved_job_session::admit_mounted(dispatch.job, params, step_grant) {
     |                                 ------------------------------------------                       ^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                 |
     |                                 arguments to this function are incorrect
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:60:15
     |
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:1165:52
     |
1165 |                     if matches!(refused.close_step(close_grant), semio_framework_job::InteractiveJobCloseStep::Refused { .. }) {
     |                                         ---------- ^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                         |
     |                                         arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:33:16
     |
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:1165:112
     |
1165 |                     if matches!(refused.close_step(close_grant), semio_framework_job::InteractiveJobCloseStep::Refused { .. }) {
     |                                                                                                                ^^^^^^^ variant not found in `InteractiveJobCloseStep`

error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:1265:397
     |
1265 | ...:default()}}else{session.close_step(grant)}}) {
     |                             ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:1265:397
     |
1265 | ...:default()}}else{session.close_step(grant)}}) {
     |                             ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                             |
     |                             arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2416:12
     |
```

```text
error[E0308]: `if` and `else` have incompatible types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:1265:378
     |
1265 | ....depth{semio_framework_job::WorkerJobCloseStep::Pending{progress:Default::default()}}else{session.close_step(grant)}}) {
     |           -----------------------------------------------------------------------------      ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `WorkerJobCloseStep`, found `Result<WorkerJobCloseStep, _>`
     |           |
     |           expected because of this
     |
     = note: expected enum `WorkerJobCloseStep`
                found enum `Result<WorkerJobCloseStep, semio_framework_job::OriginalTurnFailure>`
help: consider using `Result::expect` to unwrap the `Result<WorkerJobCloseStep, semio_framework_job::OriginalTurnFailure>` value, panicking if the value is a `Result::Err`
```

```text
error[E0599]: no variant named `Refused` found for enum `WorkerJobCloseStep`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:1271:65
     |
1271 |                     Ok(semio_framework_job::WorkerJobCloseStep::Refused{..}) | Err(_) => {
     |                                                                 ^^^^^^^ variant not found in `WorkerJobCloseStep`

warning: field `0` is never read
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/📑️copy/🦀️.rs:58:44
   |
58 | enum Advance { Child(Box<dyn Task>), Bytes(usize), Complete(Box<dyn Copied>), Fault(ValueError) }
   |                                      ----- ^^^^^
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:5871:114
     |
5871 | ...ted) = match crate::reserved_job_session::admit_mounted(dispatch.job, params, self.mounted_policy.preparation) {
     |                 ------------------------------------------                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                 |
     |                 arguments to this function are incorrect
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:60:15
     |
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:5925:250
     |
5925 | ...grant|{let step=rejected.close_step(grant).admit(grant,rejected.terminal_is_empty());Ok((match step{semio_framework_job::Intera...
     |                             ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                             |
     |                             arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:33:16
     |
```

```text
error[E0599]: no method named `admit` found for enum `Result<T, E>` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:5925:257
     |
5925 | ..._step(grant).admit(grant,rejected.terminal_is_empty());Ok((match step{semio_framework_job::InteractiveJobCloseStep::Pending{pro...
     |                 ^^^^^ method not found in `Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>`
     |
note: the method `admit` exists on the type `InteractiveJobCloseStep`
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1298:5
     |
1298 |     pub fn admit(self,grant:RetainedCloneGrant,terminal_is_empty:bool)->Result<Self,ValueError>{
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:5925:673
     |
5925 | ...ework_job::InteractiveJobCloseStep::Refused{kind,progress}=>return Err(plugin_sdk_fault(format!("media rejection retirement ref...
     |                                        ^^^^^^^ variant not found in `InteractiveJobCloseStep`

[cargo:build] running elapsedMs=380081
[script] {"owner":"procedural:Host:original-browser-output-native","command":"native:owner-command","stage":"running"}
[native:owner-command] running elapsedMs=400122
error[E0599]: no variant named `Refused` found for enum `WorkerJobCloseStep`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:80:54
```

```text
error[E0599]: no variant named `Refused` found for enum `WorkerJobCloseStep`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:80:54
   |
80 | ...   semio_framework_job::WorkerJobCloseStep::Refused{kind,progress}=>return Err(plugin_sdk_fault(format!("original worker retireme...
   |                                                ^^^^^^^ variant not found in `WorkerJobCloseStep`

error[E0308]: mismatched types
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:90:13
   |
89 |         let (step,terminal)=owner(grant);let step=match step.admit(grant,terminal){
   |                                                         -------------------------- this expression has type `Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
```

```text
error[E0308]: mismatched types
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:90:13
   |
89 |         let (step,terminal)=owner(grant);let step=match step.admit(grant,terminal){
   |                                                         -------------------------- this expression has type `Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
90 |             semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{p...
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
```

```text
error[E0308]: mismatched types
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:90:77
   |
89 | ...);let step=match step.admit(grant,terminal){
   |                     -------------------------- this expression has type `Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
90 | ...ctiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>PluginLifecycleStep::Progr...
   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
```

```text
error[E0308]: mismatched types
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:91:13
   |
89 |         let (step,terminal)=owner(grant);let step=match step.admit(grant,terminal){
   |                                                         -------------------------- this expression has type `Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
90 |             semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::Interactiv...
91 |             semio_framework_job::InteractiveJobCloseStep::Blocked=>PluginLifecycleStep::AwaitingInput{reason},
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:92:59
   |
92 | ...   semio_framework_job::InteractiveJobCloseStep::Refused{kind,progress}=>return Err(plugin_sdk_fault(format!("original rejection ...
   |                                                     ^^^^^^^ variant not found in `InteractiveJobCloseStep`

error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:126:108
    |
126 |         let (session, session_rejected) = match crate::reserved_job_session::admit_mounted(decode, params, retained) {
    |                                                 ------------------------------------------                 ^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:126:108
    |
126 |         let (session, session_rejected) = match crate::reserved_job_session::admit_mounted(decode, params, retained) {
    |                                                 ------------------------------------------                 ^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
    |                                                 |
    |                                                 arguments to this function are incorrect
    |
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:60:15
    |
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:217:147
    |
217 | ...|grant|{let step=rejected.close_step(grant);(step,rejected.terminal_is_empty())});
    |                              ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
    |                              |
    |                              arguments to this method are incorrect
    |
note: method defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:33:16
    |
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:217:155
    |
217 | ...ed.close_step(grant);(step,rejected.terminal_is_empty())});
    |                          ^^^^ expected `InteractiveJobCloseStep`, found `Result<InteractiveJobCloseStep, _>`
    |
    = note: expected enum `InteractiveJobCloseStep`
               found enum `Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>`
help: consider using `Result::expect` to unwrap the `Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>` value, panicking if the value is a `Result::Err`
    |
217 |             return original_rejection_lifecycle(demand,grant,"envelope rejection awaits its original worker",|grant|{let step=rejected.close_step(grant);(step.expect("REASON"),rejected.terminal_is_empty())});
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:248:149
     |
 248 | ...|grant|{let step=session.close_step(grant);(step,session.terminal_is_empty())});
     |                             ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                             |
     |                             arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2416:12
     |
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:248:157
    |
248 | ...session.close_step(grant);(step,session.terminal_is_empty())});
    |                               ^^^^ expected `WorkerJobCloseStep`, found `Result<WorkerJobCloseStep, _>`
    |
    = note: expected enum `WorkerJobCloseStep`
               found enum `Result<WorkerJobCloseStep, semio_framework_job::OriginalTurnFailure>`
help: consider using `Result::expect` to unwrap the `Result<WorkerJobCloseStep, semio_framework_job::OriginalTurnFailure>` value, panicking if the value is a `Result::Err`
    |
248 |             return original_worker_lifecycle(demand,grant,"envelope original terminal worker remains contended",|grant|{let step=session.close_step(grant);(step.expect("REASON"),session.terminal_is_empty())});
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:260:53
     |
 260 |         match session.pump_one_for_interactive_turn(grant){
     |                       ----------------------------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                       |
     |                       arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2351:12
     |
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:695:105
    |
695 |         let (session, session_rejected) = match crate::reserved_job_session::admit_mounted(job, params, retained) {
    |                                                 ------------------------------------------              ^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
    |                                                 |
    |                                                 arguments to this function are incorrect
    |
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:60:15
    |
```

```text
error[E0061]: this function takes 4 arguments but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:973:15
      |
  973 |         match M::begin_open(&mut ingress.request, grant) {
      |               ^^^^^^^^^^^^^----------------------------- two arguments of type `&mut semio_framework_job::StepContext<'_>` and `&OriginalCallerSourceParent` are missing
      |
note: expected `&mut MemberOpenSourceAdmission`, found `&mut ManuallyDrop<Option<_>>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:973:29
      |
  973 |         match M::begin_open(&mut ingress.request, grant) {
      |                             ^^^^^^^^^^^^^^^^^^^^
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:974:16
    |
973 |         match M::begin_open(&mut ingress.request, grant) {
    |               ------------------------------------------ this expression has type `Result<bool, semio_framework_job::OriginalTurnFailure>`
974 |             Ok(Some(open)) => {
    |                ^^^^^^^^^^ expected `bool`, found `Option<_>`
    |
    = note: expected type `bool`
               found enum `std::option::Option<_>`

```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:978:16
    |
973 |         match M::begin_open(&mut ingress.request, grant) {
    |               ------------------------------------------ this expression has type `Result<bool, semio_framework_job::OriginalTurnFailure>`
...
978 |             Ok(None) => Ok(PluginLifecycleStep::Progress(idle)),
    |                ^^^^
    |                |
    |                expected `bool`, found `Option<_>`
    |                `None` is interpreted as a unit variant, not a new binding
```

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:1149:56
     |
1149 |             let step=cx.with_retained_loan(child,|same|store::MemberOpenOperation::close_step(open,same))?;
     |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^      ---- argument #2 of type `&OriginalCallerSourceParent` is missing
     |
note: method defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:116:8
     |
 116 |     fn close_step(&mut self, parent:&semio_framework_job::OriginalCallerSourceParent, cx: &mut StepContext<'_>) -> Result<Retained...
     |        ^^^^^^^^^^
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:1233:156
     |
1233 | ...grant|{let step=rejected.close_step(grant);(step,rejected.terminal_is_empty())});
     |                             ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                             |
     |                             arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:33:16
     |
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:1233:164
     |
1233 | ...ed.close_step(grant);(step,rejected.terminal_is_empty())});
     |                          ^^^^ expected `InteractiveJobCloseStep`, found `Result<InteractiveJobCloseStep, _>`
     |
     = note: expected enum `InteractiveJobCloseStep`
                found enum `Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>`
help: consider using `Result::expect` to unwrap the `Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>` value, panicking if the value is a `Result::Err`
     |
1233 |             return original_rejection_lifecycle(demand,grant,"store initializer rejection awaits its original worker",|grant|{let step=rejected.close_step(grant);(step.expect("REASON"),rejected.terminal_is_empty())});
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:1281:158
     |
1281 | ...|grant|{let step=session.close_step(grant);(step,session.terminal_is_empty())});
     |                             ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                             |
     |                             arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2416:12
     |
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:1281:166
     |
1281 | ...session.close_step(grant);(step,session.terminal_is_empty())});
     |                               ^^^^ expected `WorkerJobCloseStep`, found `Result<WorkerJobCloseStep, _>`
     |
     = note: expected enum `WorkerJobCloseStep`
                found enum `Result<WorkerJobCloseStep, semio_framework_job::OriginalTurnFailure>`
help: consider using `Result::expect` to unwrap the `Result<WorkerJobCloseStep, semio_framework_job::OriginalTurnFailure>` value, panicking if the value is a `Result::Err`
     |
1281 |             return original_worker_lifecycle(demand,grant,"store initializer original terminal worker remains contended",|grant|{let step=session.close_step(grant);(step.expect("REASON"),session.terminal_is_empty())});
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🗃️document/🦀️.rs:1284:53
     |
1284 |         match session.pump_one_for_interactive_turn(grant) {
     |                       ----------------------------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                       |
     |                       arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2351:12
     |
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎞️media/🦀️.rs:127:149
    |
127 | ...|grant|{let step=rejected.close_step(grant);(step,rejected.terminal_is_empty())});
    |                              ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
    |                              |
    |                              arguments to this method are incorrect
    |
note: method defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:33:16
    |
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎞️media/🦀️.rs:127:157
    |
127 | ...ed.close_step(grant);(step,rejected.terminal_is_empty())});
    |                          ^^^^ expected `InteractiveJobCloseStep`, found `Result<InteractiveJobCloseStep, _>`
    |
    = note: expected enum `InteractiveJobCloseStep`
               found enum `Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>`
help: consider using `Result::expect` to unwrap the `Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>` value, panicking if the value is a `Result::Err`
    |
127 |             return original_rejection_lifecycle(demand,grant,"media original rejection awaits worker handback",|grant|{let step=rejected.close_step(grant);(step.expect("REASON"),rejected.terminal_is_empty())});
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎞️media/🦀️.rs:132:147
     |
 132 | ...|grant|{let step=session.close_step(grant);(step,session.terminal_is_empty())});
     |                             ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                             |
     |                             arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2416:12
     |
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎞️media/🦀️.rs:132:155
    |
132 | ...session.close_step(grant);(step,session.terminal_is_empty())});
    |                               ^^^^ expected `WorkerJobCloseStep`, found `Result<WorkerJobCloseStep, _>`
    |
    = note: expected enum `WorkerJobCloseStep`
               found enum `Result<WorkerJobCloseStep, semio_framework_job::OriginalTurnFailure>`
help: consider using `Result::expect` to unwrap the `Result<WorkerJobCloseStep, semio_framework_job::OriginalTurnFailure>` value, panicking if the value is a `Result::Err`
    |
132 |             return original_worker_lifecycle(demand,grant,"media original mounted worker awaits its handback",|grant|{let step=session.close_step(grant);(step.expect("REASON"),session.terminal_is_empty())});
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎞️media/🦀️.rs:137:190
     |
 137 | ...step=semio_framework_job::InteractiveJob::close_step(job,grant);(step,semio_framework_job::InteractiveJob::terminal_is_empty(jo...
     |         -----------------------------------------------     ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |         |
     |         arguments to this function are incorrect
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1329:8
     |
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎞️media/🦀️.rs:137:198
    |
137 | ...lose_step(job,grant);(step,semio_framework_job::InteractiveJob::terminal_is_empty(job))});
    |                          ^^^^ expected `InteractiveJobCloseStep`, found `Result<InteractiveJobCloseStep, _>`
    |
    = note: expected enum `InteractiveJobCloseStep`
               found enum `Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>`
help: consider using `Result::expect` to unwrap the `Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>` value, panicking if the value is a `Result::Err`
    |
137 |                 return original_rejection_lifecycle(demand,grant,"media original reserved producer awaits its handback",|grant|{let step=semio_framework_job::InteractiveJob::close_step(job,grant);(step.expect("REASON"),semio_framework_job::InteractiveJob::terminal_is_empty(job))});
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🦀️.rs:940:83
    |
940 |     let mut session = match crate::reserved_job_session::admit_batch(job, params, step_grant) {
    |                             ----------------------------------------              ^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
    |                             |
    |                             arguments to this function are incorrect
    |
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:65:15
    |
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🦀️.rs:947:44
    |
947 |                 let _ = refused.close_step(close_grant);
    |                                 ---------- ^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
    |                                 |
    |                                 arguments to this method are incorrect
    |
note: method defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:33:16
    |
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🦀️.rs:1005:36
     |
1005 |                 session.close_step(close_grant);
     |                         ---------- ^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                         |
     |                         arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2573:12
     |
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1232:44
     |
1232 |     if pump.closing {let _=pump.close_step(grant);return RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::AlreadyClosing);}
     |                                 ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                 |
     |                                 arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1018:8
     |
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1235:35
     |
1235 | ...   match rejected.close_step(grant).admit(grant,rejected.terminal_is_empty()){semio_framework_job::InteractiveJobCloseStep::Ref...
     |                      ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                      |
     |                      arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:33:16
     |
```

```text
error[E0599]: no method named `admit` found for enum `Result<T, E>` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1235:42
     |
1235 | ..._step(grant).admit(grant,rejected.terminal_is_empty()){semio_framework_job::InteractiveJobCloseStep::Refused{..}=>return Runtim...
     |                 ^^^^^ method not found in `Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>`
     |
note: the method `admit` exists on the type `InteractiveJobCloseStep`
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1298:5
     |
1298 |     pub fn admit(self,grant:RetainedCloneGrant,terminal_is_empty:bool)->Result<Self,ValueError>{
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1235:130
     |
1235 | ...ework_job::InteractiveJobCloseStep::Refused{..}=>return RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::PriorOutcome),_=>r...
     |                                        ^^^^^^^ variant not found in `InteractiveJobCloseStep`

error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1258:34
     |
1258 | ...   match session.close_step(grant){semio_framework_job::WorkerJobCloseStep::Refused{..}=>return RuntimeMaintenanceStatus::Fault...
     |                     ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1258:34
     |
1258 | ...   match session.close_step(grant){semio_framework_job::WorkerJobCloseStep::Refused{..}=>return RuntimeMaintenanceStatus::Fault...
     |                     ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                     |
     |                     arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2573:12
     |
```

```text
error[E0599]: no variant named `Refused` found for enum `WorkerJobCloseStep`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1258:82
     |
1258 | ...mio_framework_job::WorkerJobCloseStep::Refused{..}=>return RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::PriorOutcome),_...
     |                                           ^^^^^^^ variant not found in `WorkerJobCloseStep`

error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1269:69
     |
1269 |         match crate::reserved_job_session::admit_batch(job, params, cell.mounted_policy.preparation) {
     |               ----------------------------------------              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1269:69
     |
1269 |         match crate::reserved_job_session::admit_batch(job, params, cell.mounted_policy.preparation) {
     |               ----------------------------------------              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |               |
     |               arguments to this function are incorrect
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:65:15
     |
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1604:42
     |
1604 | ...   return match rejected.close_step(grant).admit(grant,rejected.terminal_is_empty()){semio_framework_job::InteractiveJobCloseSt...
     |                             ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                             |
     |                             arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:33:16
     |
```

```text
error[E0599]: no method named `admit` found for enum `Result<T, E>` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1604:49
     |
1604 | ..._step(grant).admit(grant,rejected.terminal_is_empty()){semio_framework_job::InteractiveJobCloseStep::Refused{..}=>runtime_close...
     |                 ^^^^^ method not found in `Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>`
     |
note: the method `admit` exists on the type `InteractiveJobCloseStep`
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1298:5
     |
1298 |     pub fn admit(self,grant:RetainedCloneGrant,terminal_is_empty:bool)->Result<Self,ValueError>{
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1604:137
     |
1604 | ...ework_job::InteractiveJobCloseStep::Refused{..}=>runtime_close_fault(state,RuntimeCleanupFault::PriorOutcome),_=>RuntimeCloseSt...
     |                                        ^^^^^^^ variant not found in `InteractiveJobCloseStep`

error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1629:73
     |
1629 | ...y(){return match session.close_step(grant){semio_framework_job::WorkerJobCloseStep::Refused{..}=>runtime_close_fault(state,Runt...
     |                             ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1629:73
     |
1629 | ...y(){return match session.close_step(grant){semio_framework_job::WorkerJobCloseStep::Refused{..}=>runtime_close_fault(state,Runt...
     |                             ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                             |
     |                             arguments to this method are incorrect
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2573:12
     |
```

```text
error[E0599]: no variant named `Refused` found for enum `WorkerJobCloseStep`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1629:121
     |
1629 | ...mio_framework_job::WorkerJobCloseStep::Refused{..}=>runtime_close_fault(state,RuntimeCleanupFault::PriorOutcome),_=>RuntimeClos...
     |                                           ^^^^^^^ variant not found in `WorkerJobCloseStep`

error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1644:69
     |
1644 |         match crate::reserved_job_session::admit_batch(job, params, state.mounted_policy.preparation) {
     |               ----------------------------------------              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:1644:69
     |
1644 |         match crate::reserved_job_session::admit_batch(job, params, state.mounted_policy.preparation) {
     |               ----------------------------------------              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |               |
     |               arguments to this function are incorrect
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../👷️job/🚪️session/🦀️.rs:65:15
     |
```

```text
error[E0609]: no field `kind` on type `OriginalTurnPrimary`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs:83:72
   |
83 |  let progress=error.primary.retained_progress();let code=error.primary.kind.as_str();let mut fault=Fault::new(FaultOrigin::Framework...
   |                                                                        ^^^^ unknown field

error[E0599]: no method named `into_message` found for enum `OriginalTurnPrimary` in the current scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs:83:153
   |
83 | ...ault=Fault::new(FaultOrigin::Framework,code,error.primary.into_message()).with_retained_progress(progress);
   |                                                              ^^^^^^^^^^^^
```

```text
error[E0599]: no method named `into_message` found for enum `OriginalTurnPrimary` in the current scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs:83:153
   |
83 | ...ault=Fault::new(FaultOrigin::Framework,code,error.primary.into_message()).with_retained_progress(progress);
   |                                                              ^^^^^^^^^^^^
   |
note: the method `into_message` exists on the type `semio_framework_value::ValueError`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../⚠️refusal/🦀️.rs:30:5
   |
30 |     pub fn into_message(self) -> String { self.message.into_owned() }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/../🪜️maintenance-ladder/🦀️.rs:337:87
    |
337 |             let step=self.media_closures.get_mut(operation_id).ok_or_else(authority)?.close_step(cx,grant)?;
    |                                                                                       ^^^^^^^^^^ -- unexpected argument #1 of type `&mut semio_framework_job::StepContext<'_>`
    |
note: method defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎞️media/🦀️.rs:115:19
    |
115 |     pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
    |                   ^^^^^^^^^^
```


These are ongoing source migrations and not domain Diagnostic regressions. No complete renderer/native/DOM/GPU qualification follows.
