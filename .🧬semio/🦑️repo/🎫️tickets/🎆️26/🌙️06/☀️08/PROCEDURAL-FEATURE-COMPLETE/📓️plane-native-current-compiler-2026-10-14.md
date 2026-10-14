# Current Plane Native Compilation

Registered Root26705 completed naturally Nx1 after6m19. Actual Cargo production compilation ran; no widget native assertion executed. Shared test-only original caller Cargo feature manifest changes parsed without TOML refusal. Below are exact current error blocks; warnings preserved only in generated output. This is a compiler floor, not a passing runtime test.

```text
error[E0425]: cannot find type `ControlledRetirement` in module `semio_framework_value::retirement`
 --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎮️command/📥️ingress/🦀️.rs:8:167
  |
8 | ..._framework_value::retirement::ControlledRetirement<ActionIngressSource>>>,ready:bool}
  |                                  ^^^^^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`
  |
help: consider importing one of these structs
  |
2 + use crate::__value::retirement::controlled::ControlledRetirement;
  |
2 + use replication::retirement::controlled::ControlledRetirement;
  |
2 + use semio_framework_value::retirement::controlled::ControlledRetirement;
  |
help: if you import `ControlledRetirement`, refer to it directly
  |
8 - pub struct ActionIngressRecord{source:std::mem::ManuallyDrop<Option<ActionIngressSource>>,retirement:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::ControlledRetirement<ActionIngressSource>>>,ready:bool}
8 + pub struct ActionIngressRecord{source:std::mem::ManuallyDrop<Option<ActionIngressSource>>,retirement:std::mem::ManuallyDrop<Option<ControlledRetirement<ActionIngressSource>>>,ready:bool}
  |
```

```text
error[E0433]: cannot find `ControlledRetirement` in `retirement`
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎮️command/📥️ingress/🦀️.rs:16:925
   |
16 | ...amework_value::retirement::ControlledRetirement::new(source){Ok(owner)=>{*self.retirement=Some(owner);Ok(RetainedCloneStep::Progr...
   |                               ^^^^^^^^^^^^^^^^^^^^ could not find `ControlledRetirement` in `retirement`
   |
help: consider importing one of these structs
   |
 2 + use crate::__value::retirement::controlled::ControlledRetirement;
   |
 2 + use replication::retirement::controlled::ControlledRetirement;
   |
 2 + use semio_framework_value::retirement::controlled::ControlledRetirement;
   |
help: if you import `ControlledRetirement`, refer to it directly
   |
16 -  pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(owner)=self.retirement.as_mut(){if owner.terminal_is_empty(){self.retirement.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}return owner.step(grant)}if let Some(source)=self.source.take(){let bytes=size_of::<ActionIngressSource>();let capacity=source.retirement_birth_bytes().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original action source factory unavailable"))?;match semio_framework_value::retirement::ControlledRetirement::new(source){Ok(owner)=>{*self.retirement=Some(owner);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()}))},Err((cause,source))=>{*self.source=Some(source);Err(cause)}}}else{Ok(RetainedCloneStep::Complete(Default::default()))}}
16 +  pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(owner)=self.retirement.as_mut(){if owner.terminal_is_empty(){self.retirement.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}return owner.step(grant)}if let Some(source)=self.source.take(){let bytes=size_of::<ActionIngressSource>();let capacity=source.retirement_birth_bytes().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original action source factory unavailable"))?;match ControlledRetirement::new(source){Ok(owner)=>{*self.retirement=Some(owner);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()}))},Err((cause,source))=>{*self.source=Some(source);Err(cause)}}}else{Ok(RetainedCloneStep::Complete(Default::default()))}}
   |
```

```text
error[E0308]: arguments to this function are incorrect
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🧩️app/🦀️.rs:572:9
    |
572 |         Self::command_from_action(&intent.action.name, merged, None).await
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: expected `&mut Option<DslValue>`, found `Option<DslValue>`
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🧩️app/🦀️.rs:572:56
    |
572 |         Self::command_from_action(&intent.action.name, merged, None).await
    |                                                        ^^^^^^
    = note: expected mutable reference `&mut Option<_>`
                            found enum `Option<_>`
note: expected `&mut Option<Fault>`, found `Option<_>`
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🧩️app/🦀️.rs:572:64
    |
572 |         Self::command_from_action(&intent.action.name, merged, None).await
    |                                                                ^^^^
    = note: expected mutable reference `&mut Option<semio_framework_dsl::Fault>`
                            found enum `Option<_>`
note: associated function defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🧩️app/🦀️.rs:537:14
    |
537 |     async fn command_from_action(action: &str, _args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Co...
    |              ^^^^^^^^^^^^^^^^^^^               ----------------------------  ----------------------------------
help: consider mutably borrowing here
    |
572 |         Self::command_from_action(&intent.action.name, &mut merged, None).await
    |                                                        ++++
help: consider mutably borrowing here
    |
572 |         Self::command_from_action(&intent.action.name, merged, &mut None).await
    |                                                                ++++
```

```text
error[E0308]: arguments to this function are incorrect
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🪟️surface/🦀️.rs:537:9
    |
537 |         Self::command_from_action(&intent.action.name, merged, None)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: expected `&mut Option<DslValue>`, found `Option<DslValue>`
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🪟️surface/🦀️.rs:537:56
    |
537 |         Self::command_from_action(&intent.action.name, merged, None)
    |                                                        ^^^^^^
    = note: expected mutable reference `&mut Option<_>`
                            found enum `Option<_>`
note: expected `&mut Option<Fault>`, found `Option<_>`
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🪟️surface/🦀️.rs:537:64
    |
537 |         Self::command_from_action(&intent.action.name, merged, None)
    |                                                                ^^^^
    = note: expected mutable reference `&mut Option<semio_framework_dsl::Fault>`
                            found enum `Option<_>`
note: associated function defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🪟️surface/🦀️.rs:509:8
    |
509 |     fn command_from_action(action: &str, _args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command,...
    |        ^^^^^^^^^^^^^^^^^^^               ----------------------------  ----------------------------------
help: consider mutably borrowing here
    |
537 |         Self::command_from_action(&intent.action.name, &mut merged, None)
    |                                                        ++++
help: consider mutably borrowing here
    |
537 |         Self::command_from_action(&intent.action.name, merged, &mut None)
    |                                                                ++++
```

```text
error[E0308]: arguments to this function are incorrect
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6667:36
     |
6667 |             let command = Box::new(A::command_from_action(action, source_args, None).await?);
     |                                    ^^^^^^^^^^^^^^^^^^^^^^
     |
note: expected `&mut Option<DslValue>`, found `Option<DslValue>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6667:67
     |
6667 |             let command = Box::new(A::command_from_action(action, source_args, None).await?);
     |                                                                   ^^^^^^^^^^^
     = note: expected mutable reference `&mut Option<_>`
                             found enum `Option<_>`
note: expected `&mut Option<Fault>`, found `Option<_>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6667:80
     |
6667 |             let command = Box::new(A::command_from_action(action, source_args, None).await?);
     |                                                                                ^^^^
     = note: expected mutable reference `&mut Option<semio_framework_dsl::Fault>`
                             found enum `Option<_>`
note: associated function defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🧩️app/🦀️.rs:537:14
     |
 537 |     async fn command_from_action(action: &str, _args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::C...
     |              ^^^^^^^^^^^^^^^^^^^               ----------------------------  ----------------------------------
help: consider mutably borrowing here
     |
6667 |             let command = Box::new(A::command_from_action(action, &mut source_args, None).await?);
     |                                                                   ++++
help: consider mutably borrowing here
     |
6667 |             let command = Box::new(A::command_from_action(action, source_args, &mut None).await?);
     |                                                                                ++++
```

```text
error[E0308]: arguments to this function are incorrect
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6718:23
     |
6718 |         let command = A::command_from_action(command_id, Some(args), original_fault).await?;
     |                       ^^^^^^^^^^^^^^^^^^^^^^
     |
note: expected `&mut Option<DslValue>`, found `Option<DslValue>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6718:58
     |
6718 |         let command = A::command_from_action(command_id, Some(args), original_fault).await?;
     |                                                          ^^^^^^^^^^
     = note: expected mutable reference `&mut Option<_>`
                             found enum `Option<_>`
note: expected `&mut Option<Fault>`, found `Option<Fault>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6718:70
     |
6718 |         let command = A::command_from_action(command_id, Some(args), original_fault).await?;
     |                                                                      ^^^^^^^^^^^^^^
     = note: expected mutable reference `&mut Option<_>`
                             found enum `Option<_>`
note: associated function defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🧩️app/🦀️.rs:537:14
     |
 537 |     async fn command_from_action(action: &str, _args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::C...
     |              ^^^^^^^^^^^^^^^^^^^               ----------------------------  ----------------------------------
help: consider mutably borrowing here
     |
6718 |         let command = A::command_from_action(command_id, &mut Some(args), original_fault).await?;
     |                                                          ++++
help: consider mutably borrowing here
     |
6718 |         let command = A::command_from_action(command_id, Some(args), &mut original_fault).await?;
     |                                                                      ++++
```

```text
error[E0308]: arguments to this function are incorrect
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6827:25
     |
6827 | ...   let command=A::command_from_action(&address.action_id,Some(DslValue::Object(arguments.into_iter().collect())),None).await?;
     |                   ^^^^^^^^^^^^^^^^^^^^^^
     |
note: expected `&mut Option<DslValue>`, found `Option<DslValue>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6827:67
     |
6827 | ...   let command=A::command_from_action(&address.action_id,Some(DslValue::Object(arguments.into_iter().collect())),None).await?;
     |                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: expected mutable reference `&mut Option<_>`
                             found enum `Option<_>`
note: expected `&mut Option<Fault>`, found `Option<_>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:6827:123
     |
6827 | ...   let command=A::command_from_action(&address.action_id,Some(DslValue::Object(arguments.into_iter().collect())),None).await?;
     |                                                                                                                     ^^^^
     = note: expected mutable reference `&mut Option<semio_framework_dsl::Fault>`
                             found enum `Option<_>`
note: associated function defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🧩️app/🦀️.rs:537:14
     |
 537 |     async fn command_from_action(action: &str, _args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::C...
     |              ^^^^^^^^^^^^^^^^^^^               ----------------------------  ----------------------------------
help: consider mutably borrowing here
     |
6827 |             let command=A::command_from_action(&address.action_id,&mut Some(DslValue::Object(arguments.into_iter().collect())),None).await?;
     |                                                                   ++++
help: consider mutably borrowing here
     |
6827 |             let command=A::command_from_action(&address.action_id,Some(DslValue::Object(arguments.into_iter().collect())),&mut None).await?;
     |                                                                                                                           ++++
```

```text
error[E0308]: arguments to this function are incorrect
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:9501:23
     |
9501 |         let command = A::command_from_action(verb, merged, None).await?;
     |                       ^^^^^^^^^^^^^^^^^^^^^^
     |
note: expected `&mut Option<DslValue>`, found `Option<DslValue>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:9501:52
     |
9501 |         let command = A::command_from_action(verb, merged, None).await?;
     |                                                    ^^^^^^
     = note: expected mutable reference `&mut Option<_>`
                             found enum `Option<_>`
note: expected `&mut Option<Fault>`, found `Option<_>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧵️instance/🦀️.rs:9501:60
     |
9501 |         let command = A::command_from_action(verb, merged, None).await?;
     |                                                            ^^^^
     = note: expected mutable reference `&mut Option<semio_framework_dsl::Fault>`
                             found enum `Option<_>`
note: associated function defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🧩️app/🦀️.rs:537:14
     |
 537 |     async fn command_from_action(action: &str, _args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::C...
     |              ^^^^^^^^^^^^^^^^^^^               ----------------------------  ----------------------------------
help: consider mutably borrowing here
     |
9501 |         let command = A::command_from_action(verb, &mut merged, None).await?;
     |                                                    ++++
help: consider mutably borrowing here
     |
9501 |         let command = A::command_from_action(verb, merged, &mut None).await?;
     |                                                            ++++
```

```text
error[E0308]: arguments to this function are incorrect
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1326:27
     |
1326 | ...   let command = A::command_from_action(&action.id, Some(staged.clone()), None).await.unwrap_or_else(|error| panic!("action {} ...
     |                     ^^^^^^^^^^^^^^^^^^^^^^
     |
note: expected `&mut Option<DslValue>`, found `Option<DslValue>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1326:62
     |
1326 | ...   let command = A::command_from_action(&action.id, Some(staged.clone()), None).await.unwrap_or_else(|error| panic!("action {} ...
     |                                                        ^^^^^^^^^^^^^^^^^^^^
     = note: expected mutable reference `&mut Option<_>`
                             found enum `Option<_>`
note: expected `&mut Option<Fault>`, found `Option<_>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1326:84
     |
1326 | ...   let command = A::command_from_action(&action.id, Some(staged.clone()), None).await.unwrap_or_else(|error| panic!("action {} ...
     |                                                                              ^^^^
     = note: expected mutable reference `&mut Option<semio_framework_dsl::Fault>`
                             found enum `Option<_>`
note: associated function defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🧩️app/🦀️.rs:537:14
     |
 537 |     async fn command_from_action(action: &str, _args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::C...
     |              ^^^^^^^^^^^^^^^^^^^               ----------------------------  ----------------------------------
help: consider mutably borrowing here
     |
1326 |             let command = A::command_from_action(&action.id, &mut Some(staged.clone()), None).await.unwrap_or_else(|error| panic!("action {} failed to bridge: {}", action.id, error.message));
     |                                                              ++++
help: consider mutably borrowing here
     |
1326 |             let command = A::command_from_action(&action.id, Some(staged.clone()), &mut None).await.unwrap_or_else(|error| panic!("action {} failed to bridge: {}", action.id, error.message));
     |                                                                                    ++++
```

```text
error[E0308]: arguments to this function are incorrect
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1939:26
     |
1939 | ...   let bridge = A::command_from_action(&action.id, Some(staged.clone()), None).await.map(drop).map_err(|fault| format!("{}: {}"...
     |                    ^^^^^^^^^^^^^^^^^^^^^^
     |
note: expected `&mut Option<DslValue>`, found `Option<DslValue>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1939:61
     |
1939 | ...   let bridge = A::command_from_action(&action.id, Some(staged.clone()), None).await.map(drop).map_err(|fault| format!("{}: {}"...
     |                                                       ^^^^^^^^^^^^^^^^^^^^
     = note: expected mutable reference `&mut Option<_>`
                             found enum `Option<_>`
note: expected `&mut Option<Fault>`, found `Option<_>`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️testing/🦀️.rs:1939:83
     |
1939 | ...   let bridge = A::command_from_action(&action.id, Some(staged.clone()), None).await.map(drop).map_err(|fault| format!("{}: {}"...
     |                                                                             ^^^^
     = note: expected mutable reference `&mut Option<semio_framework_dsl::Fault>`
                             found enum `Option<_>`
note: associated function defined here
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../📜️declaration/🧩️app/🦀️.rs:537:14
     |
 537 |     async fn command_from_action(action: &str, _args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::C...
     |              ^^^^^^^^^^^^^^^^^^^               ----------------------------  ----------------------------------
help: consider mutably borrowing here
     |
1939 |             let bridge = A::command_from_action(&action.id, &mut Some(staged.clone()), None).await.map(drop).map_err(|fault| format!("{}: {}", fault.code.0, fault.message));
     |                                                             ++++
help: consider mutably borrowing here
     |
1939 |             let bridge = A::command_from_action(&action.id, Some(staged.clone()), &mut None).await.map(drop).map_err(|fault| format!("{}: {}", fault.code.0, fault.message));
     |                                                                                   ++++

[cargo:build] running elapsedMs=310063
[artifact-rust:semio-s-artifact-procedural-generation3d:test] running elapsedMs=359691
```

```text
error[E0599]: no method named `retirement_birth_bytes` found for reference `&ActionIngressSource` in the current scope
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎮️command/📥️ingress/🦀️.rs:15:521
   |
15 | ...Source>(),capacity_bytes:source.retirement_birth_bytes().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"ori...
   |                                    ^^^^^^^^^^^^^^^^^^^^^^ method not found in `&ActionIngressSource`
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `RetireOwned` which provides `retirement_birth_bytes` is implemented but not in scope; perhaps you want to import it
   |
 2 + use semio_framework_value::retirement::RetireOwned;
   |
```

```text
error[E0599]: no method named `retirement_birth_bytes` found for struct `ActionIngressSource` in the current scope
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🎮️command/📥️ingress/🦀️.rs:16:743
   |
 6 | pub struct ActionIngressSource{pub args:Option<DslValue>,pub fault:Option<Fault>}
   | ------------------------------ method `retirement_birth_bytes` not found for this struct
...
16 |  pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(owner)=self.retirement.as_mut(){if owner.terminal_is_empty(){self.retirement.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}return owner.step(grant)}if let Some(source)=self.source.take(){let bytes=size_of::<ActionIngressSource>();let capacity=source.retirement_birth_bytes()....
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       ^^^^^^^^^^^^^^^^^^^^^^ method not found in `ActionIngressSource`
   |
  ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:15:8
   |
15 |     fn retirement_birth_bytes(&self) -> Option<usize> { None }
   |        ---------------------- the method is available for `ActionIngressSource` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `RetireOwned` which provides `retirement_birth_bytes` is implemented but not in scope; perhaps you want to import it
   |
 2 + use semio_framework_value::retirement::RetireOwned;
   |
```

```text
error: could not compile `semio-framework-plugin` (lib) due to 12 previous errors; 285 warnings emitted
```

```text
error: command `/Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/cargo test --no-run --message-format json-render-diagnostics --package semio-s-artifact-procedural-generation3d --lib --features component-app-assembly --profile prototype` exited with code 101
[TRACE] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️08/PROCEDURAL-FEATURE-COMPLETE/🗑️generated/root-2026-10-14/plane-native-current/semio-nextest-fWNIZL
38 |       child.once("close", (code, signal) => accept({ code, signal }));
39 |     });
40 |     const failure = reason === "timeout" ? `${command} exceeded its ${options.budgetMs}ms budget` : reason ? `${command} ${reason}` : signal ? `${command} was killed by ${signal}` : code !== 0 ? `${command} exited with status ${code}` : undefined;
41 |     if (!failure) return;
42 |     if (reason === "timeout") console.error(`[budget] ${command} ${args.join(" ")} exceeded ${options.budgetMs}ms — killed. ${options.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive)."}`);
43 |     if (options.throwOnFailure) throw Error(failure);
                                           ^
```

```text
error: cargo exited with status 101
      at runBudgetedTestCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts:43:39)
      at async runCargoTestsV1 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts:167:343)
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
                                                                 ^
OwnedCommandFailure: native:owner-command failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},

      at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:147:60)

Bun v1.3.14 (macOS arm64)



 NX   Running target verify-generation3d-widget-inputs-native for project @semio-tech/procedural-generation3d-rs failed

Failed tasks:

- @semio-tech/procedural-generation3d-rs:verify-generation3d-widget-inputs-native:prototype

Hint: run the command with --verbose for more details.

  Run duration:      6m 19s
  Cache:             Skipped (--skip-nx-cache)
  Critical path:     6m 18s (1 task)
  Recoverable time:  <1ms

  Recommendations:
    - Cache: drop --skip-nx-cache to restore unchanged tasks instantly.
    - Speed up or split the longest tasks on the critical path:
        @semio-tech/procedural-generation3d-rs:verify-generation3d-widget-inputs-native:prototype    6m 18s
```

```text
error: script "nx" exited with code 1
```