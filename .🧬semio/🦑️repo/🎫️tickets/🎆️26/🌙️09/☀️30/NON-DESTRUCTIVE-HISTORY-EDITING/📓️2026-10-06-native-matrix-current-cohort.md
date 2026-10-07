# Current Matrix Native Compile Cohort

The current matrix ungated group observed 169 compiler diagnostics before any editor assertion: PDF92, STEP69, JPG6, EN1995two. The source log is `🗑️generated/tools-execution/native-matrix-pdf-dwg-current.log`. Counts describe compiler errors only. This agent handles PDF/JPG/EN1995; STEP ownership is coordinated with root.

## PDF Exact Diagnostics

```text
error[E0753]: expected outer doc comment
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📄️document/🦀️.rs:2:1
  |
2 | //! 📄️ Complete PDF document ownership and ordered top-level entity collections.
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
3 | use semio_framework_value::{ValueError,ValueRefusalKind};
  | --------------------------------------------------------- the inner doc comment doesn't annotate this `use` import
  |
help: to annotate the `use` import, change the doc comment from inner to outer style
  |
2 - //! 📄️ Complete PDF document ownership and ordered top-level entity collections.
2 + /// 📄️ Complete PDF document ownership and ordered top-level entity collections.
  |
```

```text
error[E0432]: unresolved imports `crate::standards::v1_4::subsets::base::schema::PdfAnalyzer`, `crate::standards::v1_4::subsets::base::schema::PdfParts`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🚪️io/🦀️.rs:154:57
    |
154 |     use crate::standards::v1_4::subsets::base::schema::{PdfAnalyzer as PdfAnyAnalyzer, PdfParts};
    |                                                         -----------^^^^^^^^^^^^^^^^^^  ^^^^^^^^ no `PdfParts` in `standards::v1_4::subsets::base::schema`
    |                                                         |
    |                                                         no `PdfAnalyzer` in `standards::v1_4::subsets::base::schema`
    |
    = help: consider importing one of these structs instead:
            crate::standards::v1_4::subsets::base::io::PdfAnalyzer
            crate::standards::v1_7::subsets::base::io::PdfAnalyzer
    = help: consider importing one of these structs instead:
            crate::standards::v1_4::subsets::base::io::PdfParts
            crate::standards::v1_7::subsets::h::io::PdfParts
```

```text
error[E0432]: unresolved imports `crate::standards::v1_4::subsets::base::schema::PdfAnalyzer`, `crate::standards::v1_4::subsets::base::schema::PdfParts`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🚪️io/🦀️.rs:154:57
    |
154 |     use crate::standards::v1_4::subsets::base::schema::{PdfAnalyzer as PdfAnyAnalyzer, PdfParts};
    |                                                         -----------^^^^^^^^^^^^^^^^^^  ^^^^^^^^ no `PdfParts` in `standards::v1_4::subsets::base::schema`
    |                                                         |
    |                                                         no `PdfAnalyzer` in `standards::v1_4::subsets::base::schema`
    |
    = help: consider importing one of these structs instead:
            crate::standards::v1_4::subsets::base::io::PdfAnalyzer
            crate::standards::v1_7::subsets::base::io::PdfAnalyzer
    = help: consider importing one of these structs instead:
            crate::standards::v1_4::subsets::base::io::PdfParts
            crate::standards::v1_7::subsets::h::io::PdfParts
```

```text
error[E0432]: unresolved import `crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/🧬️mutations/🩹️patch-snapshot/🦀️.rs:6:5
  |
6 | use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------------
  |                                                               |
  |                                                               no `PatchSnapshot` in `standards::v1_7::subsets::base::schema::mutations`
  |
help: a similar name exists in the module
  |
6 - use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
6 + use crate::standards::v1_7::subsets::base::schema::mutations::patch_snapshot;
  |
help: consider importing one of these items instead
  |
6 - use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
6 + use crate::PdfMutation::PatchSnapshot;
  |
6 - use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
6 + use crate::schema::mutations::patch_snapshot::PatchSnapshot;
  |
6 - use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
6 + use crate::standards::v1_4::subsets::base::schema::mutations::PatchSnapshot;
  |
6 - use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
6 + use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation::PatchSnapshot;
  |
```

```text
error[E0432]: unresolved import `crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🩹️patch-snapshot/🦀️.rs:6:5
  |
6 | use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------------
  |                                                               |
  |                                                               no `PatchSnapshot` in `standards::v1_7::subsets::base::schema::mutations`
  |
help: a similar name exists in the module
  |
6 - use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
6 + use crate::standards::v1_7::subsets::base::schema::mutations::patch_snapshot;
  |
help: consider importing one of these items instead
  |
6 - use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
6 + use crate::PdfMutation::PatchSnapshot;
  |
6 - use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
6 + use crate::schema::mutations::patch_snapshot::PatchSnapshot;
  |
6 - use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
6 + use crate::standards::v1_4::subsets::base::schema::mutations::PatchSnapshot;
  |
6 - use crate::standards::v1_7::subsets::base::schema::mutations::PatchSnapshot;
6 + use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation::PatchSnapshot;
  |
```

```text
error[E0433]: cannot find `insert_encryption_dictionary` in `super`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:8:43
  |
8 |     ("InsertEncryptionDictionary", super::insert_encryption_dictionary::text::TEXT_OPCODE),
  |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `insert_encryption_dictionary` in `super`
```

```text
error[E0433]: cannot find `remove_encryption_dictionary` in `super`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:9:43
  |
9 |     ("RemoveEncryptionDictionary", super::remove_encryption_dictionary::text::TEXT_OPCODE),
  |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_encryption_dictionary` in `super`
```

```text
error[E0433]: cannot find `set_output_intent` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:10:32
   |
10 |     ("SetOutputIntent", super::set_output_intent::text::TEXT_OPCODE),
   |                                ^^^^^^^^^^^^^^^^^ could not find `set_output_intent` in `super`
```

```text
error[E0433]: cannot find `remove_output_intent` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:11:35
   |
11 |     ("RemoveOutputIntent", super::remove_output_intent::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^ could not find `remove_output_intent` in `super`
```

```text
error[E0433]: cannot find `set_trim_box` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:12:27
   |
12 |     ("SetTrimBox", super::set_trim_box::text::TEXT_OPCODE),
   |                           ^^^^^^^^^^^^ could not find `set_trim_box` in `super`
```

```text
error[E0433]: cannot find `remove_trim_box` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:30
   |
13 |     ("RemoveTrimBox", super::remove_trim_box::text::TEXT_OPCODE),
   |                              ^^^^^^^^^^^^^^^ could not find `remove_trim_box` in `super`
```

```text
error[E0433]: cannot find `embed_font_file` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:14:30
   |
14 |     ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
   |                              ^^^^^^^^^^^^^^^ could not find `embed_font_file` in `super`
```

```text
error[E0433]: cannot find `remove_font_file` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:31
   |
15 |     ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
   |                               ^^^^^^^^^^^^^^^^ could not find `remove_font_file` in `super`
```

```text
error[E0433]: cannot find `insert_javascript_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:39
   |
16 |     ("InsertJavascriptAction", super::insert_javascript_action::text::TEXT_OPCODE),
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `insert_javascript_action` in `super`
```

```text
error[E0433]: cannot find `remove_javascript_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:39
   |
17 |     ("RemoveJavascriptAction", super::remove_javascript_action::text::TEXT_OPCODE),
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_javascript_action` in `super`
```

```text
error[E0433]: cannot find `insert_launch_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:18:35
   |
18 |     ("InsertLaunchAction", super::insert_launch_action::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^ could not find `insert_launch_action` in `super`
```

```text
error[E0433]: cannot find `remove_launch_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:35
   |
19 |     ("RemoveLaunchAction", super::remove_launch_action::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^ could not find `remove_launch_action` in `super`
```

```text
error[E0433]: cannot find `insert_media_annotation` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:20:38
   |
20 |     ("InsertMediaAnnotation", super::insert_media_annotation::text::TEXT_OPCODE),
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^ could not find `insert_media_annotation` in `super`
```

```text
error[E0433]: cannot find `remove_media_annotation` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs:21:38
   |
21 |     ("RemoveMediaAnnotation", super::remove_media_annotation::text::TEXT_OPCODE),
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_media_annotation` in `super`
```

```text
error[E0433]: cannot find `insert_encryption_dictionary` in `super`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:9:43
  |
9 |     ("InsertEncryptionDictionary", super::insert_encryption_dictionary::text::TEXT_OPCODE),
  |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `insert_encryption_dictionary` in `super`
```

```text
error[E0433]: cannot find `remove_encryption_dictionary` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:10:43
   |
10 |     ("RemoveEncryptionDictionary", super::remove_encryption_dictionary::text::TEXT_OPCODE),
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_encryption_dictionary` in `super`
```

```text
error[E0433]: cannot find `insert_javascript_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:11:39
   |
11 |     ("InsertJavascriptAction", super::insert_javascript_action::text::TEXT_OPCODE),
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `insert_javascript_action` in `super`
```

```text
error[E0433]: cannot find `remove_javascript_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:12:39
   |
12 |     ("RemoveJavascriptAction", super::remove_javascript_action::text::TEXT_OPCODE),
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_javascript_action` in `super`
```

```text
error[E0433]: cannot find `insert_launch_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:35
   |
13 |     ("InsertLaunchAction", super::insert_launch_action::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^ could not find `insert_launch_action` in `super`
```

```text
error[E0433]: cannot find `remove_launch_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:14:35
   |
14 |     ("RemoveLaunchAction", super::remove_launch_action::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^ could not find `remove_launch_action` in `super`
```

```text
error[E0433]: cannot find `insert_media_annotation` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:38
   |
15 |     ("InsertMediaAnnotation", super::insert_media_annotation::text::TEXT_OPCODE),
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^ could not find `insert_media_annotation` in `super`
```

```text
error[E0433]: cannot find `remove_media_annotation` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:38
   |
16 |     ("RemoveMediaAnnotation", super::remove_media_annotation::text::TEXT_OPCODE),
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_media_annotation` in `super`
```

```text
error[E0433]: cannot find `set_output_intent` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:32
   |
17 |     ("SetOutputIntent", super::set_output_intent::text::TEXT_OPCODE),
   |                                ^^^^^^^^^^^^^^^^^ could not find `set_output_intent` in `super`
```

```text
error[E0433]: cannot find `remove_output_intent` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:18:35
   |
18 |     ("RemoveOutputIntent", super::remove_output_intent::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^ could not find `remove_output_intent` in `super`
```

```text
error[E0433]: cannot find `embed_font_file` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:30
   |
19 |     ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
   |                              ^^^^^^^^^^^^^^^ could not find `embed_font_file` in `super`
```

```text
error[E0433]: cannot find `remove_font_file` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs:20:31
   |
20 |     ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
   |                               ^^^^^^^^^^^^^^^^ could not find `remove_font_file` in `super`
```

```text
error[E0433]: cannot find `set_mark_info` in `super`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs:8:28
  |
8 |     ("SetMarkInfo", super::set_mark_info::text::TEXT_OPCODE),
  |                            ^^^^^^^^^^^^^ could not find `set_mark_info` in `super`
```

```text
error[E0433]: cannot find `remove_mark_info` in `super`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs:9:31
  |
9 |     ("RemoveMarkInfo", super::remove_mark_info::text::TEXT_OPCODE),
  |                               ^^^^^^^^^^^^^^^^ could not find `remove_mark_info` in `super`
```

```text
error[E0433]: cannot find `set_struct_tree_root` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs:10:34
   |
10 |     ("SetStructTreeRoot", super::set_struct_tree_root::text::TEXT_OPCODE),
   |                                  ^^^^^^^^^^^^^^^^^^^^ could not find `set_struct_tree_root` in `super`
```

```text
error[E0433]: cannot find `remove_struct_tree_root` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs:11:37
   |
11 |     ("RemoveStructTreeRoot", super::remove_struct_tree_root::text::TEXT_OPCODE),
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_struct_tree_root` in `super`
```

```text
error[E0433]: cannot find `set_lang` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs:12:24
   |
12 |     ("SetLang", super::set_lang::text::TEXT_OPCODE),
   |                        ^^^^^^^^ could not find `set_lang` in `super`
```

```text
error[E0433]: cannot find `remove_lang` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:27
   |
13 |     ("RemoveLang", super::remove_lang::text::TEXT_OPCODE),
   |                           ^^^^^^^^^^^ could not find `remove_lang` in `super`
```

```text
error[E0433]: cannot find `set_display_doc_title` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs:14:35
   |
14 |     ("SetDisplayDocTitle", super::set_display_doc_title::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^^ could not find `set_display_doc_title` in `super`
```

```text
error[E0433]: cannot find `remove_display_doc_title` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:38
   |
15 |     ("RemoveDisplayDocTitle", super::remove_display_doc_title::text::TEXT_OPCODE),
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_display_doc_title` in `super`
```

```text
error[E0433]: cannot find `set_info_title` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:29
   |
16 |     ("SetInfoTitle", super::set_info_title::text::TEXT_OPCODE),
   |                             ^^^^^^^^^^^^^^ could not find `set_info_title` in `super`
```

```text
error[E0433]: cannot find `embed_font_file` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:30
   |
17 |     ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
   |                              ^^^^^^^^^^^^^^^ could not find `embed_font_file` in `super`
```

```text
error[E0433]: cannot find `remove_font_file` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs:18:31
   |
18 |     ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
   |                               ^^^^^^^^^^^^^^^^ could not find `remove_font_file` in `super`
```

```text
error[E0433]: cannot find `insert_encryption_dictionary` in `super`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:8:43
  |
8 |     ("InsertEncryptionDictionary", super::insert_encryption_dictionary::text::TEXT_OPCODE),
  |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `insert_encryption_dictionary` in `super`
```

```text
error[E0433]: cannot find `remove_encryption_dictionary` in `super`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:9:43
  |
9 |     ("RemoveEncryptionDictionary", super::remove_encryption_dictionary::text::TEXT_OPCODE),
  |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_encryption_dictionary` in `super`
```

```text
error[E0433]: cannot find `set_output_intent` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:10:32
   |
10 |     ("SetOutputIntent", super::set_output_intent::text::TEXT_OPCODE),
   |                                ^^^^^^^^^^^^^^^^^ could not find `set_output_intent` in `super`
```

```text
error[E0433]: cannot find `remove_output_intent` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:11:35
   |
11 |     ("RemoveOutputIntent", super::remove_output_intent::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^ could not find `remove_output_intent` in `super`
```

```text
error[E0433]: cannot find `set_trim_box` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:12:27
   |
12 |     ("SetTrimBox", super::set_trim_box::text::TEXT_OPCODE),
   |                           ^^^^^^^^^^^^ could not find `set_trim_box` in `super`
```

```text
error[E0433]: cannot find `remove_trim_box` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:30
   |
13 |     ("RemoveTrimBox", super::remove_trim_box::text::TEXT_OPCODE),
   |                              ^^^^^^^^^^^^^^^ could not find `remove_trim_box` in `super`
```

```text
error[E0433]: cannot find `embed_font_file` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:14:30
   |
14 |     ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
   |                              ^^^^^^^^^^^^^^^ could not find `embed_font_file` in `super`
```

```text
error[E0433]: cannot find `remove_font_file` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:31
   |
15 |     ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
   |                               ^^^^^^^^^^^^^^^^ could not find `remove_font_file` in `super`
```

```text
error[E0433]: cannot find `insert_javascript_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:39
   |
16 |     ("InsertJavascriptAction", super::insert_javascript_action::text::TEXT_OPCODE),
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `insert_javascript_action` in `super`
```

```text
error[E0433]: cannot find `remove_javascript_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:39
   |
17 |     ("RemoveJavascriptAction", super::remove_javascript_action::text::TEXT_OPCODE),
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_javascript_action` in `super`
```

```text
error[E0433]: cannot find `insert_launch_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:18:35
   |
18 |     ("InsertLaunchAction", super::insert_launch_action::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^ could not find `insert_launch_action` in `super`
```

```text
error[E0433]: cannot find `remove_launch_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:35
   |
19 |     ("RemoveLaunchAction", super::remove_launch_action::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^ could not find `remove_launch_action` in `super`
```

```text
error[E0433]: cannot find `insert_media_annotation` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:20:38
   |
20 |     ("InsertMediaAnnotation", super::insert_media_annotation::text::TEXT_OPCODE),
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^ could not find `insert_media_annotation` in `super`
```

```text
error[E0433]: cannot find `remove_media_annotation` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:21:38
   |
21 |     ("RemoveMediaAnnotation", super::remove_media_annotation::text::TEXT_OPCODE),
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_media_annotation` in `super`
```

```text
error[E0433]: cannot find `set_dpart_root` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:22:29
   |
22 |     ("SetDpartRoot", super::set_dpart_root::text::TEXT_OPCODE),
   |                             ^^^^^^^^^^^^^^ could not find `set_dpart_root` in `super`
```

```text
error[E0433]: cannot find `remove_dpart_root` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:23:32
   |
23 |     ("RemoveDpartRoot", super::remove_dpart_root::text::TEXT_OPCODE),
   |                                ^^^^^^^^^^^^^^^^^ could not find `remove_dpart_root` in `super`
```

```text
error[E0433]: cannot find `set_dpart_metadata` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:24:33
   |
24 |     ("SetDpartMetadata", super::set_dpart_metadata::text::TEXT_OPCODE),
   |                                 ^^^^^^^^^^^^^^^^^^ could not find `set_dpart_metadata` in `super`
```

```text
error[E0433]: cannot find `remove_dpart_metadata` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs:25:36
   |
25 |     ("RemoveDpartMetadata", super::remove_dpart_metadata::text::TEXT_OPCODE),
   |                                    ^^^^^^^^^^^^^^^^^^^^^ could not find `remove_dpart_metadata` in `super`
```

```text
error[E0433]: cannot find `set_info_title` in `super`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/📝️text/🧬️mutations/🦀️.rs:8:29
  |
8 |     ("SetInfoTitle", super::set_info_title::text::TEXT_OPCODE),
  |                             ^^^^^^^^^^^^^^ could not find `set_info_title` in `super`
```

```text
error[E0433]: cannot find `set_info_author` in `super`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/📝️text/🧬️mutations/🦀️.rs:9:30
  |
9 |     ("SetInfoAuthor", super::set_info_author::text::TEXT_OPCODE),
  |                              ^^^^^^^^^^^^^^^ could not find `set_info_author` in `super`
```

```text
error[E0433]: cannot find `insert_javascript_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/📝️text/🧬️mutations/🦀️.rs:10:39
   |
10 |     ("InsertJavascriptAction", super::insert_javascript_action::text::TEXT_OPCODE),
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `insert_javascript_action` in `super`
```

```text
error[E0433]: cannot find `remove_javascript_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/📝️text/🧬️mutations/🦀️.rs:11:39
   |
11 |     ("RemoveJavascriptAction", super::remove_javascript_action::text::TEXT_OPCODE),
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_javascript_action` in `super`
```

```text
error[E0433]: cannot find `insert_launch_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/📝️text/🧬️mutations/🦀️.rs:12:35
   |
12 |     ("InsertLaunchAction", super::insert_launch_action::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^ could not find `insert_launch_action` in `super`
```

```text
error[E0433]: cannot find `remove_launch_action` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:35
   |
13 |     ("RemoveLaunchAction", super::remove_launch_action::text::TEXT_OPCODE),
   |                                   ^^^^^^^^^^^^^^^^^^^^ could not find `remove_launch_action` in `super`
```

```text
error[E0433]: cannot find `insert_signature_field` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/📝️text/🧬️mutations/🦀️.rs:14:37
   |
14 |     ("InsertSignatureField", super::insert_signature_field::text::TEXT_OPCODE),
   |                                     ^^^^^^^^^^^^^^^^^^^^^^ could not find `insert_signature_field` in `super`
```

```text
error[E0433]: cannot find `remove_signature_field` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:37
   |
15 |     ("RemoveSignatureField", super::remove_signature_field::text::TEXT_OPCODE),
   |                                     ^^^^^^^^^^^^^^^^^^^^^^ could not find `remove_signature_field` in `super`
```

```text
error[E0433]: cannot find `embed_font_file` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:30
   |
16 |     ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
   |                              ^^^^^^^^^^^^^^^ could not find `embed_font_file` in `super`
```

```text
error[E0433]: cannot find `remove_font_file` in `super`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../../././././🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:31
   |
17 |     ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
   |                               ^^^^^^^^^^^^^^^^ could not find `remove_font_file` in `super`
```

```text
error[E0425]: cannot find value `STDIO_PDF_DOCUMENT_SCHEMA` in this scope
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs:57:38
   |
57 |     let seed = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width: 612.0, height: 792.0, text: "Sem...
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this constant
   |
45 + use crate::STDIO_PDF_DOCUMENT_SCHEMA;
   |
```

```text
error[E0425]: cannot find value `OPCODE` in module `crate::standards::v1_4::subsets::base::schema::mutations::insert_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:77
   |
15 | ...ions::insert_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::print, crate::standards::v1_4::...
   |                       ^^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::insert_page`
   |
help: consider importing one of these constants
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::OPCODE;
   |
   = and 130 other candidates
help: if you import `OPCODE`, refer to it directly
   |
15 -     (crate::standards::v1_4::subsets::base::schema::mutations::insert_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::print, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::parse),
15 +     (OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::print, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::parse),
   |
```

```text
error[E0425]: cannot find value `print` in module `crate::standards::v1_4::subsets::base::schema::mutations::insert_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:156
   |
15 | ...ions::insert_page::print, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::parse),
   |                       ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::insert_page`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::print;
   |
   = and 131 other candidates
help: if you import `print`, refer to it directly
   |
15 -     (crate::standards::v1_4::subsets::base::schema::mutations::insert_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::print, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::parse),
15 +     (crate::standards::v1_4::subsets::base::schema::mutations::insert_page::OPCODE, print, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::parse),
   |
```

```text
error[E0425]: cannot find value `parse` in module `crate::standards::v1_4::subsets::base::schema::mutations::insert_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:234
   |
15 | ...ions::insert_page::parse),
   |                       ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::insert_page`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::parse;
   |
   = and 132 other candidates
help: if you import `parse`, refer to it directly
   |
15 -     (crate::standards::v1_4::subsets::base::schema::mutations::insert_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::print, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::parse),
15 +     (crate::standards::v1_4::subsets::base::schema::mutations::insert_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::print, parse),
   |
```

```text
error[E0425]: cannot find value `OPCODE` in module `crate::standards::v1_4::subsets::base::schema::mutations::remove_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:77
   |
16 | ...ions::remove_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::print, crate::standards::v1_4::...
   |                       ^^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::remove_page`
   |
help: consider importing one of these constants
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::OPCODE;
   |
   = and 130 other candidates
help: if you import `OPCODE`, refer to it directly
   |
16 -     (crate::standards::v1_4::subsets::base::schema::mutations::remove_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::print, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::parse),
16 +     (OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::print, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::parse),
   |
```

```text
error[E0425]: cannot find value `print` in module `crate::standards::v1_4::subsets::base::schema::mutations::remove_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:156
   |
16 | ...ions::remove_page::print, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::parse),
   |                       ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::remove_page`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::print;
   |
   = and 131 other candidates
help: if you import `print`, refer to it directly
   |
16 -     (crate::standards::v1_4::subsets::base::schema::mutations::remove_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::print, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::parse),
16 +     (crate::standards::v1_4::subsets::base::schema::mutations::remove_page::OPCODE, print, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::parse),
   |
```

```text
error[E0425]: cannot find value `parse` in module `crate::standards::v1_4::subsets::base::schema::mutations::remove_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:234
   |
16 | ...ions::remove_page::parse),
   |                       ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::remove_page`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::parse;
   |
   = and 132 other candidates
help: if you import `parse`, refer to it directly
   |
16 -     (crate::standards::v1_4::subsets::base::schema::mutations::remove_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::print, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::parse),
16 +     (crate::standards::v1_4::subsets::base::schema::mutations::remove_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::print, parse),
   |
```

```text
error[E0425]: cannot find value `OPCODE` in module `crate::standards::v1_4::subsets::base::schema::mutations::move_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:75
   |
17 | ...tations::move_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::move_page::print, crate::standards::v1_4::s...
   |                        ^^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::move_page`
   |
help: consider importing one of these constants
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::OPCODE;
   |
   = and 130 other candidates
help: if you import `OPCODE`, refer to it directly
   |
17 -     (crate::standards::v1_4::subsets::base::schema::mutations::move_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::move_page::print, crate::standards::v1_4::subsets::base::schema::mutations::move_page::parse),
17 +     (OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::move_page::print, crate::standards::v1_4::subsets::base::schema::mutations::move_page::parse),
   |
```

```text
error[E0425]: cannot find value `print` in module `crate::standards::v1_4::subsets::base::schema::mutations::move_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:152
   |
17 | ...tations::move_page::print, crate::standards::v1_4::subsets::base::schema::mutations::move_page::parse),
   |                        ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::move_page`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::print;
   |
   = and 131 other candidates
help: if you import `print`, refer to it directly
   |
17 -     (crate::standards::v1_4::subsets::base::schema::mutations::move_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::move_page::print, crate::standards::v1_4::subsets::base::schema::mutations::move_page::parse),
17 +     (crate::standards::v1_4::subsets::base::schema::mutations::move_page::OPCODE, print, crate::standards::v1_4::subsets::base::schema::mutations::move_page::parse),
   |
```

```text
error[E0425]: cannot find value `parse` in module `crate::standards::v1_4::subsets::base::schema::mutations::move_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:228
   |
17 | ...tations::move_page::parse),
   |                        ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::move_page`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::parse;
   |
   = and 132 other candidates
help: if you import `parse`, refer to it directly
   |
17 -     (crate::standards::v1_4::subsets::base::schema::mutations::move_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::move_page::print, crate::standards::v1_4::subsets::base::schema::mutations::move_page::parse),
17 +     (crate::standards::v1_4::subsets::base::schema::mutations::move_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::move_page::print, parse),
   |
```

```text
error[E0425]: cannot find value `OPCODE` in module `crate::standards::v1_4::subsets::base::schema::mutations::resize_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:18:77
   |
18 | ...ions::resize_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::print, crate::standards::v1_4::...
   |                       ^^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::resize_page`
   |
help: consider importing one of these constants
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::OPCODE;
   |
   = and 130 other candidates
help: if you import `OPCODE`, refer to it directly
   |
18 -     (crate::standards::v1_4::subsets::base::schema::mutations::resize_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::print, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::parse),
18 +     (OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::print, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::parse),
   |
```

```text
error[E0425]: cannot find value `print` in module `crate::standards::v1_4::subsets::base::schema::mutations::resize_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:18:156
   |
18 | ...ions::resize_page::print, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::parse),
   |                       ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::resize_page`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::print;
   |
   = and 131 other candidates
help: if you import `print`, refer to it directly
   |
18 -     (crate::standards::v1_4::subsets::base::schema::mutations::resize_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::print, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::parse),
18 +     (crate::standards::v1_4::subsets::base::schema::mutations::resize_page::OPCODE, print, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::parse),
   |
```

```text
error[E0425]: cannot find value `parse` in module `crate::standards::v1_4::subsets::base::schema::mutations::resize_page`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:18:234
   |
18 | ...ions::resize_page::parse),
   |                       ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::resize_page`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::parse;
   |
   = and 132 other candidates
help: if you import `parse`, refer to it directly
   |
18 -     (crate::standards::v1_4::subsets::base::schema::mutations::resize_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::print, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::parse),
18 +     (crate::standards::v1_4::subsets::base::schema::mutations::resize_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::print, parse),
   |
```

```text
error[E0425]: cannot find value `OPCODE` in module `crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:83
   |
19 | ...lace_page_text::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::print, crate::standards::v1_...
   |                    ^^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text`
   |
help: consider importing one of these constants
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::OPCODE;
   |
   = and 130 other candidates
help: if you import `OPCODE`, refer to it directly
   |
19 -     (crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::print, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::parse),
19 +     (OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::print, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::parse),
   |
```

```text
error[E0425]: cannot find value `print` in module `crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:168
   |
19 | ...lace_page_text::print, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::parse),
   |                    ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::print;
   |
   = and 131 other candidates
help: if you import `print`, refer to it directly
   |
19 -     (crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::print, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::parse),
19 +     (crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::OPCODE, print, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::parse),
   |
```

```text
error[E0425]: cannot find value `parse` in module `crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:252
   |
19 | ...lace_page_text::parse),
   |                    ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::parse;
   |
   = and 132 other candidates
help: if you import `parse`, refer to it directly
   |
19 -     (crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::print, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::parse),
19 +     (crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::print, parse),
   |
```

```text
error[E0425]: cannot find value `OPCODE` in module `crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:20:78
   |
20 | ...ns::set_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::print, crate::standards::v1_4::...
   |                      ^^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot`
   |
help: consider importing one of these constants
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::OPCODE;
   |
   = and 130 other candidates
help: if you import `OPCODE`, refer to it directly
   |
20 -     (crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::parse),
20 +     (OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::parse),
   |
```

```text
error[E0425]: cannot find value `print` in module `crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:20:158
   |
20 | ...ons::set_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::parse),
   |                       ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::print;
   |
   = and 131 other candidates
help: if you import `print`, refer to it directly
   |
20 -     (crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::parse),
20 +     (crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::OPCODE, print, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::parse),
   |
```

```text
error[E0425]: cannot find value `parse` in module `crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:20:237
   |
20 | ...ons::set_snapshot::parse),
   |                       ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::parse;
   |
   = and 132 other candidates
help: if you import `parse`, refer to it directly
   |
20 -     (crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::parse),
20 +     (crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::print, parse),
   |
```

```text
error[E0425]: cannot find value `OPCODE` in module `crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:21:80
   |
21 | ...:patch_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::print, crate::standards::v1_4:...
   |                     ^^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot`
   |
help: consider importing one of these constants
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::OPCODE;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::OPCODE;
   |
   = and 130 other candidates
help: if you import `OPCODE`, refer to it directly
   |
21 -     (crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::parse),
21 +     (OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::parse),
   |
```

```text
error[E0425]: cannot find value `print` in module `crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:21:162
   |
21 | ...::patch_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::parse),
   |                      ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::print;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::print;
   |
   = and 131 other candidates
help: if you import `print`, refer to it directly
   |
21 -     (crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::parse),
21 +     (crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::OPCODE, print, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::parse),
   |
```

```text
error[E0425]: cannot find value `parse` in module `crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/../.././././././🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs:21:243
   |
21 | ...::patch_snapshot::parse),
   |                      ^^^^^ not found in `crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot`
   |
help: consider importing one of these functions
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::insert_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::move_page::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::patch_snapshot::parse;
   |
 3 + use crate::standards::v1_4::subsets::base::io::text::mutations::remove_page::parse;
   |
   = and 132 other candidates
help: if you import `parse`, refer to it directly
   |
21 -     (crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::parse),
21 +     (crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::print, parse),
   |
```


## Current Source Repairs

PDF's additional 92 compiler diagnostics resolved to canonical references and one inserted-import ordering error: 65 direct text mutation tag registry paths, 21 PDF1.4 text registry printer/parser/opcode references, two analyzer/parts imports, two direct PatchSnapshot payload imports, the PDF1.4 schema constant and module doc ordering. No new payload algorithm was introduced.

JPG's duplicate IO mount and dead engine barrel are removed. Current declaration/registry, baseline conformance, actual JPEG import/export and Remodel JPEG codec consumers all use the canonical JFIF1.01 document IO owner. Its native decoded snapshot carries explicit Self type inference. EN1995 diff grammar references its declared IO text diff owner. STEP's 69 diagnostic cohort is documented separately in 📓️2026-10-06-step-compile-cohort.md. The ungated group executed zero assertions and requires a fresh rerun after these repairs; the matrix invocation continues its assembly group.

The assembly group later diagnosed one duplicate HTML schema-unit write_html_document import after concurrent canonical helper repairs. The one repeated single import was removed; the combined canonical parse/write import remains. This is a compiler fix, not a failed editor assertion.

## Completed Loaded Matrix And Fresh Retry

The `native-matrix-pdf-dwg-current.log` run completed with exit 1 and 170 compiler diagnostics: PDF 92, STEP 69, JPG 6, EN1995 2, HTML 1. The first four groups were captured before their source repairs; assembly stopped on the single duplicate HTML import. Neither group selected editor assertions. The fresh full 96-crate matrix retry writes `native-matrix-svg-pdf-tests-current.log` and includes the subsequent SVG/PDF test namespace repairs. No runtime editor coverage is claimed from the census.

The next complete loaded run (`native-matrix-svg-pdf-tests-current.log`) ended exit1 with40 diagnostics: one prior UI description type mismatch in the39-crate group, and39 MP3 diagnostics in assembly. Both groups selected zero assertions. MP3 diagnostics resolve to three source gaps, now repaired; the39-crate retry is `native-matrix-ungated-ui-current.log` and the57-crate retry is `native-matrix-assembly-mp3-current.log`. The latter declares109 registrations; both sum to148. Focused Draw choices and DWG borrowed metadata also have current retries, pending assertion execution.

The corrected39-crate group ended exit1 with9 compile diagnostics and zero assertions: the PDF nested fixture constant import, six concurrent duplicate GLTF borrowed-role declarations (coordinator owns and repaired), and two BMP stale namespace declarations. BMP registry now uses canonical standard/subset IO, its dead engine barrel is removed, and its two codec consumers explicitly target actual IO encode/decode. Fixtures and behavior are unchanged. Fresh runtime validation remains required.

BMP source corpus initially executed6 passing/3 failing tests (100 assertions), the failures all caused by the fixture loader concatenating a directory and filename without a slash. The separator-only repair preserves fixtures and assertions; fresh source retry completed9 passing,0 failing,186 assertions in1.375s. The current assembly compile separately exposed an I-JSON editor conformance helper still referenced through schema; it now explicitly uses its authored IO owner. Native editor assertions remain pending.

The57-crate MP3-corrected assembly retry completed exit1 with exactly one I-JSON conformance namespace diagnostic and zero native assertions. That source reference is repaired; fresh assembly retry writes `native-matrix-assembly-json-current.log`. The39-crate post-BMP group remains queued.

The39-crate post-BMP compilation reached EN1998 and diagnosed stale pilot grammar/protocol references, mutation type owner, and SQLite codec callback (16 diagnostics as captured so far). Existing IO text/binary facet owners now explicitly supply each pilot field; the binary mutation codec imports its declared schema enum; the record macro callback references actual IO SQLite snapshot. Algorithms and fixture values are unchanged. The concurrent focused Draw compile stopped on replication's retirement macro typo, referred to and repaired by core. Core requested no new broad reruns until its retained integration source boundary settles; already-running captures continue.

## Current 39/57 Captures

`native-matrix-ungated-bmp-current.log`: 39 selected crate owners, compilation failed on TIFF7 and EN1998 22. No selected editor test reached execution. The canonical source repairs are described in the Puzzle3d/IFC cohort report; fresh broad rerun awaits Core source stability.

`native-matrix-assembly-json-current.log`: 57 selected crate owners, compilation failed on BMP25 and GIF15, zero editor assertions executed. The live root captures have independently exposed Puzzle3d53/IFC44; our source repairs pass their independent source suites but are not retrospectively attributed to earlier loaded-source compiler captures.

BMP current mutation codec source repairs passed **9/9 independent SQLite/Ajv tests, 186 assertions, 613ms**. GIF compiler ownership repairs were applied before its follow-up was handed to UI; its existing source oracle actually ran **3 pass / 3 fail / 39 assertions / 1.447s**. UI owns the three observed runtime/source-law failures and remaining external GIF consumer paths. Both cohorts require a fresh native matrix after shared source stability.
