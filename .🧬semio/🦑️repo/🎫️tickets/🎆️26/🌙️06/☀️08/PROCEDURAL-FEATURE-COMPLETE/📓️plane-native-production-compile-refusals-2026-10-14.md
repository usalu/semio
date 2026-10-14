# Plane Native Production Compile Refusals

Actual registered native4017/session23996 reached owning production component assembly compilation, not Plugin lib-test fixtures. Strict TypeScript and source widget checks ran; native assertions did not. Actual Plugin library refused with30 errors. No native widget or browser qualification is claimed.

```text
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/../🧪️tests/🧪️history-edit-acceptance/🦀️.rs:384:56
    |
384 | ...g_identity!(|identity| artifact_app_laws::load_document_text(&mut app, &files, &mut identity).await).map_err(|fault| AcceptanceS...
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------------------------------- argument #4 of type `&mut semio_framework_job::StepContext<'_>` is missing
    |
note: function defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:464:18
    |
464 | ...fn load_document_text<A, M>(app: &mut VcsArtifactApp<A, M>, files: &store::os_store::text_format::ArtifactTextFiles, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -...
    |       ^^^^^^^^^^^^^^^^^^                                                                                                                                                                                                                    --------------------------------------------
help: provide the argument
    |
384 |             crate::with_authoring_identity!(|identity| artifact_app_laws::load_document_text(&mut app, &files, &mut identity, /* &mut semio_framework_job::StepContext<'_> */).await).map_err(|fault| AcceptanceSeedFault::Base(format!("the base document does not load: {fault:?}")))?;
    |                                                                                                                             +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 5 arguments but 4 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1223:28

error[E0061]: this method takes 5 arguments but 4 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1223:28
     |
1223 |         let admitted = app.handle_action(action, None, &meta("local"), identity).await.unwrap_or_else(|fault| panic!("history verb...
     |                            ^^^^^^^^^^^^^---------------------------------------- argument #5 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:223:14
     |
 223 | ...fn handle_action(&mut self, action: &str, args: Option<DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&m...
     |       ^^^^^^^^^^^^^                                                                                                                                                                                          --
help: provide the argument
     |
1223 |         let admitted = app.handle_action(action, None, &meta("local"), identity, /* &mut semio_framework_job::StepContext<'_> */).await.unwrap_or_else(|fault| panic!("history verb {action} admission: {fault:?}"));
     |                                                                                +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1244:13

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1244:13
     |
1244 |         app.dispatch_typed(command, &meta("local"), identity).await.expect("apply command");
     |             ^^^^^^^^^^^^^^----------------------------------- argument #4 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:8725:12
     |
8725 | ...fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -...
     |       ^^^^^^^^^^^^^^                                                                                                                                                                                                   -----------------------------------------------------
help: provide the argument
     |
1244 |         app.dispatch_typed(command, &meta("local"), identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("apply command");
     |                                                             +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 5 arguments but 4 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1663:30

error[E0061]: this method takes 5 arguments but 4 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1663:30
     |
1663 | ...let result = app.handle_action("setActiveExample", Some(boot.args.clone()), &action_meta, identity).await.unwrap_or_else(|fault...
     |                     ^^^^^^^^^^^^^--------------------------------------------------------------------- argument #5 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:223:14
     |
 223 | ...fn handle_action(&mut self, action: &str, args: Option<DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&m...
     |       ^^^^^^^^^^^^^                                                                                                                                                                                          --
help: provide the argument
     |
1663 |             let result = app.handle_action("setActiveExample", Some(boot.args.clone()), &action_meta, identity, /* &mut semio_framework_job::StepContext<'_> */).await.unwrap_or_else(|fault| panic!("the app's own boot example must be admitted: {fault:?}"));
     |                                                                                                               +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 5 arguments but 4 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1728:33

error[E0061]: this method takes 5 arguments but 4 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1728:33
     |
1728 |         let outcome = match app.handle_action(verb, Some(args.clone()), &action_meta, identity).await {
     |                                 ^^^^^^^^^^^^^-------------------------------------------------- argument #5 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:223:14
     |
 223 | ...fn handle_action(&mut self, action: &str, args: Option<DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&m...
     |       ^^^^^^^^^^^^^                                                                                                                                                                                          --
help: provide the argument
     |
1728 |         let outcome = match app.handle_action(verb, Some(args.clone()), &action_meta, identity, /* &mut semio_framework_job::StepContext<'_> */).await {
     |                                                                                               +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1799:26

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1799:26
     |
1799 |         let result = app.preview_addressed_action(&invocation, &action_meta).await?;
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^--------------------------- argument #3 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6797:25
     |
6797 | ...fn preview_addressed_action(&mut self, invocation: &ManifestActionInvocation, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^                                                                      --------------------------------------------
help: provide the argument
     |
1799 |         let result = app.preview_addressed_action(&invocation, &action_meta, /* &mut semio_framework_job::StepContext<'_> */).await?;
     |                                                                            +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0277]: `?` couldn't convert the error to `semio_framework_dsl::Fault`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1799:83

error[E0277]: `?` couldn't convert the error to `semio_framework_dsl::Fault`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1799:83
     |
1799 |         let result = app.preview_addressed_action(&invocation, &action_meta).await?;
     |                      -------------------------------------------------------------^ the trait `From<semio_framework_job::OriginalTurnFailure>` is not implemented for `semio_framework_dsl::Fault`
     |                      |
     |                      this can't be annotated with `?` because it has type `Result<_, semio_framework_job::OriginalTurnFailure>`
     |
     = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
help: `semio_framework_dsl::Fault` implements trait `From<T>`
    --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/../../🦀️.rs:651:1
     |
 651 | impl From<&str> for Fault {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^ `From<&str>`
...
 657 | impl From<String> for Fault {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `From<std::string::String>`


error[E0609]: no field `output` on type `std::option::Option<semio_framework::InvocationResult>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1800:19
     |
1800 | ...   Ok(result.output.get("documentOps").and_then(|value| value.as_str()).and_then(|value| value.parse::<usize>().ok()).unwrap_or...
     |                 ^^^^^^ unknown field
     |
help: one of the expressions' fields has a field of the same name
     |
1800 |         Ok(result.unwrap().output.get("documentOps").and_then(|value| value.as_str()).and_then(|value| value.parse::<usize>().ok()).unwrap_or(0))
     |                   +++++++++

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1833:33
     |
1833 |         let outcome = match app.preview_addressed_action(&invocation, &action_meta).await {
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^--------------------------- argument #3 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1833:33
     |
1833 |         let outcome = match app.preview_addressed_action(&invocation, &action_meta).await {
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^--------------------------- argument #3 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6797:25
     |
6797 | ...fn preview_addressed_action(&mut self, invocation: &ManifestActionInvocation, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^                                                                      --------------------------------------------
help: provide the argument
     |
1833 |         let outcome = match app.preview_addressed_action(&invocation, &action_meta, /* &mut semio_framework_job::StepContext<'_> */).await {
     |                                                                                   +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0609]: no field `output` on type `std::option::Option<semio_framework::InvocationResult>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1836:48

error[E0609]: no field `output` on type `std::option::Option<semio_framework::InvocationResult>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1836:48
     |
1836 | ...   let count = |key: &str| result.output.get(key).and_then(|value| value.as_str()).and_then(|value| value.parse::<usize>().ok()...
     |                                      ^^^^^^ unknown field
     |
help: one of the expressions' fields has a field of the same name
     |
1836 |                 let count = |key: &str| result.unwrap().output.get(key).and_then(|value| value.as_str()).and_then(|value| value.parse::<usize>().ok()).unwrap_or(0);
     |                                                +++++++++

error[E0609]: no field `requested_effects` on type `std::option::Option<semio_framework::InvocationResult>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1839:43
     |
1839 |                 let host_effects = result.requested_effects.len() + result.events.len();
     |                                           ^^^^^^^^^^^^^^^^^ unknown field
     |
help: one of the expressions' fields has a field of the same name

error[E0609]: no field `requested_effects` on type `std::option::Option<semio_framework::InvocationResult>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1839:43
     |
1839 |                 let host_effects = result.requested_effects.len() + result.events.len();
     |                                           ^^^^^^^^^^^^^^^^^ unknown field
     |
help: one of the expressions' fields has a field of the same name
     |
1839 |                 let host_effects = result.unwrap().requested_effects.len() + result.events.len();
     |                                           +++++++++

error[E0609]: no field `events` on type `std::option::Option<semio_framework::InvocationResult>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1839:76
     |
1839 |                 let host_effects = result.requested_effects.len() + result.events.len();
     |                                                                            ^^^^^^ unknown field
     |
help: one of the expressions' fields has a field of the same name

error[E0609]: no field `events` on type `std::option::Option<semio_framework::InvocationResult>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1839:76
     |
1839 |                 let host_effects = result.requested_effects.len() + result.events.len();
     |                                                                            ^^^^^^ unknown field
     |
help: one of the expressions' fields has a field of the same name
     |
1839 |                 let host_effects = result.requested_effects.len() + result.unwrap().events.len();
     |                                                                            +++++++++

error[E0609]: no field `output` on type `std::option::Option<semio_framework::InvocationResult>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1840:85
     |
1840 |                 let fingerprint = declared_verb_fingerprint([format!("{:?}", result.output).as_bytes()]);
     |                                                                                     ^^^^^^ unknown field
     |
help: one of the expressions' fields has a field of the same name

error[E0609]: no field `output` on type `std::option::Option<semio_framework::InvocationResult>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1840:85
     |
1840 |                 let fingerprint = declared_verb_fingerprint([format!("{:?}", result.output).as_bytes()]);
     |                                                                                     ^^^^^^ unknown field
     |
help: one of the expressions' fields has a field of the same name
     |
1840 |                 let fingerprint = declared_verb_fingerprint([format!("{:?}", result.unwrap().output).as_bytes()]);
     |                                                                                     +++++++++

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2024:20
     |
2024 |         instance_a.dispatch_typed(command_a, &meta("actor-a"), identity).await.expect("a applies its edit");
     |                    ^^^^^^^^^^^^^^--------------------------------------- argument #4 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2024:20
     |
2024 |         instance_a.dispatch_typed(command_a, &meta("actor-a"), identity).await.expect("a applies its edit");
     |                    ^^^^^^^^^^^^^^--------------------------------------- argument #4 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:8725:12
     |
8725 | ...fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -...
     |       ^^^^^^^^^^^^^^                                                                                                                                                                                                   -----------------------------------------------------
help: provide the argument
     |
2024 |         instance_a.dispatch_typed(command_a, &meta("actor-a"), identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("a applies its edit");
     |                                                                        +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2025:20

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2025:20
     |
2025 |         instance_b.dispatch_typed(command_b, &meta("actor-b"), identity).await.expect("b applies its edit");
     |                    ^^^^^^^^^^^^^^--------------------------------------- argument #4 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:8725:12
     |
8725 | ...fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -...
     |       ^^^^^^^^^^^^^^                                                                                                                                                                                                   -----------------------------------------------------
help: provide the argument
     |
2025 |         instance_b.dispatch_typed(command_b, &meta("actor-b"), identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("b applies its edit");
     |                                                                        +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2063:20

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2063:20
     |
2063 |         instance_a.dispatch_typed(command_a, &meta("actor-a"), identity).await.expect("a applies its edit");
     |                    ^^^^^^^^^^^^^^--------------------------------------- argument #4 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:8725:12
     |
8725 | ...fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -...
     |       ^^^^^^^^^^^^^^                                                                                                                                                                                                   -----------------------------------------------------
help: provide the argument
     |
2063 |         instance_a.dispatch_typed(command_a, &meta("actor-a"), identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("a applies its edit");
     |                                                                        +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2065:20

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2065:20
     |
2065 |         instance_b.dispatch_typed(command_b, &meta("actor-b"), identity).await.expect("b applies its edit");
     |                    ^^^^^^^^^^^^^^--------------------------------------- argument #4 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:8725:12
     |
8725 | ...fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -...
     |       ^^^^^^^^^^^^^^                                                                                                                                                                                                   -----------------------------------------------------
help: provide the argument
     |
2065 |         instance_b.dispatch_typed(command_b, &meta("actor-b"), identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("b applies its edit");
     |                                                                        +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2088:20

error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2088:20
     |
2088 |         instance_a.tick_backbone(identity).await.expect("a folds b's events");
     |                    ^^^^^^^^^^^^^---------- argument #2 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:429:14
     |
 429 | ...fn tick_backbone(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&m...
     |       ^^^^^^^^^^^^^                                                                                                                                 --
help: provide the argument
     |
2088 |         instance_a.tick_backbone(identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("a folds b's events");
     |                                          +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2089:20

error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2089:20
     |
2089 |         instance_b.tick_backbone(identity).await.expect("b folds a's events");
     |                    ^^^^^^^^^^^^^---------- argument #2 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:429:14
     |
 429 | ...fn tick_backbone(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&m...
     |       ^^^^^^^^^^^^^                                                                                                                                 --
help: provide the argument
     |
2089 |         instance_b.tick_backbone(identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("b folds a's events");
     |                                          +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 5 arguments but 4 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2092:35

error[E0061]: this method takes 5 arguments but 4 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2092:35
     |
2092 | ...et admitted = instance_a.handle_action("commitCheckpoint", None, &meta(actor_a), identity).await.expect("a commits a checkpoint");
     |                             ^^^^^^^^^^^^^---------------------------------------------------- argument #5 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:223:14
     |
 223 | ...fn handle_action(&mut self, action: &str, args: Option<DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&m...
     |       ^^^^^^^^^^^^^                                                                                                                                                                                          --
help: provide the argument
     |
2092 |         let admitted = instance_a.handle_action("commitCheckpoint", None, &meta(actor_a), identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("a commits a checkpoint");
     |                                                                                                   +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2095:20

error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2095:20
     |
2095 |         instance_b.tick_backbone(identity).await.expect("b folds a's checkpoint");
     |                    ^^^^^^^^^^^^^---------- argument #2 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:429:14
     |
 429 | ...fn tick_backbone(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&m...
     |       ^^^^^^^^^^^^^                                                                                                                                 --
help: provide the argument
     |
2095 |         instance_b.tick_backbone(identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("b folds a's checkpoint");
     |                                          +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2119:16

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2119:16
     |
2119 |         sender.dispatch_typed(command, &meta("local"), identity).await.expect("apply command");
     |                ^^^^^^^^^^^^^^----------------------------------- argument #4 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:8725:12
     |
8725 | ...fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -...
     |       ^^^^^^^^^^^^^^                                                                                                                                                                                                   -----------------------------------------------------
help: provide the argument
     |
2119 |         sender.dispatch_typed(command, &meta("local"), identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("apply command");
     |                                                                +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2130:18

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2130:18
     |
2130 |         receiver.ingest_operations(&operations, identity).await.expect("ingest once");
     |                  ^^^^^^^^^^^^^^^^^----------------------- argument #3 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:407:14
     |
 407 | ...fn ingest_operations(&mut self, mutations: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&m...
     |       ^^^^^^^^^^^^^^^^^                                                                                                                                                   --
help: provide the argument
     |
2130 |         receiver.ingest_operations(&operations, identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("ingest once");
     |                                                         +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2132:18

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2132:18
     |
2132 |         receiver.ingest_operations(&operations, identity).await.expect("ingest twice");
     |                  ^^^^^^^^^^^^^^^^^----------------------- argument #3 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:407:14
     |
 407 | ...fn ingest_operations(&mut self, mutations: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&m...
     |       ^^^^^^^^^^^^^^^^^                                                                                                                                                   --
help: provide the argument
     |
2132 |         receiver.ingest_operations(&operations, identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("ingest twice");
     |                                                         +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2165:16

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2165:16
     |
2165 |         sender.dispatch_typed(command, &meta("local"), identity).await.expect("apply command");
     |                ^^^^^^^^^^^^^^----------------------------------- argument #4 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:8725:12
     |
8725 | ...fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -...
     |       ^^^^^^^^^^^^^^                                                                                                                                                                                                   -----------------------------------------------------
help: provide the argument
     |
2165 |         sender.dispatch_typed(command, &meta("local"), identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("apply command");
     |                                                                +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2178:18

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2178:18
     |
2178 |         receiver.ingest_operations(&operations, identity).await.expect("ingest once");
     |                  ^^^^^^^^^^^^^^^^^----------------------- argument #3 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:407:14
     |
 407 | ...fn ingest_operations(&mut self, mutations: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&m...
     |       ^^^^^^^^^^^^^^^^^                                                                                                                                                   --
help: provide the argument
     |
2178 |         receiver.ingest_operations(&operations, identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("ingest once");
     |                                                         +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2181:18

error[E0061]: this method takes 3 arguments but 2 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2181:18
     |
2181 |         receiver.ingest_operations(&operations, identity).await.expect("ingest twice");
     |                  ^^^^^^^^^^^^^^^^^----------------------- argument #3 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:407:14
     |
 407 | ...fn ingest_operations(&mut self, mutations: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&m...
     |       ^^^^^^^^^^^^^^^^^                                                                                                                                                   --
help: provide the argument
     |
2181 |         receiver.ingest_operations(&operations, identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("ingest twice");
     |                                                         +++++++++++++++++++++++++++++++++++++++++++++++++

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2198:13

error[E0061]: this method takes 4 arguments but 3 arguments were supplied
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:2198:13
     |
2198 |         app.dispatch_typed(command, &meta("local"), identity).await.expect("dispatch a command whose mutations carry foreign steps");
     |             ^^^^^^^^^^^^^^----------------------------------- argument #4 of type `&mut semio_framework_job::StepContext<'_>` is missing
     |
note: method defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:8725:12
     |
8725 | ...fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -...
     |       ^^^^^^^^^^^^^^                                                                                                                                                                                                   -----------------------------------------------------
help: provide the argument
     |
2198 |         app.dispatch_typed(command, &meta("local"), identity, /* &mut semio_framework_job::StepContext<'_> */).await.expect("dispatch a command whose mutations carry foreign steps");
     |                                                             +++++++++++++++++++++++++++++++++++++++++++++++++

[script] {"owner":"procedural:plane-input-native","command":"native:owner-command","stage":"running"}
[native:owner-command] running elapsedMs=180036

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:549:19
    |
549 |             match PluginApp::maintenance_step(app, grant).expect("bounded maintenance") {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^      ----- argument #2 of type `&mut semio_framework_job::StepContext<'_>` is missing
    |
note: method defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔌️app/🦀️.rs:101:8
    |
101 |     fn maintenance_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>, loan:RetainedCloneGrant) -> Result<PluginLifecycle...
    |        ^^^^^^^^^^^^^^^^            --
help: provide the argument
    |
549 |             match PluginApp::maintenance_step(app, /* &mut semio_framework_job::StepContext<'_> */, grant).expect("bounded maintenance") {
    |                                                    ++++++++++++++++++++++++++++++++++++++++++++++++

warning: unused import: `WindowConfigOwner`
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:23:50

error[E0004]: non-exhaustive patterns: `&original_failure::PluginLifecyclePrimary::Domain(_)` not covered
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1622:59
     |
1622 | ...ode,detail)=match &failure.primary{crate::PluginLifecyclePrimary::Fault(fault)=>(fault.code.0.clone(),fault.message.clone()),cr...
     |                      ^^^^^^^^^^^^^^^^ pattern `&original_failure::PluginLifecyclePrimary::Domain(_)` not covered
     |
note: `original_failure::PluginLifecyclePrimary` defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏛️ownership/⚠️failure/🦀️.rs:9:10
     |
   9 | pub enum PluginLifecyclePrimary{Fault(Fault),Value(ValueError),Domain(semio_framework_job::OriginalDomainPrimary)}
     |          ^^^^^^^^^^^^^^^^^^^^^^                                ------ not covered
     = note: the matched value is of type `&original_failure::PluginLifecyclePrimary`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
     |
1622 |         let failure=failure.into();let(code,detail)=match &failure.primary{crate::PluginLifecyclePrimary::Fault(fault)=>(fault.code.0.clone(),fault.message.clone()),crate::PluginLifecyclePrimary::Value(error)=>("interactive-job.owner-refused".to_string(),format!("{error:?}")), &original_failure::PluginLifecyclePrimary::Domain(_) => todo!()};
     |                                                                                                                                                                                                                                                                                     +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

Some errors have detailed explanations: E0004, E0061, E0277, E0609.

error: could not compile `semio-framework-plugin` (lib) due to 30 previous errors; 285 warnings emitted
warning: build failed, waiting for other jobs to finish...
error: command `/Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/cargo test --no-run --message-format json-render-diagnostics --package semio-s-artifact-procedural-generation3d --lib --features component-app-assembly` exited with code 101
[TRACE] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️08/PROCEDURAL-FEATURE-COMPLETE/🗑️generated/root-2026-10-14/plane-input-native/semio-nextest-J9FSXZ
38 |       child.once("close", (code, signal) => accept({ code, signal }));
39 |     });
40 |     const failure = reason === "timeout" ? `${command} exceeded its ${options.budgetMs}ms budget` : reason ? `${command} ${reason}` : signal ? `${command} was killed by ${signal}` : code !== 0 ? `${command} exited with status ${code}` : undefined;
41 |     if (!failure) return;
42 |     if (reason === "timeout") console.error(`[budget] ${command} ${args.join(" ")} exceeded ${options.budgetMs}ms — killed. ${options.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive)."}`);
43 |     if (options.throwOnFailure) throw Error(failure);
                                           ^
error: cargo exited with status 101
      at runBudgetedTestCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts:43:39)
      at async runCargoTestsV1 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts:164:343)
      at async runRepositoryCargoTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1379:11)
      at async runArtifactRustPackagesTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:32:15)
      at async runArtifactRustTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:22:9)
      at async run (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts:43:39)

error: command `/Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/cargo test --no-run --message-format json-render-diagnostics --package semio-s-artifact-procedural-generation3d --lib --features component-app-assembly` exited with code 101
[TRACE] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️08/PROCEDURAL-FEATURE-COMPLETE/🗑️generated/root-2026-10-14/plane-input-native/semio-nextest-J9FSXZ
38 |       child.once("close", (code, signal) => accept({ code, signal }));
39 |     });
40 |     const failure = reason === "timeout" ? `${command} exceeded its ${options.budgetMs}ms budget` : reason ? `${command} ${reason}` : signal ? `${command} was killed by ${signal}` : code !== 0 ? `${command} exited with status ${code}` : undefined;
41 |     if (!failure) return;
42 |     if (reason === "timeout") console.error(`[budget] ${command} ${args.join(" ")} exceeded ${options.budgetMs}ms — killed. ${options.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive)."}`);
43 |     if (options.throwOnFailure) throw Error(failure);
                                           ^
error: cargo exited with status 101
      at runBudgetedTestCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts:43:39)
      at async runCargoTestsV1 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts:164:343)
      at async runRepositoryCargoTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1379:11)
      at async runArtifactRustPackagesTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:32:15)
      at async runArtifactRustTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:22:9)
      at async run (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts:43:39)
      at async run (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts:80:19)
      at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:98:23)

error: cargo exited with status 101
      at runBudgetedTestCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts:43:39)
      at async runCargoTestsV1 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts:164:343)
      at async runRepositoryCargoTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1379:11)
      at async runArtifactRustPackagesTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:32:15)
      at async runArtifactRustTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:22:9)
      at async run (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts:43:39)
      at async run (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts:80:19)
      at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:98:23)
      at async withScriptProcessEnvelope (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:47:65)

Bun v1.3.14 (macOS arm64)
142 |     stopProgress();
143 |     await Promise.race([Promise.all([...pending]), interrupted]);
144 |     await Promise.all([process.stdout,process.stderr].map(output=>new Promise<void>((accept,reject)=>output.write("",error=>error?reject(error):accept()))));
145 |     if (publicationRefused) throw publicationFailure;
146 |     if (stopped) throw new Error(`${label} stopped: ${stopped}`);
147 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
```
