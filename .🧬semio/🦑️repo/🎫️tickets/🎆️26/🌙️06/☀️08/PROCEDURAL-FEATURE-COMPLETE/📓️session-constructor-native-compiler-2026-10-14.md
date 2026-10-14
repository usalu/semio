## Original Session Constructor Native Compiler Floor

Registered94068 naturally Nx1 after2m7, source1pass/0fail/23assert GREEN. Native selected laws unrun: actual Plugin production28 errors precede Session compilation. Original compiler JSON SHA256 56be045c1fdc170b8daeff67b3485f7b6a6bd30731e3555125e02ac35e5a6e15.

```text
error[E0726]: implicit elided lifetime not allowed here
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:439:117
    |
439 |       async fn render(&mut self, body_key: &str, snapshot_override_json: Option<&str>, view_state: &ViewModel,cx:&mut semio_framework_job::StepContext) -...
    |                                                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
...
532 | / dyn_enum_close! {
533 | |     /// 🗃️ sdk-dedyn (O1/§1.5): the default-composes-nothing case (design-dedyn.md §1.6's `NoMembers`
534 | |     /// pattern, applied here to `PluginApp`) — a zero-variant enum, `dyn_enum_close!`-generated
535 | |     /// (every method's body degenerates to `match *self {}` since there is no value to construct).
...   |
538 | |     pub enum NoPluginApp: PluginApp {}
539 | | }
    | |_- in this macro invocation
    |
    = note: this error originates in the macro `__semio_dispatch_PluginApp` which comes from the expansion of the macro `dyn_enum_close` (in Nightly builds, run with -Z macro-backtrace for more info)
help: indicate the anonymous lifetime
    |
439 |     async fn render(&mut self, body_key: &str, snapshot_override_json: Option<&str>, view_state: &ViewModel,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<ComponentTree>,semio_framework_job::OriginalTurnFailure>;
    |                                                                                                                                                     ++++


error[E0726]: implicit elided lifetime not allowed here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:2691:121
     |
2691 | ...ance_id:u32,surface:&str,cx:&mut semio_framework_job::StepContext)->Result<Option<(ComponentTree,Vec<ui_contract::PresenceUpdat...
     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2691 | pub(crate) async fn plugin_render_surface<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,surface:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<(ComponentTree,Vec<ui_contract::PresenceUpdate>)>,semio_framework_job::OriginalTurnFailure>{
     |                                                                                                                                                         ++++


error[E0726]: implicit elided lifetime not allowed here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:2704:128
     |
2704 | ...str,view_state_json:&str,cx:&mut semio_framework_job::StepContext)->Result<Option<ComponentTree>,semio_framework_job::OriginalT...
     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2704 | pub async fn plugin_render<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,body_key:&str,view_state_json:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<ComponentTree>,semio_framework_job::OriginalTurnFailure>{
     |                                                                                                                                                                ++++


error[E0726]: implicit elided lifetime not allowed here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:2707:178
     |
2707 | ...tr>,view_state_json:&str,cx:&mut semio_framework_job::StepContext)->Result<Option<ComponentTree>,semio_framework_job::OriginalT...
     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2707 | pub async fn plugin_render_with_document<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,body_key:&str,snapshot_override_json:Option<&str>,view_state_json:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<ComponentTree>,semio_framework_job::OriginalTurnFailure>{
     |                                                                                                                                                                                                                  ++++


error[E0726]: implicit elided lifetime not allowed here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:2798:121
     |
2798 | ... u32, request_json: &str,cx:&mut semio_framework_job::StepContext) -> Result<Option<String>,semio_framework_job::OriginalTurnFa...
     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2798 | pub async fn plugin_refresh_ui<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, request_json: &str,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<String>,semio_framework_job::OriginalTurnFailure> {
     |                                                                                                                                                         ++++


error[E0726]: implicit elided lifetime not allowed here
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:10130:117
      |
10130 | ..., view_state: &ViewModel,cx:&mut semio_framework_job::StepContext) -> Result<Option<ComponentTree>,semio_framework_job::Origin...
      |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
      |
help: indicate the anonymous lifetime
      |
10130 |     async fn render(&mut self, body_key: &str, snapshot_override_json: Option<&str>, view_state: &ViewModel,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<ComponentTree>,semio_framework_job::OriginalTurnFailure> {
      |                                                                                                                                                     ++++


error[E0433]: cannot find type `ValueError` in this scope
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1532:78
     |
1532 | ...   PATCHES.with(|patches|patches.defer(surface)).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::Invari...
     |                                                                ^^^^^^^^^^ use of undeclared type `ValueError`
     |
help: consider importing one of these structs
     |
   1 + use crate::plugin_app_close_prelude::ValueError;
     |
   1 + use replication::ValueError;
     |
   1 + use semio_framework_value::ValueError;
     |


error[E0596]: cannot borrow `quarantine` as mutable, as it is not declared as mutable
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:931:42
    |
931 |  if !instances.can_insert(instance_id)||!quarantine.can_insert(instance_id)||!actors.can_insert(instance_id)||!document_backbones.c...
    |                                          ^^^^^^^^^^ cannot borrow as mutable
    |
help: consider changing this to be mutable
    |
930 |  let mut instances=runtime.instances.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime instance authority busy"))?;let mut quarantine=runtime.close_quarantine.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime quarantine authority busy"))?;let mut actors=runtime.instance_actors.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime actor authority busy"))?;let document_backbones=runtime.document_backbones.try_borrow_mut().map_err(|_|plugin_internal_fault("document backbone binding authority busy"))?;
    |                                                                                                                                 +++


error[E0596]: cannot borrow `document_backbones` as mutable, as it is not declared as mutable
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:931:112
    |
931 | ...ce_id)||!actors.can_insert(instance_id)||!document_backbones.can_insert(instance_id){return Err(plugin_internal_fault("native op...
    |                                              ^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
    |
help: consider changing this to be mutable
    |
930 |  let mut instances=runtime.instances.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime instance authority busy"))?;let quarantine=runtime.close_quarantine.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime quarantine authority busy"))?;let mut actors=runtime.instance_actors.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime actor authority busy"))?;let mut document_backbones=runtime.document_backbones.try_borrow_mut().map_err(|_|plugin_internal_fault("document backbone binding authority busy"))?;
    |                                                                                                                                                                                                                                                                                                                                                                                             +++


error[E0061]: this method takes 7 arguments but 6 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:2172:75
     |
2172 | ...::resolve_ready(instance.app.handle_command(&mut invocation, None, active_mode_id, &meta, identity, cx))
     |                                 ^^^^^^^^^^^^^^ ---------------  --------------------
     |                                                |                |     |
     |                                                |                |     unexpected argument #3 of type `std::option::Option<&str>`
     |                                                |                two arguments of type `&mut std::option::Option<semio_framework_value::DslValue>` and `&mut std::option::Option<semio_framework_dsl::Fault>` are missing
     |                                                |                expected `std::option::Option<&str>`, found `std::option::Option<_>`
     |                                                expected `&CommandAddress`, found `&mut CommandInvocation`
     |
     = note:      expected reference `&CommandAddress`
             found mutable reference `&mut semio_framework::manifest::CommandInvocation`
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:249:14
     |
 249 |     async fn handle_command(&mut self, address: &ManifestCommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut ...
     |              ^^^^^^^^^^^^^^            -------                           -----------                         --------------
help: did you mean
     |
2172 -                 ::semio_framework_async::poll::resolve_ready(instance.app.handle_command(&mut invocation, None, active_mode_id, &meta, identity, cx))
2172 +                 ::semio_framework_async::poll::resolve_ready(instance.app.handle_command(/* &CommandAddress */, /* &mut std::option::Option<semio_framework_value::DslValue> */, /* &mut std::option::Option<semio_framework_dsl::Fault> */, None, &meta, identity, cx))
     |


error[E0308]: `if` and `else` have incompatible types
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:4165:29
     |
4160 | /                         if !owner_matches {
4161 | |                             Err(plugin_internal_fault(format!("action plugin owner {} does not match the active program", invocation.address.plugin_id)))
     | |                             ----------------------------------------------------------------------------------------------------------------------------- expected because of this
4162 | |                         } else {
4163 | |                             let cell = runtime_instance_cell(runtime, instance_id)?;
4164 | |                             let mut instance = cell.host_instance().ok_or_else(|| plugin_internal_fault(format!("instance busy or poisoned: {instance_id}")))?;
4165 | |                             drive_self_waking_ready(instance.app.handle_action_invocation(&invocation, active_mode_id.as_deref(), &meta, identity, cx))
     | |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<_, Fault>`, found `Result<Option<InvocationResult>, _>`
4166 | |                         }
     | |_________________________- `if` and `else` have incompatible types
     |
     = note: expected enum `Result<_, semio_framework_dsl::Fault>`
                found enum `Result<std::option::Option<semio_framework::InvocationResult>, semio_framework_job::OriginalTurnFailure>`


error[E0061]: this method takes 7 arguments but 6 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:4185:103
     |
4185 | ...::resolve_ready(instance.app.handle_command(&mut invocation, None, active_mode_id, &meta, identity, cx))
     |                                 ^^^^^^^^^^^^^^ ---------------  --------------------
     |                                                |                |     |
     |                                                |                |     unexpected argument #3 of type `std::option::Option<&str>`
     |                                                |                two arguments of type `&mut std::option::Option<semio_framework_value::DslValue>` and `&mut std::option::Option<semio_framework_dsl::Fault>` are missing
     |                                                |                expected `std::option::Option<&str>`, found `std::option::Option<_>`
     |                                                expected `&CommandAddress`, found `&mut CommandInvocation`
     |
     = note:      expected reference `&CommandAddress`
             found mutable reference `&mut semio_framework::manifest::CommandInvocation`
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:249:14
     |
 249 |     async fn handle_command(&mut self, address: &ManifestCommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut ...
     |              ^^^^^^^^^^^^^^            -------                           -----------                         --------------
help: did you mean
     |
4185 -                                             ::semio_framework_async::poll::resolve_ready(instance.app.handle_command(&mut invocation, None, active_mode_id, &meta, identity, cx))
4185 +                                             ::semio_framework_async::poll::resolve_ready(instance.app.handle_command(/* &CommandAddress */, /* &mut std::option::Option<semio_framework_value::DslValue> */, /* &mut std::option::Option<semio_framework_dsl::Fault> */, None, &meta, identity, cx))
     |


error[E0308]: mismatched types
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:4185:45
     |
4185 | ...   ::semio_framework_async::poll::resolve_ready(instance.app.handle_command(&mut invocation, None, active_mode_id, &meta, identity, cx))
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<_, Fault>`, found `Result<Option<InvocationResult>, _>`
     |
     = note: expected enum `Result<_, semio_framework_dsl::Fault>`
                found enum `Result<std::option::Option<semio_framework::InvocationResult>, semio_framework_job::OriginalTurnFailure>`
note: return type inferred to be `Result<_, semio_framework_dsl::Fault>` here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:4184:64
     |
4184 | ...                   let mut instance = find_instance(list, instance_id)?;
     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0308]: mismatched types
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:4265:85
     |
4265 |                         drive_self_waking_ready(instance.app.handle_action(&action, args, &meta, identity, cx))
     |                                                              -------------          ^^^^ expected `&mut Option<DslValue>`, found `Option<DslValue>`
     |                                                              |
     |                                                              arguments to this method are incorrect
     |
     = note: expected mutable reference `&mut Option<_>`
                             found enum `Option<_>`
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:226:14
     |
 226 |     async fn handle_action(&mut self, action: &str, args: &mut Option<DslValue>, meta: &ActionMeta, identity: &mut semio_framework...
     |              ^^^^^^^^^^^^^                          ----
help: consider mutably borrowing here
     |
4265 |                         drive_self_waking_ready(instance.app.handle_action(&action, &mut args, &meta, identity, cx))
     |                                                                                     ++++


error[E0308]: mismatched types
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:4265:25
     |
4265 |                         drive_self_waking_ready(instance.app.handle_action(&action, args, &meta, identity, cx))
     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<_, Fault>`, found `Result<Option<InvocationResult>, _>`
     |
     = note: expected enum `Result<_, semio_framework_dsl::Fault>`
                found enum `Result<std::option::Option<semio_framework::InvocationResult>, semio_framework_job::OriginalTurnFailure>`
note: return type inferred to be `Result<_, semio_framework_dsl::Fault>` here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:4264:44
     |
4264 |                         let mut instance = find_instance(list, instance_id)?;
     |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0308]: arguments to this function are incorrect
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1019:38
     |
1019 | ...   let output = crate::plugin_runtime::plugin_dispatch_response_action(runtime, instance, &action, args, original_fault, identi...
     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: expected `&mut Option<DslValue>`, found `DslValue`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1019:121
     |
1019 | ...   let output = crate::plugin_runtime::plugin_dispatch_response_action(runtime, instance, &action, args, original_fault, identi...
     |                                                                                                       ^^^^
     = note: expected mutable reference `&mut std::option::Option<DslValue>`
                             found enum `DslValue`
note: expected `&mut Option<Fault>`, found `Option<Fault>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1019:127
     |
1019 | ...   let output = crate::plugin_runtime::plugin_dispatch_response_action(runtime, instance, &action, args, original_fault, identi...
     |                                                                                                             ^^^^^^^^^^^^^^
     = note: expected mutable reference `&mut Option<_>`
                             found enum `Option<_>`
note: function defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🏃️runtime/🦀️.rs:3600:14
     |
3600 | ...fn plugin_dispatch_response_action<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, i...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                                                                             ---------------------------  ----------------------------------
help: consider mutably borrowing here
     |
1019 |                         let output = crate::plugin_runtime::plugin_dispatch_response_action(runtime, instance, &action, args, &mut original_fault, identity, cx).await;
     |                                                                                                                               ++++


error[E0609]: no field `frames` on type `Result<std::option::Option<PluginExchangeOutput>, semio_framework_job::OriginalTurnFailure>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1020:51
     |
1020 |                         for frame_bytes in output.frames {
     |                                                   ^^^^^^ unknown field


error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1020:29
     |
1020 |                         for frame_bytes in output.frames {
     |                             ^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `[u8]`
     = note: all local variables must have a statically known size


error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1020:25
     |
1020 | /                         for frame_bytes in output.frames {
1021 | |                             route_app_frame(instance, &frame_bytes, &mut document_backbone_effects);
1022 | |                         }
     | |_________________________^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `[u8]`
note: required by an implicit `Sized` bound in `std::option::Option`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:598:17
     |
 598 | pub enum Option<T> {
     |                 ^ required by the implicit `Sized` requirement on this type parameter in `Option`


error[E0609]: no field `effects` on type `Result<std::option::Option<PluginExchangeOutput>, semio_framework_job::OriginalTurnFailure>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1023:44
     |
1023 |                         for one in &output.effects {
     |                                            ^^^^^^^ unknown field


error[E0609]: no field `events` on type `Result<std::option::Option<PluginExchangeOutput>, semio_framework_job::OriginalTurnFailure>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1028:44
     |
1028 |                         for one in &output.events {
     |                                            ^^^^^^ unknown field


error[E0609]: no field `frames` on type `std::option::Option<PluginExchangeOutput>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1441:43
     |
1441 |                 for frame_bytes in output.frames {
     |                                           ^^^^^^ unknown field
     |
help: one of the expressions' fields has a field of the same name
     |
1441 |                 for frame_bytes in output.unwrap().frames {
     |                                           +++++++++


error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1441:21
     |
1441 |                 for frame_bytes in output.frames {
     |                     ^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `[u8]`
     = note: all local variables must have a statically known size


error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1441:17
     |
1441 | /                 for frame_bytes in output.frames {
1442 | |                     route_app_frame(instance, &frame_bytes, &mut effects);
1443 | |                 }
     | |_________________^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `[u8]`
note: required by an implicit `Sized` bound in `std::option::Option`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:598:17
     |
 598 | pub enum Option<T> {
     |                 ^ required by the implicit `Sized` requirement on this type parameter in `Option`


error[E0609]: no field `effects` on type `std::option::Option<PluginExchangeOutput>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1444:36
     |
1444 |                 for one in &output.effects {
     |                                    ^^^^^^^ unknown field
     |
help: one of the expressions' fields has a field of the same name
     |
1444 |                 for one in &output.unwrap().effects {
     |                                    +++++++++


error[E0609]: no field `events` on type `std::option::Option<PluginExchangeOutput>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1449:36
     |
1449 |                 for one in &output.events {
     |                                    ^^^^^^ unknown field
     |
help: one of the expressions' fields has a field of the same name
     |
1449 |                 for one in &output.unwrap().events {
     |                                    +++++++++


error[E0308]: mismatched types
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1455:69
     |
1455 |             Err(fault) => effects.push(shell_fault_effect(instance, &fault)),
     |                                        ------------------           ^^^^^^ expected `&Fault`, found `&OriginalTurnFailure`
     |                                        |
     |                                        arguments to this function are incorrect
     |
     = note: expected reference `&semio_framework_dsl::Fault`
                found reference `&semio_framework_job::OriginalTurnFailure`
note: function defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🦀️.rs:75:4
     |
  75 | fn shell_fault_effect(instance: u32, fault: &semio_framework::Fault) -> Effect {
     |    ^^^^^^^^^^^^^^^^^^                ------------------------------


error[E0499]: cannot borrow `source` as mutable more than once at a time
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6776:190
     |
6776 | ...nd=A::command_from_action(command_id,if source.assembled.is_some(){&mut source.assembled}else if source.prepared.is_some(){&mut source.prepared}else{&mut source.args},&mut source.fa...
     |       ---------------------- first borrow later used by call               ------ first mutable borrow occurs here                                                             ^^^^^^ second mutable borrow occurs here


```
