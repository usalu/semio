# Original TimeTravel Identity Native Floor

Authority 3679 terminated exit1 / Cargo101 after4m22s before Rust assertions. Captured original Plugin lib-test compiler emitted2446 errors across current production and fixture callers; no TimeTravel runtime law is qualified. This is a full native lib-test epoch, so counts differ from Root prior lib compile.

## Current Original TimeTravel Diagnostic Contexts


Fresh source path normalization captures 6 original TimeTravel production error contexts.

```text
@semio-tech/framework-plugin: error[E0277]: the trait bound `Result<OwnedJsonSchemaValidator, std::string::String>: RetireOwned` is not satisfied
@semio-tech/framework-plugin:    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:141:5
@semio-tech/framework-plugin:     |
@semio-tech/framework-plugin: 132 | #[derive(semio_framework_value::RetireOwned)]
@semio-tech/framework-plugin:     |          ---------------------------------- required by a bound introduced by this call
@semio-tech/framework-plugin: ...
@semio-tech/framework-plugin: 141 |     validator: Result<semio_framework_schema::OwnedJsonSchemaValidator, String>,
@semio-tech/framework-plugin:     |     ^^^^^^^^^ the trait `RetireOwned` is not implemented for `Result<OwnedJsonSchemaValidator, std::string::String>`
@semio-tech/framework-plugin:     |
@semio-tech/framework-plugin:     = help: the following other types implement trait `RetireOwned`:
@semio-tech/framework-plugin:               &'static str
@semio-tech/framework-plugin:               ()
@semio-tech/framework-plugin:               (A, B)
@semio-tech/framework-plugin:               (A, B, C)
@semio-tech/framework-plugin:               (A, B, C, D)
@semio-tech/framework-plugin:               (A, B, C, D, E)
@semio-tech/framework-plugin:               ActionArgOption
@semio-tech/framework-plugin:               ArgFormat
@semio-tech/framework-plugin:             and 338 others
@semio-tech/framework-plugin: note: required by a bound in `deferred`
@semio-tech/framework-plugin:    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:423:20
@semio-tech/framework-plugin:     |
@semio-tech/framework-plugin: 423 | pub fn deferred<T: RetireOwned>(value: T) -> Box<dyn RetirementCursor> {
@semio-tech/framework-plugin:     |                    ^^^^^^^^^^^ required by this bound in `deferred`

```

```text
@semio-tech/framework-plugin: error[E0277]: the trait bound `Result<OwnedJsonSchemaValidator, std::string::String>: RetireOwned` is not satisfied
@semio-tech/framework-plugin:    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:141:16
@semio-tech/framework-plugin:     |
@semio-tech/framework-plugin: 141 |     validator: Result<semio_framework_schema::OwnedJsonSchemaValidator, String>,
@semio-tech/framework-plugin:     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `RetireOwned` is not implemented for `Result<OwnedJsonSchemaValidator, std::string::String>`
@semio-tech/framework-plugin:     |
@semio-tech/framework-plugin:     = help: the following other types implement trait `RetireOwned`:
@semio-tech/framework-plugin:               &'static str
@semio-tech/framework-plugin:               ()
@semio-tech/framework-plugin:               (A, B)
@semio-tech/framework-plugin:               (A, B, C)
@semio-tech/framework-plugin:               (A, B, C, D)
@semio-tech/framework-plugin:               (A, B, C, D, E)
@semio-tech/framework-plugin:               ActionArgOption
@semio-tech/framework-plugin:               ArgFormat
@semio-tech/framework-plugin:             and 338 others
@semio-tech/framework-plugin: note: required by a bound in `deferred_birth_bytes`
@semio-tech/framework-plugin:    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:50:38
@semio-tech/framework-plugin:     |
@semio-tech/framework-plugin:  50 | pub const fn deferred_birth_bytes<T: RetireOwned>() -> usize { size_of::<Deferred<T>>() }
@semio-tech/framework-plugin:     |                                      ^^^^^^^^^^^ required by this bound in `deferred_birth_bytes`

```

```text
@semio-tech/framework-plugin: error[E0061]: this method takes 2 arguments but 1 argument was supplied
@semio-tech/framework-plugin:      --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:730:46
@semio-tech/framework-plugin:       |
@semio-tech/framework-plugin:   730 | ...k(store.retire_snapshot_alias(snapshot.into_snapshot_owner()).map_err(|error| error.into_fault())?);
@semio-tech/framework-plugin:       |            ^^^^^^^^^^^^^^^^^^^^^-------------------------------- argument #2 of type `dsl::RetainedCloneGrant` is missing
@semio-tech/framework-plugin:       |
@semio-tech/framework-plugin: note: expected `&mut Option<Arc<P>>`, found `Arc<P>`
@semio-tech/framework-plugin:      --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:730:68
@semio-tech/framework-plugin:       |
@semio-tech/framework-plugin:   730 | ...   self.retirements.push_back(store.retire_snapshot_alias(snapshot.into_snapshot_owner()).map_err(|error| error.into_fault())?);
@semio-tech/framework-plugin:       |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
@semio-tech/framework-plugin:       = note: expected mutable reference `&mut std::option::Option<Arc<_>>`
@semio-tech/framework-plugin:                             found struct `Arc<_>`
@semio-tech/framework-plugin: note: method defined here
@semio-tech/framework-plugin:      --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20985:12
@semio-tech/framework-plugin:       |
@semio-tech/framework-plugin: 20985 |     pub fn retire_snapshot_alias(&self,alias:&mut Option<Arc<P>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshot...
@semio-tech/framework-plugin:       |            ^^^^^^^^^^^^^^^^^^^^^
@semio-tech/framework-plugin: help: provide the argument
@semio-tech/framework-plugin:       |
@semio-tech/framework-plugin:   730 -             self.retirements.push_back(store.retire_snapshot_alias(snapshot.into_snapshot_owner()).map_err(|error| error.into_fault())?);
@semio-tech/framework-plugin:   730 +             self.retirements.push_back(store.retire_snapshot_alias(/* &mut std::option::Option<std::sync::Arc<P>> */, /* dsl::RetainedCloneGrant */).map_err(|error| error.into_fault())?);
@semio-tech/framework-plugin:       |

```

```text
@semio-tech/framework-plugin: error[E0308]: `?` operator has incompatible types
@semio-tech/framework-plugin:    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:730:40
@semio-tech/framework-plugin:     |
@semio-tech/framework-plugin: 730 | ...ts.push_back(store.retire_snapshot_alias(snapshot.into_snapshot_owner()).map_err(|error| error.into_fault())?);
@semio-tech/framework-plugin:     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Option<(Box<_>, _)>`
@semio-tech/framework-plugin:     |
@semio-tech/framework-plugin:     = note: `?` operator cannot convert from `std::option::Option<(std::boxed::Box<dyn dsl::ErasedSnapshotRetirement>, dsl::RetainedCloneProgress)>` to `std::boxed::Box<(dyn dsl::ErasedSnapshotRetirement + 'static)>`
@semio-tech/framework-plugin:     = note: expected struct `std::boxed::Box<(dyn dsl::ErasedSnapshotRetirement + 'static)>`
@semio-tech/framework-plugin:                  found enum `std::option::Option<(std::boxed::Box<dyn dsl::ErasedSnapshotRetirement>, dsl::RetainedCloneProgress)>`

```

```text
@semio-tech/framework-plugin: error[E0308]: mismatched types
@semio-tech/framework-plugin:     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3454:106
@semio-tech/framework-plugin:      |
@semio-tech/framework-plugin: 3454 | ...e.mutation_id.0.clone(), actor: envelope.actor.0.clone(), timestamp: envelope.timestamp, scope: supersede.scope, inputs: supers...
@semio-tech/framework-plugin:      |                                    ^^^^^^^^^^^^^^^^^^^^^^^^ expected `String`, found `SharedUtf8`
@semio-tech/framework-plugin:      |
@semio-tech/framework-plugin: help: try using a conversion method
@semio-tech/framework-plugin:      |
@semio-tech/framework-plugin: 3454 -                     records.push(SupersedeRecord { transition_id: envelope.mutation_id.0.clone(), actor: envelope.actor.0.clone(), timestamp: envelope.timestamp, scope: supersede.scope, inputs: supersede.inputs, role: SupersedeRole::Edit, entry: 0 })
@semio-tech/framework-plugin: 3454 +                     records.push(SupersedeRecord { transition_id: envelope.mutation_id.0.clone(), actor: envelope.actor.0.to_string(), timestamp: envelope.timestamp, scope: supersede.scope, inputs: supersede.inputs, role: SupersedeRole::Edit, entry: 0 })
@semio-tech/framework-plugin:      |

```

```text
@semio-tech/framework-plugin: error[E0308]: mismatched types
@semio-tech/framework-plugin:     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3457:48
@semio-tech/framework-plugin:      |
@semio-tech/framework-plugin: 3457 |                     let newest = reverts.entry(envelope.actor.0.clone()).or_insert(envelope.timestamp);
@semio-tech/framework-plugin:      |                                          ----- ^^^^^^^^^^^^^^^^^^^^^^^^ expected `String`, found `SharedUtf8`
@semio-tech/framework-plugin:      |                                          |
@semio-tech/framework-plugin:      |                                          arguments to this method are incorrect
@semio-tech/framework-plugin:      |
@semio-tech/framework-plugin: help: the return type of this call is `SharedUtf8` due to the type of the argument passed
@semio-tech/framework-plugin:     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3457:34
@semio-tech/framework-plugin:      |
@semio-tech/framework-plugin: 3457 |                     let newest = reverts.entry(envelope.actor.0.clone()).or_insert(envelope.timestamp);
@semio-tech/framework-plugin:      |                                  ^^^^^^^^^^^^^^------------------------^
@semio-tech/framework-plugin:      |                                                |
@semio-tech/framework-plugin:      |                                                this argument influences the return type of `entry`
@semio-tech/framework-plugin: note: method defined here
@semio-tech/framework-plugin:     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:1013:12
@semio-tech/framework-plugin:      |
@semio-tech/framework-plugin: 1013 |     pub fn entry(&mut self, key: K) -> Entry<'_, K, V, A> {
@semio-tech/framework-plugin:      |            ^^^^^
@semio-tech/framework-plugin: help: try using a conversion method
@semio-tech/framework-plugin:      |
@semio-tech/framework-plugin: 3457 -                     let newest = reverts.entry(envelope.actor.0.clone()).or_insert(envelope.timestamp);
@semio-tech/framework-plugin: 3457 +                     let newest = reverts.entry(envelope.actor.0.to_string()).or_insert(envelope.timestamp);
@semio-tech/framework-plugin:      |
@semio-tech/framework-plugin: [cargo:build] running elapsedMs=190046

```

