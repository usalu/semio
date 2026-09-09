# Native 948 Diagnostics

error: large size difference between variants
  --> ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🦀️.rs:15:1
   |
15 | / pub enum ArtifactAssembly {
16 | |     Definition(ArtifactDefinition),
   | |     ------------------------------ the second-largest variant contains at least 72 bytes
17 | |     Runtime(ArtifactDeclaration),
   | |     ---------------------------- the largest variant contains at least 408 bytes
18 | | }
   | |_^ the entire enum is at least 408 bytes
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
   = note: `-D clippy::large-enum-variant` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`
help: consider boxing the large fields or introducing indirection in some other way to reduce the total size of the enum
   |
17 -     Runtime(ArtifactDeclaration),
17 +     Runtime(Box<ArtifactDeclaration>),
   |


error: this argument is passed by value, but not consumed in the function body
   --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:132:36
    |
132 |     pub fn from_snapshot(snapshot: crate::En1992Snapshot) -> Self {
    |                                    ^^^^^^^^^^^^^^^^^^^^^
    |
help: or consider marking this type as `Copy`
   --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:20:1
    |
 20 | pub struct En1992Snapshot {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`
help: consider taking a reference instead
    |
132 |     pub fn from_snapshot(snapshot: &crate::En1992Snapshot) -> Self {
    |                                    +


error: calls to `push` immediately after creation
   --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:156:9
    |
156 | /         let mut mutations = Vec::with_capacity(35);
157 | |         mutations.push(En1992Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: snapshot.annex }));
158 | |         mutations.push(En1992Mutation::ChangeACMm2(change_a_c_mm2::ChangeACMm2 { new_a_c_mm2: snapshot.a_c_mm2 }));
159 | |         mutations.push(En1992Mutation::ChangeAnchorASMm2(change_anchor_a_s_mm2::ChangeAnchorASMm2 { new_anchor_a_s_mm2: snapshot....
...   |
190 | |         mutations.push(En1992Mutation::ChangeUseFem(change_use_fem::ChangeUseFem { new_use_fem: snapshot.use_fem }));
191 | |         mutations.push(En1992Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: snapshot.v_ed_kn }));
    | |___________________________________________________________________________________________________________________^ help: consider using the `vec![]` macro: `let mutations = vec![..];`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#vec_init_then_push
    = note: `-D clippy::vec-init-then-push` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::vec_init_then_push)]`


error: very complex type used. Consider factoring parts into `type` definitions
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️.rs:364:20
    |
364 |     read_artifact: &dyn Fn(&str) -> Result<(Vec<u8>, Vec<u8>), SpaceZipError>,
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity
    = note: `-D clippy::type-complexity` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::type_complexity)]`


error: this argument is passed by value, but not consumed in the function body
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️.rs:495:87
    |
495 | pub fn import_blob<B: store::BlobStore>(blob_store: &B, blob: &store::BlobRef, bytes: Vec<u8>) -> Result<(), SpaceZipError> {
    |                                                                                       ^^^^^^^ help: consider changing the type to: `&[u8]`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
    = note: `-D clippy::needless-pass-by-value` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`


error: called `map(<f>).unwrap_or_else(<g>)` on an `Option` value
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1372:26
     |
1372 |             let folder = path.parent().map(|parent| parent.to_path_buf()).unwrap_or_else(|| std::path::PathBuf::from("."));
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `path.parent().map_or_else(|| std::path::PathBuf::from("."), |parent| parent.to_path_buf())`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
     = note: `-D clippy::map-unwrap-or` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::map_unwrap_or)]`


error: called `map(<f>).unwrap_or(<a>)` on a `Result` value
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🎠️activation/🦀️.rs:36:21
   |
36 |         let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: use `map_or(<a>, <f>)` instead
   |
36 -         let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
36 +         let cores = std::thread::available_parallelism().map_or(1, |n| n.get());
   |


error: called `map(<f>).unwrap_or(<a>)` on an `Option` value
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🎠️activation/🦀️.rs:111:24
    |
111 |             let lane = grant.envelopes.first().map(|envelope| envelope.lane).unwrap_or(Lane::Maintenance);
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or
help: use `map_or(<a>, <f>)` instead
    |
111 -             let lane = grant.envelopes.first().map(|envelope| envelope.lane).unwrap_or(Lane::Maintenance);
111 +             let lane = grant.envelopes.first().map_or(Lane::Maintenance, |envelope| envelope.lane);
    |


error: calls to `push` immediately after creation
   --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:242:9
    |
242 | /         let mut mutations = Vec::with_capacity(62);
243 | |         mutations.push(Din16798Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: snapshot.annex }));
244 | |         mutations.push(Din16798Mutation::ChangeOccupancy(change_occupancy::ChangeOccupancy { new_occupancy: snapshot.occupancy.clone() }));
245 | |         mutations.push(Din16798Mutation::ChangeComfortCategory(change_comfort_category::ChangeComfortCategory { new_comfort_category: snapshot.comfort_category.clone() }));
...   |
303 | |         mutations.push(Din16798Mutation::ChangeDuctTestPressurePa(change_duct_test_pressure_pa::ChangeDuctTestPressurePa { new_duct_test_pressure_pa: snapshot.duct_test_pressure_pa ...
304 | |         mutations.push(Din16798Mutation::ChangeDuctLeakageM3SM2(change_duct_leakage_m3_s_m2::ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2: snapshot.duct_leakage_m3_s_m2 }));
    | |__________________________________________________________________________________________________________________________________________________________________________________^ help: consider using the `vec![]` macro: `let mutations = vec![..];`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#vec_init_then_push
    = note: `-D clippy::vec-init-then-push` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::vec_init_then_push)]`


error: this function's return value is unnecessarily wrapped by `Result`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/../../🧬️schema/🦀️.rs:181:5
    |
181 |     pub fn from_value(value: dsl_core::DslValue) -> Result<Option<super::JsonValue>, dsl_core::ValueError> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
    = note: `-D clippy::unnecessary-wraps` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
    |
181 -     pub fn from_value(value: dsl_core::DslValue) -> Result<Option<super::JsonValue>, dsl_core::ValueError> {
181 +     pub fn from_value(value: dsl_core::DslValue) -> std::option::Option<serde_json::Value> {
    |
help: ...and then remove the surrounding `Ok()` from returning expressions
    |
183 ~             dsl_core::DslValue::Null => None,
184 ~             other => Some(super::JsonValue::from(other)),
    |


error: calls to `push` immediately after creation
   --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:130:9
    |
130 | /         let mut mutations = Vec::new();
131 | |         mutations.push(Vdi3805Mutation::UpdateManufacturerFile(update_manufacturer_file::UpdateManufacturerFile { new_manufacture...
132 | |         mutations.push(Vdi3805Mutation::ChangeCorrectionAsOf(change_correction_as_of::ChangeCorrectionAsOf { new_correction_as_of...
133 | |         mutations.push(Vdi3805Mutation::ChangeStrictMode(change_strict_mode::ChangeStrictMode { new_strict_mode: target.strict_mo...
134 | |         mutations.push(Vdi3805Mutation::UpdateLimits(update_limits::UpdateLimits { new_limits: target.limits }));
    | |_________________________________________________________________________________________________________________^ help: consider using the `vec![]` macro: `let mut mutations = vec![..];`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#vec_init_then_push
    = note: `-D clippy::vec-init-then-push` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::vec_init_then_push)]`


error: couldn't read `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/../../🔌️add-geometry-connection/🚰️attaches-the-bb4870/🦀️.rs`: No such file or directory (os error 2)
 --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs:3:1
  |
3 | mod tests_add_geometry_connection_attaches_the_drain_connection_to_geom_valve_50;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: couldn't read `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/../../🔒️change-airtightness-class/🔒️upgrades-the-8f5d1d/🦀️.rs`: No such file or directory (os error 2)
 --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs:3:1
  |
3 | mod tests_change_airtightness_class_upgrades_the_airtightness_class_to_class1;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: couldn't read `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/../../🪁️change-air-speed-ms/🪁️doubles-the-e80788/🦀️.rs`: No such file or directory (os error 2)
 --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs:3:1
  |
3 | mod tests_change_air_speed_m_s_doubles_the_draught_air_speed_to_0_point_25_ms;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: couldn't read `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/../../📐️change-ac-mm2/📐️raises-a-c-mm2-to-168750-0/🦀️.rs`: No such file or directory (os error 2)
 --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs:3:1
  |
3 | mod tests_change_a_c_mm2_raises;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: couldn't read `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🧪️tests/🔬️unit/../../../🧫️fixtures/📦️assembly/🔣️.json`: No such file or directory (os error 2)
  --> ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🧪️tests/🔬️unit/🦀️.rs:13:59
   |
13 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📦️assembly/🔣️.json")).expect("assembly f...
   |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: there is a file with the same name in a different directory
   |
13 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📦️assembly/🔣️.json")).expect("assembly fixture");
13 +     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫\u{fe0f}fixtures/📦\u{fe0f}assembly/🔣\u{fe0f}.json")).expect("assembly fixture");
   |


error: unnecessary qualification
  --> ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🧪️tests/🔬️unit/🦀️.rs:17:17
   |
17 |     let bytes = std::mem::size_of::<ArtifactAssembly>();
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-qualifications` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(unused_qualifications)]`
help: remove the unnecessary path segments
   |
17 -     let bytes = std::mem::size_of::<ArtifactAssembly>();
17 +     let bytes = size_of::<ArtifactAssembly>();
   |


error[E0433]: cannot find module or crate `serde_json` in this scope
  --> ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🧪️tests/🔬️unit/🦀️.rs:13:38
   |
13 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📦️assembly/🔣️.json")).expect("assembly f...
   |                                      ^^^^^^^^^^ use of unresolved module or unlinked crate `serde_json`
   |
   = help: if you wanted to use a crate named `serde_json`, use `cargo add serde_json` to add it to your `Cargo.toml`


error[E0433]: cannot find module or crate `serde_json` in this scope
  --> ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🧪️tests/🔬️unit/🦀️.rs:16:16
   |
16 |     assert_eq!(serde_json::to_value(assembly.definition().identity().as_str()).expect("identity oracle"), fixture["identity"]);
   |                ^^^^^^^^^^ use of unresolved module or unlinked crate `serde_json`
   |
   = help: if you wanted to use a crate named `serde_json`, use `cargo add serde_json` to add it to your `Cargo.toml`


error[E0433]: cannot find module or crate `serde_json` in this scope
  --> ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🧪️tests/🔬️unit/🦀️.rs:13:18
   |
13 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📦️assembly/🔣️.json")).expect("assembly f...
   |                  ^^^^^^^^^^ use of unresolved module or unlinked crate `serde_json`
   |
   = help: if you wanted to use a crate named `serde_json`, use `cargo add serde_json` to add it to your `Cargo.toml`


For more information about this error, try `rustc --explain E0433`.
