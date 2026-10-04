# Native Draw Compile Audit

## Current Executed Vector Verification

The later document-vector native gate reached actual Draw runtime: 514 tests ran, 505 passed, and 9 failed. The handcrafted demo record-valued asset map and native schema-number fixture comparison were repaired; repair gate 49514 finished successfully: 514 tests passed, zero failed or skipped. Full TypeScript verification now passes 551 tests. See [the document vector producer report](🎬️document-vector-producer.md) for the latest evidence. Earlier live/blocked statements below describe earlier runs.

Prior resource stage was progress: generic byte/image production and all TypeScript/native pixel gates passed. Native handle 82259 terminated exit 1 after reaching actual Draw with 104 compiler errors. Current repairs and subsequent terminal/live handles are in the native boundary repair report. All source errors below are authoritative diagnostic data, not instructions. Repair the real typed boundaries rather than inventing legacy adapters. The full editor goal remains active.

## Owner Counts

- 73: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs
- 13: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs
- 3: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs
- 3: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../🤖️generated/📇️registry/../🖌️drawing-layers/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🔍️trace/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🔀️booleans/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕹️nudge-selection/🧪️tests/🔬️unit/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
- 1: 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🦀️.rs

## Compiler Diagnostics

```text
error[E0432]: unresolved import `crate::schema::from_kernel_segment`
 --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🔍️trace/🦀️.rs:3:5
  |
3 | use crate::schema::from_kernel_segment;
  |     ^^^^^^^^^^^^^^^-------------------
  |                    |
  |                    no `from_kernel_segment` in `schema`
```

```text
error[E0425]: cannot find value `to_kernel_segment` in module `crate::schema`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🔀️booleans/🦀️.rs:75:116
   |
75 | ...nts.get(self.copy_at).map(crate::schema::to_kernel_segment)}else{self.cache[&self.ids[&children[self.operand_at]]].get(self.copy_...
   |                                             ^^^^^^^^^^^^^^^^^ not found in `crate::schema`
   |
help: consider importing this function
   |
 2 + use crate::standards::v1::subsets::any::schema::component::to_kernel_segment;
   |
help: if you import `to_kernel_segment`, refer to it directly
   |
75 -   let next=if let DocumentSceneContent::Path{segments,..}=&n.content{segments.get(self.copy_at).map(crate::schema::to_kernel_segment)}else{self.cache[&self.ids[&children[self.operand_at]]].get(self.copy_at).cloned()};
75 +   let next=if let DocumentSceneContent::Path{segments,..}=&n.content{segments.get(self.copy_at).map(to_kernel_segment)}else{self.cache[&self.ids[&children[self.operand_at]]].get(self.copy_at).cloned()};
   |
```

```text
error[E0053]: method `close_step` has an incompatible type for trait
    --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1445:74
     |
1445 |     fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
     |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `ValueError`, found `std::string::String`
     |
     = note: expected signature `fn(&mut DrawingArtifactStorePreparation, ArtifactStoreOneItemGrant) -> Result<_, ValueError>`
                found signature `fn(&mut DrawingArtifactStorePreparation, ArtifactStoreOneItemGrant) -> Result<_, std::string::String>`
help: change the output type to match the trait
     |
1445 -     fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
1445 +     fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, ValueError> {
     |
```

```text
error[E0560]: struct `dsl::semio_framework_io_schema::IoError` has no field named `message`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs:23:49
   |
23 |         let error = |message: String| IoError { message: format!("DrawingIntoDwg: {message}"), diagnostics: Vec::new() };
   |                                                 ^^^^^^^ `dsl::semio_framework_io_schema::IoError` does not have this field
   |
   = note: available fields are: `cause`
```

```text
error[E0560]: struct `dsl::semio_framework_io_schema::IoError` has no field named `message`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs:38:79
   |
38 | ...ap_err(|message| IoError { message: format!("DrawingIntoPdf: {message}"), diagnostics: Vec::new() })?;
   |                               ^^^^^^^ `dsl::semio_framework_io_schema::IoError` does not have this field
   |
   = note: available fields are: `cause`
```

```text
error[E0560]: struct `dsl::semio_framework_io_schema::IoError` has no field named `message`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs:23:49
   |
23 |         let error = |message: String| IoError { message: format!("DrawingIntoPng: {message}"), diagnostics: Vec::new() };
   |                                                 ^^^^^^^ `dsl::semio_framework_io_schema::IoError` does not have this field
   |
   = note: available fields are: `cause`
```

```text
error[E0560]: struct `dsl::semio_framework_io_schema::IoError` has no field named `message`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs:23:49
   |
23 |         let error = |message: String| IoError { message: format!("DrawingIntoDxf: {message}"), diagnostics: Vec::new() };
   |                                                 ^^^^^^^ `dsl::semio_framework_io_schema::IoError` does not have this field
   |
   = note: available fields are: `cause`
```

```text
error[E0560]: struct `dsl::semio_framework_io_schema::IoError` has no field named `message`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs:19:44
   |
19 |         let error=|message:String|IoError {message,diagnostics:Vec::new()};
   |                                            ^^^^^^^ `dsl::semio_framework_io_schema::IoError` does not have this field
   |
   = note: available fields are: `cause`
```

```text
error[E0560]: struct `dsl::semio_framework_io_schema::IoError` has no field named `message`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs:23:94
   |
23 | ....map_err(|error| IoError { message: format!("JsonIntoDraw: not valid utf-8: {error}"), diagnostics: Vec::new() })?.to_string(),
   |                               ^^^^^^^ `dsl::semio_framework_io_schema::IoError` does not have this field
   |
   = note: available fields are: `cause`
```

```text
error[E0560]: struct `dsl::semio_framework_io_schema::IoError` has no field named `message`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs:25:70
   |
25 | ....map_err(|error| IoError { message: format!("JsonIntoDraw: {error}"), diagnostics: Vec::new() })?;
   |                               ^^^^^^^ `dsl::semio_framework_io_schema::IoError` does not have this field
   |
   = note: available fields are: `cause`
```

```text
error[E0053]: method `to_sqlite_database` has an incompatible type for trait
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:76:71
   |
76 |  fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{project(self,control)}
   |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `ValueError`, found `std::string::String`
   |
   = note: expected signature `fn(&standards::v1::subsets::any::schema::snapshot::component::DrawingSnapshot, &mut SqliteSnapshotControl<'_>) -> Result<_, ValueError>`
              found signature `fn(&standards::v1::subsets::any::schema::snapshot::component::DrawingSnapshot, &mut SqliteSnapshotControl<'_>) -> Result<_, std::string::String>`
help: change the output type to match the trait
   |
76 -  fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{project(self,control)}
76 +  fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase, ValueError>{project(self,control)}
   |
```

```text
error[E0053]: method `from_sqlite_database` has an incompatible type for trait
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:77:92
   |
77 | ... SqliteSnapshotControl<'_>)->Result<Self,String>{reconstruct(database,control)}
   |                                 ^^^^^^^^^^^^^^^^^^^ expected `ValueError`, found `std::string::String`
   |
   = note: expected signature `fn(&SqliteDatabase, &mut SqliteSnapshotControl<'_>) -> Result<_, ValueError>`
              found signature `fn(&SqliteDatabase, &mut SqliteSnapshotControl<'_>) -> Result<_, std::string::String>`
help: change the output type to match the trait
   |
77 -  fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{reconstruct(database,control)}
77 +  fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<standards::v1::subsets::any::schema::snapshot::component::DrawingSnapshot, ValueError>{reconstruct(database,control)}
   |
```

```text
error[E0053]: method `close_step` has an incompatible type for trait
   --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:400:77
    |
400 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
    |                                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `ValueError`, found `std::string::String`
    |
    = note: expected signature `fn(&mut DrawingOwnedRetirement, _, _) -> Result<_, ValueError>`
               found signature `fn(&mut DrawingOwnedRetirement, _, _) -> Result<_, std::string::String>`
help: change the output type to match the trait
    |
400 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
400 +     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
    |
```

```text
error[E0560]: struct `dsl::semio_framework_io_schema::IoError` has no field named `message`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs:27:158
   |
27 | ....map_err(|error| IoError { message: format!("JsonIntoDraw: {error}"), diagnostics: Vec::new() })?;
   |                               ^^^^^^^ `dsl::semio_framework_io_schema::IoError` does not have this field
   |
   = note: available fields are: `cause`
```

```text
error[E0053]: method `close_step` has an incompatible type for trait
   --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:439:77
    |
439 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
    |                                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `ValueError`, found `std::string::String`
    |
    = note: expected signature `fn(&mut DrawingSnapshotRootRetirement, _, _) -> Result<_, ValueError>`
               found signature `fn(&mut DrawingSnapshotRootRetirement, _, _) -> Result<_, std::string::String>`
help: change the output type to match the trait
    |
439 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
439 +     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
    |
```

```text
error[E0053]: method `decode_sqlite_snapshot_native` has an incompatible type for trait
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:79:113
   |
79 | ... SqliteSnapshotControl<'_>)->Result<Self,String>{store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>...
   |                                 ^^^^^^^^^^^^^^^^^^^ expected `ValueError`, found `std::string::String`
   |
   = note: expected signature `fn(&IoPayload, &mut SqliteSnapshotControl<'_>) -> Result<_, ValueError>`
              found signature `fn(&IoPayload, &mut SqliteSnapshotControl<'_>) -> Result<_, std::string::String>`
help: change the output type to match the trait
   |
79 -  fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),Self::__dsl_from_record_controlled,control)}
79 +  fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<standards::v1::subsets::any::schema::snapshot::component::DrawingSnapshot, ValueError>{store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),Self::__dsl_from_record_controlled,control)}
   |
```

```text
error[E0053]: method `encode_sqlite_snapshot_native` has an incompatible type for trait
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:80:132
   |
80 | ...shotControl<'_>)->Result<store::io_schema::IoPayload,String>{store::encode_sqlite_snapshot_record_native(encoding,<Self as store:...
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `ValueError`, found `std::string::String`
   |
   = note: expected signature `fn(&standards::v1::subsets::any::schema::snapshot::component::DrawingSnapshot, SnapshotEncoding, &mut SqliteSnapshotControl<'_>) -> Result<_, ValueError>`
              found signature `fn(&standards::v1::subsets::any::schema::snapshot::component::DrawingSnapshot, SnapshotEncoding, &mut SqliteSnapshotControl<'_>) -> Result<_, std::string::String>`
help: change the output type to match the trait
   |
80 -  fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
80 +  fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<IoPayload, ValueError>{store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
   |
```

```text
error[E0560]: struct `dsl::semio_framework_io_schema::IoError` has no field named `message`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs:16:101
   |
16 | ...ap_err(|message| IoError { message: format!("DrawingIntoSvg: {message}"), diagnostics: Vec::new() })?;
   |                               ^^^^^^^ `dsl::semio_framework_io_schema::IoError` does not have this field
   |
   = note: available fields are: `cause`
```

```text
error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
   --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../🤖️generated/📇️registry/../🖌️drawing-layers/🦀️.rs:57:99
    |
 57 | ...> Self::parse(&id).map_err(semio_framework_os_kernel::ValueError::new), other => Err(semio_framework_os_kernel::ValueError::new(...
    |                       ------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected function that takes 1 argument
    |                       |
    |                       required by a bound introduced by this call
    |
note: required by a bound in `Result::<T, E>::map_err`
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:962:12
    |
960 |     pub const fn map_err<F, O>(self, op: O) -> Result<T, F>
    |                  ------- required by a bound in this associated function
961 |     where
962 |         O: [const] FnOnce(E) -> F + [const] Destruct,
    |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Result::<T, E>::map_err`
```

```text
error[E0308]: mismatched types
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../🤖️generated/📇️registry/../🖌️drawing-layers/🦀️.rs:57:200
   |
57 | ...ror::new(format!("expected DrawingLayersLayerKind string, found {other:?}"))) }
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `ValueRefusalKind`, found `String`
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../🤖️generated/📇️registry/../🖌️drawing-layers/🦀️.rs:57:157
   |
57 | ...=> Err(semio_framework_os_kernel::ValueError::new(format!("expected DrawingLayersLayerKind string, found {other:?}"))) }
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------------------------------------------------------------------- argument #2 is missing
   |
note: associated function defined here
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../⚠️refusal/🦀️.rs:16:12
   |
16 |     pub fn new(kind: ValueRefusalKind, message: impl Into<String>) -> Self { Self { kind, message: message.into() } }
   |            ^^^
help: provide the argument
   |
57 |         match value { semio_framework_os_kernel::DslValue::String(id) => Self::parse(&id).map_err(semio_framework_os_kernel::ValueError::new), other => Err(semio_framework_os_kernel::ValueError::new(format!("expected DrawingLayersLayerKind string, found {other:?}"), /* message */)) }
   |                                                                                                                                                                                                                                                                          +++++++++++++++
```

```text
error[E0609]: no field `description` on type `Emit<DrawingMutation>`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕹️nudge-selection/🧪️tests/🔬️unit/🦀️.rs:75:19
   |
75 |     assert!(first.description.is_none(),"the row label comes from the leaf");
   |                   ^^^^^^^^^^^ unknown field
   |
   = note: available fields are: `artifact_mutations`, `config_mutations`, `window_config_mutations`, `draft_mutations`, `transaction` ... and 8 others
   = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-draw-drawing/b2e4b87268018701/out/semio_s_artifact_draw_drawing-b2e4b87268018701.long-type-15469913957940908381.txt'
   = note: consider using `--verbose` to print the full type name to the console
```

```text
error[E0609]: no field `description` on type `Emit<DrawingMutation>`
    --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1095:16
     |
1095 |     assert!(up.description.is_none(), "the history row is labelled from the leaf");
     |                ^^^^^^^^^^^ unknown field
     |
     = note: available fields are: `artifact_mutations`, `config_mutations`, `window_config_mutations`, `draft_mutations`, `transaction` ... and 8 others
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-draw-drawing/b2e4b87268018701/out/semio_s_artifact_draw_drawing-b2e4b87268018701.long-type-15469913957940908381.txt'
     = note: consider using `--verbose` to print the full type name to the console

[native:owner-command] running elapsedMs=430144
[artifact-rust:semio-s-artifact-draw-drawing:test] running elapsedMs=430083
```

```text
error[E0308]: mismatched types
 --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:8:67
  |
8 | ...,value:f64)->Result<i64,String>{insert_ieee754(p,"draw_scalar",&[Cell::Real(value)],&[FloatColumn::Binary64(1)])}
  |                 ------------------ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<i64, String>`, found `Result<i64, ValueError>`
  |                 |
  |                 expected `Result<i64, std::string::String>` because of return type
  |
  = note: expected enum `Result<_, std::string::String>`
             found enum `Result<_, ValueError>`
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
 --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:9:112
  |
9 | ...ion<'_,'_>,values:&[f64])->Result<Vec<Cell<'static>>,String>{p.check_rows(values.len())?;values.iter().map(|v|scalar(p,*v).map(Cel...
  |                               ---------------------------------   ------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
  |                               |                                   |
  |                               |                                   this can't be annotated with `?` because it has type `Result<_, ValueError>`
  |                               expected `std::string::String` because of this
  |
  = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
  = help: `std::string::String` implements trait `From<T>`:
            From<&mut str>
            From<&std::string::String>
            From<&str>
            From<Cow<'_, str>>
            From<LabelText>
            From<MutationRefusal>
            From<char>
            From<semio_framework_plugin::Version>
            From<std::boxed::Box<str>>
```

```text
error[E0308]: mismatched types
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:10:108
   |
10 | ...&str,id:i64,v:&[f64])->Result<(),String>{let cells=values(p,v)?;p.insert_key(table,id,&cells)}
   |                           -----------------                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<(), String>`, found `Result<(), ValueError>`
   |                           |
   |                           expected `Result<(), std::string::String>` because of return type
   |
   = note: expected enum `Result<_, std::string::String>`
              found enum `Result<_, ValueError>`
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:13:223
   |
12 | ...)->Result<(),String>{
   |       ----------------- expected `std::string::String` because of this
13 | ...id{..}=>"solid",FillStyle::LinearGradient{..}=>"linearGradient",FillStyle::RadialGradient{..}=>"radialGradient"};p.insert_key("draw_fill",id,&[Cell::Text(kind)])?;le...
   |                                                                                                                       ----------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                       |
   |                                                                                                                       this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:13:588
   |
12 | ...)->Result<(),String>{
   |       ----------------- expected `std::string::String` because of this
13 | ...id{..}=>"solid",FillStyle::LinearGradient{..}=>"linearGradient",FillStyle::RadialGradient{..}=>"radialGradient"};p.insert_key("draw_fill",id,&[Cell::Text(kind)])?;let stops=match fill{FillStyle::Solid{color}=>{geometry(p,"draw_fill_solid",id,color)?;None},FillStyle::LinearGradient{x1,y1,x2,y2,stops}=>{geometry(p,"draw_fill_linear",id,&[*x1,*y1,*x2,*y2])?;Some(stops)},FillStyle::RadialGradient{cx,cy,r,stops}=>{geometry(p,"draw_fill_radial",id,&[*cx,*cy,*r])?;Some(stops)}};if let Some(stops)=stops{p.check_rows(stops.len())?;fo...
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           -----------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:13:803
   |
12 | ...)->Result<(),String>{
   |       ----------------- expected `std::string::String` because of this
13 | ...id{..}=>"solid",FillStyle::LinearGradient{..}=>"linearGradient",FillStyle::RadialGradient{..}=>"radialGradient"};p.insert_key("draw_fill",id,&[Cell::Text(kind)])?;let stops=match fill{FillStyle::Solid{color}=>{geometry(p,"draw_fill_solid",id,color)?;None},FillStyle::LinearGradient{x1,y1,x2,y2,stops}=>{geometry(p,"draw_fill_linear",id,&[*x1,*y1,*x2,*y2])?;Some(stops)},FillStyle::RadialGradient{cx,cy,r,stops}=>{geometry(p,"draw_fill_radial",id,&[*cx,*cy,*r])?;Some(stops)}};if let Some(stops)=stops{p.check_rows(stops.len())?;for(i,v)in stops.iter().enumerate(){let mut row=vec![Cell::Integer(id),Cell::Integer(ordinal(i)?)];row.extend(values(p,&[v.offset,v.color[0],v.color[1],v.color[2],v.color[3]])?);p.insert("draw_gradient_stop",&row)?;}}}
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        ---------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:14:252
   |
12 | ...)->Result<(),String>{
   |       ----------------- expected `std::string::String` because of this
13 | ...id{..}=>"solid",FillStyle::LinearGradient{..}=>"linearGradient",FillStyle::RadialGradient{..}=>"radialGradient"};p.insert_key("draw_fill",id,&[Cell::Text(kind)])?;let stops=match fill{FillStyle:...
14 | ....color[1],v.color[2],v.color[3],v.width])?;row.extend([Cell::Text(v.cap.as_str()),Cell::Text(v.join.as_str()),Cell::Integer(i64::from(v.dash.is_some()))]);p.insert_key("draw_stroke",id,&row)?;if...
   |                                                                                                                                                                 ---------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                                                                 |
   |                                                                                                                                                                 this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:14:304
   |
12 | ...)->Result<(),String>{
   |       ----------------- expected `std::string::String` because of this
13 | ...id{..}=>"solid",FillStyle::LinearGradient{..}=>"linearGradient",FillStyle::RadialGradient{..}=>"radialGradient"};p.insert_key("draw_fill",id,&[Cell::Text(kind)])?;let stops=match fill{FillStyle::Solid{color}=>{geometry(p,"draw_fill_solid",id,colo...
14 | ....color[1],v.color[2],v.color[3],v.width])?;row.extend([Cell::Text(v.cap.as_str()),Cell::Text(v.join.as_str()),Cell::Integer(i64::from(v.dash.is_some()))]);p.insert_key("draw_stroke",id,&row)?;if let Some(dash)=&v.dash{p.check_rows(dash.len())?;fo...
   |                                                                                                                                                                                                                                ----------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                                                                                                                                |
   |                                                                                                                                                                                                                                this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:14:470
   |
12 | ...)->Result<(),String>{
   |       ----------------- expected `std::string::String` because of this
13 | ...id{..}=>"solid",FillStyle::LinearGradient{..}=>"linearGradient",FillStyle::RadialGradient{..}=>"radialGradient"};p.insert_key("draw_fill",id,&[Cell::Text(kind)])?;let stops=match fill{FillStyle::Solid{color}=>{geometry(p,"draw_fill_solid",id,color)?;None},FillStyle::LinearGradient{x1,y1,x2,y2,stops}=>{geometry(p,"draw_fill_linear",id,&[*x1,*y1,*x2,*y2])?;Some(stops)},FillStyle::RadialGradient{cx,cy,r,stops}=>...
14 | ....color[1],v.color[2],v.color[3],v.width])?;row.extend([Cell::Text(v.cap.as_str()),Cell::Text(v.join.as_str()),Cell::Integer(i64::from(v.dash.is_some()))]);p.insert_key("draw_stroke",id,&row)?;if let Some(dash)=&v.dash{p.check_rows(dash.len())?;for(i,value)in dash.iter().enumerate(){let value=scalar(p,*value)?;p.insert("draw_stroke_dash",&[Cell::Integer(id),Cell::Integer(ordinal(i)?),Cell::Integer(value)])?;}}}
   |                                                                                                                                                                                                                                                                                                                             -----------------------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                                                                                                                                                                                                                             |
   |                                                                                                                                                                                                                                                                                                                             this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:20:43
   |
19 | fn project(s:&DrawingSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
   |                                                                        ----------------------------- expected `std::string::String` because of this
20 |  let mut p=Projection::new(SCHEMA,control)?;p.insert_key("draw_document",1,&[Cell::Text(&s.schema),Cell::Text(&s.id),s.title.as_dere...
   |            -------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |            |
   |            this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:20:176
   |
19 | ...SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
   |                                ----------------------------- expected `std::string::String` because of this
20 | ...;p.insert_key("draw_document",1,&[Cell::Text(&s.schema),Cell::Text(&s.id),s.title.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;
   |       ---------------------------------------------------------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |       |
   |       this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:22:30
   |
19 | fn project(s:&DrawingSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
   |                                                                        ----------------------------- expected `std::string::String` because of this
...
22 |  p.check_rows(s.assets.len())?;for(i,(key,v))in s.assets.iter().enumerate(){p.insert("draw_asset",&[Cell::Integer(1),Cell::Integer(o...
   |    --------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |    |
   |    this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:22:257
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
22 | ...erate(){p.insert("draw_asset",&[Cell::Integer(1),Cell::Integer(ordinal(i)?),Cell::Text(key),Cell::Text(&v.mime),Cell::Text(&v.data),optional_unsigned(v.width),optional_unsigned(v.height)])?;}
   |              ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |              |
   |              this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:23:30
   |
19 | fn project(s:&DrawingSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
   |                                                                        ----------------------------- expected `std::string::String` because of this
...
23 |  p.check_rows(s.layers.len())?;let mut pending=Vec::new();for(i,node)in s.layers.iter().enumerate().rev(){pending.push((node,None,i));}
   |    --------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |    |
   |    this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:24:62
   |
19 | ...rol:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
   |                                         ----------------------------- expected `std::string::String` because of this
...
24 | ...ending.pop(){p.checkpoint()?;let b=base(node);let opacity=scalar(&mut p,b.opacity)?;let id=p.insert("draw_layer",&[if parent.is_n...
   |                   ------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                   |
   |                   this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:24:505
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
24 | ...=base(node);let opacity=scalar(&mut p,b.opacity)?;let id=p.insert("draw_layer",&[if parent.is_none(){Cel...Cell::Text(b.attributes.fill_rule.as_str())])?;
   |                                                               ---------------------------------------------...---------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                               |
   |                                                               this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:27:90
   |
19 | ...ct(s:&DrawingSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
   |                                                                   ----------------------------- expected `std::string::String` because of this
...
27 | ...gLayerNode::Shape(v)=>{p.insert_key("draw_shape",id,&[Cell::Text(&v.shape_kind)])?;if let Some(v)=&v.rect{geometry(&mut p,"draw_r...
   |                             --------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                             |
   |                             this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:27:482
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
27 | ...Text(&v.shape_kind)])?;if let Some(v)=&v.rect{geometry(&mut p,"draw_rect",id,&[v.x,v.y,v.width,v.height])?;}if let Some(v)=&v.ellipse{geometry(&mut p,"draw_ellipse",id,&[v.cx,v.cy,v.rx,v.ry])?;}if let Some(v)=&v.circle{geometry(&mut p,"draw_circle",id,&[v.cx,v.cy,v.r])?;}if let Some(v)=&v.line{geometry(&mut p,"draw_line",id,&[v.x1,v.y1,v.x2,v.y2])?;}if let Some(v)=&v.polygon{p.insert_key("draw_polygon",id,&[])?;p....
   |                                                                                                                                                                                                                                                                                                                                                                                                ---------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                                                                                                                                                                                                                                                                                                |
   |                                                                                                                                                                                                                                                                                                                                                                                                this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:27:512
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
27 | ...Text(&v.shape_kind)])?;if let Some(v)=&v.rect{geometry(&mut p,"draw_rect",id,&[v.x,v.y,v.width,v.height])?;}if let Some(v)=&v.ellipse{geometry(&mut p,"draw_ellipse",id,&[v.cx,v.cy,v.rx,v.ry])?;}if let Some(v)=&v.circle{geometry(&mut p,"draw_circle",id,&[v.cx,v.cy,v.r])?;}if let Some(v)=&v.line{geometry(&mut p,"draw_line",id,&[v.x1,v.y1,v.x2,v.y2])?;}if let Some(v)=&v.polygon{p.insert_key("draw_polygon",id,&[])?;p.check_rows(v.points.len())?;fo...
   |                                                                                                                                                                                                                                                                                                                                                                                                                                     --------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                                                                                                                                                                                                                                                                                                                                     |
   |                                                                                                                                                                                                                                                                                                                                                                                                                                     this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:27:681
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
27 | ...Text(&v.shape_kind)])?;if let Some(v)=&v.rect{geometry(&mut p,"draw_rect",id,&[v.x,v.y,v.width,v.height])?;}if let Some(v)=&v.ellipse{geometry(&mut p,"draw_ellipse",id,&[v.cx,v.cy,v.rx,v.ry])?;}if let Some(v)=&v.circle{geometry(&mut p,"draw_circle",id,&[v.cx,v.cy,v.r])?;}if let Some(v)=&v.line{geometry(&mut p,"draw_line",id,&[v.x1,v.y1,v.x2,v.y2])?;}if let Some(v)=&v.polygon{p.insert_key("draw_polygon",id,&[])?;p.check_rows(v.points.len())?;for(i,v)in v.points.iter().enumerate(){let mut row=vec![Cell::Integer(id),Cell::Integer(ordinal(i)?)];row.extend(values(&mut p,v)?);p.insert("draw_polygon_point",&row)?;}}},
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       ---------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:28:63
   |
19 | ...shot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
   |                                                  ----------------------------- expected `std::string::String` because of this
...
28 | ...v)=>{p.insert_key("draw_path",id,&[])?;p.check_rows(v.segments.len())?;for(i,v)in v.segments.iter().enumerate(){let(kind,to)=matc...
   |           ------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |           |
   |           this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:28:95
   |
19 | ...hotControl<'_>)->Result<SqliteDatabase,String>{
   |                     ----------------------------- expected `std::string::String` because of this
...
28 | ...,id,&[])?;p.check_rows(v.segments.len())?;for(i,v)in v.segments.iter().enumerate(){let(kind,to)=match v{PathSegment::Move{to}=>("...
   |                ----------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                |
   |                this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
    --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:2681:108
     |
2681 |             return match store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), 1, maximum_bytes)? {
     |                          ----------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
     |                          |
     |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
     |
     = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
     = help: `std::string::String` implements trait `From<T>`:
               From<&mut str>
               From<&std::string::String>
               From<&str>
               From<Cow<'_, str>>
               From<LabelText>
               From<MutationRefusal>
               From<char>
               From<semio_framework_plugin::Version>
               From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:28:506
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
28 | ...eck_rows(v.segments.len())?;for(i,v)in v.segments.iter().enumerate(){let(kind,to)=match v{PathSegment::Move{to}=>("move",Some(to)),PathSegment::Line{to}=>("line",Some(to)),PathSegment::Quad{to,..}=>("quad",Some(to)),PathSegment::Cubic{to,..}=>("cubic",Some(to)),PathSegment::Arc{to,..}=>("arc",Some(to)),PathSegment::Close=>("close",None)};let key=p.insert("draw_segment",&[Cell::Integer(id),Cell::Integer(ordinal(i)?),Cell::Text(kind)])?;if...
   |                                                                                                                                                                                                                                                                                                                                                                  ---------------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                                                                                                                                                                                                                                                                  |
   |                                                                                                                                                                                                                                                                                                                                                                  this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:28:1000
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
28 | ...eck_rows(v.segments.len())?;for(i,v)in v.segments.iter().enumerate(){let(kind,to)=match v{PathSegment::Move{to}=>("move",Some(to)),PathSegment::Line{to}=>("line",Some(to)),PathSegment::Quad{to,..}=>("quad",Some(to)),PathSegment::Cubic{to,..}=>("cubic",Some(to)),PathSegment::Arc{to,..}=>("arc",Some(to)),PathSegment::Close=>("close",None)};let key=p.insert("draw_segment",&[Cell::Integer(id),Cell::Integer(ordinal(i)?),Cell::Text(kind)])?;if let Some(to)=to{geometry(&mut p,"draw_segment_to",key,to)?;}match v{PathSegment::Quad{ctrl,..}=>geometry(&mut p,"draw_segment_quad",key,ctrl)?,PathSegment::Cubic{ctrl1,ctrl2,..}=>geometry(&mut p,"draw_segment_cubic",key,&[ctrl1[0],ctrl1[1],ctrl2[0],ctrl2[1]])?,PathSegment::Arc{rx,ry,rotation,large_arc,sweep,..}=>{let mut row=values(&mut p,&[*rx,*ry,*rotation])?;row.extend([Cell::Integer(i64::from(*large_arc)),Cell::Integer(i64::from(*sweep))]);p.insert_key("draw_segment_arc",key,&row)?;},...
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                ---------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:29:223
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
29 | ...(&mut p,v.y)?;let size=scalar(&mut p,v.size)?;p.insert_key("draw_text",id,&[Cell::Integer(x),Cell::Integer(y),Cell::Text(&v.content),Cell::Integer(size)])?;},
   |                                                    ----------------------------------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                    |
   |                                                    this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:30:202
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
30 | ... height=scalar(&mut p,v.height)?;p.insert_key("draw_image",id,&[Cell::Text(&v.image_key),Cell::Integer(width),Cell::Integer(height)])?;},
   |                                       --------------------------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                       |
   |                                       this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:31:102
   |
19 | ...(s:&DrawingSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
   |                                                                 ----------------------------- expected `std::string::String` because of this
...
31 | ...ayerNode::Group(v)=>{p.insert_key("draw_group",id,&[Cell::Integer(i64::from(v.isolation))])?;p.check_rows(pending.len().checked_a...
   |                           --------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                           |
   |                           this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:31:194
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
31 | ...Integer(i64::from(v.isolation))])?;p.check_rows(pending.len().checked_add(v.children.len()).ok_or("Draw frontier overflow")?)?;fo...
   |                                         ----------------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                         |
   |                                         this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:32:93
   |
19 | ...(s:&DrawingSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
   |                                                                 ----------------------------- expected `std::string::String` because of this
...
32 | ...ayerNode::Boolean(v)=>{p.insert_key("draw_boolean",id,&[Cell::Text(&v.operation)])?;p.check_rows(v.children.len())?;for(i,v)in v....
   |                             ---------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                             |
   |                             this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:32:125
   |
19 | ... SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
   |                                 ----------------------------- expected `std::string::String` because of this
...
32 | ...key("draw_boolean",id,&[Cell::Text(&v.operation)])?;p.check_rows(v.children.len())?;for(i,v)in v.children.iter().enumerate(){p.in...
   |                                                          ----------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                          |
   |                                                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
    --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:2767:108
     |
2767 |             return match store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), 1, maximum_bytes)? {
     |                          ----------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
     |                          |
     |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
     |
     = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
     = help: `std::string::String` implements trait `From<T>`:
               From<&mut str>
               From<&std::string::String>
               From<&str>
               From<Cow<'_, str>>
               From<LabelText>
               From<MutationRefusal>
               From<char>
               From<semio_framework_plugin::Version>
               From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:32:260
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
32 | ...ll::Text(&v.operation)])?;p.check_rows(v.children.len())?;for(i,v)in v.children.iter().enumerate(){p.insert("draw_boolean_child",&[Cell::Integer(id),Cell::Integer(ordinal(i)?),Cell::Text(v)])?;}},
   |                                                                                                         ------------------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                         |
   |                                                                                                         this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:33:243
   |
19 | ...)->Result<SqliteDatabase,String>{
   |       ----------------------------- expected `std::string::String` because of this
...
33 | ...threshold)?;let simplify=scalar(&mut p,v.params.simplify_epsilon)?;p.insert_key("draw_trace",id,&[Cell::Text(&v.source_key),Cell::Integer(threshold),Cell::Integer(simplify)])?;}
   |                                                                         ---------------------------------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                         |
   |                                                                         this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0308]: mismatched types
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:36:2
   |
19 | fn project(s:&DrawingSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
   |                                                                        ----------------------------- expected `Result<SqliteDatabase, std::string::String>` because of return type
...
36 |  p.finish()
   |  ^^^^^^^^^^ expected `Result<SqliteDatabase, String>`, found `Result<SqliteDatabase, ValueError>`
   |
   = note: expected enum `Result<_, std::string::String>`
              found enum `Result<_, ValueError>`
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
    --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:2824:108
     |
2824 |             return match store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), 1, maximum_bytes)? {
     |                          ----------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
     |                          |
     |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
     |
     = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
     = help: `std::string::String` implements trait `From<T>`:
               From<&mut str>
               From<&std::string::String>
               From<&str>
               From<Cow<'_, str>>
               From<LabelText>
               From<MutationRefusal>
               From<char>
               From<semio_framework_plugin::Version>
               From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:41:174
   |
41 | ...ult<Self,String>{control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(datab...
   |                             -----------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                             |
   |                             this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
    --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:2877:108
     |
2877 |             return match store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), 1, maximum_bytes)? {
     |                          ----------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
     |                          |
     |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
     |
     = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
     = help: `std::string::String` implements trait `From<T>`:
               From<&mut str>
               From<&std::string::String>
               From<&str>
               From<Cow<'_, str>>
               From<LabelText>
               From<MutationRefusal>
               From<char>
               From<semio_framework_plugin::Version>
               From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:41:384
   |
41 | ...r(i,row)in database.table(name)?.rows.iter().enumerate(){if row.rowid<=0||row.values.len()!=count||row.integer(0)?!=row.rowid||ma...
   |                        -----------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                        |
   |                        this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:41:466
   |
41 | ...es.len()!=count||row.integer(0)?!=row.rowid||map.insert(row.rowid,row).is_some(){return Err("Draw row identity or fields".into())...
   |                         ----------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                         |
   |                         this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:41:642
   |
41 | ...into());}if i%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,i,0)?;}}rows.insert(name,map);}Ok(Self{rows,grou...
   |                                 --------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                 |
   |                                 this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:44:104
   |
44 |  fn text(&mut self,row:&SqliteRow,index:usize)->Result<String,String>{Reconstruction::new(self.control)?.text(row.text(index)?)}
   |                                                                       ---------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                       |
   |                                                                       this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:44:126
   |
44 |  fn text(&mut self,row:&SqliteRow,index:usize)->Result<String,String>{Reconstruction::new(self.control)?.text(row.text(index)?)}
   |                                                                                                                   -----------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                   |
   |                                                                                                                   this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
    --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:2048:54
     |
2048 |         match retirement.close_step(1, maximum_bytes)? {
     |                          ----------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
     |                          |
     |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
     |
     = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
     = help: `std::string::String` implements trait `From<T>`:
               From<&mut str>
               From<&std::string::String>
               From<&str>
               From<Cow<'_, str>>
               From<LabelText>
               From<MutationRefusal>
               From<char>
               From<semio_framework_plugin::Version>
               From<std::boxed::Box<str>>
```

```text
error[E0308]: mismatched types
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:44:71
   |
44 |  fn text(&mut self,row:&SqliteRow,index:usize)->Result<String,String>{Reconstruction::new(self.control)?.text(row.text(index)?)}
   |                                                 --------------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<String, String>`, found `Result<String, ValueError>`
   |                                                 |
   |                                                 expected `Result<std::string::String, std::string::String>` because of return type
   |
   = note: expected enum `Result<_, std::string::String>`
              found enum `Result<_, ValueError>`
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:47:86
   |
47 | ...,String>{match row.integer(index)?{0=>Ok(false),1=>Ok(true),_=>Err("Draw boolean".into())}}
   |                       --------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                       |
   |                       this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:48:151
   |
48 | ...{u32::try_from(row.integer(index)?).map(Some).map_err(|e|e.to_string())}}
   |                       --------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                       |
   |                       this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:49:118
   |
49 | ...("draw_scalar",row.integer(index)?)?;Reconstruction::new(self.control)?.scalar()?;FloatRow::new(row,&[FloatColumn::Binary64(1)])?...
   |                       --------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                       |
   |                       this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:49:155
   |
49 | ...ndex)?)?;Reconstruction::new(self.control)?.scalar()?;FloatRow::new(row,&[FloatColumn::Binary64(1)])?.real(1)}
   |             ---------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |             |
   |             this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:49:165
   |
49 | ...n::new(self.control)?.scalar()?;FloatRow::new(row,&[FloatColumn::Binary64(1)])?.real(1)}
   |                          --------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                          |
   |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:49:213
   |
49 | ...)?;FloatRow::new(row,&[FloatColumn::Binary64(1)])?.real(1)}
   |       ----------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |       |
   |       this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0308]: mismatched types
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:49:167
   |
49 | ...)->Result<f64,String>{let row=self.take("draw_scalar",row.integer(index)?)?;Reconstruction::new(self.control)?.scalar()?;FloatRow::new(row,&[FloatColumn::Binary64(1)])?.real(1)}
   |       ------------------ expected `Result<f64, std::string::String>` because of return type                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<f64, String>`, found `Result<f64, ValueError>`
   |
   = note: expected enum `Result<_, std::string::String>`
              found enum `Result<_, ValueError>`
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:51:319
   |
51 | ...;self.control.check_rows(source.len())?;for(i,row)in source.values().enumerate(){if !Self::null(row,index)?{groups.entry(row.inte...
   |                  ------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                  |
   |                  this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:51:420
   |
51 | ...?{groups.entry(row.integer(index)?).or_default().push(*row);}if i%256==0{self.control.checkpoint(SqliteSnapshotPhase::Reconstruct...
   |                       --------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                       |
   |                       this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:51:540
   |
51 | ...if i%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,i,source.len())?;}}self.groups.insert(key,groups);}l...
   |                             -------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                             |
   |                             this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:51:735
   |
51 | ...urce{let order=row.integer(order)?;if order<0||sorted.insert(order,row).is_some(){return Err("Draw duplicate or negative order".i...
   |                       --------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                       |
   |                       this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:51:1131
   |
51 | ...ow);if i%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,i,0)?;}}Ok(values)}
   |                                 --------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                 |
   |                                 this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:52:232
   |
52 | ...",id)?;Some(match row.text(1)?{"solid"=>{let v=self.take("draw_fill_solid",id)?;FillStyle::Solid{color:self.vector(v,1)?}},"linea...
   |                          -------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                          |
   |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:52:530
   |
52 | ...tor(row,4)?});}if row.text(1)?=="linearGradient"{let v=self.take("draw_fill_linear",id)?;FillStyle::LinearGradient{x1:self.real(v...
   |                          -------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                          |
   |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
   --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:402:80
    |
402 |             return match active.close_step(maximum_items.min(1), maximum_bytes)? {
    |                                 -----------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
    |                                 |
    |                                 this can't be annotated with `?` because it has type `Result<_, ValueError>`
    |
    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
    = help: `std::string::String` implements trait `From<T>`:
              From<&mut str>
              From<&std::string::String>
              From<&str>
              From<Cow<'_, str>>
              From<LabelText>
              From<MutationRefusal>
              From<char>
              From<semio_framework_plugin::Version>
              From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:52:1232
   |
52 | ...:StrokeCap::parse(row.text(6)?).map_err(str::to_owned)?,join:StrokeJoin::parse(row.text(7)?).map_err(str::to_owned)?,dash})}else{...
   |                          -------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                          |
   |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:52:1293
   |
52 | ...StrokeJoin::parse(row.text(7)?).map_err(str::to_owned)?,dash})}else{None};Ok(DrawingAttributes{fill_rule,fill,stroke})}
   |                          -------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                          |
   |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
   --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:444:65
    |
444 |             return match retirement.close_step(1, maximum_bytes)? {
    |                                     ----------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
    |                                     |
    |                                     this can't be annotated with `?` because it has type `Result<_, ValueError>`
    |
    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
    = help: `std::string::String` implements trait `From<T>`:
              From<&mut str>
              From<&std::string::String>
              From<&str>
              From<Cow<'_, str>>
              From<LabelText>
              From<MutationRefusal>
              From<char>
              From<semio_framework_plugin::Version>
              From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:53:593
   |
53 | ...ributes(row.rowid,row.text(11)?)?})}
   |                          --------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                          |
   |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:54:131
   |
54 |  fn leaf(&mut self,row:&SqliteRow)->Result<DrawingLayerNode,String>{let id=row.rowid;let base=self.base(row)?;Ok(match row.text(4)?{
   |                                                                                                                            -------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                            |
   |                                                                                                                            this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
    --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:5202:65
     |
5202 |             return match retirement.close_step(1, maximum_bytes)? {
     |                                     ----------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
     |                                     |
     |                                     this can't be annotated with `?` because it has type `Result<_, ValueError>`
     |
     = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
     = help: `std::string::String` implements trait `From<T>`:
               From<&mut str>
               From<&std::string::String>
               From<&str>
               From<Cow<'_, str>>
               From<LabelText>
               From<MutationRefusal>
               From<char>
               From<semio_framework_plugin::Version>
               From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:56:150
   |
56 | ...ow.rowid;let kind=row.text(3)?;let to=if kind=="close"{[0.0;2]}else{let v=self.take("draw_segment_to",key)?;self.vector(v,1)?};se...
   |                          -------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                          |
   |                          this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
    --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:5477:62
     |
5477 |         match active.close_step(1, DRAWING_OWNED_FIELD_BYTES)? {
     |                      ----------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
     |                      |
     |                      this can't be annotated with `?` because it has type `Result<_, ValueError>`
     |
     = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
     = help: `std::string::String` implements trait `From<T>`:
               From<&mut str>
               From<&std::string::String>
               From<&str>
               From<Cow<'_, str>>
               From<LabelText>
               From<MutationRefusal>
               From<char>
               From<semio_framework_plugin::Version>
               From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
    --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:5529:102
     |
5529 |             match runtime.close_step(&DrawingSnapshotRetirementFactory, 1, DRAWING_OWNED_FIELD_BYTES)? {
     |                           ---------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
     |                           |
     |                           this can't be annotated with `?` because it has type `Result<_, ValueError>`
     |
     = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
     = help: `std::string::String` implements trait `From<T>`:
               From<&mut str>
               From<&std::string::String>
               From<&str>
               From<Cow<'_, str>>
               From<LabelText>
               From<MutationRefusal>
               From<char>
               From<semio_framework_plugin::Version>
               From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:71:103
   |
66 | ...se:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<DrawingSnapshot,String>{
   |                                                                ------------------------------ expected `std::string::String` because of this
...
71 | ...=pending.pop(){r.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;match task{Task::Node(row)=>{let node=r.leaf(r...
   |                             --------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                             |
   |                             this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
    --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:5556:77
     |
5556 |             return match retirement.close_step(1, DRAWING_OWNED_FIELD_BYTES)? {
     |                                     ----------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
     |                                     |
     |                                     this can't be annotated with `?` because it has type `Result<_, ValueError>`
     |
     = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
     = help: `std::string::String` implements trait `From<T>`:
               From<&mut str>
               From<&std::string::String>
               From<&str>
               From<Cow<'_, str>>
               From<LabelText>
               From<MutationRefusal>
               From<char>
               From<semio_framework_plugin::Version>
               From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:71:202
   |
66 | ...)->Result<DrawingSnapshot,String>{
   |       ------------------------------ expected `std::string::String` because of this
...
71 | ...ReconstructSnapshot,0,0)?;match task{Task::Node(row)=>{let node=r.leaf(row)?;forest.0.insert(row.rowid,node);if row.text(4)?=="gr...
   |                                                                                                                        -------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                        |
   |                                                                                                                        this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:71:358
   |
66 | ...)->Result<DrawingSnapshot,String>{
   |       ------------------------------ expected `std::string::String` because of this
...
71 | ...ReconstructSnapshot,0,0)?;match task{Task::Node(row)=>{let node=r.leaf(row)?;forest.0.insert(row.rowid,node);if row.text(4)?=="group"{let children=r.list("draw_layer",row.rowid,2,3)?;r.control.check_rows(pending.len().checked_add(children.len()).ok_or("Draw frontier overflow")?)?;le...
   |                                                                                                                                                                                                     --------------------------------------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                                                                                                                                                                                     |
   |                                                                                                                                                                                                     this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0277]: `?` couldn't convert the error to `std::string::String`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:72:152
   |
66 | ...eSnapshotControl<'_>)->Result<DrawingSnapshot,String>{
   |                           ------------------------------ expected `std::string::String` because of this
...
72 | ...an Draw entities".into());}r.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;for id in root_ids{s.layers.push(f...
   |                                         --------------------------------------------------------^ the trait `From<ValueError>` is not implemented for `std::string::String`
   |                                         |
   |                                         this can't be annotated with `?` because it has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `std::string::String` implements trait `From<T>`:
             From<&mut str>
             From<&std::string::String>
             From<&str>
             From<Cow<'_, str>>
             From<LabelText>
             From<MutationRefusal>
             From<char>
             From<semio_framework_plugin::Version>
             From<std::boxed::Box<str>>
```

```text
error[E0271]: expected `__dsl_from_record_controlled` to return `Result<_, TextError>`, but it returns `Result<DrawingSnapshot, ValueError>`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:79:257
   |
79 | ...g>{store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),Self::__dsl_from_record_controlled,co...
   |       ------------------------------------------- required by a bound introduced by this call                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<DrawingSnapshot, TextError>`, found `Result<DrawingSnapshot, ValueError>`
   |
   = note: expected enum `Result<_, TextError>`
              found enum `Result<_, ValueError>`
note: required by a bound in `decode_sqlite_snapshot_record_native`
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs:18:75
   |
14 | pub fn decode_sqlite_snapshot_record_native<T>(
   |        ------------------------------------ required by a bound in this function
...
18 |     construct: impl FnOnce(&RecordValue, &mut NativeDecodeControl<'_>) -> Result<T, TextError>,
   |                                                                           ^^^^^^^^^^^^^^^^^^^^ required by this bound in `decode_sqlite_snapshot_record_native`
```

```text
error[E0308]: mismatched types
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:79:133
   |
79 | ...)->Result<Self,String>{store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),Self::__dsl_from_record_controlled,control)}
   |       ------------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<DrawingSnapshot, String>`, found `Result<_, ValueError>`
   |       |
   |       expected `Result<DrawingSnapshot, String>` because of return type
   |
   = note: expected enum `Result<standards::v1::subsets::any::schema::snapshot::component::DrawingSnapshot, std::string::String>`
              found enum `Result<_, ValueError>`
   = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-draw-drawing/b2e4b87268018701/out/semio_s_artifact_draw_drawing-b2e4b87268018701.long-type-11382450120386072369.txt'
   = note: consider using `--verbose` to print the full type name to the console
help: use the `?` operator to extract the `Result<_, ValueError>` value, propagating a `Result::Err` value to the caller
   |
79 |  fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),Self::__dsl_from_record_controlled,control)?}
   |                                                                                                                                                                                                                                                                                                            +
```

```text
error[E0308]: mismatched types
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:80:308
   |
80 | ...ve|self.__dsl_to_record_controlled(native),control)}
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<RecordValue, TextError>`, found `Result<RecordValue, ValueError>`
   |
   = note: expected enum `Result<_, TextError>`
              found enum `Result<_, ValueError>`
```

```text
error[E0308]: mismatched types
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:80:175
   |
80 | ...)->Result<store::io_schema::IoPayload,String>{store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
   |       ------------------------------------------ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<IoPayload, String>`, found `Result<IoPayload, ValueError>`
   |       |
   |       expected `Result<IoPayload, std::string::String>` because of return type
   |
   = note: expected enum `Result<_, std::string::String>`
              found enum `Result<_, ValueError>`
```

```text
error[E0277]: the trait bound `dsl::semio_framework_io_schema::IoError: From<std::string::String>` is not satisfied
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:81:344
   |
81 | ...c subset").into())}validate_sqlite_database_schema(database,SCHEMA,control.limits()).map_err(|e|e.to_string())?;Ok(store::io_sche...
   |               ^^^^ the trait `From<std::string::String>` is not implemented for `dsl::semio_framework_io_schema::IoError`
   |
   = note: required for `std::string::String` to implement `Into<dsl::semio_framework_io_schema::IoError>`
```

```text
error[E0277]: `?` couldn't convert the error to `dsl::semio_framework_io_schema::IoError`
  --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:81:443
   |
81 | ...bset").into())}validate_sqlite_database_schema(database,SCHEMA,control.limits()).map_err(|e|e.to_string())?;Ok(store::io_schema::...
   |                   ----------------------------------------------------------------- -------------------------^ the trait `From<std::string::String>` is not implemented for `dsl::semio_framework_io_schema::IoError`
   |                   |                                                                 |
   |                   |                                                                 this can't be annotated with `?` because it has type `Result<_, std::string::String>`
   |                   this has type `Result<_, ValueError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
```

```text
error[E0308]: `?` operator has incompatible types
   --> 🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🦀️.rs:154:161
    |
154 | ...ome(self.raster.as_mut().unwrap().result().map_err(|e|invalid(e.to_string()))?);self.raster=None;self.phase="complete";}}
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `RasterImage`, found `&RasterImage`
    |
    = note: `?` operator cannot convert from `&semio_framework_pixels::RasterImage` to `semio_framework_pixels::RasterImage`
help: consider using clone here
    |
154 |    "raster"=>{let p=self.raster.as_mut().unwrap().advance(1).map_err(|e|invalid(e.to_string()))?;let done=p.done;self.rendered=Some(p);if done{self.output=Some(self.raster.as_mut().unwrap().result().map_err(|e|invalid(e.to_string()))?.clone());self.raster=None;self.phase="complete";}}
    |                                                                                                                                                                                                                                           ++++++++

[cargo:build] running elapsedMs=430102
```

