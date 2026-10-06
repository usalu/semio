# Semio Whole Native Before TXT Compiler Diagnostics

Root85119 terminal exit1, no Nextest or owning runtime qualification. Exactly78 Rust diagnostics:12 E0255 duplicate imported leaf modules,12 E0432 obsolete IO-root Payload imports,54 E0433 removed binary/text nested routes (24binary,30text). Every diagnostic is retained below as bounded nine-line original context.

Current source readback differs from captured failure: all12 leaf payload imports already name schema::mutations::*Payload, which are actual schema aggregate pub-use definitions. Do not overwrite them with old IO payload imports. Current two codec roots still import six schema leaf modules colliding with actual same-named IO submodules. Retain only TxtMutation import; local leaf codec references lose the redundant ::binary or ::text segment. Binary BINARY_TAGS references actual sibling IO text mutation modules explicitly. No adapters, metadata/type visibility changes, framing/grants or test-body changes are needed. Exact two proposed guards retained in inputs/txt-current-io-mutation-two-readonly-canonical-pairs.json.

Authority schema aggregate: [TXT mutation declarations](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:22). Actual defining IO codec modules are registered by existing pub mod declarations in the two roots; their payload imports already target public schema aliases.

### Diagnostic 1 (log line 16830)

```text
error[E0255]: the name `insert_line` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:62:1
   |
 6 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                ----------- previous import of the module `insert_line` here
...
62 | pub mod insert_line;
   | ^^^^^^^^^^^^^^^^^^^^ `insert_line` redefined here
   |
```

### Diagnostic 2 (log line 16845)

```text
error[E0255]: the name `remove_line` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:74:1
   |
 6 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                             ----------- previous import of the module `remove_line` here
...
74 | pub mod remove_line;
   | ^^^^^^^^^^^^^^^^^^^^ `remove_line` redefined here
   |
```

### Diagnostic 3 (log line 16860)

```text
error[E0255]: the name `set_line` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:65:1
   |
 6 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                                          -------- previous import of the module `set_line` here
...
65 | pub mod set_line;
   | ^^^^^^^^^^^^^^^^^ `set_line` redefined here
   |
```

### Diagnostic 4 (log line 16875)

```text
error[E0255]: the name `set_line_ending` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:71:1
   |
 6 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                                                    --------------- previous import of the module `set_line_ending` here
...
71 | pub mod set_line_ending;
   | ^^^^^^^^^^^^^^^^^^^^^^^^ `set_line_ending` redefined here
   |
```

### Diagnostic 5 (log line 16890)

```text
error[E0255]: the name `set_snapshot` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:68:1
   |
 6 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                                                                     ------------ previous import of the module `set_snapshot` here
...
68 | pub mod set_snapshot;
   | ^^^^^^^^^^^^^^^^^^^^^ `set_snapshot` redefined here
   |
```

### Diagnostic 6 (log line 16905)

```text
error[E0255]: the name `set_trailing_newline` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:77:1
   |
 6 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                                                                                   -------------------- previous import of the module `set_trailing_newline` here
...
77 | pub mod set_trailing_newline;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `set_trailing_newline` redefined here
   |
```

### Diagnostic 7 (log line 16920)

```text
error[E0255]: the name `insert_line` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:77:1
   |
 3 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                ----------- previous import of the module `insert_line` here
...
77 | pub mod insert_line;
   | ^^^^^^^^^^^^^^^^^^^^ `insert_line` redefined here
   |
```

### Diagnostic 8 (log line 16935)

```text
error[E0255]: the name `remove_line` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:89:1
   |
 3 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                             ----------- previous import of the module `remove_line` here
...
89 | pub mod remove_line;
   | ^^^^^^^^^^^^^^^^^^^^ `remove_line` redefined here
   |
```

### Diagnostic 9 (log line 16950)

```text
error[E0255]: the name `set_line` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:80:1
   |
 3 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                                          -------- previous import of the module `set_line` here
...
80 | pub mod set_line;
   | ^^^^^^^^^^^^^^^^^ `set_line` redefined here
   |
```

### Diagnostic 10 (log line 16965)

```text
error[E0255]: the name `set_line_ending` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:86:1
   |
 3 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                                                    --------------- previous import of the module `set_line_ending` here
...
86 | pub mod set_line_ending;
   | ^^^^^^^^^^^^^^^^^^^^^^^^ `set_line_ending` redefined here
   |
```

### Diagnostic 11 (log line 16980)

```text
error[E0255]: the name `set_snapshot` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:83:1
   |
 3 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                                                                     ------------ previous import of the module `set_snapshot` here
...
83 | pub mod set_snapshot;
   | ^^^^^^^^^^^^^^^^^^^^^ `set_snapshot` redefined here
   |
```

### Diagnostic 12 (log line 16995)

```text
error[E0255]: the name `set_trailing_newline` is defined multiple times
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:92:1
   |
 3 | use crate::schema::mutations::{insert_line, remove_line, set_line, set_line_ending, set_snapshot, set_trailing_newline, TxtMutation};
   |                                                                                                   -------------------- previous import of the module `set_trailing_newline` here
...
92 | pub mod set_trailing_newline;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `set_trailing_newline` redefined here
   |
```

### Diagnostic 13 (log line 17015)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::binary::mutations::InsertLinePayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/📥️insert-line/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::binary::mutations::InsertLinePayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-----------------
  |                                                                     |
  |                                                                     no `InsertLinePayload` in `standards::v_utf_8::subsets::any::io::component::binary::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 14 (log line 17029)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::binary::mutations::SetLinePayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/✏️set-line/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::binary::mutations::SetLinePayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------------
  |                                                                     |
  |                                                                     no `SetLinePayload` in `standards::v_utf_8::subsets::any::io::component::binary::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 15 (log line 17043)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::binary::mutations::SetSnapshotPayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/📸️set-snapshot/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::binary::mutations::SetSnapshotPayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------------------
  |                                                                     |
  |                                                                     no `SetSnapshotPayload` in `standards::v_utf_8::subsets::any::io::component::binary::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 16 (log line 17057)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::binary::mutations::SetLineEndingPayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🔚️set-line-ending/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::binary::mutations::SetLineEndingPayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------------------
  |                                                                     |
  |                                                                     no `SetLineEndingPayload` in `standards::v_utf_8::subsets::any::io::component::binary::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 17 (log line 17071)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::binary::mutations::RemoveLinePayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🗑️remove-line/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::binary::mutations::RemoveLinePayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-----------------
  |                                                                     |
  |                                                                     no `RemoveLinePayload` in `standards::v_utf_8::subsets::any::io::component::binary::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 18 (log line 17085)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::binary::mutations::SetTrailingNewlinePayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/↩️set-trailing-newline/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::binary::mutations::SetTrailingNewlinePayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------------------------
  |                                                                     |
  |                                                                     no `SetTrailingNewlinePayload` in `standards::v_utf_8::subsets::any::io::component::binary::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 19 (log line 17099)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::text::mutations::InsertLinePayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/📥️insert-line/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::text::mutations::InsertLinePayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-----------------
  |                                                                   |
  |                                                                   no `InsertLinePayload` in `standards::v_utf_8::subsets::any::io::component::text::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 20 (log line 17113)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::text::mutations::SetLinePayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/✏️set-line/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::text::mutations::SetLinePayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------------
  |                                                                   |
  |                                                                   no `SetLinePayload` in `standards::v_utf_8::subsets::any::io::component::text::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 21 (log line 17127)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::text::mutations::SetSnapshotPayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/📸️set-snapshot/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::text::mutations::SetSnapshotPayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------------------
  |                                                                   |
  |                                                                   no `SetSnapshotPayload` in `standards::v_utf_8::subsets::any::io::component::text::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 22 (log line 17141)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::text::mutations::SetLineEndingPayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🔚️set-line-ending/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::text::mutations::SetLineEndingPayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------------------
  |                                                                   |
  |                                                                   no `SetLineEndingPayload` in `standards::v_utf_8::subsets::any::io::component::text::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 23 (log line 17155)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::text::mutations::RemoveLinePayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🗑️remove-line/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::text::mutations::RemoveLinePayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-----------------
  |                                                                   |
  |                                                                   no `RemoveLinePayload` in `standards::v_utf_8::subsets::any::io::component::text::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 24 (log line 17169)

```text
error[E0432]: unresolved import `crate::standards::v_utf_8::subsets::any::io::text::mutations::SetTrailingNewlinePayload`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/↩️set-trailing-newline/🦀️.rs:3:5
  |
3 | use crate::standards::v_utf_8::subsets::any::io::text::mutations::SetTrailingNewlinePayload;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------------------------
  |                                                                   |
  |                                                                   no `SetTrailingNewlinePayload` in `standards::v_utf_8::subsets::any::io::component::text::mutations`
  |
help: consider importing this type alias through its public re-export instead
```

### Diagnostic 25 (log line 17183)

```text
error[E0433]: cannot find `binary` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:15:38
   |
15 |     BinaryCodec { tag: set_snapshot::binary::BINARY_TAG, try_encode: set_snapshot::binary::try_encode, decode: set_snapshot::binary:...
   |                                      ^^^^^^ could not find `binary` in `set_snapshot`

error[E0433]: cannot find `binary` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:15:84
   |
```

### Diagnostic 26 (log line 17189)

```text
error[E0433]: cannot find `binary` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:15:84
   |
15 | ...::BINARY_TAG, try_encode: set_snapshot::binary::try_encode, decode: set_snapshot::binary::decode_mutation },
   |                                            ^^^^^^ could not find `binary` in `set_snapshot`

error[E0433]: cannot find `binary` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:15:126
   |
```

### Diagnostic 27 (log line 17195)

```text
error[E0433]: cannot find `binary` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:15:126
   |
15 | ...nary::try_encode, decode: set_snapshot::binary::decode_mutation },
   |                                            ^^^^^^ could not find `binary` in `set_snapshot`

error[E0433]: cannot find `binary` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:16:46
   |
```

### Diagnostic 28 (log line 17201)

```text
error[E0433]: cannot find `binary` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:16:46
   |
16 |     BinaryCodec { tag: set_trailing_newline::binary::BINARY_TAG, try_encode: set_trailing_newline::binary::try_encode, decode: set_t...
   |                                              ^^^^^^ could not find `binary` in `set_trailing_newline`

error[E0433]: cannot find `binary` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:16:100
   |
```

### Diagnostic 29 (log line 17207)

```text
error[E0433]: cannot find `binary` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:16:100
   |
16 | ..., try_encode: set_trailing_newline::binary::try_encode, decode: set_trailing_newline::binary::decode_mutation },
   |                                        ^^^^^^ could not find `binary` in `set_trailing_newline`

error[E0433]: cannot find `binary` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:16:150
   |
```

### Diagnostic 30 (log line 17213)

```text
error[E0433]: cannot find `binary` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:16:150
   |
16 | ...code, decode: set_trailing_newline::binary::decode_mutation },
   |                                        ^^^^^^ could not find `binary` in `set_trailing_newline`

error[E0433]: cannot find `binary` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:17:41
   |
```

### Diagnostic 31 (log line 17219)

```text
error[E0433]: cannot find `binary` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:17:41
   |
17 |     BinaryCodec { tag: set_line_ending::binary::BINARY_TAG, try_encode: set_line_ending::binary::try_encode, decode: set_line_ending...
   |                                         ^^^^^^ could not find `binary` in `set_line_ending`

error[E0433]: cannot find `binary` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:17:90
   |
```

### Diagnostic 32 (log line 17225)

```text
error[E0433]: cannot find `binary` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:17:90
   |
17 | ...NARY_TAG, try_encode: set_line_ending::binary::try_encode, decode: set_line_ending::binary::decode_mutation },
   |                                           ^^^^^^ could not find `binary` in `set_line_ending`

error[E0433]: cannot find `binary` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:17:135
   |
```

### Diagnostic 33 (log line 17231)

```text
error[E0433]: cannot find `binary` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:17:135
   |
17 | ...::try_encode, decode: set_line_ending::binary::decode_mutation },
   |                                           ^^^^^^ could not find `binary` in `set_line_ending`

error[E0433]: cannot find `binary` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:18:37
   |
```

### Diagnostic 34 (log line 17237)

```text
error[E0433]: cannot find `binary` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:18:37
   |
18 |     BinaryCodec { tag: insert_line::binary::BINARY_TAG, try_encode: insert_line::binary::try_encode, decode: insert_line::binary::de...
   |                                     ^^^^^^ could not find `binary` in `insert_line`

error[E0433]: cannot find `binary` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:18:82
   |
```

### Diagnostic 35 (log line 17243)

```text
error[E0433]: cannot find `binary` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:18:82
   |
18 |     BinaryCodec { tag: insert_line::binary::BINARY_TAG, try_encode: insert_line::binary::try_encode, decode: insert_line::binary::de...
   |                                                                                  ^^^^^^ could not find `binary` in `insert_line`

error[E0433]: cannot find `binary` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:18:123
   |
```

### Diagnostic 36 (log line 17249)

```text
error[E0433]: cannot find `binary` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:18:123
   |
18 | ...binary::try_encode, decode: insert_line::binary::decode_mutation },
   |                                             ^^^^^^ could not find `binary` in `insert_line`

error[E0433]: cannot find `binary` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:19:37
   |
```

### Diagnostic 37 (log line 17255)

```text
error[E0433]: cannot find `binary` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:19:37
   |
19 |     BinaryCodec { tag: remove_line::binary::BINARY_TAG, try_encode: remove_line::binary::try_encode, decode: remove_line::binary::de...
   |                                     ^^^^^^ could not find `binary` in `remove_line`

error[E0433]: cannot find `binary` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:19:82
   |
```

### Diagnostic 38 (log line 17261)

```text
error[E0433]: cannot find `binary` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:19:82
   |
19 |     BinaryCodec { tag: remove_line::binary::BINARY_TAG, try_encode: remove_line::binary::try_encode, decode: remove_line::binary::de...
   |                                                                                  ^^^^^^ could not find `binary` in `remove_line`

error[E0433]: cannot find `binary` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:19:123
   |
```

### Diagnostic 39 (log line 17267)

```text
error[E0433]: cannot find `binary` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:19:123
   |
19 | ...binary::try_encode, decode: remove_line::binary::decode_mutation },
   |                                             ^^^^^^ could not find `binary` in `remove_line`

error[E0433]: cannot find `binary` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:20:34
   |
```

### Diagnostic 40 (log line 17273)

```text
error[E0433]: cannot find `binary` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:20:34
   |
20 |     BinaryCodec { tag: set_line::binary::BINARY_TAG, try_encode: set_line::binary::try_encode, decode: set_line::binary::decode_muta...
   |                                  ^^^^^^ could not find `binary` in `set_line`

error[E0433]: cannot find `binary` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:20:76
   |
```

### Diagnostic 41 (log line 17279)

```text
error[E0433]: cannot find `binary` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:20:76
   |
20 |     BinaryCodec { tag: set_line::binary::BINARY_TAG, try_encode: set_line::binary::try_encode, decode: set_line::binary::decode_muta...
   |                                                                            ^^^^^^ could not find `binary` in `set_line`

error[E0433]: cannot find `binary` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:20:114
   |
```

### Diagnostic 42 (log line 17285)

```text
error[E0433]: cannot find `binary` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:20:114
   |
20 | ...ne::binary::try_encode, decode: set_line::binary::decode_mutation },
   |                                              ^^^^^^ could not find `binary` in `set_line`

error[E0433]: cannot find `text` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:23:20
   |
```

### Diagnostic 43 (log line 17291)

```text
error[E0433]: cannot find `text` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:23:20
   |
23 |     (set_snapshot::text::TEXT_OPCODE, set_snapshot::binary::BINARY_TAG),
   |                    ^^^^ could not find `text` in `set_snapshot`

error[E0433]: cannot find `binary` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:23:53
   |
```

### Diagnostic 44 (log line 17297)

```text
error[E0433]: cannot find `binary` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:23:53
   |
23 |     (set_snapshot::text::TEXT_OPCODE, set_snapshot::binary::BINARY_TAG),
   |                                                     ^^^^^^ could not find `binary` in `set_snapshot`

error[E0433]: cannot find `text` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:24:28
   |
```

### Diagnostic 45 (log line 17303)

```text
error[E0433]: cannot find `text` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:24:28
   |
24 |     (set_trailing_newline::text::TEXT_OPCODE, set_trailing_newline::binary::BINARY_TAG),
   |                            ^^^^ could not find `text` in `set_trailing_newline`

error[E0433]: cannot find `binary` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:24:69
   |
```

### Diagnostic 46 (log line 17309)

```text
error[E0433]: cannot find `binary` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:24:69
   |
24 |     (set_trailing_newline::text::TEXT_OPCODE, set_trailing_newline::binary::BINARY_TAG),
   |                                                                     ^^^^^^ could not find `binary` in `set_trailing_newline`

error[E0433]: cannot find `text` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:25:23
   |
```

### Diagnostic 47 (log line 17315)

```text
error[E0433]: cannot find `text` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:25:23
   |
25 |     (set_line_ending::text::TEXT_OPCODE, set_line_ending::binary::BINARY_TAG),
   |                       ^^^^ could not find `text` in `set_line_ending`

error[E0433]: cannot find `binary` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:25:59
   |
```

### Diagnostic 48 (log line 17321)

```text
error[E0433]: cannot find `binary` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:25:59
   |
25 |     (set_line_ending::text::TEXT_OPCODE, set_line_ending::binary::BINARY_TAG),
   |                                                           ^^^^^^ could not find `binary` in `set_line_ending`

error[E0433]: cannot find `text` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:26:19
   |
```

### Diagnostic 49 (log line 17327)

```text
error[E0433]: cannot find `text` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:26:19
   |
26 |     (insert_line::text::TEXT_OPCODE, insert_line::binary::BINARY_TAG),
   |                   ^^^^ could not find `text` in `insert_line`

error[E0433]: cannot find `binary` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:26:51
   |
```

### Diagnostic 50 (log line 17333)

```text
error[E0433]: cannot find `binary` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:26:51
   |
26 |     (insert_line::text::TEXT_OPCODE, insert_line::binary::BINARY_TAG),
   |                                                   ^^^^^^ could not find `binary` in `insert_line`

error[E0433]: cannot find `text` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:27:19
   |
```

### Diagnostic 51 (log line 17339)

```text
error[E0433]: cannot find `text` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:27:19
   |
27 |     (remove_line::text::TEXT_OPCODE, remove_line::binary::BINARY_TAG),
   |                   ^^^^ could not find `text` in `remove_line`

error[E0433]: cannot find `binary` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:27:51
   |
```

### Diagnostic 52 (log line 17345)

```text
error[E0433]: cannot find `binary` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:27:51
   |
27 |     (remove_line::text::TEXT_OPCODE, remove_line::binary::BINARY_TAG),
   |                                                   ^^^^^^ could not find `binary` in `remove_line`

error[E0433]: cannot find `text` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:28:16
   |
```

### Diagnostic 53 (log line 17351)

```text
error[E0433]: cannot find `text` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:28:16
   |
28 |     (set_line::text::TEXT_OPCODE, set_line::binary::BINARY_TAG),
   |                ^^^^ could not find `text` in `set_line`

error[E0433]: cannot find `binary` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:28:45
   |
```

### Diagnostic 54 (log line 17357)

```text
error[E0433]: cannot find `binary` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:28:45
   |
28 |     (set_line::text::TEXT_OPCODE, set_line::binary::BINARY_TAG),
   |                                             ^^^^^^ could not find `binary` in `set_line`

error[E0433]: cannot find `text` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:12:39
   |
```

### Diagnostic 55 (log line 17363)

```text
error[E0433]: cannot find `text` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:12:39
   |
12 |     TextCodec { opcode: set_snapshot::text::TEXT_OPCODE, try_encode: set_snapshot::text::try_encode, decode: set_snapshot::text::dec...
   |                                       ^^^^ could not find `text` in `set_snapshot`

error[E0433]: cannot find `text` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:12:84
   |
```

### Diagnostic 56 (log line 17369)

```text
error[E0433]: cannot find `text` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:12:84
   |
12 |     TextCodec { opcode: set_snapshot::text::TEXT_OPCODE, try_encode: set_snapshot::text::try_encode, decode: set_snapshot::text::dec...
   |                                                                                    ^^^^ could not find `text` in `set_snapshot`

error[E0433]: cannot find `text` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:12:124
   |
```

### Diagnostic 57 (log line 17375)

```text
error[E0433]: cannot find `text` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:12:124
   |
12 | ...::text::try_encode, decode: set_snapshot::text::decode_mutation },
   |                                              ^^^^ could not find `text` in `set_snapshot`

error[E0433]: cannot find `text` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:47
   |
```

### Diagnostic 58 (log line 17381)

```text
error[E0433]: cannot find `text` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:47
   |
13 |     TextCodec { opcode: set_trailing_newline::text::TEXT_OPCODE, try_encode: set_trailing_newline::text::try_encode, decode: set_tra...
   |                                               ^^^^ could not find `text` in `set_trailing_newline`

error[E0433]: cannot find `text` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:100
   |
```

### Diagnostic 59 (log line 17387)

```text
error[E0433]: cannot find `text` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:100
   |
13 | ...DE, try_encode: set_trailing_newline::text::try_encode, decode: set_trailing_newline::text::decode_mutation },
   |                                          ^^^^ could not find `text` in `set_trailing_newline`

error[E0433]: cannot find `text` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:148
   |
```

### Diagnostic 60 (log line 17393)

```text
error[E0433]: cannot find `text` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:13:148
   |
13 | ...encode, decode: set_trailing_newline::text::decode_mutation },
   |                                          ^^^^ could not find `text` in `set_trailing_newline`

[cargo:build] running elapsedMs=750504
[artifact-rust:semio-s-artifact-stdio-semio:test] running elapsedMs=1694408
error[E0433]: cannot find `text` in `set_line_ending`
```

### Diagnostic 61 (log line 17401)

```text
error[E0433]: cannot find `text` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:14:42
   |
14 |     TextCodec { opcode: set_line_ending::text::TEXT_OPCODE, try_encode: set_line_ending::text::try_encode, decode: set_line_ending::...
   |                                          ^^^^ could not find `text` in `set_line_ending`

error[E0433]: cannot find `text` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:14:90
   |
```

### Diagnostic 62 (log line 17407)

```text
error[E0433]: cannot find `text` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:14:90
   |
14 | ...EXT_OPCODE, try_encode: set_line_ending::text::try_encode, decode: set_line_ending::text::decode_mutation },
   |                                             ^^^^ could not find `text` in `set_line_ending`

error[E0433]: cannot find `text` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:14:133
   |
```

### Diagnostic 63 (log line 17413)

```text
error[E0433]: cannot find `text` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:14:133
   |
14 | ...xt::try_encode, decode: set_line_ending::text::decode_mutation },
   |                                             ^^^^ could not find `text` in `set_line_ending`

error[E0433]: cannot find `text` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:38
   |
```

### Diagnostic 64 (log line 17419)

```text
error[E0433]: cannot find `text` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:38
   |
15 |     TextCodec { opcode: insert_line::text::TEXT_OPCODE, try_encode: insert_line::text::try_encode, decode: insert_line::text::decode...
   |                                      ^^^^ could not find `text` in `insert_line`

error[E0433]: cannot find `text` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:82
   |
```

### Diagnostic 65 (log line 17425)

```text
error[E0433]: cannot find `text` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:82
   |
15 |     TextCodec { opcode: insert_line::text::TEXT_OPCODE, try_encode: insert_line::text::try_encode, decode: insert_line::text::decode...
   |                                                                                  ^^^^ could not find `text` in `insert_line`

error[E0433]: cannot find `text` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:121
   |
```

### Diagnostic 66 (log line 17431)

```text
error[E0433]: cannot find `text` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:15:121
   |
15 | ...ne::text::try_encode, decode: insert_line::text::decode_mutation },
   |                                               ^^^^ could not find `text` in `insert_line`

error[E0433]: cannot find `text` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:38
   |
```

### Diagnostic 67 (log line 17437)

```text
error[E0433]: cannot find `text` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:38
   |
16 |     TextCodec { opcode: remove_line::text::TEXT_OPCODE, try_encode: remove_line::text::try_encode, decode: remove_line::text::decode...
   |                                      ^^^^ could not find `text` in `remove_line`

error[E0433]: cannot find `text` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:82
   |
```

### Diagnostic 68 (log line 17443)

```text
error[E0433]: cannot find `text` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:82
   |
16 |     TextCodec { opcode: remove_line::text::TEXT_OPCODE, try_encode: remove_line::text::try_encode, decode: remove_line::text::decode...
   |                                                                                  ^^^^ could not find `text` in `remove_line`

error[E0433]: cannot find `text` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:121
   |
```

### Diagnostic 69 (log line 17449)

```text
error[E0433]: cannot find `text` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:16:121
   |
16 | ...ne::text::try_encode, decode: remove_line::text::decode_mutation },
   |                                               ^^^^ could not find `text` in `remove_line`

error[E0433]: cannot find `text` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:35
   |
```

### Diagnostic 70 (log line 17455)

```text
error[E0433]: cannot find `text` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:35
   |
17 |     TextCodec { opcode: set_line::text::TEXT_OPCODE, try_encode: set_line::text::try_encode, decode: set_line::text::decode_mutation },
   |                                   ^^^^ could not find `text` in `set_line`

error[E0433]: cannot find `text` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:76
   |
```

### Diagnostic 71 (log line 17461)

```text
error[E0433]: cannot find `text` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:76
   |
17 |     TextCodec { opcode: set_line::text::TEXT_OPCODE, try_encode: set_line::text::try_encode, decode: set_line::text::decode_mutation },
   |                                                                            ^^^^ could not find `text` in `set_line`

error[E0433]: cannot find `text` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:112
   |
```

### Diagnostic 72 (log line 17467)

```text
error[E0433]: cannot find `text` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:17:112
   |
17 |     TextCodec { opcode: set_line::text::TEXT_OPCODE, try_encode: set_line::text::try_encode, decode: set_line::text::decode_mutation },
   |                                                                                                                ^^^^ could not find `text` in `set_line`

error[E0433]: cannot find `text` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:51
   |
```

### Diagnostic 73 (log line 17473)

```text
error[E0433]: cannot find `text` in `set_snapshot`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:51
   |
19 | pub const TEXT_OPCODES: &[&str] = &[set_snapshot::text::TEXT_OPCODE, set_trailing_newline::text::TEXT_OPCODE, set_line_ending::text:...
   |                                                   ^^^^ could not find `text` in `set_snapshot`

error[E0433]: cannot find `text` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:92
   |
```

### Diagnostic 74 (log line 17479)

```text
error[E0433]: cannot find `text` in `set_trailing_newline`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:92
   |
19 | ...t::TEXT_OPCODE, set_trailing_newline::text::TEXT_OPCODE, set_line_ending::text::TEXT_OPCODE, insert_line::text::TEXT_OPCODE, remo...
   |                                          ^^^^ could not find `text` in `set_trailing_newline`

error[E0433]: cannot find `text` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:128
   |
```

### Diagnostic 75 (log line 17485)

```text
error[E0433]: cannot find `text` in `set_line_ending`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:128
   |
19 | ...ine::text::TEXT_OPCODE, set_line_ending::text::TEXT_OPCODE, insert_line::text::TEXT_OPCODE, remove_line::text::TEXT_OPCODE, set_l...
   |                                             ^^^^ could not find `text` in `set_line_ending`

error[E0433]: cannot find `text` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:160
   |
```

### Diagnostic 76 (log line 17491)

```text
error[E0433]: cannot find `text` in `insert_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:160
   |
19 | ...ne_ending::text::TEXT_OPCODE, insert_line::text::TEXT_OPCODE, remove_line::text::TEXT_OPCODE, set_line::text::TEXT_OPCODE];
   |                                               ^^^^ could not find `text` in `insert_line`

error[E0433]: cannot find `text` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:192
   |
```

### Diagnostic 77 (log line 17497)

```text
error[E0433]: cannot find `text` in `remove_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:192
   |
19 | ...sert_line::text::TEXT_OPCODE, remove_line::text::TEXT_OPCODE, set_line::text::TEXT_OPCODE];
   |                                               ^^^^ could not find `text` in `remove_line`

error[E0433]: cannot find `text` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:221
   |
```

### Diagnostic 78 (log line 17503)

```text
error[E0433]: cannot find `text` in `set_line`
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:19:221
   |
19 | ..., remove_line::text::TEXT_OPCODE, set_line::text::TEXT_OPCODE];
   |                                                ^^^^ could not find `text` in `set_line`

warning: unnecessary qualification
  --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🦀️.rs:95:23
   |
```


## Current Twelve Leaf Signature Check

All12 current six-owner×two-format codecs expose actual defining constants and try_encode/decode_mutation functions. Binary BINARY_TAG:u32, try_encode(&TxtMutation)->Option<Result<Vec<u8>,String>>, decode_mutation(&[u8])->Result<TxtMutation,String>; Text TEXT_OPCODE:&str, try_encode(&TxtMutation)->Option<Result<String,String>>, decode_mutation(&str)->Result<TxtMutation,String>. Current Payload imports all name standards::v_utf_8::subsets::any::schema::mutations::<Payload>. The proposed2wrapper roster correction therefore preserves function-pointer types and defining payload authority. Exact signatures retained inputs/txt-current-twelve-leaf-codec-signatures-readonly.json.
