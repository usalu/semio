# STEP Current Native Compile Cohort

The current matrix ungated build exposed 69 STEP compiler diagnostics before assertions. Source provenance: `🗑️generated/tools-execution/native-matrix-pdf-dwg-current.log`. Repairs target exact authored standard/subset IO metadata and codecs; fixture data is preserved. Native validation remains pending.

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::base::schema::snapshot::component::native`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:9:76
  |
9 | use crate::standards::v_ap214::subsets::base::schema::snapshot::component::native::{StepSnapshot,StepHeader,StepFileDescription,StepF...
  |                                                                            ^^^^^^ could not find `native` in `component`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🚪️io/🦀️.rs:10:9
   |
10 |     use crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
   |                                                          |
   |                                                          no `check_cc1_conformance` in `standards::v_ap214::subsets::cc1::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🚪️io/🦀️.rs:104:9
    |
104 |     use crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
    |                                                          |
    |                                                          no `check_cc1_conformance` in `standards::v_ap214::subsets::cc1::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc1::schema::MAX_RUNG`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🧬️schema/🧬️mutations/🦀️.rs:36:5
   |
36 | use crate::standards::v_ap214::subsets::cc1::schema::MAX_RUNG;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------
   |                                                      |
   |                                                      no `MAX_RUNG` in `standards::v_ap214::subsets::cc1::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🚪️io/🦀️.rs:10:9
   |
10 |     use crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
   |                                                          |
   |                                                          no `check_cc2_conformance` in `standards::v_ap214::subsets::cc2::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🚪️io/🦀️.rs:104:9
    |
104 |     use crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
    |                                                          |
    |                                                          no `check_cc2_conformance` in `standards::v_ap214::subsets::cc2::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc2::schema::MAX_RUNG`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🧬️schema/🧬️mutations/🦀️.rs:44:5
   |
44 | use crate::standards::v_ap214::subsets::cc2::schema::MAX_RUNG;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------
   |                                                      |
   |                                                      no `MAX_RUNG` in `standards::v_ap214::subsets::cc2::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🚪️io/🦀️.rs:10:9
   |
10 |     use crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
   |                                                          |
   |                                                          no `check_cc3_conformance` in `standards::v_ap214::subsets::cc3::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🚪️io/🦀️.rs:104:9
    |
104 |     use crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
    |                                                          |
    |                                                          no `check_cc3_conformance` in `standards::v_ap214::subsets::cc3::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc3::schema::MAX_RUNG`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🧬️schema/🧬️mutations/🦀️.rs:43:5
   |
43 | use crate::standards::v_ap214::subsets::cc3::schema::MAX_RUNG;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------
   |                                                      |
   |                                                      no `MAX_RUNG` in `standards::v_ap214::subsets::cc3::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🚪️io/🦀️.rs:10:9
   |
10 |     use crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
   |                                                          |
   |                                                          no `check_cc4_conformance` in `standards::v_ap214::subsets::cc4::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🚪️io/🦀️.rs:104:9
    |
104 |     use crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
    |                                                          |
    |                                                          no `check_cc4_conformance` in `standards::v_ap214::subsets::cc4::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc4::schema::MAX_RUNG`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🧬️schema/🧬️mutations/🦀️.rs:42:5
   |
42 | use crate::standards::v_ap214::subsets::cc4::schema::MAX_RUNG;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------
   |                                                      |
   |                                                      no `MAX_RUNG` in `standards::v_ap214::subsets::cc4::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🚪️io/🦀️.rs:10:9
   |
10 |     use crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
   |                                                          |
   |                                                          no `check_cc5_conformance` in `standards::v_ap214::subsets::cc5::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🚪️io/🦀️.rs:104:9
    |
104 |     use crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
    |                                                          |
    |                                                          no `check_cc5_conformance` in `standards::v_ap214::subsets::cc5::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc5::schema::MAX_RUNG`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🧬️schema/🧬️mutations/🦀️.rs:43:5
   |
43 | use crate::standards::v_ap214::subsets::cc5::schema::MAX_RUNG;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------
   |                                                      |
   |                                                      no `MAX_RUNG` in `standards::v_ap214::subsets::cc5::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🚪️io/🦀️.rs:10:9
   |
10 |     use crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
   |                                                          |
   |                                                          no `check_cc6_conformance` in `standards::v_ap214::subsets::cc6::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🚪️io/🦀️.rs:104:9
    |
104 |     use crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------------
    |                                                          |
    |                                                          no `check_cc6_conformance` in `standards::v_ap214::subsets::cc6::schema`
```

```text
error[E0432]: unresolved import `crate::standards::v_ap214::subsets::cc6::schema::MAX_RUNG`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🧬️schema/🧬️mutations/🦀️.rs:50:5
   |
50 | use crate::standards::v_ap214::subsets::cc6::schema::MAX_RUNG;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------
   |                                                      |
   |                                                      no `MAX_RUNG` in `standards::v_ap214::subsets::cc6::schema`
```

```text
error[E0433]: cannot find `sqlite_snapshot` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🚪️io/🦀️.rs:226:44
    |
226 |         let facts=crate::schema::snapshot::sqlite_snapshot::conformance_facts(snapshot,MAX_RUNG,control)?;let mut out=Vec::new();
    |                                            ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `snapshot`
```

```text
error[E0433]: cannot find `sqlite_snapshot` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🚪️io/🦀️.rs:225:44
    |
225 |         let facts=crate::schema::snapshot::sqlite_snapshot::conformance_facts(snapshot,MAX_RUNG,control)?;let mut out=Vec::new();
    |                                            ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `snapshot`
```

```text
error[E0433]: cannot find `sqlite_snapshot` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🚪️io/🦀️.rs:225:44
    |
225 |         let facts=crate::schema::snapshot::sqlite_snapshot::conformance_facts(snapshot,MAX_RUNG,control)?;let mut out=Vec::new();
    |                                            ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `snapshot`
```

```text
error[E0433]: cannot find `sqlite_snapshot` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🚪️io/🦀️.rs:225:44
    |
225 |         let facts=crate::schema::snapshot::sqlite_snapshot::conformance_facts(snapshot,MAX_RUNG,control)?;let mut out=Vec::new();
    |                                            ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `snapshot`
```

```text
error[E0433]: cannot find `sqlite_snapshot` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🚪️io/🦀️.rs:225:44
    |
225 |         let facts=crate::schema::snapshot::sqlite_snapshot::conformance_facts(snapshot,MAX_RUNG,control)?;let mut out=Vec::new();
    |                                            ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `snapshot`
```

```text
error[E0433]: cannot find `sqlite_snapshot` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🚪️io/🦀️.rs:225:44
    |
225 |         let facts=crate::schema::snapshot::sqlite_snapshot::conformance_facts(snapshot,MAX_RUNG,control)?;let mut out=Vec::new();
    |                                            ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `snapshot`
```

```text
error[E0433]: cannot find module or crate `native` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:19:107
   |
19 | ...esult<Vec<u8>, store::PackError> { native::encode_pack(self, options) }
   |                                       ^^^^^^ use of unresolved module or unlinked crate `native`
   |
   = help: if you wanted to use a crate named `native`, use `cargo add native` to add it to your `Cargo.toml`
help: consider importing this module
   |
 8 + use crate::engine::sqlite::snapshot::native;
   |
```

```text
error[E0433]: cannot find module or crate `native` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:20:111
   |
20 | ...> Result<Self, store::PackError> { native::decode_pack(bytes, options) }
   |                                       ^^^^^^ use of unresolved module or unlinked crate `native`
   |
   = help: if you wanted to use a crate named `native`, use `cargo add native` to add it to your `Cargo.toml`
help: consider importing this module
   |
 8 + use crate::engine::sqlite::snapshot::native;
   |
```

```text
error[E0425]: cannot find function `write_str_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:48:5
   |
48 |     write_str_bin(out, &s.schema);
   |     ^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
27 + use crate::engine::binary::diff::write_str_bin;
   |
```

```text
error[E0425]: cannot find function `enc_file_description_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:49:5
   |
49 |     enc_file_description_bin(&s.header.file_description, out);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
27 + use crate::engine::binary::diff::enc_file_description_bin;
   |
```

```text
error[E0425]: cannot find function `enc_file_name_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:50:5
   |
50 |     enc_file_name_bin(&s.header.file_name, out);
   |     ^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
27 + use crate::engine::binary::diff::enc_file_name_bin;
   |
```

```text
error[E0425]: cannot find function `enc_file_schema_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:51:5
   |
51 |     enc_file_schema_bin(&s.header.file_schema, out);
   |     ^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
27 + use crate::engine::binary::diff::enc_file_schema_bin;
   |
```

```text
error[E0425]: cannot find function `enc_entity_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:54:9
   |
54 |         enc_entity_bin(e, out);
   |         ^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
27 + use crate::engine::binary::diff::enc_entity_bin;
   |
```

```text
error[E0425]: cannot find function `read_str_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:60:18
   |
60 |     let schema = read_str_bin(reader)?;
   |                  ^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
27 + use crate::engine::binary::diff::read_str_bin;
   |
```

```text
error[E0425]: cannot find function `dec_file_description_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:61:28
   |
61 |     let file_description = dec_file_description_bin(reader)?;
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
27 + use crate::engine::binary::diff::dec_file_description_bin;
   |
```

```text
error[E0425]: cannot find function `dec_file_name_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:62:21
   |
62 |     let file_name = dec_file_name_bin(reader)?;
   |                     ^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
27 + use crate::engine::binary::diff::dec_file_name_bin;
   |
```

```text
error[E0425]: cannot find function `dec_file_schema_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:63:23
   |
63 |     let file_schema = dec_file_schema_bin(reader)?;
   |                       ^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
27 + use crate::engine::binary::diff::dec_file_schema_bin;
   |
```

```text
error[E0425]: cannot find function `dec_entity_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs:65:39
   |
65 |     let entities = (0..count).map(|_| dec_entity_bin(reader)).collect::<Result<Vec<_>, String>>()?;
   |                                       ^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
27 + use crate::engine::binary::diff::dec_entity_bin;
   |
```

```text
error[E0433]: cannot find module or crate `native` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:26:87
   |
26 |     fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { native::parse_text(text) }
   |                                                                                       ^^^^^^ use of unresolved module or unlinked crate `native`
   |
   = help: if you wanted to use a crate named `native`, use `cargo add native` to add it to your `Cargo.toml`
help: consider importing this module
   |
14 + use crate::engine::sqlite::snapshot::native;
   |
```

```text
error[E0433]: cannot find module or crate `native` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:27:37
   |
27 |     fn print_dsl(&self) -> String { native::print_text(self) }
   |                                     ^^^^^^ use of unresolved module or unlinked crate `native`
   |
   = help: if you wanted to use a crate named `native`, use `cargo add native` to add it to your `Cargo.toml`
help: consider importing this module
   |
14 + use crate::engine::sqlite::snapshot::native;
   |
```

```text
error[E0425]: cannot find function `enc_str` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:54:35
   |
54 |     format!("[{},{},{},{},[{}]]", enc_str(&s.schema), enc_file_description(&s.header.file_description), enc_file_name(&s.header.file...
   |                                   ^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::enc_str;
   |
```

```text
error[E0425]: cannot find function `enc_file_description` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:54:55
   |
54 |     format!("[{},{},{},{},[{}]]", enc_str(&s.schema), enc_file_description(&s.header.file_description), enc_file_name(&s.header.file...
   |                                                       ^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::enc_file_description;
   |
```

```text
error[E0425]: cannot find function `enc_file_name` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:54:105
   |
54 | ...file_description(&s.header.file_description), enc_file_name(&s.header.file_name), enc_file_schema(&s.header.file_schema), s.entit...
   |                                                  ^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::enc_file_name;
   |
```

```text
error[E0425]: cannot find function `enc_file_schema` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:54:141
   |
54 | ...iption), enc_file_name(&s.header.file_name), enc_file_schema(&s.header.file_schema), s.entities.iter().map(enc_entity).collect::<...
   |                                                 ^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::enc_file_schema;
   |
```

```text
error[E0425]: cannot find value `enc_entity` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:54:203
   |
54 | ...(&s.header.file_schema), s.entities.iter().map(enc_entity).collect::<Vec<_>>().join(","),)
   |                                                   ^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::enc_entity;
   |
```

```text
error[E0425]: cannot find function `split_top_level` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:59:17
   |
59 |     let parts = split_top_level(strip_brackets(s)?, ',');
   |                 ^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::split_top_level;
   |
```

```text
error[E0425]: cannot find function `strip_brackets` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:59:33
   |
59 |     let parts = split_top_level(strip_brackets(s)?, ',');
   |                                 ^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::strip_brackets;
   |
```

```text
error[E0425]: cannot find function `dec_str` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:64:17
   |
64 |         schema: dec_str(schema)?,
   |                 ^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::dec_str;
   |
```

```text
error[E0425]: cannot find function `dec_file_description` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:65:73
   |
65 | ...   header: crate::schema::snapshot::StepHeader { file_description: dec_file_description(file_description)?, file_name: dec_file_n...
   |                                                                       ^^^^^^^^^^^^^^^^^^^^
   |
help: a local variable with a similar name exists
   |
65 -         header: crate::schema::snapshot::StepHeader { file_description: dec_file_description(file_description)?, file_name: dec_file_name(file_name)?, file_schema: dec_file_schema(file_schema)? },
65 +         header: crate::schema::snapshot::StepHeader { file_description: file_description(file_description)?, file_name: dec_file_name(file_name)?, file_schema: dec_file_schema(file_schema)? },
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::dec_file_description;
   |
```

```text
error[E0425]: cannot find function `dec_file_name` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:65:125
   |
65 | ...tion: dec_file_description(file_description)?, file_name: dec_file_name(file_name)?, file_schema: dec_file_schema(file_schema)? },
   |                                                              ^^^^^^^^^^^^^
   |
help: a local variable with a similar name exists
   |
65 -         header: crate::schema::snapshot::StepHeader { file_description: dec_file_description(file_description)?, file_name: dec_file_name(file_name)?, file_schema: dec_file_schema(file_schema)? },
65 +         header: crate::schema::snapshot::StepHeader { file_description: dec_file_description(file_description)?, file_name: file_name(file_name)?, file_schema: dec_file_schema(file_schema)? },
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::dec_file_name;
   |
```

```text
error[E0425]: cannot find function `dec_file_schema` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:65:165
   |
65 | ...on)?, file_name: dec_file_name(file_name)?, file_schema: dec_file_schema(file_schema)? },
   |                                                             ^^^^^^^^^^^^^^^
   |
help: a local variable with a similar name exists
   |
65 -         header: crate::schema::snapshot::StepHeader { file_description: dec_file_description(file_description)?, file_name: dec_file_name(file_name)?, file_schema: dec_file_schema(file_schema)? },
65 +         header: crate::schema::snapshot::StepHeader { file_description: dec_file_description(file_description)?, file_name: dec_file_name(file_name)?, file_schema: file_schema(file_schema)? },
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::dec_file_schema;
   |
```

```text
error[E0425]: cannot find function `split_top_level` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:66:19
   |
66 | ...   entities: split_top_level(strip_brackets(entities)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_entity).collect::<Resu...
   |                 ^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::split_top_level;
   |
```

```text
error[E0425]: cannot find function `strip_brackets` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:66:35
   |
66 | ...   entities: split_top_level(strip_brackets(entities)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_entity).collect::<Resu...
   |                                 ^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::strip_brackets;
   |
```

```text
error[E0425]: cannot find value `dec_entity` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:66:109
   |
66 | ...',').into_iter().filter(|s| !s.is_empty()).map(dec_entity).collect::<Result<Vec<_>, String>>()?,
   |                                                   ^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
34 + use crate::engine::text::diff::dec_entity;
   |
```

```text
error[E0425]: cannot find function `check_cc1_conformance_controlled` in module `crate::standards::v_ap214::subsets::cc1::schema`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:73:585
   |
73 | ...ts::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc...
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::standards::v_ap214::subsets::cc1::schema`
   |
help: consider importing this function through its public re-export
   |
 2 + use crate::standards::v_ap214::subsets::cc1::io::check_cc1_conformance_controlled;
   |
help: if you import `check_cc1_conformance_controlled`, refer to it directly
   |
73 -         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
73 +         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
   |
```

```text
error[E0425]: cannot find function `check_cc2_conformance_controlled` in module `crate::standards::v_ap214::subsets::cc2::schema`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:73:689
   |
73 | ...ts::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc...
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::standards::v_ap214::subsets::cc2::schema`
   |
help: consider importing this function through its public re-export
   |
 2 + use crate::standards::v_ap214::subsets::cc2::io::check_cc2_conformance_controlled;
   |
help: if you import `check_cc2_conformance_controlled`, refer to it directly
   |
73 -         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
73 +         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
   |
```

```text
error[E0425]: cannot find function `check_cc3_conformance_controlled` in module `crate::standards::v_ap214::subsets::cc3::schema`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:73:793
   |
73 | ...ts::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc...
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::standards::v_ap214::subsets::cc3::schema`
   |
help: consider importing this function through its public re-export
   |
 2 + use crate::standards::v_ap214::subsets::cc3::io::check_cc3_conformance_controlled;
   |
help: if you import `check_cc3_conformance_controlled`, refer to it directly
   |
73 -         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
73 +         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
   |
```

```text
error[E0425]: cannot find function `check_cc4_conformance_controlled` in module `crate::standards::v_ap214::subsets::cc4::schema`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:73:897
   |
73 | ...ts::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc...
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::standards::v_ap214::subsets::cc4::schema`
   |
help: consider importing this function through its public re-export
   |
 2 + use crate::standards::v_ap214::subsets::cc4::io::check_cc4_conformance_controlled;
   |
help: if you import `check_cc4_conformance_controlled`, refer to it directly
   |
73 -         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
73 +         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
   |
```

```text
error[E0425]: cannot find function `check_cc5_conformance_controlled` in module `crate::standards::v_ap214::subsets::cc5::schema`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:73:1001
   |
73 | ...ts::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc...
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::standards::v_ap214::subsets::cc5::schema`
   |
help: consider importing this function through its public re-export
   |
 2 + use crate::standards::v_ap214::subsets::cc5::io::check_cc5_conformance_controlled;
   |
help: if you import `check_cc5_conformance_controlled`, refer to it directly
   |
73 -         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
73 +         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
   |
```

```text
error[E0425]: cannot find function `check_cc6_conformance_controlled` in module `crate::standards::v_ap214::subsets::cc6::schema`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:73:1105
   |
73 | ...ts::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "u...
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::standards::v_ap214::subsets::cc6::schema`
   |
help: consider importing this function through its public re-export
   |
 2 + use crate::standards::v_ap214::subsets::cc6::io::check_cc6_conformance_controlled;
   |
help: if you import `check_cc6_conformance_controlled`, refer to it directly
   |
73 -         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
73 +         control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::schema::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::schema::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::schema::check_cc5_conformance_controlled(self,control)?,"cc6"=>check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
   |
```

```text
error[E0603]: module `component` is private
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:9:65
    |
  9 | use crate::standards::v_ap214::subsets::base::schema::snapshot::component::native::{StepSnapshot,StepHeader,StepFileDescription,Ste...
    |                                                                 ^^^^^^^^^ private module
    |
note: the module `component` is defined here
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../🦀️.rs:264:25
    |
264 |                         mod component;
    |                         ^^^^^^^^^^^^^^
```

```text
error[E0277]: the trait bound `StepValue: RetireOwned` is not satisfied
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:75:126
   |
75 | ...rol.allocate_vec(count)?,native::close::<Vec<Option<StepValue>>>);values.as_mut().resize_with(count,||None);
   |                                             ^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
   |
help: the trait `semio_framework_value::retirement::RetireOwned` is not implemented for `standards::v_ap214::subsets::base::schema::snapshot::component::StepValue`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:26:1
   |
26 | pub enum StepValue {
   | ^^^^^^^^^^^^^^^^^^
   = help: the following other types implement trait `semio_framework_value::retirement::RetireOwned`:
             ()
             (A, B, C, D)
             (T, U)
             (T, U, V)
             ArtifactChild<S>
             ArtifactDialect
             ArtifactDownloadOutput
             ArtifactLink
           and 130 others
   = note: required for `Option<StepValue>` to implement `semio_framework_value::retirement::RetireOwned`
   = note: 1 redundant requirement hidden
   = note: required for `Vec<Option<StepValue>>` to implement `semio_framework_value::retirement::RetireOwned`
note: required by a bound in `native::close`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:37:23
   |
37 | pub(crate) fn close<T:RetireOwned>(value:T){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !cursor....
   |                       ^^^^^^^^^^^ required by this bound in `close`
   = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-stdio-step/25af3cf65409d56b/out/semio_s_artifact_stdio_step-25af3cf65409d56b.long-type-14543024209434252443.txt'
   = note: consider using `--verbose` to print the full type name to the console
```

```text
error[E0277]: the trait bound `StepValue: RetireOwned` is not satisfied
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:104:197
    |
104 | ...l.allocate_vec(range.len())?,native::close::<Vec<StepValue>>);for edge in range{let row=catalog.tables[10].row(catalog.edge(10,e...
    |                                                 ^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `semio_framework_value::retirement::RetireOwned` is not implemented for `standards::v_ap214::subsets::base::schema::snapshot::component::StepValue`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:26:1
    |
 26 | pub enum StepValue {
    | ^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `semio_framework_value::retirement::RetireOwned`:
              ()
              (A, B, C, D)
              (T, U)
              (T, U, V)
              ArtifactChild<S>
              ArtifactDialect
              ArtifactDownloadOutput
              ArtifactLink
            and 130 others
    = note: required for `Vec<standards::v_ap214::subsets::base::schema::snapshot::component::StepValue>` to implement `semio_framework_value::retirement::RetireOwned`
note: required by a bound in `native::close`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:37:23
    |
 37 | pub(crate) fn close<T:RetireOwned>(value:T){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !cursor...
    |                       ^^^^^^^^^^^ required by this bound in `close`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-stdio-step/25af3cf65409d56b/out/semio_s_artifact_stdio_step-25af3cf65409d56b.long-type-14543024209434252443.txt'
    = note: consider using `--verbose` to print the full type name to the console
```

```text
error[E0277]: the trait bound `StepValue: RetireOwned` is not satisfied
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:105:373
    |
105 | ...new(control.allocate_vec(1)?,native::close::<Vec<StepValue>>);slot.as_mut().push(self.pop(catalog,row.integer(3)?)?);StepValue::...
    |                                                 ^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `semio_framework_value::retirement::RetireOwned` is not implemented for `standards::v_ap214::subsets::base::schema::snapshot::component::StepValue`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:26:1
    |
 26 | pub enum StepValue {
    | ^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `semio_framework_value::retirement::RetireOwned`:
              ()
              (A, B, C, D)
              (T, U)
              (T, U, V)
              ArtifactChild<S>
              ArtifactDialect
              ArtifactDownloadOutput
              ArtifactLink
            and 130 others
    = note: required for `Vec<standards::v_ap214::subsets::base::schema::snapshot::component::StepValue>` to implement `semio_framework_value::retirement::RetireOwned`
note: required by a bound in `native::close`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:37:23
    |
 37 | pub(crate) fn close<T:RetireOwned>(value:T){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !cursor...
    |                       ^^^^^^^^^^^ required by this bound in `close`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-stdio-step/25af3cf65409d56b/out/semio_s_artifact_stdio_step-25af3cf65409d56b.long-type-14543024209434252443.txt'
    = note: consider using `--verbose` to print the full type name to the console
```

```text
error[E0277]: the trait bound `StepValue: RetireOwned` is not satisfied
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:113:174
    |
113 | ...l.allocate_vec(range.len())?,native::close::<Vec<StepValue>>);
    |                                                 ^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `semio_framework_value::retirement::RetireOwned` is not implemented for `standards::v_ap214::subsets::base::schema::snapshot::component::StepValue`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:26:1
    |
 26 | pub enum StepValue {
    | ^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `semio_framework_value::retirement::RetireOwned`:
              ()
              (A, B, C, D)
              (T, U)
              (T, U, V)
              ArtifactChild<S>
              ArtifactDialect
              ArtifactDownloadOutput
              ArtifactLink
            and 130 others
    = note: required for `Vec<standards::v_ap214::subsets::base::schema::snapshot::component::StepValue>` to implement `semio_framework_value::retirement::RetireOwned`
note: required by a bound in `native::close`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:37:23
    |
 37 | pub(crate) fn close<T:RetireOwned>(value:T){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !cursor...
    |                       ^^^^^^^^^^^ required by this bound in `close`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-stdio-step/25af3cf65409d56b/out/semio_s_artifact_stdio_step-25af3cf65409d56b.long-type-14543024209434252443.txt'
    = note: consider using `--verbose` to print the full type name to the console
```

```text
error[E0277]: the trait bound `StepSnapshot: RetireOwned` is not satisfied
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:122:515
    |
122 | ...:new()}},entities:Vec::new()},native::close::<StepSnapshot>);
    |                                                  ^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `semio_framework_value::retirement::RetireOwned` is not implemented for `standards::v_ap214::subsets::base::schema::snapshot::component::StepSnapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:174:1
    |
174 | pub struct StepSnapshot {
    | ^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `semio_framework_value::retirement::RetireOwned`:
              ()
              (A, B, C, D)
              (T, U)
              (T, U, V)
              ArtifactChild<S>
              ArtifactDialect
              ArtifactDownloadOutput
              ArtifactLink
            and 130 others
note: required by a bound in `native::close`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:37:23
    |
 37 | pub(crate) fn close<T:RetireOwned>(value:T){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !cursor...
    |                       ^^^^^^^^^^^ required by this bound in `close`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-stdio-step/25af3cf65409d56b/out/semio_s_artifact_stdio_step-25af3cf65409d56b.long-type-15700448738791398076.txt'
    = note: consider using `--verbose` to print the full type name to the console
```

```text
error[E0277]: the trait bound `StepEntity: RetireOwned` is not satisfied
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:138:190
    |
138 | ...Vec::new(),complex:Vec::new()},native::close::<StepEntity>);
    |                                                   ^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `semio_framework_value::retirement::RetireOwned` is not implemented for `standards::v_ap214::subsets::base::schema::snapshot::component::StepEntity`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:155:1
    |
155 | pub struct StepEntity {
    | ^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `semio_framework_value::retirement::RetireOwned`:
              ()
              (A, B, C, D)
              (T, U)
              (T, U, V)
              ArtifactChild<S>
              ArtifactDialect
              ArtifactDownloadOutput
              ArtifactLink
            and 130 others
note: required by a bound in `native::close`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:37:23
    |
 37 | pub(crate) fn close<T:RetireOwned>(value:T){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !cursor...
    |                       ^^^^^^^^^^^ required by this bound in `close`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-stdio-step/25af3cf65409d56b/out/semio_s_artifact_stdio_step-25af3cf65409d56b.long-type-15785316902742962180.txt'
    = note: consider using `--verbose` to print the full type name to the console
```

```text
error[E0277]: the trait bound `StepComplexType: RetireOwned` is not satisfied
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:142:222
    |
142 | ...text(3)?)?,args:Vec::new()},native::close::<StepComplexType>);
    |                                                ^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `semio_framework_value::retirement::RetireOwned` is not implemented for `standards::v_ap214::subsets::base::schema::snapshot::component::StepComplexType`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:146:1
    |
146 | pub struct StepComplexType {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `semio_framework_value::retirement::RetireOwned`:
              ()
              (A, B, C, D)
              (T, U)
              (T, U, V)
              ArtifactChild<S>
              ArtifactDialect
              ArtifactDownloadOutput
              ArtifactLink
            and 130 others
note: required by a bound in `native::close`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:37:23
    |
 37 | pub(crate) fn close<T:RetireOwned>(value:T){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !cursor...
    |                       ^^^^^^^^^^^ required by this bound in `close`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-stdio-step/25af3cf65409d56b/out/semio_s_artifact_stdio_step-25af3cf65409d56b.long-type-18346443435700314229.txt'
    = note: consider using `--verbose` to print the full type name to the console
```

```text
error[E0277]: the trait bound `StepSnapshot: RetireOwned` is not satisfied
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:67:115
    |
 67 |     fn retire_sqlite_snapshot(self){crate::standards::v_ap214::subsets::base::io::sqlite::snapshot::native::close(self)}
    |                                     ----------------------------------------------------------------------------- ^^^^ unsatisfied trait bound
    |                                     |
    |                                     required by a bound introduced by this call
    |
help: the trait `semio_framework_value::retirement::RetireOwned` is not implemented for `standards::v_ap214::subsets::base::schema::snapshot::component::StepSnapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:174:1
    |
174 | pub struct StepSnapshot {
    | ^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `semio_framework_value::retirement::RetireOwned`:
              ()
              (A, B, C, D)
              (T, U)
              (T, U, V)
              ArtifactChild<S>
              ArtifactDialect
              ArtifactDownloadOutput
              ArtifactLink
            and 130 others
note: required by a bound in `native::close`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:37:23
    |
 37 | pub(crate) fn close<T:RetireOwned>(value:T){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !cursor...
    |                       ^^^^^^^^^^^ required by this bound in `close`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-stdio-step/25af3cf65409d56b/out/semio_s_artifact_stdio_step-25af3cf65409d56b.long-type-15700448738791398076.txt'
    = note: consider using `--verbose` to print the full type name to the console
```

```text
error[E0220]: associated type `TypedValue` not found for `Self`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:34:291
   |
34 | ...gate(values)=>values.retirement(),Self::TypedValue{type_name,value}=>semio_framework_value::artifact_retirement_sequence![type_na...
   |                                            ^^^^^^^^^^ associated type `TypedValue` not found
```


## Canonical Source Repairs

The existing native module retirement implementations could not resolve the private removed snapshot component/native path; canonical public snapshot imports restore those exact implementations without adding or duplicating retirement behavior. Text and binary snapshot facets import their existing sibling IO diff helpers and the declared SQLite native owner. CC1–CC6 conformance checks, controlled facts and MAX_RUNG references consume their actual IO declarations. Codec algorithms and fixture data remain unchanged. Fresh native proof is pending.

- `🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🚪️io/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🧬️schema/🧬️mutations/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🚪️io/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🧬️schema/🧬️mutations/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🚪️io/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🧬️schema/🧬️mutations/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🚪️io/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🧬️schema/🧬️mutations/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🚪️io/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🧬️schema/🧬️mutations/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🚪️io/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🧬️schema/🧬️mutations/🦀️.rs`
- `🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`

## Independent Fixture Witness

The existing owned STEP `test-snapshot-sqlite source` command executed through workspace Nx: 9 tests passed, 0 failed, 102 assertions, 401ms test phase. Independent SQLite and Ajv validate the neutral relational/header/entity/value and exact scalar/backing contracts. Exact log: `🗑️generated/tools-execution/step-snapshot-source-current.log`. This is source/neutral fixture proof; native history-edit assertions remain pending.
