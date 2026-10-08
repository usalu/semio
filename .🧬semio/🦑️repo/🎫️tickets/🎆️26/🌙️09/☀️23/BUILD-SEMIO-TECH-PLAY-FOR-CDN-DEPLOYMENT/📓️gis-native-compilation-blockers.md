# GIS Native Test Compilation Blockers

Actual full default-feature native target exited 1 during test compilation after repaired production library compilation. No native cases executed. Remaining diagnostic excerpts follow.


~~~text
error[E0432]: unresolved import `crate::document_dsl`
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:2:13
  |
2 | use crate::{document_dsl, MapFeature};
  |             ^^^^^^^^^^^^ no `document_dsl` in the root
error[E0432]: unresolved import `crate::schema::gis_map_descriptor_json`
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:2:21
  |
2 | use crate::schema::{gis_map_descriptor_json};
  |                     ^^^^^^^^^^^^^^^^^^^^^^^ no `gis_map_descriptor_json` in `schema`
  |
help: a similar name exists in the module
  |
2 - use crate::schema::{gis_map_descriptor_json};
~~~

~~~text
error[E0432]: unresolved import `crate::schema::gis_map_descriptor_json`
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:2:21
  |
2 | use crate::schema::{gis_map_descriptor_json};
  |                     ^^^^^^^^^^^^^^^^^^^^^^^ no `gis_map_descriptor_json` in `schema`
  |
help: a similar name exists in the module
  |
2 - use crate::schema::{gis_map_descriptor_json};
2 + use crate::schema::{gis_map_descriptor_value};
  |
error[E0425]: cannot find type `Value` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️relocated-engine/🦀️.rs:45:40
   |
~~~

~~~text
error[E0425]: cannot find type `Value` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️relocated-engine/🦀️.rs:45:40
   |
45 |     let value = serde_json::from_str::<Value>(&semio_framework_pack_json::to_json_string(&document)).expect("document json");
   |                                        ^^^^^ not found in this scope
   |
note: struct `crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::Value` exists but is inaccessible
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/📦️pack/🦀️.rs:21:56
   |
21 | #[derive(semio_framework_dsl_record_derive::DslRecord)]struct Value{kind:Kind,boolean:Option<bool>,unsigned:Option<u64>,signed:Optio...
   |                                                        ^^^^^^^^^^^^ not accessible
help: consider importing one of these items
   |
 1 + use geojson::Value;
~~~

~~~text
error[E0425]: cannot find type `Value` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️relocated-engine/🦀️.rs:55:40
   |
55 |     let value = serde_json::from_str::<Value>(&semio_framework_pack_json::to_json_string(&GisMapSnapshot::default())).expect("empty ...
   |                                        ^^^^^ not found in this scope
   |
note: struct `crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::Value` exists but is inaccessible
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/📦️pack/🦀️.rs:21:56
   |
21 | #[derive(semio_framework_dsl_record_derive::DslRecord)]struct Value{kind:Kind,boolean:Option<bool>,unsigned:Option<u64>,signed:Optio...
   |                                                        ^^^^^^^^^^^^ not accessible
help: consider importing one of these items
   |
 1 + use geojson::Value;
~~~

~~~text
error[E0425]: cannot find function `gis_map_document_store_owners` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:49:47
   |
49 |     store.install_document_store_owners_exact(gis_map_document_store_owners());
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function
   |
 1 + use crate::host::owned::gis_map_document_store_owners;
   |
error[E0425]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:61:123
   |
61 | ...emio_framework_job::Generation) -> GisMapStoreInitializationAuthority {
~~~

~~~text
error[E0425]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:61:123
   |
61 | ...emio_framework_job::Generation) -> GisMapStoreInitializationAuthority {
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
note: struct `crate::host::owned::GisMapStoreInitializationAuthority` exists but is inaccessible
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:16:1
   |
16 | struct GisMapStoreInitializationAuthority {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible
error[E0433]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:63:5
   |
~~~

~~~text
error[E0433]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:63:5
   |
63 |     GisMapStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `GisMapStoreInitializationAuthority`
   |
note: struct `crate::host::owned::GisMapStoreInitializationAuthority` exists but is inaccessible
  --> 
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:16:1
   |
16 | struct GisMapStoreInitializationAuthority {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible
error[E0425]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:66:46
~~~

~~~text
error[E0425]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:66:46
   |
66 | fn drive_gis_map_initializer(authority: &mut GisMapStoreInitializationAuthority, operation: semio_framework_job::OperationId, genera...
   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
note: struct `crate::host::owned::GisMapStoreInitializationAuthority` exists but is inaccessible
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:16:1
   |
16 | struct GisMapStoreInitializationAuthority {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible
error[E0433]: cannot find type `GisMapStoreInitializationAuthority` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:258:29
    |
~~~

~~~text
error[E0433]: cannot find type `GisMapStoreInitializationAuthority` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:258:29
    |
258 | ...   let mut authority = GisMapStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCA...
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `GisMapStoreInitializationAuthority`
    |
note: struct `crate::host::owned::GisMapStoreInitializationAuthority` exists but is inaccessible
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:16:1
    |
 16 | struct GisMapStoreInitializationAuthority {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible
help: a trait with a similar name exists
    |
258 -         let mut authority = GisMapStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()));
~~~

~~~text
error[E0433]: cannot find `owned_pack` in `snapshot`
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:42:301
   |
42 | ...::subsets::any::io::sqlite::snapshot::owned_pack::retire_value(old);
   |                                          
^^^^^^^^^^ could not find `owned_pack` in `snapshot`
   |
help: consider importing this module
   |
 1 + use crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack;
   |
help: if you import `owned_pack`, refer to it directly
   |
42 -  let mut value=fixture();let old=std::mem::replace(&mut value.positions[0].data,semio_framework_value::DslValue::Array((0..600).map(|i|semio_framework_value::DslValue::String(if i==0{"long 世界".repeat(20000)}else{"child".into()})).collect()));crate::standards::v1::subsets::any::io::sqlite::snapshot::owned_pack::retire_value(old);
~~~

~~~text
error[E0433]: cannot find `owned_pack` in `snapshot`
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:84:215
   |
84 | ...::subsets::any::io::sqlite::snapshot::owned_pack::retire_value(old);
   |                                          ^^^^^^^^^^ could not find `owned_pack` in `snapshot`
   |
help: consider importing this module
   |
 1 + use crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack;
   |
help: if you import `owned_pack`, refer to it directly
   |
84 -  let mut snapshot=fixture();let old=std::mem::replace(&mut snapshot.positions[0].data,semio_framework_value::DslValue::String("interior 世界".repeat(20000)));crate::standards::v1::subsets::any::io::sqlite::snapshot::owned_pack::retire_value(old);
84 +  let mut snapshot=fixture();let old=std::mem::replace(&mut snapshot.positions[0].data,semio_framework_value::DslValue::String("interior 世界".repeat(20000)));owned_pack::retire_value(old);
~~~
