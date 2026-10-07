# PDF and DWG Current Native Compile Cohort

The current Draw choices native target failed before assertions, with 453 Rust diagnostics: PDF 412, DXF 38, DWG 3. These are compiler witnesses, not editor acceptance failures. Root owns the DXF 38 diagnostics; this agent owns PDF and DWG. The loaded source provenance is `🗑️generated/tools-execution/draw-choices-immutable-genesis.log`; no assertions selected or passed.

## PDF Canonical Owner Repairs

Hand-repaired 142 declared mutation payload imports to their standard/subset schema aggregate, 65 direct IO mutation tag registry references to their leaf owner, 62 base tag paths to canonical binary mutation IO, 31 conformance calls to their own IO analysis reexport, and the snapshot text projection owner used by schema/SQLite. Sibling projection helpers now have crate visibility. Removed ten ambiguous flattened IO facade reexports; canonical standard/subset modules and the public PdfSnapshot facade remain. Existing native_pack/inverseRows payload behavior and authored fixtures are preserved. These changes still require a fresh native compile.

## DWG Borrowed Metadata

The compiler rejected DwgConstraintNode and DwgLogicalObjectBody in nested borrowed snapshot metadata. Their already-authored dwg_metadata declarations now emit the borrowed record role using the existing optional owner parameter. The new logical-object borrowed role also requires its authored table-control/table-record/entity metadata carriers and their complex-color child. Their existing single metadata declarations now publish the same role. The neutral metadata native law covers all eleven corpus cases, with serde_json as independent fixture decoder. Current Nx-owned source oracle passed 2/2 tests, 194 assertions, using the closed neutral eleven-record fixture and independent Ajv decoder. Fresh native validation is pending.

## Full Matrix Current Source Provenance

The all-editor invocation finished with zero executed editor assertions. Its first 39 ungated packages stopped on the loaded older create_dag_store actor omission, already repaired by root in current source. The 57 assembly packages stopped on a framework FlowStore birth lacking actor and an MP4 test importing analysis from schema. The MP4 test now imports the canonical analysis owner. Root owns the FlowStore route. Both groups require a fresh rerun.

## DXF Exact Compiler Excerpts for Root

```text
error[E0432]: unresolved import `super::text`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/🦀️.rs:27:5
   |
27 | use super::text as snapshot_text;
   |     ^^^^^^^----^^^^^^^^^^^^^^^^^
   |            |
   |            no `text` in `standards::v_r12::subsets::any::schema::snapshot`
   |
help: consider importing one of these modules instead
   |
27 - use super::text as snapshot_text;
27 + use crate::dsl::os_spr::io::text as snapshot_text;
   |
27 - use super::text as snapshot_text;
27 + use crate::engine::text as snapshot_text;
   |
27 - use super::text as snapshot_text;
27 + use semio_framework_2d::text as snapshot_text;
   |
27 - use super::text as snapshot_text;
27 + use semio_framework_os_kernel::os_spr::io::text as snapshot_text;
   |
   = and 1 other candidate
```

```text
error[E0432]: unresolved import `crate::schema::diff::dec_block_bin`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🦀️.rs:22:5
   |
22 |     dec_block_bin, diff_insert_block, diff_insert_entity, diff_insert_layer, diff_insert_linetype, diff_insert_style, diff_remove_bl...
   |     ^^^^^^^^^^^^^ no `dec_block_bin` in `standards::v_r12::subsets::any::schema::diff`
```

```text
error[E0432]: unresolved import `crate::schema::diff::dec_block_bin`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🧬️mutations/🦀️.rs:12:5
   |
12 |     dec_block_bin, diff_insert_block, diff_insert_entity, diff_insert_layer, diff_insert_linetype, diff_insert_style, diff_remove_bl...
   |     ^^^^^^^^^^^^^ no `dec_block_bin` in `standards::v_r12::subsets::any::schema::diff`
   |
   = note: unresolved item `crate::engine::binary::mutations::mutations_codec::set_style::dec_block_bin` exists but is inaccessible
```

```text
error[E0432]: unresolved import `crate::standards::v_r12::subsets::any::schema::snapshot::text`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:113:5
    |
113 | use crate::standards::v_r12::subsets::any::schema::snapshot::text as snapshot_text;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----^^^^^^^^^^^^^^^^^
    |                                                              |
    |                                                              no `text` in `standards::v_r12::subsets::any::schema::snapshot`
    |
help: consider importing one of these modules instead
    |
113 - use crate::standards::v_r12::subsets::any::schema::snapshot::text as snapshot_text;
113 + use crate::dsl::os_spr::io::text as snapshot_text;
    |
113 - use crate::standards::v_r12::subsets::any::schema::snapshot::text as snapshot_text;
113 + use crate::engine::text as snapshot_text;
    |
113 - use crate::standards::v_r12::subsets::any::schema::snapshot::text as snapshot_text;
113 + use semio_framework_2d::text as snapshot_text;
    |
113 - use crate::standards::v_r12::subsets::any::schema::snapshot::text as snapshot_text;
113 + use semio_framework_os_kernel::os_spr::io::text as snapshot_text;
    |
    = and 1 other candidate
```

```text
error[E0432]: unresolved import `crate::schema::diff::dec_block_bin`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:5
   |
13 |     dec_block_bin, diff_insert_block, diff_insert_entity, diff_insert_layer, diff_insert_linetype, diff_insert_style, diff_remove_bl...
   |     ^^^^^^^^^^^^^ no `dec_block_bin` in `standards::v_r12::subsets::any::schema::diff`
   |
   = note: unresolved item `crate::engine::text::mutations::mutations_codec::set_style::dec_block_bin` exists but is inaccessible
```

```text
error[E0425]: cannot find function `write_str_lp` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:54:5
   |
54 |     write_str_lp(out, &s.schema);
   |     ^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
41 + use crate::engine::binary::diff::write_str_lp;
   |
```

```text
error[E0425]: cannot find function `enc_header_var_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:57:9
   |
57 |         enc_header_var_bin(hv, out);
   |         ^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
41 + use crate::engine::binary::diff::enc_header_var_bin;
   |
```

```text
error[E0425]: cannot find function `enc_block_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:66:9
   |
66 |         enc_block_bin(b, out);
   |         ^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
41 + use crate::engine::binary::diff::enc_block_bin;
   |
```

```text
error[E0425]: cannot find function `enc_dxf_entities_bin` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:68:5
    |
 68 |     enc_dxf_entities_bin(&s.entities, out);
    |     ^^^^^^^^^^^^^^^^^^^^
    |
   ::: 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🦀️.rs:947:1
    |
947 | pub(crate) fn enc_dxf_tables_bin(t: &DxfTables, out: &mut Vec<u8>) {
    | ------------------------------------------------------------------ similarly named function `enc_dxf_tables_bin` defined here
    |
help: a function with a similar name exists
    |
 68 -     enc_dxf_entities_bin(&s.entities, out);
 68 +     enc_dxf_tables_bin(&s.entities, out);
    |
help: consider importing this function through its public re-export
    |
 41 + use crate::engine::binary::diff::enc_dxf_entities_bin;
    |
```

```text
error[E0425]: cannot find function `read_str_lp` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:73:18
   |
73 |     let schema = read_str_lp(reader)?;
   |                  ^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
41 + use crate::engine::binary::diff::read_str_lp;
   |
```

```text
error[E0425]: cannot find function `dec_header_var_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:77:26
   |
77 |         header_vars.push(dec_header_var_bin(reader)?);
   |                          ^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
41 + use crate::engine::binary::diff::dec_header_var_bin;
   |
```

```text
error[E0425]: cannot find function `dec_block_bin` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:88:21
   |
88 |         blocks.push(dec_block_bin(reader)?);
   |                     ^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
41 + use crate::engine::binary::diff::dec_block_bin;
   |
```

```text
error[E0425]: cannot find function `dec_dxf_entities_bin` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:90:20
    |
 90 |     let entities = dec_dxf_entities_bin(reader)?;
    |                    ^^^^^^^^^^^^^^^^^^^^
    |
   ::: 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🦀️.rs:963:1
    |
963 | pub(crate) fn dec_dxf_tables_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfTables, String> {
    | ------------------------------------------------------------------------------------------------- similarly named function `dec_dxf_tables_bin` defined here
    |
help: a function with a similar name exists
    |
 90 -     let entities = dec_dxf_entities_bin(reader)?;
 90 +     let entities = dec_dxf_tables_bin(reader)?;
    |
help: consider importing this function through its public re-export
    |
 41 + use crate::engine::binary::diff::dec_dxf_entities_bin;
    |
```

```text
error[E0425]: cannot find function `enc_str` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:908:36
    |
908 |     format!("[{},{},{},{},{},{}]", enc_str(&s.schema), enc_list(&s.header_vars, enc_header_var), enc_dxf_tables(&s.tables), enc_lis...
    |                                    ^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::enc_str;
    |
```

```text
error[E0425]: cannot find function `enc_list` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:908:56
    |
908 |     format!("[{},{},{},{},{},{}]", enc_str(&s.schema), enc_list(&s.header_vars, enc_header_var), enc_dxf_tables(&s.tables), enc_lis...
    |                                                        ^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::enc_list;
    |
```

```text
error[E0425]: cannot find value `enc_header_var` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:908:81
    |
908 |     format!("[{},{},{},{},{},{}]", enc_str(&s.schema), enc_list(&s.header_vars, enc_header_var), enc_dxf_tables(&s.tables), enc_lis...
    |                                                                                 ^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::enc_header_var;
    |
```

```text
error[E0425]: cannot find function `enc_list` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:908:125
    |
908 | ...rs, enc_header_var), enc_dxf_tables(&s.tables), enc_list(&s.other_tables, enc_other_table), enc_list(&s.blocks, enc_block), enc_...
    |                                                    ^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::enc_list;
    |
```

```text
error[E0425]: cannot find function `enc_list` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:908:169
    |
908 | ...s), enc_list(&s.other_tables, enc_other_table), enc_list(&s.blocks, enc_block), enc_dxf_entities(&s.entities),)
    |                                                    ^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::enc_list;
    |
```

```text
error[E0425]: cannot find value `enc_block` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:908:189
    |
908 | ..._tables, enc_other_table), enc_list(&s.blocks, enc_block), enc_dxf_entities(&s.entities),)
    |                                                   ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::enc_block;
    |
```

```text
error[E0425]: cannot find function `enc_dxf_entities` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:908:201
    |
908 | ...bles, enc_other_table), enc_list(&s.blocks, enc_block), enc_dxf_entities(&s.entities),)
    |                                                            ^^^^^^^^^^^^^^^^
    |
   ::: 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/🔺️diff/🦀️.rs:805:1
    |
805 | pub(crate) fn enc_dxf_tables(t: &DxfTables) -> String {
    | ----------------------------------------------------- similarly named function `enc_dxf_tables` defined here
    |
help: a function with a similar name exists
    |
908 -     format!("[{},{},{},{},{},{}]", enc_str(&s.schema), enc_list(&s.header_vars, enc_header_var), enc_dxf_tables(&s.tables), enc_list(&s.other_tables, enc_other_table), enc_list(&s.blocks, enc_block), enc_dxf_entities(&s.entities),)
908 +     format!("[{},{},{},{},{},{}]", enc_str(&s.schema), enc_list(&s.header_vars, enc_header_var), enc_dxf_tables(&s.tables), enc_list(&s.other_tables, enc_other_table), enc_list(&s.blocks, enc_block), enc_dxf_tables(&s.entities),)
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::enc_dxf_entities;
    |
```

```text
error[E0425]: cannot find function `split_top_level` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:913:17
    |
913 |     let parts = split_top_level(strip_brackets(s)?, ',');
    |                 ^^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::split_top_level;
    |
```

```text
error[E0425]: cannot find function `strip_brackets` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:913:33
    |
913 |     let parts = split_top_level(strip_brackets(s)?, ',');
    |                                 ^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::strip_brackets;
    |
```

```text
error[E0425]: cannot find function `dec_str` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:918:17
    |
918 |         schema: dec_str(schema)?,
    |                 ^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::dec_str;
    |
```

```text
error[E0425]: cannot find function `dec_list` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:919:22
    |
919 |         header_vars: dec_list(header_vars, dec_header_var)?,
    |                      ^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::dec_list;
    |
```

```text
error[E0425]: cannot find value `dec_header_var` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:919:44
    |
919 |         header_vars: dec_list(header_vars, dec_header_var)?,
    |                                            ^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::dec_header_var;
    |
```

```text
error[E0425]: cannot find function `dec_list` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:921:23
    |
921 |         other_tables: dec_list(other_tables, dec_other_table)?,
    |                       ^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::dec_list;
    |
```

```text
error[E0425]: cannot find function `dec_list` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:922:17
    |
922 |         blocks: dec_list(blocks, dec_block)?,
    |                 ^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::dec_list;
    |
```

```text
error[E0425]: cannot find value `dec_block` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:922:34
    |
922 |         blocks: dec_list(blocks, dec_block)?,
    |                                  ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::dec_block;
    |
```

```text
error[E0425]: cannot find function `dec_dxf_entities` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:923:19
    |
923 |         entities: dec_dxf_entities(entities)?,
    |                   ^^^^^^^^^^^^^^^^
    |
   ::: 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/🔺️diff/🦀️.rs:810:1
    |
810 | pub(crate) fn dec_dxf_tables(s: &str) -> Result<DxfTables, String> {
    | ------------------------------------------------------------------ similarly named function `dec_dxf_tables` defined here
    |
help: a function with a similar name exists
    |
923 -         entities: dec_dxf_entities(entities)?,
923 +         entities: dec_dxf_tables(entities)?,
    |
help: consider importing this function through its public re-export
    |
893 + use crate::engine::text::diff::dec_dxf_entities;
    |
```

```text
error[E0603]: function `spec` is private
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:21:67
   |
21 |         let raw = store::pack_rt::encode_document(&snapshot_text::spec(), &snapshot_text::to_record(self), options)?;
   |                                                                   ^^^^ private function
   |
note: the function `spec` is defined here
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:15:1
   |
15 | pub(super) fn spec()->semio_framework_dsl_record::RecordSpec{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```text
error[E0603]: function `to_record` is private
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:21:91
   |
21 |         let raw = store::pack_rt::encode_document(&snapshot_text::spec(), &snapshot_text::to_record(self), options)?;
   |                                                                                           ^^^^^^^^^ private function
   |
note: the function `to_record` is defined here
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:28:1
   |
28 | pub(super) fn to_record(snapshot:&DxfSnapshot)->semio_framework_dsl_record::RecordValue{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```text
error[E0603]: function `spec` is private
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:30:88
   |
30 |         let (record, report) = store::pack_rt::decode_document(&inner, &snapshot_text::spec(), options)?;
   |                                                                                        ^^^^ private function
   |
note: the function `spec` is defined here
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:15:1
   |
15 | pub(super) fn spec()->semio_framework_dsl_record::RecordSpec{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```text
error[E0603]: function `from_record` is private
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:32:24
   |
32 |         snapshot_text::from_record(&record).map_err(store::text_error_to_pack_error)
   |                        ^^^^^^^^^^^ private function
   |
note: the function `from_record` is defined here
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:46:1
   |
46 | pub(super) fn from_record(record:&semio_framework_dsl_record::RecordValue)->Result<DxfSnapshot,semio_framework_diagnostic::TextError>{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```text
error[E0603]: function `spec` is private
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/📸️snapshot/🦀️.rs:34:94
   |
34 |     fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { Some(snapshot_text::spec()) }
   |                                                                                              ^^^^ private function
   |
note: the function `spec` is defined here
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🦀️.rs:15:1
   |
15 | pub(super) fn spec()->semio_framework_dsl_record::RecordSpec{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```text
error[E0433]: cannot find module or crate `snapshot_text` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:117:118
    |
117 | ...ArtifactDsl>::envelope_id(),snapshot_text::spec_producer(),|native|snapshot_text::to_record_controlled(self,native),control)
    |                                ^^^^^^^^^^^^^ use of unresolved module or unlinked crate `snapshot_text`
    |
    = help: if you wanted to use a crate named `snapshot_text`, use `cargo add snapshot_text` to add it to your `Cargo.toml`
```

```text
error[E0433]: cannot find module or crate `snapshot_text` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:117:157
    |
117 | ...xt::spec_producer(),|native|snapshot_text::to_record_controlled(self,native),control)
    |                                ^^^^^^^^^^^^^ use of unresolved module or unlinked crate `snapshot_text`
    |
    = help: if you wanted to use a crate named `snapshot_text`, use `cargo add snapshot_text` to add it to your `Cargo.toml`
```

```text
error[E0433]: cannot find module or crate `snapshot_text` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:120:120
    |
120 | ...ArtifactDsl>::envelope_id(),snapshot_text::spec_producer(),snapshot_text::from_record_controlled,control)?;let mut out=Projectio...
    |                                ^^^^^^^^^^^^^ use of unresolved module or unlinked crate `snapshot_text`
    |
    = help: if you wanted to use a crate named `snapshot_text`, use `cargo add snapshot_text` to add it to your `Cargo.toml`
```

```text
error[E0433]: cannot find module or crate `snapshot_text` in this scope
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:120:151
    |
120 | ...pshot_text::spec_producer(),snapshot_text::from_record_controlled,control)?;let mut out=Projection::forecast(control,Phase::Deco...
    |                                ^^^^^^^^^^^^^ use of unresolved module or unlinked crate `snapshot_text`
    |
    = help: if you wanted to use a crate named `snapshot_text`, use `cargo add snapshot_text` to add it to your `Cargo.toml`
```


## Later Native Source Capture

DWG controlled metadata compilation reached its own test crate without a borrowed-role diagnostic, then stopped on eight test namespace errors before selecting any assertions. Five SQLite and one DwgDrawing references were already correct in current peer source and preserved; this agent corrected the remaining two binary decoder references. The new native retry writes `dwg-controlled-metadata-namespace-current.log`.

The broader matrix subsequently diagnosed 92 additional PDF owner references and one import-before-inner-doc error introduced by this repair cohort; all now have explicit canonical source owners and the module doc remains first. See 📓️2026-10-06-native-matrix-current-cohort.md. Fresh native payload and editor validation remains required.

## Payload Test Namespace Capture

The current focused payload witness captured 31 PDF diagnostics before assertions. One missing schema constant already had its import repaired before that loaded compilation. The remaining diagnostics were test owner paths: conformance codes now resolve to each profile's IO owner, mutation catalog aliases reference binary/text IO, payload constructions reference schema owners, and framing tags reference the declared binary leaf. Test assertions, native pack behavior, inverse rows, and fixture data are preserved. Root owns the focused PDF/GLTF native retry.

The existing PDF neutral source corpus completed through workspace Nx: 54 passed, 0 failed, 1814 assertions across13 files,7.63s. This includes independent SQL and owned JSON/IEEE boundary witnesses; it is not native editor acceptance. Log: `pdf-snapshot-source-followup.log`. DWG current native retry stopped before assertions on one concurrent UI contract E0308; UI repaired it and the fresh retry writes `dwg-controlled-metadata-ui-current.log`.

A fresh39-crate compile showed the prior PDF constant import repair was incomplete: `demo_pdf_snapshot` is declared inside `source_fixtures`, so the outer and codec imports were inaccessible there. The exact nested fixture module now imports the schema constant and the redundant outer import is removed. This was a source-owner scope mistake in this repair cohort, corrected without changing the seed. No native pass is inferred.

## Native DWG Metadata Witness

The fresh canonical controlled-metadata native target completed exit0:7 tests selected,7 passed,90 unrelated skipped; Nextest28ms, whole Nx14m48 including queue/preparation/compilation. The target selects the exact `dwg_controlled_metadata_` prefix; its seven declared laws include `dwg_controlled_metadata_borrowed_manual_carriers_match_the_authored_json`, alongside the existing all-eleven, generic shape, cancellation, interior loops, ceiling and depth laws. The independent source corpus previously passed2/2 with194 assertions. Exact native log:`dwg-controlled-metadata-ui-current.log`. This is native metadata proof, not editor history acceptance.
